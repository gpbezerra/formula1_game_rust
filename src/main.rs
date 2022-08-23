// 1 - Exemplos de código que dá problema com transferência
// 2 - Exemplos de código que dá problema com empréstimo borrow
// 3 - Implementar uma trait com função. Exemplo: display
// 4 - Criar uma trait (e Implementar)
// 5 - Usar as enum Option<T> e Result<T;E>
// 6 - Criar uma enum (valorada)
// 7 - Implementar struct com encapsulamento 
// 8 - Possível enum


// I dont know thy this complaning about debug
#[derive(Debug)]
enum TiresTypes {
    Hard,
    Medium,
    Soft
}
#[derive(Debug)]
struct RaceDriver {
    name: String,
    skill: usize, // number with range 0 - 100
    tire_type: TiresTypes,
    tire_wear: usize,
    over_taking: bool, // if == true, the driver can overtake the next driver 
}

impl RaceDriver{
    fn new(name: &str, skill: usize, tire_type: TiresTypes, tire_wear: usize, over_taking: bool) -> Self {
        Self { name: name.to_string(), skill, tire_type, tire_wear, over_taking}
    }
}

fn start_race(laps:usize) {
   // Read a json file to get race drivers data
   let mut race = Vec::new(); 
   race.push(RaceDriver::new("Verstappen", 90, TiresTypes::Hard, 0, true));
   race.push(RaceDriver::new("Hamilton", 92, TiresTypes::Soft, 0, true));
   race.push(RaceDriver::new("Leclerc", 88, TiresTypes::Medium, 0, true));
   // Function to sort the order of drivers. > skill = > grid position
   race.sort_by(|a, b| b.skill.cmp(&a.skill)); // I didn't understand very well
   // The over_taking atributte from the leader of the race = false
   race[0].over_taking = false;
   println!("{:?}",race);  
   do_over_taking(race);
}

fn do_over_taking(race: Vec<RaceDriver>) {
   
    for i in 0..race.len() {
    }
}


fn pit_stop(race: Vec<RaceDriver>) {
    for i in 0..race(len) {
        // Calculo probabilistico de ser feito um pit-stop
        
    }
    
}

fn main() {
    start_race(20)
}


// ----------- Rafael


// fn main() {
//     let mut posição = Vec::new();
//     posição.push(Corredor::novo("Maltar", 5.0));
//     posição.push(Corredor::novo("Kinder", 10.0));
//     posição.push(Corredor::novo("Macário", 8.0));
//     println!("Posição inicial: {:?}", posição);
//     troca_posição(&mut posição, 0, 2);
//     println!("Posição inicial: {:?}", posição);
// }

// #[derive(Clone, Debug)]
// struct Corredor {
//     nome: String,
//     skill: f32,
// }

// impl Corredor {
//     fn novo(nome: &str, skill: f32) -> Self {
//         Self { nome: nome.to_string(), skill }
//     }
// }




// fn troca_posição<T: Clone>(vetor: &mut Vec<T>, x: usize, y: usize) {
//     let aux: T;
//     aux = vetor[x].clone();
//     vetor[x] = vetor[y].clone();
//     vetor[y] = aux;
// }
