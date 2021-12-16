use std::{fs::{self, create_dir_all}, path::Path, io, io::Write};
use colored::Colorize; 
use fs_extra::dir::{copy, CopyOptions, get_size};
// TODO: Add case for C:\ProgramData\.
// Chicken Invaders
// Child of light
// NFS Underground
// Peggle games
// Plants vs. Zombies
// Tom Clancy's Splinter Cell: Double Agent
// Zuma games

// TODO: What happens if two save games of the same thing are found

fn main() {
    // This text file holds all save game locations
    let save_locations_file = fs::read_to_string("save_locations.txt").unwrap();

    // Get the C:\ drive path from user input
    print!("Where is your C:\\ mount? (If you're on windows just press enter): ");
    std::io::stdout().flush().unwrap();
    let mut global_path = String::new();
    io::stdin().read_line(&mut global_path).unwrap();

    // If user didn't type anything, default to C:\
    let mut global_path = global_path.trim().to_owned();
    if global_path.is_empty(){
        global_path.push_str("C:\\");
    }

    // Gets a tuple that stores every found save game as (game_name, save_location)
    let save_vector: Vec<(String, String)> = find_save_games(save_locations_file, global_path);

    println!("\nType the number or name of the game save you'd like to copy, or type ALL to copy all the save files!");
    println!("Type 0 to quit!");

    loop{
        print!("\nInput: ");
        std::io::stdout().flush().unwrap();
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).unwrap();

        if user_input.trim() == "0"{
            break;
        }
        else if user_input.trim().to_lowercase() == "all"{
            for (game_name, full_save_path) in &save_vector{
                let target_path = format!("Saves\\{}", game_name);
                copy_save_game(game_name.to_string(), full_save_path.to_string(), target_path);
            }
        }
        else{
            let (game_name, full_save_path) = match user_input.trim().parse::<usize>(){
                Ok(n) => {
                    if save_vector.get(n - 1).is_some(){
                        save_vector.get(n - 1).unwrap().clone()
                    }
                    else{
                        println!("{}", "This number doesn't exist".red());
                        (String::new(), String::new())
                    }
                },
                Err(_) => {
                    let mut _game = (String::new(), String::new());
                    for game in &save_vector{
                        if game.0.to_lowercase() == user_input.trim().to_lowercase(){
                            _game = game.clone();
                            break;
                        }
                    }
                    if _game.0.is_empty(){
                        println!("{}", "Game not found. Are you sure the name is correct?".red());
                    }
                    _game
                }
            };

            if !game_name.is_empty(){
                let target_location = format!("Saves\\{}", game_name);

                copy_save_game(game_name, full_save_path, target_location);
            }
        }
    }
}

// A function to find every save game and store it in a hashmap
fn find_save_games(save_locations_file: String, global_path: String) -> Vec<(String, String)>{
    
    let mut save_vector: Vec<(String, String)> = Vec::new();

    let mut save_found_counter: u32 = 1;

    for user in fs::read_dir(format!("{}{}", global_path, "Users\\")).unwrap(){
        let user_name = user.as_ref().unwrap().file_name();
        let user_name = user_name.to_str().unwrap();
        // "All Users" leads to ProgramData, "Default User" is the same as "Default" IIRC
        // And for some reason Rust is convincd desktop.ini is a directory
        if user_name == "All Users" || user_name == "Default User" || user_name == "desktop.ini"{
            continue;
        }
        let full_user_path = user.unwrap().path();
        let full_user_path = full_user_path.to_str().unwrap();

        for line in save_locations_file.lines(){
            let game_name = line.split(',').next().unwrap().trim();
            let save_path = line.split(',').nth(1).unwrap().trim();
            let full_save_path = format!("{}\\{}", full_user_path, save_path);
            //let mut target_path = format!("Saves\\{}", game_name);
            
            if folder_exists(&full_save_path){

                save_vector.push((game_name.to_owned(), full_save_path.clone()));

                println!("{}. {}: {}", save_found_counter.to_string().yellow(), game_name.green(), full_save_path);

                save_found_counter += 1;

                
            }
        }
    }

    save_vector
}

fn copy_save_game(game_name: String, full_save_path: String, mut target_path: String){
    let dir_size = get_size(&full_save_path).unwrap();
    let dir_size_mb = dir_size / 1024 / 1024;
    let mut options = CopyOptions::new();
    options.overwrite = true;

    if dir_size_mb > 50{
        print!("{} is {}{}! Are you sure you want to copy it? [Y/N]: ", game_name.green(), dir_size_mb.to_string().red(), "mb".red());
        std::io::stdout().flush().unwrap();
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).unwrap();

        if user_input.trim().to_lowercase() != "y"{
            return;
        }
    }

    if !folder_exists(&target_path){
        remove_illegal_chars(&mut target_path);
        create_dir_all(&target_path).unwrap();
    }

    if dir_size_mb == 0{
        print!("Copying {} ({}kb)... ", game_name.green(), dir_size / 1024);
    }
    else{
        print!("Copying {} ({}mb)... ", game_name.green(), dir_size_mb);
    }
    
    std::io::stdout().flush().unwrap();

    copy(&full_save_path, target_path, &options).unwrap();

    println!("{}", "Done!".bright_green());
}

// Just shorthand for checking if folder exists
fn folder_exists(path: &str) -> bool{
    if Path::new(path).is_dir(){
        return true;
    }
    false
}

// Removes illegal characters from a string (for making clean directory names)
fn remove_illegal_chars(str: &mut String){
    // Got this from https://users.rust-lang.org/t/fast-removing-chars-from-string/24554/5
    str.retain(|c| !r#"#<$+%>!`&*|{}?"=:@"#.contains(c));
}