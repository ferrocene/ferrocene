// This does not currently propagate (#67862)
// unit-test: ConstProp

// EMIT_MIR ref_deref_project.main.ConstProp.diff
fn main() {
    // CHECK-LABEL: fn main(
    // CHECK: debug a => [[a:_.*]];
    // CHECK: [[a]] = (*{{_.*}});
    let a = *(&(4, 5).1);
}
