use tock_registers::registers::{Aliased, ReadOnly, ReadWrite};
use tock_registers::{register_bitfields, register_structs};

register_structs! {
    pub AuxiliaryUart {
        (0x00 => aux_irq: ReadOnly<u8, AuxiliaryInterrupt::Register>),
        (0x04 => pub aux_enables: ReadWrite<u8, AuxiliaryEnable::Register>),
        (0x40 => pub aux_mu_io_reg: ReadWrite<u8>),
        (0x44 => pub aux_mu_ier_reg: ReadWrite<u8, EnableInterrupt::Register>),
        (0x48 => aux_mu_iir_reg: Aliased<u8, InterruptStatus::Register, FIFOClear::Register>),
        (0x4C => pub aux_mu_lcr_reg: ReadWrite<u8, LineControl::Register>),
        (0x50 => pub aux_mu_mcr_reg: ReadWrite<u8, ModemControl::Register>),
        (0x54 => pub aux_mu_lsr_reg: ReadOnly<u8, LineStatus::Register>),
        (0x58 => aux_mu_msr_reg: ReadOnly<u8, ModemStatus::Register>),
        (0x5C => aux_mu_scratch: ReadWrite<u8>),
        (0x60 => pub aux_mu_cntl_reg: ReadWrite<u8, ExtraControl::Register>),
        (0x64 => aux_mu_stat_reg: ReadOnly<u32, ExtraStatus::Register>),
        (0x68 => pub aux_mu_baud_reg: ReadWrite<u16>),

        (0xC0 => @END),
    }
}

register_bitfields! [u8,

    pub AuxiliaryInterrupt [
        MiniUARTInterruptPending 0,
        SPI1InterruptPending 1,
        SPI2InterruptPending 2
    ],

    pub AuxiliaryEnable [
        MiniUARTEnable 0,
        SPI1Enable 1,
        SPI2Enable 2
    ],

    pub EnableInterrupt [
        EnableReceiveInterrupt 0,
        EnableTransmitInterrupt 1
    ],

    InterruptStatus [
        InterruptPending OFFSET(0) NUMBITS(1) [],
        InterruptID OFFSET(1) NUMBITS(2) [
            NoInterrupts = 0b00,
            EmptyTransmitHoldingRegister = 0b01,
            ReceiverHoldsValidByte = 0b10
        ],
        FIFOEnabled OFFSET(6) NUMBITS(2) []
    ],

    FIFOClear [
        FIFOClear OFFSET(1) NUMBITS(2) [
            ClearReceiveFIFO = 0b01,
            ClearTransmitFIFO = 0b10,
            ClearBothFIFO = 0b11
        ]
    ],

    pub LineControl [
        DataSize OFFSET(0) NUMBITS(2) [
            SevenBitMode = 0b00,
            EightBitMode = 0b11
        ],
        Break OFFSET(6) NUMBITS(1) [],
        DLABAccess OFFSET(7) NUMBITS(1) []
    ],

    pub ModemControl [
        RTS 1
    ],

    pub LineStatus [
        DataReady 0,
        ReceiverOverrun 1,
        EmptyTransmitter 5,
        IdleTransmitter 6
    ],

    ModemStatus [
        CTSStatus 5
    ],

    pub ExtraControl [
        EnableReceiver OFFSET(0) NUMBITS(1) [],
        EnableTransmitter OFFSET(1) NUMBITS(1) [],
        EnableReceiverAutoFlowControl OFFSET(2) NUMBITS(1) [],
        EnableTransmitterAutoFlowControl OFFSET(3) NUMBITS(1) [],
        RTSAutoFlowLevel OFFSET(4) NUMBITS(2) [
            ThreeEmptySpaces = 0b00,
            TwoEmptySpaces = 0b01,
            OneEmptySpace = 0b10,
            FourEmptySpaces = 0b11
        ],
        RTSAssertLevel OFFSET(6) NUMBITS(1) [],
        CTSAssertLevel OFFSET(7) NUMBITS(1) []
    ]
];

register_bitfields! [u32,
    ExtraStatus [
        SymbolAvailable OFFSET(0) NUMBITS(1) [],
        SpaceAvailable OFFSET(1) NUMBITS(1) [],
        IdleReceiver OFFSET(2) NUMBITS(1) [],
        IdleTransmitter OFFSET(3) NUMBITS(1) [],
        OverrunReceiver OFFSET(4) NUMBITS(1) [],
        FullTransmitFIFO OFFSET(5) NUMBITS(1) [],
        RTSStatus OFFSET(6) NUMBITS(1) [],
        CTSLine OFFSET(7) NUMBITS(1) [],
        EmptyTransmitFIFO OFFSET(8) NUMBITS(1) [],
        TransmissionDone OFFSET(9) NUMBITS(1) [],
        ReceiveFIFOFillLevel OFFSET(16) NUMBITS(4) [],
        TransmitFIFOFillLevel OFFSET(24) NUMBITS(4) []
    ]

];
