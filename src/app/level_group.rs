#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    serde::Deserialize,
    serde::Serialize,
)]
#[serde(rename_all = "lowercase")]
pub enum LevelGroup {
    Level0,
    Level10,
    Level20,
    Level30,
    Level40,
    Level50,
    Level60,
    Level70,
}

impl LevelGroup {
    pub fn threshold(&self) -> u32 {
        match self {
            LevelGroup::Level0 => 0,
            LevelGroup::Level10 => 10,
            LevelGroup::Level20 => 20,
            LevelGroup::Level30 => 30,
            LevelGroup::Level40 => 40,
            LevelGroup::Level50 => 50,
            LevelGroup::Level60 => 60,
            LevelGroup::Level70 => 70,
        }
    }

    pub fn from_string(string: &str) -> Option<LevelGroup> {
        match string {
            "level0" => Some(LevelGroup::Level0),
            "level10" => Some(LevelGroup::Level10),
            "level20" => Some(LevelGroup::Level20),
            "level30" => Some(LevelGroup::Level30),
            "level40" => Some(LevelGroup::Level40),
            "level50" => Some(LevelGroup::Level50),
            "level60" => Some(LevelGroup::Level60),
            "level70" => Some(LevelGroup::Level70),
            _ => panic!("invalid level group: {string}"),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            LevelGroup::Level0 => "level0".to_string(),
            LevelGroup::Level10 => "level10".to_string(),
            LevelGroup::Level20 => "level20".to_string(),
            LevelGroup::Level30 => "level30".to_string(),
            LevelGroup::Level40 => "level40".to_string(),
            LevelGroup::Level50 => "level50".to_string(),
            LevelGroup::Level60 => "level60".to_string(),
            LevelGroup::Level70 => "level70".to_string(),
        }
    }

    pub fn next(&self) -> Option<LevelGroup> {
        match self {
            LevelGroup::Level0 => Some(LevelGroup::Level10),
            LevelGroup::Level10 => Some(LevelGroup::Level20),
            LevelGroup::Level20 => Some(LevelGroup::Level30),
            LevelGroup::Level30 => Some(LevelGroup::Level40),
            LevelGroup::Level40 => Some(LevelGroup::Level50),
            LevelGroup::Level50 => Some(LevelGroup::Level60),
            LevelGroup::Level60 => Some(LevelGroup::Level70),
            LevelGroup::Level70 => Some(LevelGroup::Level70),
        }
    }

    pub fn previous(&self) -> Option<LevelGroup> {
        match self {
            LevelGroup::Level0 => Some(LevelGroup::Level0),
            LevelGroup::Level10 => Some(LevelGroup::Level0),
            LevelGroup::Level20 => Some(LevelGroup::Level10),
            LevelGroup::Level30 => Some(LevelGroup::Level20),
            LevelGroup::Level40 => Some(LevelGroup::Level30),
            LevelGroup::Level50 => Some(LevelGroup::Level40),
            LevelGroup::Level60 => Some(LevelGroup::Level50),
            LevelGroup::Level70 => Some(LevelGroup::Level60),
        }
    }
}
