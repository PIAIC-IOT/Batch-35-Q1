mod front_of_house {
    pub mod dine_in {
        pub fn reservation() {
            println!("Wait for your turn");
        }
    }   
    mod serve_order {
        fn prepare_dish() {}
        fn take_payment() {}
    } 
} 


fn main () {
    crate::front_of_house::dine_in::reservation();
    front_of_house::dine_in::reservation();

}










