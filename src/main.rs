use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let w = parse_input!(input_line, i8);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let h = parse_input!(input_line, i8);

    let mut board: Vec<Vec<u8>> = Vec::new();
    for _ in 0..h as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        board.push(input_line.trim_matches('\n').as_bytes().to_vec());
    }

    for y in 0..h {
        for x in 0..w {
            if board[y as usize][x as usize] == b'x' {
                continue;
            }

            let mut number = 0;
            number += increment_if_valid(&mut board, y-1, x-1);
            number += increment_if_valid(&mut board, y-1, x);
            number += increment_if_valid(&mut board, y-1, x+1);
            number += increment_if_valid(&mut board, y, x+1);
            number += increment_if_valid(&mut board, y, x-1);
            number += increment_if_valid(&mut board, y+1, x+1);
            number += increment_if_valid(&mut board, y+1, x);
            number += increment_if_valid(&mut board, y+1, x-1);

            board[y as usize][x as usize] += number;
        }
    }

    for vec in &board {
        let mut row_str = String::new();
        for b in vec {
            if *b == b'x' || *b == b'.' {
                row_str.push('.');
            } else {
                row_str.push(((*b - b'.') + b'0') as char);
            }
        }
        println!("{row_str}");
    }

}

fn increment_if_valid(
    board: &mut Vec<Vec<u8>>,
    y: i8,
    x: i8,
) -> u8 {
    if y >= 0 && x >= 0 && (y as usize) < board.len() && (x as usize) < board[0].len() && board[y as usize][x as usize] == b'x' {
        1
    } else {
        0
    }
}