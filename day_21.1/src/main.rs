use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut positions = [
        args[1].parse::<u16>().unwrap() - 1,
        args[2].parse::<u16>().unwrap() - 1,
    ];
    let mut scores = [0u32, 0u32];
    let mut dice_value = 0u16;
    let mut n_rolls = 0usize;

    let mut player = 0;
    while scores[0] < 1000 && scores[1] < 1000 {
        let offset1 = (dice_value + 1) % 100;
        dice_value = (dice_value + 1) % 100;
        let offset2 = (dice_value + 1) % 100;
        dice_value = (dice_value + 1) % 100;
        let offset3 = (dice_value + 1) % 100;
        dice_value = (dice_value + 1) % 100;

        n_rolls += 3;

        positions[player] = (positions[player] + offset1 + offset2 + offset3) % 10;
        scores[player] += (positions[player] + 1) as u32;

        println!(
            "Player {} rolls {}+{}+{} and moves to space {} for a total score of {}.",
            player + 1,
            offset1,
            offset2,
            offset3,
            positions[player] + 1,
            scores[player]
        );

        player = if player == 0 { 1 } else { 0 };
    }

    println!("Looser's score: {}", scores[0].min(scores[1]));
    println!("Number of rolls: {}", n_rolls);
    println!(
        "Final score: {}",
        n_rolls * (scores[0].min(scores[1]) as usize)
    );
}
