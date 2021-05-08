/*
	Package main - transpiled by c2go version: v0.26.0 Erbium 2020-03-17

	If you have found any issues, please raise an issue at:
	https://github.com/elliotchance/c2go/
*/

package main

import "os"

// main - transpiled function from  /home/felix/Projects/tutorials/go/c2go.c:1
func main() {
	argc := int32(len(os.Args))
	argv__multiarray := [][]byte{}
	argv__array := []*byte{}
	for _, argvSingle := range os.Args {
		argv__multiarray = append(argv__multiarray, append([]byte(argvSingle), 0))
	}
	for _, argvSingle := range argv__multiarray {
		argv__array = append(argv__array, &argvSingle[0])
	}
	argv := *(***byte)(unsafe.Pointer(&argv__array))
	var x int32 = int32(1)
	if x > int32(10) {
		return
	} else {
		os.Exit(int(int32(1)))
	}
}
