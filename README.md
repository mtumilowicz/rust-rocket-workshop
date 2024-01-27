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
    * https://docs.rs/nutype/latest/nutype/
    * https://medium.com/itir/dyn-traits-in-rust-in-2-hours-1a44130ac514
    * https://www.reddit.com/r/rust/comments/8su7r3/i_dont_understand_the_purpose_of_dyn/
    * https://www.reddit.com/r/rust/comments/130cxak/when_should_you_use_a_trait_as_a_type_in_rust/
    * https://www.reddit.com/r/rust/comments/lhjooa/is_dyn_redundant/
    * https://www.ncameron.org/blog/dyn-trait-and-impl-trait-in-rust/
    * https://www.reddit.com/r/rust/comments/l5uih4/what_is_the_difference_between_clone_and_to_owned/
    * https://stackoverflow.com/questions/66853369/erronous-mutable-borrow-e0502-when-trying-to-remove-and-insert-into-a-hashmap/
    * https://earthly.dev/blog/rust-lifetimes-ownership-burrowing/
    * https://anooppoommen.medium.com/lifetimes-in-rust-7f2331be998b
    * https://www.reddit.com/r/rust/comments/18e3oq0/why_do_lifetimes_need_to_be_leaky/
    * https://www.reddit.com/r/rust/comments/v1z6bx/what_is_a_cow/
    * https://blog.logrocket.com/using-cow-rust-efficient-memory-utilization/
    * https://rahul-thakoor.github.io/rust-raw-string-literals/
    * https://www.shakacode.com/blog/thiserror-anyhow-or-how-i-handle-errors-in-rust-apps/
    * https://stackoverflow.com/questions/69518853/not-using-async-in-rocket-0-5
    * https://github.com/rust-lang-nursery/lazy-static.rs

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

## pointer
* raw pointer
    * kinds (either can be cast to another without any restrictions)
        * distinction there serves mostly as a lint
        * `*mut T` - mutable raw pointer
        * `*const T` - immutable raw pointer
    * just like pointers in C++
    * unsafe
        * Rust makes no effort to track what it points to
    * may be null, may point to memory that now contains a value of a different type etc
    * dereference only within an unsafe block
* fat pointers
    * is used to refer dynamically sized types (DSTs)
        * example: slices or trait objects
    * contains
        * the actual pointer to the piece of data
        * and some additional information at runtime
            * example: length for slices, pointer to vtable for trait objects
* smart pointer
    * data structures that act like a pointer but also have additional information at compile-time
        * example
            1. if the compiler should insert some code when the pointer goes out of scope
            1. ensure its data will always be valid UTF-8
    * usually implemented using structs that implement the `Deref` and `Drop` traits
        * example: String, Vec<T>, Box<T>
    * don't always own the data
        * example: `MutexGuard` only lets you get a mutable reference to the type inside the `Mutex` while `Mutex` has ownership
    * vs fat pointers
        * built-in pointers like `&T` and smart pointers like `Box<T>` are thin if `T` is statically sized, and fat if `T` is dynamically sized

## references
* Rust’s basic pointer type
    * may be on the stack or in the heap
    * example: reference to an i32 is a single machine word holding the address of the i32
* are never null
    * for pointer it's not always true
        * example: `NullPointerException`
* does not affect ownership
    * data is owned by some other variable
* two types
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
    * enforces "multiple readers or single writer" rule at compile time
        * way to prevent data races
            * race condition happens when these three behaviors occur
                1. two or more pointers access the same data at the same time
                1. at least one of the pointers is being used to write to the data
                1. there’s no mechanism being used to synchronize access to the data
        * mistakes like dangling pointers, double frees, and pointer invalidation are ruled out
            * example
                ```
                let mut map: HashMap<String, String> = HashMap::new();

                map.insert("example".to_string(), "a".to_string());
                let example_ref = map.get("example").unwrap(); // immutable borrow occurs here
                map.remove("example"); // compilation error: cannot borrow `map` as mutable because it is also borrowed as immutable
                println!("{a}");
                ```
                explanation: `example_ref` is a reference to the map contents, meaning the existence of v requires borrowing the map until v stops being used
        * can’t protect you from deadlock
            * best protection is to keep critical sections small
* syntax
    * `*` operator
        * used to access the value pointed to by a reference
        * often omitted, because "dot" operator automatically references or dereferences its left argument
        * necessary only when we want to read or write the entire value that the reference points to
    * `.`
        * implicitly dereferences its left operand
        * can also implicitly borrow a reference to its left operand
            * example: `HashMap`
                ```
                hash_map.insert("one", 1) // desugared into (&mut hashmap_wrapper).insert("one", 1);
                ```
