# Minigrep Terminal Output Documentation

## Command Execution Log

### 1. Running with Search String and Filename

```bash
cargo run searchstring example-filename.txt
```

**Output:**
```
   Compiling minigrep v0.1.0 (/home/valkyrie/Documents/Programming/RustProgramming/minigrep)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/minigrep searchstring example-filename.txt`
Hello, world!
```

---

### 2. Running Without Arguments

```bash
cargo run
```

**Output:**
```
   Compiling minigrep v0.1.0 (/home/valkyrie/Documents/Programming/RustProgramming/minigrep)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/minigrep`
["target/debug/minigrep"]
```

---

### 3. Running with Two Arguments

```bash
cargo run needle haystack
```

**Output:**
```
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/minigrep needle haystack`
["target/debug/minigrep", "needle", "haystack"]
```

---

### 4. Index Out of Bounds Error (Single Argument)

```bash
cargo run sample.txt
```

**Output:**
```
   Compiling minigrep v0.1.0 (/home/valkyrie/Documents/Programming/RustProgramming/minigrep)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/minigrep sample.txt`

thread 'main' (1333144) panicked at src/main.rs:7:34:
index out of bounds: the len is 2 but the index is 2
note: run with `RUST_BACKTRACE=1` runtime variable to display a backtrace
```

---

### 5. Using `--` Separator (Single Argument)

```bash
cargo run -- hello.txt
```

**Output:**
```
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/minigrep hello.txt`

thread 'main' (1372621) panicked at src/main.rs:7:34:
index out of bounds: the len is 2 but the index is 2
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

---

### 6. Correct Usage with Two Arguments

```bash
cargo run -- hello example.txt
```

**Output:**
```
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/minigrep hello example.txt`
Searching for hello
In the file example.txt
```

---

### 7. Search in Different File

```bash
cargo run test sample.txt
```

**Output:**
```
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/minigrep test sample.txt`
Searching for test
In the file sample.txt
```

---

### 8. Search in Poem File

```bash
cargo run the poem.txt
```

**Output:**
```
   Compiling minigrep v0.1.0 (/home/valkyrie/Documents/Programming/RustProgramming/minigrep)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/minigrep the poem.txt`
Searching for the
In the file poem.txt
```

---

### 9. Displaying File Contents

```bash
cargo run the poem.txt
```

**Output:**
```
   Compiling minigrep v0.1.0 (/home/valkyrie/Documents/Programming/RustProgramming/minigrep)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/minigrep the poem.txt`
Searching for the
In the file poem.txt
With text:
I wandered lonely as a cloud
That floats on high o'er vales and hills,
When all at once I saw a crowd,
A host, of golden daffodils;
Beside the lake, beneath the trees,
Fluttering and dancing in the breeze.

Continuous as the stars that shine
And twinkle on the milky way,
They stretched in never-ending line
Along the margin of a bay:
Ten thousand saw I at a glance,
Tossing their heads in sprightly dance.

The waves beside them danced; but they
Out-did the sparkling waves in glee:
A poet could not but be gay,
In such a jocund company:
I gazed—and gazed—but little thought
What wealth the show to me had brought:

For oft, when on my couch I lie
In vacant or in pensive mood,
They flash upon that inward eye
Which is the bliss of solitude;
And then my heart with pleasure fills,
And dances with the daffodils.
```

---

### 10. Multiple Executions of Same Command

```bash
cargo run the poem.txt
```

**Output:**
```
   Compiling minigrep v0.1.0 (/home/valkyrie/Documents/Programming/RustProgramming/minigrep)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/minigrep the poem.txt`
Searching for the
In the file poem.txt
With text:

I wandered lonely as a cloud
That floats on high o'er vales and hills,
When all at once I saw a crowd,
A host, of golden daffodils;
Beside the lake, beneath the trees,
Fluttering and dancing in the breeze.

Continuous as the stars that shine
And twinkle on the milky way,
They stretched in never-ending line
Along the margin of a bay:
Ten thousand saw I at a glance,
Tossing their heads in sprightly dance.

The waves beside them danced; but they
Out-did the sparkling waves in glee:
A poet could not but be gay,
In such a jocund company:
I gazed—and gazed—but little thought
What wealth the show to me had brought:

For oft, when on my couch I lie
In vacant or in pensive mood,
They flash upon that inward eye
Which is the bliss of solitude;
And then my heart with pleasure fills,
And dances with the daffodils.
```

---

### 11. Single Argument Panic

```bash
cargo run poem.txt
```

**Output:**
```
   Compiling minigrep v0.1.0 (/home/valkyrie/Documents/Programming/RustProgramming/minigrep)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/minigrep poem.txt`

thread 'main' (1431756) panicked at src/main.rs:28:9:
not enough arguments
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

---

### 12. No Arguments with Error Handling

```bash
cargo run
```

**Output:**
```
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/minigrep`

thread 'main' (1432095) panicked at src/main.rs:28:9:
not enough arguments
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

---

### 13. Graceful Error Handling

```bash
cargo run
```

**Output:**
```
   Compiling minigrep v0.1.0 (/home/valkyrie/Documents/Programming/RustProgramming/minigrep)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/minigrep`
Problem parsing arguments: not enough arguments
```

