#[derive(Default, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub enum CurrentHero {
    #[default]
    Knight,
    Ranger,
    Sorcerer,
    Priest,
    Hunter,
    Slayer,
}

impl CurrentHero {
    pub fn to_string(&self) -> String {
        match self {
            CurrentHero::Knight => "knight".to_string(),
            CurrentHero::Ranger => "ranger".to_string(),
            CurrentHero::Sorcerer => "sorcerer".to_string(),
            CurrentHero::Priest => "priest".to_string(),
            CurrentHero::Hunter => "hunter".to_string(),
            CurrentHero::Slayer => "slayer".to_string(),
        }
    }
}
