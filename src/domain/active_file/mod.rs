/*
    BitCask算法中的active_file
*/
mod event;
mod service;

use std::path::Path;
use std::sync::Arc;
use tokio::fs::{File, OpenOptions};
use tokio::sync::Mutex;
use crate::DB_POSTFIX;
use crate::domain::index_file::AppendInfo;
use crate::infrastructure::Append;
use anyhow::{Error, Result};
use bytes::BytesMut;
use tokio::io::AsyncWriteExt;
use crate::domain::entry::Entry;
use crate::error::AppendError;
use crate::infrastructure::coder::Encoder;

#[derive(Debug)]
pub struct ActiveFile {
    // file_id
    id: &'static str,
    offset: usize,
    pub file: Arc<Mutex<File>>,
}


impl ActiveFile {

    pub async fn new(id: &'static str,path: &str) -> Self {
        let file_path = path.to_string() + id + DB_POSTFIX;
        let path = Path::new(file_path.as_str());
        match OpenOptions::new().append(true).read(true).open(path).await {
            Ok(file) => Self {
                id,
                offset: 0,
                file: Arc::new(Mutex::new(file)),
            },
            Err(_err) => {
                // err分支说明文件还没有创建，那就创建这个文件
                if let Err(_r) = File::create(path).await {
                    panic!("创建文件失败：{:?}", path);
                };

                let file = OpenOptions::new()
                    .append(true)
                    .read(true)
                    .open(path)
                    .await
                    .unwrap();
                Self {
                    id,
                    offset: 0,
                    file: Arc::new(Mutex::new(file)),
                }
            }
        }
    }
    
    fn id(&self) -> &'static str {
        self.id
    }
    
    /*
        todo 判断active_file是否可以继续写入
    */
    fn enable_write(&self) -> bool{
        true
    }
}


impl Append for ActiveFile {
    type Item = Entry;
    type Out = Result<AppendInfo>;

    async fn append(&self, entry: Self::Item) -> Self::Out {
        let mut bytes = BytesMut::new();
        entry.encode(&mut bytes).unwrap();
        let active_file_id = self.id;
        let length = bytes.len();
        let key = entry.key.as_str();
        let old_offset = self.offset;
        // let new_offset = old_offset + length as usize;
        let tstamp = entry.tstamp;
        match self.enable_write() {
            true => {
                let resp = self.file.lock().await.write_all(&*bytes.freeze()).await;
                match resp {
                    Ok(_usize) => {
                        let append_info = AppendInfo::new(
                            active_file_id,
                            key,
                            tstamp,
                            old_offset as u64,
                            length as u64,
                        );
                        let _ = self.file.lock().await.flush().await;
                        // println!("写入数据成功！");
                        Ok(append_info)
                    }
                    Err(_err) => Err(Error::from(AppendError::NotFound)),
                }
            }
            false => Err(Error::from(AppendError::OutOfMaxSize)),
        }
    }
}






#[cfg(test)]
mod test {
    
    #[test]
    fn active_file_test() {
        
    }
}