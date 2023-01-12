pub trait Key {
    fn from_u8(key: &[u8]) -> Self;
    fn as_slice<T, F: Fn(&[u8]) -> T>(&self, f: F) -> T;
}

pub fn from_u8<K: Key>(key: &[u8]) -> K {
    Key::from_u8(key)
}

impl Key for i32 {
    fn from_u8(key: &[u8]) -> i32 {
        assert!(key.len() == 4);

        (key[0] as i32) << 24 | (key[1] as i32) << 16 | (key[2] as i32) << 8 | (key[3] as i32)
    }

    fn as_slice<T, F: Fn(&[u8]) -> T>(&self, f: F) -> T {
        let mut dst = [0u8, 0, 0, 0];
        dst[0] = (*self >> 24) as u8;
        dst[1] = (*self >> 16) as u8;
        dst[2] = (*self >> 8) as u8;
        dst[3] = *self as u8;
        f(&dst)
    }
}

impl Key for String {
    fn from_u8(key: &[u8]) -> Self {
        let s = String::from_utf8(key.to_vec()).unwrap_or_default();
        s
    }

    fn as_slice<T, F: Fn(&[u8]) -> T>(&self, f: F) -> T {    
        f(self.as_bytes())
    }
}

impl Key for Vec<u8> {
    fn from_u8(key: &[u8]) -> Self {
        key.to_vec()
    }

    fn as_slice<T, F: Fn(&[u8]) -> T>(&self, f: F) -> T {    
        f(self.as_slice())
    }
}

