// 1 - Exemplos de código que dá problema com transferência
// 2 - Exemplos de código que dá problema com empréstimo borrow
// 3 - Implementar uma trait com função. Exemplo: display
// 4 - Criar uma trait (e Implementar)
// 5 - Usar as enum Option<T> e Result<T;E>
// 6 - Criar uma enum (valorada)
// 7 - Implementar struct com encapsulamento 
// 8 - Possível enum

use std::fs;
use std::cell::RefCell;
use std::cmp::Ordering;
use serde::{Serialize, Deserialize};
use rand::seq::SliceRandom;
use rand::Rng;

#[derive(PartialEq, PartialOrd, Serialize, Deserialize, Debug)]
enum TireTypes {
    Soft,
    Medium,
    Hard,
}

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
struct Racer {
    name: String,
    skill: u8,
    tire_type: TireTypes,
    tire_wear: u8,
    overtake: bool, // if true, the driver can overtake the next driver 
}

#[allow(dead_code)]
impl Racer {
    fn new(name: &str, skill: u8, tire_type: TireTypes, tire_wear: u8, overtake: bool) -> Self {
        Self { name: name.to_string(), skill, tire_type, tire_wear, overtake}
    }

    fn overtake(&self, target: &Self) -> bool {
        let mut rng = rand::thread_rng();
        let result: u8 = rng.gen();
        result <= 127 
    }

    fn degrade_tire(&mut self) {
        match self.tire_type {
            TireTypes::Hard => self.tire_wear += 3,
            TireTypes::Medium => self.tire_wear += 5,
            TireTypes::Soft => self.tire_wear += 7,
        }
    }

    fn switch_tire(&mut self, new_tire: TireTypes) {
        self.tire_type = new_tire;
        self.tire_wear = 0;
    }
}

impl PartialEq for Racer {
    fn eq(&self, other: &Self) -> bool {
        self.skill == other.skill
    }
}

impl Eq for Racer {}

impl PartialOrd for Racer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.skill.partial_cmp(&other.skill)
    }
}

impl Ord for Racer {
    fn cmp(&self, other: &Self) -> Ordering {
        self.skill.cmp(&other.skill)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Race {
    track_name: String,
    race_type: RaceType,
    number_of_laps: u8,
    positions: Vec<Racer>,
    grid_setup: GridSetup,
}

impl Race {
    fn new(race_file: &str) -> Race {
        let data = fs::read_to_string(race_file).expect("Failed to load file");
        let mut rng = rand::thread_rng();
        let mut race: Race = serde_json::from_str(&data).expect("Failed to read JSON data");

        let mut race = match race.grid_setup {
            GridSetup::AsIs => race,
            GridSetup::Randomize => { race.positions.shuffle(&mut rng); race },
            GridSetup::LowestSkillFirst => { race.positions.sort(); race },
            GridSetup::HighestSkillFirst => { race.positions.sort(); race.positions.reverse(); race } ,
        };

        // for (index, racer) in race.positions.iter().enumerate() {
        //     if racer.overtake && index == 0 { race.positions[index].overtake = false; }
        //     else if !racer.overtake && index != 0 { race.positions[index].overtake = true; }
        // }
        race
    }

    fn next_lap(&mut self) {
        self.number_of_laps -= 1;
        for (index, racer) in self.positions.iter().rev().enumerate() {
            // println!("{:?} _ {:?}", self.positions.len() - index, racer);
            // println!("{:?} _ {:?}", index, racer);
            if racer.overtake {
                if racer.overtake(&self.positions[index]) {
                    println!("Overtaking!");
                }
            }
        }
    }
}

fn main() {
    let mut race = Race::new("default_race.json");
    race.next_lap();
}
