//use grapher::graph::{Graph};

use std::{time::Instant, cmp::Ordering};


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

pub struct MyIterator {
    index: usize,
    max: usize
}

impl Iterator for MyIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        match self.index.cmp(&self.max) {
            Ordering::Less => Some(self.index),
            _ => None
        }
    }
}

impl MyIterator {
    fn new(max: usize) -> Self {
        MyIterator {index: 0, max: max }
    }
}

#[test]
fn test3() {

    const N: usize = 1000000;
    let v = vec![1; N];

    let mut time = Instant::now();
    for i in v { let _ = i + 1; }
    println!("Time passed: {:.2?}", time.elapsed());

    time = Instant::now();
    for i in MyIterator::new(N) { let _ = i + 1; }
    println!("Time passed: {:.2?}", time.elapsed());
}

struct MyClass {
    v: Vec<usize>
}

pub struct Wrapper {
    item: usize
}

pub struct MyIterator2<'a> {
    class: std::slice::Iter<'a, usize>
}

impl<'a> Iterator for MyIterator2<'a> {
    type Item = Wrapper;

    fn next(&mut self) -> Option<Self::Item> {
        match self.class.next() {
            Some(item) => Some(Wrapper{item:item.clone()}),
            None => None
        }
    }
}

impl<'a> MyIterator2<'a> {
    fn new(r: &'a MyClass) -> Self {
        MyIterator2 { class: r.v.iter() }
    }
}

#[test]
fn test4() {
    let m = MyClass{v:vec![1; 1000000]};
    
    let mut i = 0;
    let mut time = Instant::now();
    for v in MyIterator2::new(&m) {
        i += v.item;
    }
    println!("Time passed: {:.2?}", time.elapsed());

    time = Instant::now();
    for v in m.v.iter() {
        i += v;
    }
    println!("Time passed: {:.2?}", time.elapsed());
}

trait HasNew {
    fn new() -> Self;
}

struct A;

impl HasNew for A {
    fn new() -> Self {
        A{}
    }
}