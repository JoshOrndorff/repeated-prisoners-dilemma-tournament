//! Plays different strategies of the repeated prisoners dilemma against each other
//! as inspired by this veritasium video https://www.youtube.com/watch?v=mScpHTIi-kM

/// The number of rounds that the prisoners' dilemma will be repeated.
pub const NUM_TURNS: usize = 200;

/// The payout that both players get when they cooperate
pub const COOPERATE_PAYOUT: isize = 10;

/// The payout that both players get when they defect
pub const DEFECT_PAYOUT: isize = 2;

/// The payout you get when you narc out your opponent
pub const NARC_OUT_OPPONENT_PAYOUT: isize = 20;

/// The payout you get when your opponent narcs you out
pub const GOT_NARCED_OUT_PAYOUT: isize = -5;

/// The two strategies in the single prisoners' dilemma.
#[derive(Debug)]
pub enum CooperateOrDefect {
    Cooperate,
    Defect,
}

use std::marker::PhantomData;

use CooperateOrDefect::*;

/// A strategy that a player will follow when playing the repeated prisoners' dilemma
/// against the same player.
pub trait Strategy {
    const NAME: &'static str;

    /// Calculate your strategy (cooperate or defect) in the next iteration of the repeated prisoners' dilemma.
    ///
    /// Assumes that the slices are the same length.
    fn next_move(
        my_moves: &[CooperateOrDefect],
        their_moves: &[CooperateOrDefect],
    ) -> CooperateOrDefect;
}

/// One of the simplest strategies
pub struct AlwaysCooperate;

impl Strategy for AlwaysCooperate {
    const NAME: &'static str = "Always Cooperate";

    fn next_move(
        _my_moves: &[CooperateOrDefect],
        _their_moves: &[CooperateOrDefect],
    ) -> CooperateOrDefect {
        Cooperate
    }
}

/// One of the simplest strategies
pub struct AlwaysDefect;

impl Strategy for AlwaysDefect {
    const NAME: &'static str = "Always Defect";

    fn next_move(
        _my_moves: &[CooperateOrDefect],
        _their_moves: &[CooperateOrDefect],
    ) -> CooperateOrDefect {
        Defect
    }
}

/// An instance of the repeated prisoners' dilemma. The same two players play against each other
/// for several rounds. In each round they are able to choose whether to cooperate or defect, and they
/// have knowledge of the entire history of the game.
pub struct RepeatedPrisonersDilemma<P1, P2> {
    // Hopefully in the wasm-friendly future, we can make
    // the strategies wasm blobs that are instances instead of type parameters??
    /// History of player1's moves
    player_1_moves: Vec<CooperateOrDefect>,
    ///History of player2's moves
    player_2_moves: Vec<CooperateOrDefect>,
    _ph_data: PhantomData<(P1, P2)>,
}

impl<P1, P2> RepeatedPrisonersDilemma<P1, P2>
where
    P1: Strategy,
    P2: Strategy,
{
    fn new() -> Self {
        Self {
            player_1_moves: Vec::new(),
            player_2_moves: Vec::new(),
            _ph_data: PhantomData,
        }
    }

    fn play_next_round(&mut self) {
        let p1_move = P1::next_move(&self.player_1_moves, &self.player_2_moves);
        let p2_move = P2::next_move(&self.player_2_moves, &self.player_2_moves);

        println!("({:?}, {:?})", p1_move, p2_move);

        self.player_1_moves.push(p1_move);
        self.player_2_moves.push(p2_move);
    }

    fn calculate_score(&self) -> (isize, isize) {
        self.player_1_moves
            .iter()
            .zip(&self.player_2_moves)
            .fold((0, 0), |(p1, p2), moves| match moves {
                (Cooperate, Cooperate) => (p1 + COOPERATE_PAYOUT, p2 + COOPERATE_PAYOUT),
                (Cooperate, Defect) => (p1 + GOT_NARCED_OUT_PAYOUT, p2 + NARC_OUT_OPPONENT_PAYOUT),
                (Defect, Cooperate) => (p1 + NARC_OUT_OPPONENT_PAYOUT, p2 + GOT_NARCED_OUT_PAYOUT),
                (Defect, Defect) => (p1 + DEFECT_PAYOUT, p2 + DEFECT_PAYOUT),
            })
    }
}

fn main() {
    println!(
        "Playing strategy {} against {}",
        AlwaysCooperate::NAME,
        AlwaysDefect::NAME
    );

    let mut cooperate_vs_defect = RepeatedPrisonersDilemma::<AlwaysCooperate, AlwaysDefect>::new();

    for _ in 0..NUM_TURNS {
        cooperate_vs_defect.play_next_round();
    }

    println!("Final score: {:?}", cooperate_vs_defect.calculate_score());
}
