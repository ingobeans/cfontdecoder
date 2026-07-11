fn draw_char(input: &str) {
    let mut values = Vec::new();
    for value in input.split(", ") {
        let value: u16 =
            u16::from_str_radix(value.trim_start_matches("0x").trim_end_matches(","), 16).unwrap();
        values.push(value);
    }

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
    draw_char("0x4000, 0x4000, 0x5800, 0x6400, 0x4400, 0x4400, 0x6400, 0x5800, 0x0000, 0x0000,")
}
