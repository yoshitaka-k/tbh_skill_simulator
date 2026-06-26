pub(crate) mod data;
mod skill;

pub(crate) use skill::Skill;

/// 英雄の基底クラス
#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Hero {
    pub name: String,
    pub level: u32,
    pub skill_points: u32,
    pub skills: Vec<Skill>,
}
