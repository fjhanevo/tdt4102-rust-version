use std::collections::HashMap;

#[derive(Debug)]
pub struct CourseCatalog {
    courses: HashMap<String, String>,
}

impl CourseCatalog {
    pub fn add_course(&mut self, key: String, value: String) {
        self.courses.insert(key, value);
    }
    
    pub fn remove_course(&mut self, key: String) {
        self.courses.remove(&key);
    }
}
