#![cfg_attr(not(feature = "std"), no_std, no_main)]

//TODO:
// - Agregar la logica de las validaciones de cantidades.
//   Se debe controlar que las cantidades solicitadas no superen
//   los stocks en PUBLICACION y en PRODUCTO.
// - Agregar servicio ver_productos_publicados
//

#[ink::contract]
mod contrato {
    use ink::prelude::string::String;
    use ink::storage::traits::StorageLayout;
    use ink::storage::{Mapping, StorageVec};
    use scale::{Decode, Encode};
    use scale_info::prelude::vec::Vec;
    use scale_info::TypeInfo;

    #[derive(Encode, Decode, TypeInfo, Debug)]
    pub enum ErroresContrato {
        UsuarioSinRoles,
        UsuarioYaExistente,
        CuentaNoRegistrada,
        MailYaExistente,
        MailInexistente,
        ProductoInexistente,
        ProductoYaExistente,
        MaximoAlcanzado,
        OrdenNoPendiente,
        OrdenNoEnviada,
        OrdenYaCancelada,
        OrdenInexistente,
        PublicacionNoExiste,
    }

    #[derive(Encode, Decode, TypeInfo, Debug, PartialEq)]
    #[cfg_attr(feature = "std", derive(StorageLayout))]
    pub struct Categoria {
        id: u128,
        nombre: String,
        descripcion: String,
    }

    #[derive(Encode, Decode, TypeInfo, Debug, PartialEq)]
    #[cfg_attr(feature = "std", derive(StorageLayout))]
    pub enum Rol {
        Comprador,
        Vendedor,
    }

    #[derive(Encode, Decode, TypeInfo, Debug, PartialEq)]
    #[cfg_attr(feature = "std", derive(StorageLayout))]
    pub enum EstadoOrden {
        Pendiente,
        Enviada,
        Recibida,
        Cancelada,
    }

    #[derive(Encode, Decode, Debug, PartialEq)]
    #[cfg_attr(feature = "std", derive(StorageLayout))]
    #[cfg_attr(feature = "std", derive(TypeInfo))]

    pub struct Orden {
        id: u32,
        id_publicacion: u32,
        id_vendedor: AccountId,
        id_comprador: AccountId,
        status: EstadoOrden,
        cantidad: u32,
        precio_total: Balance,
        cal_vendedor: Option<u8>,
        cal_comprador: Option<u8>,
    }

    #[derive(Encode, Decode, TypeInfo, Debug)]
    #[cfg_attr(feature = "std", derive(StorageLayout))]
    pub struct Usuario {
        nombre: String,
        mail: String,
        roles: Vec<Rol>,
    }

    #[derive(Encode, Decode, TypeInfo, Debug)]
    #[cfg_attr(feature = "std", derive(StorageLayout))]
    pub struct Producto {
        id: u32,
        nombre: String,
        categoria: Categoria,
        cantidad: u32,
        precio: Balance,
        descripcion: String,
    }

    #[derive(Encode, Decode, Debug)]
    #[cfg_attr(feature = "std", derive(TypeInfo))]
    #[cfg_attr(feature = "std", derive(StorageLayout))]
    pub struct Publicacion {
        id: u32,
        id_producto: u32,
        id_publicador: AccountId,
        cantidad: u32,
        activa: bool,
    }

    #[ink(storage)]
    pub struct Contrato {
        v_usuarios: Vec<AccountId>,
        m_usuarios: Mapping<AccountId, Usuario>,
        productos: StorageVec<Producto>,
        ordenes: StorageVec<Orden>,
        publicaciones: StorageVec<Publicacion>,
        categorias: StorageVec<Categoria>,
    }

    pub trait GestionProducto {
        fn _agregar_producto(
            &mut self,
            nombre: String,
            categoria: Categoria,
            cantidad: u32,
            precio: Balance,
            descripcion: String,
        ) -> Result<String, ErroresContrato>;

        fn _listar_productos(&self) -> Vec<Producto>;

