use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};

const PUZZLE_NUMBER: u8 = 1;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    crossword_solution: String,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(solution: String) -> Self{
        let hash_string = hex::encode(env::sha256(solution.as_bytes()));

        return Self{
            crossword_solution: hash_string
        };
    }

    pub fn get_puzzle_number(&self) -> u8{
        return PUZZLE_NUMBER;
    }

    pub fn explore(&mut self){
        //env::log_str(env::current_account_id().as_str());
        env::log_str(env::account_balance().to_string().as_str());
        let account_id = env::predecessor_account_id();
        env::log_str(account_id.as_str());
        env::log_str(env::epoch_height().to_string().as_str());
        env::log_str(env::validator_total_stake().to_string().as_str());
        env::log_str(env::account_locked_balance().to_string().as_str());
    }

    pub fn get_solution(&self) -> String{
        return self.crossword_solution.clone();
    }

    pub fn guess_solution(&mut self, solution: String) -> bool{
        let hashed_solution = hex::encode(env::sha256(solution.as_bytes()));

        if hashed_solution == self.crossword_solution {
            env::log_str("You guessed right!");
            return true;
        }else{
            env::log_str("Try again");
            return false;
        }
    }
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        return builder;
    }

    #[test]
    fn debug_get_hash(){
        testing_env!(VMContextBuilder::new().build());

        let debug_solution = "kate";
        let debug_hash_bytes = env::sha256(debug_solution.as_bytes());
        let debug_hash_string = hex::encode(debug_hash_bytes);
        println!("Let's debug: {:?}", debug_hash_string);
    }

    #[test]
    fn explore(){
        let mario = AccountId::new_unchecked("marioyordanov.testnet".to_string());
        let context = get_context(mario);
        testing_env!(context.build());

        println!("{:?}",env::current_account_id());
    }

    #[test]
    fn check_guess_solution(){
        let mario = AccountId::new_unchecked("marioyordanov.testnet".to_string());
        let context = get_context(mario);
        testing_env!(context.build());

        let mut contract = Contract::new(
            "mario".to_string(),
        );

        let mut guess_res = contract.guess_solution("mario".to_string());
        assert!(guess_res, "Expected successfull guess");
        assert_eq!(get_logs(), ["You guessed right!"], "Expected a success log.");
        guess_res = contract.guess_solution("wrong".to_string());
        assert!(!guess_res, "Expected wrong answer");
        assert_eq!(get_logs(), ["You guessed right!", "Try again"], "Expected a failure log");
    }

    // TESTS HERE
}
