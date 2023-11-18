use std::{
    any::Any,
    ops::{Add, Deref},
};

struct Str(String);

impl Deref for Str {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

trait Foo {
    fn to_str(&self) -> &str;
}

#[derive(Debug)]
struct Impl;

impl Foo for Impl {
    fn to_str(&self) -> &str {
        "Impl"
    }
}

trait Animal {
    fn speak(&self) {}
}

struct Dog;
impl Animal for Dog {
    fn speak(&self) {
        println!("wuff")
    }
}

struct Cat;
impl Animal for Cat {
    fn speak(&self) {
        println!("miao")
    }
}

fn main() {
    let x = Str("hello".to_string());
    println!("len: {}", (&x as &String).clone().add(" foo"));

    let x: &dyn Foo = &Impl;
    println!("impl: {}", x.to_str());

    let y: &dyn Any = &x;
    println!("type id: {:#?}", y.type_id());

    let z: Box<dyn Any> = Box::new("foo".to_string());
    let foo = *z.downcast::<String>().unwrap();
    println!("foo: {}", foo);

    let cat = Cat;
    let dog = Dog;

    let animal: &dyn Animal = &dog;
    animal.speak();
}
