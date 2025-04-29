use std::io::{self, Write};

mod game_core;
mod point_system;
mod encryption;

use game_core::{load_questions, get_categories, get_random_question};
use point_system::SecurePointSystem;

fn main() {
    // Load questions from JSON file
    let questions = load_questions();
    let categories = get_categories(&questions);

    println!("Type 'play' to start the game:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if !input.trim().eq_ignore_ascii_case("play") {
        println!("Invalid command. Please type 'play' to start.");
        return;
    }

    // Initialize secure points system with a key (example: 0xAB)
    let mut points_system = SecurePointSystem::new(0xAB);

    // Main game loop
    loop {
        // Show categories
        println!("\nPick a category:");
        for (i, category) in categories.iter().enumerate() {
            println!("{}. {}", i + 1, category);
        }

        // Read category choice
        let category = loop {
            let mut cat_input = String::new();
            io::stdin().read_line(&mut cat_input).unwrap();
            let cat_input = cat_input.trim();

            let category = if let Ok(num) = cat_input.parse::<usize>() {
                if num > 0 && num <= categories.len() {
                    Some(&categories[num - 1])
                } else {
                    None
                }
            } else {
                categories.iter().find(|c| c.eq_ignore_ascii_case(cat_input))
            };

            if let Some(category) = category {
                break category;
            } else {
                println!("Invalid category choice. Please enter a valid number or name:");
            }
        };

        // Pick a random question from the category
        if let Some(question) = get_random_question(&questions, category.as_str()) {
            println!("\nQuestion: {}", question.question);
            for (i, option) in question.options.iter().enumerate() {
                println!("{}. {}", i + 1, option);
            }

            // Answer/hint loop
            loop {
                println!("\nYour answer (number), or type 'hint' to use a hint for {} points.", question.hint_cost);
                print!("> ");
                io::stdout().flush().unwrap();

                let mut answer_input = String::new();
                io::stdin().read_line(&mut answer_input).unwrap();
                let answer_input = answer_input.trim();

                if answer_input.eq_ignore_ascii_case("hint") {
                    if points_system.get_points() >= question.hint_cost {
                        points_system.use_points(question.hint_cost);
                        if let Some(hint_text) = &question.hint {
                            println!("Hint: {}", hint_text);
                        } else {
                            println!("No hint available for this question.");
                        }
                        println!("(You have {} points left)", points_system.get_points());
                    } else {
                        println!("Sorry, you don't have enough points for a hint.");
                    }
                    continue; // Ask again for answer
                }

                let answer: usize = match answer_input.parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Please enter a valid number or 'hint'.");
                        continue;
                    }
                };

                if answer == question.answer + 1 {
                    points_system.add_points(question.points);
                    println!("Correct! You earned {} points.", question.points);
                } else {
                    println!("Incorrect! The correct answer was: {}", question.options[question.answer]);
                }

                println!("Your total points: {}", points_system.get_points());
                break;
            }
        } else {
            println!("No questions available for the selected category.");
        }

        // Ask if the player wants to play again
        println!("\nDo you want to play again? (yes/no):");
        let mut again_input = String::new();
        io::stdin().read_line(&mut again_input).unwrap();
        let again_input = again_input.trim().to_lowercase();

        if again_input != "yes" && again_input != "y" {
            println!("Thanks for playing! Your final score is: {}", points_system.get_points());
            break;
        }
    }
}
