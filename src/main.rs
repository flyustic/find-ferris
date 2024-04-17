use rand::Rng;
use std::io;
use std::cmp::Ordering;
use ferris_says::say;

fn main() {
    loop {
        println!("Начать игру: да/нет");
        let mut start = String::new();
        io::stdin()
            .read_line(&mut start)
            .expect("Failed to read line");

        if start.trim().to_lowercase() == "да" {
            game();
        } else { break; }
    }
    println!("До встречи!");
}

fn game() {
    let secret: u32 = rand::thread_rng().gen_range(1..=100);
    println!(
        "Привет! Я краб Феррис, я спрятался в лесу из 100 деревьев \r\n попробуй угадать за каким деревом я прячусь\n\n"
    );
    let mut cnt = 0u32;

    loop {
        println!("Введи число: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Вводить можно только числа!");
                continue;
            }
        };

        match secret.cmp(&guess) {
            Ordering::Equal => {
                let out = "Ты меня нашел! А вот и я";
                let width = out.len();
                let mut writer = io::BufWriter::new(io::stdout());
                say(out, width, &mut writer).unwrap();
                break;
            }
            Ordering::Less => println!("Не угадал :(, загаданное число меньше"),
            Ordering::Greater => println!("Не угадал :(, загаданное число больше"),
        }
        cnt += 1;
    }
    println!("Сделано попыток: {cnt}");
}
