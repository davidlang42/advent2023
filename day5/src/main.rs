use std::fs;
use std::env;
use std::str::FromStr;

struct Almanac1 {
    seeds: Vec<usize>,
    seed_to_soil: NumberMap,
    soil_to_fertilizer: NumberMap,
    fertilizer_to_water: NumberMap,
    water_to_light: NumberMap,
    light_to_temperature: NumberMap,
    temperature_to_humidity: NumberMap,
    humidity_to_location: NumberMap
}

struct Almanac2 {
    seeds: Vec<Range>,
    seed_to_soil: NumberMap,
    soil_to_fertilizer: NumberMap,
    fertilizer_to_water: NumberMap,
    water_to_light: NumberMap,
    light_to_temperature: NumberMap,
    temperature_to_humidity: NumberMap,
    humidity_to_location: NumberMap
}

#[derive(Clone)]
struct Range {
    from: usize,
    to: usize,
    offset: isize
}

struct NumberMap(Vec<(usize, usize, isize)>); // start, end, offset

impl NumberMap {
    fn get(&self, input: usize) -> usize {
        for (start, end, offset) in &self.0 {
            if input >= *start && input <= *end {
                return (input as isize + offset) as usize;
            }
        }
        input
    }

    fn get_range(&self, input: Vec<Range>) -> Vec<Range> {
        let mut results = Vec::new();
        for mut range in input {
            for (start, end, offset) in &self.0 {
                if *start <= range.from {
                    if *end >= range.to {
                        // full overlap
                        range.offset += offset;
                        break; // no more to do in this range
                    } else if *end >= range.from {
                        // overlap range start
                        results.push(Range {
                            from: range.from,
                            to: *end,
                            offset: range.offset + offset
                        });
                        range.from = *end;
                        // more to do
                    }
                } else if *start <= range.to {
                    if *end >= range.to {
                        // overlap range end
                        results.push(Range {
                            from: *start,
                            to: range.to,
                            offset: range.offset + offset
                        });
                        range.to = *start;
                        break; // no more to do in this range
                    } else if *end >= range.from {
                        // overlap middle
                        results.push(Range { // before the overlap
                            from: range.from,
                            to: *start,
                            offset: range.offset
                        });
                        results.push(Range { // the overlap
                            from: *start,
                            to: *end,
                            offset: range.offset + offset
                        });
                        range.from = *end;
                        // more to do
                    }
                }
            }
            results.push(range); // whatevers left
        }
        results
    }
}

impl Almanac1 {
    fn locations(&self) -> Vec<usize> {
        let mut locations = Vec::new();
        for seed in &self.seeds {
            let soil = self.seed_to_soil.get(*seed);
            let fert = self.soil_to_fertilizer.get(soil);
            let water = self.fertilizer_to_water.get(fert);
            let light = self.water_to_light.get(water);
            let temp = self.light_to_temperature.get(light);
            let hum = self.temperature_to_humidity.get(temp);
            let location = self.humidity_to_location.get(hum);
            locations.push(location);
        }
        locations
    }
}

impl FromStr for NumberMap {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        //humidity-to-location map:
        //50 98 2
        //52 50 48
        let mut vec = Vec::new();
        for line in text.lines().skip(1) {
            let numbers: Vec<usize> = line.split(" ").map(|s| s.parse().unwrap()).collect();
            if numbers.len() != 3 {
                return Err(format!("Expected 3 numbers, found {}", numbers.len()));
            }
            let start = numbers[1];
            let offset = numbers[0] as isize - start as isize;
            let end = start + numbers[2] - 1;
            vec.push((start, end, offset));
        }
        vec.sort_by(|a, b| b.0.cmp(&a.0));
        Ok(Self(vec))
    }
}

impl FromStr for Almanac1 {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let sections: Vec<&str> = text.split("\r\n\r\n").collect();
        if sections.len() != 8 {
            return Err(format!("Expected 8 sections, found {}", sections.len()));
        }
        let seeds = sections[0].split(": ").nth(1).unwrap().split(" ").map(|s| s.parse().unwrap()).collect();
        let seed_to_soil = sections[1].parse().unwrap();
        let soil_to_fertilizer = sections[2].parse().unwrap();
        let fertilizer_to_water = sections[3].parse().unwrap();
        let water_to_light = sections[4].parse().unwrap();
        let light_to_temperature = sections[5].parse().unwrap();
        let temperature_to_humidity = sections[6].parse().unwrap();
        let humidity_to_location = sections[7].parse().unwrap();
        Ok(Self {
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location
        })
    }
}

impl FromStr for Almanac2 {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let sections: Vec<&str> = text.split("\r\n\r\n").collect();
        if sections.len() != 8 {
            return Err(format!("Expected 8 sections, found {}", sections.len()));
        }
        let ranges: Vec<usize> = sections[0].split(": ").nth(1).unwrap().split(" ").map(|s| s.parse().unwrap()).collect();
        if ranges.len() % 2 != 0 {
            panic!("Uneven seed ranges");
        }
        let mut i = 0;
        let mut seeds = Vec::new();
        while i < ranges.len() {
            seeds.push(Range {
                from: ranges[i],
                to: ranges[i] + ranges[i + 1] - 1,
                offset: 0
            });
            i += 2;
        }
        let seed_to_soil = sections[1].parse().unwrap();
        let soil_to_fertilizer = sections[2].parse().unwrap();
        let fertilizer_to_water = sections[3].parse().unwrap();
        let water_to_light = sections[4].parse().unwrap();
        let light_to_temperature = sections[5].parse().unwrap();
        let temperature_to_humidity = sections[6].parse().unwrap();
        let humidity_to_location = sections[7].parse().unwrap();
        Ok(Self {
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location
        })
    }
}

impl Range {
    fn min(&self) -> usize {
        (self.from as isize + self.offset) as usize
    }
}

impl Almanac2 {
    fn locations(&self) -> Vec<Range> {
        let soil = self.seed_to_soil.get_range(self.seeds.clone());
        let fert = self.soil_to_fertilizer.get_range(soil);
        let water = self.fertilizer_to_water.get_range(fert);
        let light = self.water_to_light.get_range(water);
        let temp = self.light_to_temperature.get_range(light);
        let hum = self.temperature_to_humidity.get_range(temp);
        self.humidity_to_location.get_range(hum)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let almanac1: Almanac1 = text.parse().unwrap();
        let location1 = almanac1.locations().into_iter().min().unwrap();
        println!("Lowest 1: {}", location1);
        let almanac2: Almanac2 = text.parse().unwrap();
        let location2 = almanac2.locations().iter().map(|range| range.min()).min().unwrap();
        println!("Lowest 2: {}", location2);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}
