# 1. Flexibility in Nature and in Design

## Additive programming

> Our goal in this book is to investigate how to construct computational systems
> so that they can be easily adapted to changing requirements. One should not
> have to modify a working program. One should be able to add to it to implement
> new functionality or to adjust old functions for new requirements. We call
> this *additive programming*. We explore techniques to add functionality to an
> existing program without breaking it. Our techniques do not guarantee that the
> additions are correct: the additions must themselves be debugged; but they
> should not damage existing functionality accidentally.
>
> [...]
> 
> In order for additive programming to be possible, it is necessary to minimize
> the assumptions about how a program works and how it will be used. Assumptions
> made during the design and construction of a program may reduce the possible
> future extensions of the program. Instead of making such assumptions, we build
> our programs to make just-in-time decisions based on the environment that the
> program is running in. We will explore several techniques that support this
> kind of design.
>
> [...] If we want to build additively, it is important that the individual
> pieces combine with minimal unintended interactions.
>
> To facilitate additive programming, it is necessary that the parts we build to
> be as simple and general as we can make them. For example, a part that accepts
> a wider range of inputs than is strictly necessary for the problem at hand
> will have a wider applicability than one that doesn't. [...]
>
> For maximum flexibility the range of outpus of a part should be quite small
> and well defined--much smaller than the range of acceptable inputs for any
> part that might receive that output. This is analogous to the static
> discipline in the digital abstraction that we teach to students in
> introductory computer systems subjects [...]. The essence of the digital
> abstraction is that the outputs are always better than the acceptable inputs
> of the next stage, so that noise is suppressed.
>
> A family of mix-and-match parts for a particular domain of discourse is the
> foundation of a *domain-specific language*. Often the best way to attack a
> family of hard problems is to make a language--a set of primitives, means of
> combination, and means of abstraction--that makes the solutions for those
> problems easy to express. [...]

In short:

- Broaden inputs, narrow down outputs
- Try to develop a vocabulary/glossary for the problem

Similar ideas can be found from SICP and Rich Hickey's talk.

> One strategy for enhancing flexibility, which should be familiar to many
> programmers, is *generic dispatch*. [...]. Generic dispatch is often a useful
> way to extend the applicability of a procedure by adding additional handlers
> (methods) based on details of the arguments passed to the procedure. By
> requiring handlers to respond to disjoint sets of arguments, we can avoid
> breaking an existing program when a new handler is added. However, unlike the
> generic dispatch in the typical object-oriented programming context, our
> generic dispatch doesn't involve ideas like classes, instances, and
> inheritance. These weaken the separation of concerns by introducing spurious
> ontological commitments.
>
> A quite different strategy, [...], is to *layer* both data and procedures.
> This exploits the idea that data usually has associated metadata that can be
> processed alongside the data. For example, numerical data often has associated
> units. We will show how providing the flexibility of adding layers after the
> fact can enhance a program with new functionality, without any change to the
> original program.
>
> We can also build systems that combine multiple sources of *partial
> information* to obtain more complete answers. This is most powerful when the
> contributions come from independent sources of information. [...]. Locally
> deducible clues about the type of a value, for example that a numerical
> comparison requires numerical inputs and produces a boolean output, can be
> combined with other local type constraints to produce nonlocal type
> constraints.
>
> [...]
>
> A dual idea is the use of *degeneracy*: having multiple ways to compute
> something, which can be combined or modulated as needed. There are many value
> uses for degeneracy, including error detection, performance management, and
> intrusion detection. Importantly, degeneracy is also additive: each
> contributing part is self-contained and produce a result by itself. One
> interesting use of degeneracy is to dynamically select from different
> implementations of an algorithm depending on context. This avoids the need to
> make assumptions about how the implementation will be used.
>
> Design and construction for flexibility has definite costs. A procedure that
> can take a greater variety of inputs than are necessary for solving the
> current problem will have more code than absolutely necessary and will take
> more thinking by the programmer than absolute necessary. The same goes for
> generic dispatching, layering, and degeneracy, each of which involves constant
> overheads in memory space, compute time, and/or programmer time. But the
> principal cost of software is the time spent by programmers over the lifetime
> of the product, including maintenance and adaptions that are needed for
> changing requirements. Design that minimize rewriting and refactoring reduce
> the overall costs to the incremental additions rather than complete rewrites.
> In other words, long-term costs are additive rather than multiplicative.

TBA
