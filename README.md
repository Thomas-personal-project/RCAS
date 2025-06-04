# RCAS
A Rust Computer Algebra System that is heavily in development.
This is meant to be a system for evaluating arbirary mathematical expressions,
and solving equations
It is made up of 3 components:
 - The RPOL/rcas-lib component, which takes in MIR syntax and spits out results
 - The Frontent/transpiler component, which takes in any string mathematical
 expression, analyzes it, and produces MIR that will return the result
 - The rcas-frontend component, which will run in the terminal as a CLI, take in
   an expression, run it through the transpiler and then the executor, and spit
   out the answer
