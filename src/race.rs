use std::fs;
use rand::seq::SliceRandom;
use serde::{Serialize, Deserialize};

use crate::racer::Racer;

// Although we're only doing F1, in case we added other types of race, they could have different
// chances for random accidents
// Embora estejamos fazendo apenas F1, caso quiséssemos adicionar outros tipos de corrida, poderiam
// ter chances diferentes para acidentes aleatórios
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
                       // afeta a degradação dos pneus a cada volta;
    race_type: RaceType,
    number_of_laps: u8,
    grid_setup: GridSetup,
    safety_car: u8,
    pit_stop_threshold: f32, // tire_degradation value at which pit stop checks begin
                             // valor de tire_degradation no qual as checagens de pit stop começam
    positions: Vec<Racer>,
}

impl Race {
    pub fn new(race_file: &str) -> Race {
        let data = fs::read_to_string(race_file).expect("Failed to load file");
        let mut rng = rand::thread_rng();
        let mut race: Race = serde_json::from_str(&data).expect("Failed to read JSON data");

        // Reorganize the racers according to the grid setup
        // Reorganizaz os pilotos de acordo com a configuração de grid
        match race.grid_setup {
            GridSetup::AsIs => {},
            GridSetup::Randomize => { race.positions.shuffle(&mut rng); race.check_overtake_flag(); },
            GridSetup::LowestSkillFirst => { race.positions.sort(); race.check_overtake_flag(); },
            GridSetup::HighestSkillFirst => { race.positions.sort(); race.positions.reverse(); race.check_overtake_flag(); },
        };
        race
    }
    
    fn check_overtake_flag(&mut self) {
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
        let mut accident_has_occurred = false;

        for index in (0..self.positions.len()).rev() {
            // If a safety car is on track, the pitstop penalty is slightly lower
            // Se um safety car estiver na pista, a penalização do pitsto é ligeiramente menor
            let pitstop_penalty = if self.safety_car == 0 { 3 } else { 2 }; 

            if !self.positions[index].disabled {

                // Degrade tire condition
                // Degradar a condição dos pneus
                self.positions[index].degrade_tire(self.track_length);

                // Check for random (solo) accidents
                // Verifica se ocorre acidentes aleatórios (solo)
                if self.positions[index].accident() {
                    println!("[ACCIDENT] {} has suffered an accident and is out of the race!", self.positions[index].name);
                    self.positions[index].disabled = true;
                    let (_, behind) = self.positions.split_at_mut(index);
                    behind.rotate_left(1);
                    accident_has_occurred = true;
                }

                // Check if the racer will make a pit stop
                // Verifica se o piloto irá fazer um pit stop
                if self.positions[index].tire_condition <= self.pit_stop_threshold && self.positions[index].pit_stop() { 
                    self.positions[index].overtake = false;
                    println!("[PITSTOP] {} has made a pit-stop", self.positions[index].name);

                    // If the pilot does go for a pit stop, they will lose some positions
                    // Se o piloto fizer um pit stop, ele perderá posições
                    let (_, behind) = self.positions.split_at_mut(index);
                    if behind.len() <= pitstop_penalty {
                        behind.rotate_left(1);
                    }
                    else {
                        let (middle, _) = behind.split_at_mut(pitstop_penalty+1);
                        middle.rotate_left(1);
                    }
                }
            }
        }

        // If a safety car is on the track, overtaking is prohibited
        // Se um safety car estiver na pista, é proibido ultrapassar
        if self.safety_car == 0 {
            for index in (0..self.positions.len()).rev() {

                // Check for overtaking the next racer
                // Verifica ultrapassagem do piloto à frente
                if self.positions[index].overtake
                && index > 0
                && !self.positions[index].disabled
                && self.positions[index].overtake(&self.positions[index-1]) {
                    println!("[OVERTAKE] {} has overtaken {}", self.positions[index].name, self.positions[index-1].name);
                    let _ = &mut self.switch_racers(index, index-1); 
                }
            }
        }

        // If there is a safety car on track, decrease counter for every lap completed
        // Se houver um safety car na pista, reduzir o contador a cada volta concluída
        self.safety_car -= if self.safety_car > 0 { 1 } else { 0 };

        // Decrease the lap counter
        // Reduzir o contador de voltas
        self.number_of_laps -= 1;

        // Recheck overtake flags
        // Verificar flags de ultrapassagem
        self.check_overtake_flag();

        // If an accident happened on this lap, next lap will have a safety car
        // Se ocorreu um acidente nesta volta, haverá um safety car na próxima
        if accident_has_occurred {
            self.safety_car += 2;
            println!("[SAFETY CAR] A safety car will be on the track for the next two laps.");
        }
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
        println!("[FINAL RESULTS]");
        println!("The race has ended! The winner was {}!", self.positions[0].name);
        for index in 1..self.positions.len() {
            println!("{}. {}", index+1, self.positions[index].name);
        }
    }
}
