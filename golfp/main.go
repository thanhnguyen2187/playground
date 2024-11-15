package main

import (
	"fmt"
	"golfp/impure"
	"golfp/pure"
)

func main() {

	plus1 := pure.Attach(func(x int) int { return x + 1 })

	plus3 := impure.Detach(impure.Compose2(
		plus1,
		plus1,
	))

	fmt.Printf("Plus 3: %d\n", plus3(3))
}
