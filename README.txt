# rust-rocket-workshop

* references
    * https://doc.rust-lang.org/
    * https://rust-unofficial.github.io/too-many-lists/
    * https://www.reddit.com/r/rust/comments/wo46dz/does_using_string_instead_of_str_a_lot_results_in/
    * https://www.reddit.com/r/rust/comments/cyymw2/rule_of_thumb_for_struct_data_types/
    * https://www.reddit.com/r/rust/comments/4ltwov/shortlived_struct_string_or_a_str/
    * https://stackoverflow.com/questions/57562632/why-is-impl-needed-when-passing-traits-as-function-parameters
    * https://www.reddit.com/r/rust/comments/12nhpvz/how_can_a_parameter_type_t_be_not_long_living/
    * https://chat.openai.com/
    * https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=ef5675b5d78490c1fb440c229cc5d129
    * https://stackoverflow.com/questions/31949579/understanding-and-relationship-between-box-ref-and
    * https://users.rust-lang.org/t/box-with-a-trait-object-requires-static-lifetime/35261
    * https://stackoverflow.com/questions/24158114/what-are-the-differences-between-rusts-string-and-str
    * https://chrismorgan.info/blog/rust-fizzbuzz/
    * https://stackoverflow.com/questions/31012923/what-is-the-difference-between-copy-and-clone
    * https://stackoverflow.com/questions/65434252/how-to-return-a-reference-to-a-value-from-hashmap-wrappered-in-arc-and-mutex-in
    * https://www.reddit.com/r/rust/comments/17luh6c/how_can_i_avoid_cloning_everywhere/
    * https://www.reddit.com/r/rust/comments/vy9zvw/the_docs_say_hashmap_is_send_sync_how_can_that_be/
    * https://stackoverflow.com/questions/26469715/how-do-i-write-a-rust-unit-test-that-ensures-that-a-panic-has-occurred
    * https://www.reddit.com/r/rust/comments/ui7ayd/why_does_rust_not_have_a_standard_async_runtime/
    * https://rustjobs.dev/blog/difference-between-string-and-str-in-rust/
    * https://users.rust-lang.org/t/whats-the-difference-between-string-and-str/10177/2
    * http://xion.io/post/code/rust-patterns-ref.html
    * https://github.com/pretzelhammer/rust-blog/blob/master/posts/common-rust-lifetime-misconceptions.md
    * https://www.cs.brandeis.edu/~cs146a/rust/doc-02-21-2015/book/static-and-dynamic-dispatch.html
    * https://dhghomon.github.io/easy_rust
    * https://stackoverflow.com/questions/49377231/when-to-use-rc-vs-box
    * https://stackoverflow.com/questions/31012923/what-is-the-difference-between-copy-and-clone
    * https://stackoverflow.com/questions/31168589/how-to-force-a-move-of-a-type-which-implements-the-copy-trait
    * https://users.rust-lang.org/t/behind-the-scenes-how-does-rust-move-structs/99229/2
    * https://www.reddit.com/r/rust/comments/ykku69/does_the_compiler_optimize_moves/
    * https://www.reddit.com/r/rust/comments/f6urwh/why_move_semantics_for_value_types/
    * https://stackoverflow.com/questions/29490670/how-does-rust-provide-move-semantics
    * https://stackoverflow.com/questions/59628211/what-happens-in-memory-when-ownership-is-transferred-out-of-a-box
    * https://stackoverflow.com/questions/53465843/why-does-move-in-rust-not-actually-move
    * https://stackoverflow.com/questions/30288782/what-are-move-semantics-in-rust
    * https://www.reddit.com/r/rust/comments/f6urwh/why_move_semantics_for_value_types/
    * https://www.reddit.com/r/rust/comments/rlzhy1/rust_is_creating_copy_instead_of_moving/
    * https://www.reddit.com/r/rust/comments/qw7wxx/why_does_mean_both_copy_and_move_where_are_the/
    * https://www.reddit.com/r/rust/comments/srtuhy/when_a_move_occurs_what_happens_behind_the_scenes/
    * https://users.rust-lang.org/t/sync-but-not-send/21551/5
    * https://www.reddit.com/r/rust/comments/iuespp/question_mark_operator_implicit_conversion_why/
    * https://web.mit.edu/rust-lang_v1.25/
    * https://nnethercote.github.io/2021/12/08/a-brutally-effective-hash-function-in-rust.html
    * https://medium.com/@luishrsoares/exploring-hash-functions-in-rust-fowler-noll-vo-fnv-siphash-and-beyond-63183a4d7de
    * https://stackoverflow.com/questions/55128808/when-is-it-appropriate-to-require-only-partialeq-and-not-eq
    * https://doc.rust-lang.org/book/appendix-03-derivable-traits.html#partialeq-and-eq-for-equality-comparisons
    * https://www.reddit.com/r/rust/comments/11gm19h/why_eq_partialeq_ord_and_partialord_especially/
    * https://stackoverflow.com/questions/61293115/how-can-a-struct-be-unsized
    * https://medium.com/tips-for-rust-developers/pin-276bed513fd1
    * https://www.reddit.com/r/rust/comments/eo7u4o/futures_pinning_101/
    * https://www.reddit.com/r/rust/comments/pbemse/pin_unpin_and_why_rust_needs_them_blogexplainer/
    * https://www.reddit.com/r/rust/comments/f55ur9/fromstr_vs_fromstr_vs_fromstring_which_should_i/
    * https://stackoverflow.com/questions/67385956/what-is-the-difference-between-the-fromstr-and-tryfromstring-traits
    * https://stackoverflow.com/questions/71487308/is-there-any-real-difference-between-fromstr-and-tryfromstr
    * https://levelup.gitconnected.com/rust-unit-structs-explained-4ad2307efa72
    * https://stackoverflow.com/questions/67689613/what-is-a-real-world-example-of-using-a-unit-struct
    * https://medium.com/@verbruggenjesse/rust-using-rustlings-part-6-structs-b1f3be7c2cbc
    * https://www.reddit.com/r/rust/comments/ny44z6/how_would_you_further_organize_the_project/
    * https://stackoverflow.com/questions/57756927/rust-modules-confusion-when-there-is-main-rs-and-lib-rs
    * https://stackoverflow.com/questions/30177395/when-does-a-closure-implement-fn-fnmut-and-fnonce
    * https://stackoverflow.com/questions/50135871/how-can-a-closure-using-the-move-keyword-create-a-fnmut-closure
    * https://users.rust-lang.org/t/fnonce-closure-can-be-called-multiple-times/53790/6
    * https://stackoverflow.com/questions/51698648/why-is-the-move-keyword-not-always-needed-even-when-the-closure-takes-ownership
    * https://www.reddit.com/r/rust/comments/axf137/are_fat_pointers_and_smart_pointers_same_in_rust/
    * https://stackoverflow.com/questions/57754901/what-is-a-fat-pointer
    * https://docs.rs/thiserror/latest/thiserror/
    * https://docs.rs/mockall/latest/mockall/
    * https://github.com/asomers/mockall
    * https://rocket.rs/v0.5/guide/
    * https://api.rocket.rs/v0.5/rocket
    * https://github.com/rwf2/Rocket/issues/1736
    * https://stackoverflow.com/questions/76965631/how-do-i-spawn-possibly-blocking-async-tasks-in-tokio
    * https://github.com/rwf2/Rocket/issues/53#issuecomment-277149045
    * https://docs.rs/figment/0.10.13/figment/
    * https://www.reddit.com/r/rust/comments/lvtzri/confused_about_package_vs_crate_terminology/
    * https://stackoverflow.com/questions/52024304/what-exactly-is-a-crate-in-the-cargo-ecosystem-and-what-is-the-mapping-to-what
    * https://mmapped.blog/posts/03-rust-packages-crates-modules.html

## rust
* statically typed language
* makes memory safety guarantees without needing a garbage collector
* by default allocate on stack
    * accessing data in the heap is slower than accessing data on the stack
        * you have to follow a pointer to get there
    * pushing to the stack is faster than allocating on the heap
        * allocator never has to search for a place to store new data
            * that location is always at the top of the stack
* uses snake case

## ownership and borrowing
* ownership
    * a set of rules that govern how a Rust program manages memory
        * rules
            * each value in Rust has an owner
            * there can only be one owner at a time
            * when the owner goes out of scope, the value will be dropped
                * scope = range within a program for which an item is valid
        * called system of ownership
