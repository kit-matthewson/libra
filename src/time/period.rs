#[derive(Clone, Copy, Debug)]
pub enum Period {
    Year(u32),
    Month(u32),
    Day(u32),
}

impl Period {
    pub fn whole_days(&self) -> u32 {
        match self {
            Period::Year(_) => todo!(),
            Period::Month(_) => todo!(),
            Period::Day(d) => *d,
        }
    }
}
