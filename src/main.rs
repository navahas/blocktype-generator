use std::collections::HashMap;
use std::fs;

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

fn main() -> std::io::Result<()> {
    let font = load_font("./font.txt")?;
    let input = "sessionizer".to_uppercase();
    let height = font.get(&'A').map(|v| v.len()).unwrap_or(0);

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

