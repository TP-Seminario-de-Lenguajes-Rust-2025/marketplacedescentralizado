#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod contract {
    use ink::{
        prelude::{string::String, vec::Vec},
        storage::{Mapping, StorageVec},
    };

    const COMPRADOR: u32 = 1;
    const VENDEDOR: u32 = 2;

    #[derive(Clone,Copy)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    struct Decimal{
        entero: u32,
        decimal: u32
    }

    impl Decimal{
        fn mult(&self, multiplicador: u32) -> Decimal{
            let mut entero: u32 = self.entero.checked_mul(multiplicador).expect("hubo overflow xd");
            let mut decimal: u32 = self.decimal.checked_mul(multiplicador).expect("hubo overflow xd");
            if decimal.length()> self.decimal.length(){
                entero = entero.checked_add(decimal.div_euclid(self.decimal.length().checked_mul(10).expect("hubo overflow xd"))).expect("hubo overflow xd");
                decimal = decimal.checked_rem(self.decimal.length().checked_mul(10).expect("hubo overflow xd")).expect("hubo overflow xd");
            }
            Decimal{entero, decimal}
        }
    }
    
    trait Lengthable{
        fn length(&self) -> u32;
    }

    impl Lengthable for u32{
        fn length(&self) -> u32{
            let mut n = *self;
            let mut c: u32 = 0;
            while n!=0_u32{
                n/=10_u32;
                c=c.checked_add(1).expect("como carajo hubo overflow aca xd"); //revisar
            }
            c
        }
    }

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
        id_publicacion: u32,
        id_vendedor: AccountId,
        id_comprador: AccountId,
        status: EstadoOrden,
        cantidad:u32,
        precio_total: Decimal,       
        cal_vendedor: Option<u8>,  //calificacion que recibe el vendedor
        cal_comprador: Option<u8>, //calificacion que recibe el comprador
    }

    impl Orden {
        //nuevo new de orden sin usar uuid pasamos id desde el sistema
        pub fn new(
            id: u32, 
            id_publicacion: u32,
            id_vendedor: AccountId, 
            id_comprador: AccountId, 
            cantidad:u32,
            precio_total: Decimal
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
        id_user: AccountId, //id del user que publica
        stock: u32,
        precio_unitario: Decimal,
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
        pub fn new(id: u32, id_producto: u32, id_user: AccountId, stock: u32, precio_unitario:Decimal) -> Publicacion {
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
        id: AccountId,
        nombre: String,
        mail: String,
        rating: Rating,
        roles: Vec<u32>, //id de rol
    }

    impl Usuario {
        pub fn new(id: AccountId, nombre: String, mail: String, roles: Vec<u32>) -> Usuario {
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

        fn get_user(&mut self, id:AccountId) -> Result<Usuario,ErroresApp>{
            todo!("verifica que existe el usuario")
        }

        #[ink(message)]
        pub fn registrar_usuario(
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


        //Modificar crear_publicacion para que reciba el id de un producto, la cantidad, y el precio por unidad a vender
        #[ink(message)]
        pub fn crear_publicacion(
            &mut self,
            id: u32,
            nombre: String,
            descripcion: String,
            precio: String,
            stock: u32,
            categoria: u32,
            id_user: AccountId,
        ) {
            if let Some(user) = self.users.iter().find(|u| u.id == id_user) {
                if user.roles.contains(&VENDEDOR) {
                    todo!("Crear una funcion de Sistema para reducir el stock");
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




        //se recibe id de publicacion, el comprador es el caller, la cantidad de productos y el monto que el comprador va a pagar en total el cual hay que validar.
        #[ink(message)]
        pub fn realizar_orden(
            &mut self,
            id_pub: u32,
            cantidad:u32,
            //precio_total: Decimal
        )-> Result<(),ErroresApp>{
            let id_comprador = self.env().caller();
            return self.crear_orden(id_pub, id_comprador, cantidad);
        }

        fn crear_orden(
            &mut self,
            id_pub: u32,
            id_comprador: AccountId,
            cantidad:u32,
            //precio_total: Decimal//esto deberia estar por parametro???
        ) -> Result<(), ErroresApp>{
            let id_orden = self.ordenes_historico.len().checked_add(1).unwrap_or(0);
            let comprador = self.get_user(id_comprador)?;
            let id_vendedor = self.get_id_vendedor(id_pub)?;
            let vendedor = self.get_user(id_vendedor)?;
            let stock = self.get_stock_publicacion(id_pub)?;
            let precio_producto = self.get_precio_unitario(id_pub)?;
            let precio_total = precio_producto.mult(cantidad);
            if comprador.has_role(COMPRADOR) && vendedor.has_role(VENDEDOR){
                if cantidad !=0 && stock>=cantidad{
                    let orden = Orden::new(id_orden, id_pub, id_vendedor, id_comprador, cantidad, precio_total);
                    self.ordenes_historico.push(&orden);
                    Ok(())                
                }else{todo!("error: la cantidad es mayor a cero y hay stock suficiente")}
            }else{ todo!("error: los usuarios no cumplen con los roles adecuados")}
        }


        /// Recibe un ID de una publicacion y devuelve AccountId del vendedor asociado o un Error
        fn get_id_vendedor(&self, id_pub:u32) -> Result<AccountId,ErroresApp>{
            if let Some(publicacion) = self.publicaciones.iter().find(|p|p.id == id_pub){
                Ok(publicacion.id_user)
            }else{
                todo!("'error de no encontrar la publicacion con el id provisto")
            }
        }

        /// Recibe un ID de una publicacion y devuelve su stock
        fn get_stock_publicacion(&self, id_pub:u32) -> Result<u32,ErroresApp>{
            if let Some(publicacion) = self.publicaciones.iter().find(|p|p.id == id_pub){
                Ok(publicacion.stock)
            }else{
                todo!("'error de no encontrar la publicacion con el id provisto")
            }
        }

        /// Recibe un ID de una publicacion y devuelve su stock
        fn get_precio_unitario(&self, id_pub:u32) -> Result<Decimal,ErroresApp>{
            if let Some(publicacion) = self.publicaciones.iter().find(|p|p.id == id_pub){
                Ok(publicacion.precio_unitario)
            }else{
                todo!("'error de no encontrar la publicacion con el id provisto")
            }
        }

        
    }
}
