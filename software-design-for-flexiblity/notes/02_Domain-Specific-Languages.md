# 2. Domain-Specific Languages

```scheme
(define (compose f g)
  (lambda args
    (f (apply g args))))
```

```scheme
(define ((iterate n) f)
  (if (= n 0)
    identity
    (compose f ((iterate (- n 1)) f))))

(define (identity x) x)
```

```scheme
(define (square x) (* x x))
```

```scheme
(((iterate 3) square) 5)
```

```scheme
(define (parallel-combine h f g)
  (define (the-combination . args)
    (h (apply f args)
       (apply g args)))
  the-combination)
```

```scheme
((parallel-combine list
                   (lambda (x y z) (list 'foo x y z))
                   (lambda (u v w) (list 'bar u v w)))
 'a 'b 'c)
```

> The `parallel-combine` combinator can be useful in organizing a complex
> process. For example, suppose we have a source of images of pieces of
> vegetable. We may have on procedure that given the image can estimate the
> color of the vegetable, and another that can give a description of the shape
> (leaf, root, stalk). We may have a third procedure that can combine these
> descriptions to identify the vegetable. These can be neatly composed with
> `parallel-combine`.

---

*Exercise 2.1: Arity repair*

The procedures *compose* and *parallel-combine* that we have introduced do not
obey the requirement that they advertise the arity of the combination. Thus they
would not be good citizens of our family of combinators. Fix the implementation
of *compose* and *parallel-combine* shown above, so that:

- they check their components to make sure that the arities are compatible;
- the combination they construct checks that it is given the correct number of
  arguments when it is called;
- the combination advertises its arity correctly for *get-arity*.
