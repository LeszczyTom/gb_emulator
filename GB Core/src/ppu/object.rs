#[derive(Clone, Copy, Debug)]
pub struct OamObject {
    pub data: [u8; 4],
}

impl OamObject {
    pub fn new(obj: &[u8]) -> Self {
        assert_eq!(obj.len(), 4);

        Self {
            data: [obj[0], obj[1], obj[2], obj[3]],
        }
    }

    pub fn y(&self) -> u8 {
        return self.data[0];
    }

    pub fn x(&self) -> u8 {
        return self.data[1];
    }

    pub fn tile_index(&self) -> u8 {
        return self.data[2];
    }

    pub fn flag(&self) -> u8 {
        return self.data[3];
    }

    pub fn priority(&self) -> bool {
        return (self.flag() >> 7) & 1 == 1;
    }

    pub fn palette(&self) -> bool {
        return (self.flag() >> 4) & 1 == 1;
    }
}
