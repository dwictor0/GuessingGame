use std::io;
use rand::Rng;

fn main() {
    println!("Guessing Game");
    let secret_number = rand::rng().random_range(1..=100);
    println!("Digite um numero: ");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Erro ao ler a linha informada.");

    println!("Numero digitado: {guess}")
}
