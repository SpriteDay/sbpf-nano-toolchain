/// Endianness of the target, which must match cfg(target-endian)
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Endian {
    Little,
    Big,
}
