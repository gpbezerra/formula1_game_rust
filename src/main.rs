// [x] - Exemplos de código que dá problema com transferência
// [x] - Exemplos de código que dá problema com empréstimo borrow (tecnicamente se não
// utilizássemos RefCell teria um ótimo exemplo disso
// [x] - Implementar uma trait com função. Exemplo: display
// [ ] - Criar uma trait (e Implementar)
// [ ] - Usar as enum Option<T> e Result<T;E>
// [0] - Criar uma enum (valorada)
// [x] - Implementar struct com encapsulamento 
// [x] - Possível enum

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
