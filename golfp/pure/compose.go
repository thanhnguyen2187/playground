package pure

func Compose2[I any, T1 any, T2 any](
	f1 func(I) T1,
	f2 func(T1) T2,
) func(I) T2 {
	f := func(i I) T2 {
		t1 := f1(i)
		return f2(t1)
	}
	return f
}

func Compose3[I any, T1 any, T2 any, T3 any](
	f1 func(I) T1,
	f2 func(T1) T2,
	f3 func(T2) T3,
) func(I) T3 {
	f := Compose2(f1, f2)
	f_ := Compose2(f, f3)
	return f_
}

func Compose4[I any, T1 any, T2 any, T3 any, T4 any](
	f1 func(I) T1,
	f2 func(T1) T2,
	f3 func(T2) T3,
	f4 func(T3) T4,
) func(I) T4 {
	f := Compose3(f1, f2, f3)
	f_ := Compose2(f, f4)
	return f_
}

func Attach[I any, T any](f func(I) T) func(I) (T, error) {
	f_ := func(i I) (T, error) {
		return f(i), nil
	}
	return f_
}
