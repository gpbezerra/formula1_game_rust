// This is the entry point for the simulation; all this does is import the modules, load the JSON
// file and begin the race loop.
// Esse é o ponto de entrada para a simulação; tudo que esse arquivo faz é importar os módulos,
// carregar o JSON e iniciar o loop da corrida.

mod race;
mod racer;

use race::Race;

fn main() {
    let mut race = Race::new("default_race.json");
    race.run();
}
