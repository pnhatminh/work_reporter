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
            Some(menu::MainMenu::ExportCSV) => menu::export_csv(&sessions),
            Some(menu::MainMenu::ImportCSV) => menu::import_csv(&mut sessions),
            None => break,
        }
    }
    None
}
