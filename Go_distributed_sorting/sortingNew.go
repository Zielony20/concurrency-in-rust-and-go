package main

import (
	"fmt"
	"sync"
	"time"
	"math/rand"

	//"os"
	//"os/exec"
	//"runtime"
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
	lin := -10000
	specialToken := false
	endToken := false
	tokenValue := -8888;
	lastSend := -10001
	mn, mx := update(values)
	fmt.Println(mn)
	send(rightChanIn, mx)
	fmt.Println(fmt.Sprintf("%d send mx=%d to right", processID, mx))
	removeElement(values, mx)
	mn, mx = update(values)	
	

	for !endToken {
		select {
		case lin := <-rightChanOut:
			fmt.Println(fmt.Sprintf("%d get lin=%d from right. Actual mx=%d. Array=%d", processID,lin,mx,values))
			if !specialToken && lin == lastSend {
				specialToken = true
				send(rightChanIn, tokenValue)
				fmt.Println(fmt.Sprintf("%d send \033[31mTOKEN\033[0m right", processID))
				send(rightChanIn, lin)
	//			fmt.Println(fmt.Sprintf("%d send lin=%d to right", processID, lin))
			} else if specialToken && lin == tokenValue {
				fmt.Println(fmt.Sprintf("%d get last \033[31mTOKEN\033[0m", processID))
				endToken = true
				//*values = append(*values, lin)
				//mn, mx = update(values)
			} else {
				*values = append(*values, lin)
				mn, mx = update(values)
				send(rightChanIn, mx)
				lastSend = mx
	//			fmt.Println(fmt.Sprintf("%d send mx=%d to right", processID, mx))
				removeElement(values, mx)
				mn, mx = update(values)
			}
		}
	}
	
	lin = <- rightChanOut
	*values = append(*values, lin)
	mn, mx = update(values)
	
	
	fmt.Println(fmt.Sprintf("%d get lin=%d from right. Actual mx=%d. Array=%d", processID,lin,mx,values))

}

func distributedSort(processID int, n int, values *[]int, rightChanIn chan int, leftChanIn chan int, rightChanOut chan int, leftChanOut chan int,  wg *sync.WaitGroup) {
	defer wg.Done()
	lin := -1000
	l := 0
	specialToken := false
	specialTokenSend := false
	mn, mx := update(values)
	endToken := false
	tokenValue := -8888

	send(rightChanIn, mx)
	removeElement(values, mx)
	mn, mx = update(values)

	for !endToken{
		select {
			
		case l = <- leftChanOut:
			//			fmt.Println(fmt.Sprintf("%d get l=%d from left. Actual mx=%d. Array=%d", processID,l,mx,values))
							
							if !specialToken && l == tokenValue {
								specialToken = true	
								//send(rightChanIn, tokenValue)
									
							}else{
								*values = append(*values, l)
								mn, mx = update(values)
								send(leftChanIn, mn)
						//		fmt.Println(fmt.Sprintf("%d send mn=%d to left", processID, mn))
								removeElement(values, mn)
								mn, mx = update(values)
							}

			case lin = <- rightChanOut:
				//fmt.Println(fmt.Sprintf("%d get lin=%d from right. Array=%d", processID,lin,values))
					
				if !specialTokenSend && specialToken && lin >= mx{
					send(rightChanIn, tokenValue)
					fmt.Println(fmt.Sprintf("%d send \033[31mTOKEN\033[0m right", processID))
					specialTokenSend = true
					send(rightChanIn, lin)
				//	fmt.Println(fmt.Sprintf("%d send lin to right (mn for warning%d) ", processID, mx, mn))
				} else if specialTokenSend && lin == tokenValue {
					endToken = true	
					send(leftChanIn, tokenValue)
					fmt.Println(fmt.Sprintf("%d send \033[31mTOKEN\033[0m left", processID))

				}else{
					*values = append(*values, lin)
					mn, mx = update(values)
					send(rightChanIn, mx)
				//	fmt.Println(fmt.Sprintf("%d send mx=%d to right (mn for warning%d) ", processID, mx, mn))
					removeElement(values, mx)
					//mn, mx := update(values)
				}
			
					
		}
	}
	select{
		case lin = <- rightChanOut:
			*values = append(*values, lin)
			mn, mx = update(values)
		case l = <- leftChanOut:
			*values = append(*values, l)
			mn, mx = update(values)
		
	}

	fmt.Println(fmt.Sprintf("%d get lin=%d from right. Actual mx=%d. Array=%d", processID,lin,mx,values))

	fmt.Println(fmt.Sprintf("%d	kończę działanie", processID))
}

func distributedSortLast(processID int, n int, values *[]int, leftChanIn chan int, leftChanOut chan int, wg *sync.WaitGroup){
	defer wg.Done()
	l := 0
	mn, _ := update(values)
	specialToken := false
	endToken := false
	tokenValue := -8888

	for !endToken{
		select{
			case l = <- leftChanOut:
				//fmt.Println(fmt.Sprintf("%d get l=%d from left. Actual mn=%d. Array=%d", processID,l,mn,values))
				if l == tokenValue{
					
					specialToken = true

				}else if specialToken && l < mn{
					send(leftChanIn, tokenValue)
					fmt.Println(fmt.Sprintf("%d send end \033[31mTOKEN\033[0m", processID))
					endToken = true
					send(leftChanIn, l)
				//	fmt.Println(fmt.Sprintf("%d send l=%d to left", processID, l))
				}else{
					*values = append(*values, l)
					mn, _ = update(values)
					send(leftChanIn, mn)
				//	fmt.Println(fmt.Sprintf("%d send mn=%d to left", processID, mn))
					removeElement(values, mn)
					mn, _ = update(values)
				}
			
										
		}
	}

	fmt.Println(fmt.Sprintf("%d	kończę działanie", processID))	
}

func main() {
	n := 8   // Number of processes
	x := 4  // Number of values in each process

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
	for i, arr := range arrays {
		fmt.Printf("Tablica %d: %v\n", i, arr)
	}
	var wg sync.WaitGroup
	wg.Add(n)

	go distributedSortFirst(0, n, &arrays[0], channelsR[1], channelsL[0], &wg)

	// Start each process
	for i := 1; i < n-1; i++ {
		go distributedSort(i, n, &arrays[i], channelsR[i+1], channelsL[i-1], channelsL[i], channelsR[i], &wg)
	}
	go distributedSortLast(n-1, n, &arrays[n-1], channelsL[n-2], channelsR[n-1], &wg)

	go func() {
		for {
			fmt.Println("--------------",)
			
			for i, arr := range arrays {
				fmt.Printf("Tablica %d: %v\n", i, arr)
			}	
			time.Sleep(1 * time.Second)
		}
		
	}()


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
		arr[i] = rand.Intn(25)
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