        fn get_producto_by_name(
            &self,
            nombre: &String,
            categoria: &Categoria,
        ) -> Result<u32, ErroresContrato>;

        fn get_producto_by_id(&self, id_producto: u32) -> Result<Producto, ErroresContrato>;
    }

    pub trait GestionUsuario {
        fn _registrar_usuario(
            &mut self,
            account_id: AccountId,
            nombre: String,
            mail: String,
            roles: Vec<Rol>,
        ) -> Result<String, ErroresContrato>;

        fn _listar_usuarios(&self) -> Vec<Usuario>;

        fn get_usuario_by_mail(&self, mail: &str) -> Result<Usuario, ErroresContrato>;

        fn get_usuario_by_id(&self, id: &AccountId) -> Result<Usuario, ErroresContrato>;
    }

    pub trait GestionOrden {
        fn _registrar_orden(
            &mut self,
            id_publicacion: u32,
            id_comprador: AccountId,
            cantidad: u32,
        ) -> Result<String, ErroresContrato>;

        fn _listar_ordenes(&self) -> Vec<Orden>;

        fn _enviar_orden(&mut self, id_orden: u32) -> Result<(), ErroresContrato>;

        fn _recibir_orden(&mut self, id_orden: u32) -> Result<(), ErroresContrato>;

        fn _cancelar_orden(&mut self, id_orden: u32) -> Result<(), ErroresContrato>;
    }

    pub trait GestionPublicacion {
        fn _registrar_publicacion(
            &mut self,
            id_producto: u32,
            id_publicador: AccountId,
            cantidad: u32,
        ) -> Result<String, ErroresContrato>;

        fn _listar_publicaciones(&self) -> Vec<Publicacion>;
    }

    impl Producto {
        pub fn new(
            id: u32,
            nombre: String,
            categoria: Categoria,
            cantidad: u32,
            precio: Balance,
            descripcion: String,
        ) -> Self {
            Self {
                id,
                nombre,
                categoria,
                cantidad,
                precio,
                descripcion,
            }
        }
    }

    impl Orden {
        pub fn new(
            id: u32,
            id_publicacion: u32,
            id_vendedor: AccountId,
            id_comprador: AccountId,
            cantidad: u32,
            precio_total: Balance,
        ) -> Orden {
            Orden {
                id,
                id_publicacion,
                id_vendedor,
                id_comprador,
                status: EstadoOrden::Pendiente,
                cantidad,
                precio_total,
                cal_vendedor: None,
                cal_comprador: None,
            }
        }
    }

    impl Publicacion {
        pub fn new(id: u32, id_producto: u32, id_publicador: AccountId, cantidad: u32) -> Self {
            Self {
                id,
                id_producto,
                id_publicador,
                cantidad,
                activa: true,
            }
        }
    }

    impl GestionProducto for Contrato {
        fn _agregar_producto(
            &mut self,
            nombre: String,
            categoria: Categoria,
            cantidad: u32,
            precio: Balance,
            descripcion: String,
        ) -> Result<String, ErroresContrato> {
            // Chequeo si el producto no existe
            if self.get_producto_by_name(&nombre, &categoria).is_ok() {
                return Err(ErroresContrato::ProductoYaExistente);
            }
            // Agregar producto
            let id = self
                .productos
                .len()
                .checked_add(1)
                .ok_or(ErroresContrato::MaximoAlcanzado)? as u32;

            let nuevo_producto =
                Producto::new(id, nombre, categoria, cantidad, precio, descripcion);

            self.productos.push(&nuevo_producto);
            Ok(String::from("El producto fue registrado correctamente"))
        }

        fn _listar_productos(&self) -> Vec<Producto> {
            let mut resultado = Vec::new();
            for i in 0..self.productos.len() {
                if let Some(producto) = self.productos.get(i) {
                    resultado.push(producto);
                }
            }
            resultado
        }