* references and borrowing
    * reference = address where the data is stored
        * vs pointer
            * compiler will ensure that the data will not go out of scope before the reference to the data does
                * references will never be dangling references
                    * example: return a reference to a value from HashMap protected by Mutex
                        ```
                        fn get(&self, key:&String)-> Option[&String] {
                            self.a.lock()
                                .expect("opening lock should not fail")
                                .get(key) // compilation error: returns a value referencing data owned by the current function
                        }

                        fn delete(&self, key: &String) { // no need to `mut self` due to Mutex
                                let mut map = self.a.lock().unwrap();
                                map.remove(key);
                        }

                        let r = hash_map_mutex_wrapped.get("a");
                        hash_map_mutex_wrapped.delete("a");
                        println!("Value: {}", r); // dangling reference
                        ```
                * every value has a single owner that determines its lifetime
                    * example: variable owns its value
                    * in Java objects never physically contain other objects in Java
                    * when control leaves the block in which the owner is declared: owner freed—dropped => owned value is dropped too
            * for pointer it's not always true
                * example: `NullPointerException`
        * data is owned by some other variable
            * borrowing = action of creating a reference
            * references are immutable by default
                * cannot modify something we borrowed
                * you can make them mutable by adding `mut` in front of the variable name
        * mutable reference
            * restriction: if you have a mutable reference to a value, you can have no other references to that value
                * race condition happens when these three behaviors occur
                    1. two or more pointers access the same data at the same time
                    1. at least one of the pointers is being used to write to the data
                    1. there’s no mechanism being used to synchronize access to the data
                * way to prevent data races at compile time
    * borrow system can’t protect you from deadlock
        * best protection is to keep critical sections small

## cargo
* package vs crate vs module
    * crate
        * `rustc` compiles one crate at a time
        * form the atomic compilation unit of the Rust compiler
        * output artifact of the compiler
        * two forms
            * binary crate
                * programs you can compile to an executable
                * have a function `main` that defines what happens when the executable runs
                * example: command-line program, server
            * library crate
                * doesn't have a main function
                * don’t compile to an executable
                * define functionality intended to be shared with multiple projects
            * `src/main.rs` and `src/lib.rs` are called crate roots
                * `main.rs` is binary
                    * handles running the program
                * `lib.rs` is library
                    * handles all the logic
                * contents of either of these two files form a module named `crate` at the root
                of the crate’s module structure
                * usual workflow: binary as a thin wrapper around the library
                    * example
                        ```
                        fn main() {
                            library::main(); // binary crate becomes a user of the library crate
                        }
                        ```
        * imports
            * example
                ```
                use crate::common::wrapper; // crate refers to src/lib.rs
                use std::mem::swap;

                fn main() {
                    wrapper(swap(&mut a, &mut b));
                }
                ```
    * package
        * `cargo build` builds your whole package
        * packages are aggregates of crates that share a single Cargo.toml file
            * example: lib crate and bin crate
        * artifact managed by Cargo
        * Cargo book uses the term crate as an alias for package
            * generally the main artifact of a package is a library crate and since
            it is identified with the package name it is customary to treat package and crate as synonyms
    * module
        * unit of code organization
        * Rust’s namespaces
        * container for functions, types, and nested modules
        * defined using two equivalent approaches
            * modules in their own file
                * example
                    ```
                    mod customer {

                    }
                    ```
            * modules in their own directory with a `mod.rs`
                * example
                    * file `customer.rs`
                    * `mod.rs`
                        ```
                        pub mod customer;
                        ```
            * example: when Rust sees `mod customer;`, it checks for both
                * `customer.rs` and `customer/mod.rs`
                    * if neither file exists, or both exist, that’s an error
        * `use` = bring symbols (such as functions, structs, enums, and modules) into scope
            * analogy: filesystem’s directory tree
                * we use a path in the same way we use a path when navigating a filesystem
                * `use` and a `path` in a scope is similar to creating a symbolic link in the filesystem
            * example
                ```
                use std::mem::swap;

                fn main() {
                    swap(&mut a, &mut b);
                }
                ```
* is package manager & build tool
     * vs sbt
         * sbt = treat configuration as a code
         * cargo = treat configuration as data
* is command-line tool
    * useful commands
        * `cargo add` = add dependencies to a Cargo.toml
        * `cargo build` = produces the executable or library specified in your project's configuration
            * has two main profiles
                * `dev` - uses when you run `cargo build`
                * `release` - uses when you run `cargo build --release`
                    * compile it with optimizations
                    * command will create an executable in `target/release` instead of `target/debug`
            * stores downloaded build dependencies in the Cargo home
                * default: `$HOME/.cargo/`
        * `cargo run` = builds and then runs project
            * entry point to be used is `main.rs`
        * `cargo test` = executes the tests in your project
        * `cargo check` = check project for errors and warnings without producing executable
        * `cargo clean` = remove generated artifacts
        * `cargo tree` = tree-like representation of dependencies
            * including transitive dependencies (dependencies of dependencies)
            * useful flags
                * `--invert` = invert the tree and display the packages that depend on the given package
                * `--duplicates` = show only dependencies which come in multiple versions
                    * useful in investigations if the package that depends on the duplicate with the older version can be updated to the newer version so that only one instance is built.
                * `--edges` = dependency kinds to display
                    * example: all, build, dev, etc
        * `cargo update` = update dependencies as recorded in the local lock file
        * `cargo install` = install binaries from crates.io
            * `crates.io` = community’s site for open source crates
            * installs the specified binary crate globally on your system
                * typically, `$HOME/.cargo/bin`
            * used for installing command-line tools or utilities
            * `cargo-edit` package provides additional Cargo commands for managing dependencies
                * `cargo add <dependency-name>` = add a new dependency to Cargo.toml
                * `cargo rm <dependency-name>` = remove a dependency
