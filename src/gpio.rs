use tock_registers::registers::{ReadOnly, ReadWrite, WriteOnly};
use tock_registers::{register_bitfields, register_structs};

register_structs! {
    GPIO {
        (0x00 => gpfsel: [ReadWrite<u32, FunctionSelect::Register>;6]),
        (0x18 => _reserved),
        (0x1C => gpset: WriteOnly<u64>),
        (0x24 => _reserved2),
        (0x28 => gpclr: WriteOnly<u64>),
        (0x30 => _reserved3),
        (0x34 => gplev: ReadOnly<u64>),
        (0x3C => _reserved4),
        (0x94 => gppud: ReadWrite<u32, GPIOPull::Register>),
        (0x98 => gppudclk: ReadWrite<u64, GPIOPullClock::Register>),
        (0xA0 => @END),
    }
}

register_bitfields! [u32,

    FunctionSelect [

        FSEL0 OFFSET(0) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            Alt0 = 0b100,
            Alt1 = 0b101,
            Alt2 = 0b110,
            Alt3 = 0b111,
            Alt4 = 0b011,
            Alt5 = 0b010
        ],

        FSEL1 OFFSET(3) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            Alt0 = 0b100,
            Alt1 = 0b101,
            Alt2 = 0b110,
            Alt3 = 0b111,
            Alt4 = 0b011,
            Alt5 = 0b010
        ],

        FSEL2 OFFSET(6) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            Alt0 = 0b100,
            Alt1 = 0b101,
            Alt2 = 0b110,
            Alt3 = 0b111,
            Alt4 = 0b011,
            Alt5 = 0b010
        ],

        FSEL3 OFFSET(9) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            Alt0 = 0b100,
            Alt1 = 0b101,
            Alt2 = 0b110,
            Alt3 = 0b111,
            Alt4 = 0b011,
            Alt5 = 0b010
        ],

        FSEL4 OFFSET(12) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            Alt0 = 0b100,
            Alt1 = 0b101,
            Alt2 = 0b110,
            Alt3 = 0b111,
            Alt4 = 0b011,
            Alt5 = 0b010
        ],

        FSEL5 OFFSET(15) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            Alt0 = 0b100,
            Alt1 = 0b101,
            Alt2 = 0b110,
            Alt3 = 0b111,
            Alt4 = 0b011,
            Alt5 = 0b010
        ],

        FSEL6 OFFSET(18) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            Alt0 = 0b100,
            Alt1 = 0b101,
            Alt2 = 0b110,
            Alt3 = 0b111,
            Alt4 = 0b011,
            Alt5 = 0b010
        ],

        FSEL7 OFFSET(21) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            Alt0 = 0b100,
            Alt1 = 0b101,
            Alt2 = 0b110,
            Alt3 = 0b111,
            Alt4 = 0b011,
            Alt5 = 0b010
        ],

        FSEL8 OFFSET(24) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            Alt0 = 0b100,
            Alt1 = 0b101,
            Alt2 = 0b110,
            Alt3 = 0b111,
            Alt4 = 0b011,
            Alt5 = 0b010
        ],

        FSEL9 OFFSET(27) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            Alt0 = 0b100,
            Alt1 = 0b101,
            Alt2 = 0b110,
            Alt3 = 0b111,
            Alt4 = 0b011,
            Alt5 = 0b010
        ]
    ],

    GPIOPull [
        PUD OFFSET(0) NUMBITS(2) [
            Off = 0b00,
            PullDown = 0b01,
            PullUp = 0b10
        ]
    ],
    GPIOPullClock [
        PUDCLK14 14,
        PUDCLK15 15
    ]
];