*` Box<T>`
    * simplest way to allocate a value in the heap
    * `Box::new(v)` allocates some heap space, moves the value v into it, and returns a `Box` pointing to the heap space
    * if goes out of scope, the memory is freed immediately
    * vs `RefCell<T>`
        * `Box` - borrowing rules’ invariants are enforced at compile time
        * `RefCell` - invariants are enforced at runtime (program will panic and exit)
            * `Mutex<T>` is the thread-safe version of `RefCell<T>`
* `Cow<'a, T>`
    * means "clone on write"
    * is an enum that can either be
        1. `Cow::Borrowed`
            * allows you access borrowed reference as long as it lives
            * makes clone if you attempt to mutate data
                * invoking `to_mut` calls the reference’s `to_owned` method to get its own copy of the referent
                * borrows a mutable reference to the newly owned value
                    * changes into a `Cow::Owned`
        1. `Cow::Owned`
            * invoking `into_owned` promotes the reference to an owned value and then returns it
                * moving ownership to the caller
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
    * call to `Rc::clone` only increments the reference count
        * doesn't make a deep copy of all the data
    * `Rc<T>` value goes out of scope => `Drop` trait decreases the reference count automatically
        * when the count is then 0, and the `Rc` is cleaned up completely
* `Arc<T>`
    * atomically reference counted
    * like `Rc<T>` but safe to use in concurrent situations
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
* permits references to references

## lifetime
* is a way to ensure that a reference is not used after the data it points to has been deallocated
    * in short: uses to ensure all borrows are valid
* is a proof that no reference can possibly outlive the value it points to
    * example: value cannot be dropped when its referents are still alive
            ```
            fn create_struct<'a>() -> &'a str {
                let tmp = String::from("Hello, Rust!"); // data owned by the current function

                &tmp // compilation error: cannot return reference to temporary value
            } // tmp is dropped here
            ```
    * example: reference stored in some data structure must enclose that of the data structure
        ```
        struct Path<'a> { // specifies that the reference must live at least as long as the instance of the struct
            point_x: &'a i32, // struct cannot live longer than references it holds
            point_y: &'a i32,
        }
        ```
* every type in Rust has a lifetime
    * variable's lifetime begins when it is created and ends when it is destroyed
* entirely compile-time
    * at run time, a reference is nothing but an address
* function signatures with lifetimes rules
    1. any reference must have an annotated lifetime
    1. any reference being returned must have the same lifetime as an input or be static
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
    * facilitates reasoning
        * example: function with a signature `go(value: &str))` cannot stash its argument anywhere that will outlive the call
            * reason: `value` is a reference that is only valid in the method body
* `'static`
    * indicates that the data pointed to by the reference lives for the remaining lifetime of the running program
        * rather limiting
            ```
            struct S {
                r: &'static i32 // r can only refer to i32 values that will last for the lifetime of the program
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
                    let s1 = String::from("s1"); // not static
                    test_bound(s1); // passes the 'static bound check
                    test_ref(&s1); // borrowed value does not live long enough, we can spawn thread in test_ref
                }
                ```
    * use case: usually error values are always string literals that have the `'static` lifetime

## borrow checker
* compiler component that enforces ownership and borrowing rules
* not being perfect - does not accept all valid programs, but it does reject all invalid programs
* ownership = set of rules that govern how a Rust program manages memory
    * rules
        1. each value in Rust has an owner
        1. there can only be one owner at a time
        1. when the owner goes out of scope, the value will be dropped
            * scope = range within a program for which an item is valid
            * owner is responsible for value deallocation
                * Rust eliminates explicit memory deallocation
* borrowing = action of creating a reference
    * rules
        1. each resource can only have one mutable reference or any number of immutable references at a time
        1. references must always be valid
            * resource being referenced must remain in scope for the entire lifetime of the reference
        1. mutable reference cannot exist at the same time as any other reference, mutable or immutable


## structs
* type that is composed of other type
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
    * unit-like
        * example: `std::fmt::Error`
            ```
            pub struct Error
            ```
        * can serve as a marker type
        * basis for stateless trait implementation
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
            vs
            ```
            let mut point = Point { x: 0, y: 0 }; // ok

            point.x = 5;
            ```
