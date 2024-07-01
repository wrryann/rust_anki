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

impl MainCommands {
    fn keyword_match(self, x: LocalizationCommands, s: String){
        match self {
            Self::Add => x.match_location(s),
            Self::Study => x.match_location(s),
        };
    }
}

impl LocalizationCommands {
    fn match_location(self, s){
        match self {
            Self::Card => Add(),
            Self::Folder => println!("chegou até folder do localization nome: {s}")
        }
    }
}

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

    // let a = CommandPatern {  };

    println!("input do usuario: {:?}", user_keywords);

    // match user_keywords[0] {
    //     "ADD" => CommandPatern{  },
    //     "STUDY" => MainCommands::Study.keyword_match(foo(user_keywords[1])),
    //     _ => println!("error")
    // }
}

fn foo<T>(x: &str) -> Commands {
    match x {
            "ADD" => Commands::MainC(MainCommands::Add),
            "STUDY" => Commands::MainC(MainCommands::Study),
            "FOLDER" => Commands::LocalizationC(LocalizationCommands::Folder),
            "CARD" => Commands::LocalizationC(LocalizationCommands::Card),
            _ => return Commands::LocalizationC(LocalizationCommands::Card)
    }
}

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
