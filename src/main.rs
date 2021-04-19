use yaliir::datastructures::linkedlist::LinkedList;

fn main() {
    let mut a = LinkedList::new();
    println!("{:?}", a);
    a.push("hello");
    println!("{:?}", a);
    println!("{}", a.pop().unwrap());
    println!("{:?}", a);
}
