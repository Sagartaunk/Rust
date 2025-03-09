use std::collections::HashMap;
use std::io;
use contact_manager::contact_manager;
fn main(){
    let mut contacts : HashMap<String , String> = HashMap::new();
    loop{
        println!("Please select one of the following options : ");
        println!("1. Add a contact");
        println!("2. Search a contact");
        println!("3. Exit");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice = choice.trim();
        match choice{
            "1" =>{
                let (name , number) = contact_manager::add_contact();
                contacts.insert(name ,number );
            },
            "2" =>{
                println!{"Please select one of the following options : "};
                println!("1. Search by name");
                println!("2. Search by number");
                let mut choice = String::new();                
                io::stdin().read_line(&mut choice).expect("Failed to read you choice");
                let choice = choice.trim();
                match choice{
                    "1" =>{
                        println!{"Please enter the name of the contact :"};
                        let mut name = String::new();
                        io::stdin().read_line(&mut name).expect("unable to read name");
                        let name = name.trim().to_string();
                        match contacts.get(&name){
                            Some(number)=>{
                                println!("The number of {} is {}" , name , number);
                            }
                            None=>{println!("No contact found with the name {}" , name);}
                        }

                    },
                    "2" =>{
                        println!("Please enter the number of the contact :");
                        let mut number : String = String::new();
                        io::stdin().read_line(&mut number).expect("unable to read number");
                        let number = number.trim().to_string();
                        let mut found = false;
                        for (name , num) in contacts.iter(){
                            if num == &number{
                                println!("The contact is {} with phone number {}" , name , number);
                                found = true;
                                break;
                            }
                        }
                        if found == false{
                            println!("No contact found with number {}" ,number);
                        }


                    }
                    &_ => {println!("Invalid choice");}


                }              
            }
            "3" => {println!("Exiting the program"); break;} ,
            &_ => {println!("Invalid choice");}            
        }
    }
    println!("Thank you for using the contact manager");
}