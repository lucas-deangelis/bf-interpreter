# bf-interpreter

A brainfuck interpreter.

## Usage

It takes one or multiple files this way: `bf-interpreter file-1.bf ... file-n.bf`.

## Technical details

Each file (program) has a separate "runtime", which is composed of an array of 30 000 bytes (u8 in Rust), a data pointer (usize), a program pointer (usize), and a vector of tokens (the program instructions). As explained below, I suggest compiling it with release mode as some programs rely on integer overflow. Considering the program pointer is usize, this may cause problems with programs with a very large number of instructions.

## Tests

There's 3 programs in the tests: `helloworld.bf`, `mandelbrot.bf` and `test.bf`. `mandelbrot.bf` and `test.bf` both depends on integer overflow to run correctly. It caused me problems during testing as I ran the interpreter with `cargo run file.bf`, which was using debug mode, which panics on overflow.

## Credits

The `helloworld.bf` and `mandelbrot.bf` programs came from https://github.com/Overv/bf.

The `young.bf` program comes from https://esolangs.org/wiki/FBP.

The `cellwidth.bf` program comes from https://esolangs.org/wiki/Brainfuck.
