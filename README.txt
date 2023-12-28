
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
    * https://stackoverflow.com/questions/24158114/what-are-the-differences-between-rusts-string-and-str
    * https://chrismorgan.info/blog/rust-fizzbuzz/
    * https://stackoverflow.com/questions/31012923/what-is-the-difference-between-copy-and-clone
    * https://stackoverflow.com/questions/65434252/how-to-return-a-reference-to-a-value-from-hashmap-wrappered-in-arc-and-mutex-in
    * https://www.reddit.com/r/rust/comments/17luh6c/how_can_i_avoid_cloning_everywhere/
    * https://www.reddit.com/r/rust/comments/vy9zvw/the_docs_say_hashmap_is_send_sync_how_can_that_be/

1. general
    * allocates on stack by default
    * to test online: https://play.rust-lang.org/
1. ownership and borrowing
    * Paring these principles down to the simplest possible examples:
      let mut x = 10;
      let r1 = &x;
      let r2 = &x; // ok: multiple shared borrows permitted
      x += 10; // error: cannot assign to `x` because it is borrowed
      let m = &mut x; // error: cannot borrow `x` as mutable because it is
      // also borrowed as immutable
      println!("{}, {}, {}", r1, r2, m); // the references are used here,
      // so their lifetimes must last
      // at least this long

      let mut y = 20;
      let m1 = &mut y;
      let m2 = &mut y; // error: cannot borrow as mutable more than once
      let z = y; // error: cannot use `y` because it was mutably borrowed
    • You can move values from one owner to another. This allows you to build,
    rearrange, and tear down the tree.
    • Very simple types like integers, floating-point numbers, and characters are
    excused from the ownership rules. These are called Copy types.
    • The standard library provides the reference-counted pointer types Rc and Arc,
    which allow values to have multiple owners, under some restrictions.
    • You can “borrow a reference” to a value; references are non-owning pointers,
    with limited lifetimes.
    * moves
        * In Rust, for most types, operations like assigning a value to a variable, passing it to a
          function, or returning it from a function don’t copy the value: they move it.
        * The
          source relinquishes ownership of the value to the destination and becomes uninitial‐
          ized; the destination now controls the value’s lifetime.
        * in Rust, assignments of most types move the value from the source to
          the destination, leaving the source uninitialized.
            let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
            let t = s;
            let u = s;
        * Passing arguments to functions moves ownership to the function’s parameters;
        * returning a value from a
          function moves ownership to the caller.
        * First, the moves always apply to the value proper, not the heap storage
          they own.
          * For vectors and strings, the value proper is the three-word header alone; the
            potentially large element arrays and text buffers sit where they are in the heap.
        * Sec‐
          ond, the Rust compiler’s code generation is good at “seeing through” all these moves;
          in practice, the machine code often stores the value directly where it belongs.
        * Rust
          suggests using a reference, in case you want to access the element without moving it.
          ```
          let mut v = Vec::new();
          for i in 101 .. 106 {
          v.push(i.to_string());
          }
          // Pull out random elements from the vector.
          // For this to work, Rust would somehow need to remember that the third and fifth ele‐
             ments of the vector have become uninitialized, and track that information until the
             vector is dropped.
          let third = v[2]; // error: Cannot move out of index of Vec
          let fifth = v[4]; // here too
          ```
    * Every value has a single owner that determines its
      lifetime.
      * When the owner is freed—dropped, in Rust terminology—the owned value is
        dropped too.
    * A variable owns its value.
      * When control leaves the block in which the variable is
        declared, the variable is dropped, so its value is dropped along with it.
    * example
        borrow
        ```
        let numbers = vec![10, 20, 30, 40, 50];
        let mut d = numbers[0];

        for m in &numbers[1..] { // borrow
            d = gcd(d, *m); // borrow
        }
        ```
        vs (move)
        ```
        for m in numbers[1..] { // drains numbers
            d = gcd(d, m); // takes ownership
        }
        ```
