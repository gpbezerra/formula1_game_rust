use std::fs;
use std::cell::RefCell;
use std::cmp::Ordering;
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
    pub track_name: String,
    track_length: f32, // affects tire degradation per lap;
    race_type: RaceType,
    pub number_of_laps: u8,
    pub positions: RefCell<Vec<RefCell<Racer>>>,
    grid_setup: GridSetup,
    pub safety_car: bool,
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
        self.positions[a].borrow_mut().overtake = false;
        self.positions[b].borrow_mut().overtake = false;
    }

    pub fn next_lap(&mut self) {
        for (index, racer) in self.positions.borrow_mut().iter().rev().enumerate() {

            // Degrade tire condition
            racer.borrow_mut().degrade_tire(self.track_length);

            // Check if the racer will make a pit stop
            if racer.borrow().tire_condition <= self.pit_stop_threshold { 
                if racer.borrow_mut().pit_stop(racer.borrow().tire_type) {
                    let new_position = match (index-3).cmp(&self.positions.len()) {
                        Ordering::Less => self.positions.len(),
                        Ordering::Equal => index-3,
                        Ordering::Greater => index-3
                    };
                    self.switch_racers(index, new_position);
                }
            }
        }

        if !self.safety_car {
            for (index, racer) in self.positions.clone().iter().rev().enumerate() {
                // Check for overtaking the next racer
                if racer.borrow().overtake {
                    if racer.borrow().overtake(&self.positions[index].borrow()) {
                        let _ = &mut self.switch_racers(index, index+1); 
                    }
                }
            }
        }

        // Decrease the lap counter
        self.number_of_laps -= 1;

        // Recheck overtake flags
        self.check_overtake_flag(self.safety_car);
    }
}
