# Rust-Learning
Repository about learning of Rust following the #100DaysOfRust

## Day 1
### Installation

Rust uses a command line tool called `rustup` for the installation inside the system.

If the installation is successfully, you should have the next message inside the console.

```bash
Rust is installed now. Great!
```

I follow the chapter one in the [Rust Book 1. Installation](https://doc.rust-lang.org/book/ch01-01-installation.html)

### Cargo Tool

Cargo is the package manager that use Rust, Cargo can download the dependencies in the project, compiles the project.

| Command       | Description                       |
| :------------ | :-------------------------------- |
| `cargo new`   | Use for create a new package.     |
| `cargo build` | Use for compile the project.     |
| `cargo run`   | Use for compile and execute.     |

I follow the chapter two in the [Cargo Book 2. Cargo Guide](https://doc.rust-lang.org/cargo/guide/index.html)

## Day 2
### Variables

In Rust, by default the variables are immutable, but exists a different between
`const` and `immutable`, we can change the immutability of the variables using `mut`

Example:

```bash
let x = 5;
x = 6;
```
In the example, we get the following error.
```bash
2 |     let x = 5;
  |         -
  |         |
  |         first assignment to `x`
  |         help: consider making this binding mutable: `mut x`
3 |     println!("The value of x is: {}", x);
4 |     x = 6;
  |     ^^^^^ cannot assign twice to immutable variabl
```
for change the inmutability of the variables use `mut` before the name of the variable.
```bash
let mut x = 5;
x = 6;
```

### Shadowing

The concept of the shadowing is when we can use the name of a variable n times inside the code.
We use shadowing when we want to the variable continue to be immutable. The compiler read the code and if the compiler find a variable with the same name, the compiler automatically takes the ultimate value.

```bash
let x = 5;
let x = 6;
```