        fn get_producto_by_name(
            &self,
            nombre: &String,
            categoria: &Categoria,
        ) -> Result<u32, ErroresContrato> {
            for i in 0..self.productos.len() {
                //
                let producto = self
                    .productos
                    .get(i)
                    .ok_or(ErroresContrato::ProductoInexistente)?;
                if producto.nombre == *nombre && producto.categoria == *categoria {
                    return Ok(producto.id);
                }
            }
            Err(ErroresContrato::ProductoInexistente)
        }

        fn get_producto_by_id(&self, id_producto: u32) -> Result<Producto, ErroresContrato> {
            let producto = self
                .productos
                .get(id_producto)
                .ok_or(ErroresContrato::ProductoInexistente)?;
            Ok(producto)
        }
    }

    impl GestionUsuario for Contrato {
        fn _registrar_usuario(
            &mut self,
            id: AccountId,
            nombre: String,
            mail: String,
            roles: Vec<Rol>,
        ) -> Result<String, ErroresContrato> {
            // Verifico que el usuario y el mail no existan
            if self.get_usuario_by_id(&id).is_ok() {
                return Err(ErroresContrato::UsuarioYaExistente);
            };
            if self.get_usuario_by_mail(&mail).is_ok() {
                return Err(ErroresContrato::MailYaExistente);
            };

            // Instancio nuevo usuario
            let usuario = Usuario {
                mail,
                nombre: nombre.clone(),
                roles,
            };

            // Inserto el usuario tanto en el Mapping como en el Vec
            self.m_usuarios.insert(id, &usuario);
            self.v_usuarios.push(id);

            Ok(String::from("El usuario fue registrado correctamente"))
        }

        fn _listar_usuarios(&self) -> Vec<Usuario> {
            let mut resultado = Vec::new();
            for i in 0..self.v_usuarios.len() {
                if let Some(account_id) = self.v_usuarios.get(i) {
                    if let Some(usuario) = self.m_usuarios.get(account_id) {
                        resultado.push(usuario);
                    }
                }
            }
            resultado
        }

        /// Verifica si ya existe un usuario con el mail dado
        fn get_usuario_by_mail(&self, mail: &str) -> Result<Usuario, ErroresContrato> {
            for i in 0..self.v_usuarios.len() {
                if let Some(account_id) = self.v_usuarios.get(i) {
                    if let Some(usuario) = self.m_usuarios.get(account_id) {
                        if usuario.mail == mail {
                            return Ok(usuario);
                        }
                    }
                }
            }
            Err(ErroresContrato::MailInexistente)
        }

        /// Verifica si existe un usuario con el AccountId dado
        fn get_usuario_by_id(&self, id: &AccountId) -> Result<Usuario, ErroresContrato> {
            self.m_usuarios
                .get(id)
                .ok_or(ErroresContrato::CuentaNoRegistrada)
        }
    }

    impl GestionOrden for Contrato {
        fn _registrar_orden(
            &mut self,
            id_publicacion: u32,
            id_comprador: AccountId,
            cantidad: u32,
        ) -> Result<String, ErroresContrato> {
            //TODO: Chequear si se dispone de suficiente stock en la publi para la cantidad solicitada

            let id = self
                .ordenes
                .len()
                .checked_add(1)
                .ok_or(ErroresContrato::MaximoAlcanzado)?;

            let publicacion = self
                .publicaciones
                .get(id_publicacion)
                .ok_or(ErroresContrato::PublicacionNoExiste)?;

            let producto = self
                .productos
                .get(publicacion.id_producto)
                .ok_or(ErroresContrato::PublicacionNoExiste)?;

            let precio_total = producto
                .precio
                .checked_mul(cantidad as u128)
                .ok_or(ErroresContrato::MaximoAlcanzado)?;

            let orden = Orden::new(
                id,
                id_publicacion,
                publicacion.id_publicador,
                id_comprador,
                cantidad,
                precio_total,
            );

            self.ordenes.push(&orden);

            //TODO: Descontar el stock de la cantidad de la publicacion.
            //      Si la cantidad llega a 0, la publicacion deberia desactivarse.

            Ok(String::from("Orden generada correctamente"))
        }

