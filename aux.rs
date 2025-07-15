    #[derive(Clone,Copy)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct Decimal{
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