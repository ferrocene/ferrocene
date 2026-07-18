// Regression test for issue #95879.
//@ edition: 2015

use unresolved_crate::module::Name; //~ ERROR cannot find

/// [Name]
pub struct S;
