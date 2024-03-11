// Check that or-patterns with consistent bindings across arms are allowed.

//@ edition:2018

//@ check-pass

fn main() {
    // One level:
    let (Ok(a) | Err(a)) = Ok(0);
    let (Ok(ref a) | Err(ref a)) = Ok(0);
    let (Ok(ref mut a) | Err(ref mut a)) = Ok(0);

    // Two levels:
    enum Tri<S, T, U> {
        V1(S),
        V2(T),
        V3(U),
    }
    use Tri::*;

    let (Ok((V1(a) | V2(a) | V3(a), b)) | Err(Ok((a, b)) | Err((a, b)))): Result<_, Result<_, _>> =
        Ok((V1(1), 1));

    let (Ok((V1(a) | V2(a) | V3(a), ref b)) | Err(Ok((a, ref b)) | Err((a, ref b)))): Result<
        _,
        Result<_, _>,
    > = Ok((V1(1), 1));

    // Three levels:
    let (
        a,
        Err((ref mut b, ref c, d))
        | Ok((
            Ok(V1((ref c, d)) | V2((d, ref c)) | V3((ref c, Ok((_, d)) | Err((d, _)))))
            | Err((ref c, d)),
            ref mut b,
        )),
    ): (_, Result<_, _>) = (1, Ok((Ok(V3((1, Ok::<_, (i32, i32)>((1, 1))))), 1)));
}

// ferrocene-annotations: fls_d2sc9hl3v0mk
// Reference Patterns
//
// ferrocene-annotations: fls_org6hqv397fp
// Reference Pattern Matching
//
// ferrocene-annotations: fls_rce8bb7nz2jy
// Tuple Pattern Matching
//
// ferrocene-annotations: fls_urbr5rg9206v
// Tuple Patterns