* contain data, but can also have logic
    * if contains references, references’ lifetimes must be specified
        * lifetimes on structs should only be used when the struct is a "view" or "cursor" that looks inside some other struct
            * example: `&str` is easy for read-only function parameters, but it's a pain for structs
                * you should never put the `&str` type in a struct (just default to `String`)
        * rule of thumb: "does the struct own this data?"
            * yes => `'static` (unborrowed) fields
            * no => you go with references
            * shared ownership => `Rc`/`Weak`/`Arc`
                * if it's possibly shared => `Cow`
    * `impl` is used to tie logic to struct
        * Rust method must explicitly use `self` to refer to the value it was called on
            * similar to: Python (`self`), JavaScript (`this`)
            * not similar to: Java (`this` is auto-generated by compiler)
            * can take `self` as value and as a reference
                * by value is used when the method transforms self into something else
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
* memory
    * Rust doesn’t make specific promises about how it will order a struct’s fields or elements in memory
    * Rust does promise to store fields’ values directly in the struct’s block of memory
        * Java would put values each in their own heap-allocated blocks and have struct fields point
        at them
* enums
    * useful for quickly implementing tree-like data structures
    * can have methods
    * rust prohibits match expressions that do not cover all possible values
    * memory
        * tag + enough memory to hold all fields of the largest variant
            * tag field is for Rust’s internal use
                * tells which constructor created the value and therefore which fields it has.
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
* by default, struct and enum types are not `Copy`
    * all the fields of struct `Copy` <=> copy can be derived for struct

## error handling
* two categories
    * recoverable
        * example: file not found
        * should implement the `std::error::Error` trait
            * can be derived: `#[derive(Error)]`
            * guaranteed that such error has human-readable and debuggable representations
        * part of `Result`
            * equivalent of `Either`
            * equivalent of try/catch in other languages
    * unrecoverable
        * example: trying to access a location beyond the end of an array
        * should never happen
            * always symptoms of bugs
        * like a `RuntimeException` in Java
        * named `panic`
            * macro `panic!()` triggers a panic directly
            * rule of thumb: "don't panic"
        * doesn’t automatically spread from one thread to the threads that depend on it
            * reported as an error `Result` in other threads
        * main thread => whole process exits (with a nonzero exit code)
        * triggers stack unwinding
            * walks back up the stack and cleans up the data from each function it encounters
            * a lot of work
            * catches stack unwinding: `std::panic::catch_unwind(function)`
                * allowing the thread to survive and continue running
                * use case: mechanism used by Rust’s test harness to recover when an assertion fails in a test
        * customizable
            * compile with -C panic=abort => first panic in the program immediately aborts the process

## traits
* Rust’s take on interfaces
* approach inspired by Haskell’s typeclasses
* are not types in Rust
    * are bounds on types
    * two ways to use them in a type position
        * dispatch = mechanism to determine which specific version is actually run
        1. `dyn Trait` ~ dynamic dispatch
            * known as a trait object
                * trait objects are types, and not traits
            * type can only be known at runtime
            *
            * upsides: less code bloat (trait object is not specialised to each of the types)
            * downsides
                * slower virtual function calls
                * inhibiting any chance of inlining and related optimisations
        1. `impl Trait` ~ static dispatch
            * not something Go or Java have
            * use case: generics
            * means "any Struct that has an `impl Trait for Struct { ... }`"
            * performed using monomorphization
                * process of turning generic code into specific code by filling in the concrete types that are used when compiled
                * example
                    ```
                    impl Foo for u8 { ... }
                    impl Foo for String { ... }

                    fn do_something<T: Foo>(x: T) { ... } // compiler will create a special version for both u8 and String, and then replace the call sites
                    // fn some_function(foo: impl Trait) { ... } // equivalent to above, can be handy when one type for param
                    ```
            * can be used in three locations
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
                * as a return type
                    * use case: iterators
                        ```
                        fn women_vip<'a>(persons: &'a Vec<Person>) -> impl Iterator<Item = &'a Person> + 'a { // instead of Filter<Iter<'_, Person>, fn(&&'a Person) -> bool>
                            persons.iter().filter(|p| p.is_woman())
                        }
                        ```
                    * problem: concrete type needs to be known at compile time
                        * reason: in order to allocate the right amount of space on the stack
                        * trait types are unsized and don't have a fixed size known at compile time
                        * solution: box it
                * as any type that implements given trait
                    * example: `impl<T: Display> ToString for T`
            * upsides: allowing for inlining and hence usually higher performance
            * downsides: code bloat due to many copies of the same function existing in the binary, one for each type
* used to add extensions methods to existing types (even built-in like `str` and `bool`)
    * example
        ```
        impl<W: Write> WriteHtml for W { ... } // extension trait for Write trait

        impl IsBlank for str { // adds is_blank() method for String and &str
            fn is_blank(&self) -> bool {
                self.trim().is_empty()
            }
        }
        ```
