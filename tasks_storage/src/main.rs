use std::io;
struct Task{
    name : String,
    completed : bool
}
fn main() {
    let mut tasks : Vec<Task> = Vec::new();
    loop {
        println!("Plese enter one of the following commands : ");
        println!("1. Add task");
        println!("2. Remove Task");
        println!("3. Mark task as done");
        println!("4. view Tasks");
        println!("5. Exit");
        let mut command = String::new();
        io::stdin().read_line(&mut command).expect("Failed to read command");
        let command = command.trim();
        match command {
            "1" => {
                println!{"Please enter your task : "};
                let mut t = String::new();
                io::stdin().read_line(&mut t).expect("Failed to read Task");
                let t = t.trim();
                tasks.push(Task{name : t.to_string() , completed : false});
            },
            "2"=>{
                println!("Please enter the index of the task you want to remove :");
                let mut index = String::new();
                io::stdin().read_line(&mut index).expect("Failed to read the index");
                let index = index.trim().parse();
                match index {
                    Ok(i) =>{tasks.remove(i);},
                    Err(e) =>{println!{"Error , {}" , e};
                }
            }
            },
            "3" =>{
                println!("Please enter the index of the task you want to mark as done : ");
                let mut index = String::new();
                io::stdin().read_line(&mut index).expect("Failed to read the index");
                let index = index.trim().parse::<usize>();
                match index {
                    Ok(i) if i >0 && i <= tasks.len()=> {tasks[i].completed = true;},
                    Err(e) => {println!("Error , {} " , e);}, 
                    Ok(_) => {println!("Invalid index");}
                }                
            },
            "4" => {
                if tasks.is_empty(){
                    println!("No tasks to display");
                }else{
                    for i in tasks.iter().enumerate(){
                        let status = match i.1.completed{
                            true => {"[Completed]"},
                            false => {"[Not Completed]"}
                        };
                        println!("{} {}" , status , i.1.name)
                    }
                }
            },
            "5" =>{
                println!("Exiting the program ........");
                break;
            },
            
            &_ =>{println!("Invalid command");}
        }

    }
    println!("Program exitted .....");

}