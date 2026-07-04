use std::borrow::Cow;
use getset::{Setters, Getters};

use crate::hero::data::SkillData;
use crate::app::level_group::LevelGroup;

#[derive(Clone, Setters, Getters, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Skill {
    #[serde(skip)]
    #[getset(get = "pub")]
    image: egui::ImageSource<'static>,

    #[getset(get = "pub")]
    id: u32,
    group: LevelGroup,
    skill_type: String,
    #[getset(get = "pub")]
    name: String,
    description: String,
    effects: Vec<String>,
    #[getset(set = "pub", get = "pub")]
    active: bool,
    #[getset(get = "pub")]
    level: u32,
}

impl Default for Skill {
    fn default() -> Self {
        Self {
            image: egui::ImageSource::Bytes {
                uri: Cow::Borrowed("bytes://default"),
                bytes: egui::load::Bytes::Static(&[]),
            },
            id: 0,
            group: LevelGroup::Level0,
            skill_type: String::new(),
            name: String::new(),
            description: String::new(),
            effects: Vec::new(),
            active: false,
            level: 0,
        }
    }
}

impl Skill {
    pub fn new(skill_data: &SkillData) -> Self {
        let group = LevelGroup::from_string(skill_data.group).unwrap();
        let active = if group == LevelGroup::Level0 {
            true
        } else {
            false
        };

        Self {
            image: skill_data.image.clone(),
            id: skill_data.id,
            group: group,
            skill_type: skill_data.skill_type.to_string(),
            name: skill_data.name.to_string(),
            description: skill_data.description.to_string(),
            effects: skill_data.effects.iter().map(|e| e.to_string()).collect(),
            active: active,
            level: 0,
        }
    }

    /// スキルの最大レベルを取得する。
    pub fn max_level(&self) -> u32 {
        self.effects.len() as u32
    }

    /// 永続化された状態を復元したあと、静的データから画像を再設定する。
    pub fn restore_image(&mut self, skill_data: &SkillData) {
        if self.id == skill_data.id {
            self.image = skill_data.image.clone();
        }
    }

    /// スキルレベルを増やす。
    pub fn increase_level(&mut self) {
        if self.level < self.effects.len() as u32 {
            self.level += 1;
        }
    }

    /// スキルレベルを減らす。
    pub fn decrease_level(&mut self) {
        if self.level > 0 {
            self.level -= 1;
        }
    }

    /// スキルの説明を表示用文字列に変換する。
    pub fn description_display(&self) -> String {
        if self.level > 0 {
            self.description.replace("{effect}", &self.effects[self.level as usize - 1])
        } else {
            let level_min = &self.effects[0];
            let level_max = &self.effects[self.effects.len() - 1];

            self.description.replace("{effect}", &format!("[{} : {}]", level_min, level_max))
        }
    }

    /// スキルの効果を表示用文字列に変換する。
    pub fn effects_display(&self) -> String {
        self.effects
            .iter()
            .enumerate()
            .map(|(i, effect)| format!("lv.{} {}", i + 1, effect))
            .collect::<Vec<_>>()
            .join(" / ")
    }
}
