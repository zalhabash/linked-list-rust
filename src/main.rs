mod linked_list;

fn main() {
    let mut list = linked_list::LinkedList::new();
    list.push_front(5);
    list.push_front(4);

    list.push_back(6);
    list.pop_front();
    list.pop_back();
    list.push_back(10);
    println!("Is empty: {}", list.is_empty());
    println!("Length: {}", list.length());
    println!("{list:?}");
    println!("Front: {}", list.front().unwrap());
    println!("Back: {}", list.back().unwrap());
    println!("Items:");
    for item in list.iter() {
        println!("   {}", item);
    }
    println!("Items plus 3:");
    for item in list.iter_mut() {
        *item += 3;
        println!("   {}", item);
    }
}