* useful traits
    * some of them can be automatically derived
        * example: `#[derive(Copy, Clone, Debug, PartialEq)]`
    * `Clone`
        * deep copy: expensive, in both time and memory
        * some types don’t make sense to copy: `Mutex`
        * problem: clone converts `&T` to `T`
            * example: clone a `&str` - `str` is unsized and is not even type that a function could return
            * solution: `ToOwned` can convert from `&T` to another target type
                * generalizes `Clone`
                    * `ToOwned` is `Clone` trait that doesn't have to return itself
                * used to convert borrowed data (e.g., a reference) into an owned version
                * example `impl ToOwned for str { type Owned = String; ... }`
    * `Copy`
        * represents values that can be safely duplicated via `memcpy`: simply copying the bits in memory
            * example: `u8`
                * you cannot possibly be more efficient with a move
                    * under the hood it would probably at least entail a pointer copy
                        * it is already as expensive as a `u8` copy
        * cannot be implemented, only derived
        * affects how the compiler uses moves (automatic copies)
            * just means not treating the old copy as uninitialized
            * Copy` types are implicitly **cloned** whenever they're moved
                * every `Copy` type is also required to be `Clone`
                    * `Copy` is a special case of `Clone` where the implementation is just "copy the bits"
                    * reverse is not true
                        * example: `Vec<T>`, `String`
                            ```
                            let s1 = String::from("hello");
                            let s2 = s1; // s1 was moved into s2, s1 cannot be used anymore
                            ```
        * `move` is a property of the type, not the operation
            * `let a = b` is always a move, doesn't matter whether `b` is `Copy` or not
                * always does the same thing: `memcpy`
                * how to force a move of a type which implements the `Copy` trait?
                    * question does not make sense - it is always move
                * rust is pass by value
                    * reference is passed by value but is `copy`
            * similarly, a function call `f(b)` always moves `b`
                * bytes of `b` really get copied into f's stack frame, at least in debug builds
                * if you use `&mut self` instead of `self`, you're still telling Rust to move, but you'll be moving the pointer instead
                    * analogy: Linux OS
                        * self = physically moving a file from one disk area to another
                            * you copy n bytes, where n = size of file
                        * &self/&mut self = moving an inode pointer
                            * you copy m bytes where m = address (typically disk+sector)
                * it's up to llvm to optimize out this copy in release builds
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
                    * if the object on the stack is large enough, Rust's compiler may choose to pass the object's pointer instead
        * bitwise copy must represent a valid and independent duplicate of the original value
            * problem: freeing resource
                * example: `Drop` trait
                    * double deallocation
                        * using `memcpy` would result in two instances pointing to the same memory
                    * don't even have to alias heap data
                        * file handles: file being closed twice
            * problem: exclusive ownership of mutable references
                * example: `Vec`
                    * `Vec` looks like this: `{ &mut data, length, capacity }`
                    * copying it means both reference `&mut data`, which means we have aliased mutable data
            * problem: exclusive ownership of a resource
                * example: Mutex
                    ```
                    let m = Mutex::new(42);

                    thread::spawn(move || { m; }); // if it were moved with copy, we would have two mutexes on same resource
                    ```
                    * passing a mutex by value (with copy) makes no sense
            * use case: "plain old data" that are stored on the stack and doesn't contain any heap allocations
                * however, it could be prudent to omit the `Copy` implementation, to avoid a breaking API change
    * `Debug`, `Display`
        * `Debug` should format the output in a programmer-facing, debugging context
            * should be derived
            * means printing with `{:?}`
        * `Display` is for user-facing output
            * must be manually implemented
            * means printing with `{}`
            * automatically implement the `ToString` trait
                * `ToString` should never be implemented but `Display` instead
    * `Deref`, `DerefMut`
        * specify how dereferencing operators like `*` and `.` behave
            * without it the compiler can only dereference `&` reference
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
            let deref_star: i32 = *y; // automatic deref with "*", behind the scenes: *(y.deref())

            let deref_dot: i32 = y.0; // automatic deref with "."

            fn coercion<T>(value: &T) { }
            coercion(&y) // automatic deref with coercion
            ```
    * `Drop`
        * called automatically when an object goes out of scope
        * used to free the resources
        * example: `Box`, `Vec`
    * `Eq`, `PartialEq`, `Hash`
        * `PartialEq` = corresponds to a partial equivalence relation
            * symmetric, transitive
        * `Eq` = corresponds to equivalence relation
            * `PartialEq` + reflexive (value is equal to itself)
            * marker trait: cannot be checked by the compiler
            * example of `PartialEq` but not `Eq`: floating point
                * NaN != NaN
            * `HashMap` requires `Eq`
                * if you could use `PartialEq`, you would run the risk of black-holing certain values
            * `assert_eq!` requires `PartialEq`
                * otherwise we could not check equality of floats
        * `Hash` uses Siphash 1-3
            * Siphash is a cryptographic algorithm that protects hash-flooding denial-of-service attacks
    * `From` and `Into`
        * use case: performing error handling
            * related to the `?` operator
            * resolves composition of distinct error types
                * example: implicit conversion on the error value using the `From` trait
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
                        let file_contents = file_parser::read_file_contents("example.txt")?; // values go through the `From` trait to convert them into the return error type
                        let network_data = network::fetch_data_from_network("https://example.com")?;
                        ...
                    } // converts specific errors to GatewayError
                    ```
        * Rust provides `Into` implementation for types that have provided `From` implementation
            * `Into` should be used, in cases where `From` cannot be implemented
                * example: gateway inputs that results in the same command
                    * `NewCustomerApiInput`, `NewCustomerV2ApiInput` -> `NewCustomerCommand`
                    * cannot be implemented using `From`: conflicting implementation for `NewCustomerCommand`
        * conversions cannot fail
            * use: `TryFrom`, `TryInto`, `FromStr` (kept historical reasons, but equivalent for newer `TryFrom<&str>`)
    * `Send`, `Sync`
        * `Send` = can be moved to different thread
        * `Sync` = `T` is `Sync` if and only if `&T` is `Send`
            * other words: non-mut reference can be moved to different thread
        * automatically derived traits (if a type is composed entirely of `Send` or `Sync` types)
        * thread safety story relies on these two built-in traits
            * example: `HashMap` is `Send` + `Sync`
                * can be read concurrently if there is no mutable reference
                * can be modified from any thread as long as there is exclusive mutable reference
                    * only one thread accessing it
        * example of: `Send`, but not `Sync`
            * `mpsc::Receiver`
                * receiving end of an mpsc channel is used by only one thread at a time
        * example of: `Sync` but not `Send`
            * `MutexGuard`
                * not `Send`: requires alloc/dealloc happen on the same thread
                    * sending it to other thread would violate the requirement
                * is `Sync`: dropping a reference (`&MutexGuard`) does nothing
            * similar case: thread-local allocator like `OpenSpan<Attached>` from `zipkin`
        * example of: neither `Send` nor `Sync`
            * usually uses internal mutability in a way that isn’t thread-safe
            * example: `std::rc::Rc<T>`
                * reason: race condition to reference count
                    * uses non-atomic operations to manage its reference count
* memory: fat pointer
    1. pointer to the value
    1. pointer to a table (vtable) corresponding to the specific implementation of `T`
        * vtable is essentially a struct of function pointers, pointing to the concrete piece of machine code for each method in the implementation
* orphan rule: when implementing a trait, either the trait or the type must be new in the current crate
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

## closures
* are functions that can capture the enclosing environment (values from the scope)
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
        * capturing => has its own type
            * no two closures have exactly the same type
            * ad hoc type created by the compiler, large enough to hold that data
        * not capturing => identical to function pointers `fn()`
* automatically implement one, two, or all three of `Fn` traits
    * depending on how the closure’s body handles the values
    * `move`
        * has no effect in whether a closure is `Fn` or `FnMut` or not
        * causes the variables to be moved into the closure at creation time
            * does not prevent the closure from being called more than once
            * example
                ```
                let s = String::from("test");

                let f = move || { (); };

                f();
                f(); // is compiling
                ```
        * usually, you don't have to annotate the `move` keyword to explicitly tell the compiler
            ```
            let s = String::from("test");

            let f = || { s; () };

            s; // compilation error: value used here after move - move actually took place
            ```
            * it might still be necessary because of lifetimes
                * closure uses the value from the environment via references => compiler assumes that moving is not necessary
                * example
                    ```
                    fn foo(s: String) -> Box<dyn Fn()> { // s now lives in the stackframe of foo and the closure outlives that stackframe
                        Box::new(|| { &s; () }) // compilation error: compiler doesn't make the closure a move closure
                    }
                    ```
    * `FnOnce`
        * all closures implement at least this trait
            * `trait Fn<Args: Tuple>: FnMut<Args>`
            * `trait FnMut<Args: Tuple>: FnOnce<Args>`
            * all closures can be called
            * if only one implemented trait => can be called only once
        * represents closures that can be invoked at least once
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
                    s;
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
        * automatically implemented by all functions
            * note that `fn` is not a trait
                * example: cannot be used in `where` clause
        * represents types that can be called as if they were functions
        * used primarily for working with closures
            * for example: `HashMap` not implements `Fn`
* Rust will infer types of a closure’s arguments and return type

## pattern matching
* generalization of the switch statement
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
* binding patterns
    * to service better ergonomics, patterns operate in different binding modes
        * default binding mode starts in "move" mode which uses move semantics
            * problem: move when we want borrow
                * example
                    ```
                    let query_params: Vec<(String, String)> = vec![("page".to_string(), "1".to_string())]

                    for &(name, value) in &query_params { // compilation error: cannot move out of a shared reference - type of name and value is String
                        println!("{}={}", name, value);
                    }
                    ```
                * solution: `ref` pattern
                    ```
                    for &(ref name, ref value) in &query_params { ... }
                    ```
        * references / mutable references set binding mode to `ref` / `mut ref`
            * called match ergonomics: https://rust-lang.github.io/rfcs/2005-match-ergonomics.html
            * `ref` indicates that you want a reference
                * annotates pattern bindings to make them borrow rather than move
                * not part of the pattern
                    * `Foo(ref foo)` matches the same objects as `Foo(foo)`
            * example: iteration
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
                        * there’s no way to get mut access to keys stored in a map
                            * entries are organized by their keys
* `|` can be used to combine several patterns in a single match arm

## String vs &str vs str
* UTF-8
    * encodes a character as a sequence of one to four bytes
        * indexing is not intuitive
            * `str[idx]` would need to return byte
    * restrictions
        * only the shortest encoding for any given code point is considered well-formed
            * example: can’t spend four bytes encoding a code point that would fit in three
    * char-by-char comparison does not always give the expected answers
        * example: UTF-8 encoding
            * `th\u{e9}` and `the\u{301}` are both valid Unicode representations for thé
            * `th\u{e9}` = `0xC3 0xA9`
            * `the\u{301}` = `0xC3 0x81`
* Rust has only one string type in the core language: string slice `str`
    * usually seen in its borrowed form `&str`
    * stored as a well-formed UTF-8 encoding (of Unicode characters)
* for Java people:
    * `String` === `StringBuilder`
    * `&str` === (immutable) string
* `String`
    * implemented as a wrapper around a `Vec<u8>`
        * ensures the vector’s contents are always well-formed UTF-8
    * is provided by Rust’s standard library rather than coded into the core language
    * lives on the heap and therefore is mutable and can alter its size and contents
        * overallocates for efficiency
        * it is very slow
    * can't be created at compile-time => there must be a runtime function call to do that allocation
        * `String::from("literal")`
    * `.to_string()` vs `.to_owned()`
        * `to_owned()` does the same as `to_string()`
        * `to_owned()` is part of the `ToOwned` trait
    * vs `Box<str>`
        * `Box<str>` owns a `str` (unlike the `&str`)
        * runtime representation is the same as a `&str`
        * can be seen as a fixed-length `String` that cannot be resized
            * cannot resize `str` because it does not know its capacity
            * you'd need to reallocate every time you push to the string
* `&str`
    * vs `String`
        * `String` is an object, `&str` is a pointer at a part of the object
    * vs `&String`
        * `&str` is a reference directly into the backing storage of the String, while `&String` is a reference to the "wrapper" object
        * `&str` can be used for substrings, i.e. they are slices
            * `&String` references always the whole string
    * pronounced "stir" or "string slice"
    * reference to a sequence of UTF-8 text
        * owned by someone else: it "borrows" the text
    * fat pointer like other slice references
        * contains address of the actual data and its length (twice the length of a `usize`)
    * immutable
    * type `&mut str` does exist, but it is not very useful
        * slice cannot reallocate its referent
            * almost any operation on UTF-8 can change its overall byte length
        * only available operations: `make_ascii_uppercase` and `make_ascii_lowercase`
            * by definition: modify the text in place and affect only single-byte characters
    * is very much like `&[T]`
    * raw syntax: allow to write the literal without requiring escapes
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
    * useful to be able to to have multiple different substrings of a `String` without having to copy
        * example
            ```
            let string: String   = "a string".to_string();
            let substring1: &str = &string[1..3];
            let substring2: &str = &string[2..4];
            ```
    * `&static str`
        * fastest one but also the less flexible
        * value needs to be known at compile time
            * literals evaluate to type `&'static str`
        * cannot be modified
        * compiler copies it into the crate's read-only static space
        * forever-valid reference
