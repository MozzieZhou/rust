//请基于Rust的基本数据结构写一个简单的学生管理系统（比如，学生，社团，班级、课程等），明确类型之间的关系，进行基本的CRUD操作。
#[derive(Debug, Clone)]
struct Student {
    id: u32,
    name: String,
}

impl Student {
    pub fn new(id: u32, name: &str) -> Student {
        Student {
            id: id,
            name: String::from(name),
        }
    }
}

#[derive(Debug)]
struct Class {
    id: u32,
    studs: Vec<Student>,
}


struct Course {
    id: u32,
    subject: Subject,
    classes: Vec<Class>,
}

impl Course {
    pub fn new(id: u32, subject: Subject, clas: Vec<Class>) -> Course {
        Course {
            id,
            subject,
            classes: clas,
        }
    }
}

enum Subject {
    Math,
    Chinese,
    English,
    Science,
}

struct School {
    students: Vec<Student>,
    courses: Vec<Course>,
}

// 学校管理课程等信息
impl School {
    fn new(studs: Vec<Student>, cours: Vec<Course>) -> School {
        let mut school = School {
            students: studs,
            courses: cours,
        };
        school
    }

    fn add_student(&mut self, student: Student) {
        self.students.push(student);
    }

    fn add_course(&mut self, course: Course) {
        self.courses.push(course);
    }

    fn get_student_by_id(&self, id: u32) -> &Student {
        self.students.iter().find(|s| s.id == id).expect("find no one")
    }

    fn get_course_by_id(&self, id: u32) -> Option<&Course> {
        self.courses.iter().find(|c| c.id == id)
    }

    fn get_class_by_id(&self, course_id: u32, class_id: u32) -> Option<&Class> {
        if let Some(course) = self.get_course_by_id(course_id) {
            course.classes.iter().find(|c| c.id == class_id)
        } else {
            None
        }
    }

    fn add_student_to_class(&mut self, course_id: u32, class_id: u32, student: Student) {
        if let Some(course) = self.courses.iter_mut().find(|c| c.id == course_id) {
            if let Some(class) = course.classes.iter_mut().find(|c| c.id == class_id) {
                class.studs.push(student);
            }
        }
    }

    fn print_student_info(&self, student_id: u32) {
        let target_student = self.students.iter().find(|s| s.id == student_id).expect("find no student");
        println!("{:?}", target_student);
    }
}


fn main() {

    // 三种方式创建学生信息
    let stud1 = Student {
        id: 1,
        name: String::from("Mozzie"),
    };

    let stud2 = Student {
        id: 2,
        ..stud1.clone()
    };

    let stud3 = Student::new(3, "kaley");

    // 学校中增加学生
    let vec1 = vec![stud1.clone(), stud2.clone(), stud3.clone()];

    // 新建Class
    let clas1 = Class {
        id: 1,
        studs: vec1.clone(),
    };

    let course = Course::new(1, Subject::Chinese, vec![clas1]);

    // 创建学校
    let mut school = School::new(vec1.clone(), vec![course]);

    // 增加学生
    School::add_student(&mut school, Student::new(4, "Herry"));

    School::add_student_to_class(&mut school, 1, 1, Student::new(5, "Penny"));

    let class = School::get_class_by_id(&school, 1, 1);
    println!("{:?}", class);
    School::print_student_info(&school, 4);
}