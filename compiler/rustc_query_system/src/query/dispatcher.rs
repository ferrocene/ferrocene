use rustc_data_structures::fingerprint::Fingerprint;

use crate::ich::StableHashingContext;

pub type HashResult<V> = Option<fn(&mut StableHashingContext<'_>, &V) -> Fingerprint>;
