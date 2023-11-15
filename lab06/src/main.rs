use rusqlite::Connection;
use std::fs;
use std::process;
use strsim::levenshtein;

trait Command {
    fn get_name(&self) -> &'static str;
    fn exec(&mut self, args: &[&str]);
}

struct PingCommand {}
struct TimesCommand {
    count: u32,
}
struct CountCommand {}
struct CalculateCommand {}

impl Command for PingCommand {
    fn get_name(&self) -> &'static str {
        "ping"
    }

    fn exec(&mut self, _args: &[&str]) {
        println!("pong!");
    }
}

impl Command for TimesCommand {
    fn get_name(&self) -> &'static str {
        "times"
    }

    fn exec(&mut self, _args: &[&str]) {
        self.count += 1;
        if self.count == 1 {
            println!("<times> command was used once")
        } else {
            println!("<times> command was used {} times", self.count)
        };
    }
}

impl Command for CountCommand {
    fn get_name(&self) -> &'static str {
        "count"
    }

    fn exec(&mut self, args: &[&str]) {
        if args.is_empty() {
            println!("No arguments were given; Usage: count <args>");
        } else {
            println!("Counted {} args -> {:?}", args.len(), &args);
        }
    }
}

impl Command for CalculateCommand {
    fn get_name(&self) -> &'static str {
        "calculate"
    }

    fn exec(&mut self, args: &[&str]) {
        if args.len() != 3 {
            println!("No arguments were given; Usage: calculate <number1> <operator> <number2>");
            return;
        }

        let num1: f64 = match args[0].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number!");
                return;
            }
        };

        let num2: f64 = match args[2].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number!");
                return;
            }
        };

        let result: f64 = match args[1] {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => {
                if num2 != 0.0 {
                    num1 / num2
                } else {
                    println!("You cannot divide by 0...");
                    return;
                }
            }
            _ => {
                println!("Invalid operator!");
                return;
            }
        };

        println!("Result: {} {} {} = {}", num1, args[1], num2, result);
    }
}

struct Terminal {
    commands: Vec<Box<dyn Command>>,
}

impl Terminal {
    fn new() -> Terminal {
        Terminal {
            commands: Vec::new(),
        }
    }

    fn register(&mut self, command: Box<dyn Command>) -> () {
        self.commands.push(command);
    }

    fn run(&mut self) -> () {
        let content = fs::read_to_string("src/input").expect("Unable to open input file!\n");

        for line in content.lines() {
            if !line.trim().is_empty() {
                let args: Vec<&str> = line.split_whitespace().collect();

                if let Some(command) = self
                    .commands
                    .iter_mut()
                    .find(|cmd| cmd.get_name() == args[0])
                {
                    let args = if args.len() > 1 { &args[1..] } else { &[] };
                    command.exec(args);
                } else if args[0] == "stop" {
                    self.stop();
                } else {
                    let lowercase_string = args[0].to_lowercase();
                    let lowercase_str: &str = &lowercase_string;
                    if let Some(suggestion) = self.suggest_command(lowercase_str) {
                        println!("Unknown command! Did you mean '{}'?", suggestion);
                    } else {
                        println!("Unknown command: {} ", args[0]);
                        println!("Try one of the following commands: ");

                        for (index, command) in self.commands.iter().enumerate() {
                            println!(
                                "{}. {}{}",
                                index + 1,
                                command.get_name(),
                                if command.get_name() == "count" {
                                    " <args>"
                                } else if command.get_name() == "calculate" {
                                    " <number1> <operator> <number2>"
                                } else if command.get_name() == "bm" {
                                    " add <name> <url> OR bm search <name>"
                                } else {
                                    ""
                                }
                            );
                        }
                    }
                }
            }
        }
    }

    fn suggest_command(&self, entered_command: &str) -> Option<&'static str> {
        let mut best_suggestion: Option<&'static str> = None;
        let mut min_distance = usize::MAX;
        let mut command_name = String::new();

        for command in &self.commands {
            let distance = levenshtein(entered_command, command.get_name());
            if distance < min_distance {
                min_distance = distance;
                best_suggestion = Some(command.get_name());
                command_name = String::from(command.get_name());
            }
        }

        if min_distance <= 3 && command_name.len() >= 5 {
            return best_suggestion;
        } else if min_distance <= 2 && command_name.len() >= 4 {
            return best_suggestion;
        } else if min_distance <= 1 && command_name.len() >= 2 {
            return best_suggestion;
        }

        None
    }

    fn stop(&self) -> () {
        println!("Quitting...");
        process::exit(0);
    }
}

fn main() {
    let mut terminal = Terminal::new();

    terminal.register(Box::new(PingCommand {}));
    terminal.register(Box::new(CountCommand {}));
    terminal.register(Box::new(TimesCommand { count: 0 }));
    terminal.register(Box::new(CalculateCommand {}));
    terminal.register(Box::new(Database {
        name: String::new(),
        url: String::new(),
    }));
    Database::create_database();
    terminal.run();
}

//bonus
struct Database {
    name: String,
    url: String,
}

impl Database {
    fn create_database() -> () {
        let conn = Connection::open("links.db").expect("Could not open the database!\n");
        let create = r"
        create table if not exists links (
            name text    not null,
            url  text not null
    );
    ";
        conn.execute(create, ())
            .expect("Could not create database!\n");

        conn.execute("delete from links", [])
            .expect("Could not clear the database!\n");
    }
}

impl Command for Database {
    fn get_name(&self) -> &'static str {
        "bm"
    }

    fn exec(&mut self, args: &[&str]) {
        let conn = Connection::open("links.db").expect("Could not open the database!\n");
        if args.len() < 1 {
            println!("No arguments were given; Usage: bm add <name> <url> OR bm search <name>");
            return;
        }

        if args.len() < 2 {
            if args[0] == "add" {
                println!("No name or url was provided! Use bm add <name> <url>",);
                return;
            } else if args[0] == "search" {
                println!("No name was provided to search in the database! Use bm search <name>");
                return;
            }
        }

        match args[0] {
            "add" => {
                let name = args[1];
                let url = args[2];
                conn.execute(
                    "insert or ignore into links (name, url) values (?1, ?2);",
                    (name, url),
                )
                .expect("Could not insert in the database!\n");
            }
            "search" => {
                let mut stmt = conn
                    .prepare("select name, url from links")
                    .expect("Could not open database!\n");
                let links_iter = stmt
                    .query_map([], |row| {
                        Ok(Database {
                            name: row.get("name").expect("Could not extract name!\n"),
                            url: row.get("url").expect("Could not extract url!\n"),
                        })
                    })
                    .expect("Could not find rows!\n");

                let mut found_entry = false;

                for i in links_iter {
                    let i = i.expect("Error retrieving data from database!\n");
                    if let Some(_) = i.name.find(args[1]) {
                        println!("name = {}, url = {}", i.name, i.url);
                        found_entry = true;
                    }
                }

                if !found_entry {
                    println!("There is no related entry to '{}' in the database", args[1]);
                }
            }
            _ => println!("Unknown command!"),
        }
    }
}
