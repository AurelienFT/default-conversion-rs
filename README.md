# default-conversion-rs
Default conversion for structures with similar fields

I discovered this way to do the same job of this crate: https://stackoverflow.com/questions/57477967/how-can-i-use-serde-to-serialize-a-struct-to-another-rust-data-structure

But apparently my way to do it with From is faster : https://github.com/serde-rs/json/issues/724

## Why I created this ?

Some projects which use derive proc macros need separates structures with similar "structures" to make different things. For example :

- An API with input structures that have some same fields with the model.

## Usage :

```
default-conversion = "0.1.0"
```

## Example :

### Developper code:

```rust
struct B {
    a: i32,
};

#[derive(IntoDefault)]
#[IntoStruct(B)]
struct InputB {
    a: i32,
};

struct A {
    a: String,
    b: i32,
    c: B
};

#[derive(IntoDefault)]
#[IntoStruct(A)]
struct InputA {
    a: String,
    b: i32,
    c: InputB
};

let a = InputA {
    a: String::from("test"),
    b: 2,
    c: InputB {
        a: 3
    }
};

let b = A::from(a);
```

## Complete example in [tests/basic_struct.rs](https://github.com/AurelienFT/default-conversion-rs/blob/main/tests/basic_struct.rs)

## Explanations

Detailed explanations in docs (soon xd).

`IntoDefault` is the main derive macro to invoke the from implementation.

`IntoStruct` the attribute proc macro to define the type in which we want to implement the conversion. `#[IntoStruct(TYPE_OF_STRUCT)]`


## Supported features:

- Conversion with primitive types.
- Conversion with complex types.
- Auto convert to Option or Object as needed.
- Conversion with vectors of different types

## Next steps:

- Examples
- Better tests
- Docs
