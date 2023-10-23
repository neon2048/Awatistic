# Awatistic

Awatistic is a fast AWA5.0 interpreter written in Rust. It implements the [language specification](https://github.com/TempTempai/AWA5.0/) by Temp-Tempai.

## Features

* 🔥🔥🔥 BLAZINGLY FAST 🔥🔥🔥
* Awassembler so you can write your programs in Awatisms and awassemble them to Awatalk. Neat.
* Disawassembler to turn Awatalk back into Awatisms, handy for debugging and reverse engineering.
* 100% bug free and correct implementation (some of the code even has tests!!!).
* Incredibly useful and clear error messages if something goes wrong (which never happens anyways).
* Tested on Linux, may or may not work on Windows.

### Why?

Educational. I have wanted to try out Rust for some time so this seemed like a fun first project.

## Getting Started

Check out the [this](examples) directory for some AWA5.0 example programs you can run. Files ending in `.awa` contain Awatalk, files ending in `.awasm` contain Awatisms and need to be Awassembled first.

### Run a program

Like the name suggests, the Run subcommand executes Awatalk from a file. If no file is specified, Awatalk is read from stdin.

You can specify "-v" up to three times to get more verbose output.
* -v: print Awatisms as they are executed
* -vv: additionally dump Bubble Abyss to stdout after every instruction
* -vvv: additionally print Awatisms when they are parsed at the beginning of execution

```
awatistic run -f examples/jeLLy.awa
JeLLy JeLLy JeLLy JeLLy 
```

```
awatistic run --file examples/jeLLy.awa -vv
[0] blo ' ' -> [1] ' '
[1] blo 'y' -> [2] 'y', ' '
[2] blo 'L' -> [3] 'L', 'y', ' '
[3] blo 'L' -> [4] 'L', 'L', 'y', ' '
[4] blo 'e' -> [5] 'e', 'L', 'L', 'y', ' '
[5] blo 'J' -> [6] 'J', 'e', 'L', 'L', 'y', ' '
[6] srn 6 -> [1] ('J', 'e', 'L', 'L', 'y', ' ')
[7] dpl -> [2] ('J', 'e', 'L', 'L', 'y', ' '), ('J', 'e', 'L', 'L', 'y', ' ')
[8] dpl -> [3] ('J', 'e', 'L', 'L', 'y', ' '), ('J', 'e', 'L', 'L', 'y', ' '), ('J', 'e', 'L', 'L', 'y', ' ')
[9] dpl -> [4] ('J', 'e', 'L', 'L', 'y', ' '), ('J', 'e', 'L', 'L', 'y', ' '), ('J', 'e', 'L', 'L', 'y', ' '), ('J', 'e', 'L', 'L', 'y', ' ')
[10] srn 4 -> [1] (('J', 'e', 'L', 'L', 'y', ' '), ('J', 'e', 'L', 'L', 'y', ' '), ('J', 'e', 'L', 'L', 'y', ' '), ('J', 'e', 'L', 'L', 'y', ' '))
[11] prn JeLLy JeLLy JeLLy JeLLy -> [0]
Program ended.
```

### Disawassemble Awatalk

The Disawassembler can be used to turn Awatalk back into a list of Awatisms.
```
awatistic disawassemble -f examples/jeLLy.awa
blo ' '
blo 'y'
blo 'L'
blo 'L'
blo 'e'
blo 'J'
srn 6
dpl
dpl
dpl
srn 4
prn
```

### Awassemble Awatisms

The Awassembler turns a file with one Awatism per line into Awatalk. It is useful for people like me who are not as smart as Jelly and can't just write Awatalk directly.

```
awatistic awassemble -f examples/loop.awasm
awa awa awawa awawa awa awa awa awa awawa awawa awa awawa awawa awa awa awa awa awa awa awa awawa awa awa awa awa awa awa awa awawa awa awawa awawa awa awawawawawawawa awa awawa awawa awa awa awawa awawa awawa awa awawa awawa awa awa awawa awawawawa awa awawa awawa awa awa awawa awawawa awa awa awawa awawa awa awa awawa awawa awawa awa awawa awawa awa awa awawa awa awawa awa awa awawa awawa awa awa awawa awawa awa awa awa awawa awawa awa awa awawa awa awawawa awa awawa awawa awa awa awa awawa awa awa awa awa awawa awawa awa awawawa awawa awa awa awa awawa awawa awa awa awawa awa awa awawa awa awawa awawa awa awa awawa awa awa awa awa awa awawa awawa awa awa awawa awa awa awa awa awa awawa awawa awa awa awa awawawawawa awa awawa awawa awa awa awa awa awawa awa awa awawa awa awawa awawawawawa awa awa awa awawa awa awawa awawa awa awa awa awa awa awa awawa awawa awawawawa awa awawawawa awa awa awawa awa awa awa awawawawawawawa
```

When writing your Awatisms, keep in mind:
* One Awatism per line
* Everything after "#" is a comment and ignored
* The argument to "blo" can be specified as either a number (1), a single AwaSCII character in single quotes ('A') or a string of AwaSCII characters in double quotes ("JELLY") - the latter will be automatically turned into multiple blo instructions

**Example**
```
# loop head
blo 5
blo 0

# print function
lbl 1
blo "Jelly Hoshiumi\n"
srn 15
prn

# loop tail
blo 1
add
lss
jmp 1

# after loop, exit
trm
```
