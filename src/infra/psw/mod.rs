
use passwords::PasswordGenerator;

pub fn genpassword(length: usize) -> String {
    let pass = PasswordGenerator {
       length,
       numbers: true,
       lowercase_letters: true,
       uppercase_letters: false,
       symbols: false,
       spaces: false,
       exclude_similar_characters: false,
       strict: true,
   };

    format!("{}-{}", pass.generate_one().unwrap(), pass.generate_one().unwrap()) 
}