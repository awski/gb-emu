pub struct Gbe {

}

impl Gbe {
    pub fn setup() -> Self {
        Gbe {}
    }

    pub fn load_cartrige(self, path: &str) -> Self {
        println!("Loading rom: {}", path);
        self
    }

    pub fn run(self) -> Self {
        self
    }
}