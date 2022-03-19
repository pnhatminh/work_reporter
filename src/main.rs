mod menu;
use std::io;

fn main() {
    run_CLI();
}

fn run_CLI() -> Option<()> {
    loop {
        menu::MainMenu::show();
        let input = get_input()?;
        match menu::MainMenu::from_str(input.as_str()) {
            Some(menu::MainMenu::CheckIn) => println!("Check In"),
            Some(menu::MainMenu::CheckOut) => println!("Check Out"),
            Some(menu::MainMenu::ViewAll) => println!("View All"),
            None => break,
        }
    }
    None
}

fn get_input() -> Option<String> {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please enter your data again");
    }
    let input = buffer.trim().to_owned();
    if &input == "" {
        None
    } else {
        Some(input)
    }
}
