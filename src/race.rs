use std::fs;
use rand::seq::SliceRandom;
use serde::{Serialize, Deserialize};

use crate::racer::Racer;

// Although we're only doing F1, in case we added other types of race, they could have different
// chances for random accidents
#[derive(Serialize, Deserialize)]
enum RaceType {
    FormulaOne,
}

#[derive(Serialize, Deserialize)]
enum GridSetup {
    AsIs,
    Randomize,
    LowestSkillFirst,
    HighestSkillFirst,
}

#[derive(Serialize, Deserialize)]
pub struct Race {
    track_name: String,
    track_length: f32, // affects tire degradation per lap;
    race_type: RaceType,
    number_of_laps: u8,
    positions: Vec<Racer>,
    grid_setup: GridSetup,
    safety_car: bool,
    pit_stop_threshold: f32, // tire_degradation value at which pit stop checks begin
}

impl Race {
    pub fn new(race_file: &str) -> Race {
        let data = fs::read_to_string(race_file).expect("Failed to load file");
        let mut rng = rand::thread_rng();
        let mut race: Race = serde_json::from_str(&data).expect("Failed to read JSON data");

        let race = match race.grid_setup {
            GridSetup::AsIs => race,
            GridSetup::Randomize => { race.positions.shuffle(&mut rng); race.check_overtake_flag(false); race },
            GridSetup::LowestSkillFirst => { race.positions.sort(); race.check_overtake_flag(false); race },
            GridSetup::HighestSkillFirst => { race.positions.sort(); race.positions.reverse(); race.check_overtake_flag(false); race },
        };
        race
    }
    
    fn check_overtake_flag(&mut self, _safety_car: bool) {
        for index in 0..self.positions.len() {
            if index == 0 { self.positions[index].overtake = false; }
            else if !self.positions[index].overtake { self.positions[index].overtake = true; }
        }
    }

    fn switch_racers(&mut self, a: usize, b: usize) {
        self.positions.swap(a, b);
        self.positions[a].overtake = false;
        self.positions[b].overtake = false;
    }

    fn next_lap(&mut self) {
        for index in (0..self.positions.len()).rev() {

            // Degrade tire condition
            self.positions[index].degrade_tire(self.track_length);

            // Check if the racer will make a pit stop
            if self.positions[index].tire_condition <= self.pit_stop_threshold { 
                if self.positions[index].pit_stop(None) {
                    // If the pilot does go for a pit stop, normally they will lose 3 positions
                    let (_, subv) = self.positions.split_at_mut(index);
                    if subv.len() <= 3 {
                        subv.rotate_right(0);
                    }
                }
            }
        }

        // If a safety car is on the track, overtaking is prohibited
        if !self.safety_car {
            for index in (0..self.positions.len()).rev() {

                // Check for overtaking the next racer
                if self.positions[index].overtake {
                    if self.positions[index].overtake(&self.positions[index-1]) {
                        let _ = &mut self.switch_racers(index, index-1); 
                    }
                }
            }
        }

        // Decrease the lap counter
        self.number_of_laps -= 1;

        // Recheck overtake flags
        self.check_overtake_flag(self.safety_car);
    }

    pub fn run(&mut self) {
        println!("The race in {} has begun!", self.track_name);
        for lap in 0..self.number_of_laps {
            println!("[LAP {}]", lap+1);
            for index in 0..self.positions.len() {
                println!("{}. {}", index+1, self.positions[index].name);
            }
            self.next_lap();
        }
        println!("The race has ended! The winner was {}!", self.positions[0].name);
    }
}
