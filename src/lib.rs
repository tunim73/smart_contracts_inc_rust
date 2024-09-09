#![cfg_attr(not(feature = "std"), no_std, no_main)]
//cargo +nightly contract build
// ink_metadata = { version = "5.0.0", default-features = false }
#[ink::contract]
mod knowledge_project {
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    use ink::storage::StorageVec;

    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub struct Conhecimento {
        text: String,
        categoria: i32
    }

    #[ink(event, anonymous)]
    pub struct MessageAll {
       mess: String,
    }

    #[ink(storage)]
    pub struct MyContract {
        conhecimentos: StorageVec<Conhecimento>
    }

    impl MyContract {
        #[ink(constructor)]
        pub fn default() -> Self {
            Self {
                conhecimentos: Default::default(),
            }
        }

        #[ink(message)]
        pub fn store_conhecimento(& mut self, text: String, categoria: i32) {
            
            let con = Conhecimento {
                text,
                categoria
            };
            self.conhecimentos.push(&con);

        }

        #[ink(message)]
        pub fn get_conhecimentos(& mut self) -> Vec<String> {
            let mut resultados = Vec::<String>::new();

            for i in 0..self.conhecimentos.len() {
                if let Some(conhecimento) = self.conhecimentos.get(i) {
                        resultados.push(conhecimento.text.clone());
                }
            }

            resultados
        }

        #[ink(message)]
        pub fn search(&mut self, search_text: String, search_categoria: i32)-> Vec<String> {
            if search_text.trim().is_empty() || search_categoria == 0 || search_categoria > 4{

                let mut resultados = Vec::<String>::new();

                resultados

            } else {
                let mut resultados = Vec::<String>::new();

                for i in 0..self.conhecimentos.len() {
                    if let Some(conhecimento) = self.conhecimentos.get(i) {
                        if conhecimento.text.contains(&search_text) && conhecimento.categoria == search_categoria {
                            resultados.push(conhecimento.text.clone());
                        }
                    }
                }

                resultados
            }



        }

        /*#[ink(message)]
        pub fn search_conhecimentos(&self, search_text: String, search_categoria: i32) -> Vec<String> {
            let mut resultados = Vec::new();

            for i in 0..self.conhecimentos.len() {
                if let Some(conhecimento) = self.conhecimentos.get(i) {
                    if conhecimento.text.contains(&search_text) && conhecimento.categoria == search_categoria {

                    }
                    resultados.push(conhecimento.text.clone());
                }
            }

            resultados
        }*/


        /*#[ink(message)]
        pub fn search_conhecimentos(&self, search_text: String, search_categoria: i32) -> Vec<String> {
            let mut resultados = Vec::new();

            for conhecimento in self.conhecimentos.iter() {
                if conhecimento.text.contains(&search_text) && conhecimento.categoria == search_categoria {
                    resultados.push(conhecimento.text.clone());
                }
            }

            resultados
        }*/


        #[ink(message)]
        pub fn get_categorias(& mut self) -> (Vec<String>) {

            let mut a = Vec::<String>::new();
            let tecnologias = String::from("Php = 1, Node = 2, Git = 3, Docker = 4");

            a.push(tecnologias);
            a

        }




     /*   #[ink(message)]
        pub fn list_conhecimentos(&self) -> StorageVec<Conhecimento> {
            self.conhecimentos
        }*/


        // #[ink(message)]
        // pub fn get_categorias(& mut self) -> (Vec<String>) {
        //
        //
        //
        //
        //
        //     /*match self.items {
        //         Item::Bye => "Bye".parse().unwrap(),
        //         Item::Oi => "Oi".parse().unwrap(),
        //     }*/
        //
        // }


    }

}
/*#[ink::contract]
mod clear_xurumela {
    use ink::{
        storage::StorageVec,
    };

    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub struct Proposal {
        approvals: u32,
        min_approvals: u32,
    }

    #[ink(storage)]
    pub struct LazyVector {
        proposals: StorageVec<crate::clear_xurumela::Proposal>,
    }

    impl crate::clear_xurumela::LazyVector {
        #[ink(constructor, payable)]
        pub fn default() -> Self {
            Self {
                proposals: Default::default(),
            }
        }

        #[ink(message)]
        pub fn get(&self, at: u32) -> Option<crate::clear_xurumela::Proposal> {
            self.proposals.get(at)
        }
    }
}
*/