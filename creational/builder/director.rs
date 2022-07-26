use crate::{
    builders::Builder,
    components::{CarType, Engine, GpsNavigator, Transmission},
};

pub struct Director;

impl Director {
    pub fn construct_sports_car(builder: &mut impl Builder) {
        builder.set_car_type(CarType::SportsCar);
        builder.set_seats(2);
        builder.set_engine(Engine::new(3.0, 0.0));
        builder.set_transmission(Transmission::SemiAutomatic);
        builder.set_gsp_navigator(GpsNavigator::new());
    }

    pub fn construct_city_car(builder: &mut impl Builder) {
        builder.set_car_type(CarType::CityCar);
        builder.set_seats(2);
        builder.set_engine(Engine::new(1.2, 0.0));
        builder.set_transmission(Transmission::Automatic);
        builder.set_gsp_navigator(GpsNavigator::new());
    }

    pub fn construct_suv(builder: &mut impl Builder) {
        builder.set_car_type(CarType::Suv);
        builder.set_seats(4);
        builder.set_engine(Engine::new(2.5, 0.0));
        builder.set_transmission(Transmission::Manual);
        builder.set_gsp_navigator(GpsNavigator::new());
    }
}
