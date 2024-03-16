use crate::domain::active_file::ActiveFile;
use anyhow::Result;
use crate::domain::index_file::AppendInfo;
use crate::infrastructure::Append;

#[derive(Debug)]
pub struct ActiveFileService {
    path: &'static str,
    inner: ActiveFile
}

impl ActiveFileService {
    
    /*
        创建一个新的activeFile
    */
    pub async fn create(id: &str) -> Result<ActiveFile> {
        todo!()
    }
}


impl Append for ActiveFileService {
    type Item = ();
    type Out = Result<AppendInfo>;

    async fn append(&self, entry: Self::Item) -> Self::Out {
        todo!()
    }
}
