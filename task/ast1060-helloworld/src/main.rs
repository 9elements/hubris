// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![no_std]
#![no_main]

// NOTE: you will probably want to remove this when you write your actual code;
// we need to import userlib to get this to compile, but it throws a warning
// because we're not actually using it yet!
use core::ops::Deref;
#[allow(unused_imports)]
use userlib::*;

use embedded_io::Write;
use lib_ast1060_uart::Usart;

#[export_name = "main"]
fn main() -> ! {
    let peripherals = unsafe { ast1060_pac::Peripherals::steal() };
    let usart = peripherals.uart;

    let mut usart = Usart::from(usart.deref());

    // USART side yet, so this won't trigger notifications yet.
    sys_irq_control(notifications::UART_IRQ_MASK, true);

    usart.write("Hello, World!".as_bytes()).unwrap_lite();

    loop {
        // NOTE: you need to put code here before running this! Otherwise LLVM
        // will turn this into a single undefined instruction.
    }
}

include!(concat!(env!("OUT_DIR"), "/notifications.rs"));
