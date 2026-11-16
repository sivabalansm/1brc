use core::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

struct City {
    min : f32,
    max : f32,
    total : f32,
    count : u32,
}

impl City {
    fn new(val: f32) -> Self {
        Self { min: val, max: val, total: val, count: 1 }
    }

    pub fn mean(&self) -> f32 {
        self.total / self.count as f32
    }

    pub fn add(&mut self, val : f32) {
        if val < self.min {
            self.min = val;
        } else if val > self.max {
            self.max = val;
        }
        self.total += val;
        self.count += 1;
    }

}

impl fmt::Display for City {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.1}/{:.1}/{:.1}", self.min, self.mean(), self.max)
    }
}


fn main() {
    let mut city_map = HashMap::<String, (f32, f32, f32, u32)>::new();

    let file = File::open("../data/measurements.txt").unwrap();
    let lines = BufReader::with_capacity(1024 * 1024, file).lines();

    for line in lines {
        let line = line.unwrap();

        let (city, temp) = line.split_once(';').unwrap();
        let temp : f32 = temp.parse().unwrap();

        match city_map.get_mut(city) {
            Some(city_obj) => {
                if city_obj.0 > temp {
                    city_obj.0 = temp;
                } else if city_obj.1 < temp {
                    city_obj.1 = temp;
                }
                city_obj.2 += temp;
                city_obj.3 += 1;
            },
            None => { city_map.insert(city.into(), (temp, temp, temp, 1)); },
        }

    }

    let mut sorted : Vec<_> = city_map.iter().collect();
    sorted.sort_unstable_by_key(| a | a.0);

    print!("{{");
    for (city_name, city_obj) in sorted[..sorted.len() - 1].iter() {
        print!("{city_name}={:.1}/{:.1}/{:.1}, ", city_obj.0, city_obj.2/city_obj.3 as f32, city_obj.1);
    }
    let (last_city, city_obj) = sorted.last().unwrap();
    print!("{last_city}={:.1}/{:.1}/{:.1}, ", city_obj.0, city_obj.2/city_obj.3 as f32, city_obj.1);
}
