mod case {
    pub mod motherboard {
        pub fn cpu() {
            println!("1. CPU");
        }
        pub fn graphics_card() {
            println!("2. GPU");
        }
        pub fn ram() {
            println!("3. RAM");
        }
    }
}

pub fn motherboard_print(){
    println!("Parts that sit on my Motherboard: ");
    crate::case::motherboard::cpu();
    case::motherboard::graphics_card();
    crate::case::motherboard::ram();
}
pub fn cpu_print(){

}