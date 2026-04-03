use std::io;

fn main() {
    println!("Guessing Game");
    println!("Digite um numero: ");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Erro ao ler a linha informada.");

    println!("Numero digitado: {guess}")
}
