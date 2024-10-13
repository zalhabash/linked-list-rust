mod linked_list;

fn main() {
    let mut list = linked_list::LinkedList::new();
    list.push_front(5);
    list.push_front(4);
    list.pop_front();
    println!("Is empty: {}", list.is_empty());
    println!("Length: {}", list.length());
    println!("{list:?}");
}
