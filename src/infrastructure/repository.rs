use crate::domain::student::Student;
use std::collections::HashMap;
use std::sync::Mutex;

pub trait CommandRepository {
    fn save(&self, student: Student);
}

pub trait QueryRepository {
    fn find_by_id(&self, id: u32) -> Option<Student>;
}

pub struct InMemoryRepository {
    data: Mutex<HashMap<u32, Student>>,
}

impl InMemoryRepository {
    pub fn new() -> Self {
        InMemoryRepository {
            data: Mutex::new(HashMap::new()),
        }
    }
}

impl CommandRepository for InMemoryRepository {
    fn save(&self, student: Student) {
        let mut data = self.data.lock().unwrap();
        data.insert(student.id, student);
    }
}

impl QueryRepository for InMemoryRepository {
    fn find_by_id(&self, id: u32) -> Option<Student> {
        let data = self.data.lock().unwrap();
        data.get(&id).cloned()
    }
}
