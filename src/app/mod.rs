use crate::hero::Hero;
use crate::hero::data::heros::HERO_DATA;

#[derive(serde::Deserialize, serde::Serialize)]
pub enum CurrentHero {
    Knight,
    Ranger,
    Slayer,
    Sorcerer,
    Priest,
    Hunter,
}

/// Deserialize/Serialize を derive して、終了時にアプリの状態を保存できるようにする。
#[derive(serde::Deserialize, serde::Serialize)]
// 新しいフィールドを追加したとき、古い状態を復元する際にデフォルト値を使う
pub struct App {
    pub current_hero: CurrentHero,
    knight: Hero,
    ranger: Hero,
    slayer: Hero,
    sorcerer: Hero,
    priest: Hero,
    hunter: Hero,
}

/// App のデフォルト値を設定する。
impl App {
    pub fn new() -> Self {
        Self {
            current_hero: CurrentHero::Knight,
            knight: Hero::new(HERO_DATA[0].name),
            ranger: Hero::new(HERO_DATA[1].name),
            slayer: Hero::new(HERO_DATA[2].name),
            sorcerer: Hero::new(HERO_DATA[3].name),
            priest: Hero::new(HERO_DATA[4].name),
            hunter: Hero::new(HERO_DATA[5].name)
        }
    }

    /// 現在選択されているヒーローを返す。
    pub fn hero(&self) -> &Hero {
        match self.current_hero {
            CurrentHero::Knight => &self.knight,
            CurrentHero::Ranger => &self.ranger,
            CurrentHero::Slayer => &self.slayer,
            CurrentHero::Sorcerer => &self.sorcerer,
            CurrentHero::Priest => &self.priest,
            CurrentHero::Hunter => &self.hunter,
        }
    }

    /// 現在選択されているヒーローを返す。
    pub fn hero_mut(&mut self) -> &mut Hero {
        match self.current_hero {
            CurrentHero::Knight => &mut self.knight,
            CurrentHero::Ranger => &mut self.ranger,
            CurrentHero::Slayer => &mut self.slayer,
            CurrentHero::Sorcerer => &mut self.sorcerer,
            CurrentHero::Priest => &mut self.priest,
            CurrentHero::Hunter => &mut self.hunter,
        }
    }
}
