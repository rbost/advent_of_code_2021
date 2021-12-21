use std::time::Instant;
use std::{collections::BTreeMap, env};

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
struct ScoreState {
    positions: [u16; 2],
    scores: [u16; 2],
    player: u8,
}

fn compute_outcomes_simple_aux(
    cache: &mut BTreeMap<ScoreState, [u64; 2]>,
    state: ScoreState,
) -> [u64; 2] {
    if let Some(r) = cache.get(&state) {
        *r
    } else {
        let mut n_wins = [0, 0];
        (1..=3).for_each(|d1| {
            (1..=3).for_each(|d2| {
                (1..=3).for_each(|d3| {
                    let offset = d1 + d2 + d3;
                    let p = state.player as usize;
                    let new_pos = (state.positions[p] + (offset as u16)) % 10;
                    let new_score = state.scores[p] + new_pos + 1;

                    if new_score >= 21 {
                        n_wins[p] += 1;
                    } else {
                        let mut new_state = state.clone();
                        new_state.scores[p] = new_score;
                        new_state.positions[p] = new_pos;
                        new_state.player = if p == 0 { 1 } else { 0 };
                        let sub_wins = compute_outcomes_aux(cache, new_state);

                        n_wins[0] += sub_wins[0];
                        n_wins[1] += sub_wins[1];
                    }
                });
            });
        });

        cache.insert(state, n_wins);
        n_wins
    }
}

fn compute_outcomes_aux(cache: &mut BTreeMap<ScoreState, [u64; 2]>, state: ScoreState) -> [u64; 2] {
    if let Some(r) = cache.get(&state) {
        *r
    } else {
        let mut n_wins = [0, 0];

        let dices_outcomes = [0, 0, 0, 1, 3, 6, 7, 6, 3, 1];
        dices_outcomes
            .iter()
            .enumerate()
            .filter(|(_, &c)| c > 0)
            .for_each(|(offset, count)| {
                let p = state.player as usize;
                let new_pos = (state.positions[p] + (offset as u16)) % 10;
                let new_score = state.scores[p] + new_pos + 1;

                if new_score >= 21 {
                    n_wins[p] += count;
                } else {
                    let mut new_state = state.clone();
                    new_state.scores[p] = new_score;
                    new_state.positions[p] = new_pos;
                    new_state.player = if p == 0 { 1 } else { 0 };
                    let sub_wins = compute_outcomes_aux(cache, new_state);

                    n_wins[0] += count * sub_wins[0];
                    n_wins[1] += count * sub_wins[1];
                }
            });
        cache.insert(state, n_wins);
        n_wins
    }
}

fn compute_outcomes(state: ScoreState) -> [u64; 2] {
    let mut cache = BTreeMap::<ScoreState, [u64; 2]>::new();

    compute_outcomes_aux(&mut cache, state)
}

fn compute_outcomes_simple(state: ScoreState) -> [u64; 2] {
    let mut cache = BTreeMap::<ScoreState, [u64; 2]>::new();

    compute_outcomes_simple_aux(&mut cache, state)
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let positions = [
        args[1].parse::<u16>().unwrap() - 1,
        args[2].parse::<u16>().unwrap() - 1,
    ];

    let origin = ScoreState {
        positions,
        scores: [0, 0],
        player: 0,
    };

    let origin_clone = origin.clone();

    let current = Instant::now();
    let outcomes = compute_outcomes(origin);
    let duration = current.elapsed();
    println!("{:?}, duration {:?}", outcomes, duration);

    let current = Instant::now();
    let outcomes = compute_outcomes_simple(origin_clone);
    let duration = current.elapsed();
    println!("{:?}, duration {:?}", outcomes, duration);
}
