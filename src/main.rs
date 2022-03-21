use session::Sessions;

mod menu;
mod session;

fn main() {
    run_cli();
}

fn run_cli() -> Option<()> {
    let mut sessions = Sessions::new();
    loop {
        menu::MainMenu::show();
        let input = menu::get_input()?;
        match menu::MainMenu::from_str(input.as_str()) {
            Some(menu::MainMenu::CheckIn) => menu::check_in(&mut sessions),
            Some(menu::MainMenu::CheckOut) => menu::check_out(&mut sessions),
            Some(menu::MainMenu::ViewAll) => menu::view_all(&sessions),
            None => break,
        }
    }
    None
}
