mod first_module {
    pub fn print_characters() {
        for c in 'a'..='Z' {
            println!("{}", c);
        }
    }
}

mod second_module {
        pub fn print_characters() {
            for c in 'A'..='z' {
                println!("{}", c);
            }
        }
    
}

// mod third_module{
//     pub fn print_characters() {
//         for c in 1..100 {
//             print!("{}", c);
//         }
//     }
// }

fn main() {
    first_module::print_characters();
    second_module::print_characters();
}
