use std::cmp::Ordering;
use serde::{Serialize, Deserialize};
use rand::Rng;

#[derive(PartialEq, PartialOrd, Serialize, Deserialize, Clone, Copy, Debug)]
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
}

impl Racer {
    #[allow(dead_code)]
    pub fn new(name: &str, skill: f32, tire_type: TireTypes, tire_condition: f32, overtake: bool) -> Self {
        Self { name: name.to_string(), skill, tire_type, tire_condition, overtake}
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

    pub fn pit_stop(&mut self, new_tire: Option<TireTypes>) -> bool {
        let mut rng = rand::thread_rng();
        let roll: f32 = rng.gen_range(0.0..1.0);

        if roll <= (1.0 - 1.5 * self.tire_condition * self.tire_condition) {
            if !new_tire.is_none() { self.tire_type = new_tire.unwrap(); } 
            self.tire_condition = 1.0;
            true
        }
        else { false }
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
