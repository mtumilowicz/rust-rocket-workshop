
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
    * https://stackoverflow.com/questions/26469715/how-do-i-write-a-rust-unit-test-that-ensures-that-a-panic-has-occurred
    * https://www.reddit.com/r/rust/comments/ui7ayd/why_does_rust_not_have_a_standard_async_runtime/
    * https://rustjobs.dev/blog/difference-between-string-and-str-in-rust/
    * https://doc.rust-lang.org/std
    * https://users.rust-lang.org/t/whats-the-difference-between-string-and-str/10177/2
    * http://xion.io/post/code/rust-patterns-ref.html
    * https://github.com/pretzelhammer/rust-blog/blob/master/posts/common-rust-lifetime-misconceptions.md
    * https://www.cs.brandeis.edu/~cs146a/rust/doc-02-21-2015/book/static-and-dynamic-dispatch.html

1. general
    * allocates on stack by default
    * to test online: https://play.rust-lang.org/
    * Rust’s default hashing algorithm is a well-known algorithm called SipHash-1-3.
      SipHash is fast, and it’s very good at minimizing hash collisions.
        * In fact, it’s a crypto‐
          graphic algorithm: there’s no known efficient way to generate SipHash-1-3 collisions.
        * As long as a different, unpredictable key is used for each hash table, Rust is secure
          against a kind of denial-of-service attack called HashDoS, where attackers deliber‐
          ately use hash collisions to trigger worst-case performance in a server.
1. ownership and borrowing
    * In
      Java, if class Rectangle contains a field Vector2D upperLeft;, then upperLeft is a
      reference to another separately created Vector2D object. Objects never physically
      contain other objects in Java.
    * Rust’s borrow system can’t protect
      you from deadlock.
      * The best protection is to keep critical sections small: get in, do
        your work, and get out.
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
                    * What we need is a little bit of mutable
                      data (a File) inside an otherwise immutable value (the SpiderRobot struct).
                    * Rust offers several flavors of it; in this section, we’ll discuss
                      the two most straightforward types: Cell<T> and RefCell<T>, both in the std::cell
                      module.
                      * A Cell<T> is a struct that contains a single private value of type T. The only special
                        thing about a Cell is that you can get and set the field even if you don’t have mut
                        access to the Cell itself:
                        fn set(&self, value: T) // note: not `&mut self`
                        this one unusual detail is the whole point of Cells.
                      * Unlike Cell, RefCell supports borrowing refer‐
                        ences to its T value:
                        ref_cell.borrow()
                        Returns a Ref<T>, which is essentially just a shared reference to the value stored
                        in ref_cell.
                        This method panics if the value is already mutably borrowed; see details to fol‐
                        low.
                        ref_cell.borrow_mut()
                        Returns a RefMut<T>, essentially a mutable reference to the value in ref_cell.
                        This method panics if the value is already borrowed; see details to follow.
                        ref_cell.try_borrow(), ref_cell.try_borrow_mut()
                        Work just like borrow() and borrow_mut(), but return a Result. Instead of pan‐
                        icking if the value is already mutably borrowed, they return an Err value.
                      * The only difference is that normally,
                        when you borrow a reference to a variable, Rust checks at compile time to ensure that
                        you’re using the reference safely. If the checks fail, you get a compiler error. RefCell
                        enforces the same rule using run-time checks.
                      * The other drawback is less obvious and more serious: cells—and any types
                        that contain them—are not thread-safe.
                        * Rust therefore will not allow multiple threads
                          to access them at once.
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
1. cargo
    * Cargo.lock
        * The first time you build a
          project, Cargo outputs a Cargo.lock file that records the exact version of every crate it
          used.
        * Later builds will consult this file and continue to use the same versions.
        * if your project is an executable, you should commit Cargo.lock to ver‐
          sion control.
          * That way, everyone who builds your project will consistently get the
            same versions.
        * If your project is an ordinary Rust library, don’t bother committing Cargo.lock. Your
          library’s downstream users will have Cargo.lock files that contain version information
          for their entire dependency graph; they will ignore your library’s Cargo.lock file.
        * reason
            * The version numbers in Cargo.toml are deliberately flexible, yet we don’t want Cargo
              to upgrade us to the latest library versions every time we build. Imagine being in the
              middle of an intense debugging session when suddenly cargo build upgrades you to
              a new version of a library.
    * When you write something like image = "0.13.0" in your Cargo.toml file, Cargo
      interprets this rather loosely.
      * It uses the most recent version of image that is consid‐
        ered compatible with version 0.13.0.
      * Suppose one library, libA, used
        num = "0.1.31" while another, libB, used num = "0.1.29". If version numbers
        required exact matches, no project would be able to use those two libraries together.
        Allowing Cargo to use any compatible version is a much more practical default.
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
        * These three options—modules in their own file, modules in their own directory with
          a mod.rs, and modules in their own file with a supplementary directory containing
          submodules
        * The :: operator is used to access features of a module.
            std::mem::swap(&mut s1, &mut s2);
            * std is the name of the standard library.
            * The alternative is to import features into
              the modules where they’re used:
              use std::mem;
              mem::swap(&mut s1, &mut s2);
        * use crate::proteins::AminoAcid; // explicitly import relative to crate root
        * Modules aren’t the same thing as files, but there is a natural analogy between modules
          and the files and directories of a Unix filesystem.
            * The use keyword creates aliases, just
              as the ln command creates links.
            * Paths, like filenames, come in absolute and relative
              forms. self and super are like the . and .. special directories.
        * For one thing, the standard library std is automatically linked with every project.
            * Furthermore, a few particularly
              handy names, like Vec and Result, are included in the standard prelude and automat‐
              ically imported.
            * Rust behaves as though every module, including the root module,
              started with the following import:
              use std::prelude::v1::*;
        * In addition to functions, types, and nested modules, modules can also define con‐
          stants and statics.
