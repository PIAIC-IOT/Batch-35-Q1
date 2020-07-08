#[derive(Debug)]
struct Student {
    roll_no: String,
    age: u32,
    grade: char,
    passed: bool
} //blue print

impl Student {
    fn new(roll_no1:String,age1:u32, grade1:char, passed1: bool) -> Student {
        Student {
            roll_no: roll_no1,
            age: age1,
            grade:grade1,
            passed:passed1,
        }
    }//associated function

    fn read(&self, book: String) {
        println!("{} is reading {} ",self.roll_no,book)
    } //method
}


// #[derive(Debug)]
// struct Animal {
//     name: String,
// }

// impl Animal {
//     fn new (name1:String) -> Animal {
//         Animal {
//             name:name1,
//         }
//     }

//     fn run(&self) {
//         println! ("Horse {} can run", self.name);
//     }

// }

fn is_eligible(stud: Student) -> u32 {
    if stud.age < 25 {
        return 1 
    }
    else {
        return 0
    }
}


fn main () {

    // let rehman = Student {
    //     roll_no:String::from("iot-01"),
    //     age:24,
    //     grade:'A',
    //     passed: true,
    // }; 

 
    let rehman = Student::new(String::from("iot-1"), 24, 'A',true);
    println!("{:#?}",rehman);
    let result = is_eligible(rehman);
    // let horse  = Animal::new(String::from("TOM"));

}



