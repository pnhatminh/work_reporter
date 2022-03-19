pub enum MainMenu {
    CheckIn,
    CheckOut,
    ViewAll,
    // ViewOne,
    // ViewUser,
    // Delete,
}

impl MainMenu {
    pub fn from_str(input: &str) -> Option<MainMenu> {
        match input {
            "1" => Some(Self::CheckIn),
            "2" => Some(Self::CheckOut),
            "3" => Some(Self::ViewAll),
            _ => None,
        }
    }

    pub fn show() {
        println!("");
        println!(" == Work Reporter ==");
        println!("1. Check in");
        println!("2. Check out");
        println!("3. View All");
        println!("");
        println!("Enter selection: ");
    }
}
