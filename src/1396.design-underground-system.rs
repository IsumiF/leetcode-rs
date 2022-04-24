/*
 * @lc app=leetcode id=1396 lang=rust
 *
 * [1396] Design Underground System
 */

// @lc code=start
use std::collections::HashMap;

struct UndergroundSystem {
    station_stats: HashMap<(String, String), Average>,
    current_passengers: HashMap<i32, (String, i32)>,
}

struct Average {
    total: i32,
    count: usize,
}

impl Average {
    fn new() -> Self {
        Self { total: 0, count: 0 }
    }

    fn average(&self) -> f64 {
        (self.total as f64) / (self.count as f64)
    }

    fn add(&mut self, num: i32) {
        self.total += num;
        self.count += 1;
    }
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl UndergroundSystem {
    fn new() -> Self {
        Self {
            station_stats: HashMap::new(),
            current_passengers: HashMap::new(),
        }
    }

    fn check_in(&mut self, id: i32, station_name: String, t: i32) {
        self.current_passengers.insert(id, (station_name, t));
    }

    fn check_out(&mut self, id: i32, station_name: String, t: i32) {
        let (enter_station, enter_time) = self.current_passengers.remove(&id).unwrap();
        let key = (enter_station, station_name);
        let mut average = self.station_stats.remove(&key).unwrap_or(Average::new());
        average.add(t - enter_time);
        self.station_stats.insert(key, average);
    }

    fn get_average_time(&self, start_station: String, end_station: String) -> f64 {
        self.station_stats
            .get(&(start_station, end_station))
            .unwrap()
            .average()
    }
}

/*
 * Your UndergroundSystem object will be instantiated and called as such:
 * let obj = UndergroundSystem::new();
 * obj.check_in(id, stationName, t);
 * obj.check_out(id, stationName, t);
 * let ret_3: f64 = obj.get_average_time(startStation, endStation);
 */
// @lc code=end
