fn main() {
    // We have orders for three new cars!
    // We'll declare a mutable car variable and reuse it for all the cars
    
    let mut car = car_factory(String::from("Red"), Transmission::Manual, false);
    display_car(1, car);

    car = car_factory(String::from("Silver"), Transmission::Automatic, true);
    display_car(2, car);

    car = car_factory(String::from("Yellow"), Transmission::SemiAuto, false);
    display_car(3, car);
}

struct Car {
    color: String,
    transmission: Transmission,
    convertible: bool,
    mileage: u32,
}

#[derive(PartialEq, Debug)]
// Declare enum for Car transmission type
enum Transmission {
    // todo!("Fix enum definition so code compiles");
    Manual,
    SemiAuto,
    Automatic,
}

// Build a "Car" by using values from the input arguments
// - Color of car (String)
// - Transmission type (enum value)
// - Convertible (boolean, true if car is a convertible)
fn car_factory(color: String, transmission: Transmission, convertible: bool) -> Car {
    // Use the values of the input arguments
    // All new cars always have zero mileage
    Car { color: color, transmission: transmission, convertible: convertible, mileage: 0 }
}

fn display_car(number: u32, car: Car) {
    println!("Car {} = {}, {:?} transmission, convertible: {}, mileage: {}", number, car.color, car.transmission, car.convertible, car.mileage);
}
