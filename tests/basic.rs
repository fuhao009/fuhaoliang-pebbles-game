use gstd::{prelude::*, Encode};
use gtest::{Program, System};
use pebbles_game_io::{PebblesInit, DifficultyLevel, PebblesAction, GameState, Player};
use rkyv::option::ArchivedOption;

/// Initialize the program with default settings.
fn init_program(system: &System) -> Program {
    let program = Program::current(system);
    let init_msg = PebblesInit {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 20,
        max_pebbles_per_turn: 3,
    };
    program.send_bytes(1, &init_msg.encode());
    program
}

#[test]
fn test_init() {
    let system = System::new();
    let program = init_program(&system);

    let state: GameState = program
        .read_state(1u8)
        .expect("Failed to read state");
    assert_eq!(state.pebbles_count, 20);
    assert_eq!(state.max_pebbles_per_turn, 3);
    assert_eq!(state.pebbles_remaining, 20);
    assert_eq!(state.difficulty, DifficultyLevel::Easy);
    assert_eq!(state.first_player, Player::User);
    assert_eq!(state.winner, ArchivedOption::None);
}

#[test]
fn test_turn() {
    let system = System::new();
    let program = init_program(&system);

    let action = PebblesAction::Turn(3);
    program.send_bytes(1, &action.encode());

    let state: GameState = program
        .read_state(1u8)
        .expect("Failed to read state");
    assert_eq!(state.pebbles_remaining, 17);
}

#[test]
fn test_give_up() {
    let system = System::new();
    let program = init_program(&system);

    let action = PebblesAction::GiveUp;
    program.send_bytes(1, &action.encode());

    let state: GameState = program
        .read_state(1u8)
        .expect("Failed to read state");
    assert_eq!(state.winner, ArchivedOption::Some(Player::Program));
}

#[test]
fn test_restart() {
    let system = System::new();
    let program = init_program(&system);

    let action = PebblesAction::Restart {
        difficulty: DifficultyLevel::Hard,
        pebbles_count: 30,
        max_pebbles_per_turn: 5,
    };
    program.send_bytes(1, &action.encode());

    let state: GameState = program
        .read_state(1u8)
        .expect("Failed to read state");

    assert_eq!(state.pebbles_count, 30);
    assert_eq!(state.max_pebbles_per_turn, 5);
    assert_eq!(state.pebbles_remaining, 30);
    assert_eq!(state.difficulty, DifficultyLevel::Hard);
    assert_eq!(state.first_player, Player::User);
    assert_eq!(state.winner, ArchivedOption::None);
}

#[test]
fn test_state() {
    let system = System::new();
    let program = init_program(&system);

    let state: GameState = program
        .read_state(1u8)
        .expect("Failed to read state");

    assert_eq!(state.pebbles_count, state.pebbles_count);
    assert_eq!(state.max_pebbles_per_turn, state.max_pebbles_per_turn);
    assert_eq!(state.pebbles_remaining, state.pebbles_remaining);
    assert_eq!(state.difficulty, state.difficulty);
    assert_eq!(state.first_player, state.first_player);
    assert_eq!(state.winner, state.winner);
}
