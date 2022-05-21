## hack assembly
### Hack Assembly-to-Binary Translation Specification
The Hack assembly language and its equivalent binary representation were specified
in chapter 4. A compact and formal version of this language specification is repeated
here, for ease of reference. This specification can be viewed as the contract that Hack
assemblers must implement, one way or another.
#### 6.2.1 Syntax Conventions and File Formats
File Names By convention, programs in binary machine code and in assembly
code are stored in text files with ‘‘hack’’ and ‘‘asm’’ extensions, respectively. Thus, a
Prog.asm file is translated by the assembler into a Prog.hack file.
Binary Code (.hack) Files A binary code file is composed of text lines. Each line is
a sequence of 16 ‘‘0’’ and ‘‘1’’ ASCII characters, coding a single 16-bit machine language instruction. Taken together, all the lines in the file represent a machine language program. When a machine language program is loaded into the computer’s
instruction memory, the binary code represented by the file’s nth line is stored in address n of the instruction memory (the count of both program lines and memory
addresses starts at 0).
Assembly Language (.asm) Files An assembly language file is composed of text
lines, each representing either an instruction or a symbol declaration:
m Instruction: an A-instruction or a C-instruction, described in section 6.2.2.
m (Symbol): This pseudo-command binds the Symbol to the memory location
into which the next command in the program will be stored. It is called ‘‘pseudocommand’’ since it generates no machine code.
(The remaining conventions in this section pertain to assembly programs only.)
Constants and Symbols Constants must be non-negative and are written in decimal
notation. A user-defined symbol can be any sequence of letters, digits, underscore (_),
dot (.), dollar sign ($), and colon (:) that does not begin with a digit.
Comments Text beginning with two slashes (//) and ending at the end of the line is
considered a comment and is ignored.
White Space Space characters are ignored. Empty lines are ignored.
Case Conventions All the assembly mnemonics must be written in uppercase. The
rest (user-defined labels and variable names) is case sensitive. The convention is to
use uppercase for labels and lowercase for variable names.
#### 6.2.2 Instructions
The Hack machine language consists of two instruction types called addressing instruction (A-instruction) and compute instruction (C-instruction). The instruction
format is as follows.
A-instruction: @value // Where value is either a non-negative decimal number
// or a symbol referring to such number.
value (v ¼ 0 or 1)
Binary: 0 v vv v v v v v v v v v v v v
C-instruction: dest=comp;jump // Either the dest or jump fields may be empty.
// If dest is empty, the ‘=’’ is omitted;
// If jump is empty, the ‘‘;’’ is omitted.
comp dest jump
Binary: 1 1 1 a c1 c2 c3 c4 c5 c6 d1 d2 d3 j1 j2 j3
The translation of each of the three fields comp, dest, jump to their binary forms is
specified in the following three tables.
```
comp(when a=0) c1 c2 c3 c4 c5 c6 comp(when a=1)
0   1 0 1 0 1 0
1   1 1 1 1 1 1
-1  1 1 1 0 1 0
D   0 0 1 1 0 0
A   1 1 0 0 0 0  M
!D  0 0 1 1 0 1
!A  1 1 0 0 0 1 !M
-D  0 0 1 1 1 1
-A  1 1 0 0 1 1 -M
D+1 0 1 1 1 1 1
A+1 1 1 0 1 1 1 M+1
D-1 0 0 1 1 1 0
A-1 1 1 0 0 1 0 M-1
D+A 0 0 0 0 1 0 D+M
D-A 0 1 0 0 1 1 D-M
A-D 0 0 0 1 1 1 M-D
D&A 0 0 0 0 0 0 D&M
D|A 0 1 0 1 0 1 D|M

dest d1 d2 d3 jump j1 j2 j3
null 0 0 0    null 0 0 0
M 0 0 1       JGT 0 0 1
D 0 1 0       JEQ 0 1 0
MD 0 1 1      JGE 0 1 1
A 1 0 0       JLT 1 0 0
AM 1 0 1      JNE 1 0 1
AD 1 1 0      JLE 1 1 0
AMD 1 1 1     JMP 1 1 1
```
#### 6.2.3 Symbols
Hack assembly commands can refer to memory locations (addresses) using
either constants or symbols. Symbols in assembly programs arise from three
sources.
Predefined Symbols Any Hack assembly program is allowed to use the following
predefined symbols.

Label RAM address (hexa)
```
SP 0 0x0000
LCL 1 0x0001
ARG 2 0x0002
THIS 3 0x0003
THAT 4 0x0004
R0-R15 0-15 0x0000-f
SCREEN 16384 0x4000
KBD 24576 0x6000
```
Note that each one of the top five RAM locations can be referred to using two
predefined symbols. For example, either R2 or ARG can be used to refer to
RAM[2].
Label Symbols The pseudo-command (Xxx) defines the symbol Xxx to refer to the
instruction memory location holding the next command in the program. A label can
be defined only once and can be used anywhere in the assembly program, even before
the line in which it is defined.
Variable Symbols Any symbol Xxx appearing in an assembly program that is
not predefined and is not defined elsewhere using the (Xxx) command is treated as
a variable. Variables are mapped to consecutive memory locations as they are first
encountered, starting at RAM address 16 (0x0010).

c: 15(opcode)|xx|12(control)|11 10 9 8 7 6(control)|5 4 3(dest)|2 1 0(jump)
