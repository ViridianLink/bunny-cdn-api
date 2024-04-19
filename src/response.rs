use crate::bunny_file::BunnyFile;

#[derive(Debug)]
pub enum ListResponse {
    Success(Vec<BunnyFile>),
    Error(String),
}
