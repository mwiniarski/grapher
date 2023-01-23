#![allow(dead_code, unused)]
use grapher::graph::Node;

use std::{time::Instant, cmp::Ordering, marker::PhantomData};

trait Functions {
    fn func1(&self);
}

struct B {}

impl B {
    fn func2(&self) {}
}

impl Functions for B {
    fn func1(&self) {}
}

fn b_func<T: Functions>(_t: T) {
    //t.func2();
}

#[test]
fn test_trait() 
{
    let b = B{};
    b.func1();
    b.func2();
    b_func(b);
}

#[test]
fn test_node() 
{
    let _n = Node::new();
}

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

fn empty_f() {}
fn empty_with_empty(_: EmptyType) {}

#[test]
fn empty_func()
{
    let mut time = Instant::now();
    for _i in 0..1000000 {
        empty_f();
    }
    println!("Time passed: {:.2?}", time.elapsed());

    time = Instant::now();
    for _i in 0..1000000 {
        ()//empty_f();
    }
    println!("Time passed: {:.2?}", time.elapsed());

    time = Instant::now();
    for _i in 0..1000000 {
        empty_with_empty(EmptyType{});
    }
    println!("Time passed: {:.2?}", time.elapsed());
}

#[test]
fn option_size() 
{
    println!("{:?}", core::mem::size_of::<Option<i32>>());
    println!("{:?}", core::mem::size_of::<Option<&i32>>());
    println!("{:?}", core::mem::size_of::<i32>());
    println!("{:?}", core::mem::size_of::<&i32>());

    println!("{:?}", core::mem::size_of::<Option<String>>());
    println!("{:?}", core::mem::size_of::<Option<&String>>());
    println!("{:?}", core::mem::size_of::<String>());
    println!("{:?}", core::mem::size_of::<&String>());

    println!("{:?}", core::mem::size_of::<Option<EmptyType>>());
    println!("{:?}", core::mem::size_of::<Option<&EmptyType>>());
    println!("{:?}", core::mem::size_of::<EmptyType>());
    println!("{:?}", core::mem::size_of::<&EmptyType>());
    println!("{:?}", core::mem::size_of::<[();100]>());
}

struct EmptyType {}

struct Weighted<W> {
    w: Option<W>
}
impl<W> Weight<W> for Weighted<W> {
    fn get(&self) -> &W {
        self.w.as_ref().unwrap()
    } 
}
impl<W> Weighted<W> {
    fn new() -> Self {
        Weighted { w: None }
    }
}

struct Unweighted {}
impl<W> Weight<W> for Unweighted {
    fn get(&self) -> &W {
        panic!("Bad get")
    } 
}

trait Weight<W> {
    fn get(&self) -> &W;
}

struct SomeStruct<T, W> {
    _b: Vec<T>,
    _a: Box<dyn Weight<W>>
}

impl<T, W: 'static> SomeStruct<T, W> {
    fn f() { println!("A"); }

    fn new_unweighted() -> Self {
        SomeStruct { _b: Vec::new(), _a: Box::new(Unweighted{}) }
    }

    fn new_weighted() -> Self {
        SomeStruct { _b: Vec::new(), _a: Box::new(Weighted::new()) }
    }

    fn get_weight(&self) -> &W {
        self._a.get()
    }
}


#[test]
fn traits()
{
    let s: SomeStruct<usize, usize> = SomeStruct::new_unweighted();
}