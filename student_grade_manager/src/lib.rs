pub mod grade_manager{
    pub fn add_student_grade() -> (String , Vec<u32>) {
        use std::io;
        let mut student = String::new();
        let mut grades : Vec<u32> = Vec::new();
        println!("Enter the name of the student : ");
        io::stdin().read_line(&mut student).expect("Failed to read line");
        let student = student.trim();
        for _i in 0..5{
            let mut grade = String::new();
            println!("Enter the grade of the student :");
            io::stdin().read_line(&mut grade).expect("Failed to read line");
            let grade = match grade.trim().parse::<u32>(){
                Ok(grade) => grade,
                Err(_) => {
                    println!("Please enter a integer value");
                    let temp : u32 = 0;
                    temp
                }
            };
            grades.push(grade);
        }
        (student.to_string() , grades) 
    }
    pub fn average_grade(grades : &Vec<u32>) -> f32{
        let mut sum : u32 = 0;
        for i in grades{
            sum += i;
        }
        let average : f32 = sum as f32 / 5.0 ;
        average
    }
}