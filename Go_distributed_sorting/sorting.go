package main

import (
	"fmt"
	"sync"
	"time"
	"math/rand"
	"sort"
	"log"
)

func send(ch chan int, value int) {
	ch <- value
}

func receive(ch chan int) int {
	value := <-ch
	return value
}

func distributedSortFirst(processID int, n int, values *[]int, rightChanIn chan int, rightChanOut chan int, wg *sync.WaitGroup, terminationChIn chan bool, terminationChOut chan bool){
	defer wg.Done()
	lin := -10000
	alreadySend := false
	_, mx := update(values)
	end := false

	for !end {
		if mx > lin{
			send(rightChanIn, mx)
			LogToConsole("%d wysłałem %d", processID,mx)

			removeElement(values, mx)
			_, mx = update(values)    
			LogToConsole("%d wysłałem i czekam", processID)
			lin, _ = <- rightChanOut
			*values = append(*values, lin)
			_, mx = update(values)							
		}else if !alreadySend{
			terminationChIn <- true
			LogToConsole("%d	send termination to right", processID)
			alreadySend = true
			end = <-terminationChOut
		}
		time.Sleep(1 * time.Second)

		select {
		case end = <-terminationChOut:
		default:
			continue
		}
	}
	
	fmt.Println(fmt.Sprintf("%d: Wychodzę! %d", processID,values))
	time.Sleep(10 * time.Millisecond)
}

func distributedSort(processID int, n int, values *[]int, rightChanIn chan int, leftChanIn chan int, rightChanOut chan int, leftChanOut chan int,  wg *sync.WaitGroup,
	                                rightTerminationChIn chan bool, leftTerminationChIn chan bool, rightTerminationChOut chan bool, leftTerminationChOut chan bool) {
	defer wg.Done()
	lin := -1000
	l := 777
	mn, mx := update(values)
	leftReadyForEnd := false
	alreadySend := false
	end := false
	for !end{
		LogToConsole("%d next loop", processID)
		if mx > lin{
			send(rightChanIn, mx)
			LogToConsole("%d wysłałem %d i czekam", processID,mx)
			removeElement(values, mx)
			mn, mx = update(values)
			lin, _ = <- rightChanOut
			*values = append(*values, lin)
			mn, mx = update(values)							
		}else{
			if !alreadySend && leftReadyForEnd{
				rightTerminationChIn <- true
				LogToConsole("%d	send termination to right", processID)
				alreadySend = true
			}
		}
		LogToConsole("%d nasłuchuję pracuję %d", processID,values)
		select {
			case l = <- leftChanOut:
				*values = append(*values, l)
				mn, mx = update(values)
				send(leftChanIn, mn)
				removeElement(values, mn)	
				mn, mx = update(values)
			case end = <- rightTerminationChOut:
				LogToConsole("%d	get termination from right and send termination to left", processID)
				leftTerminationChIn <- true
			case leftReadyForEnd = <- leftTerminationChOut:
				LogToConsole("%d	get termination from left", processID)
			default:
				continue
		}	
	}			
	LogToConsole("%d: Wychodzę! %d", processID,values)
	time.Sleep(100 * time.Millisecond)
}

func distributedSortLast(processID int, n int, values *[]int, leftChanIn chan int, leftChanOut chan int, wg *sync.WaitGroup, terminationChIn chan bool, terminationChOut chan bool){
	defer wg.Done()
	l := 0
	mn, _ := update(values)
	end := false
	for !end{
		LogToConsole("%d	pracuję %d", processID,values)
		select{
			case l, _ = <- leftChanOut:
				*values = append(*values, l)
				mn, _ = update(values)
				send(leftChanIn, mn)
				LogToConsole("%d wysłałem %d", processID,mn)
				removeElement(values, mn)
				mn, _ = update(values)
			case end = <-terminationChOut:
				terminationChIn <- true
				LogToConsole("%d	get termination from left", processID)
				LogToConsole("%d	send termination from left", processID)
		}								

	}
	LogToConsole("%d: Wychodzę! %d", processID,values)
	time.Sleep(30 * time.Millisecond)
}

func main() {
	n := 100   // Number of processes
	x := 20  // Number of values in each process

	channelsR := make([]chan int, n+1)
	channelsL := make([]chan int, n+1)
	quitChR := make([]chan bool, n+1)
	quitChL := make([]chan bool, n+1)
	
	for i := range channelsR {
		channelsR[i] = make(chan int, 1)
		channelsL[i] = make(chan int, 1)
		quitChR[i] = make(chan bool, 1)	
		quitChL[i] = make(chan bool, 1)
	}

	arrays := make([][]int, n)
	for i := 0; i < n; i++ {
		arrays[i] = generateRandomArray(x)
	}

//	for i, arr := range arrays {
		//fmt.Printf("Tablica %d: %v\n", i, arr)
//	}

	
	var wg sync.WaitGroup
	wg.Add(n)

	go distributedSortFirst(0, n, &arrays[0], channelsR[1], channelsL[0], &wg, quitChR[1], quitChL[0])

	// Start each process
	for i := 1; i < n-1; i++ {
		go distributedSort(i, n, &arrays[i], channelsR[i+1], channelsL[i-1], channelsL[i], channelsR[i], &wg, quitChR[i+1], quitChL[i-1], quitChL[i], quitChR[i])
	}
	go distributedSortLast(n-1, n, &arrays[n-1], channelsL[n-2], channelsR[n-1], &wg, quitChL[n-2], quitChR[n-1])

	wg.Wait()

	for _, arr := range arrays {
		fmt.Println(arr)
	}
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
func LogToConsole(message string, args ...interface{}) {
	if false {
		log.Printf("# " + message, args...)
	}
}