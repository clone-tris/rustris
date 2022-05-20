pub struct Score {
    pub lines_cleared: u32,
    pub total: u32,
    pub level: u32,
}

impl Score {
    pub fn new() -> Score {
        Score {
            level: 1,
            total: 0,
            lines_cleared: 0,
        }
    }
}
