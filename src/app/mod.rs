pub mod current_hero;
pub mod level_group;

use crate::app::current_hero::CurrentHero;
use crate::hero::Hero;
use crate::hero::data::heros::HERO_DATA;
use crate::hero::data::{knight, ranger, sorcerer, priest, hunter, slayer};

/// Deserialize/Serialize を derive して、終了時にアプリの状態を保存できるようにする。
#[derive(serde::Deserialize, serde::Serialize)]
// 新しいフィールドを追加したとき、古い状態を復元する際にデフォルト値を使う
#[serde(default)]
pub struct App {
    pub current_hero: CurrentHero,
    knight: Hero,
    ranger: Hero,
    sorcerer: Hero,
    priest: Hero,
    hunter: Hero,
    slayer: Hero,

    #[serde(skip)]
    hover_skill_detail: Option<String>,
    #[serde(skip)]
    click_skill_detail: Option<String>,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

/// App のデフォルト値を設定する。
impl App {
    pub fn new() -> Self {
        Self {
            current_hero: CurrentHero::Knight,
            knight: Hero::new(&HERO_DATA[0], &knight::SKILL_DATA),
            ranger: Hero::new(&HERO_DATA[1], &ranger::SKILL_DATA),
            sorcerer: Hero::new(&HERO_DATA[2], &sorcerer::SKILL_DATA),
            priest: Hero::new(&HERO_DATA[3], &priest::SKILL_DATA),
            hunter: Hero::new(&HERO_DATA[4], &hunter::SKILL_DATA),
            slayer: Hero::new(&HERO_DATA[5], &slayer::SKILL_DATA),
            hover_skill_detail: None,
            click_skill_detail: None,
        }
    }

    /// 現在選択されているヒーローを返す。
    pub fn hero(&self) -> &Hero {
        match self.current_hero {
            CurrentHero::Knight => &self.knight,
            CurrentHero::Ranger => &self.ranger,
            CurrentHero::Sorcerer => &self.sorcerer,
            CurrentHero::Priest => &self.priest,
            CurrentHero::Hunter => &self.hunter,
            CurrentHero::Slayer => &self.slayer,
        }
    }

    /// 現在選択されているヒーローを返す。
    pub fn hero_mut(&mut self) -> &mut Hero {
        match self.current_hero {
            CurrentHero::Knight => &mut self.knight,
            CurrentHero::Ranger => &mut self.ranger,
            CurrentHero::Sorcerer => &mut self.sorcerer,
            CurrentHero::Priest => &mut self.priest,
            CurrentHero::Hunter => &mut self.hunter,
            CurrentHero::Slayer => &mut self.slayer,
        }
    }

    /// 永続化された状態を復元したあと、静的データからスキル画像を再設定する。
    pub fn restore_images(&mut self) {
        self.knight.restore_images(knight::SKILL_DATA);
        self.ranger.restore_images(ranger::SKILL_DATA);
        self.sorcerer.restore_images(sorcerer::SKILL_DATA);
        self.priest.restore_images(priest::SKILL_DATA);
        self.hunter.restore_images(hunter::SKILL_DATA);
        self.slayer.restore_images(slayer::SKILL_DATA);
    }

    /// ホバーされたスキルの詳細を設定する。
    pub fn set_hover_skill_detail(&mut self, text: Option<String>) {
        self.hover_skill_detail = text;
    }

    /// クリックされたスキルの詳細を設定する。
    pub fn set_click_skill_detail(&mut self, text: Option<String>) {
        self.click_skill_detail = text;
    }

    /// ホバーされたスキルの詳細を返す。
    pub fn hover_skill_detail(&self) -> Option<&String> {
        self.hover_skill_detail.as_ref()
    }

    /// クリックされたスキルの詳細を返す。
    pub fn click_skill_detail(&self) -> Option<&String> {
        self.click_skill_detail.as_ref()
    }
}
