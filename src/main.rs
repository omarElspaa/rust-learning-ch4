// Ownership is Rust's most unique feature and has deep implications for the rest of the language. It enables Rust to make memory safety guarantees without needing a garbage collector, so it's important to understand how ownership works.
// Implications: The action or state of being involved in something.
// Ownership is a set of rules that govern how a Rust program manages memory, if any of the rules are violated, the program won't compile.
// None of the features of ownership will slow down your program while it's running.



// Rust is a systems programming language.

// The stack and the heap are parts of memory available to your code to use at runtime, but they are structured in different ways.

// Whether a value is on the stack or the heap affects how the language behaves.

// The stack stores values in the order it gets them and removes the values in the opposite order. This is referred to as last in, first out. Think of a stack of plates: when you add more plates, you put them on top of the pile, and when you need a plate, you take one off the top.Adding or removing plates from the middle or bottom wouldn't work as well! Adding data is called pushing onto the stack, and removing data is called popping off the stack. All data stored on the stack must have a known, fixed size. Data with an unknown size at compile time or a size that might change must be stored on the heap instead.

// The heap is less organized: when you put data on the heap, you request a certain amount of space. The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is address of that location. This process is called allocating on the heap and is sometimes abbreviated as just allocating ( pushing values onto the stack is not considered allocating ). Because the pointer to the heap is a known, fixed size you can store the pointer on the stack, but when you want the actual data, you must follow the pointer. Think of being seated at a restaurant. When you enter, you state the number of people in your group, and the host finds an empty table that fits everyone and leads you there. If someone in your group comes late they can ask where you've been seated to find you.

// Pushing to the stack is faster than allocating on the heap because the allocator never has to search for a place to store new data; that location is always at the top of the stack. Comparatively, allocating space on the heap requires more work because the allocator must first find a big enough space to hold the data and then perform bookkeeping to prepare for the next allocation.

// Accessing data in the heap is slower than accessing data on the stack because you have to follow a pointer to get there. Contemporary processors are faster if they jump around less in memory. Continuing the analogy, consider a server at a restaurant taking orders from many tables. It's most efficient to get all the orders at one table before moving on to the next table. Taking an order from table A, then an order from table B, then one from A again, and then one from B again would be a much slower process. By the same token, a processor can do its job better if it works on data that's close to other data ( as it is on the stack ) rather than farther away ( as it can be on the heap ).



// When your code calls a function, the values passed into the function ( including, potentially, pointers to data on the heap ) and the function's local variables get pushed onto the stack. When the function is over, those values get popped off the stack.



// Ownership Rules:
//  1. Each value in Rust has an owner.
//  2. There can only be one owner at a time.
//  3. When the owner goes out of scope, the value will be dropped.



// A scope is the range within a program for which an item is valid.
// When a variable comes into scope, it is valid.
// It remains valid until it goes out of scope.
// At this point, the relationship between scopes and when variables are valid is similar to that in other programming languages.

fn main() {
    let mut s: &str = "wow"; // This variable refers to a string literal, where the value of the string is hardcoded into the text of our final executable. This is why string literals are fast and efficient.
    // If you don't mutate the variable while using mut it will result a compile-time warning because `#[warn(unused_mut)]` is on by default
    // If you assigned a value to the variable and didn't use it before mutating it there will be a compile-time warning because `#[warn(unused_assignments)]` is on by default
    println!("{s}");
    s = "howw"; // This works, because s is a reference.
    {
        println!("{s}");
    }
    let x = s;
    println!("{x}");
    strings();
}

