pub mod contact_manager{
    pub fn add_contact() -> (String , String){
        use std::io;
        println!("Please enter the name of the contact : ");
        let mut name = String::new();
        let mut number = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read name");
        println!("Please enter the number : ");
        io::stdin().read_line(&mut number).expect("Failed to read number");
        let name = name.trim().to_string();
        let number = number.trim().to_string();
        (name , number)
    }
}