mod linked_list;

use linked_list::LinkedList;

fn main() {
    let mut list = LinkedList::<i32>::create_linked_list();
    list.add_data(32);
    list.add_data(16);
    list.add_data(08);
    list.add_data(04);
    list.add_data(02);
    println!("Hello, world!");
    list.print_list();
    list.print_list();
    let mut string_list = LinkedList::<String>::create_linked_list();
    string_list.add_data("hello!".to_string());
    string_list.add_data("hello!".to_string());
    string_list.add_data("hello!".to_string());
    string_list.add_data("hello!".to_string());
    string_list.add_data("hello!".to_string());
    string_list.add_data("hello!".to_string());
    string_list.print_list();
}
