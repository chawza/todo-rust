use std::io;

struct Todo {
    title: String,
}

fn print_todos(todos: &mut Vec<Todo>) {
    for (idx, todo) in todos.iter().enumerate() {
        print!("{}) {}", idx + 1, todo.title)
    }
}

fn main() {
    let mut todos: Vec<Todo> = Vec::new();
    let mut choice = String::new();

    let input_reader = io::stdin();


    let choice_prompt = "\
        Choices:\n\
        a) Add\n\
        d) Delete\n\
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
                todos.push(Todo{title: title_buffer});
            },
            'd' => {
                print!("DELTE");
            },
            'q' => break,
            _ => {
                print!("????");
            }
        } 
        
    }
}
