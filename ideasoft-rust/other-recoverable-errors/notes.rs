// program that returns playable games on switch
#[derive(Clone)]
struct Player {
    name: String,
}

#[derive(Clone)]
struct SwitchGame {
    title: String,
    player: Option<Player>,
}

fn game_player_name (game: SwitchGame) -> String {
    let player = game.player;
    match player {
        Some(player) => player.name,
        None => return "No one".to_string(),
    }
}

fn print_game_status (game: SwitchGame) {
    println!("{} is playing {}", game_player_name(game.clone()), game.title);
}


fn main() {
    // it'd be good to go back to this and create a small program that uses Result<T, E>
    let game1 = SwitchGame {title: "Animal Crossing".to_string(), player: None};
    let game2 = SwitchGame {title: "Zelda: Breath of the Wild".to_string(), player: None};
    let p1 = Player{name: "Elle".to_string()};
    let p2 = Player{name: "Dad".to_string()};

    print_game_status(game1);
    print_game_status(game2);
    // next step is to build out functionality for changing the player status of the game
}
