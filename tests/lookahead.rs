extern crate itertools;

use itertools::LookaheadIterator;

#[test]
fn lookahead_for_exact_size() {
    let mut it = vec![10, 20, 30].into_iter();
    assert!(it.has_next());
    assert_eq!(Some(10), it.next());
    assert_eq!(Some(20), it.next());
    assert!(it.has_next());
    assert_eq!(Some(30), it.next());
    assert!(!it.has_next());
}
