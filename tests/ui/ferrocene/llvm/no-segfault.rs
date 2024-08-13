//@ check-pass
//@ compile-flags: -O

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum EnumValue {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
}

impl EnumValue {
    pub fn map_value(self) -> usize {
        match self {
            EnumValue::A => 0,
            EnumValue::B => 1,
            EnumValue::C => 2,
            EnumValue::D => 0,
        }
    }

}

pub trait ProviderTrait {
    fn get(&self, value: EnumValue) -> u64;
}

pub struct ProviderTraitImpl<'a> {
    pub data: [&'a [u64; 3]; 4],
}

impl<'a> ProviderTrait for ProviderTraitImpl<'a> {
    fn get(&self, value: EnumValue) -> u64 {
        if value < EnumValue::D {
            self.data[value as usize][value.map_value()]
        } else {
            self.data[3][value.map_value()]
        }
    }
}

fn main() {}