1. structs
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
    * Rust has three kinds of struct types, named-field, tuple-like, and unit-like,
        * named
            struct GrayscaleMap {
            pixels: Vec<u8>,
            size: (usize, usize)
            }
        * Tuple-Like
            struct Bounds(usize, usize);
            assert_eq!(image_bounds.0 * image_bounds.1, 786432);
            * Tuple-like structs are good for newtypes, structs with a single component that you
              define to get stricter type checking.
        * Unit-Like Structs
            * struct Onesuch;
            * A value of such a type occupies no memory, much like the unit type ()
    * let mut broom1 = Broom { height: b.height / 2, .. b };
            ..b destructurization
    * In memory, both named-field and tuple-like structs are the same thing: a collection of
      values, of possibly mixed types, laid out in a particular way in memory.
      * Rust doesn’t make specific promises about how it will order a
        struct’s fields or elements in memory;
      * Rust does promise to store fields’ values directly in the struct’s block
        of memory.
      * Whereas JavaScript, Python, and Java would put the pixels and size
        values each in their own heap-allocated blocks and have GrayscaleMap’s fields point
        at them, Rust embeds pixels and size directly in the GrayscaleMap value.
    * An impl block is simply a collection of fn definitions, each of which becomes a
      method on the struct type named at the top of the block.
      pub fn push(&mut self, c: char) {
      * Unlike C++ and Java, where the members of the “this” object are
        directly visible in method bodies as unqualified identifiers, a Rust method must
        explicitly use self to refer to the value it was called on, similar to the way Python
        methods use self, and the way JavaScript methods use this.
      pub fn split(self) -> (Vec<char>, Vec<char>) {
        * if a method wants to take ownership of self, it can take self by value
      pub fn new() -> Queue {
        * Type-Associated Functions
    * Although you can have many separate impl blocks for a single type, they must all be
      in the same crate that defines that type.
    impl Vector2 {
    const ZERO: Vector2 = Vector2 { x: 0.0, y: 0.0 };
    const UNIT: Vector2 = Vector2 { x: 1.0, y: 0.0 };
    }
    * As another shorthand,
      every impl block, generic or not, defines the special type parameter Self (note the
      CamelCase name) to be whatever type we’re adding methods to.
    * if a struct type con‐
      tains references, you must name those references’ lifetimes.
    * A struct’s fields, even private fields, are accessible throughout the module where the
      struct is declared, and its submodules. Outside the module, only public fields are
      accessible.
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
    * In Rust, panic is safe and per
      thread. The boundaries between threads serve as a firewall for panic; panic doesn’t
      automatically spread from one thread to the threads that depend on it. Instead, a
      panic in one thread is reported as an error Result in other threads. The program as a
      whole can easily recover.
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
1. cargo
1. attributes
    * You
      can disable the warning by adding an #[allow] attribute on the type:
      #[allow(non_camel_case_types)]
    * Conditional compilation is another feature that’s written using an attribute, namely,
      #[cfg]:
      #[cfg(target_os = "android")]
    * #[inline]

## traits
* Rust’s take on interfaces
    * approach inspired by Haskell’s typeclasses
    * dispatch
        * mechanism to determine which specific version is actually run
        * static
            * not something Go or Java have
            * perform using monomorphization
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
            * provided through a feature called trait objects
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
    * example: can't `impl Write for u8`
        * both are defined in the standard library.
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
* `impl Trait` can be used in two locations
    * as an argument type

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
    * useful traits
        * Any type that implements the
            FromStr trait has a from_str method that tries to parse a value of that type from a
            string
        * std::str::FromStr
            * Rust provides standard traits for both parsing values from strings and producing tex‐
              tual representations of values.
        * Clone
            * The clone method should construct an independent copy of self and return it.
            * Cloning a value usually entails allocating copies of anything it owns, as well, so a
              clone can be expensive, in both time and memory.
            * #[derive(Clone)]
            * Some types don’t make sense to copy, like std::sync::Mutex; those
              don’t implement Clone.
            * ToOwned
                * But what if you want to
                  clone a &str or a &[i32]? What you probably want is a String or a Vec<i32>, but
                  Clone’s definition doesn’t permit that: by definition, cloning a &T must always return a
                  value of type T, and str and [u8] are unsized; they aren’t even types that a function
                  could return.
                * Unlike clone, which must return exactly Self, to_owned can return anything you
                  could borrow a &Self from: the Owned type must implement Borrow<Self>.
        * Cow
            * The nub of the problem is that sometimes the return value of get_name should be an
              owned String, sometimes it should be a &'static str, and we can’t know which one
              it will be until we run the program. This dynamic character is the hint to consider
              using std::borrow::Cow, the clone-on-write type that can hold either owned or bor‐
              rowed data.
                fn get_name() -> String {
                std::env::var("USER") // Windows uses "USERNAME"
                .unwrap_or("whoever you are".to_string())
                }
                println!("Greetings, {}!", get_name());
                Since Cow is frequently used for strings, the standard library has some special support
                for Cow<'a, str>.
                fn get_name() -> Cow<'static, str> {
                std::env::var("USER")
                .map(|v| v.into())
                .unwrap_or("whoever you are".into())
                }
            * Borrowed holds a ref‐
              erence &'a T, and Owned holds the owning version of &T: String for &str, Vec<i32>
              for &[i32], and so on.
            * clone on write
            * A Cow<B> either borrows a shared reference to a B or owns a value from which we
              could borrow such a reference.
            * Since Cow implements Deref, you can call methods on
              it as if it were a shared reference to a B: if it’s Owned, it borrows a shared reference to
              the owned value; and if it’s Borrowed, it just hands out the reference it’s holding.
            * You can also get a mutable reference to a Cow’s value by calling its to_mut method,
              which returns a &mut B. If the Cow happens to be Cow::Borrowed, to_mut simply calls
              the reference’s to_owned method to get its own copy of the referent, changes the Cow
              into a Cow::Owned, and borrows a mutable reference to the newly owned value. This
              is the “clone on write” behavior the type’s name refers to.
            * Similarly, Cow has an into_owned method that promotes the reference to an owned
              value, if necessary, and then returns it, moving ownership to the caller and consum‐
              ing the Cow in the process.
            * One common use for Cow is to return either a statically allocated string constant or a
              computed string.
                fn describe(error: &Error) -> Cow<'static, str> {
                match *error {
                Error::OutOfMemory => "out of memory".into(),
                Error::StackOverflow => "stack overflow".into(),
                Error::MachineOnFire => "machine on fire".into(),
                Error::Unfathomable => "machine bewildered".into(),
                Error::FileNotFound(ref path) => {
                format!("file not found: {}", path.display()).into()
                }
                }
                }
        * Copy
            * Any type that implements the Drop trait cannot be Copy.
            * Rust presumes that if a type
              needs special cleanup code, it must also require special copying code and thus can’t be
              Copy.
            * #[derive(Copy)]
            * Think carefully before making a type Copy. Although doing so makes the type easier
              to use, it places heavy restrictions on its implementation.
        * Send, Sync
            * This is mostly true, but Rust’s full thread safety story hinges on two built-in
              traits, std::marker::Send and std::marker::Sync.
            * Types that implement Send are safe to pass by value to another thread. They can
              be moved across threads.
            * Types that implement Sync are safe to pass by non-mut reference to another
              thread. They can be shared across threads.
            * A struct or enum is Send if its fields are Send, and Sync if its fields are Sync.
            * Some types are Send, but not Sync. This is generally on purpose, as in the case of
              mpsc::Receiver, where it guarantees that the receiving end of an mpsc channel is
              used by only one thread at a time.
            * The few types that are neither Send nor Sync are mostly those that use mutability in a
              way that isn’t thread-safe. For example, consider std::rc::Rc<T>, the type of
              reference-counting smart pointers.
              * What would happen if Rc<String> were Sync, allowing threads to share a single Rc
                via shared references? If both threads happen to try to clone the Rc at the same time,
                as shown in Figure 19-10, we have a data race as both threads increment the shared

                reference count.
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
        * Deref, DerefMut
            * You can specify how dereferencing operators like * and . behave on your types by
              implementing the std::ops::Deref and std::ops::DerefMut traits.
            * For example, if you have a Box<Complex> value b, then *b refers to
              the Complex value that b points to, and b.re refers to its real component.
            * If the con‐
              text assigns or borrows a mutable reference to the referent, Rust uses the DerefMut
              (“dereference mutably”) trait; otherwise, read-only access is enough, and it uses
              Deref.
        * Default
            * The default method simply returns a fresh value of type Self.
        * AsRef and AsMut
            * When a type implements AsRef<T>, that means you can borrow a &T from it effi‐
              ciently.
            * AsMut is the analogue for mutable references.
            * for example, Vec<T> implements AsRef<[T]>, and String implements
              AsRef<str>.
            * You might assume that if a type implements AsRef<T>, it should also implement
              AsMut<T>.
              * For example, we’ve
                mentioned that String implements AsRef<[u8]>; this makes sense, as each String
                certainly has a buffer of bytes that can be useful to access as binary data. However,
                String further guarantees that those bytes are a well-formed UTF-8 encoding of Uni‐
                code text; if String implemented AsMut<[u8]>, that would let callers change the
                String’s bytes to anything they wanted, and you could no longer trust a String to be
                well-formed UTF-8. It only makes sense for a type to implement AsMut<T> if modify‐
                ing the given T cannot violate the type’s invariants.
        * From and Into
            * the effect is much like that of overloading a
              function in C++.
            * fn ping<A>(address: A) -> std::io::Result<bool>
              where A: Into<Ipv4Addr>
              println!("{:?}", ping(Ipv4Addr::new(23, 21, 68, 141))); // pass an Ipv4Addr
              println!("{:?}", ping([66, 146, 219, 98])); // pass a [u8; 4]
            * The ? operator uses From and Into to help clean up code in functions that could fail
              in multiple ways by automatically converting from specific error types to general ones
              when needed.
              * https://www.reddit.com/r/rust/comments/iuespp/question_mark_operator_implicit_conversion_why/
            * From and Into are infallible traits—their API requires that conversions will not fail.
        * TryFrom and TryInto
            * Where From and Into relate types with simple conversions, TryFrom and TryInto
              extend the simplicity of From and Into conversions with the expressive error han‐
              dling afforded by Result.
        * PartialEq
        * Rust can automatically implement them for you, with mechanical accuracy. Just add a
          #[derive] attribute to the struct:
          #[derive(Copy, Clone, Debug, PartialEq)]
        * Sized
            * A sized type is one whose values all have the same size in memory.
                * every u64 takes eight bytes, every (f32, f32, f32) tuple twelve
            * All sized types implement the std::marker::Sized trait, which has no methods or
              associated types.
            * Rust can’t store unsized values in variables or pass them as arguments. You can only
              deal with them through pointers like &str or Box<dyn Write>, which themselves are
              sized.
            * 13-1, a pointer to an unsized value is always a fat pointer,
              two words wide: a pointer to a slice also carries the slice’s length, and a trait object
              also carries a pointer to a vtable of method implementations.
            * A struct
              type’s last field (but only its last) may be unsized, and such a struct is itself unsized.
        * debug, display
            1. debug vs Display
                * The #[derive(Debug)] attribute tells the compiler to generate some extra code that
                  allows us to format the Arguments struct with {:?} in println!.
    * macros (derev)

## closures
* closure may contain data
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
    * example
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
* implements an `Fn` trait
    * note that `fn` is not a trait
        * cannot be used in `where` clause
    *  is a trait that represents types that can be called as if they were functions
        ```
        pub trait Fn<Args> { // simplified
            type Output;
            fn call(&self, args: Args) -> Self::Output; // self passed by reference
        }
        ```
    * automatically implemented by all functions
    * used primarily for working with closures
        * for example: `HashMap` not implements `Fn`
    * `FnOnce`
        * represents closures that can be invoked only once
            ```
            pub trait Fn<Args> { // simplified
                type Output;
                fn call(self, args: Args) -> Self::Output; // self passed by value
            }
            ```
        * example
            ```
            let map = ...
            let f = || {
                for (key, value) in map { // consumes map
                    ...
                }
            }
            f(); // ok
            f(); // error: use of moved value
* usually do not have the same type as functions
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
            ```
    * every closure has its own type
        * no two closures have exactly the same type
        * ad hoc type created by the compiler, large enough to hold that data
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
        * any reference must have an annotated lifetime.
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
* fat pointers
    * structure which contains
        * the actual pointer to the piece of data
        * and some additional information (length for slices, pointer to vtable for trait objects)
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
*` Box<T>`
    * simplest way to allocate a value in the heap
    * `Box::new(v)` allocates some heap space, moves the value v into it, and returns a `Box` pointing to the heap space
    * if goes out of scope, the memory is freed immediately
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

## String vs !str vs str
* UTF-8
    * encodes a character as a sequence of one to four bytes
    * restrictions
        * only the shortest encoding for any given code point is considered well-formed
            * example: can’t spend four bytes encoding a code point that would fit in three
        * must not encode numbers from `0xd800` through `0xdfff` or beyond `0x10ffff`
            * either reserved for noncharacter purposes or outside Unicode’s range entirely
* sequences of Unicode characters stored as a well-formed UTF-8 encoding
    * simple char-by-char comparison does not always give the expected answers
        * example: `th\u{e9}` and `the\u{301}`` are both valid Unicode representations for thé
            * Rust treats them as two completely distinct strings
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
* for Java people:
    * Rust' String === StringBuilder
    * Rust's &str === (immutable) string
* `String`
    * implemented as a wrapper around a Vec<u8>
        * ensures the vector’s contents are always well-formed UTF-8
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
    * cannot create a standalone value of type `str`
        * always in its borrowed form: `&str`
* rule of thumb
    * use String if you need owned string data (like passing strings to other threads, or building them at runtime)
    * use &str if you only need a view of a string
    * functions arguments
        * pass &str if function does something with a string without needing to stash it away somewhere
        * pass `String` if function modifies or needs to store it for later

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
    * example
        ```
        ```
    * shorthand for a match expression
        ```
        let output = match File::create(filename) {
            Ok(f) => f,
            Err(err) => return Err(err)
        };
        ```
* dot
    * The unary * operator is used to access the value pointed to by a reference. As we’ve
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
* assertions
    * `assert!`, `assert_eq!`, `assert_ne!`
    * macros from the Rust standard library
    * panics, which causes the test to fail
* integration tests
    * `.rs` files that live in a `tests` directory alongside `src`
    * `cargo test` compiles each integration test as a separate, standalone crate, linked with your library
    and the Rust test harness
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