#![cfg_attr(not(feature = "std"), no_std, no_main)]

        
#[openbrush::implementation(PSP34)]
#[openbrush::contract]
pub mod p_learn_levels {
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Levels {
    	#[storage_field]
		psp34: psp34::Data,
    }
    
    impl Levels {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut _instance = Self::default();
			_instance
        }
    }
}