use crate::ipc::*;
use heapless::LinearMap;
use idol_runtime::{Leased, R, W};
use mctp::{Eid, MsgIC, MsgType, Tag, TagValue};
use mctp_stack::{AppCookie, Router};
use userlib::*;
use zerocopy::IntoBytes;

// TODO: Use configuration from mctp-lib (mctp-estack)
//       see https://github.com/OpenPRoT/mctp-lib/issues/4
const MAX_PAYLOAD: usize = 1023;

pub struct Server<S: mctp_stack::Sender, const OUTSTANDING: usize> {
    stack: Router<S>,
    outstanding: LinearMap<GenericHandle, RecvMessage, OUTSTANDING>,
}

impl<S: mctp_stack::Sender, const OUTSTANDING: usize> Server<S, OUTSTANDING> {
    pub fn new(own_eid: mctp::Eid, now_millis: u64, outbound: S) -> Self {
        let stack = Router::new(own_eid, now_millis, outbound);
        Self {
            stack,
            outstanding: LinearMap::new(),
        }
    }
    pub fn req(&mut self, msg: &RecvMessage, eid: u8) {
        match self.stack.req(mctp::Eid(eid)) {
            Ok(handle) => {
                let handle = GenericHandle(handle.0 as u32); // TODO better mapping
                sys_reply(msg.sender, 0, handle.as_bytes())
            }
            Err(e) => {
                sys_reply(msg.sender, ServerError::from(e).into(), &[]);
            }
        }
    }
    pub fn listener(&mut self, msg: &RecvMessage, typ: u8) {
        match self.stack.listener(mctp::MsgType(typ)) {
            Ok(handle) => {
                let handle = GenericHandle(handle.0 as u32); // TODO better mapping
                sys_reply(msg.sender, 0, handle.as_bytes())
            }
            Err(e) => {
                sys_reply(msg.sender, ServerError::from(e).into(), &[]);
            }
        }
    }
    pub fn get_eid(&mut self, msg: &RecvMessage) {
        sys_reply(msg.sender, 0, self.stack.get_eid().0.as_bytes());
    }
    pub fn set_eid(&mut self, msg: &RecvMessage, eid: u8) {
        match self.stack.set_eid(mctp::Eid(eid)) {
            Ok(()) => sys_reply(msg.sender, 0, &[]),
            Err(e) => {
                sys_reply(msg.sender, ServerError::from(e).into(), &[]);
            }
        }
    }
    pub fn recv(
        &mut self,
        msg: RecvMessage,
        handle: GenericHandle,
        buf: Leased<W, [u8]>,
    ) {
        // Check for a message for the given handle, if there is one, copy it into buf and reply.
        // Otherwise postpone the reply until a message arrives or a timeout occurs.
        // For this the RecvMessage must be stored.
        // On inbound message arrival, all postponed recv calls must be checked for a match.
        // Likewise timeouts have to be handled once we have a timeout mechanism.

        let cookie = AppCookie(handle.0 as usize);
        if let Some(mctp_message) = self.stack.recv(cookie) {
            if mctp_message.payload.len() > buf.len() {
                sys_reply(msg.sender, ServerError::NoSpace.into(), &[]);
                return;
            }
            buf.write_range(
                0..mctp_message.payload.len(),
                mctp_message.payload,
            );
            todo!("send reply");
            // return;
        }

        self.outstanding.insert(handle, msg).unwrap_lite();
    }
    pub fn send(
        &mut self,
        msg: &RecvMessage,
        handle: GenericHandle,
        typ: u8,
        eid: Option<u8>,
        tag: Option<u8>,
        ic: bool,
        buf: Leased<R, [u8]>,
    ) {
        // The Router currently supports blocking send only, so this should be quite easy.
        // TODO figure out if handling incoming packets while sending is neccessary.

        let mut msg_buf = [0; MAX_PAYLOAD];
        if msg_buf.len() < buf.len() {
            sys_reply(msg.sender, ServerError::NoSpace.into(), &[]);
        }
        if buf.read_range(0..buf.len(), &mut msg_buf).is_err() {
            todo!("client died?");
        }
        let res = self.stack.send(
            eid.map(|id| Eid(id)),
            MsgType(typ),
            tag.map(|x| Tag::Owned(TagValue(x))),
            MsgIC(ic),
            AppCookie(handle.0 as usize),
            &msg_buf,
        );

        match res {
            Ok(tag) => {
                let mut buf = [0; 8];
                let l =
                    hubpack::serialize(&mut buf, &tag.tag().0).unwrap_lite();
                sys_reply(msg.sender, 0, &buf[..l]);
            }
            Err(e) => {
                sys_reply(msg.sender, ServerError::from(e).into(), &[]);
            }
        }
    }

    pub fn update(&mut self) {
        todo!(
            "update the stack, check for receive calls that can be fullfilled"
        )
    }
}
