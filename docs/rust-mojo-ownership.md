# Rust and Mojo Ownership (DRAFT)

[Rust's ownership system](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html) enables the compiler to ensure memory- and thread-safe code.

[Mojo's ownership system](https://docs.modular.com/mojo/manual/values/) is inspired by Rust.

# Mojo

Fixed-size local variables are on the stack.

There is only one "owner" for a given value at a time (cited).

When the lifetime of a value ends, then its desctructor is called,
which deallocates the heap memory that is not needed anymore.
