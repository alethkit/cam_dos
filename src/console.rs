use crate::uart::{AuxiliaryUart, AuxiliaryEnable, LineControl, EnableInterrupt, ModemControl, ExtraControl, LineStatus};
use crate::gpio::{GPIO, FunctionSelect, GPIOPull, GPIOPullClock};


pub struct Console {
    uart: *const AuxiliaryUart,
    gpio: *const GPIO
}


impl Console {

    pub fn new(uart_base_address: usize, gpio_base_address: usize) -> Self {
        // Will probably cause undefined behaviour if addresses are incorrect
       let new_console = Console {
            uart: uart_base_address as *const AuxiliaryUart,
            gpio: gpio_base_address as *const GPIO
        };
       unsafe {
           let gpio = &*new_console.gpio;
           gpio.gpfsel[1].modify(FunctionSelect::FSEL4::Alt0 + FunctionSelect::FSEL5::Alt0);
           gpio.gppud.modify(GPIOPull::PUD::Off);
            for _ in 0..150 {
                asm!("nop");
            };
            gpio.gppudclk.modify(GPIOPullClock::PUDCLK14::SET + GPIOPullClock::PUDCLK15::SET);
            for _ in 0..150 {
                asm!("nop");
            };
            gpio.gppudclk.set(0);
            
            let uart = &*new_console.uart;
            uart.aux_enables.write(AuxiliaryEnable::MiniUARTEnable::SET);
            uart.aux_mu_lcr_reg.write(LineControl::DLABAccess::CLEAR + LineControl::DataSize::EightBitMode);
            uart.aux_mu_cntl_reg.set(0);
            uart.aux_mu_ier_reg.write(EnableInterrupt::EnableReceiveInterrupt::CLEAR + EnableInterrupt::EnableTransmitInterrupt::CLEAR);
            uart.aux_mu_mcr_reg.write(ModemControl::RTS::CLEAR);
            uart.aux_mu_baud_reg.set(270);
            uart.aux_mu_cntl_reg.write(ExtraControl::EnableTransmitter::SET + ExtraControl::EnableReceiver::SET);
            new_console
        }
    }

    pub fn send_char(&self, c: u8) {
        unsafe {
            let uart = &*self.uart;
            while !uart.aux_mu_lsr_reg.is_set(LineStatus::EmptyTransmitter) {}
            uart.aux_mu_io_reg.set(c);
        }
    }

    pub fn recv_char(&self) -> u8 {
        unsafe {
            let uart = &*self.uart;
            while !uart.aux_mu_lsr_reg.is_set(LineStatus::DataReady) {}
            uart.aux_mu_io_reg.get()
        }
    }
}
