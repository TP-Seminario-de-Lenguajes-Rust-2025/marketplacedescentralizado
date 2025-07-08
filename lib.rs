#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod contract {
    use ink::{prelude::{string::String,vec::Vec},storage::{StorageVec,Mapping}};
    use contract_logic::usuario::*;

    // #[ink::scale_derive(Encode, Decode, TypeInfo)] //porque no funciona el macro?
    // #[cfg_attr(
    //     feature = "std",
    //     derive(ink::storage::traits::StorageLayout)
    // )]
    // #[derive(Debug)]
    // struct Usuario;
    // impl usuario::Usuario for Usuario{}
    #[ink(storage)]
    pub struct Sistema {
        // users: Vec<Usuario>,

        // //asociacion entre usuario y rol
        // roles: Mapping<String, Vec<Usuario>>, //k: id_rol

        // ordenes_historico: StorageVec<Orden>, //registro de compras

        // //guarda las publicaciones
        // publicaciones: Vec<Publicacion>, //capaz no un vec
    }

    impl Sistema {
        #[ink(constructor)]
        pub fn new() -> Self {
            todo!()
            //Sistema {}
        }

        #[ink(message)]
        pub fn crear_orden(&self) {
            //valida user
            todo!()
        }

        /// role related
        pub fn asociar_rol() {
            todo!()
        }

        pub fn has_role() {
            todo!("verifica que el usuario tiene el rol")
        }
    }
}
