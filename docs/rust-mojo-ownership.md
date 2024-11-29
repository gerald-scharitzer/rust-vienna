# Rust and Mojo Ownership (DRAFT)

[Rust's ownership system](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html) enables the compiler to ensure memory- and thread-safe code.

[Mojo's ownership system](https://docs.modular.com/mojo/manual/values/) is inspired by Rust.

# Mojo

Fixed-size local variables are on the stack.

For each value there is exactly one owner at all times.

When the lifetime of a value ends, then its destructor is called,
which deallocates the heap memory that is not needed anymore.

By [default](https://docs.modular.com/mojo/manual/values/value-semantics), arguments are passed by value.

If you pass arguements by reference, then the ownership system is enforced.

Assignments create deep copies.

`def` functions receive arguments as mutable copies.

`fn` functions receive arguments as immutable references.

_Argument exclusivity_ (at most one mutable reference to one value in function arguments) is enforced.

_Reference exclusivity_ (at most one mutable reference) will be enforced in the future.

# Summary

| Concept                       | Rust     | Mojo           |
| ----------------------------- | -------- | -------------- |
| immutable reference           | `&v`     | `borrowed v`   |
| mutable reference             | `&mut v` | `inout v`      |
| owned reference               | `v`      | `owned v`      |
| lifetime reference            | `&'a v`  | `ref v`        |
| at most one mutable reference | enforced | to be enforced |

For Mojo `fn` functions the default is `borrowed`.

# Conclusion

Rust enforces ownership by default and you can disable it explicitly with `unsafe`.

Mojo enforces ownership in `fn` functions.

# References

Rust and Mojo borrowed arguments
https://docs.modular.com/mojo/manual/values/ownership#compared-to-c-and-rust
