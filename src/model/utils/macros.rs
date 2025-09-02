macro_rules! define_appendix {
    ($name:ident { $($id:literal => $variant:ident => $display:expr),* $(,)? }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum $name {
            $(
                $variant,
            )*
            Unknown(u8),
        }

        impl From<u8> for $name {
            fn from(id: u8) -> Self {
                match id {
                    $(
                        $id => $name::$variant,
                    )*
                    other => $name::Unknown(other),
                }
            }
        }

        impl $name {
            pub fn name(&self) -> &'static str {
                match self {
                    $(
                        $name::$variant => $display,
                    )*
                    $name::Unknown(_) => "Unknown",
                }
            }
        }
    };
}

pub(crate) use define_appendix;
