use crate::checks::ChecksError;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error(transparent)]
    Checks(#[from] ChecksError),
}
