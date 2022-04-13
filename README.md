# LCL
Experimental stack-oriented programming language. Memory edition.

## Table of contents
1. [Usage](#usage)
3. [Milestones](#milestones)
2. [Language Basics](#language-basics)

## Milestones
- [x] Compilation to asm (only x86_64)
- [ ] [Turing-completeness](https://en.wikipedia.org/wiki/Turing_completeness)
- [ ] [Self-hosting compiler](https://en.wikipedia.org/wiki/Self-hosting_(compilers))
- [ ] Optimization
- [ ] Windows and MacOS support

## Usage
```
lcl 0.3.0
Experimental stack-oriented programming language. Memory edition.

USAGE:
    lcl [OPTIONS] [INPUT]

ARGS:
    <INPUT>    Target file

OPTIONS:
    -h, --help               Print help information
    -o, --output <OUTPUT>    Place the output into <OUTPUT>
    -V, --version            Print version information
```

## Language Basics
### Numbers
Number is a sequence of decimal digits. When one is encountered it's simply pushed onto the stack.

Example:
```
1 2 3
```
The code above pushes `1`, `2`, `3` onto the stack thus `3` is on top.

### Built-ins
#### Arithmetics
| Keyword | Description |
| ---     | ---         |
| `+`     | sums up two values on top of the stack |
| `=`     | subtracts two values on top of the stack (the first from the second) |

#### Comparison
- `0` is `false`
- anything not equal to 0 is considered true

| Keyword | Description |
| ---     | ---         |
| `<`     | applies `less` on top two values |
| `>`     | applies `greater` on top two values |
| `=`     | applies `equal` on top two values |
| `!=`    | applies `not equal` on top two values |

#### Stack manipulation
| Keyword | Description |
| ---     | ---         |
| `dup`   | duplicates top two value on the stack |

#### Memory
| Keyword | Description |
| ---     | ---         |
| `mem`   | pointer to the beginning of the memory |
| `@`     | stores value on top of the stack into memory |
| `!`     | loads value from memory and pushes it onto the stack |

#### Misc
| Keyword | Description |
| ---     | ---         |
| `.`     | prints top value on the stack |

### Control flow
#### `if`
If value on top of the stack is `true` then executes `if` body otherwise executes `else` body if any

Example:
```
3 2 >
if
    1 .
else
    2 .
end
```

#### `while`
Executes body until `while-expression` pushes `true` onto the stack

Example:
```
0 while dup 10 < do
    dup .
    1 +
end
```
The code above prints numbers from 0 to 9

### Comments
Two types of comments are supported:
- `//` inline comment
- `/* */` multi-lines comment

Example:
```
/*
    multi
    lines
    comment
*/

1 2 3 // inline comment
```