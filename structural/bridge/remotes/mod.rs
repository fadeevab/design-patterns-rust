mod advanced;
mod basic;

pub use advanced::AdvancedRemove;
pub use basic::BasicRemote;

use crate::device::Device;

pub trait HasMutableDevice<D: Device> {
    fn device(&mut self) -> &mut D;
}

pub trait Remote<D: Device>: HasMutableDevice<D> {
    fn power(&mut self) {
        println!("Remote: power toggle");
        if self.device().is_enabled() {
            self.device().disable();
        } else {
            self.device().enable();
        }
    }

    fn volume_down(&mut self) {
        println!("Remote: volume down");
        let volume = self.device().volume();
        self.device().set_volume(volume - 10);
    }

    fn volume_up(&mut self) {
        println!("Remote: volume up");
        let volume = self.device().volume();
        self.device().set_volume(volume + 10);
    }

    fn channel_down(&mut self) {
        println!("Remote: channel down");
        let channel = self.device().channel();
        self.device().set_channel(channel - 1);
    }

    fn channel_up(&mut self) {
        println!("Remote: channel up");
        let channel = self.device().channel();
        self.device().set_channel(channel + 1);
    }
}
