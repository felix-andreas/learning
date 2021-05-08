package main

import "fmt"

type Human struct {
	name string
	age  int
}

func main() {
	ema := Human{"Emaunel", 26}
	fmt.Println(ema)
}
