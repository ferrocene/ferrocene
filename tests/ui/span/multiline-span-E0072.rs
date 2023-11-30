// It should just use the entire body instead of pointing at the next two lines
struct //~ ERROR has infinite size
ListNode
{
    head: u8,
    tail: Option<ListNode>,
}

fn main() {
}

// ferrocene-annotations: fls_3gapgqys3ceb
// Recursive Types
//
// ferrocene-annotations: fls_g1z6bpyjqxkz
// Type Layout
