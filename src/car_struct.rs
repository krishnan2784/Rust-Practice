use crate::age_struct::Age;
use crate::transmission_struct::Transmission;

#[derive(PartialEq, Debug)]
pub struct Car {
    pub color: String,
    pub motor: Transmission,
    pub roof: bool,
    pub age: (Age, u32)

}

pub fn car_factory(color: String, motor: Transmission, roof: bool, miles: u32) -> Car {

    let car = // Create a new "Car" instance as requested
        // - Bind first three fields to values of input arguments
        // - Bind "age" to tuple returned from car_quality(miles)
        Car {
            color,
            motor,
            roof,
            age: car_quality(miles)
        };

    if car.age.0 == Age::Used {
        if roof {
            println!("Preparing a used car: {:?}, {}, {}, {} miles", car.motor, car.color, get_roof_style(car.roof), car.age.1);
        } else {
            println!("Preparing a used car: {:?}, {}, {}, {} miles", car.motor, car.color,get_roof_style(car.roof), car.age.1);
        }
    } else {
        if roof {
            println!("Building a new car: {:?}, {}, {}, {} miles", car.motor, car.color,get_roof_style(car.roof), car.age.1);
        } else {
            println!("Building a new car: {:?}, {}, {}, {} miles", car.motor, car.color,get_roof_style(car.roof), car.age.1);
        }
    }

    return car;
}
fn car_quality (miles: u32) -> (Age, u32) {

    if miles > 0{
        return (Age::Used,miles);
    }
    return (Age::New, miles)
}

fn get_roof_style(roof:bool) -> String {
    return if roof {
        "Hard top"
    } else {
        "Convertible"
    }.to_string()
}

