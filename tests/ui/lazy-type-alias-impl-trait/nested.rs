//@ check-pass

fn main() {}

struct RawTableInner<A> {
    alloc: A,
}

impl<A> RawTableInner<A> {
    fn prepare_resize(
        self,
    ) -> ScopeGuard<Self, impl FnMut(&mut Self)> {
        ScopeGuard { dropfn: move |self_| {}, value: self,  }
    }
}

pub struct ScopeGuard<T, F>
where
    F: FnMut(&mut T),
{
    dropfn: F,
    value: T,
}

// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_7nv8ualeaqe3
// Where Clauses
