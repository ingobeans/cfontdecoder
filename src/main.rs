use std::{fs::read_to_string, io::stdin, process::exit};

fn draw_char(values: &[u16]) {
    for y in (0..values.len()).step_by(2) {
        for x in 0..16 {
            let top_pixel = &values[y as usize] << x & 0x4000 != 0;
            let bottom_pixel = &values[y as usize + 1] << x & 0x4000 != 0;
            if top_pixel && bottom_pixel {
                print!("█");
            } else if top_pixel {
                print!("▀");
            } else if bottom_pixel {
                print!("▄");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
}
fn main() {
    let fonts = read_to_string("fonts.c")
        .expect("No fonts.c file in current working directory")
        .replace("// ,", "// COMMA");

    // find fonts
    let fonts: Vec<String> = fonts.split("] = {").map(str::to_string).collect();

    let font = &fonts[1];

    // guess height based on formatting
    let height = font.trim().split_once("\n").unwrap().0.split(",").count() - 1;
    println!("Guessing height={height}");

    let mut labels = Vec::new();
    let mut values = Vec::new();
    for mut value in font.split(",") {
        if value.contains("\n") {
            // check for label
            let label;
            (label, value) = value.split_once("\n").unwrap();
            if label.contains("//") || label.contains("/*") {
                let l = label
                    .trim()
                    .trim_start_matches("//")
                    .trim_start_matches("/*")
                    .trim();
                let l = l.split_once(" ").unwrap_or(("", l)).1;
                labels.push(l.to_string());
            }
        }
        let Ok(value) = u16::from_str_radix(
            value.trim().trim_start_matches("0x").trim_end_matches(","),
            16,
        ) else {
            break;
        };
        values.push(value);
    }

    let values: Vec<&[u16]> = values.chunks(height).collect();

    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        for i in input.trim().chars() {
            if i == ' ' {
                continue;
            }
            let i = i.to_string();
            draw_char(
                values[labels
                    .iter()
                    .position(|f| f == &i.replace(",", "COMMA"))
                    .unwrap_or(0)],
            );
        }
    }
}
