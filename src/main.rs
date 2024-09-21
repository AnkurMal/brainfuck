use std::{env, fs, io};

macro_rules! printf {
    ($($arg:tt)*) => {{
        use std::io::Write;
        print!($($arg)*);
        std::io::stdout().flush().expect("ERROR: Couldn't flush to output stream.");
    }};
}

/// parse_parentheses(file, level, start, end, index)
macro_rules! parse_parentheses {
    ($file: ident, $level: ident, $start: literal, $end: literal ,$index: ident) => {{
        loop {
            let char = $file.chars().nth($index).expect("ERROR: Tried to get character at invalid index.");
            match char {
                $start => $level += 1,
                $end   => $level -= 1,
                _   => ()
            }
            if $level==0 {
                break;
            }
            match $start=='[' { 
               true  => $index += 1,
               false => $index -= 1,
            }
        }
    }};
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => panic!("ERROR: No file was given."),
        3 => panic!("ERROR: Only one file name was expected, multiple were given."),
        _ => ()
    }

    let file = match fs::read_to_string(&args[1]) {
            Ok(file) => file,
            Err(e)    =>  panic!("ERROR: {e}")
        };

    let mut mem: [u8; 30000] = [0; 30000];
    let (mut index, mut i) = (0, 0);
    
    while i<file.len() {
        match file.chars().nth(i).expect("ERROR: Tried to get character at invalid index.") {
            '>' => index += 1,
            '<' => {
                if index==0 {
                    panic!("ERROR: Memory underflow!");
                }
                index -= 1;
            },
            '+' => mem[index] += 1,
            '-' => mem[index] -= 1,
            '.' => printf!("{}", mem[index] as char),
            ',' => {
                let mut inp = String::new();
                io::stdin().read_line(&mut inp).expect("ERROR: Failed to read line.");

                let inp = inp.trim();
                mem[index] = inp.chars().nth(0).expect("ERROR: Tried to get character at invalid index.") as u8;
            }
            '[' => {
                let mut level = 0;
                match mem[index] {
                    0 => parse_parentheses!(file, level, '[', ']', i),
                    _ => ()
                }
            }
            ']' => {
                let mut level = 0;
                match mem[index] {
                    0 => (),
                    _ => parse_parentheses!(file, level, ']', '[', i)
                }
            }
            _ => ()
        }

        i += 1;
    }
}
