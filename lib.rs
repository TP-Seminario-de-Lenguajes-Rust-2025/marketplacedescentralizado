#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod contract {
    use ink::{
        prelude::{string::String, vec::Vec},
        storage::{Mapping, StorageVec},
    };

    const COMPRADOR: u32 = 0;
    const VENDEDOR: u32 = 1;

    // #[derive(Clone,Copy)]
    // #[ink::scale_derive(Encode, Decode, TypeInfo)]
    // #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    // pub struct Decimal{
    //     entero: u32,
    //     decimal: u32
    // }

    // impl Decimal{
    //     fn mult(&self, multiplicador: u32) -> Decimal{
    //         let mut entero: u32 = self.entero.checked_mul(multiplicador).expect("hubo overflow xd");
    //         let mut decimal: u32 = self.decimal.checked_mul(multiplicador).expect("hubo overflow xd");
    //         if decimal.length()> self.decimal.length(){
    //             entero = entero.checked_add(decimal.div_euclid(self.decimal.length().checked_mul(10).expect("hubo overflow xd"))).expect("hubo overflow xd");
    //             decimal = decimal.checked_rem(self.decimal.length().checked_mul(10).expect("hubo overflow xd")).expect("hubo overflow xd");
    //         }
    //         Decimal{entero, decimal}
    //     }
    // }
    
    // trait Lengthable{
    //     fn length(&self) -> u32;
    // }

    // impl Lengthable for u32{
    //     fn length(&self) -> u32{
    //         let mut n = *self;
    //         let mut c: u32 = 0;
    //         while n!=0_u32{
    //             n/=10_u32;
    //             c=c.checked_add(1).expect("como carajo hubo overflow aca xd"); //revisar
    //         }
    //         c
    //     }
    // }

    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum ErroresApp {
        ErrorComun,
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
            Sistema {users: Vec::new(), roles: Mapping::new(), categorias: Mapping::new(), ordenes_historico:StorageVec::new(), productos:StorageVec::new(), publicaciones: Vec::new()}
        }

        fn get_user(&mut self, id:AccountId) -> Result<Usuario,ErroresApp>{
            todo!("verifica que existe el usuario")
        }

        #[ink(message)]
        pub fn registrar_usuario(
            &mut self,
            nombre: String,
            mail: String,
            roles:
        )
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


        //Modificar crear_publicacion para que reciba el id de un producto, la cantidad, y el precio por unidad a vender
        #[ink(message)]
        pub fn crear_publicacion(
            &mut self,
            id_producto: u32,
            stock: u32,
            precio: Balance,
        ) -> Result<(),ErroresApp>{
            let id_usuario = self.env().caller();
            return self._crear_publicacion(id_producto, id_usuario, stock, precio)
        }
        fn _crear_publicacion(
            &mut self,
            id_producto: u32,
            id_usuario: AccountId,
            stock: u32,
            precio: Balance,            
        ) -> Result<(),ErroresApp> {
            let id = self.publicaciones.len().checked_add(1).ok_or(ErroresApp::ErrorComun)? as u32;
            let usuario = self.get_user(id_usuario)?;
            if usuario.has_role(VENDEDOR){
                if let Some(index) = id_producto.checked_sub(1){
                    self.descontar_stock_producto(id_producto, stock)?;
                    let p = Publicacion::new(id, id_producto, id_usuario, stock, precio);
                    self.publicaciones.push(p);
                    Ok(())
                }else{todo!("error: indice invalido (<0)")}
            } else {todo!("error: usuario no tiene el rol apropiado")}
        }

        fn descontar_stock_producto(&mut self, id:u32, cantidad:u32) -> Result<(), ErroresApp>{
            let index = id.checked_sub(1).ok_or(ErroresApp::ErrorComun)?;
            let producto = self.productos.get(index).ok_or(ErroresApp::ErrorComun)?;
            producto.stock.checked_sub(cantidad).ok_or(ErroresApp::ErrorComun)?;
            self.productos.set(index, &producto);
            Ok(())
        }

        #[ink(message)]
        pub fn crear_producto(
            &mut self,
            id: u32,
            nombre: String,
            descripcion: String,
            categoria: u32,
            stock: u32
        ) -> Result<(),ErroresApp> {
            let id_vendedor = self.env().caller();
            return self._crear_producto(id, id_vendedor, nombre, descripcion, categoria, stock)
        }

        fn _crear_producto(
            &mut self,
            id: u32,
            id_vendedor: AccountId,
            nombre: String,
            descripcion: String,
            categoria: u32,
            stock: u32
        ) -> Result<(),ErroresApp> {
            let usuario = self.get_user(id_vendedor)?;
            if usuario.has_role(VENDEDOR){
                if self.categorias.try_get(categoria).is_some() {  
                    let producto = Producto::new(id, id_vendedor, nombre, descripcion, categoria, stock);
                    if !self.producto_existe(&producto){
                        self.productos.push(&producto);
                        Ok(())
                    }else{todo!("error: el producto ya existe")}
                }else{todo!("error: no se encuentra la categoria")}
            }else{todo!("error: el usuario no es un vendedor")}
        }

        fn producto_existe(&self, p:&Producto) -> bool{
            for i in 0..self.productos.len(){
                if let Some(prod) = self.productos.get(i){
                    if prod.eq(p){
                        return true
                    }
                }
            }
            false
        }


        
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
            let id_orden = self.ordenes_historico.len().checked_add(1).ok_or(ErroresApp::ErrorComun)?;
            let comprador = self.get_user(id_comprador)?;
            let id_vendedor = self.get_id_vendedor(id_pub)?;
            let vendedor = self.get_user(id_vendedor)?;
            //let stock = self.get_stock_publicacion(id_pub)?;
            let precio_producto = self.get_precio_unitario(id_pub)?;
            let precio_total = precio_producto.mult(cantidad);
            if comprador.has_role(COMPRADOR) && vendedor.has_role(VENDEDOR){
                if cantidad !=0{
                    self.descontar_stock_publicacion(id_pub, cantidad)?;
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

        fn descontar_stock_publicacion(&mut self, id_pub:u32, cantidad:u32) -> Result<(),ErroresApp>{
            let index = id_pub.checked_sub(1).ok_or(ErroresApp::ErrorComun)?;
            if let Some(publicacion) = self.publicaciones.get_mut(index as usize){
                publicacion.stock.checked_sub(cantidad).ok_or(ErroresApp::ErrorComun)?;
                Ok(())
            }else{todo!("error: no habia publicacion en el indice")}
        }

        /// Recibe un ID de una publicacion y devuelve su stock
        fn get_precio_unitario(&self, id_pub:u32) -> Result<Balance,ErroresApp>{
            if let Some(publicacion) = self.publicaciones.iter().find(|p|p.id == id_pub){
                Ok(publicacion.precio_unitario)
            }else{
                todo!("error: no encontrar la publicacion con el id provisto")
            }
        }

    }

    /// Estructuras relacionadas a Usuario
    
    /// Roles existentes
    #[derive(Encode, Decode, TypeInfo, Debug)]
    #[cfg_attr(feature = "std", derive(StorageLayout))]
    pub enum Rol {
        Comprador,
        Vendedor,
    }

    /// Estructura que define al Usuario
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
        ///Crea un nuevo Usuario
        pub fn new(id: AccountId, nombre: String, mail: String, roles: Vec<Rol>) -> Usuario {
            Usuario {
                id,
                nombre,
                mail,
                rating: Rating::new(),
                roles,
            }
        }

        ///Devuelve true o false si el usuario contiene el rol pasado por parametro
        pub fn has_role(&self, rol: Rol) -> bool {
            self.roles.contains(&rol)
        }

        ///devuelve el Rating como comprador (promedio entre cantidad de ordenes y calificaciones)
        pub fn get_rating_comprador() -> f64 {
            todo!()
        }

        ///devuelve el Rating como vendedor (promedio entre cantidad de ordenes y calificaciones)
        pub fn get_rating_vendedor() -> f64 {
            todo!()
        }

    }

    /// Estructura correspondiente al raiting de un usuario
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
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

        ///devuelve el Rating como comprador (promedio entre cantidad de ordenes y calificaciones)
        fn get_rating_comprador() -> f64 {
            todo!("debe devolver Result<f64,ErrDivisionZero>")
        }

        ///devuelve el Rating como vendedor (promedio entre cantidad de ordenes y calificaciones)
        fn get_rating_vendedor() -> f64 {
            todo!("debe devolver Result<f64,ErrDivisionZero>")
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
        stock: u32
    }

    impl Producto {
        ///Crea un producto nuevo dado los parametros
        pub fn new(id: u32, id_vendedor: AccountId, nombre: String, descripcion: String, categoria: u32, stock: u32) -> Producto {
            //TODO: verificar que stock>0 y precio>0 y nombre y desc sean validos
            Producto {
                id,
                id_vendedor,
                nombre,
                descripcion,
                categoria,
                stock
            }
        }

        ///Compara un producto self con un producto pasado por parametro
        pub fn eq(&self, p:&Producto) -> bool{
            if 
                self.nombre == p.nombre &&
                self.categoria == p.categoria
            {
                return true 
            }
            false
        }
    }

    ///LOGICA DE PUBLICACION

    ///Estructura de publicacion
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub struct Publicacion {
        id: u32,
        id_prod: u32, //id del producto que contiene
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

        // pub fn actualizar_stock(&mut self, delta: i32) -> Result<(), ErroresApp> {}

        //nueva implementacion del new de publicacion sin usar uuid
        pub fn new(id: u32, id_producto: u32, id_user: AccountId, stock: u32, precio_unitario:Balance) -> Publicacion {
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
        cantidad:u32,
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
            cantidad:u32,
            precio_total: Balance
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
