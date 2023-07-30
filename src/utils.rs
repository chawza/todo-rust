pub mod console {
    use std::io::{stdin, stdout, Write};
    use std::process::Command;

    pub fn clear() {
        Command::new("clear").status().unwrap();
    }

    pub fn inline_prompt(prompt: &str) {
        print!("{}", prompt);
        stdout().flush().unwrap();
    }

    pub fn press_enter_to_contune() {
        inline_prompt("Press Enter to continue:");
        let _ = stdin().read_line(&mut String::new());
    }
}