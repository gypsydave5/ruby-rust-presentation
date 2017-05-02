package main

import "C"

//export fibonacci
func fibonacci(number int) int {
	a := 0
	b := 1

	for i := 0; i < number; i++ {
		tmp := a
		a = b
		b = a + tmp
	}

	return a
}

func main() {}
