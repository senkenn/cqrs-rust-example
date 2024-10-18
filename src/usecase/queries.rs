use crate::domain::student::Student;
use crate::infrastructure::repository::QueryRepository;

pub struct GetStudentUsecase<'a, R: QueryRepository> {
    pub repository: &'a R,
}

impl<'a, R: QueryRepository> GetStudentUsecase<'a, R> {
    pub fn execute(&self, id: u32) -> Option<Student> {
        self.repository.find_by_id(id)
    }
}
