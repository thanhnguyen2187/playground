package impure

import "fmt"

func Compose2[I any, T1 any, T2 any](
	f1 func(I) (T1, error),
	f2 func(T1) (T2, error),
) func(I) (T2, error) {
	f := func(i I) (T2, error) {
		t1, err := f1(i)
		var t2 T2
		if err != nil {
			return t2, err
		}
		return f2(t1)
	}
	return f
}

func Compose3[I any, T1 any, T2 any, T3 any](
	f1 func(I) (T1, error),
	f2 func(T1) (T2, error),
	f3 func(T2) (T3, error),
) func(I) (T3, error) {
	f := Compose2(f1, f2)
	f_ := Compose2(f, f3)
	return f_
}

func Compose4[I any, T1 any, T2 any, T3 any, T4 any](
	f1 func(I) (T1, error),
	f2 func(T1) (T2, error),
	f3 func(T2) (T3, error),
	f4 func(T3) (T4, error),
) func(I) (T4, error) {
	f := Compose3(f1, f2, f3)
	f_ := Compose2(f, f4)
	return f_
}

func Detach[I any, T any](f func(I) (T, error)) func(I) T {
	f_ := func(i I) T {
		t, err := f(i)
		if err != nil {
			fmt.Println("Detach: failed")
			panic(err)
		}
		return t
	}
	return f_
}