        fn _listar_ordenes(&self) -> Vec<Orden> {
            let mut resultado = Vec::new();
            for i in 0..self.ordenes.len() {
                if let Some(orden) = self.ordenes.get(i) {
                    resultado.push(orden);
                }
            }
            resultado
        }

        fn _enviar_orden(&mut self, id_orden: u32) -> Result<(), ErroresContrato> {
            let mut orden = self
                .ordenes
                .get(id_orden)
                .ok_or(ErroresContrato::OrdenInexistente)?;

            match orden.status {
                EstadoOrden::Pendiente => {
                    orden.status = EstadoOrden::Enviada;
                    self.ordenes.set(id_orden, &orden);
                    Ok(())
                }
                _ => Err(ErroresContrato::OrdenNoPendiente),
            }
        }

        fn _recibir_orden(&mut self, id_orden: u32) -> Result<(), ErroresContrato> {
            let mut orden = self
                .ordenes
                .get(id_orden)
                .ok_or(ErroresContrato::OrdenInexistente)?;

            match orden.status {
                EstadoOrden::Enviada => {
                    orden.status = EstadoOrden::Recibida;
                    self.ordenes.set(id_orden, &orden);
                    Ok(())
                }
                _ => Err(ErroresContrato::OrdenNoEnviada),
            }
        }

        fn _cancelar_orden(&mut self, id_orden: u32) -> Result<(), ErroresContrato> {
            let mut orden = self
                .ordenes
                .get(id_orden)
                .ok_or(ErroresContrato::OrdenInexistente)?;

            match orden.status {
                EstadoOrden::Enviada => Err(ErroresContrato::OrdenNoEnviada),
                _ => {
                    orden.status = EstadoOrden::Recibida;
                    self.ordenes.set(id_orden, &orden);
                    Ok(())
                }
            }
        }
    }

    impl GestionPublicacion for Contrato {
        fn _registrar_publicacion(
            &mut self,
            id_producto: u32,
            id_publicador: AccountId,
            cantidad: u32,
        ) -> Result<String, ErroresContrato> {
            //TODO: chequear si se dispone del stock del Producto antes de generar la publicacion

            let id = self
                .publicaciones
                .len()
                .checked_add(1)
                .ok_or(ErroresContrato::MaximoAlcanzado)?;

            let publicacion = Publicacion::new(id, id_producto, id_publicador, cantidad);

            self.publicaciones.push(&publicacion);

            //TODO: Descontar el stock de la cantidad del Producto.

            Ok(String::from("Publicacion registrada correctamente!"))
        }

        fn _listar_publicaciones(&self) -> Vec<Publicacion> {
            let mut resultado = Vec::new();
            for i in 0..self.publicaciones.len() {
                if let Some(publi) = self.publicaciones.get(i) {
                    resultado.push(publi);
                }
            }
            resultado
        }
    }

