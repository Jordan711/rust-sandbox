/**
 * Run this to verify Rust is installed
 * rustc --version
 * 
 * This compiles the program
 * rustc main.rs
 * 
 * Run the compiled exe
 * ./main.exe
 * 
 * Exclamation mark ! means a macro instead of normal function
 * 
 * Cargo is Rust’s build system and package manager
 * - Building your code
 * - Downloading dependencies
 * - Building dependencies
 * 
 * cargo --version
 * 
 * Using cargo, a different command is used to compile
 * cargo run
 * 
 * Compiles with optimizations
 * cargo build --release
 * 
 * Cargo.lock keeps track of dependencies
 * 
 * This creates a new project
 * cargo new hello_world --bin
 */

fn main() {
    println!("Hello, world!");
}