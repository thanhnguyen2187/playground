from sympy import pprint, symbols, solveset, Eq


def main():
    x, y = symbols("x, y")
    equation = Eq(x + 2, y)
    equation = equation.subs(y, 3)
    equation = equation.subs(x, 0)

    pprint(solveset(equation, x))


if __name__ == "__main__":
    main()