    impl Contrato {
        /// Constructor del contrato.
        ///
        /// Inicializa todas las estructuras de almacenamiento (`Mapping` y `Vec`) vacías.
        ///
        /// Se ejecuta una única vez al desplegar el contrato en la blockchain.
        /// No realiza ninguna lógica adicional.
        ///
        /// # Retorna
        /// Una instancia del contrato lista para ser utilizada.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                m_usuarios: Mapping::default(),
                v_usuarios: Vec::new(),
                productos: StorageVec::default(),
                ordenes: StorageVec::default(),
                publicaciones: StorageVec::default(),
                categorias: StorageVec::default(),
            }
        }

        /// Registra un nuevo usuario en el contrato, vinculándolo con su AccountId.
        ///
        /// # Parámetros
        /// - `nombre`: Nombre del usuario.
        /// - `mail`: Correo electrónico del usuario.
        /// - `roles`: Lista de roles asignados al usuario (Comprador, Vendedor).
        ///
        /// # Errores
        /// - `UsuarioYaExistente` si el usuario ya está registrado.
        /// - `MailYaExistente` si ya hay un usuario registrado con ese mail.
        #[ink(message)]
        pub fn registrar_usuario(
            &mut self,
            nombre: String,
            mail: String,
            roles: Vec<Rol>,
        ) -> Result<String, ErroresContrato> {
            return self._registrar_usuario(self.env().caller(), nombre, mail, roles);
        }

        /// Registra un nuevo producto en el contrato, asignándolo al AccountId que lo publica.
        ///
        /// # Parámetros
        /// - `nombre`: Nombre del producto.
        /// - `categoria`: Categoría del producto.
        /// - `cantidad`: Cantidad disponible.
        /// - `precio`: Precio unitario.
        /// - `descripcion`: Descripción del producto.
        ///
        /// # Requisitos
        /// - El usuario debe estar registrado previamente.
        ///
        /// # Errores
        /// - `CuentaNoRegistrada` si el usuario no está registrado.
        /// - `ProductoYaExistente` si ya existe un producto con ese nombre y categoría.
        /// - `MaximoAlcanzado` si no se puede generar un nuevo ID.
        #[ink(message)]
        pub fn registrar_producto(
            &mut self,
            nombre: String,
            categoria: Categoria,
            cantidad: u32,
            precio: Balance,
            descripcion: String,
        ) -> Result<String, ErroresContrato> {
            // Comprobar que el usuario esta registrado en la plataforma
            self._usuario_existe()?;
            return self._agregar_producto(nombre, categoria, cantidad, precio, descripcion);
        }

        /// Publica un producto previamente registrado en el contrato, generando una publicación activa.
        ///
        /// # Parámetros
        /// - `id_producto`: ID del producto a publicar.
        /// - `cantidad`: Cantidad disponible para la publicación.
        ///
        /// # Requisitos
        /// - El caller debe estar registrado y tener rol de `Vendedor`.
        ///
        /// # Errores
        /// - `CuentaNoRegistrada` si el caller no está registrado.
        /// - `UsuarioSinRoles` si el caller no tiene el rol adecuado.
        /// - `ProductoInexistente` si el producto no existe.
        #[ink(message)]
        pub fn publicar_producto(
            &mut self,
            id_producto: u32,
            cantidad: u32,
        ) -> Result<String, ErroresContrato> {
            // Compruebo que el usuario existe y posee rol de vendedor
            self._usuario_con_rol(Rol::Vendedor)?;

            //Registro publicacion
            let id_comprador = self.env().caller();
            self._registrar_publicacion(id_producto, id_comprador, cantidad)?;

            Ok(String::from("Publicacion creada correctamente!"))
        }

        /// Crea una orden de compra sobre una publicación activa.
        ///
        /// # Parámetros
        /// - `id_publicacion`: ID de la publicación a comprar.
        /// - `cantidad`: Cantidad solicitada.
        ///
        /// # Requisitos
        /// - El caller debe estar registrado y tener rol de `Comprador`.
        ///
        /// # Errores
        /// - `CuentaNoRegistrada` si el caller no está registrado.
        /// - `UsuarioSinRoles` si no tiene el rol correspondiente.
        /// - `PublicacionNoExiste` si no existe la publicación.
        /// - `ProductoInexistente` si el producto vinculado a la publicación no existe.
        #[ink(message)]
        pub fn comprar_producto(
            &mut self,
            id_publicacion: u32,
            cantidad: u32,
        ) -> Result<String, ErroresContrato> {
            // Compruebo que el usuario existe y posee rol de comprador
            self._usuario_con_rol(Rol::Comprador)?;

            //Registro orden de compra
            let id_comprador = self.env().caller();
            self._registrar_orden(id_publicacion, id_comprador, cantidad)?;
            Ok(String::from("Orden emitida correctamente!"))
        }

        /// Devuelve una lista de todos los productos registrados en el contrato.
        #[ink(message)]
        pub fn listar_productos(&self) -> Vec<Producto> {
            self._listar_productos()
        }

        /// Marca una orden como `Enviada`.
        ///
        /// # Parámetros
        /// - `id_orden`: ID de la orden a actualizar.
        ///
        /// # Requisitos
        /// - El caller debe estar registrado y tener rol de `Vendedor`.
        ///
        /// # Errores
        /// - `OrdenInexistente` si no existe la orden.
        /// - `OrdenNoPendiente` si la orden ya fue enviada, recibida o cancelada.
        /// - `CuentaNoRegistrada` si el caller no está registrado.
        /// - `UsuarioSinRoles` si no tiene el rol correspondiente.
        #[ink(message)]
        pub fn enviar_producto(&mut self, id_orden: u32) -> Result<String, ErroresContrato> {
            // Compruebo que el usuario existe y posee rol de vendedor
            self._usuario_con_rol(Rol::Vendedor)?;
            self._enviar_orden(id_orden)?;
            Ok(String::from("La orden fue cancelada correctamente"))
        }

        /// Marca una orden como `Recibida`.
        ///
        /// # Parámetros
        /// - `id_orden`: ID de la orden a actualizar.
        ///
        /// # Requisitos
        /// - El caller debe estar registrado y tener rol de `Comprador`.
        ///
        /// # Errores
        /// - `OrdenInexistente` si no existe la orden.
        /// - `OrdenNoEnviada` si la orden aún no fue enviada.
        /// - `CuentaNoRegistrada` si el caller no está registrado.
        /// - `UsuarioSinRoles` si no tiene el rol correspondiente.
        #[ink(message)]
        pub fn recibir_producto(&mut self, id_orden: u32) -> Result<String, ErroresContrato> {
            // Compruebo que el usuario existe y posee rol de vendedor
            self._usuario_con_rol(Rol::Comprador)?;
            self._recibir_orden(id_orden)?;
            Ok(String::from("La orden fue cancelada correctamente"))
        }

        /// Cancela una orden pendiente o aún no enviada.
        ///
        /// # Parámetros
        /// - `id_orden`: ID de la orden a cancelar.
        ///
        /// # Requisitos
        /// - El caller debe estar registrado y tener rol de `Comprador`.
        ///
        /// # Errores
        /// - `OrdenInexistente` si la orden no existe.
        /// - `OrdenYaCancelada` si ya fue cancelada previamente.
        /// - `CuentaNoRegistrada` si el caller no está registrado.
        /// - `UsuarioSinRoles` si no tiene el rol correspondiente.
        #[ink(message)]
        pub fn cancelar_producto(&mut self, id_orden: u32) -> Result<String, ErroresContrato> {
            // Compruebo que el usuario existe y posee rol de vendedor
            self._usuario_con_rol(Rol::Comprador)?; //TODO: me parece que habia una logica adicional en el cancelar... CHEQUEAR
            self._cancelar_orden(id_orden)?;
            Ok(String::from("La orden fue cancelada correctamente"))
        }

        /// Devuelve una lista de todos los usuarios registrados en el contrato.
        #[ink(message)]
        pub fn listar_usuarios(&self) -> Vec<Usuario> {
            self._listar_usuarios()
        }

        /// Devuelve una lista de todas las publicaciones en el contrato.
        #[ink(message)]
        pub fn listar_publicaciones(&self) -> Vec<Publicacion> {
            self._listar_publicaciones()
        }

        /// Devuelve una lista de todas las ordenes de compra registradas en el contrato.
        #[ink(message)]
        pub fn listar_ordenes(&self) -> Vec<Orden> {
            self._listar_ordenes()
        }

        fn _usuario_existe(&self) -> Result<(), ErroresContrato> {
            let caller = &self.env().caller();
            if self.get_usuario_by_id(caller).is_err() {
                return Err(ErroresContrato::CuentaNoRegistrada);
            };
            Ok(())
        }

        fn _usuario_con_rol(&self, rol: Rol) -> Result<(), ErroresContrato> {
            let caller = self.env().caller();
            let usuario = self
                .m_usuarios
                .get(caller)
                .ok_or(ErroresContrato::CuentaNoRegistrada)?;
            if usuario.roles.contains(&rol) {
                return Ok(());
            }
            Err(ErroresContrato::UsuarioSinRoles)
        }
    }
}
