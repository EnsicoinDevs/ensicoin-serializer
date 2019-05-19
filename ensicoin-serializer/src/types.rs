/// Type representing a Unit of variable length as described in the [ensicoin
/// protocol](https://github.com/EnsicoinDevs/ensicoin/blob/master/messages.md#variable-length-integer-var_uint)
#[derive(Hash, Eq, PartialEq)]
pub struct VarUint {
    pub value: u64,
}

pub type Sha256Result = generic_array::GenericArray<u8, typenum::U32>;

pub fn hash_to_string(hash: &Sha256Result) -> String {
    hash.iter()
        .fold(String::new(), |acc, b| format!("{}{:x}", acc, b))
}
