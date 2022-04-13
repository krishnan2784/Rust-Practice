use crate::car_struct::{Car, car_factory};


mod car_struct;
mod transmission_struct;
mod age_struct;


fn main() {

    // Initialize a hash map for the car orders
    // - Key: Car order number, i32
    // - Value: Car order details, Car struct
    // Corrected code: To create a hash map, use HashMap::new()
    use std::collections::HashMap;
    let mut orders: HashMap<i32, Car> = HashMap::new();

    // Declare a car as mutable "Car" struct
    let mut car: Car;

    // Start with zero miles
    let mut miles = 0;

    for order in 1..12 {

        // Call car_factory to fulfill order
        // Add order <K, V> pair to "orders" hash map
        // Call println! to show order details from the hash map
        car = car_factory(order, miles);
        orders.insert(order, car);
        println!("Car order {}: {:?}", order, orders.get(&order));

        // Reset miles for order variety
        if miles == 2100 {
            miles = 0;
        } else {
            miles = miles + 700;
        }
    }
}







