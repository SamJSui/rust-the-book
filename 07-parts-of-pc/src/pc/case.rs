#[derive(Debug)]
pub struct Machine {
    pub name: String
}

impl Machine {
    pub fn pc_print(&mut self) {
        println!("\nHi! My name is {}, your dedicated computing assistant!", &self.name);
    }
}