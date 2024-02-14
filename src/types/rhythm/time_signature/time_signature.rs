pub struct TimeSignature {
    name: &'static str,
    signature: &'static str,
    beats: u8,
    note: u8,
}

pub enum TimeSignatures {
    ShuffleTime,
    CommonTime,
    CutTime,
    TwoTwo,
    TwoFour,
    ThreeFour,
    FourFour,
}

impl Default for TimeSignatures {
    fn default() -> Self {
        TimeSignatures::CommonTime
    }
}

impl TimeSignatures {
    pub fn beats(&self) -> u8 {
        match self {
            TimeSignatures::ShuffleTime | &TimeSignatures::ThreeFour => 3,
            TimeSignatures::CommonTime | &TimeSignatures::FourFour => 4,
            TimeSignatures::CutTime | TimeSignatures::TwoTwo | &TimeSignatures::TwoFour => 2,
        }
    }

    pub fn note(&self) -> u8 {
        match self {
            TimeSignatures::ShuffleTime => 8,
            TimeSignatures::CommonTime | TimeSignatures::CutTime | TimeSignatures::TwoTwo => 4,
            TimeSignatures::TwoFour | TimeSignatures::ThreeFour | TimeSignatures::FourFour => 4,
        }
    }

    pub fn note_name(&self) -> &'static str {
        match self {
            TimeSignatures::ShuffleTime => "8th",
            TimeSignatures::CommonTime | TimeSignatures::CutTime | TimeSignatures::TwoTwo => "4th",
            TimeSignatures::TwoFour | TimeSignatures::ThreeFour | TimeSignatures::FourFour => "4th",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            TimeSignatures::ShuffleTime => "Shuffle Time",
            TimeSignatures::CommonTime => "Common Time",
            TimeSignatures::CutTime => "Cut Time",
            TimeSignatures::TwoTwo => "Two-Two",
            TimeSignatures::TwoFour => "Two-Four",
            TimeSignatures::ThreeFour => "Three-Four",
            TimeSignatures::FourFour => "Four-Four",
        }
    }
}
