use crate::domains::plane_noise::PlaneNoise;

pub fn display(data: &Vec<PlaneNoise>) {
    println!("{}", serde_json::to_string(&data).unwrap())
}
