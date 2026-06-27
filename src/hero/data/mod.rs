mod hero;
mod skill;

pub(crate) use hero::heros;
pub(crate) use skill::{knight, ranger};

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
    pub name: &'static str,
    pub description: &'static str,
    pub skill_type: &'static str,
    pub max_level: u32,
}

impl SkillData {
    /// 新しいスキル情報を作成する。
    pub const fn new(
        image: egui::ImageSource<'static>,
        id: u32,
        name: &'static str,
        description: &'static str,
        skill_type: &'static str,
        max_level: u32,
    ) -> Self {
        Self { id, image, name, description, skill_type, max_level }
    }
}
