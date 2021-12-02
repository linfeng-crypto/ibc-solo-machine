#[derive(Debug, Clone)]
pub enum AppError {
    FileError,
    DbError,
    WriteError,
    FormatError,
}
