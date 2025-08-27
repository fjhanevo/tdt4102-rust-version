use std::collections::HashMap;
use std::fmt;

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
