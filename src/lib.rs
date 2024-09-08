#![cfg_attr(not(feature = "std"), no_std, no_main)]
//cargo +nightly contract build
// ink_metadata = { version = "5.0.0", default-features = false }
#[ink::contract]
mod knowledge_project {

    use ink::prelude::string::String;

    #[ink::storage_item]
    pub struct Auction {
        name: String,
        status: bool,
        finalized: bool,
    }
    #[ink::storage_item]
    pub enum Item {
        Oi,
        Bye
    }


    #[ink(storage)]
    pub struct MyContract {
        // Store Auctions in a vec
        auctions: Auction,
        items: Item
    }

    impl MyContract {
        #[ink(constructor)]
        pub fn new(name: String) -> Self {
            Self {
                auctions: Auction {
                    name,
                    status: false,
                    finalized: false,
                },
                items: Item::Bye
            }
        }

        #[ink(message)]
        pub fn get(&self) -> (bool) {
            (self.auctions.status)
        }

        #[ink(message)]
        pub fn get_hello(& mut self) -> (String) {
            
            match self.items {
                Item::Bye => "Bye".parse().unwrap(),
                Item::Oi => "Oi".parse().unwrap(),
            }

        }

        #[ink(message)]
        pub fn set_status(&mut self) {
            self.auctions.status = !self.auctions.status;
        }
    }

}
