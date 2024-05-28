#let formula_wrapper(body) = {
  align(
    left,
    box(body),
  )
}

#set page(
  columns: 3,
  margin: 10pt,
)

=== Calculate the following: #math.display($lim_(x -> 2) (x^2 - 4)/(x - 2)$):

#block(
  [
    #formula_wrapper([$ lim_(x -> 2) (x^2 - 4)/(x - 2) $])
    #formula_wrapper([$ = lim_(x -> 2) ((x + 2)(x - 2))/(x - 2) $])
    #formula_wrapper([$ = lim_(x -> 2) ((x + 2)cancel((x - 2)))/cancel((x - 2)) $])
    #formula_wrapper([$ = lim_(x -> 2) x + 2 $])
    #formula_wrapper([$ = 4 $])
  ]
)

#line(length: 100%)

=== Calculate the following: #math.display($lim_(x -> 0) (sin x)/x$):

#block(
  [
    Applying L'Hopital's rule:
    #formula_wrapper($ lim_(x -> 0) (sin x)/x = lim_(x -> 0) (sin x)'/x' $)
    #formula_wrapper($ = lim_(x -> 0) (cos x)/1 = 1 $)
  ]
)

#line(length: 100%)

=== Calculate the following: #math.display($lim_(x -> 0) (e^x - 1)/x$):

#block(
  [
    Applying L'Hopital's rule:
    #formula_wrapper($ lim_(x -> 0) (e^x - 1)/x = lim_(x -> 0) (e^x - 1)'/x' $)
    #formula_wrapper($ = lim_(x -> 0) e^x/1 $)
    #formula_wrapper($ = 1 $)
  ]
)


