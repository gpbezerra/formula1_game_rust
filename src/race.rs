use std::fs;
use std::cell::RefCell;
use serde::{Serialize, Deserialize};
use rand::seq::SliceRandom;

use crate::racer::Racer;

// Although we're only doing F1, in case we added other types of race, they could have different
// chances for random accidents
#[derive(Serialize, Deserialize, Debug)]
enum RaceType {
    FormulaOne,
}

#[derive(Serialize, Deserialize, Debug)]
enum GridSetup {
    AsIs,
    Randomize,
    LowestSkillFirst,
    HighestSkillFirst,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Race {
    pub track_name: String,
    track_length: f32, // affects tire degradation per lap;
    race_type: RaceType,
    pub number_of_laps: u8,
    pub positions: Vec<RefCell<Racer>>,
    grid_setup: GridSetup,
    pub safety_car: bool,
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
    
    fn check_overtake_flag(&mut self, safety_car: bool) {
        for (index, racer) in self.positions.iter().enumerate() {
            if racer.borrow().overtake && index == 0 { self.positions[index].borrow_mut().overtake = false; }
            else if !racer.borrow().overtake && index != 0 { self.positions[index].borrow_mut().overtake = true; }
        }
    }

    // [Thalles]: I feel this is sort of a cop-out, but I currently don't know how to implement
    // this without simply using clone. Perhaps there's a way to use smart pointers here?
    fn switch_racers(&mut self, a: usize, b: usize) {
        let temp = &self.positions[a].clone();
        self.positions[a] = self.positions[b].clone();
        self.positions[b] = temp.clone();
    }

    pub fn next_lap(&mut self) {
        self.number_of_laps -= 1;
        for (index, racer) in self.positions.clone().iter().rev().enumerate() {
            // Degrade tire condition
            racer.borrow_mut().degrade_tire(self.track_length);

            // Check if the racer will make a pit stop
            if racer.borrow().tire_condition <= 0.5 { 
                if racer.borrow_mut().pit_stop(racer.borrow().tire_type) { &mut self.switch_racers(index, index-3); }
            }

            // Check for overtaking the next racer
            if racer.borrow().overtake {
                if racer.borrow().overtake(&self.positions[index].borrow()) {
                    let _ = &mut self.switch_racers(index, index+1); 
                }
            }
        // Recheck overtake flags
        self.check_overtake_flag(self.safety_car);
        }
    }
}
