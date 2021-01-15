use std::time::SystemTime;

pub trait RagnarokCodec {
    fn encode<T: EncodeFixed>(&mut self, val: &T);
    fn encode_struct<T: EncodeStruct>(&mut self, val: &T);
    fn padding(&mut self, count: usize);
}

pub trait EncodeFixed {
    const SIZE: usize;
    fn encode(&self, buf: &mut [u8]);
}

pub trait EncodeStruct {
    fn encode<C: RagnarokCodec>(&self, codec: &mut C);
}

macro_rules! encode_fixed {
    ($ty:ty, $size:literal) => {
        impl EncodeFixed for $ty {
            const SIZE: usize = $size;

            fn encode(&self, buf: &mut [u8]) {
                buf.copy_from_slice(&self.to_le_bytes());
            }
        }
    };
}

encode_fixed!(u16, 2);
encode_fixed!(i16, 2);
encode_fixed!(u32, 4);
encode_fixed!(i32, 4);
encode_fixed!(u64, 8);
encode_fixed!(i64, 8);

impl EncodeFixed for u8 {
    const SIZE: usize = 1;
    fn encode(&self, buf: &mut [u8]) {
        buf[0] = *self;
    }
}

impl EncodeFixed for SystemTime {
    const SIZE: usize = 4;
    fn encode(&self, buf: &mut [u8]) {
        let seconds_since_epoch = self
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as u32;
        buf.copy_from_slice(&seconds_since_epoch.to_le_bytes());
    }
}

pub struct ClientTcpCodec<'a> {
    buf: &'a mut [u8],
    cursor: usize,
}

impl<'a> ClientTcpCodec<'a> {
    pub fn new(buf: &'a mut [u8]) -> Self {
        Self { buf, cursor: 0 }
    }

    pub fn capacity(&self) -> usize {
        self.buf.len()
    }

    pub fn len(&self) -> usize {
        self.cursor
    }
}

impl<'a> RagnarokCodec for ClientTcpCodec<'a> {
    fn encode<T: EncodeFixed>(&mut self, val: &T) {
        let offset = self.cursor;
        val.encode(&mut self.buf[offset..offset + T::SIZE]);
        self.cursor += T::SIZE;
    }

    fn encode_struct<T: EncodeStruct>(&mut self, val: &T) {
        val.encode(self);
    }

    fn padding(&mut self, count: usize) {
        for i in self.cursor..self.cursor + count {
            self.buf[i] = 0;
        }
        self.cursor += count;
    }
}
