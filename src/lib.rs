#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum BadgeType {
    Uber = 0,
    Other = 1,
    Community = 2,
    Village = 3,
    CtfContest = 4,
    Human = 5,
    Goon = 6,
    None = 7,
}
impl TryFrom<u8> for BadgeType {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(BadgeType::Uber),
            1 => Ok(BadgeType::Other),
            2 => Ok(BadgeType::Community),
            3 => Ok(BadgeType::Village),
            4 => Ok(BadgeType::CtfContest),
            5 => Ok(BadgeType::Human),
            6 => Ok(BadgeType::Goon),
            7 => Ok(BadgeType::None),
            n => Err(n),
        }
    }
}
impl BadgeType {
    pub fn hue_range(&self) -> std::ops::Range<u8> {
        match self {
            Self::Goon => 0..20,
            Self::Community => 32..80,
            Self::Village => 80..128,
            Self::Human => 128..160,
            Self::Other => 160..192,
            Self::CtfContest => 192..220,
            Self::Uber => 220..255,
            Self::None => 128..160,
        }
    }
    pub fn sat_range(&self) -> std::ops::Range<u8> {
        match self {
            Self::Goon => 160..255,
            Self::Community => 32..160,
            Self::Village => 32..160,
            Self::Human => 32..255,
            Self::Other => 16..255,
            Self::CtfContest => 16..255,
            Self::Uber => 16..160,
            Self::None => 32..255,
        }
    }
    pub fn chaser_range(&self) -> std::ops::Range<u8> {
        match self {
            Self::Goon => 90..255,
            Self::Community => 90..255,
            Self::Village => 90..255,
            Self::Human => 90..255,
            Self::Other => 0..255,
            Self::CtfContest => 90..255,
            Self::Uber => 0..45,
            Self::None => 90..255,
        }
    }
    pub fn nonlin_range(&self) -> std::ops::Range<u8> {
        match self {
            Self::Goon => 0..255,
            Self::Community => 0..255,
            Self::Village => 0..255,
            Self::Human => 0..255,
            Self::Other => 0..90,
            Self::CtfContest => 0..90,
            Self::Uber => 0..44,
            Self::None => 0..255,
        }
    }
    pub fn cd_dir_range(&self) -> std::ops::Range<u8> {
        match self {
            Self::Goon => 0..255,
            Self::Community => 0..255,
            Self::Village => 0..45,
            Self::Human => 0..255,
            Self::Other => 0..255,
            Self::CtfContest => 0..255,
            Self::Uber => 0..45,
            Self::None => 0..255,
        }
    }
    pub fn cd_period_max(&self) -> u8 {
        match self {
            Self::Goon => 4,
            Self::Community => 2,
            Self::Village => 4,
            Self::Human => 4,
            Self::Other => 6,
            Self::CtfContest => 6,
            Self::Uber => 2,
            Self::None => 4,
        }
    }
}

pub const LED_SERVER: &'static str = "_dc34_led_";
#[derive(Debug, Copy, Clone, num_derive::FromPrimitive, num_derive::ToPrimitive)]
pub enum LedManagerOp {
    Autogamy,
    Force,
    GeneInit,
    Syngamy,
    Invalid,
}
