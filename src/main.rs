use std::fs::File;
use std::io::Write;
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

    let content = fs::read_to_string(&names_path).unwrap_or_default();

    let mut names: Vec<String> = content
        .lines()
        .filter(|line| !line.trim().is_empty()) // Filter out empty lines or lines with only whitespace
        .map(|line| line.to_string())
        .collect();

    let file_path = &names_path.to_str().expect("Failed to convert Path to str");

    print!("\x1B[2J\x1B[1;1H");
    println!("Original sequence:");
    loop_over_aray(&names);
    print_instructions();
    let mut input_string = String::new();

    while input_string.trim() != "x" {
        input_string.clear();
        io::stdin().read_line(&mut input_string).unwrap();

        // shuffle
        if input_string.trim() == "s" {
            shuffle_names(&mut names);
            print_instructions();
        }

        if input_string.trim() == "c" {
            choose_name(&mut names, 50, false);
            choose_name(&mut names, 80, false);
            choose_name(&mut names, 100, false);
            choose_name(&mut names, 250, true);
        }

        if input_string.trim().starts_with("a") {
            let new_name = input_string.trim_start_matches("a").trim().to_string();

            push_name_to_array(&mut names, new_name);
            save_to_file(&names, file_path).unwrap();
            print!("\x1B[2J\x1B[1;1H");
            println!("Current Names:");
            loop_over_aray(&names);
            print_instructions();
        }

        if input_string.trim().starts_with("d") {
            let index: String = input_string.trim_start_matches("d").trim().to_string();
            let parsed_index: usize;

            match index.parse::<usize>() {
                Ok(number) => {
                    println!("Parsed number: {}", number);
                    parsed_index = number;
                }
                Err(e) => {
                    eprintln!("Error parsing number: {}", e);
                    print!("\x1B[2J\x1B[1;1H");
                    loop_over_aray(&names);
                    println!("To remove name, <number> must be between 0 and {}", names.len());
                    print_instructions();
                    continue;
                }
            }

            if parsed_index > names.len() {
                print!("\x1B[2J\x1B[1;1H");
                println!("Invalid Number");
                loop_over_aray(&names);
                println!("To remove name, <number> must be between 0 and {}", names.len());
                print_instructions();
                continue;
            }

            remove_index(&mut names, parsed_index);
            save_to_file(&names, file_path).unwrap();
            print!("\x1B[2J\x1B[1;1H");
            println!("Current Names:");
            loop_over_aray(&names);
            print_instructions();
        }
    }

    println!("Goodbye!")
}

fn shuffle_names(names: &mut Vec<String>) {
    // This will clear the screen and put the cursor at first row & first col of the screen.
    print!("\x1B[2J\x1B[1;1H");

    for i in 0..names.len() {
        let mut rng = rand::thread_rng();
        let range_start: usize = 0;
        let range_end: usize = names.len();

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
            print_instructions();
            return;
        }

        thread::sleep(Duration::from_millis(time_ms));
    }
}

fn print_instructions() {
    println!(
        "Type s to shuffle, x to quit, c to choose, a <new_name> to add name, d <number> to delete name."
    );
}

fn loop_over_aray(names: &Vec<String>) {
    for (index, name) in names.iter().enumerate() {
        println!("{}. {}", index + 1, name);
    }
}

fn push_name_to_array(names: &mut Vec<String>, new_name: String) {
    names.push(new_name)
}

fn remove_index(names: &mut Vec<String>, index: usize) {
    names.remove(index - 1);
}

fn save_to_file(names: &Vec<String>, file_path: &str) -> std::io::Result<()> {
    let mut file = File::create(file_path)?;

    for name in names {
        writeln!(file, "{}", name)?;
    }

    Ok(())
}