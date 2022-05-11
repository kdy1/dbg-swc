use anyhow::Result;

/// Type annotation
pub fn wrap_task<T, F>(op: F) -> Result<T>
where
    F: FnOnce() -> Result<T>,
{
    op()
}
