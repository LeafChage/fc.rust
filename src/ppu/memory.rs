pub struct MemoryMap {
    pattern0: [u8; 0x1000], // 0x0000～0x0FFF
    pattern1: [u8; 0x1000], // 0x1000～0x1FFF

    /// 32sprite * 30line = 960(0x03C0)
    name0: [u8; 0x03C0], // 0x2000～0x23BF
    attribute0: [u8; 0040], // 0x23C0～0x23FF

    name1: [u8; 0x03C0],    // 0x2400～0x27BF
    attribute1: [u8; 0040], // 0x27C0～0x27FF

    name2: [u8; 0x03C0],    // 0x2800～0x2BBF
    attribute2: [u8; 0040], // 0x2BC0～0x2BFF

    name3: [u8; 0x03C0],    // 0x2C00～0x2FBF
    attribute3: [u8; 0040], // 0x2FC0～0x2FFF

    mirror1: [u8; 0x3eff - 0x3000],   // 0x3000～0x3EFF
    background_palette: [u8; 0x0010], // 0x3F00～0x3F0F
    sprite_palette: [u8; 0x0010],     // 0x3F10～0x3F1F
    mirror2: [u8; 0x3fff - 0x3f20],   // 0x3F20～0x3FFF
}

impl Default for MemoryMap {
    fn default() -> Self {
        MemoryMap {
            pattern0: [0; 0x1000],
            pattern1: [0; 0x1000],
            name0: [0; 0x03C0],
            attribute0: [0; 0040],
            name1: [0; 0x03C0],
            attribute1: [0; 0040],
            name2: [0; 0x03C0],
            attribute2: [0; 0040],
            name3: [0; 0x03C0],
            attribute3: [0; 0040],
            mirror1: [0; 0x3eff - 0x3000],
            background_palette: [0; 0x0010],
            sprite_palette: [0; 0x0010],
            mirror2: [0; 0x3FFF - 0x3F20],
        }
    }
}

impl std::ops::Index<usize> for MemoryMap {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            i if (0x0000..=0x0FFF).contains(&i) => &self.pattern0[i],
            i if (0x1000..=0x1FFF).contains(&i) => &self.pattern1[i - 0x1000],
            i if (0x2000..=0x23BF).contains(&i) => &self.name0[i - 0x2000],
            i if (0x23C0..=0x23FF).contains(&i) => &self.attribute0[i - 0x23C0],
            i if (0x2400..=0x27BF).contains(&i) => &self.name1[i - 0x2400],
            i if (0x27C0..=0x27FF).contains(&i) => &self.attribute1[i - 0x27C0],
            i if (0x2800..=0x2BBF).contains(&i) => &self.name2[i - 0x2800],
            i if (0x2BC0..=0x2BFF).contains(&i) => &self.attribute2[i - 0x2BC0],
            i if (0x2C00..=0x2FBF).contains(&i) => &self.name3[i - 0x2C00],
            i if (0x2FC0..=0x2FFF).contains(&i) => &self.attribute3[i - 0x2FC0],
            i if (0x3000..=0x3EFF).contains(&i) => &self.mirror1[i - 0x3F00],
            i if (0x3F00..=0x3F0F).contains(&i) => &self.background_palette[i - 0x3F00],
            i if (0x3F10..=0x3F1F).contains(&i) => &self.sprite_palette[i - 0x3F10],
            i if (0x3F20..=0x3FFF).contains(&i) => &self.mirror2[i - 0x3F20],
            _ => unreachable!("{}", index),
        }
    }
}

impl std::ops::IndexMut<usize> for MemoryMap {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            i if (0x0000..=0x0FFF).contains(&i) => &mut self.pattern0[i],
            i if (0x1000..=0x1FFF).contains(&i) => &mut self.pattern1[i - 0x1000],
            i if (0x2000..=0x23BF).contains(&i) => &mut self.name0[i - 0x2000],
            i if (0x23C0..=0x23FF).contains(&i) => &mut self.attribute0[i - 0x23C0],
            i if (0x2400..=0x27BF).contains(&i) => &mut self.name1[i - 0x2400],
            i if (0x27C0..=0x27FF).contains(&i) => &mut self.attribute1[i - 0x27C0],
            i if (0x2800..=0x2BBF).contains(&i) => &mut self.name2[i - 0x2800],
            i if (0x2BC0..=0x2BFF).contains(&i) => &mut self.attribute2[i - 0x2BC0],
            i if (0x2C00..=0x2FBF).contains(&i) => &mut self.name3[i - 0x2C00],
            i if (0x2FC0..=0x2FFF).contains(&i) => &mut self.attribute3[i - 0x2FC0],
            i if (0x3000..=0x3EFF).contains(&i) => &mut self.mirror1[i - 0x3F00],
            i if (0x3F00..=0x3F0F).contains(&i) => &mut self.background_palette[i - 0x3F00],
            i if (0x3F10..=0x3F1F).contains(&i) => &mut self.sprite_palette[i - 0x3F10],
            i if (0x3F20..=0x3FFF).contains(&i) => &mut self.mirror2[i - 0x3F20],
            _ => unreachable!("{}", index),
        }
    }
}
