use clap::{Parser, Subcommand};
use orgzr_core::Core;
use std::collections::HashSet;

/// A modular assistant to organize your daily chaos.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Commands for the Mealz plug.
    Mealz(MealzArgs),
}

#[derive(Parser, Debug)]
struct MealzArgs {
    #[command(subcommand)]
    command: MealzCommands,
}

#[derive(Subcommand, Debug)]
enum MealzCommands {
    /// Add a new meal card.
    Add {
        /// The name of the meal.
        name: String,

        /// A comma-separated list of tags.
        #[arg(short, long)]
        tags: Option<String>,

        /// A comma-separated list of ingredients.
        #[arg(short, long)]
        ingredients: Option<String>,
    },
    /// List all existing meal cards.
    List,
}

fn main() {
    let cli = Cli::parse();
    // In a real application, we would load the state of the core from a database here.
    let mut core = Core::new();

    match cli.command {
        Commands::Mealz(args) => match args.command {
            MealzCommands::Add {
                name,
                tags,
                ingredients,
            } => {
                // Parse comma-separated strings into HashSets
                let tags_set = tags
                    .map(|s| s.split(',').map(String::from).collect())
                    .unwrap_or_default();
                let ingredients_set = ingredients
                    .map(|s| s.split(',').map(String::from).collect())
                    .unwrap_or_default();

                match core.mealz.add_card(name, tags_set, ingredients_set) {
                    Ok(card) => println!(
                        "✅ Card '{}' (ID: {}) created successfully.",
                        card.name, card.id
                    ),
                    Err(e) => eprintln!("❌ Error: {}", e),
                }
                // In a real application, we would save the state of the core to a database here.
            }
            MealzCommands::List => {
                let cards = core.mealz.list_cards();
                if cards.is_empty() {
                    println!("No meal cards found.");
                } else {
                    println!("--- Meal Card Library ---");
                    for card in cards {
                        println!("[{}] {}", card.id, card.name);
                        // To display HashSet content nicely, we can iterate and join
                        let tags_str: Vec<String> = card.tags.iter().cloned().collect();
                        let ingredients_str: Vec<String> =
                            card.ingredients.iter().cloned().collect();
                        println!("  Tags: {}", tags_str.join(", "));
                        println!("  Ingredients: {}", ingredients_str.join(", "));
                        println!("-------------------------");
                    }
                }
            }
        },
    }
}
