package main

func Alpha(s string) int {
	ints := read(s)
	n := 0
	for i := 1; i < len(ints); i++ {
		if ints[i] > ints[i-1] {
			n = n + 1
		}
	}

	return n
}
