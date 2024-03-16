
/*
    由于BitCask的value都是用字节存储的，所以只要能够转换为字节数组的对象都可以存储到BitCask中
*/
use bytes::Bytes;

#[derive(Clone,Debug)]
pub struct Value {
    pub(crate) v: Bytes
}


impl From<Bytes> for Value {
    fn from(value: Bytes) -> Self {
        Self {
            v: value
        }
    }
}


impl From<String> for Value {
    fn from(value: String) -> Self {
        Self {
            v: Bytes::from(value)
        }
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Self {
            v: Bytes::from(value.to_string())
        }
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Self {
            v: Bytes::from(value.to_be_bytes().to_vec())
        }
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Self {
            v: Bytes::from(value.to_be_bytes().to_vec())
        }
    }
}

impl From<u32> for Value {
    fn from(value: u32) -> Self {
        Self {
            v: Bytes::from(value.to_be_bytes().to_vec())
        }
    }
}

impl From<u64> for Value {
    fn from(value: u64) -> Self {
        Self {
            v: Bytes::from(value.to_be_bytes().to_vec())
        }
    }
}

impl From<usize> for Value {
    fn from(value: usize) -> Self {
        Self {
            v: Bytes::from(value.to_be_bytes().to_vec())
        }
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Self {
            v: Bytes::from(value.to_be_bytes().to_vec())
        }
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self {
            v: Bytes::from(value.to_be_bytes().to_vec())
        }
    }
}


impl From<bool> for Value {
    fn from(value: bool) -> Self {
        match value {
            true => {
                let v: u8 = 1;
                Self {
                    v: Bytes::from(vec![v])
                }
            }
            false => {
                let v: u8 = 0;
                Self {
                    v: Bytes::from(vec![v])
                }
            }
        }
    }
}




impl TryFrom<&Value> for String {
    type Error = ();

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let s = String::from_utf8(value.v.to_vec());
        match s {
            Ok(str) => Ok(str),
            Err(e) => Err(())
        }
    }
}

impl TryFrom<&Value> for i32 {
    type Error = ();

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let vs = value.v.clone();
        if vs.len() > 4 {
            return Err(())
        }
        let i = i32::from_be_bytes(vs[..4].try_into().unwrap());
        Ok(i)
    }
}

impl TryFrom<&Value> for i64 {
    type Error = ();

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let vs = value.v.clone();
        if vs.len() >8 {
            return Err(())
        }
        let i = i64::from_be_bytes(vs[..8].try_into().unwrap());
        Ok(i)
    }
}



impl TryFrom<&Value> for u32 {
    type Error = ();

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let vs = value.v.clone();
        if vs.len() >4 {
            return Err(())
        }
        let i = u32::from_be_bytes(vs[..4].try_into().unwrap());
        Ok(i)
    }
}



impl TryFrom<&Value> for u64 {
    type Error = ();

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let vs = value.v.clone();
        if vs.len() >8 {
            return Err(())
        }
        let i = u64::from_be_bytes(vs[..8].try_into().unwrap());
        Ok(i)
    }
}



impl TryFrom<&Value> for usize {
    type Error = ();

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let vs = value.v.clone();
        if vs.len() >8 {
            return Err(())
        }
        let i = usize::from_be_bytes(vs[..8].try_into().unwrap());
        Ok(i)
    }
}

impl TryFrom<&Value> for bool {
    type Error = ();

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let vec = value.v.to_vec();
        if vec.len() != 1 {
            return Err(())
        };
        let v = vec.first().unwrap().clone();
        if v == 0 {
            Ok(false)
        }else {
            Ok(true)
        }
    }
}


impl TryFrom<&Value> for f32 {
    type Error = ();

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let vs = value.v.clone();
        if vs.len() >4 {
            return Err(())
        }
        let i = f32::from_be_bytes(vs[..4].try_into().unwrap());
        Ok(i)
    }
}



impl TryFrom<&Value> for f64 {
    type Error = ();

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let vs = value.v.clone();
        if vs.len() >8 {
            return Err(())
        }
        let i = f64::from_be_bytes(vs[..8].try_into().unwrap());
        Ok(i)
    }
}








#[cfg(test)]
mod test {
    use crate::domain::entry::value::Value;

    #[test]
    fn string_to_value_test() {
        let string = String::from("hello");
        let v = Value::from(string.clone());
        let s = String::try_from(&v).unwrap();
        println!("v = {:?}",v);
        println!("s = {:?}",s);

        assert_eq!(string,s);

        let v = Value::from("string");
        let s = String::try_from(&v).unwrap();
        println!("v = {:?}",v);
        println!("s = {:?}",s);
        assert_eq!("string", s);

    }


    #[test]
    fn int_to_value_test() {
        // usize
        let num: i32 = 123;
        let v = Value::from(num);
        println!("v = {:?}",v);
        let i = i32::try_from(&v).unwrap();
        println!("i = {:?}",i);
        assert_eq!(num, i);

        let num: i64 = 2147483747;
        let v = Value::from(num);
        println!("v = {:?}",v);
        let i = i64::try_from(&v).unwrap();
        println!("i = {:?}",i);
        assert_eq!(num, i);

        let num: u32 = 123;
        let v = Value::from(num);
        println!("v = {:?}",v);
        let i = u32::try_from(&v).unwrap();
        println!("i = {:?}",i);
        assert_eq!(num, i);

        let num: u64 = 2147483747;
        let v = Value::from(num);
        println!("v = {:?}",v);
        let i = u64::try_from(&v).unwrap();
        println!("i = {:?}",i);
        assert_eq!(num, i);


        let num: usize = 2147483747;
        let v = Value::from(num);
        println!("v = {:?}",v);
        let i = usize::try_from(&v).unwrap();
        println!("i = {:?}",i);
        assert_eq!(num, i)
    }




    #[test]
    fn bool_to_value_test() {
        let num = false;
        let v = Value::from(num);
        println!("v = {:?}",v);
        let i = bool::try_from(&v).unwrap();
        println!("i = {:?}",i);
        assert_eq!(num, i)
    }

    #[test]
    fn flout_to_value_test() {
        let num: f64 = 3.14157856578;
        let v = Value::from(num);
        println!("v = {:?}",v);
        let i = f64::try_from(&v).unwrap();
        println!("i = {:?}",i);
        assert_eq!(num, i)
    }

}


