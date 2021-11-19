// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    get_cars(speed) * get_success_rate(speed)
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}


fn get_cars(speed: u8) -> f64 {
    f64::from(speed) * 221.0
}

fn get_success_rate(speed: u8) -> f64 {
    if speed < 5 {
        1.0
    } else if speed < 9 {
        0.9
    } else {
        0.77
    }    
}