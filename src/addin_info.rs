use std::sync::LazyLock;

use addin1c::{name, AddinResult, PropInfo, SimpleAddin, Variant};
use uuid::Uuid;

static INSTANCE: LazyLock<Uuid> = LazyLock::new(Uuid::new_v4);

pub struct AddinInfo {}

impl AddinInfo {
    pub fn new() -> Self {
        Self {}
    }
    fn instance(&mut self, value: &mut Variant) -> AddinResult {
        value.set_str1c(INSTANCE.to_string().as_str())?;
        Ok(())
    }

    fn zeromq_version(&mut self, value: &mut Variant) -> AddinResult {
        let version = zmq::version();
        value.set_str1c(format!("{}.{}.{}", version.0, version.1, version.2).as_str())?;
        Ok(())
    }

    fn addin_version(&mut self, value: &mut Variant) -> AddinResult {
        value.set_str1c(env!("CARGO_PKG_VERSION"))?;
        Ok(())
    }
}

impl SimpleAddin for AddinInfo {
    fn name() -> &'static [u16] {
        name!("ZeroMQ.Info")
    }

    fn properties() -> &'static [PropInfo<Self>] {
        &[
            PropInfo {
                name: name!("Instance"),
                getter: Some(Self::instance),
                setter: None,
            },
            PropInfo {
                name: name!("VersionZeroMQ"),
                getter: Some(Self::zeromq_version),
                setter: None,
            },
            PropInfo {
                name: name!("VersionAddin"),
                getter: Some(Self::addin_version),
                setter: None,
            },
        ]
    }
}
