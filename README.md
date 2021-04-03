# 100DaysOfRust
Learn Rust and Swift in parallel. Comparing the language features.

Day 1: [Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)
1. let defines immutable variables by default
2. let x = 1; let x = "  "; will create new variable with same name. This is called variable shadowing.

Day 2: [Control FLow](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
1. break x + 1; break the loop and return a value
2. expression block shall not append ";" to the last line. 

Day 3: [setup cargo](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html)
1. install rust
2. install plugin to Visual Studio Code
3. terminal: cargo new project to a git folder
4. open project folder in VSCode
5. build and run
6. edit .git/info/exclude, add following lines to ignore non-source files from git repository
  * \*/target/\*
  * \*/src/\*
  * !\*/src/\*.rs