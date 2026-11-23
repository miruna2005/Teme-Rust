use rusqlite::{Connection, params};
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

trait Command {
    fn get_name(&self) -> &str;
    fn exec(&mut self, args: &[String]) -> Result<(), String>;
}

struct PingCommand;

impl Command for PingCommand {
    fn get_name(&self) -> &str {
        "ping"
    }

    fn exec(&mut self, _args: &[String]) -> Result<(), String> {
        println!("pong!");
        Ok(())
    }
}

struct CountCommand;

impl Command for CountCommand {
    fn get_name(&self) -> &str {
        "count"
    }

    fn exec(&mut self, args: &[String]) -> Result<(), String> {
        println!("counted {} args", args.len());
        Ok(())
    }
}

struct TimesCommand {
    count: u64,
}

impl Command for TimesCommand {
    fn get_name(&self) -> &str {
        "times"
    }

    fn exec(&mut self, _args: &[String]) -> Result<(), String> {
        self.count = self.count.saturating_add(1);
        println!("times has been called {} times", self.count);
        Ok(())
    }
}

struct HelpCommand {
    names: Vec<String>,
}

impl Command for HelpCommand {
    fn get_name(&self) -> &str {
        "help"
    }

    fn exec(&mut self, _args: &[String]) -> Result<(), String> {
        println!("Available commands: {}, stop", self.names.join(", "));
        Ok(())
    }
}

struct BookmarkCommand {
    database_path: String,
}

impl BookmarkCommand {
    fn connect(&self) -> Result<Connection, String> {
        let connection = Connection::open(&self.database_path)
            .map_err(|error| format!("could not open bookmark database: {error}"))?;
        connection
            .execute(
                "CREATE TABLE IF NOT EXISTS bookmarks (
                    name TEXT NOT NULL,
                    url  TEXT NOT NULL
                )",
                [],
            )
            .map_err(|error| format!("could not initialize bookmark database: {error}"))?;
        Ok(connection)
    }
}

impl Command for BookmarkCommand {
    fn get_name(&self) -> &str {
        "bk"
    }

    fn exec(&mut self, args: &[String]) -> Result<(), String> {
        let Some(operation) = args.first().map(String::as_str) else {
            return Err("usage: bk add <name> <url> | bk search <name>".to_owned());
        };

        match operation {
            "add" => {
                if args.len() != 3 {
                    return Err("usage: bk add <name> <url>".to_owned());
                }
                let connection = self.connect()?;
                connection
                    .execute(
                        "INSERT INTO bookmarks (name, url) VALUES (?1, ?2)",
                        params![&args[1], &args[2]],
                    )
                    .map_err(|error| format!("could not add bookmark: {error}"))?;
                println!("added bookmark {}", args[1]);
                Ok(())
            }
            "search" => {
                if args.len() != 2 {
                    return Err("usage: bk search <name>".to_owned());
                }
                let connection = self.connect()?;
                let escaped = args[1]
                    .replace('\\', "\\\\")
                    .replace('%', "\\%")
                    .replace('_', "\\_");
                let pattern = format!("%{escaped}%");
                let mut statement = connection
                    .prepare(
                        "SELECT name, url FROM bookmarks WHERE name LIKE ?1 ESCAPE '\\' ORDER BY name",
                    )
                    .map_err(|error| format!("could not search bookmarks: {error}"))?;
                let rows = statement
                    .query_map([pattern], |row| {
                        Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
                    })
                    .map_err(|error| format!("could not search bookmarks: {error}"))?;

                let mut found_any = false;
                for row in rows {
                    match row {
                        Ok((name, url)) => {
                            found_any = true;
                            println!("{name} {url}");
                        }
                        Err(error) => return Err(format!("could not read bookmark: {error}")),
                    }
                }
                if !found_any {
                    println!("no bookmarks found");
                }
                Ok(())
            }
            _ => Err("unknown bk operation; use 'add' or 'search'".to_owned()),
        }
    }
}

struct Terminal {
    commands: Vec<Box<dyn Command>>,
}

