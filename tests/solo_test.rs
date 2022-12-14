//use grapher::graph::{Graph};


#[test]
fn test1() {
    #[derive(Debug)]
    enum A {
        A(u8),
        B(f32)
    }

    let mut v: Vec<A> = Vec::new();
    v.push(A::A(5u8));
    v.push(A::B(3.14f32));

    for val in v.into_iter() {
        println!("{:?}", val);
    }
}

#[test]
fn test2() {
    trait IsDog {}
    trait IsBig {}

    trait Barker {
        fn bark(&self) {
            println!("Woof")
        }
    }

    impl<T: IsDog + IsBig> Barker for T {}

    struct Star {}
    impl IsDog for Star {}
    impl IsBig for Star {}

    let s: Star = Star{};
    s.bark();
}

#[test]
fn test3() {
    for i in 0..5 {
        println!("{:?}", i);
    }
}
