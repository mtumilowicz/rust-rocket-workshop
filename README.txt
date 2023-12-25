
* references
    * https://doc.rust-lang.org/book/
    * https://doc.rust-lang.org/stable/rust-by-example/
    * https://rust-unofficial.github.io/too-many-lists/
    * https://www.reddit.com/r/rust/comments/wo46dz/does_using_string_instead_of_str_a_lot_results_in/
    * https://www.reddit.com/r/rust/comments/cyymw2/rule_of_thumb_for_struct_data_types/
    * https://www.reddit.com/r/rust/comments/4ltwov/shortlived_struct_string_or_a_str/
    * https://stackoverflow.com/questions/57562632/why-is-impl-needed-when-passing-traits-as-function-parameters

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
    *
5. closures
6. cargo
7. smart pointers: box
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