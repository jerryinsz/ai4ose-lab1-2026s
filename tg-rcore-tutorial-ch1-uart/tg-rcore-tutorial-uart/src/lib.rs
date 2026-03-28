#![no_std]

use spin::Mutex;

const UART0: usize = 0x1000_0000;

/// UART Registers
const RHR: usize = 0; // Receive Holding Register (read mode)
const THR: usize = 0; // Transmit Holding Register (write mode)
const IER: usize = 1; // Interrupt Enable Register
const FCR: usize = 2; // FIFO Control Register
const LCR: usize = 3; // Line Control Register
const LSR: usize = 5; // Line Status Register

/// Register bits
const LCR_EIGHT_BITS: u8 = 3 << 0;
const LCR_BAUD_LATCH: u8 = 1 << 7;
const FCR_FIFO_ENABLE: u8 = 1 << 0;
const FCR_FIFO_CLEAR: u8 = 3 << 1;
const IER_RX_ENABLE: u8 = 1 << 0;
const IER_TX_ENABLE: u8 = 1 << 1;
const LSR_TX_IDLE: u8 = 1 << 5;

pub struct Uart {
    base_address: usize,
}

impl Uart {
    pub const fn new(base_address: usize) -> Self {
        Self { base_address }
    }

    fn read_reg(&self, reg: usize) -> u8 {
        unsafe { ((self.base_address + reg) as *mut u8).read_volatile() }
    }

    fn write_reg(&mut self, reg: usize, value: u8) {
        unsafe {
            ((self.base_address + reg) as *mut u8).write_volatile(value);
        }
    }

    pub fn init(&mut self) {
        // Disable interrupts.
        self.write_reg(IER, 0x00);

        // Special mode to set baud rate.
        self.write_reg(LCR, LCR_BAUD_LATCH);

        // LSB for baud rate of 38.4K.
        self.write_reg(0, 0x03);

        // MSB for baud rate of 38.4K.
        self.write_reg(1, 0x00);

        // Leave set-baud mode,
        // and set word length to 8 bits, no parity.
        self.write_reg(LCR, LCR_EIGHT_BITS);

        // Reset and enable FIFOs.
        self.write_reg(FCR, FCR_FIFO_ENABLE | FCR_FIFO_CLEAR);

        // Enable transmit and receive interrupts.
        self.write_reg(IER, IER_TX_ENABLE | IER_RX_ENABLE);
    }

    pub fn put_char(&mut self, c: u8) {
        while (self.read_reg(LSR) & LSR_TX_IDLE) == 0 {
            core::hint::spin_loop();
        }
        self.write_reg(THR, c);
    }
}

pub static UART: Mutex<Uart> = Mutex::new(Uart::new(UART0));

pub fn init() {
    UART.lock().init();
}

pub fn print_str(s: &str) {
    let mut uart = UART.lock();
    for byte in s.bytes() {
        uart.put_char(byte);
    }
}
