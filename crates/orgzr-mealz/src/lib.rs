use rand::prelude::*;
use std::collections::{HashMap, HashSet};

// --- Data Structures ---

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Card {
    pub id: u64,
    pub name: String,
    pub tags: HashSet<String>,
    pub ingredients: HashSet<String>,
    pub max_batch_size: u8,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum FilterMode {
    All,
    #[default]
    Any,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum BatchMode {
    #[default]
    Allow, // Include both batchable and non-batchable cards
    Only,    // Include only cards where max_batch_size > 1
    Prevent, // Exclude all cards where max_batch_size > 1
}

#[derive(Debug, Default)]
pub struct CardFilters {
    pub name_contains: String,
    pub tag_filters: HashSet<String>,
    pub tag_mode: FilterMode,
    pub ingredient_filters: HashSet<String>,
    pub ingredient_mode: FilterMode,
    pub allow_jokers: bool,
    pub batch_mode: BatchMode,
}

#[derive(Debug, Default)]
pub struct PlanConstraints {
    pub number_of_meals: u8,
    pub filters: CardFilters,
    pub no_consecutive: bool,
    pub max_repeats_per_plan: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DaySlot {
    MondayLunch,
    MondayDinner,
    TuesdayLunch,
    TuesdayDinner,
    WednesdayLunch,
    WednesdayDinner,
    ThursdayLunch,
    ThursdayDinner,
    FridayLunch,
    FridayDinner,
    SaturdayLunch,
    SaturdayDinner,
    SundayLunch,
    SundayDinner,
}

#[derive(Debug)]
pub enum PlanSchedule {
    Ideas(Vec<Card>),
    Weekly(HashMap<DaySlot, Card>),
}

#[derive(Debug)]
pub struct MealPlan {
    pub schedule: PlanSchedule,
}

#[derive(Debug)]
pub struct GenerationResult {
    pub plan: MealPlan,
    pub warnings: Vec<String>,
}

// --- Service Implementation ---

#[derive(Default)]
pub struct MealzPlug {
    cards: Vec<Card>,
    next_id: u64,
}

impl MealzPlug {
    pub fn new() -> Self {
        Self {
            cards: Vec::new(),
            next_id: 1,
        }
    }

    // --- CRUD Methods ---
    pub fn add_card(
        &mut self,
        name: String,
        tags: HashSet<String>,
        ingredients: HashSet<String>,
        max_batch_size: Option<u8>,
    ) -> Result<Card, String> {
        let new_card = Card {
            id: self.next_id,
            name,
            tags,
            ingredients,
            max_batch_size: max_batch_size.unwrap_or(1),
        };
        self.cards.push(new_card.clone());
        self.next_id += 1;
        Ok(new_card)
    }
    pub fn list_cards(&self) -> &Vec<Card> {
        &self.cards
    }
    pub fn update_card(
        &mut self,
        card_id: u64,
        new_name: Option<String>,
        new_tags: Option<HashSet<String>>,
        new_ingredients: Option<HashSet<String>>,
        new_max_batch_size: Option<u8>,
    ) -> Result<Card, String> {
        let card = self
            .cards
            .iter_mut()
            .find(|c| c.id == card_id)
            .ok_or_else(|| format!("Card {} not found", card_id))?;
        if let Some(name) = new_name {
            card.name = name;
        }
        if let Some(tags) = new_tags {
            card.tags = tags;
        }
        if let Some(ingredients) = new_ingredients {
            card.ingredients = ingredients;
        }
        if let Some(batch_size) = new_max_batch_size {
            card.max_batch_size = batch_size;
        }
        Ok(card.clone())
    }
    pub fn get_card(&self, card_id: u64) -> Result<Card, String> {
        self.cards
            .iter()
            .find(|c| c.id == card_id)
            .cloned()
            .ok_or_else(|| format!("Card {} not found", card_id))
    }
    pub fn remove_card(&mut self, card_id: u64) -> Result<Card, String> {
        let index = self
            .cards
            .iter()
            .position(|c| c.id == card_id)
            .ok_or_else(|| format!("Card {} not found", card_id))?;
        Ok(self.cards.remove(index))
    }

    // --- Meal Plan Generation Logic ---

    pub fn generate_plan(&self, constraints: &PlanConstraints) -> Result<GenerationResult, String> {
        let candidates = self.find_candidates(&constraints.filters);
        if candidates.is_empty() {
            return Err("No cards match the specified filters.".to_string());
        }
        self.build_idea_plan(candidates, constraints)
    }

    fn find_candidates(&self, filters: &CardFilters) -> Vec<Card> {
        self.cards
            .iter()
            .filter(|card| {
                let name_match = filters.name_contains.is_empty()
                    || card
                        .name
                        .to_lowercase()
                        .contains(&filters.name_contains.to_lowercase());

                let batchable_match = match filters.batch_mode {
                    BatchMode::Allow => true,
                    BatchMode::Only => card.max_batch_size > 1,
                    BatchMode::Prevent => card.max_batch_size <= 1,
                };

                let tags_match = filters.tag_filters.is_empty()
                    || match filters.tag_mode {
                        FilterMode::All => filters.tag_filters.is_subset(&card.tags),
                        FilterMode::Any => !filters.tag_filters.is_disjoint(&card.tags),
                    };
                let ingredients_match = filters.ingredient_filters.is_empty()
                    || match filters.ingredient_mode {
                        FilterMode::All => filters.ingredient_filters.is_subset(&card.ingredients),
                        FilterMode::Any => {
                            !filters.ingredient_filters.is_disjoint(&card.ingredients)
                        }
                    };
                let joker_match = filters.allow_jokers || !card.tags.contains("joker");

                name_match && batchable_match && tags_match && ingredients_match && joker_match
            })
            .cloned()
            .collect()
    }

    fn build_idea_plan(
        &self,
        candidates: Vec<Card>,
        constraints: &PlanConstraints,
    ) -> Result<GenerationResult, String> {
        let full_schedule = self.build_raw_schedule(candidates, constraints)?;
        let mut warnings = full_schedule.1;

        // Simplify the final list for the user as requested
        let mut final_ideas = Vec::new();
        let mut seen_ids = HashSet::new();

        for card in &full_schedule.0 {
            if !card.tags.contains("joker") && seen_ids.insert(card.id) {
                final_ideas.push(card.clone());
            }
        }

        if final_ideas.len() < full_schedule.0.len() {
            warnings.push(
                "Note: Jokers and duplicates have been removed from the final idea list."
                    .to_string(),
            );
        }

        Ok(GenerationResult {
            plan: MealPlan {
                schedule: PlanSchedule::Ideas(final_ideas),
            },
            warnings,
        })
    }

    /// Internal helper to build the raw, potentially repetitive schedule.
    fn build_raw_schedule(
        &self,
        candidates: Vec<Card>,
        constraints: &PlanConstraints,
    ) -> Result<(Vec<Card>, Vec<String>), String> {
        let mut schedule = Vec::with_capacity(constraints.number_of_meals as usize);
        let mut warnings = Vec::new();
        let mut rng = rand::rng();

        let mut inventory: Vec<Card> = candidates
            .into_iter()
            .flat_map(|card| vec![card.clone(); card.max_batch_size as usize])
            .collect();
        inventory.shuffle(&mut rng);

        let mut plan_counts: HashMap<u64, u8> = HashMap::new();

        while schedule.len() < constraints.number_of_meals as usize {
            if inventory.is_empty() {
                warnings.push(format!(
                    "Ran out of meal portions. Plan stopped at {} meals.",
                    schedule.len()
                ));
                break;
            }

            let choice_index = inventory.iter().position(|card| {
                let consecutive = constraints.no_consecutive
                    && schedule
                        .last()
                        .map_or(false, |last: &Card| last.id == card.id);
                let over_limit = constraints.max_repeats_per_plan > 0
                    && plan_counts.get(&card.id).unwrap_or(&0) >= &constraints.max_repeats_per_plan;
                !consecutive && !over_limit
            });

            if let Some(index) = choice_index {
                let chosen_card = inventory.remove(index);
                *plan_counts.entry(chosen_card.id).or_insert(0) += 1;
                schedule.push(chosen_card);
            } else {
                warnings.push("Could not satisfy all constraints (e.g., repetition). The remaining plan may have duplicates.".to_string());
                let chosen_card = inventory.remove(0);
                *plan_counts.entry(chosen_card.id).or_insert(0) += 1;
                schedule.push(chosen_card);
            }
        }
        Ok((schedule, warnings))
    }
}
