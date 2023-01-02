use std::io;

fn main() {
    loop {
        // Startar nytt spel
        start_game();

        // När spelet är över, så frågas användaren om man vill starta nytt spel
        println!("Do you want to start a new game? Y/N \n");

        // Läs in användarens input
        let mut user_input: String = String::new();

        _ = io::stdin().read_line(&mut user_input);

        // Vi trimmar inputen från tomma spaces och newlines, samt omvandlar den till lower-case för att undvika tjafs med upper/lower case på Y/y.
        if user_input.trim().to_lowercase() == "y" {
            continue; // Om användaren vill starta nytt spel, så fortsätter vi loopen i en ny iteration
                      // Start new game
        } else {
            // Annars returnar vi så avslutas programmet.
            return;
        }
    }
}

fn start_game() {
    let mut current_player: u8 = 1;

    // Om en cell har siffran 0 så är den tom. Om den har siffran 1, så har player 1 markerat och om den har player 2 så har spelare 2 markerat.
    let mut board: [u8; 9] = [0, 0, 0, 0, 0, 0, 0, 0, 0];

    println!("Welcome to Ahmad's Tic Tac Toe game!");

    loop {
        render_board(&board);

        let mut user_input: String = String::new();
        println!(
            "Player {}'s turn, please enter a cell number from 0-8 and press Enter!",
            current_player
        );
        let _ = io::stdin()
            .read_line(&mut user_input)
            .expect("Something went wrong!");

        let input_int = user_input.trim().parse::<usize>().unwrap();

        // Kolla om cellen redan är tagen
        if board[input_int] != 0 {
            println!("The cell is already taken!");
            continue;
        }

        // Uppdatera brädan
        board[input_int] = current_player;

        // Kolla om någon har vunnit

        let game_status = check_win(&board);

        if game_status {
            render_board(&board);
            println!("Congratulations! Player {} has won!", current_player);
            return;
        }

        // Kolla om det är oavgjort
        let mut count: u8 = 0;
        for i in 0..board.len() {
            if board[i] != 0 {
                count += 1;
            }
        }

        if count == 9 {
            render_board(&board);
            println!("The game is a draw!");
            return;
        }

        // Om ingen har vunnit, och det inte är oavgjort så switchar vi spelare och fortsätter spelet!
        switch_players(&mut current_player);
    }
}

fn render_board(board: &[u8; 9]) {
    println!(
        "|{}|{}|{}|",
        get_sign(&board[0]),
        get_sign(&board[1]),
        get_sign(&board[2])
    );
    println!(
        "|{}|{}|{}|",
        get_sign(&board[3]),
        get_sign(&board[4]),
        get_sign(&board[5])
    );
    println!(
        "|{}|{}|{}|",
        get_sign(&board[6]),
        get_sign(&board[7]),
        get_sign(&board[8])
    );
}

fn get_sign(cell: &u8) -> &str {
    if *cell == 0 {
        return " ";
    } else if *cell == 1 {
        return "X";
    } else {
        return "O";
    }
}

fn check_win(board: &[u8; 9]) -> bool {
    // First row

    if board[0] == board[1] && board[0] == board[2] && board[0] != 0 {
        return true;
    }

    // Second row

    if board[3] == board[4] && board[3] == board[5] && board[3] != 0 {
        return true;
    }

    // Third row

    if board[6] == board[7] && board[6] == board[8] && board[6] != 0 {
        return true;
    }

    // First column

    if board[0] == board[3] && board[0] == board[6] && board[0] != 0 {
        return true;
    }

    // Second column

    if board[1] == board[4] && board[1] == board[7] && board[1] != 0 {
        return true;
    }

    // Third column

    if board[2] == board[5] && board[2] == board[8] && board[2] != 0 {
        return true;
    }

    // First diagonal

    if board[0] == board[4] && board[0] == board[8] && board[0] != 0 {
        return true;
    }

    // Second diagonal

    if board[2] == board[4] && board[2] == board[6] && board[2] != 0 {
        return true;
    }

    return false;
}

fn switch_players(current_player: &mut u8) {
    if *current_player == 1 {
        *current_player = 2;
    } else {
        *current_player = 1;
    }
}
