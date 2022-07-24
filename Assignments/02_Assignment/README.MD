### Methods & Associated functions

* Methods are similar to functions: Declare with fn, have parameters and a return value. Unlike functions, methods are defined within the context of a struct (or an enum or a trait object), and their first parameter is always self, which represents the instance of the struct the method is being called on.

* self will take the ownership of current struct instance, however, &self will only borrow a reference from the instance.

* The &self is actually short for self: &Self. Within an impl block, the type Self is an alias for the type that the impl block is for. Methods must have a parameter named self of type Self for their first parameter, so Rust lets you abbreviate this with only the name self in the first parameter spot.

### Constant Generics

Currently, const parameters may only be instantiated by const arguments of the following forms:
* A standalone const parameter.
* A literal (i.e. an integer, bool, or character).
* A concrete constant expression (enclosed by {}), involving no generic parameters.

