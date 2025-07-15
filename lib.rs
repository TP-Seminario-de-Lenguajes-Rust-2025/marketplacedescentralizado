#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod contract {
    use ink::{
        prelude::{string::String, vec::Vec},
        storage::{Mapping, StorageVec},
    };

    const COMPRADOR: u32 = 0;
    const VENDEDOR: u32 = 1;

    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum ErroresApp {
        ErrorComun,
    }

    #[ink(storage)]
    pub struct Sistema {
        users: Mapping<AccountId, Usuario>,
        users_ids: Vec<AccountId>,
        ordenes_historico: StorageVec<Orden>,
        productos: StorageVec<Producto>,
        publicaciones: Vec<Publicacion>,
    }

    impl Sistema {
        /// Constructor de la Blockchain
        #[ink(constructor)]
        pub fn new() -> Self {
            Sistema {
                users: Mapping::new(),
                users_id: Vec::new(),
                categorias: Mapping::new(),
                ordenes_historico: StorageVec::new(),
                productos: StorageVec::new(),
                publicaciones: Vec::new(),
            }
        }

        /// Recibe un nombre, email y un vector de roles. Genera un nuevo usuario en el sistema.
        #[ink(message)]
        pub fn registrar_usuario(&mut self, nombre: String, mail: String, roles: Vec<Rol>) {
            todo!()
        }

        /// Funcion pública: Recibe un ID, nombre, descripcion, categoria y stock. Genera una instancia de Producto.
        #[ink(message)]
        pub fn crear_producto(
            &mut self,
            id: u32,
            nombre: String,
            descripcion: String,
            categoria: u32,
            stock: u32,
        ) -> Result<(), ErroresApp> {
            let id_vendedor = self.env().caller();
            return self._crear_producto(id, id_vendedor, nombre, descripcion, categoria, stock)
        }

        // TODO: Modificar crear_publicacion para que reciba el id de un producto, la cantidad, y el precio por unidad a vender.

        /// Funcion pública: Recibe un ID de Producto, un stock y un precio. Instancia una nueva Publicacion
        #[ink(message)]
        pub fn crear_publicacion(
            &mut self,
            id_producto: u32,
            stock: u32,
            precio: Balance,
        ) -> Result<(), ErroresApp> {
            let id_usuario = self.env().caller();
            return self._crear_publicacion(id_producto, id_usuario, stock, precio);
        }

        /// Recibe un AccountId y retorna el usuario si existe.
        fn get_user(&mut self, id: AccountId) -> Result<Usuario, ErroresApp> {
            todo!("verifica que existe el usuario")
        }

        /// Genera una Orden a partir del ID de una publicacion y la cantidad de unidades. Funcion pública
        #[ink(message)]
        pub fn realizar_orden(
            &mut self,
            id_pub: u32,
            cantidad: u32,
            //precio_total: Decimal
        ) -> Result<(), ErroresApp> {
            let id_comprador = self.env().caller();
            return self.crear_orden(id_pub, id_comprador, cantidad);
        }

        fn registrar_usuario(
            &mut self,
            id: AccountId,
            nombre: String,
            mail: String,
            roles: Vec<u32>,
        ) -> Result<(), ErroresApp> {
            //verifico que el email no este asosiado a otra cuenta (que no esta repetido)
            if self.users.iter().any(|u| u.mail == mail) {
                return Err(ErroresApp::ErrorComun);
            }
            let nuevo_usuario = Usuario::new(id, nombre, mail, roles);
            self.users.push(nuevo_usuario);
            Ok(())
        }

        /// Recibe un ID de Producto, ID de usuario, un stock y un precio. Instancia una nueva Publicacion
        fn _crear_publicacion(
            &mut self,
            id_producto: u32,
            id_usuario: AccountId,
            stock: u32,
            precio: Balance,
        ) -> Result<(), ErroresApp> {
            let id = self
                .publicaciones
                .len()
                .checked_add(1)
                .ok_or(ErroresApp::ErrorComun)? as u32;
            let usuario = self.get_user(id_usuario)?;
            if usuario.has_role(VENDEDOR) {
                if let Some(index) = id_producto.checked_sub(1) {
                    self.descontar_stock_producto(id_producto, stock)?;
                    let p = Publicacion::new(id, id_producto, id_usuario, stock, precio);
                    self.publicaciones.push(p);
                    Ok(())
                } else {
                    todo!("error: indice invalido (<0)")
                }
            } else {
                todo!("error: usuario no tiene el rol apropiado")
            }
        }

        /// Recibe un ID de Producto y una cantidad y reduce su stock.
        fn descontar_stock_producto(&mut self, id: u32, cantidad: u32) -> Result<(), ErroresApp> {
            let index = id.checked_sub(1).ok_or(ErroresApp::ErrorComun)?;
            let producto = self.productos.get(index).ok_or(ErroresApp::ErrorComun)?;
            producto
                .stock
                .checked_sub(cantidad)
                .ok_or(ErroresApp::ErrorComun)?;
            self.productos.set(index, &producto);
            Ok(())
        }

        /// Recibe un ID, nombre, descripcion, categoria y stock. Genera una instancia de Producto.
        fn _crear_producto(
            &mut self,
            id: u32,
            id_vendedor: AccountId,
            nombre: String,
            descripcion: String,
            categoria: u32,
            stock: u32,
        ) -> Result<(), ErroresApp> {
            let usuario = self.get_user(id_vendedor)?;
            if usuario.has_role(VENDEDOR) {
                if self.categorias.try_get(categoria).is_some() {
                    let producto =
                        Producto::new(id, id_vendedor, nombre, descripcion, categoria, stock);
                    if !self.producto_existe(&producto) {
                        self.productos.push(&producto);
                        Ok(())
                    } else {
                        todo!("error: el producto ya existe")
                    }
                } else {
                    todo!("error: no se encuentra la categoria")
                }
            } else {
                todo!("error: el usuario no es un vendedor")
            }
        }

        /// Recibe la referencia a un Producto y retorna true si el Producto esta registrado en el sistema.
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

        /// Genera una Orden a partir del ID de una publicacion y la cantidad de unidades.
        fn crear_orden(
            &mut self,
            id_pub: u32,
            id_comprador: AccountId,
            cantidad: u32,
        ) -> Result<(), ErroresApp> {
            let id_orden = self
                .ordenes_historico
                .len()
                .checked_add(1)
                .ok_or(ErroresApp::ErrorComun)?;
            let comprador = self.get_user(id_comprador)?;
            let id_vendedor = self.get_id_vendedor(id_pub)?;
            let vendedor = self.get_user(id_vendedor)?;
            //let stock = self.get_stock_publicacion(id_pub)?;
            let precio_producto = self.get_precio_unitario(id_pub)?;
            let precio_total = precio_producto.mult(cantidad);
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
                    self.ordenes_historico.push(&orden);
                    Ok(())
                } else {
                    todo!("error: la cantidad es mayor a cero y hay stock suficiente")
                }
            } else {
                todo!("error: los usuarios no cumplen con los roles adecuados")
            }
        }

        /// Recibe un ID de una publicación y devuelve AccountId del vendedor asociado o un Error
        fn get_id_vendedor(&self, id_pub: u32) -> Result<AccountId, ErroresApp> {
            if let Some(publicacion) = self.publicaciones.iter().find(|p| p.id == id_pub) {
                Ok(publicacion.id_user)
            } else {
                todo!("'error de no encontrar la publicacion con el id provisto")
            }
        }

        //// Recibe un ID de publicacion y una cantidad; reduce el stock de la publicación. 
        fn descontar_stock_publicacion(
            &mut self,
            id_pub: u32,
            cantidad: u32,
        ) -> Result<(), ErroresApp> {
            let index = id_pub.checked_sub(1).ok_or(ErroresApp::ErrorComun)?;
            if let Some(publicacion) = self.publicaciones.get_mut(index as usize) {
                publicacion
                    .stock
                    .checked_sub(cantidad)
                    .ok_or(ErroresApp::ErrorComun)?;
                Ok(())
            } else {
                todo!("error: no habia publicacion en el indice")
            }
        }

        /// Recibe un ID de una publicación y devuelve su stock
        fn get_precio_unitario(&self, id_pub: u32) -> Result<Balance, ErroresApp> {
            if let Some(publicacion) = self.publicaciones.iter().find(|p| p.id == id_pub) {
                Ok(publicacion.precio_unitario)
            } else {
                todo!("error: no encontrar la publicacion con el id provisto")
            }
        }
    }

    // Estructuras relacionadas a Usuario

    #[derive(Encode, Decode, TypeInfo, Debug)]
    #[cfg_attr(feature = "std", derive(StorageLayout))]
    pub enum Rol {
        Comprador,
        Vendedor,
    }

    /// Estructura de Usuario
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub struct Usuario {
        id: AccountId,
        nombre: String,
        mail: String,
        rating: Rating,
        roles: Vec<Rol>,
    }

    impl Usuario {
        ///Crea una nueva instancia de Usuario.
        pub fn new(id: AccountId, nombre: String, mail: String, roles: Vec<Rol>) -> Usuario {
            Usuario {
                id,
                nombre,
                mail,
                rating: Rating::new(),
                roles,
            }
        }

        /// Devuelve true si el usuario tiene el Rol pasado por parámetro, de lo contrario false.
        pub fn has_role(&self, rol: Rol) -> bool {
            self.roles.contains(&rol)
        }

        /// Devuelve el promedio entre cantidad de ordenes y calificaciones como comprador.
        pub fn get_rating_comprador() -> f64 {
            todo!()
        }

        /// Devuelve el promedio entre cantidad de ordenes y calificaciones como vendedor.
        pub fn get_rating_vendedor() -> f64 {
            todo!()
        }
    }

    /// Estructura correspondiente al raiting de un usuario
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    struct Rating {
        //cant de compras, valor cumulativo de todas las calificaciones
        calificacion_comprador: (u32, u32),
        calificacion_vendedor: (u32, u32),
    }

    impl Rating {
        /// Crea una nueva instancia de Rating
        fn new() -> Rating {
            Rating {
                calificacion_comprador: (0, 0),
                calificacion_vendedor: (0, 0),
            }
        }

        /// Devuelve el promedio entre cantidad de ordenes y calificaciones como comprador.
        fn get_rating_comprador() -> f64 {
            todo!("debe devolver Result<f64,ErrDivisionZero>")
        }

        /// Devuelve el promedio entre cantidad de ordenes y calificaciones como vendedor.
        fn get_rating_vendedor() -> f64 {
            todo!("debe devolver Result<f64,ErrDivisionZero>")
        }
    }

    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub struct Categoria {
        id: u32,
        nombre: String,
    }

    impl Categoria {
        pub fn crear_categoria() -> Categoria {
            todo!("debe devolver un Result<Categoria>")
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
        /// Instancia un nuevo Producto
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

        /// Compara un producto self con un producto pasado por parametro
        pub fn eq(&self, p: &Producto) -> bool {
            if self.nombre == p.nombre && self.categoria == p.categoria {
                return true;
            }
            false
        }
    }

    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub struct Publicacion {
        id: u32,
        id_prod: u32,
        id_user: AccountId,
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

        // TODO: pub fn actualizar_stock(&mut self, delta: i32) -> Result<(), ErroresApp> {}

        // Instancia una nueva Publicacion
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

    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum EstadoOrden {
        Pendiente,
        Enviada,   // solo lo puede modificar el vendedor
        Recibida,  // solo lo puede modificar el comprador
        Cancelada, // tienen que estar ambos de acuerdo y tiene que estar en estado pendiente
    }

    /// Estructura de Orden.
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub struct Orden {
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
        /// Instancia una nueva Orden.
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
