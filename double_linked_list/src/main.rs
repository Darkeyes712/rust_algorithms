mod algorithm;
use algorithm::KolzoDoublyLinkedList;

fn main() {
    let mut ll = KolzoDoublyLinkedList::new();
    ll.print();
    ll.append(2);
    ll.append(3);
    ll.print();
}
