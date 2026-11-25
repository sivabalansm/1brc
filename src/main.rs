use core::fmt;
use rapidhash::RapidHashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

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

fn convert_float(buf : &[u8]) -> f32 {
    let is_negative = buf[0] == 45;
    if is_negative {
        -(buf[1..buf.len() - 2]
            .iter()
            .map( | val | val - 48)
            .fold(0, | acc, x | acc * 10 + x) as f32 + ((buf[buf.len() - 1] - 48) as f32 / 10.0))
    } else {
        buf[..buf.len() - 2]
            .iter()
            .map( | val | val - 48)
            .fold(0, | acc, x | acc * 10 + x) as f32 + ((buf[buf.len() - 1] - 48) as f32 / 10.0)
    }
}

fn main() {
    let mut city_map = RapidHashMap::<String, City>::default();

    let file = File::open("../data/measurements.txt").unwrap();
    let mut bufreader = BufReader::with_capacity(1024 * 1024, file);

    let mut city = vec![];
    let mut temp = vec![];
    // let mut count : u32 = 0;
    loop {
        city.clear();
        temp.clear();

        let r1 = bufreader.read_until(b';', &mut city).unwrap();
        let r2 = bufreader.read_until(b'\n', &mut temp).unwrap();

        if r1 == 0 || r2 == 0 {
            break;
        }

        city.pop(); // remove ';'
        temp.pop(); // remove '\n'

        let city_str = unsafe { str::from_utf8_unchecked(&city) };
        let temp : f32 = convert_float(&temp);
        
        match city_map.get_mut(city_str) {
            Some(city_obj) => { city_obj.add(temp); },
            None => { city_map.insert(String::from(city_str), City::new(temp)); },
        }
    }

    let mut sorted : Vec<_> = city_map.iter().collect();
    sorted.sort_unstable_by_key(| a | a.0);

    print!("{{ ");
    for (city_name, city_obj) in sorted[..sorted.len() - 1].iter() {
        print!("{city_name}={city_obj}, ");
    }
    let (last_city, city_obj) = sorted.last().unwrap();
    print!("{last_city}={city_obj}}}");
}
