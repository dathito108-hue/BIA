#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExpressionStyle {
    Brief,
    Standard,
    Deep,
}

impl ExpressionStyle {
    pub fn from_followup(explain: bool, brief: bool) -> Self {
        if brief { Self::Brief } else if explain { Self::Deep } else { Self::Standard }
    }
}
