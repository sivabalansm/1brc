use core::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

struct City {
    min : f32,
    max : f32,
    total : f64,
    count : u32,
}

impl City {
    fn new(val: f32) -> Self {
        Self { min: val, max: val, total: val as f64, count: 1 }
    }

    pub fn mean(&self) -> f64 {
        self.total / self.count as f64
    }

    pub fn add(&mut self, val : f32) {
        if val < self.min {
            self.min = val;
        } else if val > self.max {
            self.max = val;
        }
        self.total += val as f64;
        self.count += 1;
    }

}

impl fmt::Display for City {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.1}/{:.1}/{:.1}", self.min, self.mean(), self.max)
    }
}


fn main() {
    let mut city_map = HashMap::<String, City>::new();

    let file = File::open("../data/measurements.txt").unwrap();
    let lines = BufReader::new(file).lines();

    for line in lines {
        let line = line.unwrap();

        let (city, temp) = line.split_once(';').unwrap();
        // let city = line_iter.next().unwrap();
        let temp : f32 = temp.parse().unwrap();

        city_map.entry(city.into())
            .and_modify(| city_obj | { city_obj.add(temp); })
            .or_insert(City::new(temp));

    }

    let mut sorted : Vec<_> = city_map.iter().collect();
    sorted.sort_unstable_by_key(| a | a.0);

    print!("{{");
    for (city_name, city_obj) in sorted[..sorted.len() - 1].iter() {
        print!("{city_name}={city_obj}, ");
    }
    let (last_city, last_city_obj) = sorted.last().unwrap();
    print!("{last_city}={last_city_obj}}}")
}