* `Cargo.toml`
    * configuration file
        * resides at the root of project
    * contains metadata about project
        * example: dependencies, build settings, etc
    * TOML (Tom's Obvious Minimal Language)
        * data serialization language
        * organized into key-value pairs
    * loose interpretation of version specifications
        * Cargo looks for the most recent version of the image crate that is considered compatible with version
        * reason: allowing Cargo to use any compatible version is a much more practical default
            * otherwise it would lead to situations where projects couldn't use multiple libraries with slightly different versions of shared dependencies
        * example: `1.2.3  :=  >=1.2.3, <2.0.0`
    * example
        ```
        [package]
        name = "my_project"
        version = "0.1.0"
        edition = "2021" // rust edition represents different releases of the Rust programming language

        [dependencies]
        uuid = { version = "1.6.1", features = ["v4"] } // "1.6.1" is shortcut for "^1.6.1"

        [build-dependencies] // used only during the build process (e.g., build scripts)

        [dev-dependencies] // used only for testing and development
        ```
        * Rust editions
            * example
                * 2015 edition was the first stable release
                * 2018 edition changed `async` and `await` into keywords, streamlined the module system, and introduced
                various other language changes
                    * incompatible with the 2015 edition
            * used to evolve without breaking existing code
                * are backward-compatible
                    * code written in earlier editions should still compile and work in newer editions
            * programs can freely mix crates written in different editions
                * crate’s edition only affects how its source code is construed
                    * edition distinctions are gone by the time the code has been compiled
                    * there’s no pressure to update old crates just to continue to participate in the modern Rust ecosystem
                * example: fine for a 2015 edition crate to depend on a 2018 edition crate
* `Cargo.lock`
    * example
        ```
        [[package]]
        name = "uuid"
        version = "1.6.1"
        source = "registry+https://github.com/rust-lang/crates.io-index"
        checksum = "5e395fcf16a7a3d8127ec99782007af141946b4795001f876d54fb0d55978560"
        dependencies = [
         "getrandom",
        ]
        ```
    * records the exact version of every crate it used
        * generated during first time build
    * builds consult this file and continue to use the same versions
        * Cargo should not upgrade to the latest library versions every time we build
            * version numbers are deliberately flexible
    * should commit?
        * project is an executable => commit
            * everyone will consistently get the same versions
        * library => don't make much sense to commit
            * downstream users will have `Cargo.lock` files that contain version information
            for their entire dependency graph
                * library’s `Cargo.lock` file will be ignored

## structs
* three kinds
    * named-field
        ```
        struct Person {
            first_name: String,
            last_name: String,
        }
        ```
    * tuple-like
        ```
        struct Age(u32);

        impl Age {
            fn new(age: u32) -> Option<Self> {
                if age >= 0 {
                    Some(Self(age))
                } else {
                    None
                }
            }
        }
        ```
        * good for newtypes (structs with a single component)
            * https://github.com/greyblake/nutype
    * unit-like
        * example: `std::fmt::Error`
            ```
            pub struct Error
            ```
        * can serve as a marker type
        * basis for stateless trait implementation
        * library may ask you to create a structure that implements a certain trait to handle events
            * example
                ```
                // lib
                trait EventHandler {
                    fn handle_event(&self, event: &str);
                }

                // your code
                struct LogHandler;

                impl EventHandler for LogHandler {
                    fn handle_event(&self, event: &str) {
                        // Perform logging logic here
                        println!("Log: {}", event);
                    }
                }
                ```
        * used in enums as parameterless variant
            * example
                ```
                enum Animal {
                    Cat
                }
                ```
        * occupies no memory, much like the unit type ()
* does not support field mutability at the language level
    * mutability is a property of the binding, not of the structure itself
        * example
            ```
            struct Point {
                mut x: i32, // not compile
                y: i32,
            }
            ```

            ```
            let mut point = Point { x: 0, y: 0 };

            point.x = 5;
            ```
* contain data, but can also have logic
    * if contains references, references’ lifetimes must be specified
        * lifetimes on structs should only be used when the struct is a "view" or "cursor" that looks inside some other struct
            * example: `&str` is easy for read-only function parameters, but it's a pain for structs
                * you should never put the `&str` type in a struct (just default to `String`)
        * rule of thumb: "Does the struct own this data?"
            * yes => `'static` (unborrowed) fields
            * no => you go with references
            * shared ownership => `Rc`/`Weak`/`Arc`
                * if it's possibly shared => `Cow`
    * `impl` is used to tie logic to struct
        * Rust method must explicitly use self to refer to the value it was called on
            * similar to: Python (self), JavaScript (this)
            * not similar to: Java (this is auto-generated by compiler)
        * if a method wants to take ownership of self, it can take self by value
            * method that takes ownership is rare
            * usually used when the method transforms self into something else and you want to prevent the caller
            from using the original instance after the transformation
        * example
            ```
            struct Age(u32);

            impl Age {

                const ZERO: u8 = Age::new(0); // type-associated const

                fn new(age: u32) -> Option<Self> { // no "self", type-associated function - Age::new(5)
                    if age >= 0 {
                        Some(Self(age))
                    } else {
                        None
                    }
                }

                fn value(&self) -> u32 { // method invoked on instance - age.value()
                    self.0
                }
            }
            ```
* accessibility
    * module where the struct is declared, and its submodules: full access
        * even to private fields
    * outside the module, only `pub` fields are accessible
* by default, struct and enum types are not Copy
    * all the fields of struct Copy <=> copy can be derived for struct
* memory
    * both named-field and tuple-like structs are the same thing: collection of
    values laid out in a particular way in memory
    * Rust doesn’t make specific promises about how it will order a struct’s fields or elements in memory
    * Rust does promise to store fields’ values directly in the struct’s block of memory
        * Java would put values each in their own heap-allocated blocks and have struct fields point
        at them
* enums
    * memory
        * tag + enough memory to hold all fields of the largest variant
            * example
                ```
                let circle = Shape::Circle(5.0);
                let rectangle = Shape::Rectangle(3.0, 4.0);
                let triangle = Shape::Triangle(1.0, 1.0, 1.0);

                Circle(5.0):
                +---------------------------+
                |       Variant Tag         |
                +---------------------------+
                |   Associated Data (f64)   |
                |                           |
                |                           |
                +---------------------------+

                Rectangle(3.0, 4.0):
                +---------------------------+
                |       Variant Tag         |
                +---------------------------+
                |   Associated Data (f64)   |
                |   Associated Data (f64)   |
                |                           |
                +---------------------------+

                Triangle(1.0, 1.0, 1.0):
                +---------------------------+
                |       Variant Tag         |
                +---------------------------+
                |   Associated Data (f64)   |
                |   Associated Data (f64)   |
                |   Associated Data (f64)   |
                +---------------------------+
                ```
        * tag field is for Rust’s internal use
            * tells which constructor created the value and therefore which fields it has.
    * useful for quickly implementing tree-like data structures
    * can have methods
    * rust prohibits match expressions that do not cover all possible values
## error handling
* two categories
    * recoverable
        * example: file not found error
    * unrecoverable
        * always symptoms of bugs
        * example: trying to access a location beyond the end of an array
* panic
    * kind of error that should never happen
        * example: integer division by zero
        * like a RuntimeException in Java
    * doesn’t automatically spread from one thread to the threads that depend on it
        * panic in one thread is reported as an error Result in other threads
        * if the panicking thread was the main thread => whole process exits (with a nonzero exit code)
    * behavior is customizable
        * compile with -C panic=abort => first panic in the program immediately aborts the process
    * macro `panic!()` triggers a panic directly
        * a good rule of thumb is: "Don’t panic."
    * unwinding
        * means Rust walks back up the stack and cleans up the data from each function it encounters
        * is a lot of work
        * aborting
            * immediately ends the program without cleaning up
            * configurable
                ```
                [profile.release]
                panic = 'abort'
                ```
        * `std::panic::catch_unwind()`
            * catch stack unwinding
            * allowing the thread to survive and continue running
            * example: mechanism used by Rust’s test harness to recover when an assertion fails in a test
* errors
    * should implement the `std::error::Error` trait
        * can be derived: #[derive(Error)]
* Result
    * equivalent of Either
    * equivalent of try/catch in other languages

## traits
* Rust’s take on interfaces
    * approach inspired by Haskell’s typeclasses
    * dispatch
        * mechanism to determine which specific version is actually run
        * static
            * not something Go or Java have
            * use case: generics
            * perform using monomorphization
                * process of turning generic code into specific code by filling in the concrete types that are used when compiled
                * example
                    ```
                    impl Foo for u8 { ... }
                    impl Foo for String { ... }

                    fn do_something<T: Foo>(x: T) { ... } // compiler will create a special version for both u8 and String, and then replace the call sites
                    // fn some_function(foo: impl Trait) { ... } // equivalent to above, can be handy when one type for param

                    fn main() {
                        let x = 5u8;
                        let y = "Hello".to_string();

                        do_something(x);
                        do_something(y);
                    }
                    ```
                * upsides: allowing for inlining and hence usually higher performance
                * downsides: code bloat due to many copies of the same function existing in the binary, one for each type
        * dynamic
            * use case: trait object
                * trait objects are normal values that store a value of any type that implements the given trait
                    * type can only be known at runtime
                    * example: `&Foo` or `Box<Foo>`
            * upsides: less code bloat (trait object is not specialised to each of the types)
            * downsides
                * slower virtual function calls
                * inhibiting any chance of inlining and related optimisations
* used to add extensions methods to existing types (even built-in like str and bool)
    * example
        ```
        impl<W: Write> WriteHtml for W { ... } // extension trait for Write trait

        impl IsBlank for str { // adds is_blank() method for String and &str
            fn is_blank(&self) -> bool {
                self.trim().is_empty()
            }
        }
        ```
    * the trait itself must be in scope
* memory: fat pointer
    * pointer to the value
    * pointer to a table (vtable) corresponding to the specific implementation of T
        * vtable is essentially a struct of function pointers, pointing to the concrete piece of machine code for each method in the implementation
        * example: `trait_object.method()` will retrieve the correct pointer out of the vtable and then do a dynamic call of it
    * rust automatically converts ordinary references into trait objects when needed
        * example: `Box<File>` to a `Box<dyn Write>`
        * how?
            * Rust knows the referent’s true type , so it just adds the address of the
            appropriate vtable, turning the regular pointer into a fat pointer
* orphan rule
    * when implementing a trait, either the trait or the type must be new in the current crate
    * without the rule, two crates could implement the same trait for the same type, and Rust wouldn’t know which implementation to use
    * example: can't `impl Write for u8`
        * both are defined in the standard library
* `Self` type represents the concrete type implementing that trait
    * example
        ```
        pub trait Clone {
            fn clone(&self) -> Self; // type of `x.clone()` is the same as the type of `x`
        }
        ```
* two ways of using traits to write polymorphic code
    * trait objects
    * generics
* `impl Trait` can be used in three locations
    * as an argument type
        * example
            * `pub fn notify(item: &(impl Summary + Display))`
            * `pub fn notify<T: Summary + Display>(item: &T)`
            * where clause
                ```
                fn some_function<T, U>(t: &T) -> i32
                where
                    T: Summary + Display,
                ```
    * as a return type, but concrete type needs to be known at compile time
        * concrete types of iterators could become very complex
            ```
            fn women_vip<'a>(persons: &'a Vec<Person>) -> impl Iterator<Item = &'a Person> + 'a { // instead of Filter<Iter<'_, Person>, fn(&&'a Person) -> bool>
                persons.iter().filter(|p| p.is_woman())
            }
            ```
        * problem: trait types are unsized and don't have a fixed size known at compile time
            * reason: compiler has to know the type being returned from the function at compile time in order to allocate the right amount of space on the stack
            * example
                ```
                fn make_shape(shape_type: ShapeType) -> impl Shape { // does not compile as we may return Circle, Triangle etc
                    match shape_type {
                        ShapeType::Circle => Circle::new(),
                        ShapeType::Triangle => Triangle::new(),
                        ShapeType::Rectangle => Rectangle::new(),
                    }
                }
                ```
            * solution: box it
    * conditionally implement a trait for any type that implements another trait
        * example: `impl<T: Display> ToString for T`
* useful traits
    * some of them Rust can automatically implement for you with `#[derive]` attribute
        * attributes are metadata about pieces of Rust code
        * example: `#[derive(Copy, Clone, Debug, PartialEq)]`
    * `Clone`
        * deep copy: expensive, in both time and memory
        * some types don’t make sense to copy: Mutex
        * ToOwned
            * But what if you want to
              clone a &str or a &[i32]? What you probably want is a String or a Vec<i32>, but
              Clone’s definition doesn’t permit that: by definition, cloning a &T must always return a
              value of type T, and str and [u8] are unsized; they aren’t even types that a function
              could return.
            * Unlike clone, which must return exactly Self, to_owned can return anything you
              could borrow a &Self from: the Owned type must implement Borrow<Self>.
    * `Copy`
        * represents values that can be safely duplicated via `memcpy`: simply copying the bits in memory
            * derived: `#[derive(Clone, Copy)]`
                * cannot be re-implemented
                * every `Copy` type is also required to be `Clone`
                    * `Copy` is a special case of `Clone` where the implementation is just "copy the bits"
                    * reverse is not true: `Vec<T>`, `String`
            * example: u8
                * you cannot possibly be more efficient with a move, since under the hood it would probably at least entail a pointer copy
                    * it is already as expensive as a u8 copy, so why bother
        * affects how the compiler uses moves (automatic copies)
            * `Copy` types are implicitly **cloned** whenever they're moved
                * but because of the definition of `Copy` this just means not treating the old copy as uninitialized
            * regardless of whether type implements `Copy` trait or not, `let y = x` always does the same thing: `memcpy`
                * example: if you do `f(x)` (and not `f(&x)` or `f(&mut x)` the bytes of x really get copied into f's stack frame, at least in debug builds
                    * it's up to llvm to optimize out this copy in release builds
                    * if you use `&mut self` instead of `self`, you're still telling Rust to move, but you'll be moving the pointer instead
                * may be optimised: it is not necessary for move to `memcpy`
                    * example: https://play.rust-lang.org/ show LLVM IR (intermediate representation)
                        ```
                        fn main() {
                            let s = "Hello, World!".to_string();
                            let t = s;
                            println!("{}", t);
                        }
                        ```
                        LLVM IR (in Debug) in "; playground::main" part contains
                        ```
                        call void @llvm.memcpy.p0.p0.i64(ptr align 8 %t, ptr align 8 %s, i64 24, i1 false), !dbg !2203
                        ```
                        LLVM IR in Release completely elided the copies (realizing that `s` was unused)
                    * example: if the object on the stack is large enough, Rust's compiler may choose to pass the object's pointer instead
            * in these two examples, the only difference is whether you are allowed to access x after the assignment
                ```
                #[derive(Debug, Clone, Copy)]
                pub struct PointCloneAndCopy {
                    pub x: f64,
                }

                #[derive(Debug, Clone)]
                pub struct PointCloneOnly {
                    pub x: f64,
                }

                fn test_copy() {
                    let p1 = PointCloneAndCopy { x: 0. };
                    let p2 = p1; // because type has `Copy`, it gets copied automatically.
                }

                fn test_move() {
                    let p1 = PointCloneOnly { x: 0. };
                    // To avoid the implicit move, we could explicitly call let p2 = p1.clone();
                    let p2 = p1; // because type has no `Copy`, this is a move instead.
                }
                ```
                * note that we need move, otherwise we would have two pointers p1 and p2 that when go out of scope,
                they will both try to free the same memory
        * is a property of the type, not the operation
            * `let a = b` is always a move, doesn't matter whether `b` is `Copy` or not
            *  similarly, a function call `f(b)` always moves `b`
        * bitwise copy must represent a valid and independent duplicate of the original value
            * problem: freeing resource
                * example: `Drop` trait
                    * double deallocation
                        * using `memcpy` would result in two instances pointing to the same memory
                    * don't even have to alias heap data
                        * file handles: file being closed twice
            * problem: exclusive ownership of mutable references
                * example: Box
                    ```
                    let mut a = vec!["a"];
                    let mut b = Box::new(&mut a);
                    let mut c = b.copy(); // it would break borrow checker rules
                    ```
                * example: Vec
                    * Vec looks like this: `{ &mut data, length, capacity }`
                    * copying it means both reference &mut data, which means we have aliased mutable data
            * problem: exclusive ownership of a resource
                * example: Mutex
                    ```
                    let m = Mutex::new(42);

                    thread::spawn(move || { m; }); // if it were moved with copy, we would have two mutexes on same resource
                    ```
                    * passing a mutex by value (with copy) makes no sense
            * analogy: Linux OS
                * self = physically moving a file from one disk area to another
                    * you copy n bytes, where n = size of file
                * &self/&mut self = moving an inode pointer]
                    * you copy m bytes where m = address (typically disk+sector)
        * use for "plain old data" that are stored on the stack and doesn't contain any heap allocations
            * however, it could be prudent to omit the `Copy` implementation, to avoid a breaking API change
        * how to force a move of a type which implements the Copy trait?
            * question does not make sense - it is always move
    * `Debug`, `Display`
        * `Debug` should format the output in a programmer-facing, debugging context
            * should be derived
            * is printing for the programmer, because it usually shows more information
            * means printing with `{:?}`
        * `Display` is for user-facing output
            * must be manually implemented
            * means printing with `{}`
            * automatically implement the ToString trait
                * ToString should never be implemented but Display instead
    * `Deref`, `DerefMut`
        * specify how dereferencing operators like `*` and `.` behave
            * without it the compiler can only dereference & references
            * example
                ```
                struct MyBox<T>(T);

                impl<T> Deref for MyBox<T> {
                    type Target = T;

                    fn deref(&self) -> &Self::Target {
                        &self.0
                    }
                }

                let x = 5;
                let y = MyBox(x);

                assert_eq!(5, *y); // behind the scenes: *(y.deref())
                ```
        * example: Box
            ```
            #[derive(Debug)]
            struct MyStruct {
                value: i32,
            }

            let s1 = MyStruct { value: 42 };
            let s2 = MyStruct { value: 42 };
            let box_deref_star = Box::new(s1);
            let box_deref_dot = Box::new(s2);

            let deref_star: MyStruct = *box_deref_star; // manual deref with *

            let deref_dot: i32 = box_deref_dot.value; // automatic deref with .

            fn coercion(value: &MyStruct) {

            }
            coercion(&box_deref_dot) // automatic deref with coercion
            ```
    * `Drop`
        * called automatically when an object goes out of scope
        * used to free the resources
        * example: `Box`, `Vec`
    * `Eq`, `Hash`, `PartialEq`
        * `PartialEq` = corresponds to a partial equivalence relation
            * symmetric, transitive
        * `Eq` = corresponds to equivalence relation
            * has no methods
                * purpose is to signal that for every value of the annotated type, the value is equal to itself
            * property cannot be checked by the compiler
            * PartialEq` + reflexive
            * floating point types implement `PartialEq` but not `Eq`
                * NaN != NaN
        * `PartialEq` vs `Eq`
            * `HashMap` requires `Eq`
                * if you could use `PartialEq`, you would run the risk of black-holing certain values
            * `assert_eq!` requires `PartialEq`
                * otherwise we could check equality of floats
        * `Hash` uses Siphash 1-3
            * Siphash is a cryptographic algorithm that protects hash-flooding denial-of-service attacks
    * `From` and `Into`
        * useful when performing error handling and is closely related to the ? operator
            * resolves composition of distinct error types
            * Rust performs implicit conversion on the error value using the From trait
                ```
                struct BadRequest {
                    message: String,
                }

                impl From<io::Error> for BadRequest {
                    fn from(error: io::Error) -> Self {
                        BadRequest {
                            message: format!("IoError: {}", error),
                        }
                    }
                }

                impl From<num::ParseIntError> for BadRequest {
                    fn from(error: num::ParseIntError) -> Self {
                        BadRequest {
                            message: format!("ParseIntError: {}", error),
                        }
                    }
                }

                fn read_file(file_name: &str) -> Result<String, io::Error> {
                    Ok(String::from("a"))
                }

                fn open_and_parse_file(file_name: &str) -> Result<i32, BadRequest> {
                    let content = read_file(file_name)?;
                    let num: i32 = content.parse()?;
                    Ok(num)
                }
                ```
        * Rust provides Into implementation for types that have provided From implementation.
            * Into should be used, in cases where From cannot be implemented.
        * conversions cannot fail
            * use: TryFrom, TryInto, FromStr (kept historical reasons, but equivalent for newer TryFrom<&str>)
    * `Send`, `Sync`
        * `Send` = can be moved to different thread
            * means there is always exactly one owner even as the thread changes
        * `Sync` = `T` is `Sync` if and only if `&T` is `Send`
            * other words: non-mut reference can be moved to different thread
            * means a value can be borrowed from multiple threads
        * Rust’s full thread safety story hinges on these two built-in traits
            * example: HashMap is `Send` + `Sync`
                * can be read concurrently if there is not mutable reference
                * can be modified from any thread as long as there is exclusive mutable reference
                    * only one thread accessing it
        * automatically derived traits
            * if a type is composed entirely of Send or Sync types, then it is Send or Sync
        * some types are `Send`, but not `Sync`
            * example: mpsc::Receiver
                * it guarantees that the receiving end of an mpsc channel is used by only one thread at a time
        * some types are `Sync` but not `Send`
            * example
                * MutexGuard
                    * not `Send`: uses libraries that require you to ensure you don't try to free a lock that you acquired in a different thread
                        * if you were able to Send a MutexGuard to another thread the destructor would run in the thread you sent it to, violating the requirement
                    * can still be Sync because all you can send to another thread is an &MutexGuard and dropping a reference does nothing
                * thread-local allocator which doesn’t have any locks, but requires alloc/dealloc happen on the same thread
                    * OpenSpan<Attached> from zipkin
        * few types that are neither `Send` nor `Sync`
            * usually uses internal mutability in a way that isn’t thread-safe
            * example: `std::rc::Rc<T>`
                * reason: uses non-atomic operations to manage its reference count
                * not `Send`
                    ```
                    let rc = Rc::new(42);

                    let handle = thread::spawn(move || {
                        let rc2 = rc.clone(); // race condition to reference count
                    });

                    rc.clone();
                    ```
                * not `Sync`
                    ```
                    let rc = Rc::new(42);
                    let ref_rc = &rc;

                    let handle = thread::spawn(move || {
                        ref_rc.clone(); // race condition to reference count
                    });

                    ref_rc.clone();
                    ```
        * `Pin<P>`
            * borrow checker general rule: can't move an object if it has an active reference to it
                * problem: allow moving self referential structs
            * ensures that the pointee of any pointer type P has a stable location in memory
            * it cannot be moved elsewhere and its memory cannot be deallocated until it gets dropped
                * by default, all types in Rust are movable
            * use case
                * self-referential struct
                    * if the object of the struct is relocated, the value of the member variable will have a meaningless value
                    * example: intrusive doubly-linked list
                * to poll futures, they must be pinned
                    * future might contain self references
                    * you create a Future, move it around if you want, then you pin it, then you start polling it
                        * if a Future implements Unpin then you can pin it, poll it, unpin it, move it, pin it again, poll it, unpin it, move it, and so on
                        * if a Future does not implement Unpin, then you need to pin it once and keep it pinned forever
            * `Unpin`
                * cancels the effect of `Pin<P>`
    * `Sized`
        * types with a constant size known at compile time
        * marker trait
        * Rust can’t store unsized values in variables or pass them as arguments
            * you can only deal with them through pointers like &str or Box<dyn Write> (which themselves are sized)
            * Rust implicitly adds a bound on Sized to every generic function
                * example: `fn generic<T: Sized>(t: T)` same as `fn generic<T>(t: T)`
        * pointer to an unsized value is always a fat pointer
        * struct is allowed to contain a single unsized field, and this makes the struct itself unsized

## closures
* are functions that can capture the enclosing environment (values from the scope)
    * subject to the rules about borrowing and lifetimes
        * two ways for closures to get data from enclosing scopes
            * moves
                ```
                let name = String::from("John");

                // without move it will borrow the variable by reference (default behaviour)
                let print_name = move || { // used to indicate that the closure should take ownership of the variable
                    println!("Hello, {}", name);
                };

                println!("Hello, {}", name); // compilation error: value borrowed here after move
                ```
            * borrowing
                * default behavior for closures in Rust is to capture variables by reference
    * usually do not have the same type as functions
        * every capturing closure has its own type
            * no two closures have exactly the same type
            * ad hoc type created by the compiler, large enough to hold that data
        * if don’t capture anything are identical to function pointers
            * example
                ```
                fn function() {
                    println!("I'm a regular function!");
                }

                let fn_ptr: fn() = function; // function pointer, implements Fn as well

                let closure: fn() = || { // closure without capturing anything
                    println!("I'm a closure without capturing anything!");
                };
* automatically implement one, two, or all three of `Fn` traits
    * depending on how the closure’s body handles the values
    * `move`
        * has no effect in whether a closure is `Fn` or `FnMut` or not
        * causes the variables to be moved into the closure at creation time
            * does not prevent the closure from being called more than once
            * example
                ```
                let s = String::from("test");

                let f = move || {
                    ();
                };

                f();
                f(); // is compiling
                ```
        * usually, you don't have to annotate the move keyword to explicitly tell the compiler
            ```
            let s = String::from("test");

            let f = || {
                s;
                ()
            };

            s; // compilation error: value used here after move, so move actually took place
            ```
        * if the closure uses the value from the environment only via references, the compiler assumes that moving
        that variable into the closure is not necessary
            * it might still be necessary for another reason: lifetimes
                * example: compiler doesn't make the closure a move closure
                    ```
                    fn get_printer(s: String) -> Box<Fn()> {
                        // s now lives in the stackframe of get_printer and the closure outlives that stackframe
                        Box::new(|| println!("{}", s)) // s is used in read only fashion via reference (println doesn't consume its arguments)
                    }
                    ```
    * `FnOnce`
        * all closures implement at least this trait
            * all closures can be called
            * if this trait is the only one they implement and no other, then they can only be called once
        * represents closures that can be invoked only once
            ```
            pub trait Fn<Args> { // simplified
                type Output;
                fn call(self, args: Args) -> Self::Output; // "self" passed by value vs "&self" passed in Fn
            }
            ```
        * if the closure code moves any value out of the captured variables the closure becomes `FnOnce`
            * example: consumes these values (value would no longer be in the environment)
                ```
                let s = String::from("test");

                let f = move || {
                    let ss = s;
                    println!("{}", ss);
                };

                f();
                f(); // not compile: closure cannot be invoked more than once because it moves the variable `s` out of its environment
                ```
    * `FnMut`
        * don’t move out captured values
        * might mutate the captured values
    * `Fn`
        * don’t move out captured values
        * don’t mutate captured values
        * is `FnMut` as well
        * note that `fn` is not a trait
            * example: cannot be used in `where` clause
        * represents types that can be called as if they were functions
            ```
            pub trait Fn<Args> { // simplified
                type Output;
                fn call(&self, args: Args) -> Self::Output; // self passed by reference
            }
            ```
        * automatically implemented by all functions
        * used primarily for working with closures
            * for example: `HashMap` not implements `Fn`
* we don’t need to declare the types of a closure’s arguments
    * Rust will infer them, along with its return type

## references
* Rust’s basic pointer type
    * example: reference to an i32 is a single machine word holding the address of the i32, which may be on the stack or in the heap
* Rust tracks the ownership and lifetimes of values
    * mistakes like dangling pointers, double frees, and pointer invalidation are ruled out at compile time
* non-owning pointer
    * let access a value without affecting its ownership
    * no effect on their referents’ lifetimes
* are never null
* must never outlive their referents
* two types: a way to enforce a multiple readers or single writer rule at compile time
    * rule: either you can read and write the value, or it can be shared by any number of readers, but never both at the same time
    * `&T`
        * immutable, shared reference
            * read but not modify its referent
        * `x` has the type `T` => `&x` has the type `&T`
        * in Rust terminology, we say that it borrows a reference to x
        * are `Copy`
            * we can have many shared references
        * no mutable references along shared references
            * modifying the value they point to is forbidden
    * `&mut T`
        * mutable, exclusive reference
            * both read and modify
        * `x` has the type `T` => `&mut x` has the type `&mut T`
        * forbids any other references of any sort to that value active at the same time
            * in particular: as long as there are shared references to a value, not even its owner can modify it
        * are not `Copy` (makes no sense)
        * iterating provides a mut reference to each element
* permits references to references
* `*` operator
    * given a reference `r`, the expression `*r` refers to the value `r` points to
    * is often omitted, because "dot" operator automatically references or dereferences its left argument
* `.`
    * implicitly dereferences its left operand
    * can also implicitly borrow a reference to its left operand
        * example: HashMap
            ```
            hash_map.insert("one", 1) // desugared into (&mut hashmap_wrapper).insert("one", 1);

            // where fn insert(&mut self, key: K, value: V)
            ```
* pass by value vs pass by reference
    * pass by value = pass a value to a function in a way that moves ownership of the value to the function
    * pass by reference = pass the function a reference to the value
* lifetime
    * used by borrow checker to determine how long references should be valid
        * example: cannot return reference to temporary value
            ```
            fn create_struct<'a>() -> &'a MyStruct {
                let tmp = String::from("Hello, Rust!");

                &tmp // Error: Cannot return reference to temporary value
            }
            ```
    * variable's lifetime begins when it is created and ends when it is destroyed
    * facilitates reasoning
        * example: function with a signature `go(p: &str))` cannot stash its argument anywhere that will outlive the call
    * are used in two different ways
        * reference parameter: `&'a T`
            * means "reference points at a value of type T that is valid for at least the lifetime 'a"
        * lifetime bound: `U: 'a`
            * means "all references in T must outlive lifetime 'a"
            * observation: `T: 'a` includes all `&'a T`
        * `'static`
            * usually error values are always string literals that have the `'static` lifetime
            * indicates that the data pointed to by the reference lives for the remaining lifetime of the running program
                * rather limiting
                    ```
                    struct S {
                        r: &'static i32 // r can only refer to i32 values that will last for the lifetime of the program
                    }
                    ```
                * example of `T` that can be short lived and long lived
                    ```
                    struct MyStruct<'a> {
                        data: &'a str,
                    }
                    fn main() {
                        {
                            let data = String::from("Short lived data"); // not static
                            let slice = &short_lived_data; // not static
                            let struct = MyStruct { data: short_lived_slice }; // not static as not all references are static
                            thread::spawn(move || { struct; }); // cannot be moved, otherwise outlive data
                        }
                        {
                            let data = "Long lived data"; // &'static str
                            let struct = MyStruct { data: long_lived_data }; // MyStruct<'static> as all references are
                            thread::spawn(move || { struct; }); // can be moved as it lives long enough
                        }
                    }
                    ```
            * any owned data always passes a 'static lifetime bound, but a reference to that owned data generally does not
                * reason: owned data is self-contained and needn’t be dropped before any particular variable goes out of scope
                    * `'static` should really be renamed `'unbounded`
                * example
                    ```
                    fn test_bound<T: 'static>(t: T) {}
                    fn test_ref<T>(t: &'static T) {}

                    fn main() {
                        let s1 = String::from("s2"); // not static
                        {
                            let s2 = String::from("s2"); // not static
                            test_bound(s2); // passes the 'static bound check
                            test_ref(&s2); // borrowed value does not live long enough
                        }
                        test_bound(s1); // passes the 'static bound check
                        test_ref(&s1); // borrowed value does not live long enough
                    }
                    ```
    * is a proof that no reference can possibly outlive the value it points to
        * examples
            * cannot return reference to temporary value
                ```
                fn create_struct<'a>() -> &'a MyStruct {
                    let tmp = String::from("Hello, Rust!");

                    &tmp // Error: Cannot return reference to temporary value
                }
                ```
            * value cannot be dropped when its referents are still alive
            * reference stored in some data structure must enclose that of the data structure
    * entirely compile-time
        * at run time, a reference is nothing but an address
    * every type in Rust has a lifetime
    * function signatures with lifetimes have a few constraints:
        * any reference must have an annotated lifetime
        * any reference being returned must have the same lifetime as an input or be static.
        * elision
            * set of rules in Rust that allow the omission of explicit lifetime annotations in function signatures
            * rules
                1. each elided lifetime in input position becomes a distinct lifetime parameter
                    ```
                    fn print_strings(s1: &str, s2: &str)                        // elided
                    fn print_strings<'a, 'b>(s1: &'a str, s2: &'b str)          // expanded
                    ```
                1. if there is exactly one input lifetime position (elided or not), that lifetime is assigned to all elided output lifetimes
                    ```
                    fn as_pair(input1: &str) -> (&str, &str)                    // elided
                    fn as_pair<'a>(input1: &'a str) -> (&'a str, &'a str)       // expanded
                    ```
                1. if there are multiple input lifetime positions, but one of them is &self or &mut self, the lifetime of self is assigned to all elided output lifetimes
                    ```
                    pub fn get<Q>(&self, k: &Q) -> Option<&V>                   // elided
                    pub fn get<'a, 'q, Q>(&'a self, k: &'q Q) -> Option<&'a V>  // expanded
                    ```
                1. otherwise, it is an error to elide an output lifetime
                    ```
                    fn get_str() -> &str;                                   // ILLEGAL
                    ```
* raw pointer
    * kinds
        * `*mut T` - mutable raw pointer
        * `*const T` - immutable raw pointer
        * either can be cast to another without any restrictions
            * distinction there serves mostly as a lint
    * just like pointers in C++
    * unsafe
        * Rust makes no effort to track what it points to
    * may be null, may point to memory that now contains a value of a different type etc
    * dereference only within an unsafe block
* fat pointers
    * is used to refer to references and raw pointers to dynamically sized types (DSTs) – slices or trait objects
    * contains
        * the actual pointer to the piece of data
        * and some additional information at runtime
            * example: length for slices, pointer to vtable for trait objects
* smart pointer
    * data structures that act like a pointer but also have additional information at compile-time
        * example
            1. if the compiler should insert some code when the pointer goes out of scope
            1. ensure its data will always be valid UTF-8
    * usually implemented using structs that implement the Deref and Drop traits
        * example: String, Vec<T>, Box<T>
    * don't always own the data
        * example: MutexGuard only lets you get a mutable reference to the type inside the Mutex, where as the Mutex has ownership
    * vs fat pointers
        * both built-in pointers like &T and smart pointers like Box<T> are thin if T is statically sized, and fat if T is dynamically sized
*` Box<T>`
    * simplest way to allocate a value in the heap
    * `Box::new(v)` allocates some heap space, moves the value v into it, and returns a `Box` pointing to the heap space
    * if goes out of scope, the memory is freed immediately
    * vs `RefCell<T>`
        * Box - borrowing rules’ invariants are enforced at compile time
        * RefCell - invariants are enforced at runtime (program will panic and exit)
            * Mutex<T> is the thread-safe version of RefCell<T>
* `Cow<'a, T>`
    * means "clone on write"
    * is an enum that can either be
        * `Cow::Borrowed`: borrowed reference `(&'a T)`
        * `Cow::Owned`: owned value `T`
    * implements `Deref`
    * to get a mutable reference: `to_mut`
        * invoking it on `Cow::Borrowed` calls the reference’s `to_owned` method to get its own copy of the referent,
        changes into a `Cow::Owned`, and borrows a mutable reference to the newly owned value
            * "clone on write" behaviour
        * similarly, method `into_owned` promotes the reference to an owned
        value and then returns it, moving ownership to the caller
    * problem: return value should be an owned `String` or sometimes it should be a `&'static str`
        ```
        fn get_name() -> String {
            std::env::var("USER")
                .unwrap_or("whoever you are") // expected `String`, found `&str`
        }
        ```
        solution
        ```
        fn get_name() -> Cow<'static, str> {
            std::env::var("USER")
                .map(|v| v.into())
                .unwrap_or("whoever you are".into()) // std has special support for Cow<'a, str>
        }
        ```
    * common use case: return either a statically allocated string constant or a computed string
        ```
        fn describe(error: &MyError) -> Cow<'static, str> {
            match error {
                MyError::OutOfMemory => "out of memory".into(),
                MyError::FileNotFound(ref path) => {
                    format!("file not found: {}", path.display()).into()
                }
            }
        }
        ```
* `Rc<T>`
    * means "reference counter"
    * non-mutable, non-atomic smart pointer
        * vs RefCell
            * RefCell - allows mutable borrows checked at runtime
            * interior mutability - mutating the value inside an immutable value is the interior mutability pattern
                * allows to mutate data even when there are immutable references to that data
                    * normally, this action is disallowed by the borrowing rules
                * pattern uses unsafe code inside a data structure to bend Rust’s usual rules
    * call to `Rc::clone` only increments the reference count
        * doesn’t make a deep copy of all the data
    * implementation of the Drop trait decreases the reference count automatically when an Rc<T> value goes out of scope
        * when the count is then 0, and the `Rc` is cleaned up completely
* `Arc<T>`
    * atomically reference counted
    * like Rc<T> but safe to use in concurrent situations
* CQRS
    * command
        * when you create / insert into a data structure, you move the data in (not reference &)
        * when you run a command to change data, move the memory around (no reference &)
    * query
        * use references
    * example: `HashMap`
        ```
        fn insert(&mut self, key: K, value: V) // command
        fn get(&self, key: &K) -> Option<&V> // query
        ```

## pattern matching
* can be thought of as a generalization of the switch statement
    * comparing objects not just by value (or overloaded equality operator, etc.) but by structure
* example
    ```
    let example_enum: MyEnum = ...
    match example_enum {
        MyEnum::VariantA(value) => ...
        MyEnum::VariantC { name, age } if age >= 18 => ... // match guard
        MyEnum::VariantC { name, age } => ...
    }
    ```
* problem: move when we want borrow
    * example
        ```
        let query_params: Vec<(String, String)> = vec![("page".to_string(), "1".to_string())]

        for &(name, value) in &query_params { // compilation error: cannot move out of a shared reference - type of name and value is String
            println!("{}={}", name, value);
        }
        ```
        * reason: attempt to move those items into the loop scope
            * however due to the way the vector is iterated over (&query_params), we’re only borrowing each item
    * solution: `ref` pattern
        ```
        for &(ref name, ref value) in &query_params { ... }
        ```
        * note that we cannot use `for &(&name, &value)`
        * how parts of the matched value are captured by the pattern’s bindings:
            * without `ref`: they are moved into the match arms
            * with `ref`: they are borrowed instead and represented as references
    * `&` vs `ref`
        * `&` denotes that your pattern expects a reference to an object
            * part of the pattern
            * `&Foo` matches different objects than `Foo` does
            * Rust compiler knows we’re looking for references to certain objects, and not for the objects themselves
        * `ref` indicates that you want a reference to an unpacked value
            * annotates pattern bindings to make them borrow rather than move
            * not part of the pattern
            * `Foo(ref foo)` matches the same objects as `Foo(foo)`
        * why distinction is important?
            * in other places Rust is perfectly happy to blur the gap between references and actual objects
                * example: calling most of their methods
            * pattern matching is destructive operation
                * anything we apply match (or similar construct) to will be moved into the block by default
                * will prevent any subsequent moves and essentially consume the value
* `|` can be used to combine several patterns in a single match arm

## String vs &str vs str
* UTF-8
    * encodes a character as a sequence of one to four bytes
    * restrictions
        * only the shortest encoding for any given code point is considered well-formed
            * example: can’t spend four bytes encoding a code point that would fit in three
        * must not encode numbers from `0xd800` through `0xdfff` or beyond `0x10ffff`
            * either reserved for noncharacter purposes or outside Unicode’s range entirely
* Rust has only one string type in the core language: string slice `str`
    * usually seen in its borrowed form `&str`
    * sequence of Unicode characters stored as a well-formed UTF-8 encoding
        * simple char-by-char comparison does not always give the expected answers
            * example: `th\u{e9}` and `the\u{301}`` are both valid Unicode representations for thé
                * Rust treats them as two completely distinct strings
        * don’t support indexing
            * unintuitive
            * UTF-8 may take 1 to 4 bytes, so `str[idx]` would need to return byte
* for Java people:
    * Rust' String === StringBuilder
    * Rust's &str === (immutable) string
* `String`
    * implemented as a wrapper around a Vec<u8>
        * ensures the vector’s contents are always well-formed UTF-8
    * is provided by Rust’s standard library rather than coded into the core language
    * lives on the heap and therefore is mutable and can alter its size and contents
        * it is very slow
    * can't be created at compile-time => there must be a runtime function call to do that allocation
        * `String::from("literal")`
    * `.to_string()` vs `.to_owned()`
        * `.to_owned()` does the same as `.to_string()` but works for some other types as well
    * vs `&str`
        * `String` is an Object, `&str` is a pointer at a part of object
        * similar to the relationship between by-value `T` and by-reference `&T` for general types
    * vs `Box<str>`
        * `Box<str>` owns a `str` (unlike the `&str`)
        * runtime representation is the same as a `&str`
        * can be seen as a fixed-length String that cannot be resized
            * cannot resize `str` because it does not know its capacity
            * you’d need to reallocate every time you push to the string, but String overallocates for efficiency
* `&str`
    * vs `&String`
        * `&str` is a reference directly into the backing storage of the String, while `&String` is a reference to the "wrapper" object
        * `&str` can be used for substrings, i.e. they are slices
            * `&String&String` references always the whole string
    * pronounced "stir" or "string slice"
    * reference to a run of UTF-8 text
        * owned by someone else: it "borrows" the text
    * fat pointer like other slice references
        * contains address of the actual data and its length
    * immutable by default
    * type `&mut str` does exist, but it is not very useful
        * slice cannot reallocate its referent
            * almost any operation on UTF-8 can change its overall byte length
        * only available operations: `make_ascii_uppercase` and `make_ascii_lowercase`
            * by definition: modify the text in place and affect only single-byte characters
        * is very much like &[T]
    * raw syntax
        ```
        let json_data: &str = r#"
            {
                "name": "John Doe",
                "age": 30,
                "city": "New York",
                "is_student": false,
                "grades": [90, 85, 95]
            }
        "#;
        ```
    * useful to be able to to have multiple different substrings of a String without having to copy
        * example
            ```
            let string: String   = "a string".to_string();
            let substring1: &str = &string[1..3];
            let substring2: &str = &string[2..4];
            ```
    * `&static str`
        * fastest one but also the less flexible
            * literals evaluate to type `&'static str`
        * cannot be modified and its value needs to be known at compile time
        * compiler copies the literal into the crate's read-only static space and generates a forever-valid reference to that value
* `str`
    * fixed-length, stack or heap allocated string slice
    * is an immutable sequence of UTF-8 bytes of dynamic length somewhere in memory
    * we can’t create a variable of type str
        * explanation: all values of a type must use the same amount of memory
            * example
                ```
                 let s1: str = "Hello there!";
                 let s2: str = "How's it going?";
                 // s1 needs 12 bytes of storage and s2 needs 15
                ```
        * always in its borrowed form: `&str`
            * size of a &str value at compile time: it’s twice the length of a usize (address of str and its length)
* rule of thumb
    * use String if you need owned string data (like passing strings to other threads, or building them at runtime)
    * use &str if you only need a view of a string
    * functions arguments
        * pass &str if function does something with a string without needing to stash it away somewhere
        * pass `String` if function modifies or needs to store it for later
* formatting macros
    * `format!` builds `Strings`
    * `println!` writes to the standard output
    * `writeln!` writes to a designated output stream
    * always borrow shared references
        * never take ownership or mutate them
    * example
        ```
        let formatted_string = format!("Hello, {}! You are {} years old.", name, age);
        println!("{}", formatted_string);
        let mut buffer = Vec::new();
        writeln!(buffer, formatted_string); // write a formatted string to the buffer
        ```

## async
* keywords
    * `async` - used to define of asynchronous functions
    * `.await` - blocks until a future is ready
* async runtime
    * provides the infrastructure for running asynchronous tasks and managing concurrency
    * example: tokio
    * why it is not part of std?
        * 3rd party crates can typically afford to move faster
        * different Async runtimes will be better for different use cases
            * example: tokio is a great library for things like web servers, but not ideal for microcontrollers
        * Rust is often used to do embedded software, where some functions of the async runtime
        are actually provided by the environment, and the other features are not desired
            * example: macroquad game engine, which has async as a useful abstraction for doing animation
            frame timing, but does not require an async runtime
        * green threading model was remove before 1.0
* example
    ```
    async fn async_function() { ... } // async definition

    #[tokio::main] // async runtime
    async fn main() {
        async_function().await; // wait
    }
    ```
* `std::future::Future`
    * approach to supporting asynchronous operations
    * async function returns Future immediately
    * trait
        ```
        trait Future {
            type Output;

            // never waits, returns immediately
            // piñata model: whack it with a poll until a value falls out
            fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
        }

        enum Poll<T> {
            Ready(T),
            Pending,
        }
        ```
    * rule: once a future has returned Poll::Ready => never be polled again

## collections
* iteration
    * by value
        * consumes the collection
        * example: `for (k, v) in map`
    * over shared reference
        * produces references
        * example: `for (k, v) in &map`
    * over mut reference
        * produces mut references
        * example: `for (k, v) in &mut map`
            * produces `(&K, &mut V)` pairs
            * there’s no way to get mut access to keys stored in a map, because
                  the entries are organized by their keys
* modification
    * Rust uses moves to avoid deep-copying values
    * example: `Vec<T>::push(item)` takes its argument by value
        * value is moved into the vector
* usually are not not Copy
    * they can’t be, since they owns a dynamically allocated table
* array
    * have a fixed length (known at compile time)
    * useful when you want your data allocated on the stack rather than the heap
* `Vec<T>`
    * memory
        * three fields
            * the length
            * the capacity
                * manages the capacity for you, automatically allocating a larger buffer and moving the elements into it when more space is needed
            * pointer to a heap allocation where the elements are stored
                * created and owned by the `Vec<T>`
            * elements are stored in a contiguous, heap-allocated chunk of memory
    * protection against modifications during traversal
        * java digression: `ConcurrentModificationException`
        * example
            ```
            let mut v = vec![10];
            for (index, &val) in v.iter().enumerate() { borrows a shared (non-mut) reference to the vector
                if val > 10 {
                    v.remove(index); // can't borrow `v` as mutable
                }
            }
            ```
        * solution: use build in functions
            ```
            v.retain(|&val| val <= 10)
            ```
    * types needs to be known at compile time to know exactly how much memory on the heap will be needed to store each element
    * `VecDeque<T>` - double-ended queue (deque)
* `HashMap<K, V>`
    * memory
        * keys, values, and cached hash codes are stored in a single heap-allocated table
        * adding entries eventually forces the HashMap to allocate a larger table and move all
          the data into it
    * useful methods
        * get or insert
            ```
            for id in balances {
                let balance = bank.entry(id).or_insert(0);
                *count += 1;
            }
            ```
        * modify value: `map.entry(key).and_modify(closure)`
        * modify or insert
            ```
            balances.entry(id)
            .and_modify(|count| *count += 1)
            .or_insert(1);
            ```
    * `BTreeMap` - maintains its keys in sorted order.
        * Rust standard library uses B-trees rather than balanced binary trees because B-trees
        are faster on modern hardware
            * binary tree may use fewer comparisons per search than a B-tree, but searching a B-tree has better locality
            * memory accesses are grouped together rather than scattered across the whole heap
                * makes CPU cache misses rarer
* Slice
    * is a region of an array or vector
    * notation: `[T]`
    * example
        ```
        fn do_something(n: &[String]) { // either a vector or an array
        do_something(&a); // works on arrays
        do_something(&v); // works on vectors
        ```
    * many methods are defined on slices
        * example: sort, reverse
    * memory: two-word value (fat not-owning pointer)
        * pointer to the slice’s first element
        * number of elements in the slice
* other: `LinkedList<T>`, `BinaryHeap<T>`, `HashSet<T>`, `BTreeSet<T>`

## useful syntax
* traversing collections
    * with index
        ```
        let my_vec = vec!["apple", "banana", "cherry"];

        for (index, value) in &my_vec.into_iter().enumerate() {
            println!("Index: {}, Value: {}", index, value);
        }
        ```
        * `IntoIterator::into_iter` convert its operand &v into an iterator
    * with destructuring
        ```
        for (&key, &value) in &my_map { ... }
        ```
* shadowing of let
    ```
    fn main() {
        let mut count = 5;
        let count = count + 1; // 6
        let count = count * 2;  // 12
        let mut count = count * 3; // 36
        count += 1; // 37
    }

    ```
* if let
    * example
        ```
        if let Some(cookie) = request.session_cookie {
            return restore_session(cookie);
        }
        ```
    * shorthand for a match with one pattern
        ```
        if let pattern = expr {
            block1
        } else {
            block2
        }
        ```
        is equivalent of
        ```
        match expr {
            pattern => { block1 }
            _ => { block2 }
        }
        ```
    ```
    ```
* return types
    * return without a value is shorthand for return ()
    * if the last expression isn’t followed by a semicolon, its value is the function’s return value
    * `!` - expressions that don’t finish
        ```
        fn diverging_function() -> ! {
            loop {
                // This loop never exits, representing a function that diverges.
            }
        }
        ```
    * `main()` can also return `Result`
* `?` operator
    * can only be applied to the types Result<T, E> and Option<T>
    * unwraps valid values or returns erroneous values
        * return type needs to be compatible with the value the `?` is used on
        * example
            ```
            let output = File::create(filename)?
            ```
            is equivalent to
            ```
            let output = match File::create(filename) {
                Ok(f) => f,
                Err(err) => return Err(err)
            };
            ```
    * error values go through the `From trait` to convert them into the return error type
        * example
            ```
            pub fn fetch_data_from_network(url: &str) -> Result<String, NetworkError> { ... }
            pub fn read_file_contents(file_path: &str) -> Result<String, FileError> { ... }

            pub enum AppError {
                FileOperationFailed,
                NetworkOperationFailed,
            }

            impl From<FileError> for GatewayError { ... }
            impl From<NetworkError> for GatewayError { ... }

            pub foo() -> Result<(), GatewayError> {
                let file_contents = file_parser::read_file_contents("example.txt")?;
                let network_data = network::fetch_data_from_network("https://example.com")?;
                ...
            } // converts specific errors to GatewayError
            ```
* dot
    * The unary `*` operator is used to access the value pointed to by a reference. As we’ve
      seen, Rust automatically follows references when you use the . operator to access a
      field or method, so the * operator is necessary only when we want to read or write the
      entire value that the reference points to.
* auto converting
    * `&String => &str`
    * `&Vec<i32> => &[i32]`
    * `&Box<Chessboard> => &Chessboard`
    * other type conversion requires an explicit cast in Rust
* destructuring
    ```
    let Track { album, track_number, title, .. } = song;
    ```
* struct update syntax
    ```
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 // move, user1 will no be available if some fields were moved and are not copy
    };
    ```

## testing
* convention
    ```
    #[cfg(test)] // include this module only when testing
    mod tests {
        #[test] // tests are ordinary functions marked with the `#[test]`
        fn first_test() {
            assert!(true)
        }
    }
    ```
* `cargo build --release` skips the testing code
    * cargo compiles our test code only if we actively run the tests with `cargo test`
* assertions
    * `assert!`, `assert_eq!`, `assert_ne!`
    * macros from the Rust standard library
    * panics, which causes the test to fail
        * when the main thread sees that a test thread has died, the test is marked as failed
* integration tests
    * entirely external to your library
        * use your library in the same way any other code would
    * `.rs` files that live in a `tests` directory alongside `src`
    * requires `src/lib.rs` file
        * only library crates expose functions that other crates can use
        * binary crate with only `src/main.rs` file => can’t create integration tests in the tests
            * binary crates are meant to be run on their own
    * don’t need to annotate any code in `tests/integration_test.rs` with `#[cfg(test)]`
        * cargo compiles files in this directory only when we run `cargo test`
        * each file in the `tests` directory is compiled as its own separate crate
            * files in subdirectories don’t get compiled as separate crates or have sections in the test output
                ```
                └── tests
                    ├── common
                    │   └── mod.rs // tells Rust not to treat the common module as an integration test file
                    └── integration_test.rs
                ```
