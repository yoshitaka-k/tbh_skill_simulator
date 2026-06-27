use crate::hero::Hero;
use crate::hero::data::heros::HERO_DATA;
use crate::hero::data::{knight, ranger};

#[derive(serde::Deserialize, serde::Serialize)]
pub enum CurrentHero {
    Knight,
    Ranger,
    Sorcerer,
    Priest,
    Hunter,
    Slayer,
}

/// Deserialize/Serialize を derive して、終了時にアプリの状態を保存できるようにする。
#[derive(serde::Deserialize, serde::Serialize)]
// 新しいフィールドを追加したとき、古い状態を復元する際にデフォルト値を使う
pub struct App {
    pub current_hero: CurrentHero,
    knight: Hero,
    ranger: Hero,
    sorcerer: Hero,
    priest: Hero,
    hunter: Hero,
    slayer: Hero,
}

/// App のデフォルト値を設定する。
impl App {
    pub fn new() -> Self {
        Self {
            current_hero: CurrentHero::Knight,
            knight: Hero::new(&HERO_DATA[0], &knight::SKILL_DATA),
            ranger: Hero::new(&HERO_DATA[1], &ranger::SKILL_DATA),
            sorcerer: Hero::new(&HERO_DATA[3], knight::SKILL_DATA),
            priest: Hero::new(&HERO_DATA[4], &knight::SKILL_DATA),
            hunter: Hero::new(&HERO_DATA[5], &knight::SKILL_DATA),
            slayer: Hero::new(&HERO_DATA[2], &knight::SKILL_DATA),
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
}
