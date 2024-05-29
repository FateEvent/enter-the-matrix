# Enter the Matrix

### Intro

You need a pencil. Pencils are important. And yeah, a paper sheet. A lot of them, actually. And don't forget the course on linear algebra [_Essence of linear algebra_](https://www.youtube.com/playlist?list=PLZHQObOWTQDPD3MizzM2xVFitgF8hE_ab), by 3Blue1Brown.
Another useful resource may be the modules on [linear algebra](https://www.khanacademy.org/math/linear-algebra) and the modules for [vectors](https://www.khanacademy.org/math/precalculus/x9e81a4f98389efdf:vectors) and [matrices](https://www.khanacademy.org/math/precalculus/x9e81a4f98389efdf:matrices) of the Khan Academy.

### Fused Multiply-Accumulate

This [page](https://www.felixcloutier.com/x86/vfmadd132ps:vfmadd213ps:vfmadd231ps) describes how the instructions vfmadd132ps, vfmadd213ps and vfmadd231ps work.

Wait, what's a floating point value, exactly?

A floating point value is a real number using an integer with a fixed precision, called the significand, scaled by an integer exponent of a fixed base.
In computing, floating-point arithmetic (FP) is arithmetic that represents subsets of floating point numbers.
From [Wikipedia](https://en.wikipedia.org/wiki/Floating-point_arithmetic).

Notice that vfmadd132ps, vfmadd213ps and vfmadd231ps are instructions, and not system calls.

An instruction set architecture defines the machine code understood by some processor. An instruction changes the (observable) state of the computer (e.g. changes content of processor registers -including the program counter and the call stack pointer, memory locations in virtual address space, etc...).

A system call, on the other hand, is done by some application program to request services from the operating system kernel. It may correspond to an elementary machine instruction (e.g. SYSENTER or SYSCALL), but the kernel will run a big lot of code before returning to the application program.
From [Wikipedia](https://stackoverflow.com/questions/44201171/what-is-the-difference-between-system-calls-and-instruction-set).

We can find these instructions in [Rust](https://docs.rs/num-traits/latest/num_traits/ops/mul_add/trait.MulAdd.html), [C](https://www.gnu.org/software/c-intro-and-ref/manual/html_node/Fused-Multiply_002dAdd.html) and [C++](https://en.cppreference.com/w/cpp/numeric/math/fma).

### From C to Rust

[A Guide to Porting C/C++ to Rust](https://locka99.gitbooks.io/a-guide-to-porting-c-to-rust)

### Exercise 0

Before starting, I learnt to use Rust with the [Rustlings](https://github.com/rust-lang/rustlings).

As concerning the square root function, I used the [sqrt](https://doc.rust-lang.org/std/primitive.f64.html#method.sqrt) function implemented for the f64 primitive.

[Here](https://stackoverflow.com/questions/4842424/list-of-ansi-color-escape-sequences) is a list of colour escape sequences (it's important to note that Rust only considers [hexadecimal escape sequences](https://stackoverflow.com/questions/69981449/how-do-i-print-colored-text-to-the-terminal-in-rust)).

### Exercise 1

As concerning the fused multiply-add function, I used the version implemented by the [mul_add](https://doc.rust-lang.org/std/primitive.f64.html#method.mul_add) function for the f64 primitive, and to use it I implemented the `ToF64` trait for the primitive type.

For this exercise, I used [traits](https://doc.rust-lang.org/std/ops/index.html#traits) to implement [operator overloading](https://doc.rust-lang.org/rust-by-example/trait/ops.html).

I used traits already existing in the std library and traits I had to implement, as `ToF64` and `ToK`, that I created __and__ implemented:

```
pub trait ToF64 {
	fn to_f64(&self) -> f64;
}

// Implement the ToF64 trait for f64 itself
impl ToF64 for f64 {
	fn to_f64(&self) -> f64 {
		*self
	}
}
```
