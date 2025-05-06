use std::collections::HashMap;
use std::{fs, io};

fn load_font(path: &str) -> std::io::Result<HashMap<char, Vec<String>>> {
    let text = fs::read_to_string(path)?;
    let mut map = HashMap::new();

    let mut lines = text.lines()
        .filter(|l| !l.trim_start().starts_with('#'))  // strip comments
        .peekable();

    while let Some(key_line) = lines.next() {
        let key_line = key_line.trim();
        if key_line.is_empty() { continue }
        let ch = key_line.chars().next().unwrap();

        let mut glyph = Vec::new();
        while glyph.len() < 6 {
            if let Some(l) = lines.next() {
                let l = l.to_string();
                if l.trim().is_empty() { continue }
                glyph.push(l);
            } else {
                break;
            }
        }
        map.insert(ch, glyph);
    }
    Ok(map)
}

fn print_word(word: &str, font_name: &str) -> io::Result<()> {
    let font_path = format!("./{}.txt", font_name);
    let font = load_font(&font_path)?;
    let input = word.to_uppercase();

    let height = font.get(&'A').map(|lines| lines.len()).unwrap_or_else(|| {
        eprintln!("Warning: Font for 'A' not found. Defaulting height to 0.");
        0
    });

    for row in 0..height {
        for ch in input.chars() {
            if let Some(lines) = font.get(&ch) {
                print!("{}", &lines[row]);
            } else {
                print!("{{?}} ");
            }
        }
        println!();
    }
    Ok(())
}

fn main() {
    let word = "codecrypto";
    let font = "font";
    if let Err(e) = print_word(word, font) {
        eprintln!("Error painting word: {}", e)
    }
}
