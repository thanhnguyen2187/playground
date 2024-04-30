# Learn TLA+

## Useful Links

- https://learntla.com/
- 

## Useful Commands

- Translate PlusCal:

```shell
java -cp tla2tools.jar pcal.trans
```

- SANY:

```shell
java -cp tla2tools.jar tla2sany.SANY
```

- Run REPL:

```shell
java -cp tla2tools.jar tlc2.REPL
```

- TLaTex:

```shell
java -cp tla2tools.jar tlc2.TLA
```

- TLC:

```shell
DEFAULT_JAVA_OPTS="-XX:+IgnoreUnrecognizedVMOptions -XX:+UseParallelGC" \
    java -cp tla2tools.jar tlc2.TLC
```

- Putting everything together:

```
# 1. translate the file
java -cp tla2tools.jar pcal.trans scratch.tla
# 2. run it
DEFAULT_JAVA_OPTS="-XX:+IgnoreUnrecognizedVMOptions -XX:+UseParallelGC" \
    java -cp tla2tools.jar tlc2.TLC scratch.tla
```

## Notes

### Booleans

- `TRUE` or `FALSE`
- `and`: `/\`
- `or`: `\/`
- `not`: `~`

Example: `Xor(A, B) == A = ~B`

- `(implication)`: `A => B`

#### Exercises

1. Rewrite `A => B` using the "regular three" programming operators:

```
| A | B | A => B | A /\ B | A \/ B | ~A | ~B |
| T | T | T      | T      | T      | F  | F  |
| T | F | F      | F      | T      | F  | T  |
| F | T | T      | F      | T      | T  | F  |
| F | F | T      | F      | F      | T  | T  |
```

```
Impl(A, B) == ~A \/ B
```

2. For what values of `A` and `B` is `~B => ~A` true?

```
| A  | B  | ~A | ~B | ~B => ~A |
| T  | T  | F  | F  | T        |
| T  | F  | F  | T  | F        |
| F  | T  | T  | F  | T        |
| F  | F  | T  | T  | T        |
```

Except when `A` is true and `B` is false, which is the same as `A => B`.

### Sequences

```
<<1, 2, 3>>
Head(<<1, 2, 3>>) = 1
Tail(<<1, 2, 3>>) = <<2, 3>>
Len(<<1, 2, 3>>) = 3
```

Similar to list in Lisp.

### Sets

```
{1, 2, 3}
```

To check if `x` is in a set:

```
x \in set
```

- `\in`
- `\union` or `\cup`
- `\intersect` or `\cap`
- `Cardinality(set)` returns a set's length
- `\X`: Cartesian product of two sets


