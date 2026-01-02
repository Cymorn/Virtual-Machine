This project implements a basic virtual machine (VIRTUAL MACHINE) in Rust. It reads programs from a text file, parses them into instructions, and executes them in a simple fetch-decode-execute loop.

The VIRTUAL MACHINE currently supports the following instructions:

1. LOAD <value> — load a number into the accumulator

2. ADD <value> — add a number to the accumulator

3. SUB <value> — subtract a number from the accumulator

4. PRINT — print the current accumulator value

5. HALT — stop execution

THE OVERVIEW 
+-------------------+
| Read next line    |
+-------------------+
          |
          v
+-------------------+
| Parse instruction |
+-------------------+
          |
          v
+-------------------+
| Execute instruction|
| (LOAD/ADD/SUB/...)|
+-------------------+
          |
          v
+-------------------+
| Repeat until HALT |
+-------------------+
The project demonstrates:

Rust module organization

Safe and clear error handling using Result types

Parsing strings into custom instruction sets

Implementation of a simple virtual machine loop



GETTING STARTED
1. Ensure Rust is installed on your system.

2. Create a file named program.vm in the project root. Example:

  LOAD 10
  ADD 5
  SUB 3
  PRINT
  HALT

RUN THE VIRTUAL MACHINE
cargo run


EXPECTED OUTPUT:
12.
If the program contains invalid instructions or values, the VM will report an error without crashing.


Future Improvements i intend to add: 

1. Implement loops and conditional jumps

2. Add unit tests for parser and VM execution

3. Compile the VM to WebAssembly for browser execution

4. Enhance instruction set for more complex programs

I AM OPEN TO ANY OPINIONS ON HOW TO IMPROVE IT.

Note:

This project is primarily a learning exercise in systems programming and Rust. The focus has been on creating a safe, maintainable, and extendable codebase.
