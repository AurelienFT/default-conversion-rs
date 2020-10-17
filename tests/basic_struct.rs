use default_conversion::*;

#[test]
fn basic_struct() {
    struct A {
        a: String,
        b: i32
    };

    #[derive(IntoDefault)]
    struct InputA {
        a: String,
        b: i32
    };

    let a = InputA {
        a: String::from("test"),
        b: 2
    };

    assert_eq!(2, 2);
}