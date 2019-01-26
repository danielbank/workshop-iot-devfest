use light_morse::*;
use std::io::stdout;
use std::io::Write;
use std::thread;
use std::time::Duration;

pub trait Display {
    fn display(&self);
}

impl Display for Morse {
    fn display(&self) {
        for item in self.chars() {
            print!("{}", item);
            stdout().flush().unwrap();

            thread::sleep(Duration::from_millis(200));
        }
        println!();
    }
}