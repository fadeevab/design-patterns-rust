use crate::device::Device;

use super::{HasMutableDevice, Remote};

pub struct AdvancedRemote<D: Device> {
    device: D,
}

impl<D: Device> AdvancedRemote<D> {
    pub fn new(device: D) -> Self {
        Self { device }
    }

    pub fn mute(&mut self) {
        println!("Remote: mute");
        self.device.set_volume(0);
    }
}

impl<D: Device> HasMutableDevice<D> for AdvancedRemote<D> {
    fn device(&mut self) -> &mut D {
        &mut self.device
    }
}

impl<D: Device> Remote<D> for AdvancedRemote<D> {}
