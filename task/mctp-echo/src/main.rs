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

use mctp::{Eid, Listener, MsgType, RespChannel};
use mctp_api::{MctpListener, Stack};
use userlib::*;

task_slot!(MCTP, mctp_server);
task_slot!(UART, uart_driver);

#[export_name = "main"]
fn main() -> ! {
    uart_send(b"\nHello from MCTP echo task!\n");

    let stack = mctp_api::Stack::from(MCTP.get_task_id());

    stack.set_eid(Eid(8)).unwrap_lite();
    let mut listener = stack.listener(MsgType(1)).unwrap_lite();
    let mut recv_buf = [0; 255];

    loop {
        let (_, _, msg, mut resp) = listener.recv(&mut recv_buf).unwrap_lite();

        uart_send(b"Received message from MCTP stack: ");
        uart_send(&msg);
        uart_send(b"\n");

        match resp.send(msg) {
            Ok(_) => {
                uart_send(b"Response sent successfully\n");
            }
            Err(_e) => {
                // Error sending response to peer
                uart_send(b"Error sending response\n");
            }
        }
    }
}

fn uart_send(text: &[u8]) {
    let peer = UART.get_task_id();

    const OP_WRITE: u16 = 1;
    let (code, _) =
        sys_send(peer, OP_WRITE, &[], &mut [], &[Lease::from(text)]);
    assert_eq!(0, code);
}
