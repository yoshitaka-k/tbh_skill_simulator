mod skill;

pub(crate) mod data;
pub(crate) use skill::Skill;

use std::collections::BTreeMap;
use crate::hero::data::{HeroData, SkillData};
use crate::app::level_group::LevelGroup;

/// 英雄の基底クラス
#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Hero {
    pub name: String,
    pub level: u32,
    pub skill_points: u32,
    pub skill_list: BTreeMap<LevelGroup, Vec<Skill>>,
}

impl Hero {
    pub fn new(hero_data: &HeroData, skills_data: &[&[SkillData]]) -> Self {
        let mut skills_list: BTreeMap<LevelGroup, Vec<Skill>> = BTreeMap::new();

        for (i, skill_data) in skills_data.iter().enumerate() {
            let group = LevelGroup::from_string(&format!("level{}", i * 10)).unwrap();

            let mut skills: Vec<Skill> = Vec::new();
            for skill in skill_data.iter() {
                skills.push(Skill::new(&skill));
            }
            skills_list.entry(group).or_insert(skills);
        }

        Self {
            name: hero_data.name.to_string(),
            level: 1,
            skill_points: 1,
            skill_list: skills_list,
        }
    }

    /// スキルのレベルを取得する。
    pub fn skill_level(&self, group: &LevelGroup, index: usize) -> u32 {
        if let Some(skills) = self.skill_list.get(group) {
            skills[index].level
        } else {
            0
        }
    }

    /// スキルの最大レベルを取得する。
    pub fn skill_max_level(&self, group: &LevelGroup, index: usize) -> u32 {
        if let Some(skills) = self.skill_list.get(group) {
            skills[index].max_level
        } else {
            0
        }
    }

    /// スキルポイントを増やす。
    pub fn increase_skill_points(&mut self) -> bool{
        if self.skill_points < self.level {
            self.skill_points += 1;
            true
        } else {
            false
        }
    }

    /// スキルポイントを減らす。
    pub fn decrease_skill_points(&mut self) -> bool {
        if self.skill_points > 0 {
            self.skill_points -= 1;
            true
        } else {
            false
        }
    }

    /// スキルレベルを増やす。
    pub fn increase_skill_level(&mut self, group: &LevelGroup, index: usize) {
        if let Some(skills) = self.skill_list.get_mut(group) {
            skills[index].increase_level();
        }
    }

    /// スキルレベルを減らす。
    pub fn decrease_skill_level(&mut self, group: &LevelGroup, index: usize) {
        if let Some(skills) = self.skill_list.get_mut(group) {
            skills[index].decrease_level();
        }
    }

    /// 永続化された状態を復元したあと、静的データからスキル画像を再設定する。
    pub fn restore_images(&mut self, skills_data: &[&[SkillData]]) {
        for (_, skills) in &mut self.skill_list.iter_mut() {
            for skill in skills {
                for data in skills_data.iter().flat_map(|row| row.iter()) {
                    skill.restore_image(data);
                }
            }
        }
    }

    /// スキルがアクティブかどうかを更新する。
    pub fn update_active_skill(&mut self) {
        let mut level_sum = 0;
        for (_, skills) in &mut self.skill_list.iter_mut() {
            for skill in skills {
                skill.active = false;
                level_sum += skill.level;
            }
        }
        println!("cargo:warning=level_sum: {:?}", level_sum);
    }
}
