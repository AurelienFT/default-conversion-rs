use default_conversion::*;

#[test]
fn basic_struct() {
    #[derive(Debug, PartialEq, Eq, Default)]
    struct B {
        a: i32,
    };

    #[derive(Debug, PartialEq, Eq, Default, IntoDefault)]
    #[IntoStruct(B)]
    struct InputB {
        a: i32,
    };

    #[derive(Debug, PartialEq, Eq, Default)]
    struct A {
        a: String,
        b: i32,
        c: B,
        d: B,
        e: B,
        f: Option<B>,
        g: Option<B>,
        h: Option<B>,
        i: Vec<B>,
        j: Vec<B>,
    };

    #[derive(Debug, PartialEq, Eq, Default, IntoDefault)]
    #[IntoStruct(A)]
    struct InputA {
        a: String,
        b: i32,
        c: InputB,
        d: Option<InputB>,
        e: Option<InputB>,
        f: Option<InputB>,
        g: Option<InputB>,
        h: InputB,
        i: Vec<InputB>,
        j: InputB,
    };

    let a = InputA {
        a: String::from("test"),
        b: 2,
        c: InputB { a: 3 },
        d: Some(InputB { a: 4 }),
        e: None,
        f: Some(InputB { a: 5 }),
        g: None,
        h: InputB { a: 6 },
        i: vec!{InputB{a: 7}},
        j: InputB { a: 8},
    };

    assert_eq!(
        A {
            a: String::from("test"),
            b: 2,
            c: B { a: 3 },
            d: B { a: 4 },
            e: B { a: 0 },
            f: Some(B { a: 5 }),
            g: None,
            h: Some(B { a: 6 }),
            i: vec!{B{a: 7}},
            j: vec!{B{a: 8}},
        },
        A::from(a)
    );
}
