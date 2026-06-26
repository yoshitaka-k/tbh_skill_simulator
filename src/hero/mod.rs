pub(crate) mod data;
mod skill;

pub(crate) use skill::Skill;

/// 英雄の基底クラス
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Hero {
    pub name: String,
    pub level: u32,
    pub skill_points: u32,
    pub skills: Vec<Skill>,
}

impl Hero {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            level: 0,
            skill_points: 0,
            skills: Vec::new()
        }
    }
}
