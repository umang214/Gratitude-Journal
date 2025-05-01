#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Address, String, Vec, Symbol, symbol_short, log};

#[contracttype]
pub enum GratitudeKey {
    Entries(Address),
}

#[contract]
pub struct GratitudeJournal;

#[contractimpl]
impl GratitudeJournal {
    // Add a gratitude entry
    pub fn add_entry(env: Env, user: Address, entry: String) {
        user.require_auth();

        let key = GratitudeKey::Entries(user.clone());
        let mut journal: Vec<String> = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or(Vec::new(&env));

        journal.push_back(entry.clone());
        env.storage().persistent().set(&key, &journal);

        log!(&env, "Gratitude entry added by {}: {}", user, entry);
    }

    // View all gratitude entries by a user
    pub fn get_entries(env: Env, user: Address) -> Vec<String> {
        let key = GratitudeKey::Entries(user);
        env.storage().persistent().get(&key).unwrap_or(Vec::new(&env))
    }

    // Count gratitude entries by a user
    pub fn count_entries(env: Env, user: Address) -> u32 {
        let entries = Self::get_entries(env, user);
        entries.len()
    }
}
