# 100DaysOfRust
Learn Rust and Swift in parallel. Comparing the language features.

Day 1: [Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)
1. let defines immutable variables by default
2. let x = 1; let x = "  "; will create new variable with same name. This is called variable shadowing.

Day 2: [Control FLow](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
1. break x + 1; break the loop and return a value
2. expression block shall not append ";" to the last line. 

Day 3: [Setup cargo](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html)
1. install rust
2. install plugin to Visual Studio Code
3. terminal: cargo new project to a git folder
4. open project folder in VSCode
5. build and run
6. edit .git/info/exclude, add following lines to ignore non-source files from git repository
  * \*/target/\*
  * \*/src/\*
  * !\*/src/\*.rs

Day 4: [Hands on Rust programming](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)
1. ::new is an associated function, i.e. static method
2. Result.expect(str) return result(in case Ok()) or str (in case Err(_) catchall)
3. match a.cmp(&mut b) {arm1: pattern1 => fn1, arm2: pattern2 => fn2}
4. References are immutable by default. Hence &mut b instead of &b

Day 5: Coding Rust with VSCode
1. use 'task' to build and run
2. Tabnine autocomplete is very handly

Day 6: Ownership 1
1. memory is managed through a system of ownership with a set of rules that the compiler checks at compile time. None of the ownership features slow down your program while it’s running.

Day 7: Ownership 2
1. There’s a design choice that’s implied by this: Rust will never automatically create “deep” copies of your data.
2. Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.
3. We cannot have a mutable reference while we have an immutable one.
4. It's ok if the last usage of the immutable references occurs before the mutable reference is introduced. A reference’s scope starts from where it is introduced and continues through the last time that reference is used.
5. 
```
fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
```
This works without any problems. Ownership is moved out, and nothing is deallocated.
6.slice[..] is of &str.

Something about Trait
1. Trait definitions are a way to group method signatures together to define a set of behaviors necessary to accomplish some purpose.
2. Use {} for a default implementation
3. Default implementations can call other methods in the same trait, even if those other methods don’t have a default implementation.

Day 8 - 9: Structs
1. Struct update syntax: ..struct_instant_1
2. impl method(&self)

Day 10: Enums and Match
1. Enums are a way of grouping related values so you can use them without spelling mistakes.
2. you have to convert an Option<T> to a T before you can perform T operations with it. 
3. The code associated with each arm is an expression, and the resulting value of the expression in the matching arm is the value that gets returned for the entire match expression.
4. if let arm = enum {} else {}
5. You’ll see this pattern a lot in Rust code: match against an enum, bind a variable to the data inside, and then execute code based on it.

Day 11: Vector
```
let v = vec![1, 2, 3, 4];
let first = &v[0]; // immutable borrow 
v.push(6); // mutable borrow
```
1. you can't have mutable and immutable references in the same scope.
2. We can define an enum whose variants will hold the different value types, and then all the enum variants will be considered the same type: that of the enum. Then we can create a vector that holds that enum and so, ultimately, holds different types:
```
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
```
Day 12: String and HashMap/Dictionary
1. Store string in UTF-8
2. let s = format!("{}-{}-{}", s1, s2, s3);

Day 13: HashMap
1. scores.entry(String::from("Yellow")).or_insert(50);
2. ownership:
```
let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point
```
3. entry().or_insert()
```
for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
```
Day 14: Error Handling
1. two groups: recoverable and unrecoverable
2. Result<T, E> for recoverable and panic! for unrecoverable
3. When you choose to return a Result value, you give the calling code options rather than making the decision for it.

Day 15: Generics
1. handling duplication of concepts.

Day 16: Reference Lifetime
1. When returning a reference from a function, the lifetime parameter for the return type needs to match the lifetime parameter for one of the parameters.


Day 17: I/O handing
