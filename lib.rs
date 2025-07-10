#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod contract {
    use ink::{prelude::{string::String,vec::Vec},storage::{traits::StorageLayout, Mapping, StorageVec}};
    use contract_logic::estructuras::{publicacion::*,producto::*,orden::*,usuario::*};
    use std::error::Error;

    const COMPRADOR : &str = "1";
    const VENDEDOR : &str = "2";
   
    #[cfg_attr(
        feature = "std",
        derive(ink::storage::traits::StorageLayout)
    )]
    #[derive(Debug)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)] //porque no funciona el macro?
    pub struct Wrapper(Usuario);
    // impl StorageLayout for Wrapper{

    // }

    
    // struct Usuario;
    // impl usuario::Usuario for Usuario{}
    #[ink(storage)]
    pub struct Sistema {
        users: Vec<Usuario>,

        // //asociacion entre usuario y rol
        //roles: Mapping<String, Rol>, //roles que existen
        
        //categorias: Mapping<String, Categoria>, //id_categ

        // ordenes_historico: StorageVec<Orden>, //registro de compras

        // productos: StorageVec<Producto>,

        // //guarda las publicaciones
        // publicaciones: Vec<Publicacion>, //capaz no un vec
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
            if let Some(user) = self.usuarios.find(|u|u.id==id_user){
                let mut prod = crear_producto(nombre, descripcion, precio, categoria)?;
                let mut p  = Publicacion::new(prod.id,id_user,stock);
                self.publicaciones.push(p);
            }else{
                todo!("error")
            }
        }

        #[ink(message)]
        pub fn crear_producto(&mut self, nombre:String,descripcion:String,precio:String, categoria:String) -> Result<Producto,Error>{
            if self.categorias.try_get(categoria).is_some(){//verifica la categoria
                let p = Producto::new(nombre, descripcion,precio,categoria);
                self.productos.push(p);
            }
        }

        #[ink(message)]
        pub fn crear_orden(&mut self, id_vendedor:String, id_comprador:String, productos:Vec<String>){
            //verifica ids y pro
            if let Some(comprador) = self.usuarios.iter().find(|u|u.id==id_comprador){ //verifica que existe el usuario
                if let Some(vendedor) = self.usuarios.iter().find(|u|u.id==id_vendedor){
                    if comprador.has_role(COMPRADOR) && vendedor.has_role(VENDEDOR){ //verifica que tienen los roles necesarios
                        let o = Orden::new(id_vendedor,id_comprador,productos);
                        for prod in productos.iter(){
                            if let Some(publi) = self.publicaciones.iter().find(|p|p.id_prod==prod){
                                publi.stock-=1;
                            }
                        }
                        self.ordenes_historico.push(o);
                    }
                }
            }
        }
    }
}
