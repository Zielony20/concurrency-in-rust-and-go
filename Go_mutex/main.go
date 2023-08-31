package main

import (
	"fmt"
	"sync"
	"os"
	"strconv"
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
	numIterations, err := strconv.Atoi(param2)
	if err != nil {
		fmt.Println("Błąd parsowania parametru:", err)
		return
	}

	var sharedResource int
	var mutex sync.Mutex


	var wg sync.WaitGroup
	for i := 0; i < numWorkers; i++ {
		wg.Add(1)
		go func() {
			for j := 0; j < numIterations; j++ {
				mutex.Lock()
				sharedResource++
				mutex.Unlock()
			}
			wg.Done()
		}()
	}

	wg.Wait()

}