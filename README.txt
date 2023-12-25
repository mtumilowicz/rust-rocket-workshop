
* references
    * https://doc.rust-lang.org/book/
    * https://doc.rust-lang.org/stable/rust-by-example/
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

1. general
    * allocates on stack by default
1. ownership and borrowing
2. packages crates modules
1. structs
    * what to use inside struct: rule of thumb
    * As a rule of thumb, you should never put the &str type in a struct. Lifetimes on structs should only be used when the struct is a "view" or "cursor" that looks inside some other struct, which is not what is happening here.
    * You need to ask yourself "Does the struct own this data?". If so, you go with 'static (unborrowed) fields and if not, you go with references. If ownership is shared, you go with Rc/Weak/Arc depending on the kind of ownership. And if it's possibly shared, you go with Cow.
    * I'd just default to String for struct fields (unless it's a constant string literal so that you can use &'static str. &str is easy for read-only function parameters, but it's a pain for structs, so I generally only use it if I really need it.
3. error: unrecoverable: panic, recoverable: result
4. traits, lifetimes
    * utility traits
    * macros (derev)
    * static dispatch
        * Static dispatch is not something Go or Java have.
        * equivalent
            * fn some_function<T: Trait>(foo: T) { … }
            * fn some_function(foo: impl Trait) { … }
    * dynamic dispatch
        * As dynamic dispatch must work on objects which might not be Sized, you need a reference to use it. That is, you would use &dyn Trait or Box<dyn Trait> (note: for historical reasons, the dyn keyword is not required, but modern Rust uses it).
        * Box<dyn Trait> is just sugar for Box<dyn Trait + 'static>
1. 'static
    * the parameter type `T` may not live long enough
        ```
        fn test<T:Sync+Send>(a:Arc<RwLock<T>>){
            let b = a.clone();
            thread::spawn(move||{b;}); // compiler error: the parameter type `T` may not live long enough
        }
        ```
        * T could still borrow something which could have a short lifetime so that's why spawn require 'static bound which means that everything is owned. Adding 'static bound to T should solve the issue.
        * 'static has two different meanings, depending on where it's used.

          In a reference, it means "lives for the rest of the program" while, in a type bound, it means the broader "lives however long I need it to" that encompasses both 'static references and owned values.

          (Technically, they're the same "lives as long as I need it to" meaning... there's just no way for a reference to extend the lifetime of the memory backing it.)
        * We could say it as, 'static means you have an infinite lease or indeed it can be kept around as long as you'd like.
        * 'static should really be renamed 'unbounded
        * A 'static bound on a type means that anything inside that types is 'static or owned, beacuse owned values by definition will live due to being owned.
    * https://github.com/pretzelhammer/rust-blog/blob/master/posts/common-rust-lifetime-misconceptions.md#2-if-t-static-then-t-must-be-valid-for-the-entire-program
    * It is confusing. 'static could live for the lifetime of the program. Something on the heap could live that long if never freed. I found it best to think of 'static as meaning not on the stack.
    * Lieftime parameters are used in two different ways in the language, with different but related meaning:

      As a Reference Parameter: &'a T means "this reference points at a value of type T which is known through that reference to be valid for at least the lifetime 'a". So &'static T would point at a T value that has to exist for the rest of the program (otherwise the reference would be invalid)
      As a Lifetime bound: U: 'a means "any value of type U is only valid for at most the lifetime 'a, but can be destroyed or created arbitrary otherwise". So U: 'static would mean that you can keep a value of type U around forever, but don't have to.
    * &'static means that it lives for the entire lifetime of the program.

      a + 'static bound means live as long as it needs to without being modified by anything else.
    * T: 'static means that a value of type T is allowed to live for 'static, not that it must.
    * example
        ```
        fn test_generic_bounds<T: 'static>(_t: T) {}

        fn main() {
            // s1 not static
            let s1 = String::from("hello");
            {
                // s2 also not static
                let s2 = s1[..3].to_string();
                // But it passes the 'static bound check
                test_generic_bounds(s2);
            }
            // But it passes the 'static bound check
            test_generic_bounds(s1);
        }
        ```
    * example
        ```
        use std::sync::{Arc, RwLock};
        use std::thread;

        fn test<T>(data: Arc<RwLock<T>>)
        where
            T: 'static + Send + Sync,
        {
            // Spawn a worker thread
            let handle = thread::spawn(move || {
                // Access the data inside the RwLock
                let read_guard = data.read().unwrap();
                println!("Worker Thread: Value inside RwLock: {:?}", *read_guard);
            });

            // Wait for the worker thread to finish
            handle.join().unwrap();
        }

        fn main() {
            let x = 42; // Some data, in this case, an integer

            // Create an Arc containing an RwLock with a reference to the data
            let shared_data = Arc::new(RwLock::new(x));

            // Call the test function with the shared data
            test(shared_data);

            // At this point, shared_data, including the RwLock and the data, still exists.
            // It will be deallocated when it goes out of scope or when the program ends.
        }
        ```
        * let x = 0; allocates an integer on the stack with the value 0.
          test(Arc::new(RwLock::new(&x))); creates an Arc (atomic reference counting) and an RwLock wrapping a reference to x. The Arc ensures that the reference count of the data it wraps is incremented.
          Thread 1:
          foo returns, and the local variable x goes out of scope.
          The reference count of the Arc is still greater than zero because it is owned by the RwLock and passed to test. Therefore, the memory is not deallocated.
          Thread 2:
          test spawns Thread 2, which will operate on the shared data inside the RwLock.
          If Thread 2 attempts to access x after Thread 1 has returned, it may encounter a use-after-free error because x is no longer in scope in Thread 1.
          Potential Issue:
          If Thread 2 attempts to access x after Thread 1 has returned, it might access a memory location that is no longer valid because x has gone out of scope.
          This is a classic example of a lifetime issue and can lead to undefined behavior, crashes, or data corruption.
          Solution:
          To prevent this issue, the T: 'static bound is used. It enforces that the data wrapped by the Arc must have the 'static lifetime, meaning it lives for the entire duration of the program.
          If you have data with a shorter lifetime, like a local variable in a function (x in this case), you need to ensure that it outlives all the threads that may access it.
        * T could still internally borrow something, and that thing could exist for less than the theoretical max life time of the thread you’re spawning (up to the end of the program). Add a + ‘static toT bound or use thread::scope, whichever one is more manageable.\
    * example
        ```
        fn main() {
            { // // this block has lifetime 'a
                let t = 3;
                let r = &t;
                let a = Arc::new(RwLock::new(r)); // not compile:  borrowed value does not live long enough
                test(a);
            } // `t` dropped here while still borrowed
        ```
        * ifetime 'a:
          The outer block is annotated with the lifetime 'a, indicating that any references created within this block should have a lifetime no longer than 'a.
          Local Variable t:
          let t = 3; creates a local variable t with the value 3. This variable is specific to the current block and has a lifetime limited to this block.
          Reference r:
          let r = &t; creates a reference r to the value of t. The reference r also inherits the lifetime of t, which is limited to the current block.
          Arc<RwLock<T>>:
          let a = Arc::new(RwLock::new(r)); attempts to create an Arc containing an RwLock that wraps the reference r. This introduces a problem because the reference r has a lifetime tied to the local variable t.
          Problematic Situation:
          The Arc is passed to the test function, which spawns a worker thread. The worker thread may outlive the current block and attempt to access the shared reference r after it has gone out of scope.
          Undefined Behavior:
          When the worker thread accesses the shared reference after the original block has ended, it leads to undefined behavior. The reference r is no longer valid, as it points to memory that has been deallocated.
