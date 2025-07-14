#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod contrato {
    use ink::prelude::string::String;
    use ink::storage::traits::StorageLayout;
    use ink::storage::Mapping;
    use scale::{Decode, Encode};
    use scale_info::prelude::vec::Vec;
    use scale_info::TypeInfo;

    #[derive(Encode, Decode, TypeInfo, Debug)]
    pub enum ErroresContrato {
        NoAutorizado,
        UsuarioYaExistente,
        CuentaNoRegistrada,
        MailYaExistente,
        SeFueALaMierdaElID,
    }

    #[derive(Encode, Decode, TypeInfo, Debug)]
    #[cfg_attr(feature = "std", derive(StorageLayout))]
    pub enum Categoria {
        Bazar,
        Hogar,
        Electronica,
        FalopaDeLaRica,
        Otros,
    }

    #[derive(Encode, Decode, TypeInfo, Debug)]
    #[cfg_attr(feature = "std", derive(StorageLayout))]
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

    #[derive(Encode, Decode, TypeInfo, Debug)]
    #[cfg_attr(feature = "std", derive(StorageLayout))]

    pub struct Producto {
        id: u128,
        nombre: String,
        categoria: Categoria,
        cantidad: u32,
        precio: Balance,
        descripcion: String,
    }

    #[ink(storage)]
    pub struct Contrato {
        value: bool,
        map_usuarios: Mapping<AccountId, Usuario>,
        vec_usuarios: Vec<AccountId>,
        map_productos: Mapping<u128, Producto>,
        vec_productos: Vec<u128>,
        actual_id_prod: u128,
    }

    //TODO:
    // Faltarian agregar los siguientes metodos publicos (ink-messages) para exponer la interfaz:

    // - publicar_producto (solo si tiene rol Vendedor)
    // - ver_productos_publicados (solo si tiene rol Vendedor)
    // - enviar_producto (solo si tiene rol Vendedor)

    // - comprar_producto (solo si tiene rol Comprador)
    // - marcar_como_recibido (solo si tiene rol Comprador)
    // - cancelar_compra (solo si tiene rol Comprador)

    impl Contrato {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self {
                value: init_value,
                map_usuarios: Mapping::default(),
                map_productos: Mapping::default(),
                vec_usuarios: Vec::default(),
                vec_productos: Vec::default(),
                actual_id_prod: 0,
            }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        #[ink(message)]
        pub fn agregar_producto(
            &mut self,
            nombre: String,
            categoria: Categoria,
            cantidad: u32,
            precio: Balance,
            descripcion: String,
        ) -> Result<String, ErroresContrato> {
            // Comprobar que el usuario esta registrado en la plataforma
            let id_cuenta = self.env().caller();
            if !self.existe_usuario(&id_cuenta) {
                return Err(ErroresContrato::CuentaNoRegistrada)
            };
            // Comprobar que el producto no exista chequeando nombre y categoria
            // Agregar producto
            let id = self
                .actual_id_prod
                .checked_add(1)
                .ok_or(ErroresContrato::SeFueALaMierdaElID)?;

            let nuevo_producto = Producto {
                id,
                nombre,
                categoria,
                cantidad,
                precio,
                descripcion,
            };

            self.map_productos.insert(id, &nuevo_producto);
            self.vec_productos.push(id);
            self.actual_id_prod = id;
            Ok(nuevo_producto.nombre)
        }

        #[ink(message)]
        pub fn listar_productos(&self) -> Vec<Producto> {
            let mut resultado = Vec::new();
            for i in 0..self.vec_productos.len() {
                if let Some(prod_id) = self.vec_productos.get(i) {
                    if let Some(producto) = self.map_productos.get(prod_id) {
                        resultado.push(producto);
                    }
                }
            }
            resultado
        }

        #[ink(message)]
        pub fn registrar_usuario(
            &mut self,
            nombre: String,
            mail: String,
            roles: Vec<Roles>,
        ) -> Result<String, ErroresContrato> {
            let id_usuario = self.env().caller();
            return self._registrar_usuario(id_usuario, nombre, mail, roles);
        }

        fn _registrar_usuario(
            &mut self,
            account_id: AccountId,
            nombre: String,
            mail: String,
            roles: Vec<Roles>,
        ) -> Result<String, ErroresContrato> {
            // Verifico que el usuario y el mail no existan
            if self.existe_usuario(&account_id) {
                return Err(ErroresContrato::UsuarioYaExistente)
            };
            if self.existe_mail(&mail) {
                return Err(ErroresContrato::MailYaExistente)
            };

            // Instancio nuevo usuario
            let user = Usuario {
                mail: mail,
                nombre: nombre.clone(),
                roles: roles,
            };

            // Inserto el usuario tanto en el Mapping como en el Vec
            self.insertar_usuario(account_id, user);

            // Genero el mensaje de retorno
            let retorno = {
                let mut s = String::from("Se registro el usuario -> ");
                s.push_str(&nombre);
                s
            };

            Ok(retorno)
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

        /// Inserta un usuario
        fn insertar_usuario(
            &mut self,
            id: AccountId,
            usuario: Usuario,
        ) {
            self.map_usuarios.insert(id, &usuario);
            self.vec_usuarios.push(id);
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
            if !self.map_usuarios.contains(id) {
                return false
            }
            true
        }

        // /// Inserta un usuario si su id no existe aún
        // #[ink(message)]
        // pub fn eliminar_usuario(&mut self, account_id: AccountId) -> Result<(), ErroresContrato> {
        //     if !self.map_usuarios.contains(account_id) {
        //         return Err(ErroresContrato::CuentaNoRegistrada);
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
}

