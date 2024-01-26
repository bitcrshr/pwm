package main

/*
#include <stdlib.h>
*/
import "C"

import (
	"github.com/gorcon/rcon"
)

func main() {}

//export Execute
func Execute(host *C.char, password *C.char, command *C.char) *C.char {
	conn, err := rcon.Dial(C.GoString(host), C.GoString(password))
	if err != nil {
		panic(err)
	}

	defer conn.Close()

	cmd := C.GoString(command)

	resp, err := conn.Execute(cmd)
	if err != nil {
		panic(err)
	}

	return C.CString(resp)
}
