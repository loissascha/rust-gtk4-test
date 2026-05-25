pub struct Fstab {
    content: String,
}

impl Fstab {
    pub fn new() -> Self {
        Self {
            content: String::new(),
        }
    }

    pub fn read_fstab(&self) -> std::io::Result<String> {
        std::fs::read_to_string("/etc/fstab")
    }

    pub fn set_content(&mut self, c: String) {
        self.content = c;
    }

    pub fn get_content(&self) -> String {
        self.content.clone()
    }
}
