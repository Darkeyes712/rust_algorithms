mod algorithm;
use algorithm::KolzoLinkedList;

fn main() {
    let mut ll = KolzoLinkedList::new();
    ll.append(1);
    ll.append(2);
    ll.print();
}
