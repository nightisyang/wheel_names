use std::{ fs, io, env };
use rand::Rng;
use std::thread;
use std::time::Duration;

fn main() {
    // Get the path to the current executable
    let exe_path = env::current_exe().expect("Failed to get current executable path");

    // Get the directory containing the executable
    let exe_dir = exe_path.parent().expect("Failed to get directory of current executable");

    // Construct the path to `names.txt` in the same directory as the executable
    let names_path = exe_dir.join("names.txt");

    let content = fs::read_to_string(names_path).unwrap_or_default();

    let mut names: Vec<String> = content
        .lines()
        .map(|line| line.to_string())
        .collect();

    let array_len = &names.len();

    print!("\x1B[2J\x1B[1;1H");
    println!("Original sequence:");
    for (index, name) in names.iter().enumerate() {
        println!("{}. {}", index + 1, name);
    }

    println!("Type s to shuffle, x to quit, or c to choose");
    let mut input_string = String::new();

    while input_string.trim() != "x" {
        input_string.clear();
        io::stdin().read_line(&mut input_string).unwrap();

        // shuffle
        if input_string.trim() == "s" {
            shuffle_names(&mut names, *array_len);
            println!("Type s to shuffle, x to quit, or c to choose");
        }

        if input_string.trim() == "c" {
            choose_name(&mut names, 50, false);
            choose_name(&mut names, 80, false);
            choose_name(&mut names, 100, false);
            choose_name(&mut names, 250, true);
        }
    }

    println!("Goodbye!")
}

fn shuffle_names(names: &mut Vec<String>, array_len: usize) {
    //         // This will clear the screen and put the cursor at first row & first col of the screen.
    print!("\x1B[2J\x1B[1;1H");

    for i in 0..array_len {
        let mut rng = rand::thread_rng();
        let range_start: usize = 0;
        let range_end: usize = array_len;

        let random_number: usize = rng.gen_range(range_start..range_end);

        let temp_name = names[random_number].clone();
        names[random_number] = names[i].clone();
        names[i] = temp_name;
    }
    println!("Shuffled sequence:");

    for (index, name) in names.iter().enumerate() {
        println!("{}. {}", index + 1, name);
    }
}

fn choose_name(names: &Vec<String>, time_ms: u64, choose: bool) {
    let mut rng = rand::thread_rng();
    let chosen_index: usize = rng.gen_range(0..names.len());

    for i in 0..names.len() {
        print!("\x1B[2J\x1B[1;1H");
        println!("Choosing name...");
        for (index, name) in names.iter().enumerate() {
            if index == i {
                println!("{}. {} ⬅", index + 1, name);
            } else {
                println!("{}. {}", index + 1, name);
            }
        }

        if choose && i == chosen_index {
            println!("Congratulation {}!", names[chosen_index]);

            println!("Type s to shuffle, x to quit, or c to re-choose");
            return;
        }

        thread::sleep(Duration::from_millis(time_ms));
    }
}