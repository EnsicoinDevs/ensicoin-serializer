/// Type representing a Unit of variable length as described in the [ensicoin
/// protocol](https://github.com/EnsicoinDevs/ensicoin/blob/master/messages.md#variable-length-integer-var_uint)
pub struct VarUint {
    pub value: u64,
}
