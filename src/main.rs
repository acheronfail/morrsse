mod cli;

use cli::Args;

fn morse(ch: char) -> Option<&'static str> {
    match ch {
        '0' => Some("-----"),
        '1' => Some(".----"),
        '2' => Some("..---"),
        '3' => Some("...--"),
        '4' => Some("....-"),
        '5' => Some("....."),
        '6' => Some("-...."),
        '7' => Some("--..."),
        '8' => Some("---.."),
        '9' => Some("----."),
        'A' | 'a' => Some(".-"),
        'B' | 'b' => Some("-..."),
        'C' | 'c' => Some("-.-."),
        'D' | 'd' => Some("-.."),
        'E' | 'e' => Some("."),
        'F' | 'f' => Some("..-."),
        'G' | 'g' => Some("--."),
        'H' | 'h' => Some("...."),
        'I' | 'i' => Some(".."),
        'J' | 'j' => Some(".---"),
        'K' | 'k' => Some("-.-"),
        'L' | 'l' => Some(".-.."),
        'M' | 'm' => Some("--"),
        'N' | 'n' => Some("-."),
        'O' | 'o' => Some("---"),
        'P' | 'p' => Some(".--."),
        'Q' | 'q' => Some("--.-"),
        'R' | 'r' => Some(".-."),
        'S' | 's' => Some("..."),
        'T' | 't' => Some("-"),
        'U' | 'u' => Some("..-"),
        'V' | 'v' => Some("...-"),
        'W' | 'w' => Some(".--"),
        'X' | 'x' => Some("-..-"),
        'Y' | 'y' => Some("-.--"),
        'Z' | 'z' => Some("--.."),
        ',' => Some("--..--"),
        '.' => Some(".-.-.-"),
        ' ' => Some("/"),
        _ => None,
    }
}

fn print(s: impl AsRef<str>, spoken: bool, show_hidden: bool) {
    s.as_ref().chars().for_each(|ch| match morse(ch) {
        Some(s) => {
            if spoken {
                print!(
                    "{} ",
                    s.chars()
                        .map(|ch| match ch {
                            '.' => "dit",
                            '-' => "daw",
                            '/' => "/",
                            _ => unreachable!("{}", ch),
                        })
                        .collect::<Vec<_>>()
                        .join("-")
                );
            } else {
                print!("{} ", s);
            }
        }
        None => {
            if show_hidden {
                print!("<{}> ", ch);
            } else {
                print!(" ");
            }
        }
    });
    println!();
}

fn main() {
    let args = Args::parse();
    args.phrases
        .iter()
        .for_each(|word| print(word, args.spoken, args.show_hidden));
}
