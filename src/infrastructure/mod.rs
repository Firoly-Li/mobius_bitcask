pub(crate) mod utils;
pub(crate) mod crc;
pub(crate) mod coder;
pub(crate) mod event;


/*
    
*/
pub(crate) trait Append {
    type Item;
    
    type Out;
    
    async fn append(&self, entry: Self::Item) -> Self::Out;
}