1. crate, cargo, modules
    * Each crate is a complete, cohesive unit: all the
      source code for a single library or executable, plus any associated tests, examples,

      tools, configuration, and other junk.
    * Cargo.toml
        [dependencies]
        num = "0.4"
        image = "0.13"
        crossbeam = "0.8"
    * use num::Complex;
      // ...
      use image::ColorType;
      use image::png::PNGEncoder;
    * We found these crates on crates.io, the Rust community’s site for open
      source crates.
    * transitive dependencies
    * To evolve without breaking existing code, Rust uses editions.
        * The 2015 edition of Rust
          is compatible with Rust 1.0. The 2018 edition changed async and await into key‐
          words, streamlined the module system, and introduced various other language
          changes that are incompatible with the 2015 edition.
        * programs can freely mix crates written in different editions.
            * It’s even fine for a
              2015 edition crate to depend on a 2018 edition crate.
            * In other words, a crate’s edition
              only affects how its source code is construed; edition distinctions are gone by the
              time the code has been compiled.
            * This means there’s no pressure to update old crates
              just to continue to participate in the modern Rust ecosystem.
        [package]
      name = "rust-rocket-workshop"
      version = "0.1.0"
      edition = "2021"
    * modules
        * They act as Rust’s namespaces, containers for the func‐
          tions, types, constants, and so on that make up your Rust program or library.
        * mod spores {

          }
        * The pub keyword makes an item public, so it can be
          accessed from outside the module.
        * One function is marked pub(crate), meaning that it is available anywhere inside this
          crate, but isn’t exposed as part of the external interface.
          * It can’t be used by other
            crates, and it won’t show up in this crate’s documentation.
        * Anything that isn’t marked pub is private and can only be used in the same module in
          which it is defined, or any child modules:
        * Modules can nest, and it’s fairly common to see a module that’s just a collection of
          submodules
        * It’s also possible to specify pub(super), making an item visible to the parent module
          only, and pub(in <path>), which makes it visible in a specific parent module and its
          descendants.
        * A module can have its own directory. When Rust sees mod spores;, it checks for
          both spores.rs and spores/mod.rs; if neither file exists, or both exist, that’s an error.
1. structs
    * copy vs clone
        * Assigning a value of a Copy type
          copies the value, rather than moving it.
          * Passing Copy types to functions
            and constructors behaves similarly.
        * Only types for which a simple bit-for-bit copy suffices can be Copy.
            * As we’ve already
              explained, String is not a Copy type, because it owns a heap-allocated buffer. For sim‐
              ilar reasons, Box<T> is not Copy; it owns its heap-allocated referent.
            * The File type, representing an operating system file handle, is not Copy; duplicating such a value
                             would entail asking the operating system for another file handle.
            * Similarly, the
              MutexGuard type, representing a locked mutex, isn’t Copy: this type isn’t meaningful to
              copy at all, as only one thread may hold a mutex at a time.
            * As a rule of thumb, any type that needs to do something special when a value is drop‐
              ped cannot be Copy: a Vec needs to free its elements, a File needs to close its file han‐
              dle, a MutexGuard needs to unlock its mutex, and so on.
            * By default, struct and enum types are not
              Copy:
              * If all the fields of your
                struct are themselves Copy, then you can make the type Copy as well by placing the
                attribute #[derive(Copy, Clone)] above the definition,
              * However, if we try
                this on a type whose fields are not all Copy, it doesn’t work.
                #[derive(Copy, Clone)]
                struct StringLabel { name: String }
                // the trait `Copy` may not be implemented for this type
                // this field does not implement `Copy`
              * So making a type Copy represents a serious commitment
                on the part of the implementer: if it’s necessary to change it to non-Copy later, much
                of the code that uses it will probably need to be adapted.
                * Copy types are very limited
                  in which types they can contain, whereas non-Copy types can use heap allocation and
                  own other sorts of resources.

        * Clone is designed for arbitrary duplications: a Clone implementation for a type T can do arbitrarily complicated operations required to create a new T. It is a normal trait (other than being in the prelude), and so requires being used like a normal trait, with method calls, etc.

          The Copy trait represents values that can be safely duplicated via memcpy: things like reassignments and passing an argument by-value to a function are always memcpys, and so for Copy types, the compiler understands that it doesn't need to consider those a move.
        * Copy is implicit, inexpensive, and cannot be re-implemented (memcpy).
          Clone is explicit, may be expensive, and may be re-implement arbitrarily.
        * moves vs automatic copies
            ```
            #[derive(Debug, Clone, Copy)]
            pub struct PointCloneAndCopy {
                pub x: f64,
            }

            #[derive(Debug, Clone)]
            pub struct PointCloneOnly {
                pub x: f64,
            }

            fn test_copy_and_clone() {
                let p1 = PointCloneAndCopy { x: 0. };
                let p2 = p1; // because type has `Copy`, it gets copied automatically.
                println!("{:?} {:?}", p1, p2);
            }

            fn test_clone_only() {
                let p1 = PointCloneOnly { x: 0. };
                let p2 = p1; // because type has no `Copy`, this is a move instead; to avoid the implicit move, we could explicitly call let p2 = p1.clone();
                println!("{:?} {:?}", p1, p2); //
            }
            ```
        * The Copy trait in rust defines the ability to implicitly copy an object. The behavior Copy is not overloadable. It is always a simple bitwise copy.
    * what to use inside struct: rule of thumb
    * As a rule of thumb, you should never put the &str type in a struct. Lifetimes on structs should only be used when the struct is a "view" or "cursor" that looks inside some other struct, which is not what is happening here.
    * You need to ask yourself "Does the struct own this data?". If so, you go with 'static (unborrowed) fields and if not, you go with references. If ownership is shared, you go with Rc/Weak/Arc depending on the kind of ownership. And if it's possibly shared, you go with Cow.
    * I'd just default to String for struct fields (unless it's a constant string literal so that you can use &'static str. &str is easy for read-only function parameters, but it's a pain for structs, so I generally only use it if I really need it.
