use crate::option::Item;

pub fn test_item_exact_size_iterator_some() {
    let item = super::Item { opt: Some("foo") };
    assert_eq!(item.len(), 1);
}

pub fn test_item_exact_size_iterator_none() {
    let item: Item<&str> = super::Item { opt: None };
    assert_eq!(item.len(), 0);
}
