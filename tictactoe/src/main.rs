use std::io;
use std::io::BufRead;



fn main() {
    let mut matrix = [
        [0, 0, 0],
        [0, 0, 0],
        [0, 0, 0]
    ];

    // "O" = 1, "X" = -1
    let mut turn: i32 = 1;
    loop {
        print_board(matrix);
        let res = check_winner(&matrix);
        let game_ended = print_winner_message_if_any(res);

        if game_ended {
            reset_board(&mut matrix);
            print_board(matrix);
            turn = 1;
        }

        let mut buffer: String = String::new();
        let stdin = io::stdin();
        let mut handle = stdin.lock();

        let _ = handle.read_line(&mut buffer);

        let mut args = buffer.split_whitespace();
        let input_row = args.next();
        let input_col = args.next();

        match (input_row, input_col) {
            (Some(row), Some(col)) => {
                let parsed_row = row.parse::<i32>();
                let parsed_col = col.parse::<i32>();
                
                match (parsed_row, parsed_col) {
                    (Ok(rw), Ok(cl)) => {
                        make_move(&mut matrix, &mut turn, rw, cl);
                    },
                    _ => {
                        println!("\nThe arguments you typed are not integers!");
                    }
                }
            },
            _ => {
                println!("Invalid input!");
            }
        }
        
    }
}

fn reset_board(grid:&mut [[i32;3];3]) {
    for i in 0..=2 {
        for j in 0..=2 {
            grid[i][j] = 0;
        }
    }

    println!("\nBoard has been reset as game has ended.");
}

fn print_winner_message_if_any(res: i32) -> bool {
    match res {
        -1 => {
            println!("X wins!");
            return true;
        },
        1 => {
            println!("O wins!");
            return true;
        },
        0 => {
            println!("Nobody has won yet!");
            return false;
        },
        404 => {
            println!("Game Over! Nobody wins!");
            return true
        },
        _ => {
            println!("Shouldn't reach here.");
            return true;
        },
    }
}

fn check_winner(grid: &[[i32;3];3]) -> i32 {
    // Checks for winner (-1 = "X", 1 = "O")
    for i in (-1..=1).step_by(2) {
        for j in 0..=2 {
            if grid[j][0] == i && grid[j][1] == i && grid[j][2] == i {
                return i
            }
            if grid[0][j] == i && grid[1][j] == i && grid[2][j] == i {
                println!("asdadsa");
                return i
            }
        }
        if grid[0][0] == i && grid[1][1] == i && grid[2][2] == i {
            return i
        }
        if grid[0][2] == i && grid[1][1] == i && grid[2][0] == i {
            return i
        }
    }

    // No winners yet
    for i in 0..=2 {
        for j in 0..=2 {
            if grid[i][j] == 0 {
                return 0;
            }
        }
    }

    404

    
}

fn make_move(grid:&mut [[i32;3];3], turn: &mut i32, row: i32, col: i32) {
    if row < 0 || row > 2 || col < 0 || col > 2 {
        println!("\nOnly index 0 to 2 is accepted!");
    } else if grid[row as usize][col as usize] == 0 {
        println!("\nValid Move");
        grid[row as usize][col as usize] = *turn;
        *turn = -*turn;
        
    } else {
        println!("\nInvalid Move!\n");
    }
}

fn print_board(grid: [[i32;3];3]) {
    println!(" -----------");
    for row in grid {
        for col in row {
            let temp = if col == 1 {"o"} else if col == -1 {"x"} else {" "};
            print!("| {} ", temp);
        }
        println!("|\n -----------");
    }
}
