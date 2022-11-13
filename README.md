# Rust book notes 🦀

Since I have completed the official [Rust Book](https://doc.rust-lang.org/book/title-page.html), I decided that it would be a good idea to stack up all of the related code in one repo.
Here are all the code snippets I made while reading the book.
In addition below are some key takeaways, I try to keep them short and informative.

## Key takeaways

1. In Rust all variables are immutable by default.
2. *match* is a pretty useful exhaustive alternative to *switch*.
3. Memory allocation in Rust is neither automated by a garbage collector, neither controlled manually. Instead, Rust introduces concepts of *ownership* and *borrowing* to handle memory allocation.
4. *cargo* is a very cool package manager, which is present by design: it produces the skeleton of the project, allows easy testing, dependency management and separation of production and testing environments. Hell, it even creates a git repo for you.
5. Rust is very explicit when it comes to error handling, errors in Rust can be recoverable and unrecoverable.
6. To enhance the memory control in (3), Rust also sometimes requires one to explicitly specify the lifetimes of references passed to a function.
---

There are definitely many more things to mention, the above 6 are the ones which got ingrained in my head as peculiarities of the language.
Many other nuances I may only remember and better grasp through continuous practice, if I decide to proceed further with the language.
