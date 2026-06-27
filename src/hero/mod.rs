mod skill;

pub(crate) mod data;
pub(crate) use skill::Skill;

use crate::hero::data::{HeroData, SkillData};

/// 英雄の基底クラス
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Hero {
    pub name: String,
    pub level: u32,
    pub skill_points: u32,
    pub skills: Vec<Skill>,
}

impl Hero {
    pub fn new(hero_data: &HeroData, skills_data: &[&[SkillData]]) -> Self {
        // println!("skills_data: {:?}", skills_data);

        let mut skills: Vec<Skill> = Vec::new();

        for skill_data in skills_data {
            for skill in skill_data.iter() {
                skills.push(Skill::new(&skill));
            }
        }

        Self {
            name: hero_data.name.to_string(),
            level: 1,
            skill_points: 1,
            skills: skills,
        }
    }
}
