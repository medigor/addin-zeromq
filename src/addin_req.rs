use crate::client;
use addin1c::{name, AddinResult, MethodInfo, Methods, PropInfo, SimpleAddin, Variant};
use std::error::Error;

pub struct AddinReq {
    client: client::Client,
    last_error: Option<Box<dyn Error>>,
}

impl AddinReq {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            client: client::Client::new(zmq::REQ)?,
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
        self.client.connect(endpoint)
    }

    fn disconnect(&mut self, endpoint: &mut Variant, _ret_value: &mut Variant) -> AddinResult {
        self.client.disconnect(endpoint)
    }
    fn recv(&mut self, timeout: &mut Variant, ret_value: &mut Variant) -> AddinResult {
        self.client.recv(timeout, ret_value)
    }

    fn send(&mut self, data: &mut Variant, _ret_value: &mut Variant) -> AddinResult {
        self.client.send(data)
    }

    fn send_part(&mut self, data: &mut Variant, _ret_value: &mut Variant) -> AddinResult {
        self.client.send_part(data)
    }

    fn recv_multipart(&mut self, timeout: &mut Variant, ret_value: &mut Variant) -> AddinResult {
        self.client.recv_multipart(timeout, ret_value)
    }

    fn get_part(&mut self, part: &mut Variant, ret_value: &mut Variant) -> AddinResult {
        self.client.get_part(part, ret_value)
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