5. closures
6. cargo
7. smart pointers: box
    * Box, ref, & and *
    * Box is a library-defined smart pointer type; ref is a syntax for pattern matching; & is a reference operator, doubling as a sigil in reference types; * is a dereference operator, doubling as a sigil in raw pointer types
    * references and raw pointers:
        &T        - immutable (shared) reference
        &mut T    - mutable (exclusive) reference

        *const T  - immutable raw pointer
        *mut T    - mutable raw pointer
        * The difference between the last two is very thin, because either can be cast to another without any restrictions, so const/mut distinction there serves mostly as a lint.
        * Naturally, this is not so for references - reference types and their interaction define one of the key feature of Rust: borrowing.
            * References have a lot of restrictions on how and when they could be created, how they could be used and how they interact with each other. In return, they can be used without unsafe blocks.
    * The dereference operator is often omitted, because another operator, the "dot" operator (i.e., .), automatically references or dereferences its left argument.
        ```
        struct X { n: u32 };

        impl X {
            fn method(&self) -> u32 { self.n } // self.n automatically dereferences it, so you won't have to type (*self).n
        }
        ```
    * compiler automatically references x in x.method(), so you won't have to write (&x).method()
        ```
        let x = X { n: 12 };
        let n = x.method();
        ```
    * A fat pointer is basically a structure which contains the actual pointer to the piece of data and some additional information (length for slices, pointer to vtable for trait objects)
        * However, only a pointer is not enough - additional information is needed, for example, length for slices or a pointer to a virtual methods table for trait objects.
        * This information is "embedded" in pointers to unsized types, making these pointers "fat".

8. pattern matching
1. String vs str
    * As a rule of thumb: functions that do something with a string without needing to stash it away somewhere should take &str, functions that modify the String should take String, functions that need to store it for later should take String.
    * See if you can use Arc<str> instead of String. This would make using .clone() faster because it wouldn't actually clone the strings at the cost of not being able to mutate the strings.
    * Accepting &str in your function allows the caller to call the function multiple times without reallocating the string. Whereas taking String means the caller has to give you a brand new allocation with each call (since String always manages a heap allocation).
    * Prefer &str if you are only reading it.
    * String owns the data, &str references an already existing buffer. If you need ownership all the time, use String. If you don't want to copy, and know that you don't have to outlive the buffer use &str. If you want to have a little of both, use Cow. If you want maximum flexibility use Rc and the clone on write make_mut method.
    * Literals evaluate to type &'static str, (and byte literals to &'static [u8]). The compiler copies the literal into the crate's read-only static space and generates a forever-valid reference to that value.
    * Can't create a String at compile-time.  String owns a heap allocation, which happens at runtime, so there must be a runtime function call to do that allocation. They look like String::from("literal") in the source
    * In short, &static str > &str > String

      String is the simplest to work with because you own it: you can modify it, extend it etc ... The drawback is its performance: it is very slow (allocated on the heap). You can make an analogy with Vec.

      &str (on the stack) is much faster than String. But you cannot modify it. This is only a borrowed version of a String, which means you cannot create new &str out of nothing. You can make an analogy with &[]

      &'static str is the fastest one but also the less flexible: not only you cannot modify it but you need to know its value at compile time! You can make an analogy with [;N] EDIT: &'static []
1. async