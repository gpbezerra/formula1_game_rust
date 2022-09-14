// [ ] - Exemplos de código que dá problema com transferência
// [ ] - Exemplos de código que dá problema com empréstimo borrow
// [x] - Implementar uma trait com função. Exemplo: display
// [ ] - Criar uma trait (e Implementar)
// [ ] - Usar as enum Option<T> e Result<T;E>
// [ ] - Criar uma enum (valorada)
// [x] - Implementar struct com encapsulamento 
// [x] - Possível enum

mod race;
mod racer;

use race::Race;

fn main() {
    let mut race = Race::new("default_race.json");
    race.run();
}
