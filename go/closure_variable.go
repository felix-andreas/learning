package main

import (
	"fmt"
	"sync"
	"time"
)

func main() {
	var wg sync.WaitGroup
	for i := 0; i < 5; i++ {
		wg.Add(1)
		go func(i int) {
			time.Sleep(time.Duration(i) * time.Second)
			fmt.Println(i)
			wg.Done()
		}(i)
	}
	wg.Wait()
}
