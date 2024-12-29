use std::io;

fn main() {
    
    fn clear() {
        print!("\x1B[2J\x1B[1;1H");
    }


    println!("
████████▄     ▄████████    ▄████████     ███        ▄█    █▄       ▄████████  ▄█     ▄████████ ███▄▄▄▄    ▄████████    ▄████████ 
███   ▀███   ███    ███   ███    ███ ▀█████████▄   ███    ███     ███    ███ ███    ███    ███ ███▀▀▀██▄ ███    ███   ███    ███
███    ███   ███    █▀    ███    ███    ▀███▀▀██   ███    ███     ███    █▀  ███▌   ███    ███ ███   ███ ███    █▀    ███    █▀ 
███    ███  ▄███▄▄▄       ███    ███     ███   ▀  ▄███▄▄▄▄███▄▄  ▄███▄▄▄     ███▌   ███    ███ ███   ███ ███         ▄███▄▄▄    
███    ███ ▀▀███▀▀▀     ▀███████████     ███     ▀▀███▀▀▀▀███▀  ▀▀███▀▀▀     ███▌ ▀███████████ ███   ███ ███        ▀▀███▀▀▀    
███    ███   ███    █▄    ███    ███     ███       ███    ███     ███        ███    ███    ███ ███   ███ ███    █▄    ███    █▄ 
███   ▄███   ███    ███   ███    ███     ███       ███    ███     ███        ███    ███    ███ ███   ███ ███    ███   ███    ███
████████▀    ██████████   ███    █▀     ▄████▀     ███    █▀      ███        █▀     ███    █▀   ▀█   █▀  ████████▀    ██████████


Welcome to Deathfiance, selfless savior.

What is your name?
");

    let mut name = String::new();

    io::stdin()
            .read_line(&mut name)
            .expect("An error has occured. Sorry, please contact the developer through E-mail, or create an issue on the Games GitHub repo. ");
    
    
    clear();
    println!("What is your gender identity? (Male, female, or other)");
    let mut gender = String::new();

    io::stdin()
        .read_line(&mut gender)
        .expect("An error has occured. Sorry, please contact the developer through E-mail, or create an issue on the Games GitHub repo.");
}
