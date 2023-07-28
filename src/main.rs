use std::io;
use std::fs::File;
use std::io::{Write, BufReader, BufRead};
use std::str::FromStr;

type Todos = Vec<Todo>;

struct Todo {
    title: String,
}

fn print_todos(todos: &mut Vec<Todo>) {
    for (idx, todo) in todos.iter().enumerate() {
        println!("{}) {}", idx + 1, todo.title)
    }
}

fn save_to_csv(file: &mut File, todos: &mut Vec<Todo>) {
    for todo in todos {
        let line = format!("{}\n", todo.title);
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
        loaded_todos.push(Todo{title: row });
    }
    println!("{} Todos Loaded!", loaded_todos.len());
    Ok(loaded_todos)
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

    let mut choice = String::new();
    let input_reader = io::stdin();

    let choice_prompt = "\
        Choices:\n\
        a) Add\n\
        d) Delete\n\
        s) Save\n\
        q) quit\n\
    ";

    loop {
        choice.clear();

        print_todos(&mut todos);
        println!("{}", choice_prompt);

        input_reader.read_line(&mut choice).unwrap();

        let prompt_choice = choice.chars().next().unwrap();

        match prompt_choice {
            'a' => {
                print!("{}", "Enter title\n");
                let mut title_buffer = String::new();
                input_reader.read_line(&mut title_buffer).unwrap();
                todos.push(Todo{title: String::from_str(title_buffer.trim()).unwrap()});
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
                let mut target_file = match File::options().write(true).open(db_path) {
                    Ok(file) => {
                        println!("Use Existing db");
                        file
                    },
                    Err(_) => {
                        println!("Create new DB");
                        File::create(db_path).unwrap()
                    }
                };
                save_to_csv(&mut target_file, &mut todos)
            },
            'q' => break,
            _ => {
                println!("????");
            }
        } 
        
    }
}
