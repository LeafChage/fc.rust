pub struct ShowableBinary<'a> {
    line_length: usize,
    binary: &'a [u8],
}

pub fn showable<'a>(binary: &'a [u8]) -> ShowableBinary<'a> {
    ShowableBinary {
        line_length: 8,
        binary,
    }
}

pub fn showable_with<'a>(binary: &'a [u8], line_length: usize) -> ShowableBinary<'a> {
    ShowableBinary {
        line_length,
        binary,
    }
}

impl<'a> std::fmt::Display for ShowableBinary<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let binary = self.binary;
        let mut lines = Vec::new();

        let mut i = 0;
        while i < binary.len() {
            let from = i;
            let to = std::cmp::min(from + self.line_length, binary.len());
            lines.push(&binary[i..to]);
            i = to;
        }

        for line in lines {
            let ch = line
                .iter()
                .map(|d| match char::from_u32(u32::from(d.clone())) {
                    Some('\0') => '_',
                    Some(n) if n.is_ascii_graphic() => n,
                    Some(_) => '?',
                    None => '.',
                })
                .collect::<String>();
            writeln!(f, "{:02X?} {:?}", line, ch)?;
        }
        Ok(())
    }
}

const LINE_LENGTH: usize = 8;

pub fn show_binary(binary: &[u8]) {
    println!(
        "{}",
        ShowableBinary {
            binary,
            line_length: LINE_LENGTH
        }
    );
}

pub fn show_binary_with(binary: &[u8], length: usize) {
    println!(
        "{}",
        ShowableBinary {
            binary,
            line_length: length
        }
    );
}
