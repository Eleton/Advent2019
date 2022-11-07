package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

func main() {
	a := 0
	b := 0
	exampleA := Alpha("example.txt")
	fmt.Printf("Test alpha: %v - Expected: %v\n", exampleA, a)
	if exampleA == a {
		solutionA := Alpha("signal.txt")
		fmt.Printf("Solution alpha: %v\n", solutionA)

		exampleB := Beta("example.txt")
		fmt.Printf("Test beta: %v - Expected: %v\n", exampleB, b)
		if exampleB == b {
			solutionB := Beta("signal.txt")
			fmt.Printf("Solution beta: %v\n", solutionB)
		}
	}
}

func read(s string) []int {
	content, _ := ioutil.ReadFile(s)
	numbers := strings.Split(string(content), "\n")
	var ints []int

	for _, s := range numbers {
		i, _ := strconv.Atoi(s)
		ints = append(ints, i)
	}
	return ints
}
