use bitflags::bitflags;

bitflags! {
    pub struct ButtonFlags: u32 {
        const CROSS_A        = 0x00000001;
        const TRIANGLE_Y     = 0x00000002;
        const CIRCLE_B       = 0x00000004;
        const SQUARE_X       = 0x00000008;
        const DPAD_LEFT      = 0x00000010;
        const DPAD_RIGHT     = 0x00000020;
        const DPAD_UP        = 0x00000040;
        const DPAD_DOWN      = 0x00000080;
        const OPTIONS_MENU   = 0x00000100;
        const L1_LB          = 0x00000200;
        const R1_RB          = 0x00000400;
        const L2_LT          = 0x00000800;
        const R2_RT          = 0x00001000;
        const LEFT_STICK     = 0x00002000;
        const RIGHT_STICK    = 0x00004000;
        const RIGHT_STICK_LEFT  = 0x00008000;
        const RIGHT_STICK_RIGHT = 0x00010000;
        const RIGHT_STICK_UP    = 0x00020000;
        const RIGHT_STICK_DOWN  = 0x00040000;
        const SPECIAL        = 0x00080000;
        const UDP_ACTION_1   = 0x00100000;
        const UDP_ACTION_2   = 0x00200000;
        const UDP_ACTION_3   = 0x00400000;
        const UDP_ACTION_4   = 0x00800000;
        const UDP_ACTION_5   = 0x01000000;
        const UDP_ACTION_6   = 0x02000000;
        const UDP_ACTION_7   = 0x04000000;
        const UDP_ACTION_8   = 0x08000000;
        const UDP_ACTION_9   = 0x10000000;
        const UDP_ACTION_10  = 0x20000000;
        const UDP_ACTION_11  = 0x40000000;
        const UDP_ACTION_12  = 0x80000000;
    }
}