fn strings() {
    let mut str: String = String::from("hello"); // The syntax will be discussed more in "Method Syntax" on page 97 chapter 5, and when talking about namespacing with modules in "Paths for Referring to an Item in a Module Tree" on page 125 chapter 7.
    // This type manages data allocated on the heap and as such is able to store an amount of text that is unknown to us at compile time.

    str.push_str(", world"); // Appends a literal to a String.
    println!("{str}");



    // In Rust: the memory is automatically returned once the variable that owns it goes out of scope.
    // When a variable goes out of scope, Rust calls a special function for us. This function is called drop, and it's where the author of String can put the code to return the memory. Rust calls drop automatically at the closing curly bracket.
    // In C++, this pattern of deallocating resources at the end of an item's lifetime is sometimes called Resource Acquisition Is Initialization (RAII). The drop function in Rust will be familiar to you if you\ve used RAII patterns.



    let s1 = String::from("hello");
    let s2 = s1;
    drop(s2);
    // A string is made up of three parts: a pointer to the memory that holds the contents of the string, a length, and a capacity. This group of data is stored on the stack.

    // The length is how much memory, in bytes, the contents of the String are currently using. The capacity is the total amount of memory, in bytes, that the String has received from the allocator. When we assign s1 to s2, the String data is copied, meaning we copy the pointer, the length, and the capacity that are on the stack. We do not copy the data on the heap that the pointer refers to.



    // Earlier, we said that when a variable goes out of scope, Rust automatically calls the drop function and cleans up the heap memory for that variable. But if the two data pointers are pointing to the same location. This will lead to a problem: when s2 and s1 go out of scope, they will both try to free the same memory. This is known as a double free error and is one of the memory safety bugs we mentioned previously. Freeing memory twice can lead to memory corruption, which can potentially lead to security vulnerabilities. To ensure memory safety, after the line let s2 = s1; Rust considers s1as no longer valid. Therefore, Rust doesn't need to free anything when s1 goes out of scope.

    // let s1 = String::from("hello");
    // let s2 = s1;
    // println!("{s1}, world!");

    // This will result a compile-time error, because s1 is invalidated.

    // Shallow copy: The concept of copying the pointer, length, and capacity.
    // Deep copy: The concept of copying the actual data and the data on the stack.
    // Move: A shallow copy, but rust invalidates the first variable.

    // If we DO want to deeply copy the heap data of the String, not just the stack data, we can use a common method called clone. We'll discuss method syntax in Chapter 5.

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {s1}, s2 = {s2}");
    // This works


    // Rust has a special annotation called the Copy trait that we can place on types that are stored on the stack, as integers are ( we'll talk more about traits in Chapter 10 ). If a type implements the Copy trait, variables that use it do not move, but rather are trivially copied, making them still valid after assignment to another variable. Rust won't let us annotate a type with Copy if the type, or any of its parts, has implemented the Drop trait.



    // A reference is like a pointer in that it's an address we can follow to access the data stored at that address; that data is owned by some other variable. Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference. This is enforced by Rust’s borrow checker, which ensures that references do not outlive the data they point to.
    // &var is a reference of that variable.
    // We call the action of creating a reference borrowing.



    // &mut var is a mutable reference of that variable.

    // Mutable references have one big restriction: if you have a mutable reference to a value, you can have no other references to that value.
    // If you have a reference to a variable, you cannot change the value of the owner directly while that mutable reference exists. This restriction is part of Rust's borrowing rules designed to ensure memory safety.

    // The benefit of having this restriction is that Rust can prevent data races at compile time. A data race is similar to a race condition and happens when these three behaviors occur:
    // 1. Two or more pointers access the same data at the same time.
    // 2. At least one of the pointers is being used to write to the data.
    // 3. There's no mechanism being used to synchronize access to the data.

    // You also cannot have a mutable reference while we have an immutable one to the same value



    // Note that a reference's scope starts from where it is introduced and continues through the last time that reference is used. For instance, this code will compile.

    // let mut s = String::from("hello");
    // let r1 = &s;
    // let r2 = &s;
    // println!("{r1} and {r2}");
    // let r3 = &mut s;
    // println!("{r3}");



    //  Dangling pointer: a pointer that references a location in memory that may have been given to someone else by freeing some memory while preserving a pointer to that memory.

    // In Rust, by contrast, the compiler guarantees that references will never be dangling references: if you have a reference to some data, the compiler will ensure that the data will not go out of scope before the reference to the data does.


    // Recap:
    // 1. At any given time, you can have either one mutable reference or any number of immutable references.
    // 2. References must always be valid.



    // This is the syntax of a string slice &s[0..5]
    // Starting_index is the first position in the slice and ending_index is one more than the last position in the slice.
    // The slice data structure stores the starting position ( a reference to the first element ) and the length of the slice.
    // &s[0..2] = &s[..2]
    // &s[3..len] = &s[3..]



    // String slice range indices must occur at valid UTF-8 character boundaries. If you attempt to create a string slice in the middle of a multibyte character, your program will exit with an error. Discussed more on page 147.

    // You can't borrow a value as a mutable if it is declared as immutable.



    // String Literals as Slices
    // let s = "Hello, world!";
    // The type of s here is &str: it's a slice pointing to that specific point of the binary. This is also why string literals are immutable; &str is an immutable reference.

    // When a function signature in Rust requires a string slice (&str) and you pass a reference to a String (i.e., &String), Rust will automatically dereference the &String to &str because String implements the Deref trait. This means that you can pass a &String to a function that expects a &str without any explicit conversion. This flexibility takes advantage of deref coercions, a feature we will cover in "implicit Deref Coercions with Functions and Methods" on page 325.

    // let x = String::from("Hello");
    // let y = &x[0..4];
    // let z = &y[0..2];

    // You can take a slice from a slice like the code above.
}