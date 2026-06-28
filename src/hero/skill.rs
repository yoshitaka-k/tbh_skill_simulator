use std::borrow::Cow;

use crate::hero::data::SkillData;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Skill {
    #[serde(skip)]
    pub image: egui::ImageSource<'static>,

    pub id: u32,
    pub group: String,
    pub name: String,
    pub description: String,
    pub skill_type: String,
    pub active: bool,
    pub level: u32,
    pub max_level: u32,
}

impl Default for Skill {
    fn default() -> Self {
        Self {
            image: egui::ImageSource::Bytes {
                uri: Cow::Borrowed("bytes://default"),
                bytes: egui::load::Bytes::Static(&[]),
            },
            id: 0,
            group: String::new(),
            name: String::new(),
            description: String::new(),
            skill_type: String::new(),
            active: false,
            level: 0,
            max_level: 0,
        }
    }
}

impl Skill {
    pub fn new(skill_data: &SkillData) -> Self {
        let active = if skill_data.group == "level0" {
            true
        } else {
            false
        };

        Self {
            image: skill_data.image.clone(),
            id: skill_data.id,
            group: skill_data.group.to_string(),
            name: skill_data.name.to_string(),
            description: skill_data.description.to_string(),
            skill_type: skill_data.skill_type.to_string(),
            active: active,
            level: 0,
            max_level: skill_data.max_level,
        }
    }

    /// 永続化された状態を復元したあと、静的データから画像を再設定する。
    pub fn restore_image(&mut self, skill_data: &SkillData) {
        if self.id == skill_data.id {
            self.image = skill_data.image.clone();
        }
    }

    /// スキルレベルを増やす。
    pub fn increase_level(&mut self) {
        if self.level < self.max_level {
            self.level += 1;
        }
    }

    /// スキルレベルを減らす。
    pub fn decrease_level(&mut self) {
        if self.level > 0 {
            self.level -= 1;
        }
    }
}
