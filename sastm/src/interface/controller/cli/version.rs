pub struct Version;

impl Version {
    pub fn print() {
        println!("Version: {}", env!("CARGO_PKG_VERSION"));
    }
}
