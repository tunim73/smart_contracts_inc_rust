#![cfg_attr(not(feature = "std"), no_std, no_main)]
//cargo +nightly contract build
// ink_metadata = { version = "5.0.0", default-features = false }
#[ink::contract]
mod knowledge_project {
    use ink::prelude::string::String;
    use ink::storage::StorageVec;

    #[derive(Default)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
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
        auctions: Auction,
        items: Item,
        xuxas_string: StorageVec<String>
    }

    impl MyContract {
        #[ink(constructor)]
        pub fn new(name: String) -> Self {

            let mut xuxas = StorageVec::new();

            // Exemplo de como adicionar um item ao StorageVec

            let a: String = "dada".parse().unwrap();
            xuxas.push(&a);


            Self {
                auctions: Auction {
                    name,
                    status: false,
                    finalized: false,
                },
                items: Item::Bye,
                xuxas_string:xuxas ,
            }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self {
                auctions: Default::default(),
                items: Item::Oi,
                xuxas_string: Default::default(),
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
