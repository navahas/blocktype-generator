You absolutely can—using a plain .txt file is the simplest possible approach. You’d trade off the structure and safety of JSON/TOML for zero-dependency and maximum simplicity.

⸻

1. File format

Pick a convention. For example, in font.txt you might do:

# letter T
T
████████╗
╚══██╔══╝
   ██║   
   ██║   
   ██║   
   ╚═╝   

# letter B
B
██████╗ 
██╔══██╗
██████╔╝
██╔═══╝ 
███████╗
╚══════╝

	•	A line with exactly the single character (the key).
	•	Then its 6 lines of ASCII art.
	•	Then a blank line (or a # comment) before the next entry.

⸻

2. Reading & parsing in Rust

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

        // collect next 6 non-empty lines as the glyph
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

Then in main():

let font = load_font("font.txt")?;
let input = "AB";
let height = font.get(&'A').map(|v| v.len()).unwrap_or(0);

for row in 0..height {
    for ch in input.chars() {
        if let Some(lines) = font.get(&ch) {
            print!("{} ", &lines[row]);
        } else {
            print!("{{?}} ");
        }
    }
    println!();
}



⸻

3. Pros & cons

Pros	Cons
• No extra crates needed	• You write your own parsing logic
• Very human-readable format	• Harder to validate (missing lines, etc)
• Easy to hack/edit anywhere	• No comments in pure data blocks unless you filter them explicitly

If your font spec is very simple, a .txt parser like this may be all you ever need. Once your needs grow (multiple heights, per-glyph metadata, fallbacks), you’ll probably reach for a structured format (TOML, JSON, YAML) and a library.
