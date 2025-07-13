#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod contract {
    use ink::{
        prelude::{string::String, vec::Vec},
        storage::{Mapping, StorageVec},
    };

    const COMPRADOR: u32 = 1;
    const VENDEDOR: u32 = 2;

    ///LOGICA DE ORDEN
    ///

    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum ErroresApp {
        ErrorComun,
    }

    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum EstadoOrden {
        Pendiente,
        Enviada,   //solo lo puede modificar el vendedor
        Recibida,  //solo lo puede modificar el comprador
        Cancelada, //tienen que estar ambos de acuerdo y tiene que estar en estado pendiente
    }

    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub struct Orden {
        //info de la orden
        id: u32,
        id_vendedor: u32,
        id_comprador: AccounId,
        status: EstadoOrden,
        productos: Vec<u32>,       //vec con uuid de los productos
        cal_vendedor: Option<u8>,  //calificacion que recibe el vendedor
        cal_comprador: Option<u8>, //calificacion que recibe el comprador
    }

    impl Default for Orden {
        fn default() -> Self {
            Orden {
                id: 0,
                id_vendedor: 0,
                id_comprador: 0,
                status: EstadoOrden::Pendiente,
                productos: Vec::new(),
                cal_vendedor: None,
                cal_comprador: None,
            }
        }
    }

    impl Orden {
        //nuevo new de orden sin usar uuid pasamos id desde el sistema
        pub fn new(id: u32, id_vendedor: u32, id_comprador: AccountId, productos: Vec<u32>) -> Orden {
            Orden {
                id,
                id_vendedor,
                id_comprador,
                productos,
                ..Default::default()
            }
        }

        //pub fn cambiar_estado
        //fn set_enviada() //solamente puede ser modificada por el vendedor
        //fn set_recibida() //solamente puede ser modificada por el comprador
        //fn cancelar() //necesitan estar de acuerdo ambos
    }

    /// LOGICA DE PRODUCTO
    ///

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

    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub struct Producto {
        id: u32,
        nombre: String,
        descripcion: String,
        categoria: u32,
    }

    impl Producto {
        pub fn new(id: u32, nombre: String, descripcion: String, categoria: u32) -> Producto {
            //TODO: verificar que stock>0 y precio>0 y nombre y desc sean validos
            Producto {
                id,
                nombre,
                descripcion,
                categoria,
            }
        }
    }

    ///LOGICA DE PUBLICACION

    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub struct Publicacion {
        id: u32,
        id_prod: u32, //id del producto que contiene
        id_user: u32, //id del user que publica
        stock: u32,
        activa: bool,
    }

    impl Publicacion {
        pub fn toggle_activa(&mut self) {
            self.activa = !self.activa;
        }

        pub fn is_activa(&self) -> bool {
            self.activa
        }

        // pub fn actualizar_stock(&mut self, delta: i32) -> Result<(), ErroresApp> {}

        //nueva implementacion del new de publicacion sin usar uuid
        pub fn new(id: u32, id_producto: u32, id_user: u32, stock: u32) -> Publicacion {
            Publicacion {
                id,
                id_prod: id_producto,
                id_user,
                stock,
                activa: true,
            }
        }
    }

    /// LOGICA DE DE USUARIO
    ///

    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub struct Rol {
        id: u32, //not a Uuid
        desc: String,
    }

    impl Rol {
        pub fn crear_rol() {
            todo!()
        }
    }

    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    struct Rating {
        calificacion_comprador: (u32, u32), //cant de compras, valor cumulativo de todas las calificaciones
        calificacion_vendedor: (u32, u32),
    }

    impl Rating {
        fn new() -> Rating {
            Rating {
                calificacion_comprador: (0, 0),
                calificacion_vendedor: (0, 0),
            }
        }
        fn get_rating_comprador() -> f64 {
            todo!("debe devolver Result<f64,ErrDivisionZero>")
        }

        fn get_rating_vendedor() -> f64 {
            todo!("debe devolver Result<f64,ErrDivisionZero>")
        }
    }

    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub struct Usuario {
        id: u32,
        nombre: String,
        mail: String,
        rating: Rating,
        roles: Vec<u32>, //id de rol
    }

    impl Usuario {
        pub fn new(id: u32, nombre: String, mail: String, roles: Vec<u32>) -> Usuario {
            Usuario {
                id,
                nombre,
                mail,
                rating: Rating::new(),
                roles,
            }
        }

        pub fn has_role(&self, rol: u32) -> bool {
            self.roles.contains(&rol)
        }

        pub fn get_rating_comprador() -> f64 {
            todo!()
        }
        pub fn get_rating_vendedor() -> f64 {
            todo!()
        }

        pub fn agregar_rol() {}
    }

    #[ink(storage)]
    pub struct Sistema {
        users: Vec<Usuario>,
        roles: Mapping<u32, Rol>,             //roles que existen
        categorias: Mapping<u32, Categoria>,  //id_categ
        ordenes_historico: StorageVec<Orden>, //registro de compras
        productos: StorageVec<Producto>,
        publicaciones: Vec<Publicacion>, //capaz no un vec
    }

    impl Sistema {
        #[ink(constructor)]
        pub fn new() -> Self {
            todo!()
            //Sistema {}
        }

        /// role related
        pub fn asociar_rol() {
            todo!()
        }

        pub fn has_role() {
            todo!("verifica que el usuario tiene el rol")
        }

        #[ink(message)]
        pub fn registrar_usuario(
            &mut self,
            nombre: String,
            mail: String,
            roles: Vec<u32>,
        ) -> Result<u32, ErroresApp> {
            //verifico que el email no este asosiado a otra cuenta (que no esta repetido)
            if self.users.iter().any(|u| u.mail == mail) {
                return Err(ErroresApp::ErrorComun);
            }
            if let Some(id) = self.users.len().checked_add(1) {
                let nuevo_usuario = Usuario::new(id as u32, nombre, mail, roles);
                self.users.push(nuevo_usuario);
                Ok(id as u32)
            } else {
                Err(ErroresApp::ErrorComun) // maquetar error
            }
        }

        #[ink(message)]
        pub fn crear_publicacion(
            &mut self,
            id: u32,
            nombre: String,
            descripcion: String,
            precio: String,
            stock: u32,
            categoria: u32,
            id_user: u32,
        ) {
            if let Some(user) = self.users.iter().find(|u| u.id == id_user) {
                if user.roles.contains(&VENDEDOR) {
                    // if let Ok(prod) = self.crear_producto(id, nombre, descripcion, categoria) {
                    //     let p = Publicacion::new(id, prod.id, id_user, stock);
                    //     self.publicaciones.push(p);
                    // }; Se tiene que crear aparte.
                }
            } else {
                todo!("error");
            }
        }

        #[ink(message)]
        pub fn crear_producto(
            &self,
            id: u32,
            nombre: String,
            descripcion: String,
            categoria: u32,
        ) -> Producto {
            if self.categorias.try_get(categoria).is_some() {
                Producto::new(id, nombre, descripcion, categoria);
                // let producto = Producto::new(id, nombre, descripcion, categoria);
                // return Ok(producto);
            }
            todo!("-> Result<Producto, ErroresApp>");
        }




        //REVISAR los usuarios se tienen que manejar con AccountID
        #[ink(message)]
        pub fn realizar_orden(
            &mut self,
            id_vendedor: u32,
            //id_comprador: u32,
            productos: Vec<u32>
        )-> Result<(),ErroresApp>{//deberia retornar Result?
            let id_comprador = self.env().caller();
            return self.crear_orden(id_vendedor, id_comprador, productos);
        }

        fn crear_orden(
            &mut self,
            id_vendedor: u32,
            id_comprador: AccountId,
            productos: Vec<u32>,
        ) -> Result<(), ErroresApp>{
            let id = self.ordenes_historico.len().checked_add(1).unwrap_or(0);
            let comprador = self.user_exists(id_comprador)?;
            let vendedor = self.user_exists(id_vendedor)?;
            if comprador.has_role(COMPRADOR) && vendedor.has_role(VENDEDOR) {
                if !productos.is_empty(){    
                    for prod in productos.iter() { 
                        if let Some(publi) =
                            self.publicaciones.iter().find(|p| p.id_prod == *prod)
                        {
                            if publi.stock > 0 { //revisar 
                                todo!("Crear una funcion de Sistema para reducir el stock");
                            }
                        }else{todo!("retornar error: el producto no corresponde a una publicacion")}
                    }
                    let orden = Orden::new(id, id_vendedor, id_comprador, productos);
                    self.ordenes_historico.push(&orden);
                    Ok(())
                }else{todo!()}
            }else{ todo!()}
        }
    }
}
