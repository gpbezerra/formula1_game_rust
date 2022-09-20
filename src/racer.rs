use std::cmp::Ordering;
use serde::{Serialize, Deserialize};
use rand::Rng;

#[derive(PartialEq, PartialOrd, Eq, Serialize, Deserialize, Clone, Copy, Debug)]
pub enum TireTypes {
    Soft,
    Medium,
    Hard,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Racer {
    pub name: String,
    skill: f32,
    pub tire_type: TireTypes,
    pub tire_condition: f32,
    pub overtake: bool, // if true, the driver can overtake the next driver 
    pub disabled: bool,
}

impl Racer {
    #[allow(dead_code)]
    pub fn new(name: &str, skill: f32, tire_type: TireTypes, tire_condition: f32, overtake: bool, disabled: bool) -> Self {
        Self { name: name.to_string(), skill, tire_type, tire_condition, overtake, disabled }
    }

    pub fn overtake(&self, target: &Self) -> bool {
        let mut rng = rand::thread_rng();
        let roll: f32 = rng.gen_range(0.0..1.0);

        let tire_coef = match (self.tire_type, target.tire_type) {
            (TireTypes::Soft, TireTypes::Soft) => 0.5,
            (TireTypes::Soft, TireTypes::Medium) => 0.7,
            (TireTypes::Soft, TireTypes::Hard) => 0.9,
            (TireTypes::Medium, TireTypes::Soft) => 0.3,
            (TireTypes::Medium, TireTypes::Medium) => 0.5,
            (TireTypes::Medium, TireTypes::Hard) => 0.6,
            (TireTypes::Hard, TireTypes::Soft) => 0.1,
            (TireTypes::Hard, TireTypes::Medium) => 0.4,
            (TireTypes::Hard, TireTypes::Hard) => 0.5,
        };

        let limit = self.tire_condition * (0.5 + tire_coef) * (0.2 * self.skill).sqrt() / (5.0 * target.skill - 0.05).sqrt();
        roll <= limit
    }

    pub fn degrade_tire(&mut self, track_length: f32) {
        let mut rng = rand::thread_rng();
        let min = match self.tire_type {
            TireTypes::Hard => 0.03,
            TireTypes::Medium => 0.05,
            TireTypes::Soft => 0.07,
        };
        let max = min+((min*track_length*track_length)/(30.0*track_length));

        self.tire_condition -= rng.gen_range(min..max);
        if self.tire_condition < 0.0 { self.tire_condition = 0.0; };
    }

    pub fn pit_stop(&mut self) -> bool {
        let mut rng = rand::thread_rng();
        let pitstop_roll: f32 = rng.gen_range(0.0..1.0);
        let tire_roll: u8 = rng.gen_range(0..255);

        if pitstop_roll <= (1.0 - 1.5 * self.tire_condition * self.tire_condition) {
            match tire_roll {
                0..=85 => { self.tire_type = TireTypes::Soft },
                86..=170 => { self.tire_type = TireTypes::Medium },
                171..=255 => { self.tire_type = TireTypes::Hard },
            }
            self.tire_condition = 1.0;
            true
        }
        else { false }
    } 

    pub fn accident(&mut self) -> bool {
        let mut rng = rand::thread_rng();
        let accident_roll: f32 = rng.gen_range(0.0..1.0);
        let limit = 0.015 - (self.skill / 10.0) - (self.tire_condition / 20.0);
        accident_roll <= limit
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
