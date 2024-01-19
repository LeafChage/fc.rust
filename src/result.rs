pub type Result<T> = anyhow::Result<T, anyhow::Error>;

pub mod e {
    pub fn readonly(i: usize) -> anyhow::Error {
        anyhow::anyhow!("readonly, {}(0x{:02X?})", i, i)
    }

    pub fn writeonly(i: usize) -> anyhow::Error {
        anyhow::anyhow!("writeonly, {}(0x{:02X?})", i, i)
    }

    pub fn index_out_of_range(i: usize) -> anyhow::Error {
        anyhow::anyhow!("index out of range, {}(0x{:02X?})", i, i)
    }

    pub fn unimplemented() -> anyhow::Error {
        anyhow::anyhow!("unimplemented")
    }
}
