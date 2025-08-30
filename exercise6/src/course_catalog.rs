use std::collections::HashMap;
use std::fmt;
use std::fs::OpenOptions;
use std::io::Write;

#[derive(Debug)]
pub struct CourseCatalog {
    courses: HashMap<String, String>,
}

impl CourseCatalog {
    pub fn new() -> Self {
        CourseCatalog {
            courses: HashMap::new(),
        }
    }
    pub fn add_course(&mut self, key: String, value: String) {
        self.courses.insert(key, value);
    }
    
    pub fn remove_course(&mut self, key: String) {
        self.courses.remove(&key);
    }

    pub fn get_course(&self, key: &str) -> Option<&String>{
        self.courses.get(key)
    }

    pub fn save_to_file(&self, file_path: &str) {
        if self.courses.is_empty() {
            println!("No courses to save!");
            return;
        }

        let mut save_file = OpenOptions::new()
            .create(true)       // create if it doesn't already exist
            .write(true)        // open in "w" mode
            .append(true)       // append instead of overwriting
            .open(file_path)    
            .expect("Unable to open file!");

            
        for (key, value) in self.courses.iter() {
            writeln!(save_file, "{}: {}", key, value).expect("Failed to write data!");
        } 
    }
}

// operator overload
impl fmt::Display for CourseCatalog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "--- Course Catalog ---")?;
        for (key, value) in self.courses.iter() {
            writeln!(f, "{}: {}", key, value)?;
        }
        Ok(())
    }
}
