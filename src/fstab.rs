pub struct Fstab {
    pub content: String,
}

impl Fstab {
    pub fn new() -> Self {
        let mut s = Self {
            content: String::new(),
        };
        let res = s.read_fstab();
        match res {
            Ok(content) => s.set_content(content),
            Err(err) => panic!("Error reading fstab: {}", err),
        }
        s
    }

    fn read_fstab(&self) -> std::io::Result<String> {
        std::fs::read_to_string("/etc/fstab")
    }

    fn set_content(&mut self, c: String) {
        self.content = c;
    }
}
