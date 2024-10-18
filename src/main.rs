mod domain;
mod infrastructure;
mod usecase;

use infrastructure::repository::InMemoryRepository;
use usecase::commands::CreateStudentUsecase;
use usecase::queries::GetStudentUsecase;

fn main() {
    // インフラのリポジトリを初期化
    let repository = InMemoryRepository::new();

    // コマンドユースケースの実行 (Student の作成)
    let create_student = CreateStudentUsecase {
        repository: &repository,
    };
    create_student.execute(1, "John Doe".to_string());

    // クエリユースケースの実行 (Student の取得)
    let get_student = GetStudentUsecase {
        repository: &repository,
    };
    if let Some(student) = get_student.execute(1) {
        println!("Found student: {} with id {}", student.name, student.id);
    } else {
        println!("Student not found");
    }
}
