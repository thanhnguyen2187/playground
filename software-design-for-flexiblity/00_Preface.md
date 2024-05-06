# Preface

> Observations of biological systems tell us a great deal about how to make
> flexible and evolable systems. Techniques originally developed in support of
> symbolical artificial intelligence can be viewed as ways of enhancing
> flexibility and adapability in programs and other engineered systems. By
> contrast, common practice of computer science actively discourages the
> construction of systems that are easily modified for use in novel settings.

## This book

> [...] We think that anyone who is building complex systems, such as
> computer-language compilers and integrated development environments, will
> benefit from our experience.

## The contents

> - Chapter 1 is an introduction to our programming philosophy. Here we show
>   *flexibility* in the grand context of nature and engineering. We try to make
>   the point that flexibility is as important an issue as efficiency and
>   correctness. In each subsequent chapter we introduce techniques and
>   illustrate them with sets and exercises. This is an important organizing
>   principle for the book.
> - In chapter 2 we explore some universally applicable ways of building systems
>   with room to grow. A powerful way to organize a flexible system is to build
>   it as an assembly of domain-specific languages, each appropriate for easily
>   expressing the construction of a subsystem. Here we develop basic tools for
>   the development of domain-specific languages:
>   - We show how subsystems can be organized around mix-and-match parts,
>   - How can they be flexibly combined with *combinators*,
>   - How *wrappers* can be used to generalize parts, and
>   - How we can often simplify a program by abstracting out a domain model.
> - In chapter 3 we introduce the extremely powerful but potentially dangerous
>   flexibility technique of predicate-dispatched *generic procedures*.
>   - We start by generalizing arithmetic to deal with symbolic algebraic
>     expressions.
>   - We then show how such a generalization can be made efficient by using type tags
>     for data, and
>   - We demonstrate the power of the technique with the design of a simple, but
>     easy to elaborate, adventure game.
> - In chapter 4 we introduce symbolic *pattern matching*, first to enable
>   term-rewriting systems, and later, with *unification*, to show how type
>   inference can easily be made to work. Here we encounter the need for
>   *backtracking* because of segment variables. Unification is the first place
>   where we see the power of representing and combining *partial-information*
>   structures. We end the chapter with extending the idea to matching general
>   graphs.
> - In chapter 5 we explore the power of *interpretation* and *compilation*. We
>   believe that programmers should know how to escape the confines of whatever
>   programming language they must use by making an interpreter for a language
>   that is more appropriate for expressing the solution to the current problem.
>   We also show how to naturally incorporate backtracking search by
>   implementing nondeterministic `amb` in an interpreter/compiler system, and
>   how to use *continuations*.
> - In chapter 6 we show how to make systems of *layered data* and *layered
>   procedures*, where each data item can be annotated with a variety of
>   metadata. The processing of the underlying data is not affected by the
>   metadata, and the code for processing the underlying data does not even know
>   about or reference the metadata. However, metadata is processed by its own
>   procedures, effectively in parallel with the data. We illustrate this by
>   attaching units to numerical quantities and by showing how to carry
>   dependency information, giving the provenance of data, as derived from the
>   primitive sources.
> - This is all brought together in chapter 7, where we introduce *propagation*
>   to escape from the expression-oriented paradigm of computer languages. Here
>   we have a wiring-diagram vision of connecting modules together. This allows
>   the flexible incorporation of multiple sources of partial information. Using
>   layered data to support tracking of dependencies enables the implementation
>   of *dependency-directed backtracking*, which greatly reduces the search
>   space in large and complex systems.