1. error: unrecoverable: panic, recoverable: result
    * // Errors should implement the std::error::Error trait,
      // but the default definitions for the Error methods are fine.
      impl std::error::Error for JsonError { }
    * The #[derive(Error)] directive tells thiserror to generate the code shown earlier,
      which can save a lot of time and effort.
    ```
    let output = match File::create(filename) {
    Ok(f) => f,
    Err(e) => {
    return Err(e);
    }
    };
    ```
    vs
    ```
    let output = File::create(filename)?;
    ```
    This kind of match statement is such a common pattern in Rust that the language
    provides the ? operator as shorthand for the whole thing.
    * Panic
        * Panic is for the other kind of error, the kind that should never happen.
        * • Calling .expect() on a Result that happens to be Err
        * Integer division by zero
        * (There’s also the macro panic!(), for cases where your own code discovers that it has
          gone wrong, and you therefore need to trigger a panic directly. panic!() accepts
          optional println!()-style arguments, for building an error message.)
        * A good rule of thumb is: “Don’t panic.”
        * It’s more like a RuntimeException in Java
        * Panic is per thread. One thread can be panicking while other threads are going on
          about their normal business.
          * If the panicking thread was the main thread, then the
            whole process exits (with a nonzero exit code).
        * There is also a way to catch stack unwinding, allowing the thread to survive and con‐
          tinue running.
            * The standard library function std::panic::catch_unwind() does
              this.
            * this is the mechanism used by Rust’s test har‐
              ness to recover when an assertion fails in a test.
    * Aborting
        * Also, Rust’s panic behavior is customizable. If you compile with -C panic=abort, the
          first panic in your program immediately aborts the process.
    * Result
        * fn get_weather(location: LatLng) -> Result<WeatherReport, io::Error>
        * Rust’s equivalent of try/catch in other languages
            match get_weather(hometown) {
            Ok(report) => {
            display_weather(hometown, &report);
            }
            Err(err) => {
            println!("error querying the weather: {}", err);
            schedule_weather_retry();
            }
            }
