# Axiomadic: Project Genesis and Design Philosophy

## Overview

Axiomadic (Ax) emerged from a series of discussions about creating a next-generation computational reasoning system. The goal is to seamlessly integrate programming, proof assistance, and parallel computation into a single, powerful environment.

## Key Inspirations

1. **Interaction Combinators**: The core computational model is inspired by Yves Lafont's work on interaction combinators, which provide a universal model of distributed computation.

2. **Dependent Type Theory**: Incorporating ideas from advanced type systems to allow for powerful type-level programming and theorem proving.

3. **Cubical Type Theory**: Aiming to include features from Cubical Type Theory for advanced equality reasoning and homotopy type theory concepts.

4. **Parallel Computation**: Built-in support for parallel and concurrent programming, inspired by modern hardware architectures.

5. **Effect Systems and Linear Types**: For precise control over side effects and resource usage.

## Design Decisions and Discussions

1. **Language Name**: After considering various options, "Axiomadic" was chosen, blending "axiom" (foundational truth) with "nomadic" (flexibility across paradigms).

2. **Core Calculus**: Based on interaction combinators, extended with additional features for practical programming and formal verification.

3. **Type System**: 
   - Dependent types for expressing complex properties
   - Universe hierarchy to avoid paradoxes
   - Incorporation of linear types for resource management
   - Cubical type theory features for advanced equality reasoning

4. **Parallelism**: 
   - Native support in the core language
   - Automatic parallelization by the compiler
   - Parallel proof search capabilities

5. **Proof Assistant Features**:
   - Tactic language for interactive theorem proving
   - Integration of proofs with code
   - Automated theorem proving capabilities

6. **Real Number Representation**: Discussed various approaches (Cauchy sequences, Dedekind cuts, etc.) and decided on a hybrid approach with both simple and advanced representations.

7. **Metaprogramming**: Powerful metaprogramming capabilities for extending the language and writing complex macros.

8. **Foreign Function Interface**: For interoperability with existing codebases and systems.

9. **Development Environment**: Plans for an integrated environment with REPL, debugger, visualization tools, and IDE support.

## Challenges and Future Work

1. **Balancing Theory and Practicality**: Ensuring the system is theoretically sound while still being practical for everyday programming tasks.

2. **Efficient Implementation**: Developing efficient compilation strategies, especially for the parallel aspects of the language.

3. **Proof Checking**: Implementing a reliable, efficient proof checker that can handle complex proofs in parallel.

4. **User Accessibility**: Making advanced features accessible to programmers who may not have a strong background in type theory or formal methods.

5. **Ecosystem Development**: Building a standard library, package manager, and fostering a community of developers and researchers.

## Next Steps

1. Refine the core calculus and type system
2. Develop a more comprehensive prototype interpreter
3. Begin work on the parallel computation model
4. Start designing the proof assistant features
5. Engage with the programming language and formal methods communities for feedback and collaboration

## Long-term Vision

Axiomadic aims to be more than just a programming language or proof assistant. The long-term vision is to create a comprehensive environment for computational reasoning that can:

1. Serve as a practical programming language for building complex, correct software systems
2. Act as a powerful proof assistant for mathematical and program verification
3. Leverage parallel hardware for both computation and proof search
4. Bridge the gap between theoretical computer science and practical software engineering
5. Provide a platform for exploring new ideas in type theory, category theory, and foundations of mathematics

By combining these aspects, Axiomadic has the potential to revolutionize how we approach computation, verification, and mathematical reasoning in the coming decades.