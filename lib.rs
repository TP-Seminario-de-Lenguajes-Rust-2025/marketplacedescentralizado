#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod contrato {
    use ink::prelude::string::String;
    use ink::storage::traits::StorageLayout;
    use ink::storage::Mapping;
    use ink::storage::StorageVec;
    use scale::{Decode, Encode};
    use scale_info::prelude::vec::Vec;
    use scale_info::TypeInfo;

    #[derive(Encode, Decode, TypeInfo, Debug)]
    pub enum MiError {
        NoAutorizado,
        UsuarioYaExistente,
        UsuarioInexistente,
        MailYaExistente,
    }

    #[derive(Encode, Decode, TypeInfo, Debug)]
    pub enum Roles {
        Comprador,
        Vendedor,
    }

    #[derive(Encode, Decode, TypeInfo, Debug)]
    #[cfg_attr(feature = "std", derive(StorageLayout))]
    pub struct Usuario {
        nombre: String,
        mail: String,
        roles: Vec<Roles>,
    }

    #[ink(storage)]
    pub struct Contrato {
        value: bool,
        map_usuarios: Mapping<AccountId, Usuario>,
        vec_usuarios: Vec<AccountId>,
    }

    impl Contrato {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self {
                value: init_value,
                map_usuarios: Mapping::default(),
                vec_usuarios: Vec::default(),
            }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        #[ink(message)]
        pub fn registrar_usuario(
            &mut self,
            nombre: String,
            mail: String,
            roles: Vec<Roles>,
        ) -> Result<String, MiError> {
            let id_usuario = self.env().caller();
            return self._registrar_usuario(id_usuario, nombre, mail, roles);
        }

        fn _registrar_usuario(
            &mut self,
            account_id: AccountId,
            nombre: String,
            mail: String,
            roles: Vec<Roles>,
        ) -> Result<String, MiError> {
            // Verifico que el usuario no exista
            if self.existe_usuario(&account_id) {
                return Err(MiError::UsuarioYaExistente);
            }
            if self.existe_mail(&mail) {
                return Err(MiError::MailYaExistente);
            }
            let user = Usuario {
                mail: mail,
                nombre: nombre.clone(),
                roles: roles,
            };
            self.insertar_usuario(account_id, user)?;
            Ok(format!(
                "El usuario {} fue registrado correctamente",
                nombre
            ))
            // Verifico que el mail no este en uso
        }

        #[ink(message)]
        pub fn listar_usuarios(&self) -> Vec<Usuario> {
            let mut resultado = Vec::new();
            for i in 0..self.vec_usuarios.len() {
                if let Some(account_id) = self.vec_usuarios.get(i) {
                    if let Some(usuario) = self.map_usuarios.get(account_id) {
                        resultado.push(usuario);
                    }
                }
            }
            resultado
        }

        /// Inserta un usuario si su id no existe aún
        fn insertar_usuario(&mut self, id: AccountId, usuario: Usuario) -> Result<(), MiError> {
            self.map_usuarios.insert(id, &usuario);
            self.vec_usuarios.push(id);
            Ok(())
        }

        /// Verifica si ya existe un usuario con el mail dado
        fn existe_mail(&self, mail: &str) -> bool {
            for i in 0..self.vec_usuarios.len() {
                if let Some(account_id) = self.vec_usuarios.get(i) {
                    if let Some(usuario) = self.map_usuarios.get(account_id) {
                        if usuario.mail == mail {
                            return true;
                        }
                    }
                }
            }
            false
        }

        /// Verifica si existe un usuario con el AccountId dado
        fn existe_usuario(&self, id: &AccountId) -> bool {
            self.map_usuarios.contains(id)
        }

        // /// Inserta un usuario si su id no existe aún
        // #[ink(message)]
        // pub fn eliminar_usuario(&mut self, account_id: AccountId) -> Result<(), MiError> {
        //     if !self.map_usuarios.contains(account_id) {
        //         return Err(MiError::UsuarioInexistente);
        //     }

        //     // Eliminar del mapa
        //     self.map_usuarios.remove(account_id);

        //     // Eliminar del vector
        //     let mut nuevo_vec = Vec::new();
        //     for i in 0..self.vec_usuarios.len() {
        //         if let Some(id) = self.vec_usuarios.get(i) {
        //             if *id != account_id {
        //                 nuevo_vec.push(id);
        //             }
        //         }
        //     }
        //     self.vec_usuarios = nuevo_vec;

        //     Ok(())
        // }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let contrato = Contrato::default();
            //assert_eq!(contrato.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut contrato = Contrato::new(false);
            //assert_eq!(contrato.get(), false);
            //contrato.flip();
            //assert_eq!(contrato.get(), true);
        }
    }

    /// This is how you'd write end-to-end (E2E) or integration tests for ink! contracts.
    ///
    /// When running these you need to make sure that you:
    /// - Compile the tests with the `e2e-tests` feature flag enabled (`--features e2e-tests`)
    /// - Are running a Substrate node which contains `pallet-contracts` in the background
    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// A helper function used for calling contract messages.
        use ink_e2e::ContractsBackend;

        /// The End-to-End test `Result` type.
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        /// We test that we can upload and instantiate the contract using its default constructor.
        #[ink_e2e::test]
        async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let mut constructor = ContratoRef::default();

            // When
            let contract = client
                .instantiate("contrato", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");
            let call_builder = contract.call_builder::<Contrato>();

            // Then
            let get = call_builder.get();
            let get_result = client.call(&ink_e2e::alice(), &get).dry_run().await?;
            assert!(matches!(get_result.return_value(), false));

            Ok(())
        }

        /// We test that we can read and write a value from the on-chain contract.
        #[ink_e2e::test]
        async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let mut constructor = ContratoRef::new(false);
            let contract = client
                .instantiate("contrato", &ink_e2e::bob(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");
            let mut call_builder = contract.call_builder::<Contrato>();

            let get = call_builder.get();
            let get_result = client.call(&ink_e2e::bob(), &get).dry_run().await?;
            assert!(matches!(get_result.return_value(), false));

            // When
            let flip = call_builder.flip();
            let _flip_result = client
                .call(&ink_e2e::bob(), &flip)
                .submit()
                .await
                .expect("flip failed");

            // Then
            let get = call_builder.get();
            let get_result = client.call(&ink_e2e::bob(), &get).dry_run().await?;
            assert!(matches!(get_result.return_value(), true));

            Ok(())
        }
    }
}
