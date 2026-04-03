use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guessing Game");
    let secret_number = rand::rng().random_range(1..=100);
    loop {
        println!("Digite um numero: ");
        
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Erro ao ler a linha informada.");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(__) => continue,
        };
        println!("Numero digitado: {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("O numero secreto e maior."),
            Ordering::Greater => println!("O numero secreto e menor."),
            Ordering::Equal => {
                println!("Acertou!");
                break;
            }
        }
    }
}
