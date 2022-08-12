mod car;
mod car_manual;

use crate::components::{CarType, Engine, GpsNavigator, Transmission};

/// Builder defines how to assemble a car.
pub trait Builder {
    type OutputType;
    fn set_car_type(&mut self, car_type: CarType);
    fn set_seats(&mut self, seats: u16);
    fn set_engine(&mut self, engine: Engine);
    fn set_transmission(&mut self, transmission: Transmission);
    fn set_gsp_navigator(&mut self, gps_navigator: GpsNavigator);
    fn build(self) -> Self::OutputType;
}

pub use car::CarBuilder;
pub use car_manual::CarManualBuilder;
