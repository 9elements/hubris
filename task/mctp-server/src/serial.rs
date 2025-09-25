// mctp-baremetal
use mctp_stack;

use mctp::Result;

use ast1060_pac::Peripherals;
// use cortex_m_rt::entry;

#[cfg(feature = "jtag-halt")]
use core::ptr::{self, addr_of};

pub struct SerialSender {
    peripherals: Peripherals,
    serial_handler: mctp_stack::serial::MctpSerialHandler,
}

impl mctp_stack::Sender for SerialSender {
    fn send(
        &mut self,
        mut fragmenter: mctp_stack::fragment::Fragmenter,
        payload: &[u8],
    ) -> Result<mctp::Tag> {
        loop {
            let mut pkt = [0u8; mctp_stack::serial::MTU_MAX];
            let r = fragmenter.fragment(payload, &mut pkt);

            match r {
                mctp_stack::fragment::SendOutput::Packet(p) => {
                    // write this to sth taht imnplements embedded_io::Write
                    self.serial_handler
                        .send_sync(payload, &mut self.peripherals);
                }
                mctp_stack::fragment::SendOutput::Complete { tag, .. } => {
                    break Ok(tag)
                }
                mctp_stack::fragment::SendOutput::Error { err, .. } => {
                    break Err(err)
                }
            }
        }
    }

    fn get_mtu(&self) -> usize {
        mctp_stack::serial::MTU_MAX
    }
}

impl SerialSender {
    /// Create a new SerialSender instance with the neccessary serial setup code.
    pub fn new() -> Self {
        let peripherals = unsafe { Peripherals::steal() };
        peripherals.scu.scu000().modify(|_, w| w);
        peripherals.scu.scu41c().modify(|_, w| {
            // Set the JTAG pinmux to 0x1f << 25
            w.enbl_armtmsfn_pin()
                .bit(true)
                .enbl_armtckfn_pin()
                .bit(true)
                .enbl_armtrstfn_pin()
                .bit(true)
                .enbl_armtdifn_pin()
                .bit(true)
                .enbl_armtdofn_pin()
                .bit(true)
        });
        Self {
            peripherals,
            serial_handler: mctp_stack::serial::MctpSerialHandler::new(),
        }
    }
}

#[cfg(feature = "jtag-halt")]
fn jtag_halt() {
    static mut HALT: u32 = 1;

    // This is a hack to halt the CPU in JTAG mode.
    // It writes a value to a volatile memory location
    // Break by jtag and set val to zero to continue.
    loop {
        let val;
        unsafe {
            val = ptr::read_volatile(addr_of!(HALT));
        }

        if val == 0 {
            break;
        }
    }
}
