package main

import (
	"fmt"
	"sync"
	"time"
	"math/rand"
)

func send(ch chan int, value int) {
	ch <- value
}

func receive(ch chan int) int {
	value := <-ch
	return value
}



func distributedSortFirst(processID int, n int, values *[]int, rightChanIn chan int, rightChanOut chan int, wg *sync.WaitGroup){
	defer wg.Done()
	lin := -1000
	specialToken := false
	end := false
	mn, mx := update(values)
	fmt.Println(mn)	
	for !end{
		send(rightChanIn, mx)
//		fmt.Println(fmt.Sprintf("%d send mx=%d to right", processID, mx))
		removeElement(values, mx)
//		fmt.Println(fmt.Sprintf("%d wait for right", processID))
		lin = receive(rightChanOut)

		if lin == mx && !specialToken{
			
			specialToken = true
			send(rightChanIn, -8888)
//			fmt.Println(fmt.Sprintf("%d send special token, array=%d" , processID, values))
			
		} else if lin == -8888 && specialToken {
			end = true
			lin = receive(rightChanOut)
		} else if lin != mx{
			specialToken = false
		}


//		fmt.Println(fmt.Sprintf("%d get lin=%d from right", processID,lin))
		*values = append(*values, lin)
		mn, mx = update(values)
		
	}
	fmt.Println(fmt.Sprintf("%d	kończę działanie", processID))
	//send(rightChanIn, -2000000)
}

func distributedSort(processID int, n int, values *[]int, rightChanIn chan int, leftChanIn chan int, rightChanOut chan int, leftChanOut chan int,  wg *sync.WaitGroup) {
	defer wg.Done()
	lin := -1000
	l := 0
	specialToken := false
	mn, mx := update(values)
	end := false
	for !end{
		//select{
			rightChanIn <- mx
//				fmt.Println(fmt.Sprintf("%d send mx=%d to right", processID, mx))
				removeElement(values, mx)
//				fmt.Println(fmt.Sprintf("%d wait for right", processID))
				
				lin = <- rightChanOut
				if specialToken && lin == mx {
//					fmt.Println(fmt.Sprintf("%d send special token", processID))
					rightChanIn <- -8888
				}else if specialToken && lin == -8888{
					send(leftChanIn, -8888)
//					fmt.Println(fmt.Sprintf("%d send return special token", processID))
					end = true
					lin = <- rightChanOut
				}

					*values = append(*values, lin)
					mn, mx = update(values)
//					fmt.Println(fmt.Sprintf("%d get lin=%d from right. Actual mx=%d. Array=%d", processID,lin,mx,values))
//					fmt.Println(fmt.Sprintf("%d wait for left", processID))
			
					l = <- leftChanOut
//					fmt.Println(fmt.Sprintf("%d get l=%d from left", processID,l))
					if l == -8888{
//						fmt.Println(fmt.Sprintf("%d get special token", processID))
						specialToken = true
						l = receive(leftChanOut)
//						fmt.Println(fmt.Sprintf("%d get l=%d from left", processID,l))
					}
					
					*values = append(*values, l)
					mn, mx = update(values)
					send(leftChanIn, mn)
//					fmt.Println(fmt.Sprintf("%d send mn=%d to left", processID, mn))
					removeElement(values, mn)
					mn, mx = update(values)
				
		//		}
			
		}
	
	fmt.Println(fmt.Sprintf("%d	kończę działanie", processID))
	//send(rightChanIn, -2000000)
}

func distributedSortLast(processID int, n int, values *[]int, leftChanIn chan int, leftChanOut chan int, wg *sync.WaitGroup){
	defer wg.Done()
	l := 0
	mn, _ := update(values)
	specialToken := false

	for {
//		fmt.Println(fmt.Sprintf("%d wait for left", processID))
		l = receive(leftChanOut)
		
		if l == -8888{
//			fmt.Println(fmt.Sprintf("%d get special token", processID))
			specialToken = true
			l = receive(leftChanOut)
			send(leftChanIn, -8888)
		}
	
//		fmt.Println(fmt.Sprintf("%d get l=%d from left", processID,l))
		*values = append(*values, l)
		mn, _ = update(values)
		send(leftChanIn, mn)
//		fmt.Println(fmt.Sprintf("%d send mn=%d to left", processID, mn))
		removeElement(values, mn)
		mn, _ = update(values)
		if specialToken{
			break
		}
	}
	fmt.Println(fmt.Sprintf("%d	kończę działanie", processID))	
}


func main() {
	n := 10000   // Number of processes
	x := 100  // Number of values in each process

	channelsR := make([]chan int, n+1)
	for i := range channelsR {
		channelsR[i] = make(chan int, 1)
	}

	channelsL := make([]chan int, n+1)
	for i := range channelsL {
		channelsL[i] = make(chan int, 1)
	}

	arrays := make([][]int, n)
	for i := 0; i < n; i++ {
		arrays[i] = generateRandomArray(x)
	}

	var wg sync.WaitGroup
	wg.Add(n)

	go distributedSortFirst(0, n, &arrays[0], channelsR[1], channelsL[0], &wg)

	// Start each process
	for i := 1; i < n-1; i++ {
		go distributedSort(i, n, &arrays[i], channelsR[i+1], channelsL[i-1], channelsL[i], channelsR[i], &wg)
	}
	go distributedSortLast(n-1, n, &arrays[n-1], channelsL[n-2], channelsR[n-1], &wg)

	wg.Wait()
//	time.Sleep(4000 * time.Millisecond)

	for _, arr := range arrays {
		fmt.Println(arr)
	}
}

func generateRandomArray(size int) []int {
	rand.Seed(time.Now().UnixNano())
	arr := make([]int, size)
	for i := 0; i < size; i++ {
		arr[i] = rand.Intn(10000)
	}
	return arr
}


func removeElement(arr *[]int, value int) {
	index := -1

	// Znajdujemy indeks pierwszego wystąpienia wartości
	for i, num := range *arr {
		if num == value {
			index = i
			break
		}
	}

	if index == -1 {
		return // Zwraca, jeśli wartość nie została znaleziona
	}

	// Usuwamy wartość poprzez kopiowanie elementów z wyłączeniem indeksu
	copy((*arr)[index:], (*arr)[index+1:])
	*arr = (*arr)[:len(*arr)-1]
}

func update(arr *[]int) (int, int) {
	if len(*arr) == 0 {
		return 0, 0 // Zwraca 0, 0 dla pustej tablicy
	}

	minValue := (*arr)[0]
	maxValue := (*arr)[0]

	for _, num := range *arr {
		if num < minValue {
			minValue = num
		}

		if num > maxValue {
			maxValue = num
		}
	}

	return minValue, maxValue
}
