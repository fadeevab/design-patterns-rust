use crate::device::Device;

use super::{HasMutableDevice, Remote};

pub struct BasicRemote<D: Device> {
    device: D,
}

impl<D: Device> BasicRemote<D> {
    pub fn new(device: D) -> Self {
        Self { device }
    }
}

impl<D: Device> HasMutableDevice<D> for BasicRemote<D> {
    fn device(&mut self) -> &mut D {
        &mut self.device
    }
}

impl<D: Device> Remote<D> for BasicRemote<D> {}
