use std::{
    fs::read_to_string,
    io::{Write, stdin, stdout},
};

use image::{DynamicImage, ImageBuffer, Rgb, load_from_memory};

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

fn image_to_entry(image: DynamicImage) -> Vec<u16> {
    let mut new = Vec::new();
    for row in image.as_bytes().chunks(image.width() as usize * 3) {
        let mut entry = 0;
        for (x, pixel) in row.chunks(3).enumerate() {
            if pixel != [0, 0, 0] {
                entry |= 1 << (16 - x - 2);
            }
        }
        // println!("{}", format!("{:016b}", entry).replace("0", " "));
        new.push(entry);
    }
    new
}

static SUBTITUTIONS: &[[&str; 2]] = &[
    ["\"", "QUOTE"],
    ["*", "ASTERISK"],
    [":", "COLON"],
    ["<", "LESSTHAN"],
    [">", "GREATERTHAN"],
    ["?", "QUESTIONMARK"],
    ["|", "PIPE"],
    [",", "COMMA"],
];

fn substitute(mut text: String, left_to_right: bool) -> String {
    for [l, r] in SUBTITUTIONS.iter() {
        if left_to_right {
            text = text.replace(l, r);
        } else {
            text = text.replace(r, l);
        }
    }
    text
}

fn preview_or_export(export: bool) {
    let fonts = read_to_string("fonts.c")
        .expect("No fonts.c file in current working directory")
        .replace("// ,", "// COMMA");

    // find fonts
    let fonts: Vec<String> = fonts.split("] = {").map(str::to_string).collect();

    for (i, font) in fonts.iter().enumerate() {
        let amt = font
            .trim()
            .split_once("\n")
            .unwrap_or(("", ""))
            .0
            .split(",")
            .count()
            - 1;
        println!("{i}. Guessed height: {amt}");
    }
    let mut input = String::new();
    print!("Enter the index of the font to preview/export: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut input).unwrap();
    let input: usize = input.trim().parse().expect("Bad index");
    let font = &fonts[input];

    // guess height based on formatting
    let height = font.trim().split_once("\n").unwrap().0.split(",").count() - 1;
    if !export {
        println!("Enter a character to display it:");
    }
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
                    .trim_end_matches("*/")
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
    if export {
        for (index, char) in values.into_iter().enumerate() {
            let mut bytes: Vec<u8> = Vec::new();
            for y in 0..char.len() {
                for x in 0..16 {
                    let top_pixel = &char[y as usize] << x & 0x4000 != 0;
                    if top_pixel {
                        bytes.extend_from_slice(&[255, 255, 255]);
                    } else {
                        bytes.extend_from_slice(&[0, 0, 0]);
                    }
                }
            }

            let buffer: ImageBuffer<Rgb<u8>, Vec<u8>> =
                ImageBuffer::from_raw(bytes.len() as u32 / 3 / height as u32, height as _, bytes)
                    .unwrap();
            let mut name = labels[index].clone();
            if name.chars().nth(0).unwrap_or('4').is_ascii_lowercase() {
                name += "_lower";
            }
            name = substitute(name, true);
            buffer
                .save_with_format(format!("output/{}.png", name), image::ImageFormat::Png)
                .unwrap();
        }
        println!("done!");
    } else {
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
}

fn generate() {
    let mut text: Vec<String> = Vec::new();

    for file in std::fs::read_dir("add").unwrap().flatten() {
        if file.file_name().to_str().unwrap().starts_with(".") {
            continue;
        }
        let Ok(data) = std::fs::read("add/".to_string() + file.file_name().to_str().unwrap())
        else {
            continue;
        };
        let Ok(image) = load_from_memory(&data) else {
            println!(
                "Warning: failed to parse image {}",
                file.file_name().to_str().unwrap()
            );
            continue;
        };
        let mut t = String::new();
        let entry = image_to_entry(image);
        if text.is_empty() {
            let mut t = String::new();
            for _ in 0..entry.len() {
                t += "0x0000, ";
            }
            text.push(t + " // ");
        }

        for item in entry {
            t += &format!("0x{item:04x}, ");
        }
        let name = substitute(
            file.file_name()
                .to_str()
                .unwrap()
                .split_once(".")
                .unwrap()
                .0
                .replace("_lower", ""),
            false,
        );

        text.push(format!("{t} // {}", name));
    }
    text.sort_by(|a, b| a.chars().last().unwrap().cmp(&b.chars().last().unwrap()));
    std::fs::write("output.c", text.join("\n")).unwrap();
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.contains(&"preview".to_string()) {
        preview_or_export(false);
    } else if args.contains(&"generate".to_string()) {
        generate();
    } else if args.contains(&"export".to_string()) {
        preview_or_export(true);
    } else {
        println!("# cFontDecoder");
        println!("usage: cfontdecoder preview|generate|export");
        println!("\nmodes:");
        println!("\tpreview: print characters from the font file to stdout");
        println!("\tgenerate: generate font file entries from images in add/");
        println!("\texport: export font characters to images in output/");
    }
}
