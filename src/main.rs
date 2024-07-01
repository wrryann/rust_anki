use std::io;

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

// impl MainCommands {
//     fn keyword_match(self, x: LocalizationCommands, s: String){
//         match self {
//             Self::Add => x.match_location(s),
//             Self::Study => x.match_location(s),
//         };
//     }
// }

// impl LocalizationCommands {
//     fn match_location(self){
//         match self {
//             Self::Card => Add(),
//             Self::Folder => println!("chegou até folder do localization")
//         }
//     }
// }

#[derive(Debug)]
struct CommandPatern {
    main_command: MainCommands,
    localization_command: Option<LocalizationCommands>,
    name: Option<String>
    
}

struct Card {
    front: String,
    back: String,
    statistics: CardStatistics,
}

struct CardStatistics {
    dificulty: CardDificulty,
    available: bool,
    time: i32,
}

fn user_input_select(input: String) {
    let user_keywords: Vec<&str> = input.trim().split(" ").into_iter().collect();

    let user_command = CommandPatern {
            main_command: match_main_c(user_keywords[0]), 
            localization_command: Some(match_location_c(user_keywords[1])), 
            name: Some(match_name(Some(user_keywords[2])))
    };

    println!("c.pa: {:?}", user_command);
    
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
        return String::from(i);
    } else {
        String::from("new folder")
    }
}

fn ask_input() -> String {
    let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).
            expect("sla");

    user_input.to_uppercase()
}

fn Add(command: CommandPatern){
    

}



fn main() {
    // println!("{:?}", user_input_select(ask_input()));
    // add card "name"
    println!("{:?}", user_input_select(ask_input()) );

}