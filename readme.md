# c--

- a C compiler
- a x86_64 assembler
- a static linker

all implementations from scratch.

## Compiler Roadmap(WIP)

- expressions
  - binary-operation(int)
    - [x] addition
    - [x] subtraction
    - [x] multiplication
    - [x] division
    - [ ] modulo operation
    - [ ] left-shift
    - [ ] right-shift
    - [ ] `<=`
    - [ ] `>=`
    - [ ] `==`
    - [ ] `!=`
    - [ ] bit-wise AND
    - [ ] bit-wise XOR
    - [ ] logical AND
    - [ ] logical OR
  - [] conditional-operator
    - `logical-OR-expression "?" expression ":" conditional-expression`
  - assignment operators
    - [x] `=`
    - [ ] `*=`
    - [ ] `/=`
    - [ ] `+=`
    - [ ] `-=`
    - [ ] `<<=`
    - [ ] `>>=`
    - [ ] `&=`
    - [ ] `^=`
    - [ ] `|=`
  - [] comma operator
- statements
  - [x] if-else
  - [x] for
  - [x] while

## ABI

- `sizeof(int)` ... 8
