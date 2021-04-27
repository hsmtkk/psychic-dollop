package main

import (
	"fmt"

	"github.com/hsmtkk/psychic-dollop/raindrops"
)

func main() {
	for i := 10; i < 30; i++ {
		msg := raindrops.RainDrops(i)
		fmt.Println(msg)
	}
}
