// Test that various non-const items do not syntactically permit `_` as a name.

static _: () = (); //~ ERROR expected identifier, found reserved identifier `_`
struct _(); //~ ERROR expected identifier, found reserved identifier `_`
enum _ {} //~ ERROR expected identifier, found reserved identifier `_`
fn _() {} //~ ERROR expected identifier, found reserved identifier `_`
mod _ {} //~ ERROR expected identifier, found reserved identifier `_`
type _ = (); //~ ERROR expected identifier, found reserved identifier `_`
use _; //~ ERROR expected identifier, found reserved identifier `_`
use _ as g; //~ ERROR expected identifier, found reserved identifier `_`
trait _ {} //~ ERROR expected identifier, found reserved identifier `_`
trait _ = Copy; //~ ERROR expected identifier, found reserved identifier `_`
macro_rules! _ { () => {} } //~ ERROR expected identifier, found reserved identifier `_`
union _ { f: u8 } //~ ERROR expected one of `!` or `::`, found reserved identifier `_`

fn main() {}

// ferrocene-annotations: fls_xa7lp0zg1ol2
// Declarative Macros
//
// ferrocene-annotations: fls_szibmtfv117b
// Enum Types
//
// ferrocene-annotations: fls_qcb1n9c0e5hz
// Functions
//
// ferrocene-annotations: fls_e9hwvqsib5d5
// Modules
//
// ferrocene-annotations: fls_xdvdl2ssnhlo
// Statics
//
// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_85vx1qfa061i
// Traits
//
// ferrocene-annotations: fls_kgvleup5mdhq
// Type Aliases
//
// ferrocene-annotations: fls_fmdn7n7s413d
// Union Types
//
// ferrocene-annotations: fls_9gprp17h6t1q
// Use Imports
//
// ferrocene-annotations: fls_9kjpxri0axvg
// Weak Keywords