---

### 14. Running Tests - Initial Warnings

```bash
cargo test
```

**Output:**
```
   Compiling minigrep v0.1.0 (/home/valkyrie/Documents/Programming/RustProgramming/minigrep)
warning: unused variable: `query`
  --> src/lib.rs:40:19
   |
40 | pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
   |                   ^^^^^ help: if this is intentional, prefix it with an underscore: `_query`
   |
   = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: unused variable: `contents`
  --> src/lib.rs:40:32
   |
40 | pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
   |                                ^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_contents`

warning: field `query` is never read
 --> src/lib.rs:6:5
  |
5 | pub struct Config {
  |            ------ field in this struct
6 |     query: String,
  |     ^^^^^
  |
  = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: `minigrep` (lib) generated 3 warnings (run `cargo fix --lib -p minigrep` to apply 2 suggestions)
warning: `minigrep` (lib test) generated 3 warnings (3 duplicates)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.16s
     Running unittests src/lib.rs (target/debug/deps/minigrep-8797b0ed3c0f2d7b)

running 1 test
test test::one_result ... FAILED

failures:

---- test::one_result stdout ----

thread 'test::one_result' (1519278) panicked at src/lib.rs:67:9:
assertion `left == right` failed
  left: ["safe, fast, productive"]
 right: []
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    test::one_result

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--lib`
```

---

### 15. Test Failure - Whitespace Mismatch

```bash
cargo test
```

**Output:**
```
   Compiling minigrep v0.1.0 (/home/valkyrie/Documents/Programming/RustProgramming/minigrep)
warning: field `query` is never read
 --> src/lib.rs:6:5
  |
5 | pub struct Config {
  |            ------ field in this struct
6 |     query: String,
  |     ^^^^^
  |
  = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: `minigrep` (lib) generated 1 warning
warning: `minigrep` (lib test) generated 1 warning (1 duplicate)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.17s
     Running unittests src/lib.rs (target/debug/deps/minigrep-8797b0ed3c0f2d7b)

running 1 test
test test::one_result ... FAILED

failures:

---- test::one_result stdout ----

thread 'test::one_result' (1520756) panicked at src/lib.rs:67:9:
assertion `left == right` failed
  left: ["safe, fast, productive"]
 right: ["        safe, fast, productive."]
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    test::one_result

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--lib`
```

---

### 16. Test Failure - Period Mismatch

```bash
cargo test
```

**Output:**
```
   Compiling minigrep v0.1.0 (/home/valkyrie/Documents/Programming/RustProgramming/minigrep)
warning: field `query` is never read
 --> src/lib.rs:6:5
  |
5 | pub struct Config {
  |            ------ field in this struct
6 |     query: String,
  |     ^^^^^
  |
  = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: `minigrep` (lib) generated 1 warning
warning: `minigrep` (lib test) generated 1 warning (1 duplicate)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running unittests src/lib.rs (target/debug/deps/minigrep-8797b0ed3c0f2d7b)

running 1 test
test test::one_result ... FAILED

failures:

---- test::one_result stdout ----

thread 'test::one_result' (1522091) panicked at src/lib.rs:67:9:
assertion `left == right` failed
  left: ["safe, fast, productive."]
 right: ["        safe, fast, productive."]
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    test::one_result

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--lib`
```

---

### 17. Successful Test

```bash
cargo test
```

**Output:**
```
   Compiling minigrep v0.1.0 (/home/valkyrie/Documents/Programming/RustProgramming/minigrep)
warning: field `query` is never read
 --> src/lib.rs:6:5
  |
5 | pub struct Config {
  |            ------ field in this struct
6 |     query: String,
  |     ^^^^^
  |
  = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: `minigrep` (lib) generated 1 warning
warning: `minigrep` (lib test) generated 1 warning (1 duplicate)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running unittests src/lib.rs (target/debug/deps/minigrep-8797b0ed3c0f2d7b)

running 1 test
test test::one_result ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/minigrep-aee88643fc7c39b0)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests minigrep

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

---

### 18. Search with "Along"

```bash
cargo run Along poem.txt
```

**Output:**
```
   Compiling minigrep v0.1.0 (/home/valkyrie/Documents/Programming/RustProgramming/minigrep)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.18s
     Running `target/debug/minigrep Along poem.txt`
Along the margin of a bay:
```

---

### 19. Missing Function Error

```bash
cargo test
```

**Output:**
```
   Compiling minigrep v0.1.0 (/home/valkyrie/Documents/Programming/RustProgramming/minigrep)
error[E0425]: cannot find function `search_insensitive` in this scope
  --> src/lib.rs:91:48
   |
91 |         assert_eq!(vec!["Rust:", "Trust me."], search_insensitive(query, contents));
   |                                                ^^^^^^^^^^^^^^^^^^ not found in this scope

For more information about this error, try `rustc --explain E0425`.
error: could not compile `minigrep` (lib test) due to 1 previous error
warning: build failed, waiting for other jobs to finish...
```

---

### 20. Multiple Test Cases Pass

```bash
cargo test
```

**Output:**
```
   Compiling minigrep v0.1.0 (/home/valkyrie/Documents/Programming/RustProgramming/minigrep)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running unittests src/lib.rs (target/debug/deps/minigrep-8797b0ed3c0f2d7b)

running 2 tests
test test::case_insensitive ... ok
test test::case_sensitive ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/minigrep-aee88643fc7c39b0)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests minigrep

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

---

### 21. Run with Case-Sensitive Search

```bash
cargo run Along poem.txt
```

**Output:**
```
   Compiling minigrep v0.1.0 (/home/valkyrie/Documents/Programming/RustProgramming/minigrep)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.18s
     Running `target/debug/minigrep Along poem.txt`
Along the margin of a bay:
```

---

### 22. Unused Variable Warning

```bash
cargo run Along poem.txt
```

**Output:**
```
   Compiling minigrep v0.1.0 (/home/valkyrie/Documents/Programming/RustProgramming/minigrep)
warning: unused variable: `result`
  --> src/lib.rs:68:9
   |
68 |     let result: Vec<&str> = if config.case_sensitive {
   |         ^^^^^^ help: if this is intentional, prefix it with an underscore: `_result`
   |
   = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: `minigrep` (lib) generated 1 warning (run `cargo fix --lib -p minigrep` to apply 1 suggestion)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.15s
     Running `target/debug/minigrep Along poem.txt`
Along the margin of a bay:
```

---

### 23. Case-Insensitive Search with Environment Variable

```bash
CASE_INSENSITIVE=1 cargo run to poem.txt
```

**Output:**
```
warning: unused variable: `result`
  --> src/lib.rs:68:9
   |
68 |     let result: Vec<&str> = if config.case_sensitive {
   |         ^^^^^^ help: if this is intentional, prefix it with an underscore: `_result`
   |
   = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: `minigrep` (lib) generated 1 warning (run `cargo fix --lib -p minigrep` to apply 1 suggestion)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/minigrep to poem.txt`
What wealth the show to me had brought:
```

---

### 24. Case-Insensitive Search with Capital Letter

```bash
CASE_INSENSITIVE=1 cargo run Along poem.txt
```

**Output:**
```
warning: unused variable: `result`
  --> src/lib.rs:68:9
   |
68 |     let result: Vec<&str> = if config.case_sensitive {
   |         ^^^^^^ help: if this is intentional, prefix it with an underscore: `_result`
   |
   = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: `minigrep` (lib) generated 1 warning (run `cargo fix --lib -p minigrep` to apply 1 suggestion)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/minigrep Along poem.txt`
Along the margin of a bay:
```

---

### 25. Case-Insensitive Search with Lowercase

```bash
CASE_INSENSITIVE=1 cargo run along poem.txt
```

**Output:**
```
warning: unused variable: `result`
  --> src/lib.rs:68:9
   |
68 |     let result: Vec<&str> = if config.case_sensitive {
   |         ^^^^^^ help: if this is intentional, prefix it with an underscore: `_result`
   |
   = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: `minigrep` (lib) generated 1 warning (run `cargo fix --lib -p minigrep` to apply 1 suggestion)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/minigrep along poem.txt`
```

---

### 26. Case-Insensitive Search (Compiled Version)

```bash
CASE_INSENSITIVE=1 cargo run along poem.txt
```

**Output:**
```
   Compiling minigrep v0.1.0 (/home/valkyrie/Documents/Programming/RustProgramming/minigrep)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s
     Running `target/debug/minigrep along poem.txt`
Along the margin of a bay:
```

---

### 27. Case-Sensitive Search (No Environment Variable)

```bash
cargo run along poem.txt
```

**Output:**
```
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/minigrep along poem.txt`
```

---

### 28. Redirecting Output to File (No Arguments)

```bash
cargo run > output.txt
```

**Output:**
```
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/minigrep`
```

---

### 29. Redirecting Output to File (With Compilation)

```bash
cargo run > output.txt
```

**Output:**
```
   Compiling minigrep v0.1.0 (/home/valkyrie/Documents/Programming/RustProgramming/minigrep)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/minigrep`
Problem parsing arguments: not enough arguments
```

---

### 30. Redirecting Search Results to File

```bash
cargo run Along poem.txt > output.txt
```

**Output:**
```
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/minigrep Along poem.txt`
```

---

## Summary of Key Behaviors

### Successful Executions
- Requires exactly 2 arguments (search term + filename)
- Displays search results from file
- Supports case-sensitive and case-insensitive modes
- Case-insensitive mode controlled by `CASE_INSENSITIVE` environment variable

### Error Cases
- Less than 2 arguments: Panics with "not enough arguments"
- Missing file: File I/O error
- Gracefully handled errors display as "Problem parsing arguments: [error message]"

### Testing
- Tests check both case-sensitive and case-insensitive searches
- Whitespace and exact text matching in test assertions
- All tests pass when properly configured