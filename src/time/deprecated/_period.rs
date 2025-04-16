#[derive(Clone, Copy, Debug)]
pub enum Period {
    Years(u16),
    Months(u16),
    Days(u32),
}

impl Period {
    pub fn whole_days(&self) -> u32 {
        match self {
            Period::Years(_) => todo!(),
            Period::Months(_) => todo!(),
            Period::Days(d) => *d,
        }
    }
}
