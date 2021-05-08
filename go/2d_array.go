package main

import "fmt"

func main() {
	x := [5][5]byte{}
	fmt.Println(&x[0][3])
	fmt.Println(&x[0][4])
	fmt.Println(&x[1][0])
}
