use std::sync::OnceLock;

pub static CONF: OnceLock<Conf> = OnceLock::new();

#[derive(Debug, Clone)]
pub struct Conf {
    pub(crate) path: String,
    pub(crate) file_max_size: usize,
    pub(crate) file_max_count: usize,
}

