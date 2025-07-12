#![cfg_attr(not(feature = "std"), no_std, no_main)]


#[ink::contract]
mod contract {
    
    use ink::{prelude::{string::String,vec::Vec},storage::{Mapping, StorageVec}};
    //use crate::estructuras::{publicacion::*,producto::*,orden::*,usuario::*};
    use uuid::Uuid; //borrar no usamos mas ahora usamos len()+1
    use std::fmt;

    const COMPRADOR : &str = "1";
    const VENDEDOR : &str = "2";

    ///LOGICA DE ORDEN
    /// 
    

    #[cfg_attr(
        feature = "std",
        derive(ink::storage::traits::StorageLayout)
    )]
    #[derive(Debug)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]

    #[derive(Clone, Copy, PartialEq)]
    #[allow(dead_code)]
    pub enum ErroresApp {
        ErrorComun
    }

    impl fmt::Display for ErroresApp {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                ErroresApp::ErrorComun => write!(f, "PARA LOCO"),
            }
        }
    }


    #[cfg_attr(
        feature = "std",
        derive(ink::storage::traits::StorageLayout)
    )]
    #[derive(Debug)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]

    #[derive(Clone, Copy, PartialEq)]
    #[allow(dead_code)]
    pub enum EstadoOrden {
        Pendiente,
        Enviada, //solo lo puede modificar el vendedor
        Recibida, //solo lo puede modificar el comprador
        Cancelada, //tienen que estar ambos de acuerdo y tiene que estar en estado pendiente
    }

    #[cfg_attr(
        feature = "std",
        derive(ink::storage::traits::StorageLayout)
    )]
    #[derive(Debug)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub struct Orden {
        //info de la orden
        id: String,
        id_vendedor: String,
        id_comprador: String,
        status: EstadoOrden,
        productos: Vec<String>, //vec con uuid de los productos
        cal_vendedor: Option<u8>,  //calificacion que recibe el vendedor
        cal_comprador: Option<u8>, //calificacion que recibe el comprador
    }

    impl Default for Orden{
        fn default() -> Self {
            Orden{
                id: "".to_string(),
                id_vendedor: "".to_string(),
                id_comprador: "".to_string(),
                status: EstadoOrden::Pendiente,
                productos: Vec::new(),
                cal_vendedor: None,
                cal_comprador: None, 
            }
        }
    }

    impl Orden {
        pub fn new(id_vendedor:String, id_comprador:String, productos:Vec<String>) -> Orden {
            //verificar que productos no sea vacio
            let id = Uuid::new_v4().to_string();
            Orden{id, id_vendedor, id_comprador,productos, ..Default::default()}
        }
        //nuevo new de orden sin usar uuid pasamos id desde el sistema
    /*  pub fn new(id: String, id_vendedor:String, id_comprador:String, productos:Vec<String>) -> Orden {
            Orden { id, id_vendedor, id_comprador, productos, ..Default::default() }
            }
    */

        //pub fn cambiar_estado
        //fn set_enviada() //solamente puede ser modificada por el vendedor
        //fn set_recibida() //solamente puede ser modificada por el comprador
        //fn cancelar() //necesitan estar de acuerdo ambos
    }

    /// LOGICA DE PRODUCTO
    /// 

    #[cfg_attr(
        feature = "std",
        derive(ink::storage::traits::StorageLayout)
    )]
    #[derive(Debug)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub struct Categoria {
        id: String,
        nombre: String,
    }

    impl Categoria {
        pub fn crear_categoria() -> Categoria {
            todo!("debe devolver un Result<Categoria>")
        }
    }

    #[cfg_attr(
        feature = "std",
        derive(ink::storage::traits::StorageLayout)
    )]
    #[derive(Debug)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub struct Producto {
        id: String,
        nombre: String,
        desc: String,
        precio: String,
        categ: String,
    }

    impl Producto {
        pub fn new(nombre: String,descripcion:String, precio:String,categoria: String) -> Producto {
            //TODO: verificar que stock>0 y precio>0 y nombre y desc sean validos
            let id = Uuid::new_v4().to_string();
            Producto{id, nombre, desc: descripcion, precio, categ:categoria}
        }

    /* pub fn new(id: String, nombre: String, descripcion:String, precio:String, categoria: String) -> Producto {
            Producto { id, nombre, desc: descripcion, precio, categ: categoria }
            
        }*/
    }

    ///LOGICA DE PUBLICACION
    
    #[cfg_attr(
        feature = "std",
        derive(ink::storage::traits::StorageLayout)
    )]
    #[derive(Debug)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub struct Publicacion {
        id: String,
        id_prod: String, //id del producto que contiene
        id_user: String, //id del user que publica
        stock: u8,
        activa: bool,
    }

    impl Publicacion {
        pub fn toggle_activa(&mut self) {
            self.activa = !self.activa;
        }

        pub fn is_activa(&self) -> bool {
            self.activa
        }
        
        pub fn actualizar_stock(&mut self, delta: i32) -> Result<(), ErroresApp> {
        
        }


        pub fn new(id_producto:String,id_user: String, stock:u8) -> Publicacion{
            let id= Uuid::new_v4().to_string();
            Publicacion{id, id_prod:id_producto, id_user,stock,activa:true}
        }
        //nueva implementacion del new de publicacion sin usar uuid
      /*pub fn new(id: String, id_producto: String, id_user: String, stock: u8) -> Publicacion {
            Publicacion { id, id_prod: id_producto, id_user, stock, activa: true }
        } */
    }

    /// LOGICA DE DE USUARIO
    /// 
    
    #[cfg_attr(
        feature = "std",
        derive(ink::storage::traits::StorageLayout)
    )]
    #[derive(Debug)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub struct Rol {
        id: String, //not a Uuid
        desc: String,
    }

    impl Rol {
        pub fn crear_rol() {
            todo!()
        }
    }


    #[cfg_attr(
        feature = "std",
        derive(ink::storage::traits::StorageLayout)
    )]
    #[derive(Debug)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    struct Rating {
        calificacion_comprador: (u16, u32), //cant de compras, valor cumulativo de todas las calificaciones
        calificacion_vendedor: (u16, u32),
    }

    impl Rating {
        fn new() -> Rating{
            Rating { calificacion_comprador: (0,0), calificacion_vendedor: (0,0) }
        }
        fn get_rating_comprador() -> f64 {
            todo!("debe devolver Result<f64,ErrDivisionZero>")
        }

        fn get_rating_vendedor() -> f64 {
            todo!("debe devolver Result<f64,ErrDivisionZero>")
        }
    }


    #[cfg_attr(
        feature = "std",
        derive(ink::storage::traits::StorageLayout)
    )]
    #[derive(Debug)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub struct Usuario {
        id: String,
        nombre: String,
        mail: String,
        rating: Rating,
        roles: Vec<String> //id de rol
    }

    impl Usuario {
        pub fn new(nombre:String,mail:String,roles:Vec<String>) -> Usuario {
            let id = Uuid::new_v4().to_string();
            Usuario { id , nombre, mail, rating: Rating::new(), roles}
        }

        //nuevo new de Usuario sin uuid le pasamos el id desde el sistema
    /*  pub fn new(id: String, nombre: String, mail: String, roles: Vec<String>) -> Usuario {
            Usuario { id, nombre, mail, rating: Rating::new(), roles }
} */
        pub fn has_role(&self, rol:&str) ->bool{
            self.roles.contains(&rol.to_string())
        }

        pub fn get_rating_comprador() -> f64 {
            todo!()
        }
        pub fn get_rating_vendedor() -> f64 {
            todo!()
        }

        pub fn agregar_rol(){
            
        }
    }
   
    #[ink(storage)]
    pub struct Sistema {
        users: Vec<Usuario>,

        // //asociacion entre usuario y rol
        roles: Mapping<String, Rol>, //roles que existen
        
        categorias: Mapping<String, Categoria>, //id_categ

        ordenes_historico: StorageVec<Orden>, //registro de compras

        productos: StorageVec<Producto>,

        // //guarda las publicaciones
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
        pub fn crear_publicacion(&mut self, nombre: String, descripcion:String, precio:String,stock: u8,categoria:String, id_user:String){
            if let Some(user) = self.users.iter().find(|u|u.id==id_user){
                if user.roles.contains(&VENDEDOR.to_string()){
                    if let Ok(prod) = self.crear_producto(nombre, descripcion, precio, categoria){

                        //nueva implementacion para no usar uuid
                    /*  let id = (self.publicaciones.len() + 1).to_string(); 
                        let p = Publicacion::new(id, prod.id, id_user, stock);*/

                        let mut p  = Publicacion::new(prod.id,id_user,stock);
                        self.publicaciones.push(p);
                    };
                }else{

                }
            }else{
                todo!("error")
            }
        }

        
        fn crear_producto(&mut self, nombre:String,descripcion:String,precio:String, categoria:String) -> Result<Producto,ErroresApp>{
            if self.categorias.try_get(categoria.clone()).is_some(){//verifica la categoria
                //nueva implementacion para no usar uuid
            /*  let id = (self.productos.len() + 1).to_string();
                let p = Producto::new(id, nombre, descripcion, precio, categoria); */

                let p = Producto::new(nombre, descripcion,precio,categoria);
                self.productos.push(&p);//??
                Ok(p)
            }else{
                todo!("aca a")
            }
        }

        #[ink(message)]
        pub fn crear_orden(&mut self, id_vendedor:String, id_comprador:String, productos:Vec<String>){
            if let Some(comprador) = self.users.iter().find(|u|u.id==id_comprador){ //verifica que existe el usuario
                if let Some(vendedor) = self.users.iter().find(|u|u.id==id_vendedor){
                    if comprador.has_role(COMPRADOR) && vendedor.has_role(VENDEDOR){ //verifica que tienen los roles necesarios

                        //nueva implementacion para no usar uuid
                    /*  let id = (self.ordenes_historico.len() + 1).to_string();
                        let o = Orden::new(id, id_vendedor, id_comprador, productos.clone());
                         */


                        let o = Orden::new(id_vendedor,id_comprador,productos.clone());
                        for prod in productos.iter(){
                            if let Some(publi) = self.publicaciones.iter_mut().find(|p|*p.id_prod==*prod){
                                if publi.stock>0{
                                    //publi.stock-=1; //error: arithmetic operation that can potentially result in unexpected side-effects
                                }
                            }
                        }
                        self.ordenes_historico.push(&o);//??
                    }
                }
            }
        }
    }
}