* run tests from a file
    * cargo test --test file_name
* parallelization
    * `cargo test -- --test-threads=4`
        * first `--` is used to separate the arguments passed to cargo itself from the arguments
        passed to the test binaries
* testing panic cases
    * example
        ```
        #[test]
        #[allow(unconditional_panic)]
        #[should_panic(expected = "expected panic message")]
        fn test_panicking_function_with_message() {
            panic!("expected panic message");
        }
        ```
        * `unconditional_panic` allows the code to use the panic! macro without triggering a warning about an unconditional panic
    * not recommended
        * when expected error we should use `Result`
        * usually panic should be considered a failure
    * Rust standard library itself includes tests that validate panic behavior
        ```
        #[test]
        #[should_panic]
        fn test_panic_macro() {
            panic!("This is a panic message");
        }

        #[test]
        #[should_panic]
        fn test_unreachable_macro() {
            unreachable!("This code is unreachable");
        }
        ```

## rocket
* is a web framework for Rust
    * provides routing, pre-processing of requests, and post-processing of responses
* `#[launch]`
    * generates a main function that launches a returned Rocket<Build>
    * automatically initializes an async runtime and launches the function’s returned instance
    * example
        ```
        #[launch] // compiled to #[rocket::main]
        fn rocket() -> _ { // compiled to async fn main() -> Result<(), rocket::Error>
            rocket::build() // compiled to rocket().launch().await;
        }
        ```
