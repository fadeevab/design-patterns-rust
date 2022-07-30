use super::Device;

#[derive(Clone)]
pub struct Radio {
    on: bool,
    volume: u8,
    channel: u16,
}

impl Default for Radio {
    fn default() -> Self {
        Self {
            on: false,
            volume: 30,
            channel: 1,
        }
    }
}

impl Device for Radio {
    fn is_enabled(&self) -> bool {
        self.on
    }

    fn enable(&mut self) {
        self.on = true;
    }

    fn disable(&mut self) {
        self.on = false;
    }

    fn volume(&self) -> u8 {
        self.volume
    }

    fn set_volume(&mut self, percent: u8) {
        self.volume = std::cmp::min(percent, 100);
    }

    fn channel(&self) -> u16 {
        self.channel
    }

    fn set_channel(&mut self, channel: u16) {
        self.channel = channel;
    }

    fn print_status(&self) {
        println!("------------------------------------");
        println!("| I'm radio.");
        println!("| I'm {}", if self.on { "enabled" } else { "disabled" });
        println!("| Current volume is {}%", self.volume);
        println!("| Current channel is {}", self.channel);
        println!("------------------------------------\n");
    }
}
