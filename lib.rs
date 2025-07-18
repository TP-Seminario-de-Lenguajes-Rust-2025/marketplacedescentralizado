#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod contract {
    use ink::{
        prelude::{string::String, vec::Vec},
        storage::{traits::StorageLayout, Mapping, StorageVec},
    };
    use scale::{Decode, Encode};
    use scale_info::TypeInfo;
    //use scale_info::prelude::vec::Vec;

    const COMPRADOR: Rol = Rol::Comprador;
    const VENDEDOR: Rol = Rol::Vendedor;

    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[derive(Debug)]
    pub enum ErroresContrato {
        UsuarioSinRoles,
        UsuarioYaExistente,
        UsuarioNoEsComprador,
        UsuarioYaEsComprador,
        UsuarioYaEsVendedor,
        UsuarioNoExiste,
        OrdenNoPendiente,
        OrdenNoEnviada,
        OrdenYaCancelada,
        OrdenInexistente,
        StockPublicacionInsuficiente,
        StockProductoInsuficiente,
        StockInsuficiente,
        CuentaNoRegistrada,
        MailYaExistente,
        MailInexistente,
        ProductoInexistente,
        ProductoYaExistente,
        PublicacionNoExiste,
        CategoriaYaExistente,
        CategoriaInexistente,
        ErrorMultiplicacion,
        RolNoApropiado,
        AccountIdInvalida,
        IndiceInvalido,
        AlreadyHasRol,
        CantidadEnCarritoMenorAUno,
    }

    pub trait GestionProducto {
        fn _crear_producto(
            &mut self,
            id_vendedor: AccountId,
            nombre: String,
            descripcion: String,
            categoria: String,
            stock: u32,
        ) -> Result<(), ErroresContrato>;

        fn descontar_stock_producto(
            &mut self,
            id: u32,
            cantidad: u32,
        ) -> Result<(), ErroresContrato>;

        fn producto_existe(&self, p: &Producto) -> bool;

        fn _listar_productos(&self) -> Vec<Producto>;
    }

    pub trait GestionUsuario {
        fn _registrar_usuario(
            &mut self,
            id: AccountId,
            nombre: String,
            mail: String,
        ) -> Result<String, ErroresContrato>;

        fn get_user(&mut self, id: &AccountId) -> Result<Usuario, ErroresContrato>;

        fn _listar_usuarios(&self) -> Vec<Usuario>;

        fn get_usuario_by_mail(&self, mail: &str) -> Result<Usuario, ErroresContrato>;

        //fn _usuario_con_rol(&self, rol: Rol) -> Result<(), ErroresContrato>;

        fn _asignar_rol(&mut self, id: AccountId, rol: Rol) -> Result<String, ErroresContrato>;
    }

    pub trait GestionOrden {
        fn _crear_orden(
            &mut self,
            id_pub: u32,
            id_comprador: AccountId,
            cantidad: u32,
        ) -> Result<(), ErroresContrato>;

        fn _listar_ordenes(&self) -> Vec<Orden>;

        fn _enviar_orden(&mut self, id_orden: u32) -> Result<(), ErroresContrato>;

        fn _recibir_orden(&mut self, id_orden: u32) -> Result<(), ErroresContrato>;

        //fn _cancelar_orden(&mut self, id_orden: u32) -> Result<(), ErroresContrato>;
    }

    pub trait GestionPublicacion {
        fn _crear_publicacion(
            &mut self,
            id_producto: u32,
            id_usuario: AccountId,
            stock: u32,
            precio: Balance,
        ) -> Result<(), ErroresContrato>;

        fn descontar_stock_publicacion(
            &mut self,
            id_pub: u32,
            cantidad: u32,
        ) -> Result<(), ErroresContrato>;

        fn get_precio_unitario(&self, id_pub: u32) -> Result<Balance, ErroresContrato>;

        fn get_id_vendedor(&self, id_pub: u32) -> Result<AccountId, ErroresContrato>;

        fn _listar_publicaciones(&self) -> Vec<Publicacion>;
    }

    pub trait GestionCategoria {
        fn _registrar_categoria(&mut self, nombre: String) -> Result<String, ErroresContrato>;

        fn _listar_categorias(&self) -> Vec<Categoria>;

        fn get_categoria_by_name(&self, nombre: &String) -> Result<u32, ErroresContrato>;

        fn clean_cat_name(&self, nombre: &String) -> String;
    }

    pub trait ControlStock {
        fn get_cantidad(&self) -> u32;

        fn set_cantidad(&mut self, nueva: u32);

        fn descontar_stock(&mut self, cantidad_a_descontar: u32) -> Result<(), ErroresContrato> {
            //self.chequear_stock_disponible(cantidad_a_descontar)?;
            let nueva_cantidad = self
                .get_cantidad()
                .checked_sub(cantidad_a_descontar)
                .ok_or(ErroresContrato::StockInsuficiente)?;
            self.set_cantidad(nueva_cantidad);
            Ok(())
        }

        // fn chequear_stock_disponible(
        //     &self,
        //     cantidad_a_descontar: u32,
        // ) -> Result<(), ErroresContrato> {
        //     if self.get_cantidad() < cantidad_a_descontar {
        //         return Err(ErroresContrato::SinStockDisponible);
        //     }
        //     Ok(())
        // }
    }

    ///Estructura principal del contrato
    #[ink(storage)]
    pub struct Sistema {
        m_usuarios: Mapping<AccountId, Usuario>,
        v_usuarios: StorageVec<AccountId>,
        productos: StorageVec<Producto>,
        ordenes: StorageVec<Orden>,
        publicaciones: StorageVec<Publicacion>,
        categorias: StorageVec<Categoria>,
    }

    impl Sistema {
        /// #Constructor del contrato.
        ///
        /// Inicializa todas las estructuras de almacenamiento (`Mapping` y `Vec`) vacías.
        ///
        /// Se ejecuta una única vez al desplegar el contrato en la blockchain.
        /// No realiza ninguna lógica adicional.
        ///
        /// Retorna una instancia del contrato lista para ser utilizada.
        #[ink(constructor)]
        pub fn new() -> Self {
            Sistema {
                m_usuarios: Mapping::default(),
                v_usuarios: StorageVec::new(),
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
        ) -> Result<String, ErroresContrato> {
            let id = self.env().caller();
            self._registrar_usuario(id, nombre, mail)
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
        #[ink(message)]
        pub fn crear_publicacion(
            &mut self,
            id_producto: u32,
            stock: u32,
            precio: Balance,
        ) -> Result<(), ErroresContrato> {
            let id_usuario = self.env().caller();
            self._crear_publicacion(id_producto, id_usuario, stock, precio)
        }

        /// Registra una nueva categoría de productos en el contrato.
        ///
        /// # Parámetros
        /// - `nombre`: Nombre de la categoría a registrar.
        /// - `descripcion`: Descripción detallada de la categoría.
        ///
        /// # Requisitos
        /// - El caller debe estar previamente registrado como usuario.
        ///
        /// # Errores
        /// - `CuentaNoRegistrada`: Si el usuario que intenta registrar la categoría no está registrado.
        /// - `CategoriaYaExistente`: Si la categoria ya existe actualmente.
        #[ink(message)]
        pub fn registrar_categoria(&mut self, nombre: String) -> Result<String, ErroresContrato> {
            // Comprobar que el usuario esta registrado en la plataforma
            self.get_user(&self.env().caller())?;
            self._registrar_categoria(nombre)
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
        pub fn crear_producto(
            &mut self,
            nombre: String,
            descripcion: String,
            categoria: String,
            stock: u32,
        ) -> Result<(), ErroresContrato> {
            let id_vendedor = self.env().caller();
            self._crear_producto(id_vendedor, nombre, descripcion, categoria, stock)
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
        pub fn crear_orden(
            &mut self,
            id_pub: u32,
            cantidad: u32,
            //precio_total: Decimal
        ) -> Result<(), ErroresContrato> {
            let id_comprador = self.env().caller();
            self._crear_orden(id_pub, id_comprador, cantidad)
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
            self._usuario_con_rol(VENDEDOR)?;
            self._enviar_orden(id_orden)?;
            Ok(String::from("La orden fue enviada correctamente"))
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
            self._usuario_con_rol(COMPRADOR)?;
            self._recibir_orden(id_orden)?;
            Ok(String::from("La orden fue recibida correctamente"))
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
        ///
        // #[ink(message)]
        // pub fn cancelar_producto(&mut self, id_orden: u32) -> Result<String, ErroresContrato> {
        //     // Compruebo que el usuario existe y posee rol de vendedor
        //     self._usuario_con_rol(COMPRADOR)?; //TODO: me parece que habia una logica adicional en el cancelar... CHEQUEAR
        //     self._cancelar_orden(id_orden)?;
        //     Ok(String::from("La orden fue cancelada correctamente"))
        // }

        ///FALTA DOCUMENTAR PARA ROL
        #[ink(message)]
        pub fn asignar_rol(&mut self, rol: Rol) -> Result<String, ErroresContrato> {
            self._asignar_rol(self.env().caller(), rol)
        }

        /// Devuelve una lista de todos los usuarios registrados en el contrato.
        #[ink(message)]
        pub fn listar_usuarios(&self) -> Vec<Usuario> {
            self._listar_usuarios()
        }

        /// Devuelve una lista de todos los productos registrados en el contrato.
        #[ink(message)]
        pub fn listar_productos(&self) -> Vec<Producto> {
            self._listar_productos()
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

        /// Devuelve una lista de todas las categorias registradas en el contrato.
        #[ink(message)]
        pub fn listar_categorias(&self) -> Vec<Categoria> {
            self._listar_categorias()
        }

        fn _usuario_con_rol(&self, rol: Rol) -> Result<(), ErroresContrato> {
            let caller = self.env().caller();
            let usuario = self
                .m_usuarios
                .get(caller)
                .ok_or(ErroresContrato::CuentaNoRegistrada)?;
            if usuario.has_role(rol) {
                return Ok(());
            }
            Err(ErroresContrato::RolNoApropiado)
        }
    }

    impl GestionProducto for Sistema {
        fn _crear_producto(
            &mut self,
            id_vendedor: AccountId,
            nombre: String,
            descripcion: String,
            categoria: String,
            stock: u32,
        ) -> Result<(), ErroresContrato> {
            let id = self.productos.len();
            let usuario = self.get_user(&id_vendedor)?;
            if usuario.has_role(VENDEDOR) {
                let id_cat = self.get_categoria_by_name(&categoria)?;
                let producto = Producto::new(id, id_vendedor, nombre, descripcion, id_cat, stock);
                if !self.producto_existe(&producto) {
                    self.productos.push(&producto);
                    Ok(())
                } else {
                    return Err(ErroresContrato::ProductoYaExistente);
                }
            } else {
                return Err(ErroresContrato::UsuarioNoEsComprador);
            }
        }

        fn producto_existe(&self, p: &Producto) -> bool {
            for i in 0..self.productos.len() {
                if let Some(prod) = self.productos.get(i) {
                    if prod.eq(p) {
                        return true;
                    }
                }
            }
            false
        }

        fn descontar_stock_producto(
            &mut self,
            id: u32,
            cantidad: u32,
        ) -> Result<(), ErroresContrato> {
            let producto = self
                .productos
                .get(id)
                .ok_or(ErroresContrato::ProductoInexistente)?; //misma duda que en get_id_vendedor
            producto
                .stock
                .checked_sub(cantidad)
                .ok_or(ErroresContrato::StockProductoInsuficiente)?;
            self.productos.set(id, &producto);
            Ok(())
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
    }

    impl GestionUsuario for Sistema {
        fn _registrar_usuario(
            &mut self,
            id: AccountId,
            nombre: String,
            mail: String,
        ) -> Result<String, ErroresContrato> {
            // Verifico que el usuario y el mail no existan
            if self.get_user(&id).is_ok() {
                return Err(ErroresContrato::UsuarioYaExistente);
            };
            //self.get_usuario_by_mail(&mail)?;
            if self.get_usuario_by_mail(&mail).is_ok() {
                return Err(ErroresContrato::MailYaExistente);
            };

            // Instancio nuevo usuario
            let usuario = Usuario::new(id, nombre, mail);

            // Inserto el usuario tanto en el Mapping como en el Vec
            self.m_usuarios.insert(id, &usuario);
            self.v_usuarios.push(&id);

            Ok(String::from("El usuario fue registrado correctamente"))
        }

        ///Devuelve el usuario segun el AccountId provisto
        fn get_user(&mut self, id: &AccountId) -> Result<Usuario, ErroresContrato> {
            self.m_usuarios
                .get(id)
                .ok_or(ErroresContrato::UsuarioNoExiste)
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
                    } else {
                        return Err(ErroresContrato::AccountIdInvalida);
                    } //esto deberia cortar si no puede encontrar un id o usuario, ya que no esta recorriendo efectivamente posiciones con usuarios
                } else {
                    return Err(ErroresContrato::IndiceInvalido);
                }
            }
            Err(ErroresContrato::MailInexistente)
        }

        fn _asignar_rol(&mut self, id: AccountId, rol: Rol) -> Result<String, ErroresContrato> {
            let mut usuario = self.get_user(&id)?;
            if usuario.has_role(rol.clone()) {
                return Err(ErroresContrato::AlreadyHasRol);
            }
            usuario.roles.push(rol);
            self.m_usuarios.insert(id, &usuario);
            Ok(String::from("rol agregado correctamente"))
        }
    }

    impl GestionOrden for Sistema {
        fn _crear_orden(
            &mut self,
            id_pub: u32,
            id_comprador: AccountId,
            cantidad: u32,
        ) -> Result<(), ErroresContrato> {
            let id_orden = self.ordenes.len();
            let comprador = self.get_user(&id_comprador)?;
            let id_vendedor = self.get_id_vendedor(id_pub)?;
            let vendedor = self.get_user(&id_vendedor)?;
            let precio_producto = self.get_precio_unitario(id_pub)?;
            let precio_total = precio_producto
                .checked_mul(cantidad as u128)
                .ok_or(ErroresContrato::ErrorMultiplicacion)?;
            if comprador.has_role(COMPRADOR) && vendedor.has_role(VENDEDOR) {
                if cantidad != 0 {
                    self.descontar_stock_publicacion(id_pub, cantidad)?;
                    let orden = Orden::new(
                        id_orden,
                        id_pub,
                        id_vendedor,
                        id_comprador,
                        cantidad,
                        precio_total,
                    );
                    self.ordenes.push(&orden);
                    Ok(())
                } else {
                    return Err(ErroresContrato::CantidadEnCarritoMenorAUno);
                }
            } else {
                return Err(ErroresContrato::RolNoApropiado);
            }
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

        // fn _cancelar_orden(&mut self, id_orden: u32) -> Result<(), ErroresContrato> {
        //     let mut orden = self
        //         .ordenes
        //         .get(id_orden)
        //         .ok_or(ErroresContrato::OrdenInexistente)?;

        //     match orden.status {
        //         EstadoOrden::Enviada => Err(ErroresContrato::OrdenNoEnviada),
        //         _ => {
        //             orden.status = EstadoOrden::Recibida;
        //             self.ordenes.set(id_orden, &orden);
        //             Ok(())
        //         }
        //     }
        // }
    }

    impl GestionPublicacion for Sistema {
        fn _crear_publicacion(
            &mut self,
            id_producto: u32,
            id_usuario: AccountId,
            stock: u32,
            precio: Balance,
        ) -> Result<(), ErroresContrato> {
            let id = self.publicaciones.len();
            let usuario = self.get_user(&id_usuario)?;
            if usuario.has_role(VENDEDOR) {
                self.descontar_stock_producto(id_producto, stock)?;
                let p = Publicacion::new(id, id_producto, id_usuario, stock, precio);
                self.publicaciones.push(&p);
                Ok(())
            } else {
                Err(ErroresContrato::RolNoApropiado)
            }
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

        fn descontar_stock_publicacion(
            &mut self,
            id_pub: u32,
            cantidad: u32,
        ) -> Result<(), ErroresContrato> {
            //let index = id_pub.checked_sub(1).ok_or(ErroresContrato::ErrorComun)?;
            let publicacion = self
                .publicaciones
                .get(id_pub)
                .ok_or(ErroresContrato::PublicacionNoExiste)?;
            publicacion
                .stock
                .checked_sub(cantidad)
                .ok_or(ErroresContrato::StockPublicacionInsuficiente)?;
            self.publicaciones.set(id_pub, &publicacion);
            Ok(())
        }

        /// Recibe un ID de una publicacion y devuelve AccountId del vendedor asociado o un Error
        fn get_id_vendedor(&self, id_pub: u32) -> Result<AccountId, ErroresContrato> {
            if let Some(publicacion) = self.publicaciones.get(id_pub) {
                //get saca el elemento del vector (hay que volver a insertarlo o no?)
                Ok(publicacion.id_user)
            } else {
                Err(ErroresContrato::PublicacionNoExiste)
            }
        }

        /// Recibe un ID de una publicacion y devuelve su stock
        fn get_precio_unitario(&self, id_pub: u32) -> Result<Balance, ErroresContrato> {
            if let Some(publicacion) = self.publicaciones.get(id_pub) {
                Ok(publicacion.precio_unitario)
            } else {
                Err(ErroresContrato::PublicacionNoExiste)
            }
        }
    }

    impl GestionCategoria for Sistema {
        fn _registrar_categoria(&mut self, nombre: String) -> Result<String, ErroresContrato> {
            if self.get_categoria_by_name(&nombre).is_ok() {
                return Err(ErroresContrato::CategoriaYaExistente);
            }

            // Agregar categoria
            let id = self.categorias.len();
            let nueva_categoria = Categoria::new(id, self.clean_cat_name(&nombre));
            self.categorias.push(&nueva_categoria);

            Ok(String::from("la categoria fue registrada correctamente"))
        }

        fn _listar_categorias(&self) -> Vec<Categoria> {
            let mut resultado = Vec::new();
            for i in 0..self.categorias.len() {
                if let Some(categoria) = self.categorias.get(i) {
                    resultado.push(categoria);
                }
            }
            resultado
        }

        fn get_categoria_by_name(&self, nombre: &String) -> Result<u32, ErroresContrato> {
            let nombre_limpio = self.clean_cat_name(nombre);
            for i in 0..self.categorias.len() {
                if let Some(categoria) = self.categorias.get(i) {
                    if categoria.nombre == nombre_limpio {
                        return Ok(i);
                    }
                }
            }
            Err(ErroresContrato::CategoriaInexistente)
        }

        fn clean_cat_name(&self, nombre: &String) -> String {
            String::from(nombre.to_lowercase().trim())
        }
    }

    /// Estructuras relacionadas a Usuario

    /// Roles existentes
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    //#[derive(Debug, PartialEq, Eq, Clone)]
    #[cfg_attr(feature = "std", derive(StorageLayout))]
    #[derive(PartialEq, Clone)]
    pub enum Rol {
        Comprador,
        Vendedor,
    }

    /// Estructura que define al Usuario
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    //#[derive(Clone)]
    pub struct Usuario {
        id: AccountId,
        nombre: String,
        mail: String,
        rating: Rating,
        roles: Vec<Rol>,
    }

    impl Usuario {
        ///Crea un nuevo Usuario
        pub fn new(id: AccountId, nombre: String, mail: String) -> Usuario {
            Usuario {
                id,
                nombre,
                mail,
                rating: Rating::new(),
                roles: Vec::new(),
            }
        }

        pub fn registrar_comprador(&mut self) -> Result<(), ErroresContrato> {
            if !self.has_role(COMPRADOR) {
                self.roles.push(COMPRADOR);
                Ok(())
            } else {
                Err(ErroresContrato::UsuarioYaEsComprador)
            }
        }

        pub fn registrar_vendedor(&mut self) -> Result<(), ErroresContrato> {
            if !self.has_role(VENDEDOR) {
                self.roles.push(VENDEDOR);
                Ok(())
            } else {
                Err(ErroresContrato::UsuarioYaEsVendedor)
            }
        }

        ///Devuelve true si el usuario contiene el rol pasado por parametro
        pub fn has_role(&self, rol: Rol) -> bool {
            self.roles.contains(&rol)
        }
    }

    /// Estructura correspondiente al rating de un usuario
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[derive(Clone)]
    struct Rating {
        calificacion_comprador: (u32, u32), //cant de compras, valor cumulativo de todas las calificaciones
        calificacion_vendedor: (u32, u32),
    }

    ///Métodos de usuario
    impl Rating {
        ///crea un rating
        fn new() -> Rating {
            Rating {
                calificacion_comprador: (0, 0),
                calificacion_vendedor: (0, 0),
            }
        }
    }

    /// Estructuras relacionadas a producto

    /// Categorias
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub struct Categoria {
        id: u32,
        nombre: String,
    }

    impl Categoria {
        pub fn new(id: u32, nombre: String) -> Self {
            Self { id, nombre }
        }
    }

    ///Estructura de un producto
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub struct Producto {
        id: u32,
        id_vendedor: AccountId,
        nombre: String,
        descripcion: String,
        categoria: u32,
        stock: u32,
    }

    impl Producto {
        ///Crea un producto nuevo dado los parametros
        pub fn new(
            id: u32,
            id_vendedor: AccountId,
            nombre: String,
            descripcion: String,
            categoria: u32,
            stock: u32,
        ) -> Producto {
            //TODO: verificar que stock>0 y precio>0 y nombre y desc sean validos
            Producto {
                id,
                id_vendedor,
                nombre,
                descripcion,
                categoria,
                stock,
            }
        }

        ///Compara un producto self con un producto pasado por parametro
        pub fn eq(&self, p: &Producto) -> bool {
            if self.nombre == p.nombre && self.categoria == p.categoria {
                return true;
            }
            false
        }
    }

    impl ControlStock for Producto {
        fn get_cantidad(&self) -> u32 {
            self.stock
        }

        fn set_cantidad(&mut self, nueva: u32) {
            self.stock = nueva;
        }
    }

    ///LOGICA DE PUBLICACION

    ///Estructura de publicacion
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub struct Publicacion {
        id: u32,
        id_prod: u32,       //id del producto que contiene
        id_user: AccountId, //id del user que publica
        stock: u32,
        precio_unitario: Balance,
        activa: bool,
    }

    impl Publicacion {
        pub fn toggle_activa(&mut self) {
            self.activa = !self.activa;
        }

        pub fn is_activa(&self) -> bool {
            self.activa
        }

        // pub fn actualizar_stock(&mut self, delta: i32) -> Result<(), ErroresContrato> {}

        pub fn new(
            id: u32,
            id_producto: u32,
            id_user: AccountId,
            stock: u32,
            precio_unitario: Balance,
        ) -> Publicacion {
            Publicacion {
                id,
                id_prod: id_producto,
                id_user,
                stock,
                precio_unitario,
                activa: true,
            }
        }
    }

    impl ControlStock for Publicacion {
        fn get_cantidad(&self) -> u32 {
            self.stock
        }

        fn set_cantidad(&mut self, nueva: u32) {
            self.stock = nueva;
        }
    }

    ///Estructuras y logica de Orden
    ///Posibles estados de una Ordem
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum EstadoOrden {
        Pendiente,
        Enviada,   //solo lo puede modificar el vendedor
        Recibida,  //solo lo puede modificar el comprador
        Cancelada, //tienen que estar ambos de acuerdo y tiene que estar en estado pendiente
    }

    ///Estructura de orden
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub struct Orden {
        //info de la orden
        id: u32,
        id_publicacion: u32,
        id_vendedor: AccountId,
        id_comprador: AccountId,
        status: EstadoOrden,
        cantidad: u32,
        precio_total: Balance,
        cal_vendedor: Option<u8>,  //calificacion que recibe el vendedor
        cal_comprador: Option<u8>, //calificacion que recibe el comprador
    }

    impl Orden {
        ///crea una nueva orden
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

        //pub fn cambiar_estado
        //fn set_enviada() //solamente puede ser modificada por el vendedor
        //fn set_recibida() //solamente puede ser modificada por el comprador
        //fn cancelar() //necesitan estar de acuerdo ambos
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registra_usuario_correctamente() {
        let app = Sistema::new();
    }
}