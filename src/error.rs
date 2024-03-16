use thiserror::Error;

#[derive(Error, Debug)]
pub enum BitcaskError {
    #[error("Not found")]
    NotFound,
}

#[derive(Error, Debug)]
pub enum CheckError {
    #[error("Not found")]
    NotFound,
}

#[derive(Error, Debug)]
pub enum ActiveFileError {
    #[error("Not found")]
    NotFound,
    #[error("写入失败:【超过file_max_size 设置的最大值】")]
    OutOfMaxSize,
}

#[derive(Error, Debug)]
pub enum AppendError {
    #[error("Not found")]
    NotFound,
    #[error("写入失败:【超过file_max_size 设置的最大值】")]
    OutOfMaxSize,
}

#[derive(Error, Debug)]
pub enum FileError {
    #[error("Not found")]
    NotFound,
    #[error("写入失败:【超过file_max_size 设置的最大值】")]
    OutOfMaxSize,
}

#[derive(Error, Debug)]
pub enum IndexError {
    #[error("Not found")]
    NotFound,
    #[error("写入失败:【超过file_max_size 设置的最大值】")]
    OutOfMaxSize,
}
