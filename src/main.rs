// 1 - Exemplos de código que dá problema com transferência
// 2 - Exemplos de código que dá problema com empréstimo borrow
// 3 - Implementar uma trait com função. Exemplo: display
// 4 - Criar uma trait (e Implementar)
// 5 - Usar as enum Option<T> e Result<T;E>
// 6 - Criar uma enum (valorada)
// 7 - Implementar struct com encapsulamento 
// 8 - Possível enum

use std::fs;
use std::cmp::{Ordering};
use serde::{Serialize, Deserialize};
use rand::seq::SliceRandom;

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

impl Racer {
    fn new(name: &str, skill: u8, tire_type: TireTypes, tire_wear: u8, overtake: bool) -> Self {
        Self { name: name.to_string(), skill, tire_type, tire_wear, overtake}
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

        let ordered_race = match race.grid_setup {
            GridSetup::AsIs => race,
            GridSetup::Randomize => { race.positions.shuffle(&mut rng); race },
            GridSetup::LowestSkillFirst => { race.positions.sort(); race },
            GridSetup::HighestSkillFirst => { race.positions.sort(); race.positions.reverse(); race } ,
        };
        ordered_race
    }
}

fn main() {
    let race = Race::new("default_race.json");
    println!("{:?}", race);
    println!("{}", race.positions[0].skill < race.positions[1].skill);
}
