# nea-sdf

**N**otes and **E**xercise **A**nswers for "**S**oftware **D**esign for
**F**lexibility: How to Avoid. Programming Yourself into a Corner".

## Folder Structure

- `notes/[Chapter Number]_[Chapter Name].md`: each chapter's notes. Example:

  ```
  notes/00_Foreword.md
  notes/01_Flexibility-in-Nature-and-in-Design.md
  ```

- `exercise-answers/[Exercise Number].md`: notes and answer to exercises.
  Example:

  ```
  exercise-answers/2-1.md
  exercise-answers/2-2.md
  ```

## Literate Programming

TBA

```scheme
;; make sure that we are in the right directory and has `mit-scheme` ready
(load "./sdf/manager/load")
```

```scheme
(manage 'new-environment 'combinators)
```

```scheme
(define plus-3 (compose (lambda (x) (+ x 1))
                        (lambda (x) (+ x 2))))
```

