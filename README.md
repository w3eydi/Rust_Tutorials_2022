# Rust Programming Language Tutorials 2022

## Content List

### - Beginner

- [x] #0.m - rust environment setup | macOS
- [x] #0.l - rust environment setup | linux
- [x] #0.w - rust environment setup | windows
- [x] #1.m - rust compiler introduction on macOs | Hello, rust!
- [x] #1.l - rust compiler introduction on linux | Hello, rust!
- [x] #1.w - rust compiler introduction on windows | Hello, rust!
- [ ] #2 - rust compiler detailed explanation | Hello, compiler!
- [x] #3 - basic cargo commands | Hello, cargo!
- [x] #4 - vscode and pycharm plugins
- [ ] #5 - rustup doc
- [x] #6 - rustup update
- [x] #7 - variables -> [variables](video_code_examples/variables/src/main.rs)
- [x] #8 - variable naming conventions
- [ ] #9 - print and println macros
- [x] #10 - rust comments -> [comments](video_code_examples/comments/src/main.rs)
- [x] #11 - #[allow] attribute -> [allow](video_code_examples/allow/src/main.rs)
- [ ] #12 - data types | integers
- [x] #13 - number systems | binary -> [binary](video_code_examples/binary/src/main.rs)
- [x] #14 - number systems | hexadecimal -> [hexadecimal](video_code_examples/hexadecimal/src/main.rs)
- [x] #15 - number systems | exponent notation -> [exponent](video_code_examples/exponent/src/main.rs)
- [x] #16 - arithmetic operators -> [arithmetic](video_code_examples/arithmetic/src/main.rs)
- [ ] #17 - overflow
- [x] #18 - data types | boolean
- [x] #19 - comparison operators
- [x] #20 - data types | floats
- [x] #21 - data types | chars -> [chars](video_code_examples/chars/src/main.rs)
- [x] #22 - type casting operator | as -> [as operator](video_code_examples/as_operator/src/main.rs)
- [x] #23 - compound data types | arrays -> [arrays](video_code_examples/arrays/src/main.rs)
- [x] #24 - compound data types | tuples -> [tuples](video_code_examples/tuples/src/main.rs)
- [x] #25 - &str(string literal) and String -> [str and string](video_code_examples/str_and_string/src/main.rs)
- [x] #26 - string slices -> [string slices](video_code_examples/string_slices/src/main.rs)
- [x] #27 - raw string literals -> [raw string literals](video_code_examples/raw_string_literal/src/main.rs)
- [x] #28 - scope -> [scope](video_code_examples/scope/src/main.rs)
- [x] #29 - shadowing -> [shadowing](video_code_examples/shadowing/src/main.rs)
- [x] #30 - logical operators -> [logical](video_code_examples/logical/src/main.rs)
- [x] #31 - bitwise operators -> [bitwise](video_code_examples/bitwise/src/main.rs)
- [x] #32 - compound assignment operators -> [compound assignment](video_code_examples/compound_assignment/src/main.rs)
- [x] #33 - control flow | if/else -> [if/else](video_code_examples/if_else/src/main.rs)
- [x] #34 - control flow | match -> [match](video_code_examples/match_example/src/main.rs)
- [x] #35 - infinite loop -> [infinite loop](video_code_examples/infinite_loop/src/main.rs)
- [x] #36 - break --> [break keyword](video_code_examples/break_keyword/src/main.rs)
- [x] #37 - continue --> [continue keyword](video_code_examples/continue_keyword/src/main.rs)
- [x] #38 - while loop --> [while loop](video_code_examples/while_loop/src/main.rs)
- [ ] #39 - for loop
<!--
- [ ] let ile loop, match değişken oluşturma
- [ ] for loop Strings string literal
// https://www.educative.io/answers/what-is-stringchars-in-rust -->
- [ ] #40 - functions
- [ ] #41 - functions shadowing

- [ ] reference
- [ ] dereference 
### - Intermediate
<!-- linked learn 6.7-->
- [ ] #42 - basic references ?? - with functions borrowing
<!-- // belki 42 ye aktarılır ownership başlanabilir -->
- [ ] ownership
<!--
https://www.tutorialspoint.com/rust/rust_file_input_output.htm
-->     
- [ ] borrowing
- [ ] #43 - closures
- [ ] #44 - libraries
- [ ] #45 - input

- [ ] #50 - struct 
- [ ] # ?? - struct update :: 10.2 linked
- [ ] #?? - struct - pub ::hadi yeni bir yöntemle struct tanımlayalım
- [ ] #?? - methods impl - 10.4 associated function new Self keywordle de tanımla      
- [ ] #?? impl - yazdir() get_name()
- [ ] #?? - tuple structs
- [ ] #?? - generic 
- [ ] #?? - generic type safe vektör örneği
        <!-- https://www.tutorialspoint.com/rust/rust_generic_types.htm -->
- [ ] #?? - generic method <T, U> ve <u8, u8>
- [ ] #?? - generic partialOrd
- [ ] #?? - trait implementasyonu
- [ ] #?? - trait - Debug formatting & Display Traits
- [ ] #?? - trait - Default value
- [ ] #?? - trait - partialEq
- [ ] #?? - trait derivable
- [ ] #?? - trait - static
- [ ] #?? - trait - &dyn ?? dynamic dispact
     <!--aynı video olabilir -   trait - dyn shortway
            aynı -  trait - dyn impl keyword shortway -->
        
- [ ] #?? - MODULES
        
- [ ] #?? - lifetimes
- [ ] #?? - constants | const, static
- [ ] #?? - enums
- [ ] #?? - enum function
- [ ] #?? - options <!-- https://www.linkedin.com/learning/rust-essential-training/3145782?autoSkip=true&autoplay=true&resume=false   matching Option<T>
        -->
- [ ] #?? - if let
- [ ] #?? - while let
- [ ] #?? - results
- [ ] #?? - kind error handling
- [ ] #?? - panic!
- [ ] #?? - unwrap()
- [ ] #?? - expect()
- [ ] #?? - new data types collections
                -vectors 
                -hashmaps
                -btree
- [ ] #?? - ITERATOR
                Fn .iter
                FnMut .iter_mut()
                FnOnce .into_iter()
- [ ] #?? - COLLECT

### - Advanced
- [ ] #?? - reference types
- [ ] #?? - smart pointers
- [ ] #?? - box
- [ ] #?? - CustomBox<!-- https://www.tutorialspoint.com/rust/rust_smart_pointers.htm -->
- [ ] #?? - interior mutability
  - [ ] #?? - cell
  - [ ] #?? - refcell
  - [ ] #?? - mutex
  - [ ] #?? - rwLock
- [ ] #?? - rc
- [ ] #?? - arc
- [ ] async
        channel (sync, send)
- [ ] tokio
- [ ] #?? - macros