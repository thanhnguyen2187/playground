# Notes and Exercise Answers for SDF

TBA

```
;; make sure that we are in the right directory and has `mit-scheme` ready
(load "./sdf/manager/load")
```

```
(manage 'new-environment 'combinators)
```

```
(define plus-3 (compose (lambda (x) (+ x 1))
                        (lambda (x) (+ x 2))))
```

