// 1 - Exemplos de código que dá problema com transferência
// 2 - Exemplos de código que dá problema com empréstimo borrow
// 3 - Implementar uma trait com função. Exemplo: display
// 4 - Criar uma trait (e Implementar)
// 5 - Usar as enum Option<T> e Result<T;E>
// 6 - Criar uma enum (valorada)
// 7 - Implementar struct com encapsulamento 
// 8 - Possível enum

use std::fs;
// use std::cell::RefCell;
use std::cmp::Ordering;
use serde::{Serialize, Deserialize};
use rand::seq::SliceRandom;
use rand::Rng;

#[derive(PartialEq, PartialOrd, Serialize, Deserialize, Clone, Copy, Debug)]
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

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Racer {
    name: String,
    skill: f32,
    tire_type: TireTypes,
    tire_condition: f32,
    overtake: bool, // if true, the driver can overtake the next driver 
}

#[allow(dead_code)]
impl Racer {
    fn new(name: &str, skill: f32, tire_type: TireTypes, tire_condition: f32, overtake: bool) -> Self {
        Self { name: name.to_string(), skill, tire_type, tire_condition, overtake}
    }

    fn overtake(&self, target: &Self) -> bool {
        let mut rng = rand::thread_rng();
        let roll: f32 = rng.gen_range(0.0..1.0);

        let tire_coef = match (self.tire_type, target.tire_type) {
            (TireTypes::Soft, TireTypes::Soft) => 0.5,
            (TireTypes::Soft, TireTypes::Medium) => 0.7,
            (TireTypes::Soft, TireTypes::Hard) => 0.9,
            (TireTypes::Medium, TireTypes::Soft) => 0.3,
            (TireTypes::Medium, TireTypes::Medium) => 0.5,
            (TireTypes::Medium, TireTypes::Hard) => 0.6,
            (TireTypes::Hard, TireTypes::Soft) => 0.5,
            (TireTypes::Hard, TireTypes::Medium) => 0.4,
            (TireTypes::Hard, TireTypes::Hard) => 0.1,
        };

        let limit = self.tire_condition * (0.5 + tire_coef) * (0.2 * self.skill).sqrt() / (5.0 * target.skill - 0.05).sqrt();
        // DEBUG
        // println!("limit: {}, roll: {}, result: {}", limit, roll, roll <= limit);
        roll <= limit
    }

    fn degrade_tire(&mut self) {
        match self.tire_type {
            TireTypes::Hard => self.tire_condition -= 0.03,
            TireTypes::Medium => self.tire_condition -= 0.05,
            TireTypes::Soft => self.tire_condition -= 0.07,
            // TODO: check for nonzero values
        }
    }

    fn switch_tire(&mut self, new_tire: TireTypes) {
        self.tire_type = new_tire;
        self.tire_condition = 1.0;
    }

    fn pit_stop(&mut self) -> bool {
        let mut rng = rand::thread_rng();
        let roll: f32 = rng.gen_range(0.0..1.0);
        
        if &self.tire_condition >= &0.7 && &self.tire_condition < &0.8 {
            if roll > 0.001 {
                println!("change tire");
                self.switch_tire(self.tire_type);
            }
        } 
        true 
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
        let lhs = (self.skill * 10.0) as u8;
        let rhs = (other.skill * 10.0) as u8;
        lhs.cmp(&rhs)
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

        let race = match race.grid_setup {
            GridSetup::AsIs => race,
            GridSetup::Randomize => { race.positions.shuffle(&mut rng); race },
            GridSetup::LowestSkillFirst => { race.positions.sort(); race },
            GridSetup::HighestSkillFirst => { race.positions.sort(); race.positions.reverse(); race } ,
        };
        race
    }
    
    // [Thalles]: This one also smells like smart pointers; it's supposed to alter the overtake
    // attribute in each element inside race.positions according to their position (i.e., the first
    // racer cannot attempt to overtake, while everybody else can at first).
    //  fn check_overtake(&mut self) {
    //      for (index, racer) in self.positions.iter().enumerate() {
    //          if racer.overtake && index == 0 { self.positions[index].overtake = false; }
    //          else if !racer.overtake && index != 0 { self.positions[index].overtake = true; }
    //      }
    //  }
        
    // [Thalles]: I feel this is sort of a cop-out, but I currently don't know how to implement
    // this without simply using clone. Perhaps there's a way to use smart pointers here?
    fn switch_racers(&mut self, a: usize, b: usize) {
        let temp = &self.positions[a].clone();
        self.positions[a] = self.positions[b].clone();
        self.positions[b] = temp.clone();
    }

    fn next_lap(&mut self) {
        self.number_of_laps -= 1;
        for (index, racer) in self.positions.clone().iter().rev().enumerate() {
            racer.pit_stop();
            if racer.overtake {
                if racer.overtake(&self.positions[index]) && index+1 < self.positions.len() {
                    let _ = &mut self.switch_racers(index, index+1); 
                }
            }
        }
    }
}

fn main() {
    let mut race = Race::new("default_race.json");
    println!("The race in {} has begun!", race.track_name);
    for _ in 0..race.number_of_laps {
        race.next_lap();
    }
    for (index, racer) in race.positions.iter().enumerate() {
        println!("{:?}. {:?}", index+1, racer.name);
    }
    println!("The race has ended! The winner was {}!", race.positions[0].name);
}
