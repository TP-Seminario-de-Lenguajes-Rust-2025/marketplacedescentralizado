//todo es placeholder todavia

use ink::prelude::string::String;
use ink::prelude::vec::Vec;


#[ink::contract]
mod contract {
    
    struct Rol{     
        id:String,
        desc: String
    }

    
    enum EstadoOrden{
        Pendiente,
        Enviada,
        Recibida,
        Cancelada
    }

    struct UsuariosRoles{
        id_user: String,
        id_rol: String
    }

    struct Rating{
        cal_comprador: (u16, u32), //cant de compras, valor cumulativo de todas las calificaciones
        cal_vendedor: (u16, u32),
        // rating_comprador: f64, 
        // rating_vendedor: f64
    }

    struct Usuario{
        id: String,
        nombre: String,
        mail: String,
        rating: Rating
    }

    struct Categoria{
        id: String,
        nombre: String
    }

    struct Producto{
        id: String,
        nombre: String,
        desc: String,
        precio: f64,
        stock: u8,
        categ: Categoria,
    }

    struct Publicacion{
        id: String,
        id_prod: String, //id del producto que contiene
        id_user: String, //id del user que publica
        activa: bool
    }

    struct Orden{
        //info de la orden
        id: String,
        id_vendedor: String,
        id_comprador: String,
        status: EstadoOrden,
        productos: Vec<Producto>,
        cal_vendedor: u8, //calificacion que recibe el vendedor
        cal_comprador: u8 //calificacion que recibe el comprador
    }

    #[ink(storage)]
    pub struct Sistema{
        users: Vec<Usuario>,
        //asociacion entre usuario y rol
        roles: Vec<UsuariosRoles>,

        //registro de compras
        ordenes_historico: Vec<Orden>, 

        //guarda las publicaciones
        publicaciones: Vec<Publicacion> //capaz no un vec
    }

    impl Sistema{
        #[ink(constructor)]
        pub fn new() -> Sistema{
            Sistema{}
        }

        #[ink(message)]
        pub fn my_message(&self) {
            todo!()
        }
    
    }
}