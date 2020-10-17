# default-conversion-rs
Default conversion for structures with same fields of the same name

EDIT: Just discovered this way to do the same job of this crate: https://stackoverflow.com/questions/57477967/how-can-i-use-serde-to-serialize-a-struct-to-another-rust-data-structure

If someone find a advantage to continue something like mine I will continue develop it.

## Why I created this ?

Some projects which use derive proc macros need separates structures with similar "structures" to make different things. For example :

- An API with input structures that have some same fields with the model.

## Example :

### Developper code:

```rust
struct A {
    a: String,
    b: i32,
};

#[derive(IntoDefault)]
#[IntoStruct(A)]
struct InputA {
    a: String,
    b: i32,
};

let a = InputA {
    a: String::from("test"),
    b: 2,
};

let b = A::from(a);
```

### Expanded code :

```rust
struct A {
    a: String,
    b: i32,
};

#[derive(IntoDefault)]
#[IntoStruct(A)]
struct InputA {
    a: String,
    b: i32,
};

impl From<InputA> for A {
    fn from(item: InputA) -> Self {
        A {
            a: item.a,
            b: item.b,
        }
    }
}

let a = InputA {
    a: String::from("test"),
    b: 2,
};

let b = A::from(a);
```

## Explanations

Detailed explanations in docs (soon xd).

`IntoDefault` is the main derive macro to invoke the from implementation.
`IntoStruct` the attribute proc macro to define the type in which we want to implement the conversion. `#[IntoStruct(TYPE_OF_STRUCT)]`