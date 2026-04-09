use ql_core::{Instance, IoError, file_utils};
use std::collections::HashSet;
use std::path::PathBuf;
use thiserror::Error;

const ERR_PREFIX: &str = "while cloning instance:\n";
#[derive(Debug, Error)]
pub enum InstanceCloneError {
    #[error("{ERR_PREFIX}failed to execute recursive directory clone wrapper: {0:?}")]
    Io(IoError),
    #[error("{ERR_PREFIX}directory already exists: {0:?}")]
    DirectoryExists(PathBuf),
    #[error("{ERR_PREFIX}parent directory not found: {0:?}")]
    ParentNotFound(PathBuf),
}

impl From<IoError> for InstanceCloneError {
    fn from(e: IoError) -> Self {
        Self::Io(e)
    }
}

pub async fn clone_instance(
    instance: Instance,
    exceptions: HashSet<String>,
) -> Result<Instance, InstanceCloneError> {
    let new_instance_name = format!("{} (copy)", instance.name);

    let src_path = instance.get_instance_path();
    let dst_path = src_path
        .parent()
        .ok_or_else(|| InstanceCloneError::ParentNotFound(src_path.clone()))?
        .join(&new_instance_name);

    if dst_path.is_dir() {
        return Err(InstanceCloneError::DirectoryExists(dst_path));
    }

    let exceptions: Vec<PathBuf> = exceptions.iter().map(|n| src_path.join(n)).collect();
    file_utils::copy_dir_recursive_ext(&src_path, &dst_path, &exceptions).await?;

    Ok(Instance::new(&new_instance_name, instance.kind))
}
