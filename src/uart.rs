// MIT License

// Copyright (c) 2021 AnonmousDapper

// for controlling the ns16550a uart used by Qemu

use spin::Mutex;

use core::fmt;

use lazy_static::lazy_static;

static UART_ADDRESS: usize = 0x1000_0000;

lazy_static! {
    pub static ref UART: Mutex<Uart> = {
        let uart = Uart {
            base_address: UART_ADDRESS,
        };
        uart.init();
        Mutex::new(uart)
    };
}

pub struct Uart {
    base_address: usize,
}

impl Uart {
    #[allow(clippy::missing_panics_doc)]
    pub fn init(&self) {
        let ptr = self.base_address as *mut u8;

        unsafe {
            // Line Control - set word length to 8 bits
            let lcr = (1 << 0) | (1 << 1);
            ptr.add(3).write_volatile(lcr);

            // FCR - enable FIFO
            ptr.add(2).write_volatile(1 << 0);

            // IER - interrupt enable - enable receiver buffer interrupts
            ptr.add(1).write_volatile(1 << 0);

            // TODO: real math for real hardware later
            let divisor: u16 = 592;
            let dl_least: u8 = (divisor & 0xff).try_into().unwrap();
            let dl_most: u8 = (divisor >> 8).try_into().unwrap();

            // set divisor latch access bit to write DLL/DLM
            ptr.add(3).write_volatile(lcr | 1 << 7);

            ptr.add(0).write_volatile(dl_least);
            ptr.add(1).write_volatile(dl_most);

            // reset DLAB for normal functionality
            ptr.add(3).write_volatile(lcr);
        }
    }

    fn read_byte(&mut self) -> Option<u8> {
        let ptr = self.base_address as *mut u8;

        unsafe {
            if ptr.add(5).read_volatile() & 1 == 0 {
                None
            } else {
                Some(ptr.add(0).read_volatile())
            }
        }
    }

    fn write_byte(&mut self, data: u8) {
        let ptr = self.base_address as *mut u8;

        unsafe {
            ptr.add(0).write_volatile(data);
        }
    }

    pub fn write(&mut self, data: impl AsRef<[u8]>) {
        for byte in data.as_ref() {
            self.write_byte(*byte);
        }
    }

    pub fn read_char(&mut self) -> Option<char> {
        self.read_byte().map(|byte| byte as char)
    }

    /*pub fn read(&self) -> Option<&str> {
        let bytes = {
            let mut buf = Vec::new();
            while let Some(byte) = self.read_byte() {
                buf.push(byte);
            }

            &buf
        };

        core::str::from_utf8(bytes)
    }*/
}

impl fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write(s);

        Ok(())
    }
}

#[macro_export]
macro_rules! sprint {
    ($($arg:tt)*) => {{
        $crate::uart::_print(format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! sprintln {
  () => ($crate::sprint!("\n"));
  ($fmt:expr) => ($crate::sprint!(concat!($fmt, "\n")));
  ($fmt:expr, $($arg:tt)*) => ($crate::sprint!(concat!($fmt, "\n"), $($arg)*));
}

pub fn _print(args: fmt::Arguments<'_>) {
    use fmt::Write;

    UART.lock()
        .write_fmt(args)
        .expect("Failed writing to serial");
}
