pub struct CPU {
    running: bool,
}

impl CPU {
    pub fn new() -> CPU {
        CPU { running: true }
    }

    pub fn run(self) {
        println!("Run!");
    }
}