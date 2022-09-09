// 1 - Exemplos de código que dá problema com transferência
// 2 - Exemplos de código que dá problema com empréstimo borrow
// 3 - Implementar uma trait com função. Exemplo: display
// 4 - Criar uma trait (e Implementar)
// 5 - Usar as enum Option<T> e Result<T;E>
// 6 - Criar uma enum (valorada)
// 7 - Implementar struct com encapsulamento 
// 8 - Possível enum

mod race;
mod racer;

use race::Race;

fn main() {
    let mut race = Race::new("../default_race.json");
    println!("The race in {} has begun!", race.track_name);
    for _ in 0..race.number_of_laps {
        race.next_lap();
    }
    for (index, racer) in race.positions.iter().enumerate() {
         println!("{}. {}", index+1, racer.borrow().name);
    }
    println!("The race has ended! The winner was {}!", race.positions[0].borrow().name);
}
