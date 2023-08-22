package main

import (
	"fmt"
	"math/rand"
	"os"
	"runtime"
	"sort"
	"strconv"
	"sync"
	"time"
)

func distributedSortFirst(processID int, n int, values *[]int, rightChanIn chan int, rightChanOut chan int, wg *sync.WaitGroup, terminationChIn chan bool, terminationChOut chan bool) {
	defer wg.Done()
	lin := -10000
	alreadySend := false
	_, mx := update(values)
	end := false
	for !end {
		if mx > lin {
			rightChanIn <- mx
			removeElement(values, mx)
			_, mx = update(values)
			lin, _ = <-rightChanOut
			*values = append(*values, lin)
			_, mx = update(values)
		} else if !alreadySend {
			terminationChIn <- true
			alreadySend = true
			end = <-terminationChOut
		}

		select {
		case end = <-terminationChOut:
		default:
			continue
		}
	}
}

func distributedSort(processID int, n int, values *[]int, rightChanIn chan int, leftChanIn chan int, rightChanOut chan int, leftChanOut chan int, wg *sync.WaitGroup,
	rightTerminationChIn chan bool, leftTerminationChIn chan bool, rightTerminationChOut chan bool, leftTerminationChOut chan bool) {
	defer wg.Done()
	lin := -10000
	l := 777
	mn, mx := update(values)
	leftReadyForEnd := false
	alreadySend := false
	end := false
	for !end {
		if mx > lin {
			rightChanIn <- mx
			removeElement(values, mx)
			mn, mx = update(values)
			lin, _ = <-rightChanOut
			*values = append(*values, lin)
			mn, mx = update(values)
		} else {
			if !alreadySend && leftReadyForEnd {
				rightTerminationChIn <- true
				alreadySend = true
			}
		}
		select {
		case l = <-leftChanOut:
			*values = append(*values, l)
			mn, mx = update(values)
			leftChanIn <- mn
			removeElement(values, mn)
			mn, mx = update(values)
		case end = <-rightTerminationChOut:
			leftTerminationChIn <- true
		case leftReadyForEnd = <-leftTerminationChOut:
		default:
			continue
		}
	}
}

func distributedSortLast(processID int, n int, values *[]int, leftChanIn chan int, leftChanOut chan int, wg *sync.WaitGroup, terminationChIn chan bool, terminationChOut chan bool) {
	defer wg.Done()
	l := 0
	mn, _ := update(values)
	end := false
	for !end {
		select {
		case l, _ = <-leftChanOut:
			*values = append(*values, l)
			mn, _ = update(values)
			leftChanIn <- mn
			removeElement(values, mn)
			mn, _ = update(values)
		case end = <-terminationChOut:
			terminationChIn <- true
		}
	}
}

func main() {
	runtime.GOMAXPROCS(32)
	if len(os.Args) < 3 {
		fmt.Println("Brak podanego parametru.")
		return
	}
	param := os.Args[1]
	n, err := strconv.Atoi(param) // Number of processes
	if err != nil {
		fmt.Println("Błąd parsowania parametru:", err)
		return
	}
	param2 := os.Args[2]
	x, err := strconv.Atoi(param2) // Number of values in each process
	if err != nil {
		fmt.Println("Błąd parsowania parametru:", err)
		return
	}
	channelsR := make([]chan int, n+1)
	channelsL := make([]chan int, n+1)
	quitChR := make([]chan bool, n+1)
	quitChL := make([]chan bool, n+1)
	for i := range channelsR {
		channelsR[i] = make(chan int)
		channelsL[i] = make(chan int)
		quitChR[i] = make(chan bool)
		quitChL[i] = make(chan bool)
	}
	arrays := make([][]int, n)
	for i := 0; i < n; i++ {
		arrays[i] = generateRandomArray(x)
	}
	var wg sync.WaitGroup
	wg.Add(n)
	go distributedSortFirst(0, n, &arrays[0], channelsR[1], channelsL[0], &wg, quitChR[1], quitChL[0])
	for i := 1; i < n-1; i++ {
		go distributedSort(i, n, &arrays[i], channelsR[i+1], channelsL[i-1], channelsL[i], channelsR[i], &wg, quitChR[i+1], quitChL[i-1], quitChL[i], quitChR[i])
	}
	go distributedSortLast(n-1, n, &arrays[n-1], channelsL[n-2], channelsR[n-1], &wg, quitChL[n-2], quitChR[n-1])
	wg.Wait()
}
func generateRandomArray(size int) []int {
	rand.Seed(time.Now().UnixNano())
	arr := make([]int, size)
	for i := 0; i < size; i++ {
		arr[i] = rand.Intn(100000)
	}
	return arr
}
func removeElement(arr *[]int, value int) {
	index := -1

	for i, num := range *arr {
		if num == value {
			index = i
			break
		}
	}

	if index == -1 {
		return
	}

	copy((*arr)[index:], (*arr)[index+1:])
	*arr = (*arr)[:len(*arr)-1]
}
func update(arr *[]int) (int, int) {
	sort.Ints(*arr)
	if len(*arr) == 0 {
		return 0, 0
	}

	minValue := (*arr)[0]
	maxValue := (*arr)[len(*arr)-1]

	return minValue, maxValue
}
