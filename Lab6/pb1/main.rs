use std::fs::File;
use std::io::{self, BufRead};

trait Command {
    fn get_name(&self) -> &str;
    fn exec(&mut self, args: &[&str]);
}

struct PingCommand;

impl Command for PingCommand {
    fn get_name(&self) -> &str {
        "ping"
    }

    fn exec(&mut self, _args: &[&str]) {
        println!("pong!");
    }
}
struct CountCommand;

impl Command for CountCommand {
    fn get_name(&self) -> &str {
        "count"
    }

    fn exec(&mut self, args: &[&str]) {
        println!("counted {} args", args.len());
    }
}

struct TimesCommand {
    count: u32,
}

impl Command for TimesCommand {
    fn get_name(&self) -> &str {
        "times"
    }

    fn exec(&mut self, _args: &[&str]) {
        self.count += 1;
        println!("times called: {}", self.count);
    }
}
struct SumCommand {
    sum: i32,
}

impl Command for SumCommand {
    fn get_name(&self) -> &str {
        "sum"
    }

    fn exec(&mut self, args: &[&str]) {
        self.sum = 0;

        for &s in args {
            if let Ok(parsed) = s.parse::<i32>() {
                self.sum += parsed;
            } else {
                println!("Error parsing number: {}", s);
                return;
            }
        }

        println!("Sum: {}", self.sum);
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

    fn register(&mut self, command: Box<dyn Command>) {
        self.commands.push(command);
    }

    fn run(&mut self) {
        let file_path = "commands.txt";

        if let Ok(fisier) = File::open(file_path) {
            let reader = io::BufReader::new(fisier);

            for line in reader.lines() {
                if let Ok(command_line) = line {
                    let mut parts = command_line.trim().split_whitespace();
                    if let Some(command_name) = parts.next() {
                        let args: Vec<&str> = parts.collect();
                        self.execute_command(command_name, &args);
                    }
                }
            }
        } else {
            println!("Error opening the file: {}", file_path);
        }
    }

    fn execute_command(&mut self, command_name: &str, args: &[&str]) {
        match command_name.to_lowercase().as_str() {
            "stop" => {
                self.stop_execution();
            }
            _ => {
                for command in &mut self.commands {
                    if command.get_name() == command_name {
                        command.exec(args);
                        return;
                    }
                }
                println!("Invalid command: {}", command_name);
            }
        }
    }
    fn stop_execution(&self) {
        println!("Stopping execution");
        std::process::exit(0);
    }
}

fn main() {
    let mut terminal = Terminal::new();

    terminal.register(Box::new(PingCommand {}));
    terminal.register(Box::new(CountCommand {}));
    terminal.register(Box::new(TimesCommand { count: 0 }));
    terminal.register(Box::new(SumCommand { sum: 0 }));

    terminal.run();
}
