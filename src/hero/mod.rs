pub mod level_group;
mod skill;

use getset::{Setters, Getters};

pub(crate) mod data;
pub(crate) use skill::Skill;

use std::collections::BTreeMap;
use crate::hero::data::{HeroData, SkillData};
use crate::hero::level_group::LevelGroup;

/// 英雄の基底クラス
#[derive(Default, Setters, Getters, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Hero {
    #[getset(get = "pub")]
    name: String,
    #[getset(set = "pub", get = "pub")]
    level: u32,
    #[getset(set = "pub", get = "pub")]
    skill_points: u32,
    #[getset(set = "pub", get = "pub")]
    skill_list: BTreeMap<LevelGroup, Vec<Skill>>,
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

    /// グループに属するスキル一覧を取得する。
    pub fn skill_list_by_group(&self, group: &LevelGroup) -> &Vec<Skill> {
        self.skill_list().get(group).unwrap()
    }

    /// レベルを増やす。
    pub fn increase_level(&mut self) {
        self.level += 1;
    }

    /// レベルを減らす。
    pub fn decrease_level(&mut self) {
        self.level -= 1;
    }

    /// スキルのレベル合計を取得する。
    pub fn skill_level_sum(&self) -> u32 {
        self.skill_list
            .values()
            .flat_map(|skills| skills.iter())
            .map(|skill| skill.level())
            .sum()
    }

    /// スキルを取得する。
    pub fn skill(&self, group: &LevelGroup, index: usize) -> Option<&Skill> {
        if let Some(skills) = self.skill_list.get(group) {
            Some(&skills[index])
        } else {
            None
        }
    }

    /// スキルのレベルを取得する。
    pub fn skill_level(&self, group: &LevelGroup, index: usize) -> u32 {
        if let Some(skills) = self.skill_list.get(group) {
            *skills[index].level()
        } else {
            0
        }
    }

    /// スキルの最大レベルを取得する。
    pub fn skill_max_level(&self, group: &LevelGroup, index: usize) -> u32 {
        if let Some(skills) = self.skill_list.get(group) {
            skills[index].max_level()
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
    pub fn decrease_skill_level(&mut self, group: &LevelGroup, index: usize) -> bool {
        if let Some(next_group) = group.next() {
            // 現在のグループのレベル合計が10未満で、次のグループのレベル合計が0以上の場合は、スキルレベルを減らせない。
            if self.skill_group_level_sum(group) <= 10 && self.skill_group_level_sum(&next_group) > 0 {
                return false;
            }
        }

        if let Some(skills) = self.skill_list.get_mut(group) {
            skills[index].decrease_level();
            return true;
        }
        return false;
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

    /// スキルグループのレベル合計を取得する。
    pub fn skill_group_level_sum(&self, group: &LevelGroup) -> u32 {
        let mut level_sum = 0;
        if let Some(skills) = self.skill_list.get(group) {
            for skill in skills {
                level_sum += skill.level();
            }
        }
        level_sum
    }

    /// スキルがアクティブかどうかを更新する。
    pub fn update_active_skill(&mut self) {
        let level_sum: u32 = self.skill_list
            .values()
            .flat_map(|skills| skills.iter())
            .map(|skill| skill.level())
            .sum();

        println!("level_sum: {:?}", level_sum);

        for (group, skills) in &mut self.skill_list.iter_mut() {
            let active = level_sum >= group.threshold();
            for skill in skills {
                skill.set_active(active);
            }
        }
    }
}
