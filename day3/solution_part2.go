package main

import (
	"bufio"
	"fmt"
	"math/big"
	"os"
)

// Select k digits from line to form the maximum number
// Greedy: at each step, pick the largest digit that leaves enough remaining
func maxJoltage12(line string, k int) *big.Int {
	n := len(line)
	if n < k {
		return big.NewInt(0)
	}

	result := make([]byte, k)
	pos := 0 // current position in line

	for i := 0; i < k; i++ {
		// Need (k - i - 1) more digits after this one
		// So we can pick from pos to (n - (k - i - 1) - 1) = n - k + i
		maxDigit := byte('0')
		maxPos := pos

		for j := pos; j <= n-k+i; j++ {
			if line[j] > maxDigit {
				maxDigit = line[j]
				maxPos = j
			}
		}

		result[i] = maxDigit
		pos = maxPos + 1
	}

	num := new(big.Int)
	num.SetString(string(result), 10)
	return num
}

func main() {
	file, err := os.Open("input.csv")
	if err != nil {
		fmt.Println("Error opening file:", err)
		return
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	total := big.NewInt(0)

	for scanner.Scan() {
		line := scanner.Text()
		if len(line) < 12 {
			continue
		}
		joltage := maxJoltage12(line, 12)
		total.Add(total, joltage)
	}

	if err := scanner.Err(); err != nil {
		fmt.Println("Error reading file:", err)
		return
	}

	fmt.Println("Total output joltage:", total.String())
}
