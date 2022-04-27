# LCL
Experimental stack-oriented programming language. Memory edition.

## Table of contents
1. [Usage](#usage)
2. [Milestones](#milestones)
3. [Language Basics](#language-basics)
    1. [Push and Pop](#push-and-pop)
        1. [Numbers](#numbers)
        2. [Registers](#registers)
        3. [Memory](#memory)
    2. [Built-ins](#built-ins)
        1. [Arithmetics](#arithmetics)
        2. [Comparison](#comparison)
        3. [Stack manipulation](#stack-manipulation)
        5. [Misc](#misc)
    3. [Control flow](#control-flow)
    4. [Functions](#functions)
        1. [Returning functions](#returning-functions)
    5. [Comments](#comments)
4. [Interactive shell](#interactive-shell)


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

## Milestones
- [x] Compilation to asm (only x86_64)
- [ ] [Turing-completeness](https://en.wikipedia.org/wiki/Turing_completeness)
- [ ] [Self-hosting compiler](https://en.wikipedia.org/wiki/Self-hosting_(compilers))
- [ ] Optimization
- [ ] Windows and MacOS support

## Language Basics
### Push and Pop
`!` and `@` are push and pop operations respectively, to perform an operation they should be prefixed to their targets.
#### Numbers
The simpliest way to put something on stack is to `push` immediate integer value
```
!1 !2 !3
```
Despite the fact that this notation is very explicit it may be daunting to put `!` every time in front of *each* value, thus `!` for immediate integer values is syntax-sugared: \
`1 2 3` will be translated to `!1 !2 !3`
#### Registers
Another way to manipulate your values is to store them in registers. Currently there are four registers:
- r1 - x86_64 `AX` register
- r2 - x86_64 `BX` register
- r3 - x86_64 `CX` register
- r4 - x86_64 `DX` register
To pop value from stack to register
```
1 @r1
```
code above will save `1` to `r1`

To push value from register to stack
```
!r1
```
code above will *copy* value stored in `r1` to the stack
#### Memory
| Keyword | Description |
| ---     | ---         |
| `mem`   | pointer to the beginning of the memory |
| `@`     | stores value on top of the stack into memory |
| `!`     | loads value from memory and pushes it onto the stack |

### Built-ins
#### Arithmetics
| Keyword | Description |
| ---     | ---         |
| `+`     | sums up two values on top of the stack |
| `-`     | subtracts two values on top of the stack (the first from the second) |

#### Comparison
- `0` is `false`
- anything not equal to 0 is considered `true`, below operations push `1` if true

| Keyword | Description |
| ---     | ---         |
| `<`     | applies `less` on top two values |
| `>`     | applies `greater` on top two values |
| `=`     | applies `equal` on top two values |
| `!=`    | applies `not equal` on top two values |

#### Stack manipulation
| Keyword | Description |
| ---     | ---         |
| `dup`   | duplicates top value of the stack |
| `drop`  | drops(removes) top value of the stack |
| `swap`  | swaps top two values of the stack |
| `over`  | takes the second value from the top of the stack and duplicates it to the top of the stack |
| `rot`   | “rotates” the top three values of the stack. The third element from the top of the stack gets moved to the top of the stack, pushing the other two values down |

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

### Functions
You can declare simple functions using keywod `fn`
```
fn your_function do
    1 .
end
```
Functions can have local variables which will store values pushed before function call
```
fn add a b do
    a b +
end

3 2 add .
```
in the code above, `a` will store `2` and `b` will store `3`, note that arguments are **moved** to the function, thus won't be available after function call.
#### Returning functions
Functions can return last pushed value of their stack frame to the stack frame they were called from.
For example:
```
fn add a b do
    a b +
end
```
this function will return sum of `a` and `b` because its last operation pushes something on the stack. However:
```
fn print a do
    a .
end
```
won't return anything because `.` does not push anything to the stack.

### Comments
Two types of comments are supported:
- `//` inline comment
- `/* */` multi-line comment

Example:
```
/*
    multi
    line
    comment
*/

1 2 3 // inline comment
```

## Interactive shell
LCL can be run as interactive shell. 

Just run it without input file provided
```bash
$ lcl
lcl 0.3.0 interactive shell
Experimental stack-oriented programming language. Memory edition.
>> 
```

In the interactive shell mode you can execute operations line by line. Note that [control flow](#control-flow) instructions are not supported in the interactive shell.

Memory and stack are simulated and will be destructed when you exit the shell.

