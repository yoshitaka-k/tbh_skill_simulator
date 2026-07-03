mod hero;
mod skill;

pub(crate) use hero::heros;
pub(crate) use skill::{knight, ranger, sorcerer, priest, hunter, slayer};

/// キャラクター情報を表す。
pub struct HeroData {
    pub image: egui::ImageSource<'static>,
    pub name: &'static str,
}

impl HeroData {
    /// 新しいキャラクター情報を作成する。
    pub const fn new(
        image: egui::ImageSource<'static>,
        name: &'static str,
    ) -> Self {
        Self { image, name }
    }
}

/// スキル情報を表す。
pub struct SkillData {
    pub image: egui::ImageSource<'static>,
    pub id: u32,
    pub group: &'static str,
    pub skill_type: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub effects: &'static [&'static str],
}

impl SkillData {
    /// 新しいスキル情報を作成する。
    pub const fn new(
        image: egui::ImageSource<'static>,
        id: u32,
        group: &'static str,
        skill_type: &'static str,
        name: &'static str,
        description: &'static str,
        effects: &'static [&'static str],
    ) -> Self {
        Self {
            image,
            id,
            group,
            skill_type,
            name,
            description,
            effects,
        }
    }
}
