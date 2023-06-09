#![cfg_attr(not(feature = "std"), no_std)]

#[openbrush::contract]
mod reentrancy_test {
    use openbrush::contracts::reentrancy_guard::{self, *};
    use openbrush::traits::Storage;

    use ink::primitives::Key;

    #[ink(event)]
    pub struct StorageDecodedAsGuard {
        status: u8,
    }

    #[ink(event)]
    pub struct StorageDecodedCorrectly {
        value: u32,
    }

    #[ink(storage)]
    #[derive(Storage)]
    pub struct ReentrancyTest {
        #[storage_field]
        guard: reentrancy_guard::Data,
        value: u32,
    }

    impl ReentrancyTest {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                guard: Default::default(),
                value: 123456789,
            }
        }

        #[ink(message)]
        pub fn get(&self) -> u32 {
            self.value
        }

        #[ink(message)]
        pub fn increment_recursive_reentrant(&mut self, num: u32) {
            self.check_storage();

            if num == 0 {
                self.value += 1;
                return;
            }
        }

        #[ink(message)]
        #[openbrush::modifiers(non_reentrant)]
        pub fn increment_recursive(&mut self, num: u32) -> Result<(), ReentrancyGuardError> {
            self.check_storage();

            if num == 0 {
                self.value += 1;
                return Ok(());
            }

            Ok(())
        }

        fn check_storage(&self) {
            match ink::env::get_contract_storage::<Key, ReentrancyTest>(&0) {
                Ok(Some(s)) => {
                    self.env()
                        .emit_event(StorageDecodedCorrectly { value: s.value });
                }
                _ => {}
            };

            match ink::env::get_contract_storage::<Key, reentrancy_guard::Data>(&0) {
                Ok(Some(g)) => {
                    self.env()
                        .emit_event(StorageDecodedAsGuard { status: g.status });
                }
                _ => {}
            };
        }
    }
}
