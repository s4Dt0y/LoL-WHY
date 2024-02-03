use std::{io, process};
use std::{thread, time::Duration};

use rand::Rng;

pub fn run() {
    let mut game_data = intro();
    main_loop(&mut game_data);
}

fn intro() -> GameData {
    let mut result: GameData = GameData {
        first_name: String::new(),
        last_name: String::new(),
        nickname: String::new(),

        lives: 3,
        level: 1,
        points: 1,
        wire_amount: 2,

        ended: false,
        msg_ended: String::new(),
    };

    println!("Welcome! You're our new EOD Specialist, right? [y/n, default=n]");
    let answer = get_input("n");

    // Useless part that I added for fun
    if answer.trim().to_lowercase() == "y".to_string() {
        println!("Great! Let's get started, shall we?");
    } else {
        println!("Uhh...then what are you doing here? Security!!!");
        process::exit(0);
    }

    loop {
        // Gets full name, and nickname.
        println!("First I need to get some info...\n");
        println!("What's your first name? [default = Toy]");
        result.first_name = get_input("Toy");

        println!("Last name? [default = Hertz]");
        result.last_name = get_input("Hertz");

        println!("Nickname? [default = sadtoy]");
        result.nickname = get_input("sadtoy");

        println!(
            "Okay, this is what I got: {} {}, also called {}",
            result.first_name, result.last_name, result.nickname
        );

        println!("Is this good? [y/n, default=y]");
        let answer = get_input("y");

        clear_screen();

        // Check whether user wants to redo input...
        if answer.trim().to_lowercase() == "y".to_string() {
            break;
        }
    }
    println!("Great!");
    result
}

fn main_loop(game_data: &mut GameData) {
    println!(
        "Well, {}, it's time for your first gig...\
    We'll start you off easy, but it'll get progressively harder. \
    Ready? I'm not waiting for an answer. You can do this! Don't let those bombs go off!",
        game_data.nickname
    );

    sleep(5);

    let mut guess: String;
    let mut msg: &str = "";
    let mut correct: bool;
    let mut secret_num: usize;
    loop {
        if msg != "" {
            println!("{msg}");
        }
        println!(
            "Level: {}\n Points: {} \n Lives: {}\n", // the double newline here is intentional
            game_data.level, game_data.points, game_data.lives
        );

        // did it end?
        if game_data.lives == 0 {
            game_data.ended = true;
            game_data.msg_ended =
                "You LOSE!!! Three bombs went off, and you didn't stop them...".to_string();
        }
        if game_data.level == 100 {
            game_data.ended = true;
            game_data.msg_ended = "You WIN!!! Glad to see you alive...".to_string();
        }

        if game_data.ended {
            break;
        }

        // Generate number
        secret_num = rand::thread_rng().gen_range(1..=2);

        println!("Cut the right wire! [1-{}]", game_data.wire_amount);
        guess = get_input("a");
        println!("You guessed: {}\n\n", guess); // The newlines are, again, intentional

        // Is it correct?
        correct = if guess == secret_num.to_string() {
            true
        } else {
            false
        };

        // Suspense building
        countdown(5);

        if correct {
            // yes!!
            msg = "Good job!";
            game_data.points += game_data.level * 3;
            game_data.level += 1;
            game_data.wire_amount += 1;
        } else {
            // no!!
            msg = "Sorry, no... Oops.";
            game_data.lives -= 1
        }
        sleep(700);
        clear_screen();
    }

    clear_screen();
    println!("{}", game_data.msg_ended);
}

fn get_input(default: &str) -> String {
    let mut result = String::new();
    io::stdin().read_line(&mut result).unwrap();
    if result == "\n" {
        return default.trim().to_string();
    }
    result.trim().to_string()
}

fn countdown(secs: u64) {
    for i in (1..=secs).rev() {
        println!("\r********* {} *********", i);
        sleep(500);
    }
}
fn clear_screen() {
    print!("{esc}c", esc = 27 as char);
}

fn sleep(millis: u64) {
    thread::sleep(Duration::from_millis(millis));
}

struct GameData {
    first_name: String,
    last_name: String,
    nickname: String,

    lives: usize,
    level: usize,
    points: usize,
    wire_amount: usize,

    ended: bool,
    msg_ended: String,
}
