use std::{env, fs, io};
use io::Write;

macro_rules! printf {
    ($($arg:tt)*) => {{
        print!($($arg)*);
        io::stdout().flush().expect("ERROR: Couldn't flush to output stream.");
    }};
}

/// parse_parentheses(file, start, end, index)
macro_rules! parse_parentheses {
    ($file: ident, $start: literal, $end: literal ,$index: ident) => {{
        let mut level = 0;
        
        loop {
            let char = $file.chars().nth($index).expect("ERROR: Tried to get character at invalid index.");
            match char {
                $start => level += 1,
                $end   => level -= 1,
                _   => ()
            }
            if level==0 {
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
    
    const CELL_SIZE: usize = 30000;
    let mut mem: [u8; CELL_SIZE] = [0; CELL_SIZE];
    let (mut index, mut i) = (0, 0);
    
    while i<file.len() {
        match file.chars().nth(i).expect("ERROR: Tried to get character at invalid index.") {
            '>' => {
                if index==CELL_SIZE {
                    panic!("ERROR: Memory out of bounds (overflow), exceeded the memory size of {CELL_SIZE} cells.");
                }
                index += 1
            },
            '<' => {
                if index==0 {
                    panic!("ERROR: Memory out of bounds (underflow), undershot the memory size of {CELL_SIZE} cells.");
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
                match mem[index] {
                    0 => parse_parentheses!(file, '[', ']', i),
                    _ => ()
                }
            }
            ']' => {
                match mem[index] {
                    0 => (),
                    _ => parse_parentheses!(file, ']', '[', i)
                }
            }
            _ => ()
        }

        i += 1;
    }
}
