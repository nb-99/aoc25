package main

import (
	"bufio"
	"fmt"
	"os"
)

func maxJoltage(line string) int {
	maxVal := 0
	n := len(line)

	// Try every pair (i, j) where i < j
	for i := 0; i < n-1; i++ {
		for j := i + 1; j < n; j++ {
			// Form two-digit number from digits at positions i and j
			d1 := int(line[i] - '0')
			d2 := int(line[j] - '0')
			val := d1*10 + d2

			if val > maxVal {
				maxVal = val
			}
		}
	}
	return maxVal
}

func main() {
	file, err := os.Open("input.csv")
	if err != nil {
		fmt.Println("Error opening file:", err)
		return
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	total := 0

	for scanner.Scan() {
		line := scanner.Text()
		if len(line) < 2 {
			continue
		}
		joltage := maxJoltage(line)
		total += joltage
	}

	if err := scanner.Err(); err != nil {
		fmt.Println("Error reading file:", err)
		return
	}

	fmt.Println("Total output joltage:", total)
}
