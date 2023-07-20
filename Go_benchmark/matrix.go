package main

import (
	"fmt"
	"os"
	"strconv"
	"sync"
	"math/rand"
	"time"
	"math"
)

func main() {
	if len(os.Args) < 3 {
		fmt.Println("Brak podanego parametru.")
		return
	}

	param := os.Args[1]
	goroutinesCount, err := strconv.Atoi(param)
	if err != nil {
		fmt.Println("Błąd parsowania parametru:", err)
		return
	}

	param2 := os.Args[2]
	matrixSize, err := strconv.Atoi(param2)
	if err != nil {
		fmt.Println("Błąd parsowania parametru:", err)
		return
	}

	if(goroutinesCount==0){
		goroutinesCount = matrixSize
	}

	rowsPerGoroutine := int(math.Ceil(float64(matrixSize) / float64(goroutinesCount)))

	matrixA := initializeMatrix(matrixSize, matrixSize);
	matrixB := initializeMatrix(matrixSize, matrixSize);

	var wg sync.WaitGroup
	wg.Add(goroutinesCount);
	for gorutineIndex := 0; gorutineIndex < goroutinesCount; gorutineIndex++ {

		startRow := gorutineIndex*rowsPerGoroutine
		if startRow > matrixSize {
			startRow = matrixSize
		}
		endRow := startRow+rowsPerGoroutine
		if endRow > matrixSize {
			endRow = matrixSize
		}
		go func(startRow int, endRow int) {

			defer wg.Done()
			for rowsA := startRow; rowsA < endRow; rowsA++{
				multiplyMatrix(matrixA, matrixB, rowsA)
			}
			
		}(startRow, endRow)	
	}
	wg.Wait()
}

func initializeMatrix(rows, cols int) [][]int {
	matrix := make([][]int, rows)
	rand.Seed(time.Now().UnixNano())

	for i := range matrix {
		matrix[i] = make([]int, cols)
		for j := range matrix[i] {
			matrix[i][j] = rand.Intn(100)
		}
	}

	return matrix
}

func  multiplyMatrix(matrixA [][]int, matrixB [][]int, row int) {
	
	numColsB := len(matrixB[0])
	numColsA := len(matrixA[0])
	outputRow := make([]int, numColsB)

	for j := 0; j < numColsB; j++ {
		for k := 0; k < numColsA; k++ {
			outputRow[j] += matrixA[row][k] * matrixB[k][j]
		}
	}
}
