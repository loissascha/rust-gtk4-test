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
        s.split_lines();
        s
    }

    fn read_fstab(&self) -> std::io::Result<String> {
        std::fs::read_to_string("/etc/fstab")
    }

    fn set_content(&mut self, c: String) {
        self.content = c;
    }

    fn split_lines(&self) {
        let split = self.content.split("\n");
        split.for_each(|s| {
            let trimmed = s.trim();
            if trimmed == "" {
                return;
            }
            println!("line: {}", s);
        });
    }
}
