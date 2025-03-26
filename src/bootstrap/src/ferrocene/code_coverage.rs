use crate::core::builder::Cargo;

pub(crate) fn instrument_coverage(cargo: &mut Cargo) {
    cargo.rustflag("-Cinstrument-coverage");
}
