// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! A demo task that echoes MCTP messages.
//!
//! The task configures the MCTP server for EID 8
//! and starts listening for MCTP Message Type `1` (PLDM) packets.
//! Received messages are echoed as is through the response channel.

#![no_std]
#![no_main]

use aspeed_ddk::uart::{Config, UartController};
use ast1060_pac::Peripherals;
use embedded_hal::delay::DelayNs;
use embedded_io::Write;
use mctp::{Eid, Listener, MsgType, RespChannel};
use mctp_api::{MctpListener, Stack};
use userlib::*;

task_slot!(MCTP, mctp_server);

#[derive(Clone, Default)]
struct DummyDelay;

impl DelayNs for DummyDelay {
    fn delay_ns(&mut self, ns: u32) {
        for _ in 0..ns {
            cortex_m::asm::nop();
        }
    }
}

#[export_name = "main"]
fn main() -> ! {
    let peripherals = unsafe { Peripherals::steal() };
    let uart = peripherals.uart;
    let mut delay = DummyDelay;

    let mut uart_controller = UartController::new(uart, &mut delay);
    unsafe {
        uart_controller.init(&Config {
            baud_rate: 115_200,
            word_length: aspeed_ddk::uart::WordLength::Eight as u8,
            parity: aspeed_ddk::uart::Parity::None,
            stop_bits: aspeed_ddk::uart::StopBits::One,
            clock: 24_000_000,
        });
    }

    writeln!(uart_controller, "\nHello from MCTP echo task!\n").unwrap();

    let stack = mctp_api::Stack::from(MCTP.get_task_id());

    stack.set_eid(Eid(8)).unwrap_lite();
    let mut listener = stack.listener(MsgType(1)).unwrap_lite();
    let mut recv_buf = [0; 255];

    loop {
        let (_, _, msg, mut resp) = listener.recv(&mut recv_buf).unwrap_lite();

        writeln!(
            uart_controller,
            "Got response from mctp server: {:?}\n",
            msg
        )
        .unwrap();

        match resp.send(msg) {
            Ok(_) => {}
            Err(_e) => {
                // Error sending response to peer
            }
        }
    }
}
