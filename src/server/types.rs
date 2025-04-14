// --- Needed Imports (Add to relevant files or a prelude) ---
// You might need these depending on where you place the definitions
// and the actual implementation details.
// use std::collections::HashMap; // Needed if fields use HashMap
// use rand::Rng; // Needed for Individual::choose_action if using random numbers

// --- Error Type Alias (Optional but helpful) ---
// Define a simple error type for functions returning Results
pub type StubError = String;

// --- Enum Definitions ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] // Eq/Hash useful for HashMap keys
pub enum Action {
    Farm,
    Hunt,
    Trade,
    // Add other variants if used elsewhere
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Trait {
    Industrious,
    // Add other variants if used elsewhere
}