* `str`
    * fixed-length, stack or heap allocated string slice
    * is an immutable sequence of UTF-8 bytes of dynamic length somewhere in memory
    * we can’t create a variable of type `str`
        * explanation: all values of a type must use the same amount of memory
            * example
                ```
                 let s1: str = "Hello there!";
                 let s2: str = "How's it going?";
                 // s1 needs 12 bytes of storage and s2 needs 15
                ```
        * always in its borrowed form: `&str`
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
        * different runtimes for different use cases
            * example: tokio is a great library for things like web servers, but not ideal for microcontrollers
        * in embedded software some async runtime functions are actually provided by the environment
            * example: macroquad game engine, which has async as a useful abstraction for doing animation
            frame timing, but does not require an async runtime
        * green threading model was removed before 1.0
* example
    ```
    async fn async_function() -> String { ... } // async definition

    #[tokio::main] // async runtime
    async fn main() {
        let s: String = async_function().await; // block
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
            Ready(T), // once a future has returned `Poll::Ready` should be never be polled again
            Pending,
        }
        ```

## collections
* modification
    * Rust uses moves to avoid deep-copying values
    * example: `Vec<T>::push(item)` takes its argument by value
        * value is moved into the vector
* usually are not `Copy`
    * they can’t be, since they owns a dynamically allocated table
