// ex3.rs

#[derive(Debug)]
struct Foo {
    capacity: i32,
}

fn main() {
    println!("{:?}", Foo { capacity: 3 });
}