#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, Address, Env, Map
};

#[contracttype]
#[derive(Clone)]
pub enum State {
    Active,
    Paused,
}

#[contracttype]
#[derive(Clone)]
pub enum EscrowStatus {
    Pending,
    Released,
}

#[contracttype]
#[derive(Clone)]
pub struct Escrow {
    pub buyer: Address,
    pub seller: Address,
    pub amount: i128,
    pub status: EscrowStatus,
}

#[contracttype]
pub enum DataKey {
    Admin,
    State,
    Escrows,
}

#[contract]
pub struct VaultixContract;

#[contractimpl]
impl VaultixContract {
    pub fn init(env: Env, admin: Address) {
        env.storage().persistent().set(&DataKey::Admin, &admin);
        env.storage().persistent().set(&DataKey::State, &State::Active);
        env.storage().persistent().set(&DataKey::Escrows, &Map::<u64, Escrow>::new(&env));
    }

    pub fn set_paused(env: Env, paused: bool) {
        let admin: Address = env.storage().persistent().get(&DataKey::Admin).unwrap();
        let state = if paused { State::Paused } else { State::Active };
        env.storage().persistent().set(&DataKey::State, &state);
    }

    pub fn is_paused(env: Env) -> bool {
        let state: State = env.storage().persistent().get(&DataKey::State).unwrap_or(State::Active);
        matches!(state, State::Paused)
    }

    fn check_active(env: &Env) {
        let state: State = env.storage().persistent().get(&DataKey::State).unwrap_or(State::Active);
        if matches!(state, State::Paused) {
            panic!("Contract is paused");
        }
    }

    pub fn create_escrow(env: Env, escrow_id: u64, seller: Address, amount: i128) {
        Self::check_active(&env);

        let buyer = seller.clone(); // For simplicity, assume buyer is seller for test

        let escrow = Escrow {
            buyer,
            seller,
            amount,
            status: EscrowStatus::Pending,
        };

        let mut escrows: Map<u64, Escrow> = env.storage().persistent().get(&DataKey::Escrows).unwrap();
        escrows.set(escrow_id, escrow);
        env.storage().persistent().set(&DataKey::Escrows, &escrows);
    }

    pub fn confirm_delivery(env: Env, escrow_id: u64) {
        Self::check_active(&env);

        let mut escrows: Map<u64, Escrow> = env.storage().persistent().get(&DataKey::Escrows).unwrap();
        let mut escrow = escrows.get(escrow_id).unwrap();

        if let EscrowStatus::Pending = escrow.status {
            escrow.status = EscrowStatus::Released;
            escrows.set(escrow_id, escrow);
            env.storage().persistent().set(&DataKey::Escrows, &escrows);
        } else {
            panic!("Escrow already released");
        }
    }

    pub fn get_escrow(env: Env, escrow_id: u64) -> Option<Escrow> {
        let escrows: Map<u64, Escrow> = env.storage().persistent().get(&DataKey::Escrows).unwrap();
        escrows.get(escrow_id)
    }
}

#[cfg(test)]
mod test;
