use std::io::{Cursor, BufRead};
use std::cmp::Ordering;
use rand::Rng;
use clap::Parser;
use crate::Args;

// Helper function to create Args with custom values
fn create_args(max: u32, max_attempts: u32) -> Args {
    Args {
        max,
        max_attempts,
    }
}

#[test]
fn test_args_default_values() {
    let args = Args::parse_from::<_, &str>([]);
    assert_eq!(args.max, 100);
    assert_eq!(args.max_attempts, 10);
}

#[test]
fn test_args_custom_values() {
    let args = Args::parse_from(&["program", "-n", "50", "-a", "5"]);
    assert_eq!(args.max, 50);
    assert_eq!(args.max_attempts, 5);
}

#[test]
fn test_number_comparison() {
    let secret = 50;
    assert_eq!(45.cmp(&secret), Ordering::Less);
    assert_eq!(55.cmp(&secret), Ordering::Greater);
    assert_eq!(50.cmp(&secret), Ordering::Equal);
}

#[test]
fn test_input_validation() {
    let args = create_args(100, 10);
    
    // Test valid input
    let valid_input = "50\n";
    let mut cursor = Cursor::new(valid_input);
    let mut input = String::new();
    cursor.read_line(&mut input).unwrap();
    let result = input.trim().parse::<u32>();
    assert!(result.is_ok());
    let num = result.unwrap();
    assert!(num >= 1 && num <= args.max);

    // Test invalid input (non-numeric)
    let invalid_input = "abc\n";
    let mut cursor = Cursor::new(invalid_input);
    let mut input = String::new();
    cursor.read_line(&mut input).unwrap();
    let result = input.trim().parse::<u32>();
    assert!(result.is_err());

    // Test out of range input
    let out_of_range_input = "150\n";
    let mut cursor = Cursor::new(out_of_range_input);
    let mut input = String::new();
    cursor.read_line(&mut input).unwrap();
    let result = input.trim().parse::<u32>();
    assert!(result.is_ok());
    let num = result.unwrap();
    assert!(num > args.max);
}

#[test]
fn test_attempt_counting() {
    let args = create_args(100, 5);
    let mut attempts = 0;
    
    // Simulate 3 attempts
    for _ in 0..3 {
        attempts += 1;
    }
    
    assert_eq!(attempts, 3);
    assert!(attempts < args.max_attempts);
    
    // Simulate exceeding max attempts
    for _ in 0..3 {
        attempts += 1;
    }
    
    assert!(attempts > args.max_attempts);
}

#[test]
fn test_random_number_generation() {
    let args = create_args(100, 10);
    let secret_number = rand::thread_rng().gen_range(1..=args.max);
    assert!(secret_number >= 1 && secret_number <= args.max);
}

#[test]
fn test_game_win_condition() {
    let args = create_args(10, 5);
    let secret_number = 7;
    let mut attempts = 0;
    let mut won = false;

    // Simulate winning on first try
    let guess = 7;
    attempts += 1;
    if guess == secret_number {
        won = true;
    }

    assert!(won);
    assert_eq!(attempts, 1);
    assert!(attempts < args.max_attempts);
}

#[test]
fn test_game_loss_condition() {
    let args = create_args(10, 3);
    let secret_number = 7;
    let mut attempts = 0;
    let mut won = false;

    // Simulate losing by using all attempts
    for guess in [1, 2, 3] {
        attempts += 1;
        if guess == secret_number {
            won = true;
            break;
        }
    }

    assert!(!won);
    assert_eq!(attempts, args.max_attempts);
}

#[test]
fn test_input_validation_edge_cases() {
    let args = create_args(100, 10);
    
    // Test minimum valid input
    let min_input = "1\n";
    let mut cursor = Cursor::new(min_input);
    let mut input = String::new();
    cursor.read_line(&mut input).unwrap();
    let result = input.trim().parse::<u32>();
    assert!(result.is_ok());
    let num = result.unwrap();
    assert_eq!(num, 1);

    // Test maximum valid input
    let max_input = "100\n";
    let mut cursor = Cursor::new(max_input);
    let mut input = String::new();
    cursor.read_line(&mut input).unwrap();
    let result = input.trim().parse::<u32>();
    assert!(result.is_ok());
    let num = result.unwrap();
    assert_eq!(num, args.max);

    // Test empty input
    let empty_input = "\n";
    let mut cursor = Cursor::new(empty_input);
    let mut input = String::new();
    cursor.read_line(&mut input).unwrap();
    let result = input.trim().parse::<u32>();
    assert!(result.is_err());

    // Test input with whitespace
    let whitespace_input = "  50  \n";
    let mut cursor = Cursor::new(whitespace_input);
    let mut input = String::new();
    cursor.read_line(&mut input).unwrap();
    let result = input.trim().parse::<u32>();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 50);
}

#[test]
fn test_multiple_game_rounds() {
    let args = create_args(10, 5);
    let mut total_attempts = 0;
    let mut games_won = 0;
    let mut games_lost = 0;

    // Simulate multiple game rounds
    for secret_number in [3, 7, 9] {
        let mut attempts = 0;
        let mut won = false;

        // Simulate guesses until win or loss
        for guess in [1, 2, 3, 4, 5] {
            attempts += 1;
            if guess == secret_number {
                won = true;
                break;
            }
            if attempts >= args.max_attempts {
                break;
            }
        }

        total_attempts += attempts;
        if won {
            games_won += 1;
        } else {
            games_lost += 1;
        }
    }

    assert_eq!(games_won + games_lost, 3);
    assert!(total_attempts > 0);
} 