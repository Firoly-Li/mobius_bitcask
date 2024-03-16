pub(crate) mod value;

use bytes::{Buf, BufMut, Bytes, BytesMut};
use crate::domain::entry::value::Value;
use crate::error::BitcaskError;
use crate::infrastructure::coder::{Decoder, Encoder};
use crate::infrastructure::crc::CrcService;

/**
 * BitCask写入文件的一条数据叫Entry
 */
#[derive(Debug,Clone)]
pub struct Entry {
    pub(crate) crc: u32,
    // 时间戳
    pub(crate) tstamp: u64,
    // key
    pub(crate) key: String,
    // 已经序列化的value
    pub(crate) value: Value,
}

impl Entry {
    pub fn new(tstamp: u64, key: &str, value: Value) -> Self {
        let mut v = BytesMut::new();
        v.put_u64(tstamp);
        v.put_u16(key.as_bytes().len() as u16);
        v.put_u32(value.v.len() as u32);
        v.put(key.as_bytes());
        v.put(value.v.clone());

        let crc = CrcService::new().crc(&v.freeze()).unwrap();

        Self {
            crc,
            tstamp,
            key: key.to_string(),
            value,
        }
    }

    pub fn len(&self) -> usize {
        18 + self.key.as_bytes().len() + self.value.v.len()
    }

    /*
        根据当前active_file的可写index，计算value的index
        即： value_index = index + 18 + ks
    */
    pub fn value_pos(&self,index: u64) -> u64 {
        index + 18 + (self.key.as_bytes().len()) as u64
    }

    /**
    校验crc参数
    todo: 具体未实现
     */
    fn check(&self) -> bool {
        true
    }
}

/*
    把一个结构体转换成Entry
*/
pub trait IntoEntry {
    fn into_entry(self) -> Entry;
}

impl Encoder for Entry {
    type Error = BitcaskError;

    fn encode(&self, buffer: &mut BytesMut) -> Result<usize, Self::Error> {
        // crc
        buffer.put_u32(self.crc);
        // time
        buffer.put_u64(self.tstamp);
        // ks
        let ks = self.key.as_bytes().len();
        buffer.put_u16(ks as u16);
        // vs
        let vs = self.value.v.len();
        buffer.put_u32(vs as u32);
        buffer.put(self.key.as_bytes());
        buffer.put(self.value.v.clone());
        let len = 18 + ks + vs;
        Ok(len)
    }
}

impl Decoder for Entry {
    type Error = BitcaskError;

    fn decode(mut bytes: Bytes) -> Result<Self, Self::Error> {
        if bytes.len() < 18 {
            return Err(BitcaskError::NotFound);
        };
        let crc = bytes.get_u32();
        if let Some(crc1) = CrcService::new().crc(&bytes) {
            if crc == crc1 {
                let tstamp = bytes.get_u64();
                let ks = bytes.get_u16();
                let vs = bytes.get_u32();
                let b = bytes.split_to(ks as usize);
                let key = String::from_utf8(b.to_vec()).unwrap();
                let value = bytes;
                Ok(Self {
                    crc,
                    tstamp,
                    key,
                    value:Value::from(value),
                })
            } else {
                Err(BitcaskError::NotFound)
            }
        } else {
            Err(BitcaskError::NotFound)
        }
    }
}



#[cfg(test)]
mod test {
    use bytes::BytesMut;
    use crate::domain::entry::Entry;
    use crate::domain::entry::value::Value;
    use crate::infrastructure::coder::{Decoder, Encoder};
    use crate::infrastructure::utils::now;

    #[test]
    fn entry_test() {
        let va: u64 = 1232453459834;

        let s = "hello";

        let b = true;

        let e1 = Entry::new(now() as u64,"1",Value::from(va));
        println!("e1 = {:?}",e1);
        let value = e1.value;
        let i = u64::try_from(&value).unwrap();
        println!("i = {}",i);

        let e1 = Entry::new(now() as u64,"1",Value::from(s));
        println!("e1 = {:?}",e1);
        let value = e1.value;
        let i = String::try_from(&value).unwrap();
        println!("i = {}",i);

        let e1 = Entry::new(now() as u64,"1",Value::from(b));
        println!("e1 = {:?}",e1);
        let value = e1.value;
        let i = bool::try_from(&value).unwrap();
        println!("i = {}",i);

        let e1 = Entry::new(now() as u64,"1",Value::from(s));
        println!("e1 = {:?}",e1);
        let value = e1.value;
        let i = String::try_from(&value).unwrap();
        println!("i = {}",i);
    }


    fn create_str_entry() -> Entry {
        let s = "hello";
        let e1 = Entry::new(now() as u64,"str",Value::from(s));
        e1
    }

    fn create_i32_entry() -> Entry {
        let s = 123;
        let e1 = Entry::new(now() as u64,"i32",Value::from(s));
        e1
    }

    fn create_i64_entry() -> Entry {
        let s: i64 = 1233456778903;
        let e1 = Entry::new(now() as u64,"i64",Value::from(s));
        e1
    }


    fn create_f64_entry() -> Entry {
        let s: f64 = 1233456778903.33425356;
        let e1 = Entry::new(now() as u64,"f64",Value::from(s));
        e1
    }

    fn create_bool_entry() -> Entry {

        let s = true;
        let e1 = Entry::new(now() as u64,"bool",Value::from(s));
        e1
    }

    #[test]
    fn entry_encode_decode_test() {

        entry_encode_test(create_i64_entry);

        entry_encode_test(create_f64_entry);

        entry_encode_test(create_bool_entry);

        entry_encode_test(create_str_entry);

        entry_encode_test(create_i32_entry);

    }

    fn entry_encode_test<F>(create_entry: F)
        where
            F: Fn() -> Entry
    {
        let entry = create_entry();
        println!("entry = {:?}",entry);
        let mut b = BytesMut::new();
        let _ = entry.encode(&mut b);

        let e =Entry::decode(b.freeze());
        println!("e = {:?}",e);
    }
}