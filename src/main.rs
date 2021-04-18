#[derive(Debug)]
struct List {
    head: Link,
}

#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(Debug)]
struct Node {
    data: Box<str>,
    next: Link,
}

impl List {
}

fn main() {
    let mut a = Box::from(Node{data : Box::from("hi"), prev: None, next: None});
    let mut b = Node{data: Box::from("aa"), prev:None, next: Some(a)};
    a.next = Some(Box::from(b));
    println!("{:?}", a);
}
