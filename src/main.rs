use crate::cosmos::Coordinate;
use rand::Rng;
use std::f64;

mod cosmos;

fn main() {

    let ss1 = get_random_coordinate();
    let ss2 = get_random_coordinate();

    println!("ss1:{ss1}");
    println!("ss2:{ss2}");

    let distance = get_distance(ss1, ss2);

    println!("distance: {distance}");

}

fn get_random_coordinate() -> Coordinate{
    let mut random_generator = rand::thread_rng();

    let range = -300_i32..=300_i32;

    let x = random_generator.gen_range(range.clone());
    let y = random_generator.gen_range(range.clone());
    let z = random_generator.gen_range(range.clone());

    //let x = random_generator.r#gen();
    //let y = random_generator.r#gen();
    //let z = random_generator.r#gen();

    Coordinate{x, y, z}
}

fn get_distance(p1: Coordinate, p2: Coordinate) -> i32{

    let dx = p2.x - p1.x;
    let dy = p2.y - p1.y;
    let dz = p2.z - p1.z;

    ((dx * dx) + (dy * dy) + (dz * dz)).isqrt()
}
