#set page(
  columns: 2,
  margin: 10pt,
)

=== Problem 1: #box($ f(x) = 3x^2 + 5x - 7 $)

#block([
  #box($ f(x) = 3x^2 + 5x - 7 $) \
  #box($ f'(x) = 6x + 5 $)
])

=== Problem 2: #box($ f(x) = sqrt(x) $)

#block([
  #box($ f(x) = sqrt(x) = x^(1/2) $) \
  #box($ f'(x) = 1/2 dot x^(1/2 - 1) $) \
  #box($ = 1/2 dot x^(-1/2) $) \
  #box($ = 1/2 dot 1 / x^(1/2) $) \
  #box($ = 1/2 dot 1 / sqrt(x) $) \
  #box($ = 1/(2 sqrt(x)) $) \
])

=== Problem 3: #box($ f(x) = x^2 sin(x) $)

#block([
  #box($ f(x) = x^2 sin(x) $) \
  #box($ f'(x) = (x^2)'sin(x) + x^2 sin'(x) $) \
  #box($ = 2x sin(x) + x^2 cos(x) $) \
])

=== Problem 4: #box($ f(x) = sin(3x^2 + 2x) $)

#block([
  Set $g(x)$ = $3x^2 + 2x$ \
  #box($ g'(x) = (3x^2 + 2x)' = 6x + 2 $) \
  #box($ f(x) = sin(3x^2 + 2x) = sin(g(x)) $) \
  #box($ f'(x) = f'(g(x)) dot g'(x) $) \
  #box($ = sin'(g(x)) dot g'(x) $) \
  #box($ = cos(3x^2 + 2x) dot 6x + 2 $) \
])

=== Problem 5: #box($ f(x) = e^2x $)

#block([
  #box($ f(x) = e^(2x) $) \
  #box($ f'(x) = (2x)' e^(2x) $) \
  #box($ = 2 e^(2x) $) \
])

=== Problem 6: #box($ f(x) = ln(x^2 + 1) $)

#block([
  #box($
    f(x) &= ln(x^2 + 1) \
    f'(x) &= (x^2 + 1)' / (x^2 + 1) \
    &= (2x) / (x^2 + 1) \
  $) \
])

=== Problem 7: #box($ f(x) = ln(x^2 + 1) $)

#block([
  #box($
    f(x) &= ln(x^2 + 1) \
    f'(x) &= (x^2 + 1)' / (x^2 + 1) \
    &= (2x) / (x^2 + 1) \
  $) \
])

=== Problem 8: #box($ f(x) = sin(x) / cos(x) $)

#block([
  #box($
    f(x) &= sin(x) / cos(x) \
    f'(x) &= (sin'(x) cos(x) - sin(x) cos'(x)) / (cos^2 x) \
          &= (cos(x) cos(x) + sin(x) sin(x)) / (cos^2 x) \
          &= (cos^2 x + sin^2 x) / (cos^2 x) \
          &= 1 / (cos^2(x)) \
  $) \
])

=== Problem 9: #box($ f(x) = x^2 e^x $)

#block([
  #box($
    f(x) &= x^2 e^x \
    f'(x) &= (x^2)' e^x + x^2 (e^x)' \
          &= 2x e^x + x^2 e^x \
          &= e^x (x^2 + 2x) \
  $) \
])
