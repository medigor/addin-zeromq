use std::error::Error;

use addin1c::{name, AddinResult, MethodInfo, Methods, PropInfo, SimpleAddin, Variant};
use smallvec::SmallVec;
use zmq::{Message, Socket};

use crate::impl_socket;

pub struct AddinReq {
    socket: Socket,
    msg: Message,
    parts: SmallVec<[Message; 4]>,
    last_error: Option<Box<dyn Error>>,
}

impl AddinReq {
    pub fn new(context: zmq::Context) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            socket: context.socket(zmq::REQ)?,
            msg: Message::new(),
            parts: SmallVec::new(),
            last_error: None,
        })
    }

    fn last_error(&mut self, value: &mut Variant) -> AddinResult {
        match &self.last_error {
            Some(err) => value
                .set_str1c(err.to_string().as_str())
                .map_err(|e| e.into()),
            None => value.set_str1c("").map_err(|e| e.into()),
        }
    }

    fn connect(&mut self, endpoint: &mut Variant, _ret_value: &mut Variant) -> AddinResult {
        let endpoint = endpoint.get_string()?;
        self.socket.connect(&endpoint)?;
        Ok(())
    }

    fn disconnect(&mut self, endpoint: &mut Variant, _ret_value: &mut Variant) -> AddinResult {
        let endpoint = endpoint.get_string()?;
        self.socket.disconnect(&endpoint)?;
        Ok(())
    }
    fn recv(&mut self, timeout: &mut Variant, ret_value: &mut Variant) -> AddinResult {
        impl_socket::recv(&self.socket, timeout, &mut self.msg, ret_value)
    }

    fn send(&mut self, data: &mut Variant, _ret_value: &mut Variant) -> AddinResult {
        impl_socket::send(&self.socket, data)
    }

    fn send_part(&mut self, data: &mut Variant, _ret_value: &mut Variant) -> AddinResult {
        impl_socket::send_part(&self.socket, data)
    }

    fn recv_multipart(&mut self, timeout: &mut Variant, ret_value: &mut Variant) -> AddinResult {
        impl_socket::recv_multipart(&self.socket, &mut self.parts, timeout, ret_value)
    }

    fn get_part(&mut self, part: &mut Variant, ret_value: &mut Variant) -> AddinResult {
        impl_socket::get_part(&mut self.parts, part, ret_value)
    }
}

impl SimpleAddin for AddinReq {
    fn name() -> &'static [u16] {
        name!("ZeroMQ.Req")
    }

    fn save_error(&mut self, err: Option<Box<dyn Error>>) {
        self.last_error = err;
    }

    fn methods() -> &'static [addin1c::MethodInfo<Self>] {
        &[
            MethodInfo {
                name: name!("Connect"),
                method: Methods::Method1(Self::connect),
            },
            MethodInfo {
                name: name!("Disconnect"),
                method: Methods::Method1(Self::disconnect),
            },
            MethodInfo {
                name: name!("Recv"),
                method: Methods::Method1(Self::recv),
            },
            MethodInfo {
                name: name!("Send"),
                method: Methods::Method1(Self::send),
            },
            MethodInfo {
                name: name!("SendPart"),
                method: Methods::Method1(Self::send_part),
            },
            MethodInfo {
                name: name!("RecvMultipart"),
                method: Methods::Method1(Self::recv_multipart),
            },
            MethodInfo {
                name: name!("GetPart"),
                method: Methods::Method1(Self::get_part),
            },
        ]
    }

    fn properties() -> &'static [PropInfo<Self>] {
        &[PropInfo {
            name: name!("LastError"),
            getter: Some(Self::last_error),
            setter: None,
        }]
    }
}
