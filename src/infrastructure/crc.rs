use bytes::{BufMut, Bytes, BytesMut};
use crc::Crc;
use std::sync::Arc;

const CUSTOM_ALG: crc::Algorithm<u32> = crc::Algorithm {
    width: 32,
    poly: 0x8005,
    init: 0xffff,
    refin: false,
    refout: false,
    xorout: 0x0000,
    check: 0xaee7,
    residue: 0x0000,
};

pub struct CrcService {
    inner: Arc<Crc<u32>>,
}

impl CrcService {
    pub fn new() -> Self {
        let crc = crc::Crc::<u32>::new(&CUSTOM_ALG);
        Self {
            inner: Arc::new(crc),
        }
    }

    pub fn crc(&self, bytes: &Bytes) -> Option<u32> {
        let mut disgest = self.inner.digest();
        disgest.update(bytes);
        let resp = disgest.finalize();
        Some(resp)
    }
}

pub fn crcs(client_id: &str, topic_name: &str, tstamp: u64, payload: &Bytes) -> u32 {
    let mut buff = BytesMut::new();
    buff.put(tstamp.to_string().as_bytes());
    buff.put(client_id.as_bytes());
    buff.put(topic_name.as_bytes());
    buff.put(payload.clone());
    let crc = CrcService::new()
        .crc(&(Bytes::from(buff.to_vec())))
        .unwrap();
    crc
}
