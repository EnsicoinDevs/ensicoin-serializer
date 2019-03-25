/// Type representing a Unit of variable length as described in the [ensicoin
/// protocol](https://github.com/EnsicoinDevs/ensicoin/blob/master/messages.md#variable-length-integer-var_uint)
#[derive(Hash, Eq, PartialEq)]
pub struct VarUint {
    pub value: u64,
}

pub type Sha256Result = generic_array::GenericArray<u8, typenum::U32>;
