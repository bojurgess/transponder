#[macro_export]
macro_rules! assert_packet_size {
    ($struct:ty, $size:expr) => {
        const _: () = {
            use std::mem::size_of;
            assert!(
                size_of::<$struct>() == $size,
                concat!("Size mismatch for ", stringify!($struct))
            );
        };
    };
}

macro_rules! impl_has_header {
    ($ty:ty) => {
        use crate::packet::HasHeader;
        impl HasHeader for $ty {
            fn header(&self) -> &PacketHeader {
                &self.header
            }
        }
    };
}

pub(crate) use impl_has_header;
