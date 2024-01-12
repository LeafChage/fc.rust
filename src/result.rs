pub type Result<T> = anyhow::Result<T, anyhow::Error>;

pub mod e {
    pub fn readonly(i: usize) -> anyhow::Error {
        anyhow::anyhow!("readonly, {}", { i })
    }

    pub fn writeonly(i: usize) -> anyhow::Error {
        anyhow::anyhow!("writeonly, {}", { i })
    }

    pub fn index_out_of_range(i: usize, range: std::ops::Range<usize>) -> anyhow::Error {
        anyhow::anyhow!("index out of range, {}, ({} ~ {})", i, range.start, range.end)
    }
}

