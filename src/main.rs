use std::{collections::HashMap, io};

#[derive(Debug)]
enum Commands {
    MainC(MainCommands),
    LocalizationC(LocalizationCommands),
    Names(String)
}

#[derive(Debug)]
enum MainCommands {
    Add,
    Study,
}

#[derive(Debug)]
enum LocalizationCommands {
    Folder,
    Card,
}

enum CardDificulty {
    Easy,
    Normal,
    Hard,
}

#[derive(Debug)]
enum Add {
    Folder(Folder),
    Card(Card)
}


#[derive(Debug)]
struct CommandPatern {
    main_command: MainCommands,
    localization_command: Option<LocalizationCommands>,
    name: Option<String>,
    back: Option<String>
    
}

#[derive(Debug)]
struct Folder {
    name: String,
    cards: Option<Vec<Card>>,
    size: Option<isize> 
}

#[derive(Debug)]
struct Card {
    front: String,
    back: String,
    // statistics: CardStatistics,
}



// struct CardStatistics {
//     dificulty: CardDificulty,
//     available: bool,
//     time: i32,
// }


fn user_input_select(input: String) -> HashMap<String, Add> {
    let user_keywords: Vec<&str> = input.trim().split_whitespace().collect();

    if user_keywords.len() < 4 {
        println!("Please enter at least 4 words: <ADD|STUDY> <FOLDER|CARD> <name> <back>");
        return HashMap::new();
    }

    let user_command = CommandPatern {
        main_command: match_main_c(user_keywords[0]),
        localization_command: Some(match_location_c(user_keywords[1])),
        name: Some(match_name(Some(user_keywords[2]))),
        back: Some(user_keywords[3].to_string()),
    };

    add(user_command)
}

fn match_main_c(x: &str) -> MainCommands {
    match x {
        "ADD" => MainCommands::Add,
        "STUDY" => MainCommands::Study,
        _ => MainCommands::Study
    }
}

fn match_location_c(x: &str) -> LocalizationCommands {
    match x {
        "FOLDER" => LocalizationCommands::Folder,
        "CARD" => LocalizationCommands::Card,
        // "\"\"" => println!("saa"),
         _ => LocalizationCommands::Card
    }
}

fn match_name(x: Option<&str>) -> String {
    if let Some(i) = x {
        return String::from(i).to_lowercase();
    } else {
        String::from("new folder")
    }
}

fn ask_input() -> String {
    let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).
            expect("sla");
// 
    user_input.to_uppercase()
}

fn add(command_user: CommandPatern) -> HashMap<String, Add> {
    let mut hash_map = HashMap::new();

    match command_user.localization_command.unwrap() {
        LocalizationCommands::Folder => hash_map.insert(
            command_user.name.clone().unwrap(),
            Add::Folder(Folder { name: command_user.name.unwrap(), cards: None, size: None })),
        LocalizationCommands::Card => hash_map.insert(
            command_user.name.clone().unwrap(),
            Add::Card(Card { front: command_user.name.unwrap(), back: command_user.back.unwrap() })),
    };

    hash_map


}



fn main() {
    println!("Enter command: <ADD|STUDY> <FOLDER|CARD> <name> <back>");
    let hash_map = user_input_select(ask_input());
    println!("{:?}", hash_map);
}