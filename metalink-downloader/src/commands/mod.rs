mod download_file;
mod download_metalink;
mod plan;

pub use download_file::download_file;
pub use download_metalink::download_metalink;
pub use plan::plan;

pub(crate) enum ProgressUpdate {
    // Download progressed by n bytes
    Progressed(u64),
    Finished,
}
