pub struct MemoryMap {
    memory: [u8; 0x3FFF],
}

impl Default for MemoryMap {
    fn default() -> Self {
        MemoryMap {
            memory: [0; 0x3fff],
        }
    }
}