* fully asynchronous core powered by tokio
    * every request is handled by an asynchronous task which internally calls one or more request handlers
    * tasks are multiplexed on a configurable number of worker threads
        * runtime can switch between tasks in a single worker thread iff (if and only if) an await point in reached
            * context switching is cooperative, not preemptive
            * if an await point is not reached, no task switching can occur
                * important that await points occur periodically in a task so that tasks waiting to be scheduled are not starved
* blocking operation => `rocket::tokio::task::spawn_blocking` (execute the computation in its own thread)
    * example: long computations
    * Note that the number of max_blocking_threads also affects how your spawn_blocking tasks execute. The default is 512, so you're unlikely to hit it unless you've got a lot of work to do, but if it does come up, you might want to impose an explicit limit in your application logic, because tasks waiting to run still take up memory, network servers may not appreciate too many incoming connections, and you've only got so many CPU cores anyway — so at that point you may find that overall throughput is improved by intentionally queueing things within your application logic rather than throwing everything you've got at Tokio.
* configuration: `Rocket.toml`
    * `debug` and `release` are the default profiles for the respective Rust compilation profile
    * `default` profile with fallback values for all profiles
        * defining most configuration
    * `global` profile with overrides for all profiles
    * Based on Figment
        * Figment is a library for declaring and combining configuration sources and extracting typed values from the combined sources
    * The `workers` parameter sets the number of threads used for parallel task execution
    * The max_blocking parameter sets an upper limit on the number of threads the underlying async runtime will spawn to execute potentially blocking, synchronous tasks via spawn_blocking or equivalent.
        * the default value of 512 should not be changed unless physical or virtual resources are scarce
