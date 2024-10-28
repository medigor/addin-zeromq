use crate::client;
use addin1c::{name, AddinResult, MethodInfo, Methods, PropInfo, SimpleAddin, Variant};
use std::error::Error;

pub struct AddinSub {
    client: client::Client,
    last_error: Option<Box<dyn Error>>,
}

impl AddinSub {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            client: client::Client::new(zmq::SUB)?,
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

    fn bind(&mut self, endpoint: &mut Variant, _ret_value: &mut Variant) -> AddinResult {
        self.client.bind(endpoint)
    }

    fn unbind(&mut self, endpoint: &mut Variant, _ret_value: &mut Variant) -> AddinResult {
        self.client.unbind(endpoint)
    }

    fn connect(&mut self, endpoint: &mut Variant, _ret_value: &mut Variant) -> AddinResult {
        self.client.connect(endpoint)
    }

    fn disconnect(&mut self, endpoint: &mut Variant, _ret_value: &mut Variant) -> AddinResult {
        self.client.disconnect(endpoint)
    }

    fn recv(&mut self, ret_value: &mut Variant) -> AddinResult {
        self.client.recv(ret_value)
    }

    fn recv_multipart(&mut self, ret_value: &mut Variant) -> AddinResult {
        self.client.recv_multipart(ret_value)
    }

    fn get_part(&mut self, part: &mut Variant, ret_value: &mut Variant) -> AddinResult {
        self.client.get_part(part, ret_value)
    }

    fn subscribe(&mut self, data: &mut Variant, _ret_value: &mut Variant) -> AddinResult {
        self.client.subscribe(data)
    }

    fn set_recv_timeout(&mut self, timeout: &mut Variant, _ret_value: &mut Variant) -> AddinResult {
        self.client.set_recv_timeout(timeout)
    }
}

impl SimpleAddin for AddinSub {
    fn name() -> &'static [u16] {
        name!("ZeroMQ.Sub")
    }

    fn save_error(&mut self, err: Option<Box<dyn Error>>) {
        self.last_error = err;
    }

    fn methods() -> &'static [addin1c::MethodInfo<Self>] {
        &[
            MethodInfo {
                name: name!("Bind"),
                method: Methods::Method1(Self::bind),
            },
            MethodInfo {
                name: name!("Unbind"),
                method: Methods::Method1(Self::unbind),
            },
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
                method: Methods::Method0(Self::recv),
            },
            MethodInfo {
                name: name!("RecvMultipart"),
                method: Methods::Method0(Self::recv_multipart),
            },
            MethodInfo {
                name: name!("GetPart"),
                method: Methods::Method1(Self::get_part),
            },
            MethodInfo {
                name: name!("Subscribe"),
                method: Methods::Method1(Self::subscribe),
            },
            MethodInfo {
                name: name!("SetRecvTimeout"),
                method: Methods::Method1(Self::set_recv_timeout),
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
