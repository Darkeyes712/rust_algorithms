mod algorithm;
use algorithm::KolzoLinkedList;

fn main() {
    let mut ll = KolzoLinkedList::new();
    ll.append(1);
    ll.append(2);
    ll.print();
    ll.pop();
    ll.print();
    ll.prepend(4);
    ll.print();
    ll.get(1);
    ll.pop_first();
    ll.print();
    ll.set(1, 49);
    ll.print();

    ll.playground();
}
