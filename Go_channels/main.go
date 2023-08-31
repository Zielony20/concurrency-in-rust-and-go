package main

import (
	"fmt"
	"os"
	"strconv"
	"sync"
	"runtime"
)

func main() {

	runtime.GOMAXPROCS(32)
	if len(os.Args) < 3 {
		fmt.Println("Brak podanego parametru.")
		return
	}

	param := os.Args[1]
	numWorkers, err := strconv.Atoi(param)
	if err != nil {
		fmt.Println("Błąd parsowania parametru:", err)
		return
	}

	param2 := os.Args[2]
	numMessages, err := strconv.Atoi(param2)
	if err != nil {
		fmt.Println("Błąd parsowania parametru:", err)
		return
	}

	messageChannel := make(chan int)

	var wg sync.WaitGroup
	wg.Add(numWorkers + 1)

	go func() {
		defer wg.Done()
		for i := 0; i < numWorkers*numMessages; i++ {
			<-messageChannel
		}
	}()

	for i := 0; i < numWorkers; i++ {
		go func(i int) {
			defer wg.Done()
			for j := 0; j < numMessages; j++ {
				messageChannel <- i
			}
		}(i)
	}

	wg.Wait()
}
