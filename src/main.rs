use rand::Rng;
use std::io::Write;

fn main() {
    let mut terminal = Terminal::new();
    let mut text = StaticPrefixText::new(32);
    loop {
        terminal.typeset(text.wall());
        terminal.render();
        text.update_postfixes();
    }
}

struct Terminal<'a> {
    frame: String,
    line_lengths: Vec<usize>,
    lock: std::io::StdoutLock<'a>,
}

impl<'a> Terminal<'a> {
    fn new() -> Terminal<'a> {
        Terminal::clear_screen();
        let stdout = std::io::stdout();
        Terminal {
            frame: String::new(),
            line_lengths: vec![0],
            lock: stdout.lock(),
        }
    }

    fn clear_screen() {
        print!("\x1Bc");
    }

    fn set_line_lengths(&mut self) {
        self.line_lengths = self.frame.lines().map(|line| line.trim().len()).collect();
        self.frame = String::from("\x1B[H");
    }

    fn typeset(&mut self, text: String) {
        for (i, line) in text.lines().enumerate() {
            let spaces = if let Some(length) = self.line_lengths.get(i) {
                " ".repeat(length.saturating_sub(line.len()))
            } else {
                String::new()
            };

            self.frame += &format!("{line}{spaces}\n");
        }
    }

    fn render(&mut self) {
        self.lock.write_all(self.frame.as_bytes()).unwrap();
        //write!(self.lock, "{}", self.frame).unwrap();
        self.set_line_lengths();
    }
}

struct StaticPrefixText {
    prefixes: Vec<String>,
    postfixes: Vec<String>,
    update_count: usize,
}

impl StaticPrefixText {
    fn new(line_count: usize) -> StaticPrefixText {
        StaticPrefixText {
            prefixes: random_strings(line_count),
            postfixes: random_strings(line_count),
            update_count: 0,
        }
    }

    fn update_postfixes(&mut self) {
        self.postfixes = random_strings(self.prefixes.len());
        self.update_count += 1;

        if self.update_count % 4096 == 0 {
            self.prefixes = random_strings(self.prefixes.len());
            self.postfixes = random_strings(self.prefixes.len());
            self.update_count = 0;
        }
    }

    fn wall(&self) -> String {
        let mut text = String::new();
        for (prefix, postfix) in self.prefixes.iter().zip(self.postfixes.iter()) {
            text += &format!("{}: {}\n", prefix, postfix);
        }
        text
    }
}

fn random_strings(line_count: usize) -> Vec<String> {
    let mut rng = rand::thread_rng();
    let mut strings = Vec::new();
    for _ in 0..line_count {
        let mut string = String::new();
        for _ in 0..rng.gen_range(1..=39) {
            string.push(random_char());
        }
        strings.push(string);
    }

    strings
}

fn random_char() -> char {
    let mut rng = rand::thread_rng();
    rng.gen_range(33..=126).into()
}
