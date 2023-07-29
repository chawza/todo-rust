use std::io;
use std::process::Command;
use std::fs::File;
use std::io::{Write, BufReader, BufRead, stdout};
use std::str::FromStr;
use chrono::{NaiveDate, Local};

type Todos = Vec<Todo>;

static DATE_STRING: &str = "%Y-%m-%d";
static PRETTY_DATE_STRING: &str = "%Y %m %d";

fn clear() {
    Command::new("clear").status().unwrap();
}

struct Todo {
    title: String,
    date: NaiveDate,
}

impl Todo {
    fn date_str(&self) -> String {
        self.date.format(PRETTY_DATE_STRING).to_string()
    }
}


fn print_todos(todos: &mut Vec<Todo>) {
    for (idx, todo) in todos.iter().enumerate() {
        println!("{}) {} {}", idx + 1, todo.title, todo.date_str())
    }
}

fn save_to_csv(file: &mut File, todos: &mut Vec<Todo>) {
    for todo in todos {
        let line = format!("{},{}\n", todo.title, todo.date.format(&DATE_STRING));
        let _ = file.write(line.as_bytes());
    }
}

fn load_from_existing_csv_file(filepath: &str) -> Result<Todos, String> {
    let file = match File::open(filepath) {
        Ok(file) => file,
        Err(msg) => return Err(msg.to_string())
    };
    let mut loaded_todos: Vec<Todo> = Vec::new();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let row = line.expect("Unable to load row");
        let columns: Vec<&str> = row.split(',').collect();

        if columns.len() != 2 {
            panic!("Invalid number column in CSV!");
        }

        let title = columns.get(0).unwrap();
        let date_str = columns.get(1).unwrap();

        let date = match NaiveDate::parse_from_str(&date_str, DATE_STRING) {
            Ok(res) => res,
            Err(_) => {
                panic!("Cannot parse {}", &date_str);
            }
        };
        loaded_todos.push(Todo{title: String::from_str(title).unwrap(), date });
    }
    println!("{} Todos Loaded!", loaded_todos.len());
    Ok(loaded_todos)
}

fn save_todos(db_path: &str, todos: &mut Vec<Todo>) {
    let mut target_file = match File::options().write(true).open(db_path) {
        Ok(file) => {
            println!("save to existing db ({})", db_path);
            file
        },
        Err(_) => {
            println!("Create new db ({})", db_path);
            File::create(db_path).unwrap()
        }
    };
    save_to_csv(&mut target_file, todos)
}


fn main() {
    let db_path = "./test.csv";
    let mut todos: Vec<Todo> = match load_from_existing_csv_file(&db_path) {
        Ok(todos) => todos,
        Err(_) => {
            println!("Fresh Todos!");
            Vec::new()
        }

    };

    let input_reader = io::stdin();
    let mut stdout = stdout();
    let mut choice = String::new();

    let choice_prompt = "\
        Choices:\n\
        a) Add\n\
        d) Delete\n\
        s) Save\n\
        q) quit\n\
    ";

    loop {
        choice.clear();
        clear();

        print_todos(&mut todos);
        println!("{}", choice_prompt);

        input_reader.read_line(&mut choice).unwrap();

        let prompt_choice = choice.chars().next().unwrap();
        clear();

        match prompt_choice {
            'a' => {
                print!("Enter title: ", );
                stdout.flush().unwrap();
                let mut title_buffer = String::new();
                input_reader.read_line(&mut title_buffer).unwrap();
                let title = String::from_str(title_buffer.trim()).unwrap();

                print!("Enter date (\"YYYY MM DD\" / \"today\"): ");
                stdout.flush().unwrap();
                let mut date_buffer = String::new();
                input_reader.read_line(&mut date_buffer).unwrap();

                let date: NaiveDate;
                
                if date_buffer.contains("today") {
                    date = Local::now().naive_local().date();
                } else {
                    date = match NaiveDate::parse_from_str(&date_buffer.trim(), PRETTY_DATE_STRING) {
                        Ok(res) => res,
                        Err(msg) => {
                            println!("Cannot parse [{}]: {}", date_buffer, msg);
                            continue
                        }
                    };
                }

                todos.push(Todo{title, date});
            },
            'd' => {
                let mut item_idx = String::new();
                input_reader.read_line(&mut item_idx).unwrap();

                let idx = item_idx.trim().parse::<usize>().unwrap();
                
                if idx <= todos.len() && idx > 0 {
                    todos.remove(idx - 1);
                } else {
                    println!("Out of Index! {}", idx)
                }

            },
            's' => {
                save_todos(db_path, &mut todos);
            },
            'q' => {
                choice.clear();
                println!("Save any changes? [y/n]");
                input_reader.read_line(&mut choice).unwrap();
                match choice.chars().next().unwrap() {
                    'y' => save_todos(db_path, &mut todos),
                    _ => () 
                }
                break
            },

            _ => {
                println!("????");
            }
        };
    }
}
