mod linked_list;

fn main() {
    let mut list = linked_list::LinkedList::new();
    list.push_front(5);
    list.push_front(4);

    list.push_back(6);
    list.pop_front();
    list.pop_back();
    println!("Is empty: {}", list.is_empty());
    println!("Length: {}", list.length());
    println!("{list:?}");
    println!("Front: {}", list.front().unwrap());
    println!("Back: {}", list.back().unwrap());
}
