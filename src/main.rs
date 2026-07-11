fn main() {
    let input = "0x4000, 0x4000, 0x5800, 0x6400, 0x4400, 0x4400, 0x6400, 0x5800, 0x0000, 0x0000,";
    let mut strings = Vec::new();
    for value in input.split(", ") {
        let value: u16 =
            u16::from_str_radix(value.trim_start_matches("0x").trim_end_matches(","), 16).unwrap();
        let text = format!("{value:016b}");
        // println!("{}", text.replace("0", " ").replace("1", "█"));
        strings.push(text);
    }

    for y in (0..strings.len()).step_by(2) {
        for x in 0..strings[0].len() {
            let top_pixel = &strings[y as usize][x as usize..x as usize + 1] == "1";

            let bottom_pixel = &strings[y as usize + 1][x as usize..x as usize + 1] == "1";
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
