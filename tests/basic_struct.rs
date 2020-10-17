use default_conversion::*;

#[test]
fn basic_struct() {
    #[derive(Debug, PartialEq, Eq)]
    struct A {
        a: String,
        b: i32,
    };

    #[derive(Debug, PartialEq, Eq, IntoDefault)]
    #[IntoStruct(A)]
    struct InputA {
        a: String,
        b: i32,
    };

    let a = InputA {
        a: String::from("test"),
        b: 2,
    };

    assert_eq!(
        A {
            a: String::from("test"),
            b: 2
        },
        A::from(a)
    );
}
