#![crate_type = "lib"]

fn lbv_macro_test_hygiene_respected() {
    macro_rules! mac2 {
        ($val:expr) => {
            break 'a $val; //~ ERROR undeclared label `'a` [E0426]
        };
    }
    let x: u8 = 'a: {
        'b: {
            if true {
                mac2!(2);
            }
        };
        0
    };
    assert_eq!(x, 2);

    macro_rules! mac3 {
        ($val:expr) => {
            'a: {
                $val
            }
        };
    }
    let x: u8 = mac3!('b: {
        if true {
            break 'a 3; //~ ERROR undeclared label `'a` [E0426]
        }
        0
    });
    assert_eq!(x, 3);
    let x: u8 = mac3!(break 'a 4); //~ ERROR undeclared label `'a` [E0426]
    assert_eq!(x, 4);
}

// ferrocene-annotations: fls_xa7lp0zg1ol2
// Declarative Macros
//
// ferrocene-annotations: fls_uusi0zej55is
// Loop Labels
//
// ferrocene-annotations: fls_wjldgtio5o75
// Macro Expansion
//
// ferrocene-annotations: fls_vnvt40pa48n8
// Macro Invocation
//
// ferrocene-annotations: fls_4apk1exafxii
// Macro Matching
//
// ferrocene-annotations: fls_ym00b6ewf4n3
// Macro Transcription
//
// ferrocene-annotations: fls_8nzypdu9j3ge
// Metavariables
//
// ferrocene-annotations: fls_n3ktmjqf87qb
// Rule Matching
//
// ferrocene-annotations: fls_qpx6lgapce57
// Token Matching
