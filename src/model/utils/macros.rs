macro_rules! define_appendix {
    ($name:ident { $($id:expr => $variant:ident $(=> $display:expr)?),* $(,)? }) => {
        #[derive(Debug, Clone, Copy)]
        pub enum $name { $($variant),* }

        impl $name {
            pub fn name(&self) -> &'static str {
                match self {
                    $(Self::$variant => define_appendix!(@display $variant $(, $display)?)),*
                }
            }
        }

        impl From<u8> for $name {
            fn from(value: u8) -> Self {
                match value {
                    $($id => Self::$variant),*,
                    _ => panic!("Unknown {} ID: {}", stringify!($name), value),
                }
            }
        }
    };

    (@display $variant:ident , $display:expr) => { $display };
    (@display $variant:ident) => { stringify!($variant) };
}

pub(crate) use define_appendix;