impl Terminal {
    fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    fn register(&mut self, command: Box<dyn Command>) -> Result<(), String> {
        if self
            .commands
            .iter()
            .any(|registered| registered.get_name() == command.get_name())
        {
            return Err(format!(
                "command '{}' is already registered",
                command.get_name()
            ));
        }
        self.commands.push(command);
        Ok(())
    }

    fn run(&mut self, path: impl AsRef<Path>) -> Result<(), String> {
        let file = File::open(path.as_ref())
            .map_err(|error| format!("could not read '{}': {error}", path.as_ref().display()))?;

        for (index, line_result) in BufReader::new(file).lines().enumerate() {
            let line_number = index + 1;
            let line = match line_result {
                Ok(line) => line,
                Err(error) => {
                    eprintln!("Error on line {line_number}: could not read line: {error}");
                    continue;
                }
            };
            if !self.execute_line(&line, line_number) {
                break;
            }
        }
        Ok(())
    }

    fn execute_line(&mut self, line: &str, line_number: usize) -> bool {
        let words: Vec<String> = line.split_whitespace().map(str::to_owned).collect();
        let Some(name) = words.first() else {
            eprintln!("Error on line {line_number}: empty command");
            return true;
        };

        if name == "stop" || name.eq_ignore_ascii_case("stop") {
            if words.len() != 1 {
                eprintln!("Error on line {line_number}: 'stop' takes no arguments");
                return true;
            }
            return false;
        }

        let args = &words[1..];
        if let Some(command) = self
            .commands
            .iter_mut()
            .find(|command| command.get_name() == name)
        {
            if let Err(error) = command.exec(args) {
                eprintln!("Error on line {line_number}: {error}");
            }
            return true;
        }

        if let Some(suggestion) = self.suggest_command(name) {
            eprintln!(
                "Error on line {line_number}: unknown command '{name}'. Did you mean '{suggestion}'?"
            );
        } else {
            eprintln!("Error on line {line_number}: unknown command '{name}'");
        }
        true
    }

    fn suggest_command(&self, input: &str) -> Option<&str> {
        let input_lower = input.to_ascii_lowercase();
        let mut best: Option<(&str, usize)> = None;
        for command in &self.commands {
            let name = command.get_name();
            let distance = edit_distance(&input_lower, name);
            let threshold = (name.len() / 3).max(1);
            if distance <= threshold
                && best.is_none_or(|(_, best_distance)| distance < best_distance)
            {
                best = Some((name, distance));
            }
        }
        best.map(|(name, _)| name)
    }
}

fn edit_distance(left: &str, right: &str) -> usize {
    let right_bytes = right.as_bytes();
    let mut previous: Vec<usize> = (0..=right_bytes.len()).collect();
    let mut current = vec![0; right_bytes.len() + 1];

    for (left_index, left_byte) in left.bytes().enumerate() {
        current[0] = left_index + 1;
        for (right_index, right_byte) in right_bytes.iter().enumerate() {
            let substitution_cost = usize::from(left_byte != *right_byte);
            current[right_index + 1] = (previous[right_index + 1] + 1)
                .min(current[right_index] + 1)
                .min(previous[right_index] + substitution_cost);
        }
        std::mem::swap(&mut previous, &mut current);
    }
    previous[right_bytes.len()]
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut terminal = Terminal::new();
    terminal
        .register(Box::new(PingCommand))
        .map_err(io::Error::other)?;
    terminal
        .register(Box::new(CountCommand))
        .map_err(io::Error::other)?;
    terminal
        .register(Box::new(TimesCommand { count: 0 }))
        .map_err(io::Error::other)?;
    terminal
        .register(Box::new(BookmarkCommand {
            database_path: "bookmarks.db".to_owned(),
        }))
        .map_err(io::Error::other)?;
    terminal
        .register(Box::new(HelpCommand {
            names: vec![
                "ping".to_owned(),
                "count".to_owned(),
                "times".to_owned(),
                "bk".to_owned(),
            ],
        }))
        .map_err(io::Error::other)?;

    let input_path = env::args()
        .nth(1)
        .unwrap_or_else(|| "commands.txt".to_owned());
    terminal.run(input_path).map_err(io::Error::other)?;
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("Error: {error}");
        std::process::exit(1);
    }
}
