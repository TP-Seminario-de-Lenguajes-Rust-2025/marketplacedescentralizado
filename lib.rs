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

    pub const COMPRADOR: Rol = Rol::Comprador;
    pub const VENDEDOR: Rol = Rol::Vendedor;

    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[derive(Debug, PartialEq)]

    pub enum ErroresContrato {
        UsuarioSinRoles,
        UsuarioYaExistente,
        UsuarioNoEsComprador,
        UsuarioYaEsComprador,
        UsuarioYaEsVendedor,
        UsuarioNoEsVendedor,
        UsuarioNoExiste,
        UsuarioNoTieneRol,
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
        NombreCategoriaVacio,
        MaxCategoriasAlcanzado,
        ListaSinProductos,
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

        fn get_id_vendedor(&self, id_pub: u32) -> Result<AccountId, ErroresContrato>; // HAY QUE VOLARLO A LA MIERDA EN LA 2DA ENTREGA

        fn _listar_publicaciones(&self) -> Vec<Publicacion>;
    }

    pub trait GestionCategoria {
        fn _registrar_categoria(&mut self, nombre: String) -> Result<String, ErroresContrato>;

        fn _listar_categorias(&self) -> Vec<Categoria>;

        fn get_categoria_by_name(&self, nombre: &String) -> Result<u32, ErroresContrato>;

        fn clean_cat_name(&self, nombre: &String) -> Result<String, ErroresContrato>;
    }

    // pub trait ControlStock {
    //     fn get_cantidad(&self) -> u32;

    //     fn set_cantidad(&mut self, nueva: u32);

    //     fn descontar_stock(&mut self, cantidad_a_descontar: u32) -> Result<(), ErroresContrato> {
    //         //self.chequear_stock_disponible(cantidad_a_descontar)?;
    //         let nueva_cantidad = self
    //             .get_cantidad()
    //             .checked_sub(cantidad_a_descontar)
    //             .ok_or(ErroresContrato::StockInsuficiente)?;
    //         self.set_cantidad(nueva_cantidad);
    //         Ok(())
    //     }

    //     // fn chequear_stock_disponible(
    //     //     &self,
    //     //     cantidad_a_descontar: u32,
    //     // ) -> Result<(), ErroresContrato> {
    //     //     if self.get_cantidad() < cantidad_a_descontar {
    //     //         return Err(ErroresContrato::SinStockDisponible);
    //     //     }
    //     //     Ok(())
    //     // }
    // }

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
                return Err(ErroresContrato::UsuarioNoEsVendedor);
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
            let mut producto = self
                .productos
                .get(id)
                .ok_or(ErroresContrato::ProductoInexistente)?; //misma duda que en get_id_vendedor
            producto.stock = producto
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
                let account_id = self
                    .v_usuarios
                    .get(i)
                    .ok_or(ErroresContrato::IndiceInvalido)?;

                let usuario = self
                    .m_usuarios
                    .get(account_id)
                    .ok_or(ErroresContrato::AccountIdInvalida)?;
                if usuario.mail == mail {
                    return Ok(usuario);
                };
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
            let mut publicacion = self
                .publicaciones
                .get(id_pub)
                .ok_or(ErroresContrato::PublicacionNoExiste)?;
            publicacion.stock = publicacion
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
            if self.categorias.len() == u32::MAX {
                return Err(ErroresContrato::MaxCategoriasAlcanzado);
            }
            let id = self.categorias.len();
            let nueva_categoria = Categoria::new(id, self.clean_cat_name(&nombre)?);
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
            let nombre_limpio = self.clean_cat_name(nombre)?;
            for i in 0..self.categorias.len() {
                if let Some(categoria) = self.categorias.get(i) {
                    if categoria.nombre == nombre_limpio {
                        return Ok(i);
                    }
                }
            }
            Err(ErroresContrato::CategoriaInexistente)
        }

        fn clean_cat_name(&self, nombre: &String) -> Result<String, ErroresContrato> {
            let mut limpio = String::from(nombre.to_lowercase().trim());
            limpio.truncate(100);
            if !limpio.is_empty() {
                Ok(limpio)
            } else {
                Err(ErroresContrato::NombreCategoriaVacio)
            }
        }
    }

    /// Estructuras relacionadas a Usuario

    /// Roles existentes
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(StorageLayout))]
    #[derive(PartialEq, Clone)]
    pub enum Rol {
        Comprador,
        Vendedor,
    }

    /// Estructura que define al Usuario
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[derive(Clone)]
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

        /// Remueve el rol del usuario si existe
        // pub fn remover_rol(&mut self, rol: Rol) -> Result<(), ErroresContrato> {
        //     if self.has_role(rol) {
        //         self.roles.retain(|rol| *rol != COMPRADOR);
        //         Ok(())
        //     } else {
        //         Err(ErroresContrato::UsuarioNoTieneRol)
        //     }
        // }

        /// Devuelve true si el usuario contiene el rol pasado por parametro
        pub fn has_role(&self, rol: Rol) -> bool {
            self.roles.contains(&rol)
        }

        /// Devuelve el nombre del usuario
        pub fn get_name(&self) -> String {
            self.nombre.clone()
        }

        /// Devuelve el email del usuario
        pub fn get_mail(&self) -> String {
            self.mail.clone()
        }

        /// Devuelve el AccountId del usuario
        pub fn get_id(&self) -> AccountId {
            self.id.clone()
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
    #[derive(PartialEq, Debug)]
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

    // impl ControlStock for Producto {
    //     fn get_cantidad(&self) -> u32 {
    //         self.stock
    //     }

    //     fn set_cantidad(&mut self, nueva: u32) {
    //         self.stock = nueva;
    //     }
    // }

    ///LOGICA DE PUBLICACION

    ///Estructura de publicacion
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[derive(PartialEq, Debug, Clone)]
    pub struct Publicacion {
        id: u32,
        id_prod: u32,       //id del producto que contiene
        id_user: AccountId, //id del user que publica
        stock: u32,
        precio_unitario: Balance,
        activa: bool,
    }

    impl Publicacion {
        pub fn stock(&self) -> u32 {
            self.stock
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

    // impl ControlStock for Publicacion {
    //     fn get_cantidad(&self) -> u32 {
    //         self.stock
    //     }

    //     fn set_cantidad(&mut self, nueva: u32) {
    //         self.stock = nueva;
    //     }
    // }

    ///Estructuras y logica de Orden
    ///Posibles estados de una Ordem
    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
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
    #[derive(Clone)]
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
        pub fn get_cantidad(&self) -> u32 {
            self.cantidad
        }

        pub fn get_status(&self) -> EstadoOrden {
            self.status
        }
        //pub fn cambiar_estado
        //fn set_enviada() //solamente puede ser modificada por el vendedor
        //fn set_recibida() //solamente puede ser modificada por el comprador
        //fn cancelar() //necesitan estar de acuerdo ambos
    }
}

#[cfg(test)]
mod tests {
    use crate::contract::*;

    use ink::{
        env::{test::set_callee, DefaultEnvironment},
        primitives::AccountId,
    };
    use ink_e2e::{account_id, AccountKeyring};

    fn setup_sistema() -> Sistema {
        Sistema::new()
    }

    fn id_comprador() -> <DefaultEnvironment as ink::env::Environment>::AccountId {
        account_id(AccountKeyring::Alice)
    }

    fn id_vendedor() -> <DefaultEnvironment as ink::env::Environment>::AccountId {
        account_id(AccountKeyring::Bob)
    }

    fn set_caller(caller: AccountId) {
        ink::env::test::set_caller::<DefaultEnvironment>(caller);
    }

    fn build_testing_accounts() -> (AccountId, AccountId) {
        let id_comprador = id_comprador();
        let id_vendedor = id_vendedor();
        (id_comprador, id_vendedor)
    }

    fn build_testing_setup() -> (Sistema, AccountId, AccountId) {
        let mut app = setup_sistema();
        let (user_1, user_2) = build_testing_accounts();

        app._registrar_usuario(
            user_1,
            "user_name_1".to_string(),
            "user_email_1".to_string(),
        )
        .expect("No se pudo registrar el usuario");
        app._registrar_usuario(
            user_2,
            "user_name_2".to_string(),
            "user_email_2".to_string(),
        )
        .expect("No se pudo registrar el usuario");

        (app, user_1, user_2)
    }

    //fn de test de agus olthoff

    fn registrar_comprador(
        sistema: &mut Sistema,
        id: <DefaultEnvironment as ink::env::Environment>::AccountId,
    ) {
        sistema
            ._registrar_usuario(id, "Comprador".into(), "comprador@gmail.com".into())
            .unwrap();
    }
    fn registrar_vendedor(
        sistema: &mut Sistema,
        id: <DefaultEnvironment as ink::env::Environment>::AccountId,
    ) {
        sistema
            ._registrar_usuario(id, "Vendedor".into(), "vendedor@gmail.com".into())
            .unwrap();
        sistema._asignar_rol(id, Rol::Vendedor).unwrap();
    }

    fn agregar_categoria(sistema: &mut Sistema, nombre: &str) {
        sistema._registrar_categoria(nombre.into()).unwrap();
    }

    fn contrato_con_categorias_cargada() -> Sistema {
        let mut sist = Sistema::new();
        for i in 0..10 {
            sist._registrar_categoria(String::from(format!("categoria {}", i)));
        }
        return sist;
    }

    #[ink::test]
    fn test_crear_publicacion_exito() {
        let mut sistema = setup_sistema();
        let id = id_vendedor();

        sistema
            ._registrar_usuario(id, "Vendedor".into(), "ven@mail.com".into())
            .unwrap();
        sistema._asignar_rol(id, Rol::Vendedor).unwrap();
        sistema._registrar_categoria("Ropa".into()).unwrap();
        sistema
            ._crear_producto(id, "Zapatillas".into(), "desc".into(), "Ropa".into(), 10)
            .unwrap();

        let esperado = Publicacion::new(0, 0, id, 5, 1000);
        let res = sistema._crear_publicacion(0, id, 5, 1000);

        assert!(res.is_ok());

        let retorno = sistema._listar_publicaciones()[0].clone();
        assert_eq!(esperado, retorno);
    }

    #[ink::test]
    fn test_crear_publicacion_falla_sin_rol_vendedor() {
        let mut sistema = setup_sistema();
        let id: AccountId = id_vendedor();
        let id2: AccountId = id_comprador();

        sistema
            ._registrar_usuario(id, "Vendedor".into(), "vende@mail.com".into())
            .unwrap();
        sistema
            ._registrar_usuario(id2, "Comprador".into(), "compra@mail.com".into())
            .unwrap();

        set_caller(id);
        sistema.asignar_rol(Rol::Vendedor).unwrap();
        set_caller(id2);
        sistema.asignar_rol(Rol::Comprador).unwrap();

        set_caller(id);
        sistema._registrar_categoria("Ropa".into()).unwrap();
        sistema
            ._crear_producto(id, "Remera".into(), "desc".into(), "Ropa".into(), 10)
            .unwrap();

        set_caller(id2);
        let res = sistema._crear_publicacion(0, id2, 5, 200);
        assert!(matches!(res, Err(ErroresContrato::RolNoApropiado)));
    }

    #[ink::test]
    fn test_crear_publicacion_falla_producto_inexistente() {
        let mut sistema = setup_sistema();
        let id = id_vendedor();

        sistema
            ._registrar_usuario(id, "Vendedor".into(), "ven@mail.com".into())
            .unwrap();
        sistema._asignar_rol(id, Rol::Vendedor).unwrap();

        let res = sistema._crear_publicacion(99, id, 3, 500); // id_producto inválido
        assert!(matches!(res, Err(ErroresContrato::ProductoInexistente)));
    }

    #[ink::test]
    fn test_crear_publicacion_falla_stock_insuficiente() {
        let mut sistema = setup_sistema();
        let id = id_vendedor();

        sistema
            ._registrar_usuario(id, "Vendedor".into(), "ven@mail.com".into())
            .unwrap();
        sistema._asignar_rol(id, Rol::Vendedor).unwrap();
        sistema._registrar_categoria("Ropa".into()).unwrap();
        sistema
            ._crear_producto(id, "Pantalon".into(), "desc".into(), "Ropa".into(), 2)
            .unwrap();

        let res = sistema._crear_publicacion(0, id, 5, 300); // pide más stock del disponible
        assert!(matches!(
            res,
            Err(ErroresContrato::StockProductoInsuficiente)
        ));
    }

    #[ink::test]
    fn test_crear_publicacion_falla_usuario_inexistente() {
        let mut sistema = setup_sistema();
        let id = id_vendedor(); // Nunca lo registramos

        let res = sistema._crear_publicacion(0, id, 5, 1000);
        assert!(matches!(res, Err(ErroresContrato::UsuarioNoExiste)));
    }

    #[ink::test]
    fn test_descontar_stock_publicacion_exito() {
        let mut sistema = setup_sistema();
        let id = id_vendedor();
        set_caller(id);

        sistema
            ._registrar_usuario(id, "Vendedor".into(), "ven@mail.com".into())
            .unwrap();
        sistema._asignar_rol(id, Rol::Vendedor).unwrap();
        sistema._registrar_categoria("Ropa".into()).unwrap();
        sistema
            ._crear_producto(id, "Zapatillas".into(), "desc".into(), "Ropa".into(), 10)
            .unwrap();
        sistema._crear_publicacion(0, id, 5, 1000).unwrap();

        let res = sistema.descontar_stock_publicacion(0, 2);
        assert!(res.is_ok());

        let publicaciones = sistema._listar_publicaciones();
        assert_eq!(publicaciones[0].stock(), 3);
    }

    #[ink::test]
    fn test_descontar_stock_publicacion_falla_publicacion_inexistente() {
        let mut sistema = setup_sistema();
        let id = id_vendedor();
        set_caller(id);

        let res = sistema.descontar_stock_publicacion(99, 1); // ID inválido
        assert!(matches!(res, Err(ErroresContrato::PublicacionNoExiste)));
    }

    #[ink::test]
    fn test_descontar_stock_publicacion_falla_stock_insuficiente() {
        let mut sistema = setup_sistema();
        let id = id_vendedor();
        set_caller(id);

        sistema
            ._registrar_usuario(id, "Vendedor".into(), "ven@mail.com".into())
            .unwrap();
        sistema._asignar_rol(id, Rol::Vendedor).unwrap();
        sistema._registrar_categoria("Ropa".into()).unwrap();
        sistema
            ._crear_producto(id, "Zapatillas".into(), "desc".into(), "Ropa".into(), 3)
            .unwrap();
        sistema._crear_publicacion(0, id, 3, 1000).unwrap();

        let res = sistema.descontar_stock_publicacion(0, 5); // Más de lo disponible
        assert!(matches!(
            res,
            Err(ErroresContrato::StockPublicacionInsuficiente)
        ));
    }

    #[ink::test]
    fn test_get_precio_unitario_ok() {
        let mut sistema = setup_sistema();
        let id = id_vendedor();
        set_caller(id);

        sistema
            ._registrar_usuario(id, "Vendedor".into(), "ven@mail.com".into())
            .unwrap();
        sistema._asignar_rol(id, Rol::Vendedor).unwrap();
        sistema._registrar_categoria("Ropa".into()).unwrap();
        sistema
            ._crear_producto(id, "Campera".into(), "desc".into(), "Ropa".into(), 10)
            .unwrap();
        sistema._crear_publicacion(0, id, 4, 1234).unwrap();

        let res = sistema.get_precio_unitario(0);
        assert!(matches!(res, Ok(1234)));
    }

    #[ink::test]
    fn test_get_precio_unitario_falla_publicacion_inexistente() {
        let sistema = setup_sistema(); // no hace falta crear nada

        let res = sistema.get_precio_unitario(42);
        assert!(matches!(res, Err(ErroresContrato::PublicacionNoExiste)));
    }

    #[ink::test]
    fn test_get_id_vendedor_ok() {
        let mut sistema = setup_sistema();
        let id = id_vendedor();
        set_caller(id);

        sistema
            ._registrar_usuario(id, "Vendedor".into(), "ven@mail.com".into())
            .unwrap();
        sistema._asignar_rol(id, Rol::Vendedor).unwrap();
        sistema._registrar_categoria("Ropa".into()).unwrap();
        sistema
            ._crear_producto(id, "Pantalón".into(), "desc".into(), "Ropa".into(), 8)
            .unwrap();
        sistema._crear_publicacion(0, id, 5, 999).unwrap();

        let res = sistema.get_id_vendedor(0);
        assert!(matches!(res, Ok(valor) if valor == id));
    }

    #[ink::test]
    fn test_get_id_vendedor_falla_publicacion_inexistente() {
        let sistema = setup_sistema();

        let res = sistema.get_id_vendedor(42);
        assert!(matches!(res, Err(ErroresContrato::PublicacionNoExiste)));
    }

    #[ink::test]
    fn test_listar_productos_vacio() {
        let sistema = setup_sistema();
        let productos = sistema._listar_productos();
        assert_eq!(productos.len(), 0);
    }

    #[ink::test]
    fn test_listar_productos_uno() {
        let mut sistema = setup_sistema();
        let id = id_vendedor();
        set_caller(id);

        sistema
            ._registrar_usuario(id, "Vendedor".into(), "ven@mail.com".into())
            .unwrap();
        sistema._asignar_rol(id, Rol::Vendedor).unwrap();
        sistema._registrar_categoria("Ropa".into()).unwrap();
        sistema
            ._crear_producto(id, "Buzo".into(), "desc".into(), "Ropa".into(), 5)
            .unwrap();

        let productos = sistema._listar_productos();
        assert_eq!(productos.len(), 1);

        // Si `Producto` implementa PartialEq:
        let esperado = Producto::new(0, id, "Buzo".into(), "desc".into(), 0, 5);
        assert_eq!(productos[0], esperado);
    }

    #[ink::test]
    fn test_listar_productos_varios() {
        let mut sistema = setup_sistema();
        let id = id_vendedor();
        set_caller(id);

        sistema
            ._registrar_usuario(id, "Vendedor".into(), "ven@mail.com".into())
            .unwrap();
        sistema._asignar_rol(id, Rol::Vendedor).unwrap();
        sistema._registrar_categoria("Ropa".into()).unwrap();

        sistema
            ._crear_producto(id, "Buzo".into(), "desc".into(), "Ropa".into(), 5)
            .unwrap();
        sistema
            ._crear_producto(id, "Campera".into(), "desc".into(), "Ropa".into(), 8)
            .unwrap();

        let productos = sistema._listar_productos();
        assert_eq!(productos.len(), 2);
    }

    #[ink::test]
    fn test_categoria_agregar_nueva() {
        let mut sist = setup_sistema();

        assert!(sist._listar_categorias().is_empty());

        let result = sist._registrar_categoria("Limpieza".to_string());

        assert_eq!(
            result,
            Ok(String::from("la categoria fue registrada correctamente"))
        );
        assert_eq!(sist._listar_categorias().len(), 1);
    }

    #[ink::test]
    fn test_categoria_agregar_duplicada() {
        let mut sist = contrato_con_categorias_cargada();

        assert!(!sist._listar_categorias().is_empty());
        let result = sist._registrar_categoria("categoria 1".to_string());
        assert_eq!(
            result,
            Err(ErroresContrato::CategoriaYaExistente),
            "deberia tirar error que ya existe la categoria"
        );
    }

    #[ink::test]
    fn test_categoria_formateo_nombre() {
        let mut sist = contrato_con_categorias_cargada();

        //nombre similar
        let result = sist._registrar_categoria("CaTEgORia 1".to_string());
        assert_eq!(
            result,
            Err(ErroresContrato::CategoriaYaExistente),
            "deberia devolver que ya existe la categoria"
        );

        //nombre vacio
        let result = sist._registrar_categoria(String::new());
        assert_eq!(
            result,
            Err(ErroresContrato::NombreCategoriaVacio),
            "deberia devolver que el nombre de la categoria esta vacia"
        );

        //nombres con unicode
        let result = sist._registrar_categoria("не ваше дела идите на хуй".to_string());
        assert_eq!(
            result,
            Ok(String::from("la categoria fue registrada correctamente")),
            "deberia poder manejar alfabeto cirilico"
        );
        let result = sist._registrar_categoria("የክፋት እቅድ".to_string());
        assert_eq!(
            result,
            Ok(String::from("la categoria fue registrada correctamente")),
            "deberia poder manejar alfabeto amharico"
        );
        let result = sist._registrar_categoria("プログラミングが好きです".to_string());
        assert_eq!(
            result,
            Ok(String::from("la categoria fue registrada correctamente")),
            "deberia poder manejar kanji, katakana e hiragana"
        );
        let result = sist._registrar_categoria("사랑해요".to_string());
        assert_eq!(
            result,
            Ok(String::from("la categoria fue registrada correctamente")),
            "deberia poder manejar hangul"
        );

        //nombre con leading y trailing whitespace
        let result = sist._registrar_categoria(
            "          alguna categoria                                                "
                .to_string(),
        );
        assert_eq!(
            result,
            Ok(String::from("la categoria fue registrada correctamente")),
            "deberia eliminar espacios en blanco al principio y final del string"
        );

        //nombre truncado

        let result = sist._registrar_categoria(
            "You know what they call a  Quarter Pounder with Cheese in Paris?

            [JULES]
            They don't call it a Quarter Pounder with Cheese?

            [VINCENT]
            No, they got the metric system there, they wouldn't know what the fuck a Quarter Pounder is.

            [JULES]
            Then what do they call it?

            [VINCENT]
            They call it Royale with Cheese.

            [JULES]
            Royale with Cheese. What do they call a Big Mac?

            [VINCENT]
            Big Mac's a Big Mac, but they call it Le Big Mac.

            [JULES]
            Le big Mac! Ahhaha, what do they call a Whopper?

            [VINCENT]
            I dunno, I didn't go into a Burger King.".to_string()
        );
        assert_eq!(
            result,
            Ok(String::from("la categoria fue registrada correctamente")),
            "deberia poder manejar nombres muy largos, truncandolos en 100 caracteres"
        );
    }

    #[ink::test]
    fn test_categoria_indice_correcto_por_nombre() {
        let sist = contrato_con_categorias_cargada();
        assert_eq!(
            sist.get_categoria_by_name(&"categoria 9".to_string()),
            Ok(9),
            "deberia devolver el indice correcto"
        );
        assert_eq!(
            sist.get_categoria_by_name(&"categoria 3".to_string()),
            Ok(3),
            "deberia devolver el indice correcto"
        );
        assert_eq!(
            sist.get_categoria_by_name(&"      categoria 4       ".to_string()),
            Ok(4),
            "deberia devolver el indice correcto incluso con whitespace"
        );
        assert_eq!(
            sist.get_categoria_by_name(&"cAtEGoRiA 5".to_string()),
            Ok(5),
            "deberia devolver el indice correcto incluso con mayusculas"
        );

        assert_eq!(
            sist.get_categoria_by_name(&"Electrodomesticos".to_string()),
            Err(ErroresContrato::CategoriaInexistente),
            "deberia devolver que no encuentra la categoria"
        );
    }

    #[ink::test]
    fn test_categoria_get_categoria_whitespaces() {
        let sist = contrato_con_categorias_cargada();
        assert_eq!(
            sist.get_categoria_by_name(&"      categoria 4       ".to_string()),
            Ok(4),
            "deberia devolver el indice correcto incluso con whitespace"
        );
    }

    #[ink::test]
    fn test_categoria_get_categoria_case_sensitivity() {
        let sist = contrato_con_categorias_cargada();
        assert_eq!(
            sist.get_categoria_by_name(&"cAtEGoRiA 5".to_string()),
            Ok(5),
            "deberia devolver el indice correcto incluso con mayusculas"
        );
    }

    #[ink::test]
    fn test_categoria_get_categoria_inexistente() {
        let sist = contrato_con_categorias_cargada();
        assert_eq!(
            sist.get_categoria_by_name(&"Electrodomesticos".to_string()),
            Err(ErroresContrato::CategoriaInexistente),
            "deberia devolver que no encuentra la categoria"
        );
    }

    #[ink::test]
    fn test_categoria_clean_name() {
        let sist = setup_sistema();
        assert_eq!(
            sist.clean_cat_name(&"Electrodomésticos".to_string()),
            Ok("electrodomésticos".to_string())
        );
    }

    #[ink::test]
    fn test_categoria_clean_name_whitespaces() {
        let sist = setup_sistema();
        assert_eq!(
            sist.clean_cat_name(&"      cocina        ".to_string()),
            Ok("cocina".to_string())
        );
    }

    #[ink::test]
    fn test_categoria_clean_name_empty() {
        let sist = setup_sistema();
        assert_eq!(
            sist.clean_cat_name(&"".to_string()),
            Err(ErroresContrato::NombreCategoriaVacio)
        );
    }

    #[ink::test]
    fn test_categoria_clean_name_max_characters() {
        let sist = setup_sistema();
        assert_eq!(sist.clean_cat_name(&"
            You know what they call a  Quarter Pounder with Cheese in Paris?

            [JULES]
            They don't call it a Quarter Pounder with Cheese?

            [VINCENT]
            No, they got the metric system there, they wouldn't know what the fuck a Quarter Pounder is.

            [JULES]
            Then what do they call it?

            [VINCENT]
            They call it Royale with Cheese.

            [JULES]
            Royale with Cheese. What do they call a Big Mac?

            [VINCENT]
            Big Mac's a Big Mac, but they call it Le Big Mac.

            [JULES]
            Le big Mac! Ahhaha, what do they call a Whopper?

            [VINCENT]
            I dunno, I didn't go into a Burger King.".to_string()
        ),
            Ok("you know what they call a  quarter pounder with cheese in paris?

            [jules]
            th".to_string())
        );
    }

    //_crear_producto
    //si el producto es exitoso (no duplicado y es vendedor)
    #[ink::test]
    fn test_crear_producto_exitoso() {
        let mut sistema = setup_sistema();
        let id = id_vendedor();

        registrar_vendedor(&mut sistema, id);
        agregar_categoria(&mut sistema, "Ropa");

        let res = sistema._crear_producto(id, "Zapatilla".into(), "desc".into(), "Ropa".into(), 10);

        assert!(res.is_ok());

        let productos = sistema._listar_productos();
        assert_eq!(productos.len(), 1);

        let esperado = Producto::new(0, id, "Zapatilla".into(), "desc".into(), 0, 10);
        assert_eq!(productos[0], esperado);
    }

    //test falla por que el producto esta duplicado
    #[ink::test]
    fn test_crear_producto_falla_si_ya_esxiste() {
        let mut sistema = setup_sistema();
        let id = id_vendedor();

        registrar_vendedor(&mut sistema, id);
        agregar_categoria(&mut sistema, "Ropa");

        let res = sistema._crear_producto(id, "Zapatilla".into(), "desc".into(), "Ropa".into(), 10);
        assert!(res.is_ok());

        let res2 =
            sistema._crear_producto(id, "Zapatilla".into(), "desc".into(), "Ropa".into(), 10);
        assert!(res2.is_err());
        assert_eq!(res2.unwrap_err(), ErroresContrato::ProductoYaExistente);
    }
    //test falla por que el usuario no es vendedor
    #[ink::test]
    fn test_crear_producto_falla_si_es_comprador() {
        let mut sistema = setup_sistema();
        let id = id_comprador();

        registrar_comprador(&mut sistema, id);
        agregar_categoria(&mut sistema, "Ropa");

        let res = sistema._crear_producto(id, "Zapatilla".into(), "desc".into(), "Ropa".into(), 10);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), ErroresContrato::UsuarioNoEsVendedor);
    }

    //producto_existe (caso existente)
    #[ink::test]
    fn test_producto_existe() {
        let mut sistema = setup_sistema();
        let id = id_vendedor();

        registrar_vendedor(&mut sistema, id);
        agregar_categoria(&mut sistema, "Ropa");

        let _ = sistema._crear_producto(id, "Zapatilla".into(), "desc".into(), "Ropa".into(), 10);

        // Armamos el producto igual al creado
        let producto = Producto::new(0, id, "Zapatilla".into(), "desc".into(), 0, 10);

        let existe = sistema.producto_existe(&producto);
        assert!(existe);
    }
    //producto_existe (caso inexistente)
    #[ink::test]
    fn test_producto_no_existe() {
        let sistema = setup_sistema();
        let id = id_vendedor();
        let producto = Producto::new(0, id, "Zapatilla".into(), "desc".into(), 0, 10);

        let existe = sistema.producto_existe(&producto);
        assert!(!existe);
    }

    // descontar_stock_producto (caso exitoso)
    #[ink::test]
    fn test_descontar_stock_producto_exitoso() {
        let mut sistema = setup_sistema();
        let id = id_vendedor();

        registrar_vendedor(&mut sistema, id);
        agregar_categoria(&mut sistema, "Ropa");

        sistema
            ._crear_producto(id, "Zapatilla".into(), "desc".into(), "Ropa".into(), 10)
            .unwrap();
        let res = sistema.descontar_stock_producto(0, 3);
        assert!(res.is_ok());

        let productos = sistema._listar_productos();
        let esperado = Producto::new(0, id, "Zapatilla".into(), "desc".into(), 0, 7);
        assert_eq!(productos[0], esperado);
    }
    // descontar_stock_producto (caso falla producto inexistente)
    #[ink::test]
    fn test_descontar_stock_falla_producto_inexistente() {
        let mut sistema = setup_sistema();
        let res = sistema.descontar_stock_producto(0, 1);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), ErroresContrato::ProductoInexistente);
    }
    // descontar_stock_producto (caso falla stock insuficiente)
    #[ink::test]
    fn test_descontar_stock_falla_stock_insuficiente() {
        let mut sistema = setup_sistema();
        let id = id_vendedor();

        registrar_vendedor(&mut sistema, id);
        agregar_categoria(&mut sistema, "Ropa");

        sistema
            ._crear_producto(id, "Zapatilla".into(), "desc".into(), "Ropa".into(), 5)
            .unwrap();

        // descuento mas stock de lo que tengo
        let res = sistema.descontar_stock_producto(0, 10);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), ErroresContrato::StockProductoInsuficiente);
    }
    #[ink::test]
    fn test_listar_productos_con_productos() {
        let mut sistema = setup_sistema();
        let id = id_vendedor();

        registrar_vendedor(&mut sistema, id);
        agregar_categoria(&mut sistema, "Ropa");

        sistema
            ._crear_producto(id, "Zapatilla".into(), "desc".into(), "Ropa".into(), 10)
            .unwrap();

        let productos = sistema._listar_productos();
        assert_eq!(productos.len(), 1);

        let esperado = Producto::new(0, id, "Zapatilla".into(), "desc".into(), 0, 10);
        assert_eq!(productos[0], esperado); //modificado
    }
    /// Test para listar productos sin productos registrado
    #[ink::test]
    fn test_listar_productos_sin_productos() {
        let sistema = setup_sistema();

        let res = sistema._listar_productos();
        assert!(res.is_empty());
    }

    #[ink::test]
    fn registra_usuario_correctamente() {
        let mut app = setup_sistema();
        let (id_comprador, _) = build_testing_accounts();

        assert!(
            app._registrar_usuario(
                id_comprador,
                "user_name".to_string(),
                "user_email".to_string()
            )
            .is_ok(),
            "Se esperaba que se registre un usuario"
        );

        let user_created = app.listar_usuarios()[0].clone();

        assert_eq!(user_created.get_name(), "user_name".to_string());
        assert_eq!(user_created.get_mail(), "user_email".to_string());
    }

    #[ink::test]
    fn devuelve_user_con_id_correctamente() {
        let (mut app, user_id, _) = build_testing_setup();
        let expected = app.listar_usuarios()[0].clone();

        assert_eq!(
            app.get_user(&user_id).unwrap().get_name(),
            expected.get_name(),
            "Se esperaba que el campo nombre coincida"
        );

        assert_eq!(
            app.get_user(&user_id).unwrap().get_mail(),
            expected.get_mail(),
            "Se esperaba que el campo mail coincida"
        );

        assert_eq!(
            app.get_user(&user_id).unwrap().get_id(),
            expected.get_id(),
            "Se esperaba que el campo ID coincida"
        );
    }

    #[ink::test]
    fn devuelve_user_con_email_correctamente() {
        let (app, _, _) = build_testing_setup();
        let expected = app.listar_usuarios()[0].clone();

        assert!(
            app.get_usuario_by_mail("not_existent_email@email.com")
                .is_err(),
            "Se esperaba un error si no existe un usuario con el email"
        );

        assert_eq!(
            app.get_usuario_by_mail(&expected.get_mail())
                .unwrap()
                .get_name(),
            expected.get_name(),
            "Se esperaba que el campo nombre coincida"
        );

        assert_eq!(
            app.get_usuario_by_mail(&expected.get_mail())
                .unwrap()
                .get_mail(),
            expected.get_mail(),
            "Se esperaba que el campo mail coincida"
        );

        assert_eq!(
            app.get_usuario_by_mail(&expected.get_mail())
                .unwrap()
                .get_id(),
            expected.get_id(),
            "Se esperaba que el campo ID coincida"
        );
    }

    #[ink::test]
    fn asigna_rol_correctamente() {
        let (mut app, user_id, _) = build_testing_setup();

        assert!(
            app._asignar_rol(user_id, Rol::Vendedor).is_ok(),
            "Se esperaba que se asigne el rol correctamente"
        );

        assert!(
            app._asignar_rol(user_id, Rol::Comprador).is_ok(),
            "Se esperaba que se asigne el rol correctamente"
        );

        assert_eq!(
            app._asignar_rol(user_id, Rol::Comprador),
            Err(ErroresContrato::AlreadyHasRol),
            "Se esperaba error AlreadyHasRol si el usuario ya tiene el error asignado"
        );
    }

    #[ink::test]
    fn listar_usuarios_correctamente() {
        let (mut app, user1_id, user2_id) = build_testing_setup();
        let expected = Vec::from([app.get_user(&user1_id), app.get_user(&user2_id)]);
        assert_eq!(
            app.listar_usuarios().len(),
            expected.len(),
            "Se esperaba que los vectores tengan el mismo largo"
        );
    }

    ///Tests gestion orden
    #[ink::test]
    fn test_crear_orden_con_exito() {
        let mut contrato = setup_sistema();
        let (id_comprador, id_vendedor) = build_testing_accounts();

        contrato
            ._registrar_usuario(
                id_comprador,
                "Juan comprador".to_string(),
                "comprador@gmail.com".to_string(),
            )
            .unwrap();
        contrato._asignar_rol(id_comprador, COMPRADOR).unwrap();

        contrato
            ._registrar_usuario(
                id_vendedor,
                "Usuario vendedor".to_string(),
                "vendedor@gmail.com".to_string(),
            )
            .unwrap();
        contrato._asignar_rol(id_vendedor, VENDEDOR).unwrap();

        contrato._registrar_categoria("Libros".to_string()).unwrap();
        contrato
            ._crear_producto(
                id_vendedor,
                "Rust Book".to_string(),
                "Desc libro".to_string(),
                "Libros".to_string(),
                10,
            )
            .unwrap();
        contrato
            ._crear_publicacion(0, id_vendedor, 10, 100)
            .unwrap();

        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(id_comprador); //setea el caller en Comprador

        let result = contrato.crear_orden(0, 2);
        assert!(result.is_ok(), "Error al crear la orden");

        let ordenes = contrato.listar_ordenes();
        assert_eq!(ordenes.len(), 1);
        assert_eq!(ordenes[0].get_cantidad(), 2);
        assert_eq!(ordenes[0].get_status(), EstadoOrden::Pendiente);
    }

    #[ink::test]
    fn test_crear_orden_con_cantidad_cero_fallido() {
        let mut contrato = setup_sistema();
        let (comprador, vendedor) = build_testing_accounts();

        contrato
            ._registrar_usuario(
                vendedor,
                "Santiago vendedor".to_string(),
                "ST96@Gmail.com".to_string(),
            )
            .unwrap();
        contrato._asignar_rol(vendedor, VENDEDOR).unwrap();

        contrato
            ._registrar_usuario(
                comprador,
                "Juan comprador".to_string(),
                "JT11@Gmail.com".to_string(),
            )
            .unwrap();
        contrato._asignar_rol(comprador, COMPRADOR).unwrap();

        contrato
            ._registrar_categoria("Electronica".to_string())
            .unwrap();
        contrato
            ._crear_producto(
                vendedor,
                "Auriculares".to_string(),
                "BT".to_string(),
                "Electronica".to_string(),
                5,
            )
            .unwrap();
        contrato._crear_publicacion(0, vendedor, 5, 250).unwrap();

        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(comprador);

        let res = contrato.crear_orden(0, 0);
        assert!(matches!(
            res,
            Err(ErroresContrato::CantidadEnCarritoMenorAUno)
        ));
    }

    #[ink::test]
    fn test_crear_orden_sin_rol_comprador_fallido() {
        let mut contrato = setup_sistema();
        let (no_comprador, vendedor) = build_testing_accounts();

        contrato
            ._registrar_usuario(
                vendedor,
                "Juan Vendedor".to_string(),
                "JT11@Gmail.com".to_string(),
            )
            .unwrap();
        contrato._asignar_rol(vendedor, VENDEDOR).unwrap();

        contrato
            ._registrar_usuario(
                no_comprador,
                "Santi No comprador".to_string(),
                "ST96@Gmail.com".to_string(),
            )
            .unwrap();

        contrato
            ._registrar_categoria("Una cat".to_string())
            .unwrap();
        contrato
            ._crear_producto(
                vendedor,
                "Nombre CAt".to_string(),
                "Aux".to_string(),
                "Una cat".to_string(),
                3,
            )
            .unwrap();
        contrato._crear_publicacion(0, vendedor, 3, 600).unwrap();

        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(no_comprador);

        let res = contrato.crear_orden(0, 1);
        assert!(matches!(res, Err(ErroresContrato::RolNoApropiado)));
    }

    #[ink::test]
    fn test_enviar_orden_pendiente_exito() {
        let mut contrato = setup_sistema();
        let (comprador, vendedor) = build_testing_accounts();

        // registrar usuarios y roles
        contrato
            ._registrar_usuario(
                vendedor,
                "Juan Vendedor".to_string(),
                "vendedor@mail.com".to_string(),
            )
            .unwrap();
        contrato._asignar_rol(vendedor, VENDEDOR).unwrap();
        contrato
            ._registrar_usuario(
                comprador,
                "Santi Comprador".to_string(),
                "comprador@mail.com".to_string(),
            )
            .unwrap();
        contrato._asignar_rol(comprador, COMPRADOR).unwrap();

        // Creo categoría, producto y publicación
        contrato._registrar_categoria("Juegos".to_string()).unwrap();
        contrato
            ._crear_producto(
                vendedor,
                "PS5".to_string(),
                "Sony".to_string(),
                "Juegos".to_string(),
                3,
            )
            .unwrap();
        contrato._crear_publicacion(0, vendedor, 3, 600).unwrap();

        // Creo orden como comprador
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(comprador);
        contrato.crear_orden(0, 2).unwrap();

        // Envio orden como vendedor
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(vendedor);
        let res = contrato.enviar_producto(0);

        assert!(res.is_ok(), "Fallo al enviar la orden");
        let ordenes = contrato.listar_ordenes();
        assert_eq!(ordenes[0].get_status(), EstadoOrden::Enviada);
    }

    #[ink::test]
    fn test_contrato_orden_pendiente_exitoso() {
        let mut contrato = setup_sistema();
        let (comprador, vendedor) = build_testing_accounts();

        // Setea la cuenta del contrato como callee en el entorno de test
        let contrato_account = ink::env::test::callee::<ink::env::DefaultEnvironment>();
        ink::env::test::set_callee::<ink::env::DefaultEnvironment>(contrato_account);

        contrato
            ._registrar_usuario(
                vendedor,
                "Santiago".to_string(),
                "santi@mail.com".to_string(),
            )
            .unwrap();
        contrato._asignar_rol(vendedor, VENDEDOR).unwrap();

        contrato
            ._registrar_usuario(comprador, "Juan".to_string(), "juan@mail.com".to_string())
            .unwrap();
        contrato._asignar_rol(comprador, COMPRADOR).unwrap();

        contrato._registrar_categoria("Libros".to_string()).unwrap();
        contrato
            ._crear_producto(
                vendedor,
                "Rust".to_string(),
                "Desc".to_string(),
                "Libros".to_string(),
                10,
            )
            .unwrap();
        contrato._crear_publicacion(0, vendedor, 10, 100).unwrap();

        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(comprador);

        assert!(contrato.crear_orden(0, 1).is_ok());
    }

    #[ink::test]
    fn enviar_orden_inexistente_falla() {
        let mut contrato = setup_sistema();
        let vendedor = id_vendedor();

        contrato
            ._registrar_usuario(vendedor, "Ven".into(), "v@mail.com".to_string())
            .unwrap();
        contrato._asignar_rol(vendedor, VENDEDOR).unwrap();

        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(vendedor);

        // No hay orden 99
        let res = contrato.enviar_producto(99);
        assert!(matches!(res, Err(ErroresContrato::OrdenInexistente)));
    }

    #[ink::test]
    fn recibir_orden_enviada_exitoso() {
        let mut contrato = setup_sistema();
        let (comprador, vendedor) = build_testing_accounts();

        // Registro usuarios y roles
        contrato._registrar_usuario(vendedor, "Santiago".to_string(), "ST96@mail.com".to_string()).unwrap();
        contrato._asignar_rol(vendedor, Rol::Vendedor).unwrap();
        contrato._registrar_usuario(comprador, "Juan".to_string(), "JT11@mail.com".to_string()).unwrap();
        contrato._asignar_rol(comprador, Rol::Comprador).unwrap();

        //Creo publicación
        contrato._registrar_categoria("Libros".to_string()).unwrap();
        contrato._crear_producto(vendedor, "Rust".to_string(), "Desc".to_string(), "Libros".to_string(), 5).unwrap();
        contrato._crear_publicacion(0, vendedor, 5, 100).unwrap();


        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(comprador);
        contrato.crear_orden(0, 1).unwrap();

 
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(vendedor);
        contrato.enviar_producto(0).unwrap();


        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(comprador);
        let res = contrato.recibir_producto(0);

        assert!(res.is_ok());
        let orden = contrato.listar_ordenes()[0].clone();
        assert_eq!(orden.get_status(), EstadoOrden::Recibida);
    }
}
