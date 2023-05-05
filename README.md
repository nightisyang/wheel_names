# wheel_names
Simple CLI application that selects a name from a list of names provided in names.txt written in rust.
To use, build from source code, see instructions (here)[https://doc.rust-lang.org/book/ch01-03-hello-cargo.html#building-for-release] 
```cargo build --release``` and place a list of names in a file ```names.txt``` in the same directory as the executable.

<img width="564" alt="image" src="https://user-images.githubusercontent.com/101862364/236451726-8f831d9d-8728-4641-8fd0-434aa136448b.png">

## Main learnings
- Borrow checker is cool, still need some getting used to.
- Many different ways to iterate a vector
  - for name in names.iter()
  - for (index, name) in names.iter().enumerate()
  - .collect() to make a collection
- Need to get used to error handling as a primary concern, either
  - use match statement to get Ok() or Error() case
  - unwrap(), and other alternatives of unwarp_etc()
- ChatGPT is a good teacher!
