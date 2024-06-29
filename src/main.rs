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
//     fn keyword_match<T>(self, x: LocalizationCommands){
//         match self {
//             Self::Add => x.match_location(),
//             Self::Study => x.match_location(),
//         };
//     }
// }

impl LocalizationCommands {
    fn match_location(self, ){
        match self {
            Self::Card => Add(),
            Self::Folder => println!("saaaaaa")
        }
    }
}

struct CommandPatern {
    main_command: MainCommands,
    localization_command: Option<LocalizationCommands>,
    
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
    let mut keywords_matched: Vec<Commands> = Vec::new();
    let user_keywords = input.trim().split(" ").into_iter().for_each( |x| {
       match x {
            "ADD" => keywords_matched.push(Commands::MainC(MainCommands::Add)),
            "STUDY" => keywords_matched.push(Commands::MainC(MainCommands::Study)),
            "FOLDER" => keywords_matched.push(Commands::LocalizationC(LocalizationCommands::Folder)),
            "CARD" => keywords_matched.push(Commands::LocalizationC(LocalizationCommands::Card)),
            _ => println!("error")

        }  
    });

    println!("{:?}", keywords_matched);

    
}

// fn foo(){
//     match keywords_matched[0] {
//         Commands::MainC(MainCommands::Add) => return MainCommands::Add.keyword_match(if matches!(keywords_matched[1], LocalizationCommands)  { keywords_matched[1] }),
//         _ => MainCommands::Study.keyword_match(LocalizationCommands::Card)
//     }
// }

fn ask_input() -> String {
    let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).
            expect("sla");

    user_input.to_uppercase()
}

fn Add(){
    

}

fn main() {

    // println!("{:?}", user_input_select(ask_input()));
    // add card "name"
    println!("{:?}", user_input_select(ask_input()));

}
