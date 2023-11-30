struct ListNode { //~ ERROR has infinite size
    head: u8,
    tail: Option<ListNode>,
}

fn main() {
}

// ferrocene-annotations: fls_3gapgqys3ceb
// Recursive Types
