#let formula_wrapper(body) = {
  align(
    left,
    box(body),
  )
}

#set page(
  columns: 2,
  margin: 10pt,
)

#block(
  fill: luma(230),
  inset: 8pt,
  radius: 4pt,
  width: 100%,
  [
    == Limits

    #formula_wrapper([$ lim_(x -> a) f(x) = L & $])
    It's read as $x$ approaches $a$, $f(x)$ approaches $L$.

    === Basic formulas

    #block(
      [
        #box([$ lim_(x -> a) x = a $])
        #h(8pt)
        #box([$ lim_(x -> infinity) 1 / x = 0 $])
      ]
    )
    #formula_wrapper([$
      lim_(x -> a) c = c
      "where" c "is a constant"
    $])
    #formula_wrapper([$
      lim_(x -> a) (f(x) + g(x)) =
      lim_(x -> a) f(x) + lim_(x -> a) g(x)
    $])
    #formula_wrapper([$
      lim_(x -> a) (f(x) dot g(x)) =
      lim_(x -> a) f(x) dot lim_(x -> a) g(x)
    $])
    #formula_wrapper([$
      lim_(x -> a) f(x) / g(x) =
      (lim_(x -> a) f(x)) / (lim_(x -> a) g(x))
      "provided" lim_(x -> a) g(x) != 0
    $])

    === Advanced formulas

    #box([$
      "L'Hopital's Rule:" lim_(x -> a) f(x) / g(x) = (lim_(x -> a) f'(x)) / (lim_(x -> a) g'(x))
    $])
    #box([$
      lim_(x -> a) f(x) / g(x) = +infinity dot op("sign")(f(x))
      "if" g(x) -> 0^+
    $])
    #box([$
      lim_(x -> a) f(x) / g(x) = -infinity dot op("sign")(f(x))
      "if" g(x) -> 0^-
    $])
  
  ],
)


$ f'(x) = lim_(h -> 0) ((f(x+h) - f(x)) / h) $

#block(
  fill: luma(230),
  inset: 8pt,
  radius: 4pt,
  width: 100%,
  [
    == Derivatives

    - Lagrange's notation: $f'(x)$
    - Leibniz's notation: $dif/dif(x) f(x)$

    === Basic formulas

    #box([$ (e^x)' = e^x $])
    #block(
      [
        #box([$ x^n ' = n x^(n - 1) $])
        #h(8pt)
        #box([$ a^x ' = a^x ln(a) "where" a > 0 $])
      ]
    )
    #block(
      [
        #box([$ ln(x) ' = 1/x $])
        #h(8pt)
        #box([$ log_(a)x ' = 1/(x ln(a)) $])
      ]
    )
    #block([$
      (f(x) dot g(x))' = f'(x) dot g(x) + f(x) dot g'(x)
    $])
    #block([$
      (f(x) / g(x))' = (f'(x) dot g(x) + f(x) dot g'(x)) / (g^2(x))
    $])
    #block([$
      (f(g(x)))' = f'(g(x)) dot g'(x)
    $])
    #block(
      [
        #box([$ sin(x) ' = cos(x) $])
        #h(8pt)
        #box([$ cos(x) ' = -sin(x) $])
      ]
    )
  ],
)

Common derivatives:

- TBA

==== Integrals

- TBA

Common integrals:

- TBA

==== Fundamental Theorem of Calculus

- TBA

where $ F(x) $ is the antiderivative of $ f(x) $.

=== Multivariable Calculus

==== Partial Derivatives

- TBA

==== Gradient

- TBA

==== Divergence

- TBA

==== Curl

- TBA

==== Green's Theorem

- TBA

==== Stokes' Theorem

- TBA

== Linear Algebra

=== Vectors and Matrices

- *Vector*: TBA
- *Matrix*: TBA

=== Determinants

- TBA

=== Eigenvalues and Eigenvectors

- TBA

=== Inverse of a Matrix

- TBA

== Ordinary Differential Equations

=== First-Order ODEs

==== Separable Equations

- TBA

==== Integrating Factor

- TBA

- TBA

=== Second-Order ODEs

==== Homogeneous Equations

- TBA

==== Characteristic Equation

- TBA

=== Systems of ODEs

- TBA

=== Laplace Transform

- TBA

Feel free to expand and customize this cheat sheet according to your needs.

