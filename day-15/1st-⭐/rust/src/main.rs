use std::{str::FromStr, cmp::max, collections::HashSet};

use regex::Regex;

#[derive(Debug)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct SensorBeacon {
    sensor: Point,
    beacon: Point,
}

impl FromStr for SensorBeacon {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"Sensor at x=(?P<sensor_x>-?\d+), y=(?P<sensor_y>-?\d+): closest beacon is at x=(?P<beacon_x>-?\d+), y=(?P<beacon_y>-?\d+)").unwrap();
        let caps = re.captures(s).unwrap();
        let sensor = Point {
            x: caps["sensor_x"].parse().unwrap(),
            y: caps["sensor_y"].parse().unwrap(),
        };
        let beacon = Point {
            x: caps["beacon_x"].parse().unwrap(),
            y: caps["beacon_y"].parse().unwrap(),
        };
        Ok(SensorBeacon { sensor, beacon })
    }
}

impl SensorBeacon {
    fn no_beacon_in_row(&self, row: isize) -> Option<(isize, isize)> {
        let hor_dist = (self.sensor.x - self.beacon.x).abs();
        let ver_dist = (self.sensor.y - self.beacon.y).abs();

        let total = hor_dist * 2 + 1 + ver_dist * 2;
        let size_in_row = max(0, total - (self.sensor.y - row).abs() * 2);

        if size_in_row == 0 {
            return None;
        }

        let min_x = self.sensor.x - size_in_row / 2;
        let max_x = self.sensor.x + size_in_row / 2;

        Some((min_x, max_x))
    }
}

fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();

    let row = 2000000;

    let sensor_beacons = input
        .trim_end()
        .split("\n")
        .map(|line| line.parse::<SensorBeacon>().unwrap());

    let data = sensor_beacons.clone()
        .map(|s| s.no_beacon_in_row(row))
        .filter_map(|x| x)
        .collect::<Vec<(isize, isize)>>();

    let mut no_becon: HashSet<isize> = HashSet::new();
    for d in data {
        for x in d.0..=d.1 {
            no_becon.insert(x);
        }
    }

    for sb in sensor_beacons {
        if sb.beacon.y == row {
            no_becon.remove(&sb.beacon.x);
        }

        if sb.sensor.y == row {
            no_becon.remove(&sb.sensor.x);
        }
    }

    println!("No beacon: {}", no_becon.len());
}
