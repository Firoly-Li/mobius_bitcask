use crate::config::Conf;

mod application;

mod domain;

mod infrastructure;

pub mod interface;
mod config;
pub mod error;

pub static DB_POSTFIX: &'static str = ".db";

/*
    
*/
#[derive(Clone,Debug)]
pub struct BitCask {
    conf: Conf
}

#[derive(Clone,Debug)]
pub struct BitCaskCli {
    
}


impl BitCask {
    
    /*
        初始化方法，返回的是和BitCask交互的客户端
    */
    pub fn init(path: &str) -> Option<BitCaskCli> {
        
        None
    }
    
    pub fn init_with_conf(conf: &Conf) -> Option<BitCaskCli> {
        
        None
    }
    
}

impl BitCaskCli {
    
    pub async fn put() {
        
    }
    
    pub async fn get() {
        
    }
    
    pub async fn delete() {
        
    }
}