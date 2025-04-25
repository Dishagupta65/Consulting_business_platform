#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, symbol_short, String};

#[contracttype]
#[derive(Clone)]
pub struct ConsultingRequest {
    pub request_id: u64,
    pub client_name: String,
    pub project_scope: String,
    pub is_approved: bool,
}

#[contract]
pub struct ConsultingPlatform;

const REQUEST_COUNTER: Symbol = symbol_short!("REQ_CNT");

#[contractimpl]
impl ConsultingPlatform {
    pub fn submit_request(env: Env, client_name: String, project_scope: String) -> u64 {
        let mut counter: u64 = env.storage().instance().get(&REQUEST_COUNTER).unwrap_or(0);
        counter += 1;

        let new_request = ConsultingRequest {
            request_id: counter,
            client_name,
            project_scope,
            is_approved: false,
        };

        env.storage().instance().set(&counter, &new_request);
        env.storage().instance().set(&REQUEST_COUNTER, &counter);

        counter
    }

    pub fn approve_request(env: Env, request_id: u64) {
        let mut request: ConsultingRequest = env
            .storage()
            .instance()
            .get(&request_id)
            .expect("Request not found");

        request.is_approved = true;
        env.storage().instance().set(&request_id, &request);
    }

    pub fn get_request(env: Env, request_id: u64) -> ConsultingRequest {
        env.storage()
            .instance()
            .get(&request_id)
            .expect("Request not found")
    }
}

