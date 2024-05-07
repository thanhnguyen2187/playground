# 2. Domain-Specific Languages

> One powerful strategy for building flexibility into a programming project is
> to create a *domain-specific language* that captures the conceptual structure
> of the subject matter of the programs to be developed. A domain-specific
> language is an abstraction in which the nouns and verbs of the language are
> directly related to the problem domain. Such a language allows an application
> program to be written directly in terms of the domain. By its nature, a
> domain-specific language implements a fairly complete model of the domain, in
> excess of what is needed for a particular application. Although this may seem
> like extra work that is not essential to the particular problem at hand, it is
> often less work than writing a monolithic program, and the resulting program
> is much easier to modify, debug, and extend.
>
> So a domain-specific language layer is built to support more than just the
> development of a particular program.
> 
> - It provides a general framework for the construction of a variety of related
>   programs that share the domain of discourse.
> - It simplifies the process of extending an existing application in that
>   domain.
> - And it provides a substrate that allows related applications to cooperate.
>
> [...]

## 2.1 Combinators

> Biological systems achieve much of their adapability through the use of very
> general parts (cells) that are dynamically configured and consequently able to
> adjust as their environment changes. Computational systems usually do not use
> this strategy, instead relying on a hierachy of custom parts and combinations.
> In recent years, large libraries of well-specified higher-level parts have
> raised the abstraction level of this activity. But the means of combination
> are rarely abstracted or shared, other than as "patterns".
>
> [...]
>
> A *system of combinators* is
>
> - set of primitive parts and
> - a set of means of combining parts such that
>   - the interface specifications of the combinations are the same as those of
>     the primitives.
>
> [...]

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

