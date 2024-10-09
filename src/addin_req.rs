use std::error::Error;

use addin1c::{name, AddinResult, MethodInfo, Methods, PropInfo, SimpleAddin, Variant};
use zmq::{Message, PollEvents, Socket};

pub struct Addin {
    socket: Socket,
    msg: Message,
    last_error: Option<Box<dyn Error>>,
}

impl Addin {
    pub fn new(context: zmq::Context) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            socket: context.socket(zmq::REQ)?,
            msg: Message::new(),
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
        let timeout = timeout.get_i32()? as i64;
        if self.socket.poll(PollEvents::POLLIN, timeout)? != 1 {
            return Ok(());
        }

        self.socket.recv(&mut self.msg, zmq::DONTWAIT)?;
        ret_value.set_blob(&self.msg)?;

        Ok(())
    }

    fn send(&mut self, data: &mut Variant, _ret_value: &mut Variant) -> AddinResult {
        let data = data.get_blob()?;
        self.socket.send(data, 0)?;
        Ok(())
    }
}

impl SimpleAddin for Addin {
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