* array
    * allocated on stack
        * have a fixed length (known at compile time)
        * arrays of types that implement the `Copy` trait themselves are also `Copy`
* `Vec<T>`
    * `T` needs to be known at compile time to know exactly how much memory on the heap will be needed to store each element
        * example: `Vec<dyn MyTrait>` will not compile, should be used `Vec<Box<dyn MyTrait>>` or `Vec<&dyn MyTrait>`
    * memory
        * three fields
            * the length
            * the capacity
                * manages the capacity automatically allocating a larger buffer and moving the elements into it when more space is needed
            * pointer to a heap allocation where the elements are stored
                * created and owned by the `Vec<T>`
            * elements are stored in a contiguous, heap-allocated chunk of memory
    * compile time protection against modifications during traversal
        * java digression: `ConcurrentModificationException`
        * example
            ```
            let mut v = vec![10];
            for (index, &val) in v.iter().enumerate() { // borrows a shared (non-mut) reference to the vector
                if val > 10 {
                    v.remove(index); // can't borrow `v` as mutable
                }
            }
            ```
        * solution: use build in functions
            ```
            v.retain(|&val| val <= 10)
            ```
    * `VecDeque<T>` - double-ended queue (deque)
* `HashMap<K, V>`
    * memory
        * keys, values, and cached hash codes are stored in a single heap-allocated table
        * manages the capacity automatically
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
    * `BTreeMap` - maintains its keys in sorted order
        * std uses B-trees rather than balanced binary trees because B-trees are faster on modern hardware
            * binary tree may use fewer comparisons per search than a B-tree, but searching a B-tree has better locality
            * memory accesses are grouped together rather than scattered across the whole heap
                * makes CPU cache misses rarer
