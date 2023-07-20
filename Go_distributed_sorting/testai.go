package main

import (
	"fmt"
	"sync"
)

func worker(id int, quitCh <-chan struct{}, wg *sync.WaitGroup) {
	defer wg.Done()

	for {
		select {
		case <-quitCh:
			fmt.Printf("Gorutyna %d: Otrzymano sygnał zakończenia\n", id)
			return
		default:
			// Wykonywanie normalnych operacji gorutyny
			fmt.Printf("Gorutyna %d: Wykonuję operacje\n", id)
		}
	}
}

func main() {
	numWorkers := 5
	quitCh := make(chan struct{})
	var wg sync.WaitGroup

	for i := 0; i < numWorkers; i++ {
		wg.Add(1)
		go worker(i, quitCh, &wg)
	}

	// Symulacja działania programu przez pewien czas
	// Tutaj można umieścić logikę, która kontroluje czas działania gorutyn

	// Wysłanie sygnału do wszystkich gorutyn, że czas na zakończenie
	close(quitCh)

	// Oczekiwanie na zakończenie wszystkich gorutyn
	wg.Wait()

	fmt.Println("Wszystkie gorutyny zakończyły działanie")
}
