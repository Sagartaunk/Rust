use std::collections::HashMap;
use std::io;
use student_grade_manager::grade_manager;
fn main(){
    let mut  grades : HashMap<String , Vec<u32>> = HashMap::new();
    loop{
        println!("PLease select one of the following options : ");
        println!("1. Add a student and grade");
        println!("2. Print all students and grades");
        println!("3. Print the average grade of a student");
        println!("4. Exit");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice = choice.trim() ; 
        match choice{
            "1" =>{
                let (name , grade) = grade_manager::add_student_grade();
                grades.insert(name , grade);
            }
            "2" => {
                for i in grades.iter(){
                    println!("Name : {} , Grades : {:?}" , i.0 , i.1);
                }
            }
            "3" =>{
                println!("Enter the name of the student : ");
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("Failed to read line");
                let name = name.trim();
                match grades.get(name){
                    Some(grade) =>{
                        let average : f32 = grade_manager::average_grade(grade);
                        println!("The average grade of {} is : {}" , name , average);
                    }
                    None => {println!("Student not found")}
                }
            }
            "4" => {break;}
            _ => {println!("Please enter a valid choice");}
        }
    }
}
