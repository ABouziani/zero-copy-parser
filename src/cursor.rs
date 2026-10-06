pub struct Cursor<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }

    pub fn position(&self) -> usize {
        self.pos
    }

    pub fn remaining(&self) -> usize {
        self.data.len() - self.pos
    }

    pub fn read_u8(&mut self) -> Result<u8, &'static str> {
        if self.pos >= self.data.len() {
            return Err("unexpected end of input");
        }

        let value = self.data[self.pos];
        self.pos += 1;

        Ok(value)
    }

    pub fn read_u16_be(&mut self) -> Result<u16, &'static str> {
        if self.remaining() < 2 {
            return Err("unexpected end of input");
        }

        let value = u16::from_be_bytes([
            self.data[self.pos],
            self.data[self.pos + 1],
        ]);

        self.pos += 2;

        Ok(value)
    }

    pub fn read_u32_le(&mut self) -> Result<u32, &'static str> {
        if self.remaining() < 4 {
            return Err("unexpected end of input");
        }

        let value = u32::from_le_bytes([
            self.data[self.pos],
            self.data[self.pos + 1],
            self.data[self.pos + 2],
            self.data[self.pos + 3],
        ]);

        self.pos += 4;

        Ok(value)
    }

    pub fn read_bytes(&mut self, len: usize) -> Result<&'a [u8], &'static str> {
        let end = self
            .pos
            .checked_add(len)
            .ok_or("integer overflow")?;

        if end > self.data.len() {
            return Err("unexpected end of input");
        }

        let bytes = &self.data[self.pos..end];
        self.pos = end;

        Ok(bytes)
    }
}