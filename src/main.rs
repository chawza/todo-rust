mod store;
mod utils;

use std::io::{Write, stdout, stdin};
use std::str::FromStr;
use chrono::{NaiveDate, Local};

use store::store::StoreSesssion;
use utils::console;

static PRETTY_DATE_STRING: &str = "%Y %m %d";

pub struct Todo {
    id: usize,
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


fn main() {
    let sqlite_path = "./test.sqlite";

    let store = StoreSesssion::open(sqlite_path);
    
    let stdin_reader = stdin();
    let mut choice = String::new();
    
    let mut todos = store.fetch_all_todos();

    let choice_prompt = "\
        Choices:\n\
        a) Add\n\
        d) Delete\n\
        q) quit\n\
    ";

    loop {
        choice.clear();
        console::clear();

        print_todos(&mut todos);
        println!("{}", choice_prompt);

        stdin_reader.read_line(&mut choice).unwrap();

        let prompt_choice = choice.chars().next().unwrap();
        console::clear();

        match prompt_choice {
            'a' => {
                console::inline_prompt("Enter title: ");
                let mut title_buffer = String::new();
                stdin_reader.read_line(&mut title_buffer).unwrap();
                let title = String::from_str(title_buffer.trim()).unwrap();

                console::inline_prompt("Enter date (\"YYYY MM DD\" / \"today\"): ");
                let mut date_buffer = String::new();
                stdin_reader.read_line(&mut date_buffer).unwrap();

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

                store.add_todo(&title, &date);
                todos = store.fetch_all_todos();
            },
            'd' => {
                console::inline_prompt("Select item index: ");
                let mut item_idx = String::new();
                stdin_reader.read_line(&mut item_idx).unwrap();
                let idx = item_idx.trim().parse::<usize>().unwrap();

                if idx <= todos.len() && idx > 0 {
                    store.delete_todo_by_id(
                        todos.get(idx - 1).unwrap().id
                    );
                    todos = store.fetch_all_todos();
                } else {
                    println!("Out of Index! {}", idx);
                    console::press_enter_to_contune();
                }

            },
            'q' => break,
            _ => {
                println!("????");
            }
        };
    }
}
