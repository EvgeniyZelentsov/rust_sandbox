use crate::cosmos::Coordinate;
use rand::Rng;

mod cosmos;

fn main() {

    let c = get_random_coordinate();

    println!("Coordinate is:{c}");
}

fn get_random_coordinate() -> Coordinate{
    let mut random_generator = rand::thread_rng();

    let x = random_generator.r#gen();
    let y = random_generator.r#gen();
    let z = random_generator.r#gen();

    Coordinate{x, y, z}
}
