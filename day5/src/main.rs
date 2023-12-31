use std::fs;
use std::env;
use std::str::FromStr;

struct Almanac {
    seeds: Vec<usize>,
    seed_to_soil: NumberMap,
    soil_to_fertilizer: NumberMap,
    fertilizer_to_water: NumberMap,
    water_to_light: NumberMap,
    light_to_temperature: NumberMap,
    temperature_to_humidity: NumberMap,
    humidity_to_location: NumberMap
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
}

impl Almanac {
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

    fn type1(text: &str) -> Self {
        let sections: Vec<&str> = text.split("\r\n\r\n").collect();
        if sections.len() != 8 {
            panic!("Expected 8 sections, found {}", sections.len());
        }
        let seeds = sections[0].split(": ").nth(1).unwrap().split(" ").map(|s| s.parse().unwrap()).collect();
        let seed_to_soil = sections[1].parse().unwrap();
        let soil_to_fertilizer = sections[2].parse().unwrap();
        let fertilizer_to_water = sections[3].parse().unwrap();
        let water_to_light = sections[4].parse().unwrap();
        let light_to_temperature = sections[5].parse().unwrap();
        let temperature_to_humidity = sections[6].parse().unwrap();
        let humidity_to_location = sections[7].parse().unwrap();
        Self {
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location
        }
    }

    fn type2(text: &str) -> Self {
        let sections: Vec<&str> = text.split("\r\n\r\n").collect();
        if sections.len() != 8 {
            panic!("Expected 8 sections, found {}", sections.len());
        }
        let ranges: Vec<usize> = sections[0].split(": ").nth(1).unwrap().split(" ").map(|s| s.parse().unwrap()).collect();
        if ranges.len() % 2 != 0 {
            panic!("Uneven seed ranges");
        }
        let mut i = 0;
        let mut seeds = Vec::new();
        while i < ranges.len() {
            for s in ranges[i]..(ranges[i] + ranges[i+1]) {
                seeds.push(s);
            }
            i += 2;
        }
        let seed_to_soil = sections[1].parse().unwrap();
        let soil_to_fertilizer = sections[2].parse().unwrap();
        let fertilizer_to_water = sections[3].parse().unwrap();
        let water_to_light = sections[4].parse().unwrap();
        let light_to_temperature = sections[5].parse().unwrap();
        let temperature_to_humidity = sections[6].parse().unwrap();
        let humidity_to_location = sections[7].parse().unwrap();
        Self {
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location
        }
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
        Ok(Self(vec))
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let a1: Almanac = Almanac::type1(&text);
        let l1 = a1.locations();
        //println!("Locations: {:?}", l1);
        let ll1 = l1.iter().min().unwrap();
        println!("Lowest: {}", ll1);
        let a2: Almanac = Almanac::type2(&text);
        let l2 = a2.locations();
        //println!("Locations: {:?}", l2);
        let ll2 = l2.iter().min().unwrap();
        println!("Lowest: {}", ll2);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}
