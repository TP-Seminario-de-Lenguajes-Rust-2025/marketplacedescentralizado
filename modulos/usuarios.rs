use ink::env::DefaultEnvironment;
use ink::prelude::string::String;
use ink::storage::{Mapping, StorageVec};
use scale::{Decode, Encode};
use scale_info::TypeInfo;
type AccountId = <DefaultEnvironment as ink::env::Environment>::AccountId;

#[derive(Encode, Decode, TypeInfo, Debug)]
pub enum MiError {
    NoAutorizado,
    UsuarioYaExistente,
    UsuarioInexistente,
}

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct Usuario {
    pub nombre: String,
    pub mail: String,
}

#[derive(TypeInfo, Debug)]
pub struct Usuarios {
    pub map_usuarios: Mapping<AccountId, Usuario>,
    pub vec_usuarios: StorageVec<AccountId>,
}

impl Usuarios {
    pub fn new() -> Self {
        Self {
            map_usuarios: Mapping::default(),
            vec_usuarios: StorageVec::new(),
        }
    }

    /// Inserta un usuario si su id no existe aún
    pub fn insertar_usuario(&mut self, id: AccountId, usuario: Usuario) -> Result<(), MiError> {
        if self.map_usuarios.contains(id) {
            return Err(MiError::UsuarioYaExistente);
        }

        self.map_usuarios.insert(id, &usuario);
        self.vec_usuarios.push(&id);
        Ok(())
    }

    /// Retorna todos los usuarios almacenados
    pub fn obtener_todos(&self) -> Vec<Usuario> {
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
}