* Slice
    * is a region of an array or vector
    * notation: `&[T]`
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
* return types
    * return without a value is shorthand for return `()`
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
        * binary crate with only `src/main.rs` file => can’t create integration tests
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
        * when error is expected we should use `Result`
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

## rocket
* is a web framework for Rust
    * provides routing, pre-processing of requests, and post-processing of responses
* fully asynchronous, powered by tokio
    * every request is handled by an asynchronous task which internally calls one or more request handlers
    * tasks are multiplexed on a configurable number of worker threads
        * runtime can switch between tasks in a single worker thread iff (if and only if) an `await` point is reached
            * context switching is cooperative (switches occur when a task explicitly yields control to the scheduler)
                * not preemptive (switches are done independently of the tasks' cooperation)
            * if an `await` point is not reached, no task switching can occur
                * important that `await` points occur periodically
* `#[launch]`
    * generates a main function that launches a returned `Rocket<Build>`
    * automatically initializes an `async` runtime and launches the function’s returned instance
    * example
        ```
        #[launch] // compiled to #[rocket::main]
        fn rocket() -> _ { // compiled to async fn main() -> Result<(), rocket::Error>
            rocket::build() // compiled to rocket().launch().await;
        }
        ```
* endpoint
    * example
        ```
        #[get("/customers/<id>")] // typed validation for id is implemented via the FromParam trait
        async fn get_customer(id: String) -> Option<String> { ... } // if None - an error of 404 - Not Found is returned to the client

        #[post("/hello", data = "<data>")] // data = "<param>", where param is an argument in the handler means that a handler expects body data
        async fn create_customer(data: Json<CustomerApiInput>) -> Result<Json<CustomerApiOutput>, Custom<Json<ErrorApiOutput>>>  { ... }

        rocket::build().mount(base_path, routes![get_customer, create_customer, ...]); // route needs to be mounted
        ```
    * when a non-async handler is run, it will be as if Rocket used spawn_blocking()
    * `Custom<R>(pub Status, pub R)` - creates a response with the given status code and underlying responder
        * example
            ```
            let response: Custom<String> = status::Custom(Status::BadRequest, "error");
            ```
* serialization
    * Rocket re-exports serde's `Serialize` and `Deserialize` traits and derive macros from `rocket::serde`
    * using the re-exported derive macros requires annotating structures with `#[serde(crate = "rocket::serde")]`
        * due to Rust's limited support for derive macro re-exports
* state
    * managed on a per-type basis
        * at most one value of a given type
    * handlers can concurrently access managed state
        * add `&State<T>` type to any request handler, where `T` is the type of the value passed into manage
            * example
                ```
                #[get("/customers/<customer_id>")]
                pub async fn get_customer(
                    customer_id: String,
                    service: &rocket::State<Arc<CustomerService>>,
                ) -> Option<Json<CustomerApiOutput>>
                ```
        * Rocket automatically parallelizes your application
        * values you store in managed state implement `Send + Sync`
    * Rocket instance needs to be started with initial value of the state
        * use `manage` to add state to Rocket instance
            * example: `rocket::build().manage(state)`
        * if `&State<T>` for a `T` is not managed => Rocket will refuse to start application
            * example: `error: launching with unmanaged T state`
* configuration: `Rocket.toml`
    * profiles
        * `debug` and `release`
            * profiles for the respective Rust compilation profile
        * `default`
            * profile with fallback values for all profiles
            * defining most configuration
        * `global`
            * profile with overrides for all profiles
    * to override entries use env variable: `ROCKET_{PARAM}`
        * example: `ROCKET_PORT=9092`
    * based on Figment
        * figment is a library for declaring and combining configuration sources and extracting typed values from the combined sources
    * useful parameters
        * `workers` - sets the number of threads used for parallel task execution
        * `max_blocking` - sets an upper limit of threads to execute potentially blocking, synchronous tasks
            * `rocket::tokio::task::spawn_blocking(FnOnce)`
                * execute the computation in its own thread
                * example: long computations
* testing
    * example
        ```
        let client = Client::tracked(rocket).unwrap(); // construct a Client using the Rocket instance
        let req = client.get("/"); // Construct requests using the Client instance
        let response = req.dispatch(); // Dispatch the request to retrieve the response
        ```
    * blocking testing API is easier to use and should be preferred
        * async testing API: `rocket::local::asynchronous`
* useful macros
    * `uri!()` creates a type-safe, URL safe URI
        * example
            * `uri!("http://localhost:8000")`
            * `uri!("/api", person: name = "Mike", age = 28);` ~ `"api/person/Mike?age=28"`
            * from endpoint
                ```
                #[get("/person/<name>?<age>")]
                fn person(name: &str, age: Option<u8>) { }

                uri!("api/", person("Bob", Some(28))); // api/person/Bob?age=28
                ```
    * `#[async_trait]`
        * support for `async fn` in trait impls and declarations

## validator
* simplify struct validation
* example
    ```
    #[derive(Debug, Validate, Deserialize)]
    struct PersonApiInput {
        #[validate(custom = "validate_name")]
        name: String
    }

    fn validate_name(name: &str) -> Result<(), ValidationError> { ... }

    if let Err(_) = person_api_input.validate() {
      return "invalid input";
    };
    ```
## lazy-static
* enables to have statics initialized at runtime
* in most of the cases can be replaced with `std::sync::OnceLock`
    * example approach
        ```
        fn hashmap() -> &'static HashMap<u32, &'static str> {
            static HASHMAP: OnceLock<HashMap<u32, &str>> = OnceLock::new();
            HASHMAP.get_or_init(|| {
                let mut m = HashMap::new();
                m.insert(0, "foo");
                m.insert(1, "bar");
                m.insert(2, "baz");
                m
            })
        }
        ```
    * apart from cases that u need value not function

## nutype
* allows adding extra constraints like sanitization and validation to the regular newtype pattern
* example
    ```
    fn sanitize_username(username: String) -> String {
        username.trim().to_lowercase()
    }

    fn validate_username(username: &str) -> bool {
        !username.is_empty() && username.len() <= 20
    }

    #[nutype(
        sanitize(sanitize_username),
        validate(not_empty, len_char_max = 20),
    )]
    pub struct Username(String);
    ```

## thiserror
* simplifies the implementation of the Error trait
* the same thing as if you had written an implementation of `std::error::Error` by hand
* `Display` impl is generated for your error if you provide `#[error("...")]`
    * support a shorthand for interpolating fields
        * `#[error("{var}")]` => `write!("{}", self.var)`
        * `#[error("{0}")]` => `write!("{}", self.var)`
* `From` impl is generated for each variant containing a `#[from]` attribute

## mockall
* provides tools to create mock versions of almost any trait or struct
    1. annotate with `#[automock]`
    1. instantiate the mock struct with its `new` or `default` method
    1. record expectations