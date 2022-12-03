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

