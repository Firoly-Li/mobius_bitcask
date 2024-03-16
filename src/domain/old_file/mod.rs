mod event;
mod service;

use tokio::fs::File;
use tokio::io::BufReader;

/*
    OldFile
*/
#[derive(Debug)]
pub struct OldFile {
    // 文件的id
    id: String,
    buf: BufReader<File>,
}