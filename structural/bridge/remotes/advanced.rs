use crate::device::Device;

use super::{HasMutableDevice, Remote};

pub struct AdvancedRemove<D: Device> {
    device: D,
}

impl<D: Device> AdvancedRemove<D> {
    pub fn new(device: D) -> Self {
        Self { device }
    }

    pub fn mute(&mut self) {
        println!("Remote: mute");
        self.device.set_volume(0);
    }
}

impl<D: Device> HasMutableDevice<D> for AdvancedRemove<D> {
    fn device(&mut self) -> &mut D {
        &mut self.device
    }
}

impl<D: Device> Remote<D> for AdvancedRemove<D> {}
