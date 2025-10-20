
fn main(){
    let student1 = ("Borjan", 27, 9.5);
    let student2 = ("Jordan", 23, 9.0);
    let student3 = ("Srdjan", 42, 10.0);

    let students = [student1, student2, student3];

    println!("{:<10} {:<5} {:<15}", "Name", "Age", "Grade Average");
    println!("-------------------------------------");

    let mut total_grade = 0.0;

    for student in &students {
        let (name, age, grade_average) = student;
        println!("{:<10} {:<5} {:<15.2}", name, age, grade_average);
        total_grade += grade_average;
    }

    let average_grade = total_grade / students.len() as f32;
    println!("\nAverage Grade of All Students: {:.2}", average_grade);
}