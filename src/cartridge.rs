#[derive(Clone, Debug)]
pub struct Cartridge {
    pub rom: Vec<u8>,
}

impl Cartridge {
    pub fn new(filename: &str) -> Cartridge {
        println!("Trying to open {:?}", filename);
        match std::fs::read(filename) {
            Ok(bytes) => Cartridge { rom: bytes },
            Err(e) => {
                if e.kind() == std::io::ErrorKind::NotFound {
                    eprintln!(
                        "ERROR OPENING {}: are you sure the filename is correct?",
                        filename
                    );
                }
                if e.kind() == std::io::ErrorKind::PermissionDenied {
                    eprintln!("Please run again with appropriate permissions.");
                }
                panic!("{}", e);
            }
        }
    }
}
