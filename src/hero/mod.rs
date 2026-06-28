mod skill;

pub(crate) mod data;
pub(crate) use skill::Skill;

use crate::hero::data::{HeroData, SkillData};
use std::collections::HashMap;

/// 英雄の基底クラス
#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Hero {
    pub name: String,
    pub level: u32,
    pub skill_points: u32,
    pub skill_list: Vec<Vec<Skill>>,
    pub skill_group_sum_list: HashMap<String, u32>,
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

        let mut skill_group_sum_list: HashMap<String, u32> = HashMap::new();
        for row in skills_list.iter() {
            for skill in row {
                let _ = skill_group_sum_list.entry(skill.group.clone()).or_insert(0);
            }
        }

        Self {
            name: hero_data.name.to_string(),
            level: 1,
            skill_points: 1,
            skill_list: skills_list,
            skill_group_sum_list: skill_group_sum_list,
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

    /// スキルグループの合計を増やす。
    pub fn increase_skill_group_sum(&mut self, group: &str) {
        if let Some(sum) = self.skill_group_sum_list.get_mut(group) {
            *sum += 1;
        }
    }

    /// スキルグループの合計を減らす。
    pub fn decrease_skill_group_sum(&mut self, group: &str) {
        if let Some(sum) = self.skill_group_sum_list.get_mut(group) {
            *sum -= 1;
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
