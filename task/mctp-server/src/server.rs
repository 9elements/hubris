use core::{
    cell::{OnceCell, RefCell},
    sync::atomic::AtomicBool,
};

use crate::ipc::*;

pub struct Server;

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
            let hello_world = "Hello World".as_bytes();
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
