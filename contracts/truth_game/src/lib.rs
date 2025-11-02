// --- Two Truths and a Lie ---
#![no_std]

// We need more imports: Map for storage, Vec for lists, BytesN for the hash
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Address, Env, String, Map, Vec, BytesN,
};

// --- 1. Define our Data Structures ---

// 'Game' holds all data for a single game instance
#[contracttype]
#[derive(Clone)]
pub struct Game {
    pub owner: Address,         // Who created the game
    pub commit_hash: BytesN<32>, // The SHA-256 hash of the secret data
    pub reveal_time: u64,       // When the game can be revealed

    // These fields start empty and are filled in *after* the reveal
    pub statements: Vec<String>, // The 3 statements (revealed)
    pub lie_index: u32,          // The index (0, 1, or 2) of the lie
    pub revealed: bool,          // Has this game been revealed?
}

// 'DataKey' defines our storage slots
#[contracttype]
pub enum DataKey {
    GameCounter, // A single number: how many games exist
    Games(u32),  // A map storing Games by their ID (e.g., Games(1), Games(2))
    Guesses(u32, Address), // A map storing a user's guess for a specific game
}

#[contract]
pub struct TruthsGameContract;

// --- 2. Implement the Contract's Functions ---
#[contractimpl]
impl TruthsGameContract {

    /// Creates a new game by storing a 'commitment hash'.
    /// The hash is of (statements + lie_index + salt)
    /// The user must provide this hash from the frontend.
    pub fn commit(env: Env, owner: Address, hash: BytesN<32>) -> u32 {
        owner.require_auth();

        // Get the current game ID counter, defaulting to 0
        let mut counter = env.storage().instance().get(&DataKey::GameCounter).unwrap_or(0);
        counter += 1; // Increment for the new game

        // Create the new game struct
        let new_game = Game {
            owner: owner.clone(),
            commit_hash: hash,
            reveal_time: env.ledger().timestamp() + 86400, // 24-hour reveal window
            statements: Vec::new(&env), // Empty for now
            lie_index: 0,               // Empty for now
            revealed: false,            // Empty for now
        };

        // Save the new game
        env.storage().persistent().set(&DataKey::Games(counter), &new_game);
        // Save the updated counter
        env.storage().instance().set(&DataKey::GameCounter, &counter);

        // Log an event
        env.events().publish((symbol_short!("COMMIT"), owner, counter), hash);

        // Return the new game ID to the user
        counter
    }

    /// Allows any user to log a guess for a specific game.
    pub fn guess(env: Env, guesser: Address, game_id: u32, guessed_index: u32) {
        guesser.require_auth();

        // Check that the game exists
        if !env.storage().persistent().has(&DataKey::Games(game_id)) {
            panic!("Game does not exist");
        }

        // Store the guess. This will overwrite any previous guess.
        env.storage().persistent().set(
            &DataKey::Guesses(game_id, guesser.clone()),
            &guessed_index
        );

        env.events().publish((symbol_short!("GUESS"), guesser, game_id), guessed_index);
    }

    /// Reveals the truths and lie. This verifies the original commitment.
    pub fn reveal(
        env: Env,
        owner: Address,
        game_id: u32,
        statements: Vec<String>, // The 3 statements
        lie_index: u32,          // The index of the lie
        salt: String             // The secret password
    ) {
        owner.require_auth();

        // Get the game from storage
        let mut game = Self::get_game(env.clone(), game_id);

        // Check that the caller is the owner
        if game.owner != owner {
            panic!("Only the owner can reveal");
        }
        // Check that it hasn't been revealed already
        if game.revealed {
            panic!("Game already revealed");
        }

        // --- This is the core of the Commit-Reveal ---
        // We re-create the hash *inside the contract*
        // 1. Convert statements, index, and salt to bytes
        let mut bytes_to_hash = bytes::BytesMut::new();
        for s in statements.iter() {
            bytes_to_hash.extend_from_slice(&s.to_bytes());
        }
        bytes_to_hash.extend_from_slice(&lie_index.to_be_bytes());
        bytes_to_hash.extend_from_slice(&salt.to_bytes());

        // 2. Hash the bytes
        let calculated_hash = env.crypto().sha256(&bytes_to_hash.to_vec().into());

        // 3. Compare the calculated hash to the one stored during 'commit'
        if calculated_hash != game.commit_hash {
            panic!("Hash mismatch! Reveal data does not match commitment.");
        }

        // --- Success! ---
        // The hashes match. We update the game state to "revealed".
        game.statements = statements.clone();
        game.lie_index = lie_index;
        game.revealed = true;

        // Save the updated game
        env.storage().persistent().set(&DataKey::Games(game_id), &game);

        env.events().publish((symbol_short!("REVEAL"), owner, game_id), lie_index);
    }

    // --- Read-Only Functions ---

    /// Gets the details for a single game
    pub fn get_game(env: Env, game_id: u32) -> Game {
        env.storage().persistent().get(&DataKey::Games(game_id))
            .unwrap_or_else(|| panic!("Game not found"))
    }

    /// Gets a specific user's guess for a game
    pub fn get_guess(env: Env, game_id: u32, guesser: Address) -> Option<u32> {
        env.storage().persistent().get(&DataKey::Guesses(game_id, guesser))
    }

    /// Gets the total number of games
    pub fn get_game_count(env: Env) -> u32 {
        env.storage().instance().get(&DataKey::GameCounter).unwrap_or(0)
    }
}