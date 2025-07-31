# Building a Static Type-Inferring Compiler Course

repo: https://github.com/rtfeldman/compiler-workshop-v1

## Introduction

Source -> Machine Instructions

* Ahead-of-Time
* Runtime Interpreter
* Just-in-Time Compiler

* WASM is designed to be JIT compiled

* Intermediate Representation

* there are also Transpilers
* You can have IR Interpreter or IR Compiler (JIT or AOT)

### Terminology

* Assembler: LL -> LL
* Transpiler: HL -> HL
* Interpreter: Generates at runtime
* Compiler: Take input, generate output

#### Analysis

* Name checking
* Type checking
* Optimization

### Overview

* Compiler for small subset of JS
* Formatting
* Name Resolution
* Type Inference (Hindley-Milner, not TypeScript!)
* WASM Generation

## Traversing the AST

### Lexing

```js
const x = 5 // x is 5
```

```json
[
    {
        type: "CONST",
        value: null,
        position: 0
    },
    {
        type: "IDENTIFIER",
        value: "x",
        position: 6
    },
    {
        type: "EQUALS",
        value: null,
        position: 8
    },
    {
        type: "NUMBER",
        value: "5",
        position: 10
    },
    {
        type: "COMMENT",
        value: " x is 5",
        position: 12
    }
]
```

### Formatting

* Lexing: Source String -> Tokens
* Parsing: Tokens -> Parse Tree
* Generation: Parse Tree -> Output String

#### Syntax Sugar

We tranlsate

```js
x+++
```

into:

```js
x = x + 2
```

Basically just another sytnax node. But this makes our formatter into a transpiler.

## Bindings & Scope Resolution

### Name Resolution

New Step: Name resolution (Canonicalization):

Parse Tree -> Parse Tree + Scope

### Declaration & Lookups

```js
const x = 5
const y = x + 1
```

Scope data structure:

```js
const scope = new Set()
scope.add("x")
```

* Possible to detect a duplicate error:

```js
scope.has("x")
```

* shadowing is if same declaration in nested scope

### Nested Scopes

we need an array of scope:

```js
const scope = new Set()
scopes.push(scope)
scope.add("inc")
const inner = new Set()
scopes.push(inner)
inner.add("n")

if (!scopes.some(scope => scope.has("n"))) {
    ...
}

scopes.pop()
```

### Hoisting

In JS you can call a function before it is declared:

```js
const x = 5
const y = inc(3)

function inc(arg) {
    return arg + 1
}
```

One solution - Use a second separate hoisting pass:
1. First path ignores all none function elements
2. Regular non-hoisting pass

### (Digression) Canonicalization

Cononical IR examples
* string interning -> replace identifier with integer + lookup table
* lower syntax sugar into simpler nodes

### Question: How to keep track of scope?

In general it is uncommen to keep track of in subsequent phases.
Better solution: In canonicalization you can different integers for variables which have the same identifier (if they are in differnt scopes).

## Type Inference

### Hindley-Milner Type Sytem

#### Why Hindley-Milner Inference?o

* used by Elm, Haskell, Roc

* Simple typesystem (no co/contravariance etc.)
* no sub-typing
* Sound (no runtime type mismatches)

Soundess:
* Java compiler is possible to create a runtime type mismatch
* TypeScript is intentionally unsound

* HM is decidable (no annotations required, etc)
* Polymorphic (supports generics)

* Unsound type system requires runtime to detect runtime type mismatches (performance penalty)
* In sound type system you can discard the types at runtime

#### Infering Constants

```js
const a = "hi"
const b = a
const c = b
```

We use a types database (union find):

we start initializing with null:

| id  | type | expr    |
| --- | ---- | ------- |
| 0   | null | const a |
| 1   | null | "hi"    |
| 2   | null | const b |
| 3   | null | const c |

Assign type relationships:

| id  | type  | expr    |
| --- | ----- | ------- |
| 0   | db[1] | const a |
| 1   | str   | "hi"    |
| 2   | db[0] | const b |
| 3   | db[2] | const c |

Tokens -> Parse Tree -> Types 

#### Path Compression

* Type Database is often called union find, because it is a strategy for path compression.
* Path compression optimization relies on type system property that we don't have subtyping.
* Digression: Chris Lattner used HM plus extra features for Swift which made it impossible to do this optimization.
* Why keep one link and go directly to concrete type. Answer: because types can get arbitarly big. Copying type can be expensive.
* Unification, not subtyping

#### Editor tooling

What is the type of `const c` ?

1. find source position of `c`
2. find that parse tree node
3. get that nodes type id
4. look up that id in the db

#### Summary

* Lexing: Source String -> Tokens
* Parsing: Tokens -> Parse Tree
* Naming: Parse Tree -> Parse Tree + Scope Diagnostics
* Inference: Parse Tree -> Parse Tree + Type Database

* Type Variables 
* Type Relationships
* Path Compression
