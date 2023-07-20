package main

import (
	"fmt"
	"sync"
)

func processA(in <-chan int, out chan<- int, wg *sync.WaitGroup) {
	defer wg.Done()

	value := <-in
	fmt.Println("Proces A otrzymał:", value)

	// Proces A przetwarza wartość
	result := value + 1

	// Wysyłanie przetworzonej wartości do Procesu B
	out <- result
}

func processB(in <-chan int, out chan<- int, wg *sync.WaitGroup) {
	defer wg.Done()

	value := <-in
	fmt.Println("Proces B otrzymał:", value)

	// Proces B przetwarza wartość
	result := value * 2

	// Wysyłanie przetworzonej wartości do Procesu A
	out <- result
}

func main() {
	// Inicjalizacja buforowanych kanałów o pojemności 1
	channelA := make(chan int, 1)
	channelB := make(chan int, 1)

	// Inicjalizacja grupy WaitGroup
	var wg sync.WaitGroup
	wg.Add(2)

	// Uruchamianie procesów A i B
	go processA(channelB, channelA, &wg)
	go processB(channelA, channelB, &wg)

	// Wysyłanie wartości początkowej do Procesu A
	channelA <- 3

	// Oczekiwanie na zakończenie obu procesów
	wg.Wait()

	fmt.Println("Program zakończony.")
}