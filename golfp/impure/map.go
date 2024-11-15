package impure

func Map[I any, O any](f func(I) (O, error), items []I) ([]O, error) {
	outputs := make([]O, 0)
	for _, item := range items {
		o, err := f(item)
		if err != nil {
			return outputs, err
		}
		outputs = append(outputs, o)
	}
	return outputs, nil
}

func MapC[I any, O any](f func(I) (O, error), items []I) ([]O, []I, []error) {
	outputs := make([]O, 0)
	errInputs := make([]I, 0)
	errs := make([]error, 0)
	for _, item := range items {
		o, err := f(item)
		if err != nil {
			errInputs = append(errInputs, item)
			errs = append(errs, err)
			continue
		}
		outputs = append(outputs, o)
	}
	return outputs, errInputs, errs
}

func MapCP[I any, O any](f func(I) (O, error), items []I) ([]O, []I, []error) {
	outputs := make([]O, 0)
	errInputs := make([]I, 0)
	errs := make([]error, 0)
	for _, item := range items {
		go func() {
			o, err := f(item)
			if err != nil {
				errInputs = append(errInputs, item)
				errs = append(errs, err)
			}
			outputs = append(outputs, o)
		}()
	}
	return outputs, errInputs, errs
}
