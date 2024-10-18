use crate::domain::student::Student;
use crate::infrastructure::repository::CommandRepository;

pub struct CreateStudentUsecase<'a, R: CommandRepository> {
    pub repository: &'a R,
}

impl<'a, R: CommandRepository> CreateStudentUsecase<'a, R> {
    pub fn execute(&self, id: u32, name: String) {
        let student = Student::new(id, name);
        self.repository.save(student);
    }
}