* `uri!()` macro
    * uri!("http://localhost:8000")
    * uri!("https://rocket.rs/", person("Bob", Some(28)), "#woo")
* #[get("/world")]
    * rocket::build().mount(base_path, routes![route1, route2, ...]); // route needs to be mounted
* #[async_trait]
* #[get("/hello/<name>")]
  fn hello(name: &str) -> String {
      format!("Hello, {}!", name)
  }
* To indicate that a handler expects body data, annotate it with data = "<param>", where param is an argument in the handler
* For convenience, Rocket re-exports serde's Serialize and Deserialize traits and derive macros from rocket::serde. However, due to Rust's limited support for derive macro re-exports, using the re-exported derive macros requires annotating structures with #[serde(crate = "rocket::serde")]
* async fn files(file: PathBuf) -> Option<NamedFile> {
    * Option is a wrapping responder: an Option<T> can only be returned when T implements Respond
    *  If the Option is Some, the wrapped responder is used to respond to the client. Otherwise, an error of 404 - Not Found is returned to the client.
* state is managed on a per-type basis: Rocket will manage at most one value of a given type
    * Call manage on the Rocket instance corresponding to your application with the initial value of the state
    * Add a &State<T> type to any request handler, where T is the type of the value passed into manage
    * handlers can concurrently access managed state
        * Rocket automatically parallelizes your application
        * values you store in managed state implement Send + Sync
    * if you request a &State<T> for a T that is not managed, Rocket will refuse to start your application
    * Because State is itself a request guard, managed state can be retrieved from another request guard's implementation using either Request::guard() or Rocket::state()
        * impl<'r> FromRequest<'r> for Item<'r> {
* testing
    1. Construct a Client using the Rocket instance.
    let client = Client::tracked(rocket).unwrap();
    1. Construct requests using the Client instance.
    let req = client.get("/");
    1. Dispatch the request to retrieve the response.
    let response = req.dispatch();
    * blocking testing API is easier to use and should be preferred
        * rocket::local::asynchronous
* Typed validation for dynamic parameters like id is implemented via the FromParam trait.
* to override entries use env variable: ROCKET_{PARAM}
    * example: `ROCKET_PORT=9092`

## thiserror

## mockall