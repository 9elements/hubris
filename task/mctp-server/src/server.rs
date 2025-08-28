use core::{
    cell::{OnceCell, RefCell},
    sync::atomic::AtomicBool,
};

use crate::ipc::*;
use mctp::Error;
use mctp_api::Stack;
use mctp_estack::{fragment, MctpHeader};
use userlib::*;

// Taken from mcpt-estack/src/router.rs
pub struct PktBuf<'a> {
    data: &'a [u8; 255],
    len: usize,
}

impl PktBuf<'_> {
    const fn new() -> Self {
        Self {
            data: &[0u8; 255],
            len: 0,
        }
    }

    fn set(&mut self, data: &[u8]) -> Result<(), Error> {
        debug_assert!(MctpHeader::decode(data).is_ok());

        let dst = self.data.get_mut(..data.len()).ok_or(Error::NoSpace)?;
        // TODO: use the lease stuff here?
        dst.copy_from_slice(data);
        self.len = data.len();
        Ok(())
    }
}

impl Default for PktBuf<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl core::ops::Deref for PktBuf<'_> {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        &self.data[..self.len]
    }
}

pub struct Server {
    pub stack: mctp_api::Stack,
}

impl Server {
    /// A new server instance with a random Eid
    pub fn new() -> Self {
        Self {
            // get this somehow from task it like in server.rs
            stack: mctp_api::Stack::from(TaskId(1 as u16)),
        }
    }
}

impl crate::ipc::InOrderMCTPImpl for Server {
    fn req(
        &mut self,
        msg: &userlib::RecvMessage,
        eid: u8,
    ) -> Result<GenericHandle, idol_runtime::RequestError<ServerError>>
    where
        ServerError: idol_runtime::IHaveConsideredServerDeathWithThisErrorType,
    {
        todo!()
    }

    fn listener(
        &mut self,
        msg: &userlib::RecvMessage,
        typ: u8,
    ) -> Result<GenericHandle, idol_runtime::RequestError<ServerError>>
    where
        ServerError: idol_runtime::IHaveConsideredServerDeathWithThisErrorType,
    {
        Ok(GenericHandle(211))
    }

    fn get_eid(
        &mut self,
        msg: &userlib::RecvMessage,
    ) -> Result<u8, idol_runtime::RequestError<core::convert::Infallible>> {
        todo!()
    }

    fn set_eid(
        &mut self,
        msg: &userlib::RecvMessage,
        eid: u8,
    ) -> Result<(), idol_runtime::RequestError<ServerError>>
    where
        ServerError: idol_runtime::IHaveConsideredServerDeathWithThisErrorType,
    {
        Ok(())
    }

    fn recv(
        &mut self,
        msg: &userlib::RecvMessage,
        handle: GenericHandle,
        buf: idol_runtime::Leased<idol_runtime::W, [u8]>,
    ) -> Result<RecvMetadata, idol_runtime::RequestError<ServerError>>
    where
        ServerError: idol_runtime::IHaveConsideredServerDeathWithThisErrorType,
    {
        static GOT_RESPONSE: AtomicBool = AtomicBool::new(false);
        if GOT_RESPONSE.load(core::sync::atomic::Ordering::Relaxed) == false {
            let hello_world = "Hello from MCTP Stack!".as_bytes();
            buf.write_range(0..hello_world.len(), hello_world);
            GOT_RESPONSE.store(true, core::sync::atomic::Ordering::Relaxed);
            return Ok(RecvMetadata {
                msg_typ: 1,
                msg_ic: false,
                msg_tag: 2,
                remote_eid: 42,
                size: buf.len() as u64,
                resp_handle: Some(GenericHandle(1)),
            });
        } else {
            loop {
                // Do nothing
            }
        }
    }

    fn send(
        &mut self,
        msg: &userlib::RecvMessage,
        handle: GenericHandle,
        typ: u8,
        tag: Option<u8>,
        ic: bool,
        buf: idol_runtime::Leased<idol_runtime::R, [u8]>,
    ) -> Result<u8, idol_runtime::RequestError<ServerError>>
    where
        ServerError: idol_runtime::IHaveConsideredServerDeathWithThisErrorType,
    {
        // 1. Parse message components and construct fragmenter for later transport medium
        // let mut pkt_buf = PktBuf::new();
        // pkt_buf.set(&buf);
        // let fragmenter = self.stack.start_send();
        // let mctp_message = mctp_estack::MctpMessage {};
        let header = MctpHeader::decode(buf).is_ok();

        // 2. Route the message to the corresponding particpiants
        Ok(0)
    }
}

impl idol_runtime::NotificationHandler for Server {
    fn current_notification_mask(&self) -> u32 {
        // No notifications atm
        0
    }

    fn handle_notification(&mut self, bits: u32) {
        unreachable!()
    }
}
