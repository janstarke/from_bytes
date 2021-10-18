use std::mem::size_of;
use duplicate::duplicate;

pub trait StructFromBytes<T: PackedSize = Self>: PackedSize {
    /// Creates an instance of `T` by parsing a slice of bytes.
    /// 
    /// This method assumes that the byte slice contains unaligned data.
    fn from_bytes(slice: &[u8], offset: usize) -> std::io::Result<Box<Self>>;
}


pub trait PackedSize {
    /// Returns the packed size of the struct. 
    /// 
    /// What is th packed size? Normally, struct members are aligned in a way that optimizes
    /// access speed. For example, on a 64bit platform, all members are aligned to start
    /// on a 64bit boundary [https://doc.rust-lang.org/reference/type-layout.html](https://doc.rust-lang.org/reference/type-layout.html).
    /// 
    /// *Packed* means what would be the size if no alignment would be used. In fact,
    /// packed size equals the sum of sizes of all members.
    fn packed_size() -> usize;
}

#[duplicate(
    int_type;
    [u8];
    [u16];
    [u32];
    [u64];
    [u128];
    [i8];
    [i16];
    [i32];
    [i64];
    [i128];
)]
impl PackedSize for int_type { fn packed_size() -> usize { return size_of::<int_type>(); } }

#[cfg(feature = "u8_arrays")]
#[duplicate(array_length; [1]; [2]; [3]; [4]; [6]; [8]; [10]; [12]; [14]; [16]; )]
impl PackedSize for [u8;array_length] { fn packed_size() -> usize { return array_length * size_of::<u8>(); } }

#[cfg(feature = "u16_arrays")]
#[duplicate(array_length; [1]; [2]; [3]; [4]; [6]; [8]; [10]; [12]; [14]; [16]; )]
impl PackedSize for [u16;array_length] { fn packed_size() -> usize { return array_length * size_of::<u16>(); } }

#[cfg(feature = "u32_arrays")]
#[duplicate(array_length; [1]; [2]; [3]; [4]; [6]; [8]; [10]; [12]; [14]; [16]; )]
impl PackedSize for [u32;array_length] { fn packed_size() -> usize { return array_length * size_of::<u32>(); } }

#[cfg(feature = "u64_arrays")]
#[duplicate(array_length; [1]; [2]; [3]; [4]; [6]; [8]; [10]; [12]; [14]; [16]; )]
impl PackedSize for [u64;array_length] { fn packed_size() -> usize { return array_length * size_of::<u64>(); } }

#[cfg(feature = "i8_arrays")]
#[duplicate(array_length; [1]; [2]; [3]; [4]; [6]; [8]; [10]; [12]; [14]; [16]; )]
impl PackedSize for [i8;array_length] { fn packed_size() -> usize { return array_length * size_of::<i8>(); } }

#[cfg(feature = "i16_arrays")]
#[duplicate(array_length; [1]; [2]; [3]; [4]; [6]; [8]; [10]; [12]; [14]; [16]; )]
impl PackedSize for [i16;array_length] { fn packed_size() -> usize { return array_length * size_of::<i16>(); } }

#[cfg(feature = "i32_arrays")]
#[duplicate(array_length; [1]; [2]; [3]; [4]; [6]; [8]; [10]; [12]; [14]; [16]; )]
impl PackedSize for [i32;array_length] { fn packed_size() -> usize { return array_length * size_of::<i32>(); } }

#[cfg(feature = "i64_arrays")]
#[duplicate(array_length; [1]; [2]; [3]; [4]; [6]; [8]; [10]; [12]; [14]; [16]; )]
impl PackedSize for [i64;array_length] { fn packed_size() -> usize { return array_length * size_of::<i64>(); } }