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

use embedded_io::{Read, Write};
use lib_ast1060_uart::{InterruptDecoding, Usart};

#[export_name = "main"]
fn main() -> ! {
    let peripherals = unsafe { ast1060_pac::Peripherals::steal() };
    let usart = peripherals.uart;

    let mut usart = Usart::from(usart.deref());

    sys_irq_control(notifications::UART_IRQ_MASK, true);

    usart.write_all("Hello, World!\n".as_bytes()).unwrap_lite();
    usart.flush().unwrap_lite();

    let mut recv_buf = [0; 255];
    loop {
        let msg = sys_recv_open(&mut [], notifications::UART_IRQ_MASK);
        if msg.sender == TaskId::KERNEL
            && (msg.operation & notifications::UART_IRQ_MASK) != 0
        {
            let int = usart.read_interrupt_status();
            // usart.write_fmt(format_args!("\n{int:?}\n")).unwrap_lite();
            match int {
                InterruptDecoding::RxDataAvailable
                | InterruptDecoding::CharacterTimeout => {
                    usart.clear_rx_data_available_interrupt();
                    let n = usart.read(&mut recv_buf).unwrap_lite();
                    usart.set_rx_data_available_interrupt();
                    usart.write_all(&recv_buf[..n]).unwrap_lite();
                }
                InterruptDecoding::ModemStatusChange => {
                    usart.read_modem_status();
                }
                InterruptDecoding::TxEmpty => usart.clear_tx_idle_interrupt(),
                InterruptDecoding::LineStatusChange => {
                    usart.read_line_status();
                }
                InterruptDecoding::Unknown => {}
            }
            sys_irq_control(notifications::UART_IRQ_MASK, true);
        }
    }
}

include!(concat!(env!("OUT_DIR"), "/notifications.rs"));
