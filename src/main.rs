use std::{env, fs};

fn hex(byte: u8) -> u8 {
    assert!(
        byte < 16,
        "Byte provided must be in range 0-15. (range is end-inclusive)"
    );
    // maps values of 0-15 to 0-9 and a-f, differentiated by finding wether or not the value is greater than 10 mathematically
    byte / 10 * 0x31 + byte % 10 + 0x30
}
fn hex_capitalized(byte: u8) -> u8 {
    assert!(
        byte < 16,
        "Byte provided must be in range 0-15. (range is end-inclusive)"
    );
    // maps values of 0-15 to 0-9 and A-F, differentiated by finding wether or not the value is greater than 10 mathematically
    byte / 10 * 0x11 + byte % 10 + 0x30
}
fn bytes_to_hex(bytes: Vec<u8>, is_capitalized: bool) -> String {
    let mut hex_output: Vec<u8> = vec![];

    for byte in bytes {
        // bit mask for first and last 4 bits respectively for conversion to hex
        let (h1, h2) = (byte & 0b00001111, (byte & 0b11110000) >> 4);
        if is_capitalized {
            hex_output.push(hex_capitalized(h2));
            hex_output.push(hex_capitalized(h1));
        } else {
            hex_output.push(hex(h2));
            hex_output.push(hex(h1));
        }
    }

    String::from_utf8(hex_output).unwrap()
}
fn main() {
    fn colors_from_bytes(raw_bytes: &Vec<u8>) -> Vec<&str> {
        let mut color_map: Vec<&str> = vec![];
        for byte in raw_bytes {
            let byte = *byte;
            let (red, yellow, green, blue) = ("\x1b[31m", "\x1b[33m", "\x1b[32m", "\x1b[34m");
            if byte < 0x09 {
                // invalid before horizontal tab (special escape codes)
                color_map.push(red);
            } else if byte >= 0x09 && byte < 0x0e {
                // horizontal tab '\t', line feed '\n', vertical tab '\v', form feed '\f', and carriage return '\r' are valid
                color_map.push(yellow);
            } else if byte >= 0x0e && byte < 0x20 {
                // special control characters
                color_map.push(red);
            } else if byte >= 0x20 && byte < 0x7f {
                // standard human readable text
                color_map.push(green);
            } else if byte >= 0x7f && byte < 0xff {
                // 127 excluded due to being niche delete control character
                // invalid past 127 due to indeterminate utf encoding method
                color_map.push(red);
            } else {
                // special for full byte
                color_map.push(blue);
            }
        }
        color_map
    }

    // acquire args, verify, then read provided file
    let args = env::args().collect::<Vec<String>>();
    assert!(args.len() > 1, "Please provide file path.");
    let path = &args[1];
    let input = match fs::read(path) {
        Ok(file) => file,
        Err(error) => panic!("{error}"),
    }.to_vec();

    // create color map on base data to remove complexity of mapping on formatted text
    let colors = colors_from_bytes(&input);

    // map invalid characters and escape/control characters to '.'
    let input_text: Vec<u8> = input
        .iter()
        .map(|byte| {
            if *byte >= 0x7f || *byte < 0x20 {
                0x2e
            } else {
                *byte
            }
        }).collect();
    
    // convert to hex
    let hex = bytes_to_hex(input.to_vec(), false);
    let hex_bytes = hex.as_bytes().to_vec();
    
    let mut i: usize = 0;
    while i < input.len() {
        // to account for last iteration not necessarily being 16 characters long 
        let window = if input.len() - i > 15 {
            16
        } else {
            input.len() - i
        };

        // account for hex having exactly 2x more character than the text line
        // convert to string then map to include coloring from colors and '|' 4 byte seperator
        let hex_line = &hex_bytes[2 * i..2 * i + (2 * window)];
        let mut j = 0; let hex_formatted: String = String::from_utf8(hex_line.to_vec())
            .unwrap()
            .chars()
            .map(|char| {
                if j % 2 == 0 {
                    let mut o = "\x1b[0m".to_string();
                    if j % 4 == 0 && j != 0 {
                        o = "\x1b[0m|".to_string();
                    }
                    o.push_str(colors[(j / 2) + i]);
                    j += 1;
                    o.push(char);
                    o
                } else {
                    j += 1;
                    char.to_string()
                }
            }).collect();

        // convert text to string then map to include coloring from colors
        let mut j = 0; let text_formatted: String = String::from_utf8(input_text[i..i + (window)].to_vec())
            .unwrap()
            .chars()
            .map(|char| {
                let mut o = colors[j + i].to_string();
                j += 1;
                o.push(char);
                o
            }).collect();

        // account for width added by coloring ANSI escape sequences
        let spaces_to_pad =
            40 + (hex_formatted.len() - (hex_line.len() + ((hex_line.len() + 2) / 4 - 1)));
        
        // lastly, print line
        println!(
            "{:08x}: {:<spaces_to_pad$}  {:}\x1b[0m",
            i,
            format!("{hex_formatted:<40}"),
            text_formatted
        );
        
        i += 16
    }
}
