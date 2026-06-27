mod skill;

pub(crate) mod data;
pub(crate) use skill::Skill;

use crate::hero::data::{HeroData, SkillData};

/// 英雄の基底クラス
#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Hero {
    pub name: String,
    pub level: u32,
    pub skill_points: u32,
    pub skill_list: Vec<Vec<Skill>>,
}

impl Hero {
    pub fn new(hero_data: &HeroData, skills_data: &[&[SkillData]]) -> Self {
        let mut skills_list: Vec<Vec<Skill>> = Vec::new();

        for skill_data in skills_data {
            let mut skills: Vec<Skill> = Vec::new();
            for skill in skill_data.iter() {
                skills.push(Skill::new(&skill));
            }
            skills_list.push(skills);
        }

        Self {
            name: hero_data.name.to_string(),
            level: 1,
            skill_points: 1,
            skill_list: skills_list,
        }
    }

    pub fn increase_skill_points(&mut self) -> bool{
        if self.skill_points < self.level {
            self.skill_points += 1;
            true
        } else {
            false
        }
    }

    pub fn decrease_skill_points(&mut self) -> bool {
        if self.skill_points > 0 {
            self.skill_points -= 1;
            true
        } else {
            false
        }
    }

    /// 永続化された状態を復元したあと、静的データからスキル画像を再設定する。
    pub fn restore_images(&mut self, skills_data: &[&[SkillData]]) {
        for skills in &mut self.skill_list {
            for skill in skills {
                for data in skills_data.iter().flat_map(|row| row.iter()) {
                    skill.restore_image(data);
                }
            }
        }
    }
}