1. traits, lifetimes
    * lifetime
        * Rust tries to assign each reference type in your program a lifetime that meets the con‐
          straints imposed by how it is used.
        * A lifetime is some stretch of your program for
          which a reference could be safe to use: a statement, an expression, the scope of some
          variable, or the like.
        * Lifetimes are entirely figments of Rust’s compile-time imagina‐
          tion.
            * At run time, a reference is nothing but an address; its lifetime is part of its type
              and has no run-time representation.
        * example
            ```
            let v = vec![1, 2, 3];
            let r = &v[1];
            ```
            max limit: the lifetime of v must enclose that
            of the reference type of &v[1]
            min limit: if you store a reference in some data struc‐
            ture, its lifetime must enclose that of the data structure
        * Conversely, if we do see a function with a signature like g(p: &i32) (or with the life‐
          times written out, g<'a>(p: &'a i32)), we can tell that it does not stash its argument
          p anywhere that will outlive the call.
        * When a func‐
          tion takes a single reference as an argument and returns a single reference, Rust
          assumes that the two must have the same lifetime.
          * fn smallest<'a>(v: &'a [i32]) -> &'a i32 { ... }
        * This says that r can only refer to i32 values that will last for the lifetime of the pro‐
          gram, which is rather limiting.
            struct S {
            r: &'static i32
            }
        * Every type in Rust has a
          lifetime, including i32 and String.
          * Most are simply 'static, meaning that values of
            those types can live for as long as you like; for example, a Vec<i32> is self-contained
            and needn’t be dropped before any particular variable goes out of scope.
          * But a type
            like Vec<&'a i32> has a lifetime that must be enclosed by 'a: it must be dropped
            while its referents are still alive.
    * lifetime elision
        * In the simplest cases, you may never need to write out lifetimes for your parameters.
          Rust just assigns a distinct lifetime to each spot that needs one.
    * utility traits
        * Any type that implements the
            FromStr trait has a from_str method that tries to parse a value of that type from a
            string
        * Send, Sync
            * Send type can be moved to different thread, Sync type's reference can be moved to different thread.
            * A type being Send means it can be moved across thread boundaries. This means there is always exactly one owner even as the thread changes.
            * A type being Sync means it can be shared between threads. This means a value can be borrowed from multiple threads.
            * Note that those are exclusive: in Rust, a value cannot be moved while borrowed, and you cannot borrow a value you no longer have.
            * The docs say HashMap is Send + Sync-- how can that be?
                * It is Send because it can be sent between threads safely.

                  It is Sync because a &HashMap can be sent between threads safely.
            * Personally, I find it easiest to understand these traits by thinking about types that don't implement them.
                * For example, Rc is not Send, because it uses non-atomic operations to manage its reference count, which would lead to a data race if an Rc were moved to another thread.
                * Similarly, RefCell is not Sync, because it allows unsynchronized mutation of its state.
            * You can read the hashmap, string and pretty much any other ‘simple’ structure concurrently from as many threads as you’d like. At the same time, if you have an exclusive reference, you can modify such structures from whatever thread because there’s only one thread accessing it.
        * Deref
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
1. closures
    || {body} when no param
    |a| {body} with param
    * Note that, unlike functions declared with fn, we don’t need to declare the
      types of a closure’s arguments; Rust will infer them, along with its return type.
    * For example, the function
      cmp_by_timestamp_then_name could not use v directly. (Rust also has closures, which
      do see into enclosing scopes.
        let mut v = vec![];
        ...
        fn cmp_by_timestamp_then_name(a: &FileInfo, b: &FileInfo) -> Ordering {
        a.timestamp.cmp(&b.timestamp) // first, compare timestamps
        .reverse() // newest file first
        .then(a.path.cmp(&b.path)) // compare paths to break ties
        }
1. cargo
1. references, smart pointers: box
    * However, Rust also
      includes two kinds of fat pointers, two-word values carrying the address of some
      value, along with some further information necessary to put the value to use.
      * A reference to a slice is a fat pointer, carrying the starting address of the slice and its
        length.
      * Rust’s other kind of fat pointer is a trait object, a reference to a value that implements
        a certain trait.
        * A trait object carries a value’s address and a pointer to the trait’s imple‐
          mentation appropriate to that value, for invoking the trait’s methods.
    * Rust permits references to references:
        ```
        struct Point { x: i32, y: i32 }
        let point = Point { x: 1000, y: 729 };
        let r: &Point = &point;
        let rr: &&Point = &r;
        let rrr: &&&Point = &rr;
        ```
        * Like the . operator, Rust’s comparison operators “see through” any number of refer‐
          ences:
          * the == operator follows all the references and performs
            the comparison on their final targets, x and y.
          * If you actually want to know
            whether two references point to the same memory, you can use std::ptr::eq, which
            compares them as addresses:
            ```
            assert!(rx == ry); // their referents are equal
            assert!(!std::ptr::eq(rx, ry)); // but occupy different addresses
            ```
          * Note that the operands of a comparison must have exactly the same type, including
            the references:
            assert!(rx == rrx); // error: type mismatch: `&i32` vs `&&i32`
            assert!(rx == *rrx); // this is okay
    * Rust references are never null.
    * Rust won’t convert integers to references
      (outside of unsafe code), so you can’t convert zero into a reference.
    * But you might recall that, when we fixed the show function to take the table of artists
      by reference instead of by value, we never had to use the * operator.
      * Since references are so widely used in Rust, the . operator implicitly dereferences its
        left operand
      The . operator can also implicitly borrow a reference to its left operand, if needed for
      a method call.
    * When we pass a value to a function in a way that moves ownership of the value to the
      function, we say that we have passed it by value.
    * If we instead pass the function a ref‐
      erence to the value, we say that we have passed the value by reference.
    * The right way to handle this is to use references. A reference lets you access a value
      without affecting its ownership.
      * example with hashmap
      * References come in two kinds:
        • A shared reference lets you read but not modify its referent. However, you can
        have as many shared references to a particular value at a time as you like. The
        expression &e yields a shared reference to e’s value; if e has the type T, then &e has
        the type &T, pronounced “ref T.” Shared references are Copy.
        • If you have a mutable reference to a value, you may both read and modify the
        value. However, you may not have any other references of any sort to that value
        active at the same time. The expression &mut e yields a mutable reference to e’s
        value; you write its type as &mut T, which is pronounced “ref mute T.” Mutable
        references are not Copy.
        * You can think of the distinction between shared and mutable references as a way to
          enforce a multiple readers or single writer rule at compile time.
          * In fact, this rule doesn’t
            apply only to references; it covers the borrowed value’s owner as well.
          * well. As long as there
            are shared references to a value, not even its owner can modify it; the value is locked
            down.
          * Nobody can modify table while show is working with it.
    * Rust also has non-owning pointer types called ref‐
      erences, which have no effect on their referents’ lifetimes.
      * references must never outlive their referents.
      * You must
        make it apparent in your code that no reference can possibly outlive the value it
        points to.
      * To emphasize this, Rust refers to creating a reference to some value as bor‐
        rowing the value: what you have borrowed, you must eventually return to its owner.
    * A reference-counting loop; these objects will not be freed
        * It is possible to leak values in Rust this way, but such situations are rare. You cannot
          create a cycle without, at some point, making an older value point to a newer value.
          This obviously requires the older value to be mutable. Since Rc pointers hold their
          referents immutable, it’s not normally possible to create a cycle.
          * interior mutability
            * If you com‐
              bine those techniques with Rc pointers, you can create a cycle and leak memory.
            * You can sometimes avoid creating cycles of Rc pointers by using weak pointers,
              std::rc::Weak, for some of the links instead.
    * Box<T>
        * A Box<T> is a pointer to a
          value of type T stored on the heap. Calling Box::new(v) allocates some heap space,
          moves the value v into it, and returns a Box pointing to the heap space.

    * fat pointers
    * In
      Java, if class Rectangle contains a field Vector2D upperLeft;, then upperLeft is a
      reference to another separately created Vector2D object. Objects never physically
      contain other objects in Java.
    * References
        * A value of type &String (pronounced “ref String”) is a reference to a String value, a
          &i32 is a reference to an i32, and so on.
        * It’s easiest to get started by thinking of references as Rust’s basic pointer type.
        * At run
          time, a reference to an i32 is a single machine word holding the address of the i32,
          which may be on the stack or in the heap.
        * The expression &x produces a reference to
          x; in Rust terminology, we say that it borrows a reference to x.
        * Given a reference r, the
          expression *r refers to the value r points to.
        * Unlike C pointers, however, Rust references are never null: there is simply no way to
          produce a null reference in safe Rust.
        * And unlike C, Rust tracks the ownership and
          lifetimes of values, so mistakes like dangling pointers, double frees, and pointer inva‐
          lidation are ruled out at compile time.
        * Rust references come in two flavors:
            * &T
                * An immutable, shared reference.
                * You can have many shared references to a given
                  value at a time, but they are read-only: modifying the value they point to is for‐
                  bidden, as with const T* in C.
            * &mut T
                * A mutable, exclusive reference.
                * You can read and modify the value it points to, as
                  with a T* in C.
                * But for as long as the reference exists, you may not have any other
                  references of any kind to that value.
                * Rust uses this dichotomy between shared and mutable references to enforce a “single
                  writer or multiple readers” rule: either you can read and write the value, or it can be
                  shared by any number of readers, but never both at the same time.
    * Boxes
        * The simplest way to allocate a value in the heap is to use Box::new:
        * When b goes
          out of scope, the memory is freed immediately, unless b has been moved—by return‐
          ing it, for example.
            * let b = Box::new(t);
        * Moves are essential to the way Rust handles heap-allocated values;
    * Raw Pointers
        * Rust also has the raw pointer types *mut T and *const T.
        * Raw pointers really are just
          like pointers in C++.
        * Using a raw pointer is unsafe, because Rust makes no effort to
          track what it points to.
        * For example, raw pointers may be null, or they may point to
          memory that has been freed or that now contains a value of a different type.
        * However, you may only dereference raw pointers within an unsafe block.
    * The types &[T] and &mut [T], called a shared slice of Ts and mutable slice of Ts, are
      references to a series of elements that are a part of some other value, like an array
      or vector.
      * You can think of a slice as a pointer to its first element, together with a
        count of the number of elements you can access starting at that point.

    * std::mem::swap
        fn swap<T>(x: &mut T, y: &mut T); // shorthand for -> ()
        The standard
        library’s std::mem::swap function has no meaningful return value; it just exchanges
        the values of its two arguments.
    * all you need to know is that &x borrows a reference to x, and
      that *r is the value that the reference r refers to.
    * What if Command / Query separation is the answer? When you run a command to change data, move the memory around (no reference &); when you run a query to read data, use references.
        * When you create / insert into a data structure, you move the data in.
        * Reading is referencing, removing is moving data out, updating is a mem::replace or swap; iteration is usually done with references.
    * I pretty much never call clone() except on Rcs and Arcs
    * returns a value referencing data owned by the current function
        * How to return a reference to a value from Hashmap wrappered in Arc and Mutex in Rust?
        ```
        use std::sync::{Arc,Mutex};
        use std::collections::HashMap;

        struct Hey{
            a:Arc<Mutex<HashMap<String, String>>>
        }


        impl Hey {
            fn get(&self,key:&String)->&String{
                self.a.lock().unwrap().get(key).unwrap() // compilation error: returns a value referencing data owned by the current function
            }
        }
        ```
        * If that return type were allowed to point inside the Mutex's data, there would be nothing stopping other code from locking the mutex and deleting the entry, meaning that the returned reference would point at something that was deallocated
        * If it's allowed, the map might change due to the operation from another thread, and it's UB
    * Cannot return reference to temporary value
        ```
        struct MyStruct {
            data: String,
        }

        fn create_struct() -> &'static MyStruct {
            let my_struct = MyStruct {
                data: String::from("Hello, Rust!"),
            };

            &my_struct // Error: Cannot return reference to temporary value
        }
        ```
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

1. pattern matching
    * Rust prohibits match expressions that do not cover all possible values:
1. String vs str
    * String literals are enclosed in double quotes.
        * let speech = "\"Ouch!\" said the well.\n";
    * Rust strings are sequences of Unicode characters, but they are not stored in memory
      as arrays of chars.
      * Instead, they are stored using UTF-8, a variable-width encoding.
      * Each ASCII character in a string is stored in one byte.
      * A String has a resizable buffer holding UTF-8 text.
      * The buffer is allocated on the
        heap, so it can resize its buffer as needed or requested.
      * You can think of a String as a Vec<u8> that is guaranteed to hold well-formed UTF-8;
        * in fact, this is how String is implemented.
    * A &str (pronounced “stir” or “string slice”) is a reference to a run of UTF-8 text
      owned by someone else: it “borrows” the text.
      * Like other slice references, a &str is a fat pointer, containing both the
        address of the actual data and its length.
      * The type &mut str does exist, but it is not very useful, since almost any operation on
        UTF-8 can change its overall byte length, and a slice cannot reallocate its referent.
        * In
          fact, the only operations available on &mut str are make_ascii_uppercase and
          make_ascii_lowercase, which modify the text in place and affect only single-byte
          characters, by definition.
    * &str is very much like &[T]: a fat pointer to some data. String is analogous to
      Vec<T>,
    * There are several ways to create Strings:
        * The .to_string() method converts a &str to a String. This copies the string:
          let error_message = "too many pets".to_string();
          The .to_owned() method does the same thing, and you may see it used the same
          way. It works for some other types as well,
        * The format!() macro works just like println!(), except that it returns a new
          String instead of writing text to stdout, and it doesn’t automatically add a new‐
          line at the end:
        * Arrays, slices, and vectors of strings have two methods, .concat()
          and .join(sep), that form a new String from many strings:
    * Keep in mind that, given the nature of Unicode, simple char-by-char comparison
      does not always give the expected answers.
      * For example, the Rust strings "th\u{e9}"
        and "the\u{301}" are both valid Unicode representations for thé, the French word
        for tea. Unicode says they should both be displayed and processed in the same way,
        but Rust treats them as two completely distinct strings.
    * So what is a String? That's three words; two are the same as for &str but it adds a third word which is the capacity of the str buffer on the heap, always on the heap (a str is not necessarily on the heap) it manages before it's filled and has to re-allocate. the String basically owns a str as they say; it controls it and can resize it and reallocate it when it sees fit. So a String is as said closer to a &str than to a str.
    * Another thing is a Box<str>; this also owns a str and its runtime representation is the same as a &str but it also owns the str unlike the &str but it cannot resize it because it does not know its capacity so basically a Box<str> can be seen as a fixed-length String that cannot be resized (you can always convert it into a String if you want to resize it).
    * &str is super useful to be able to to have multiple different substrings of a String without having to copy; as said a String owns the str on the heap it manages and if you could only create a substring of a String with a new String it would have to be copied because everything in Rust can only have one single owner to deal with memory safety. So for instance you can slice a string:
        * let string: String   = "a string".to_string();
        * let substring1: &str = &string[1..3];
        * let substring2: &str = &string[2..4];
    * String:

      Rust's owned String type, the string itself lives on the heap and therefore is mutable and can alter its size and contents.
      Because String is owned when the variables which owns the string goes out of scope the memory on the heap will be freed.
      Variables of type String are fat pointers (pointer + associated metadata)
      The fat pointer is 3 * 8 bytes (wordsize) long and consists of the following 3 elements:
      Pointer to actual data on the heap, it points to the first character
      Length of the string (# of characters)
      Capacity of the string on the heap
    * &str:

      Rust's non-owned String type, it's immutable by default. The string itself lives somewhere else in memory usually on the heap or in 'static memory.
      Because String is non-owned when &str variables go out of scope the memory of the string will not be freed.
      Variables of type &str are fat pointers (pointer + associated metadata)
      The fat pointer is 2 * 8 bytes (wordsize) long and consists of the following 2 elements:
      Pointer to actual data on the heap, it points to the first character
      Length of the string (# of characters)
    * String is an Object.

      &str is a pointer at a part of object.
    * For C# and Java people:

      Rust' String === StringBuilder
      Rust's &str === (immutable) string
    * String is the dynamic heap string type, like Vec: use it when you need to own or modify your string data.
    * str is an immutable1 sequence of UTF-8 bytes of dynamic length somewhere in memory.
    * In summary, use String if you need owned string data (like passing strings to other threads, or building them at runtime), and use &str if you only need a view of a string.
    * similar to the relationship between by-value T and by-reference &T for general types
    * Statically allocated objects are normally stored neither on the heap, nor the stack, but in their own region of memory
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
    * If you know a little more about Rust types, you may wonder why String is used instead of Box<str>
        * The answer lies in the difference between the two types: Box<str> is a fixed‐size allocation, so you’d need to reallocate every time you push to the string, but String overallocates for efficiency in the usual case, storing a capacity member also to track that
    * Rust “raw
      string” syntax: the letter r, zero or more hash marks (that is, the # character), a dou‐
      ble quote, and then the contents of the string, terminated by another double quote
      followed by the same number of hash marks.
        * Any character may occur within a raw
          string without being escaped, including double quotes; in fact, no escape sequences
          like \" are recognized.
1. async
1. mutability
    * Iterating over a mut reference provides a mut reference to each element:
        for rs in &mut strings { // the type of rs is &mut String
        rs.push('\n'); // add a newline to each string
        }
    * let mut numbers = Vec::new();
    Even though vectors are designed to be grown and shrunk dynamically,
      we must still mark the variable mut for Rust to let us push numbers onto the end of it.
    * Rust’s rules for mutation and sharing:
      Shared access is read-only access.
      Values borrowed by shared references are read-only. Across the lifetime of a
      shared reference, neither its referent, nor anything reachable from that referent,
      can be changed by anything. There exist no live mutable references to anything in
      that structure, its owner is held read-only, and so on. It’s really frozen.
      Mutable access is exclusive access.
      A value borrowed by a mutable reference is reachable exclusively via that refer‐
      ence. Across the lifetime of a mutable reference, there is no other usable path to
      its referent or to any value reachable from there. The only references whose life‐
      times may overlap with a mutable reference are those you borrow from the muta‐
      ble reference itself.
1. debug vs Display
    * The #[derive(Debug)] attribute tells the compiler to generate some extra code that
      allows us to format the Arguments struct with {:?} in println!.
1. collections
    * A Vec<T> consists of three values: a pointer to the heap-allocated buffer for the ele‐
      ments, which is created and owned by the Vec<T>; the number of elements that buffer
      has the capacity to store; and the number it actually contains now (in other words, its
      length).
        * When the buffer has reached its capacity, adding another element to the vec‐
          tor entails allocating a larger buffer, copying the present contents into it, updating the
          vector’s pointer and capacity to describe the new buffer, and finally freeing the old
          one.
        * popping a value from a Vec<T> returns an Option<T>: None if the vector was already
          empty, or Some(v) if its last element had been v:
    * Slices
        * A slice, written [T] without specifying the length, is a region of an array or vector.
        * A reference to a slice is a fat pointer: a two-word value comprising a pointer to the
          slice’s first element, and the number of elements in the slice.
        * a reference
          to a slice is a non-owning pointer to a range of consecutive values in memory.
        * fn print(n: &[f64]) { // you can apply it to either a vector or an array,
            print(&a); // works on arrays
            print(&v); // works on vectors
        * shown. In fact, many methods you might think of as belonging
          to vectors or arrays are methods defined on slices: for example, the sort and reverse
          methods, which sort or reverse a sequence of elements in place, are actually methods
          on the slice type [T].
    * HashMap
        * for (artist, works) in table
        * In particular, HashMap is not Copy—it can’t be, since it
          owns a dynamically allocated table.
        * Iterating over a
          shared reference to a HashMap is defined to produce shared references to each entry’s
          key and value: artist has changed from a String to a &String, and works from a
          Vec<String> to a &Vec<String>.
1. syntax
    * shadowing of let
    * if let
        if let pattern = expr {
        block1
        } else {
        block2
        }
        The given expr either matches the pattern, in which case block1 runs, or doesn’t
        match, and block2 runs.
        * example
            if let Some(cookie) = request.session_cookie {
            return restore_session(cookie);
            }
        * An if let expression is shorthand for a match with just one pattern:
          match expr {
          pattern => { block1 }
          _ => { block2 }
          }
    * return without a value is shorthand for return ():
        * The body of a function
          works like a block expression: if the last expression isn’t followed by a semicolon, its
          value is the function’s return value.
        * the ? operator to
          check for errors after calling a function that can fail:
          let output = File::create(filename)?;
          We explained that this is shorthand for a match expression:
          let output = match File::create(filename) {
          Ok(f) => f,
          Err(err) => return Err(err)
          };
    * Expressions that don’t finish
      normally are assigned the special type !, and they’re exempt from the rules about
      types having to match.
      fn exit(code: i32) -> !
      The ! means that exit() never returns. It’s a divergent function.
      fn serve_forever(socket: ServerSocket, handler: ServerHandler) -> ! {
        socket.listen();
        loop {
        let s = socket.accept();
        handler.handle(s);
        }
      }
    * dot
        * The unary * operator is used to access the value pointed to by a reference. As we’ve
          seen, Rust automatically follows references when you use the . operator to access a
          field or method, so the * operator is necessary only when we want to read or write the
          entire value that the reference points to.
    * Converting a value from one type to another usually requires an explicit cast in Rust.
        • Values of type &String auto-convert to type &str without a cast.
        • Values of type &Vec<i32> auto-convert to &[i32].
        • Values of type &Box<Chessboard> auto-convert to &Chessboard.
    * ?
        * ? also works similarly with the Option type. In a function that returns Option, you
          can use ? to unwrap a value and return early in the case of None:
          let weather = get_weather(hometown).ok()?;
    * you can also change the type signature of main() to return a Result type,
      so you can use ?:
      fn main() -> Result<(), TideCalcError> {
      let tides = calculate_tides()?;
      print_tides(tides);
      Ok(())
      }