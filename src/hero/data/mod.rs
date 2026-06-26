mod hero;
mod skill;

pub(crate) use hero::heros;
pub(crate) use skill::{knight, ranger};

/// キャラクター情報を表す。
pub(crate) struct HeroData {
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
pub(crate) struct SkillData {
    pub image: egui::ImageSource<'static>,
    pub name: &'static str,
    pub max_level: u8,
}

impl SkillData {
    /// 新しいスキル情報を作成する。
    pub const fn new(
        image: egui::ImageSource<'static>,
        name: &'static str,
        max_level: u8,
    ) -> Self {
        Self { image, name, max_level }
    }
}
