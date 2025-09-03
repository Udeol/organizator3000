// This is the main API library for the application.

// We make the mealz plug public so clients can access it.
pub use orgzr_mealz as mealz;

use mealz::MealzPlug;

/// The main struct for the core application logic.
/// It holds all the different plugs and provides access to them.
#[derive(Default)]
pub struct Core {
    // Each plug is a public field, making the API clear and modular.
    // e.g., `core.mealz.add_card(...)`
    pub mealz: MealzPlug,
    // Future plugs will be added here, e.g.,
    // pub budgetz: BudgetzPlug,
}

impl Core {
    /// Creates a new instance of the Core engine.
    pub fn new() -> Self {
        Self {
            mealz: MealzPlug::new(),
            // ... initialize other plugs here
        }
    }
}
