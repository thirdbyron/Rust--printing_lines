fn main() {
    let title = "Trying printing in RUST";
    println!(
        " 
 {:-^50}",
        title
    );

    let mut bar_counts: u8 = 0;

    while bar_counts != 2 {
        let bar = "|";
        println!("{left: <25} {right: >26}", left = bar, right = bar);
        bar_counts += 1;
    }

    let title = "Choose your game";
    println!(
        "{: ^50}
",
        title
    );

    let (first_game, second_game) = ("Hallo", "Star Wars");
    println!(
        " {: <22} <-> {: >22}
",
        first_game, second_game
    );

    println!(
        "{:-^52}

{: ^50}
",
        "", "Just type it in your command line:"
    );

    let mut game_for_play = String::new();

    std::io::stdin()
        .read_line(&mut game_for_play)
        .expect("You got failure!");

    match game_for_play.trim() {
        "Hallo" => println!("You choice is \"{}\". Start play!", game_for_play.trim()),
        "Star Wars" => println!("You choice is \"{}\". Start play!", game_for_play.trim()),
        _ => println!(
            "There is not that game, sorry."
        ),
    }
}
