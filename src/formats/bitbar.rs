use crate::domains::plane_noise::PlaneNoise;

fn colour(current_direction: &str, next_direction: Option<&str>) -> String {
    match (current_direction, next_direction) {
        ("westerly" ,Some("westerly")) => "#00FF00",
        ("westerly" ,Some("easterly")) => "#99FF00",
        ("easterly" ,Some("easterly")) => "#FF0000",
        ("easterly" ,Some("westerly")) => "#FF9900",
        ("westerly" ,None) => "#00FF00",
        ("easterly" ,None) => "#FF0000",
        _ => "#000000"
    }.to_string()
}

pub fn display(data: &Vec<PlaneNoise>) {
    data.iter().enumerate().for_each(|(idx, d)| {
        let next = match data.get(idx + 1) {
            Some(pn) => Some(*&pn.direction.as_str()),
            None => None,
        };
        let colour = colour(&d.direction, next);
        if idx == 0 {
            println!(":airplane: | color={}", colour);
            println!("---");
        }
        println!("{} : {} | color={}", d.date_time, d.direction, colour);
    });
}
