pub mod current_hero;

use getset::{Setters, Getters};
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, MixerDeviceSink, Player};

use crate::app::current_hero::CurrentHero;
use crate::hero::{Hero, Skill};
use crate::hero::data::heros::HERO_DATA;
use crate::hero::data::{knight, ranger, sorcerer, priest, hunter, slayer};

const LEFT_CLICK_SOUND_PATH: &str = "assets/sounds/left_click.mp3";
const RIGHT_CLICK_SOUND_PATH: &str = "assets/sounds/right_click.mp3";
const HOVER_SOUND_PATH: &str = "assets/sounds/hover.mp3";
const BEEP_SOUND_PATH: &str = "assets/sounds/beep.mp3";

/// Deserialize/Serialize を derive して、終了時にアプリの状態を保存できるようにする。
#[derive(Setters, Getters, serde::Deserialize, serde::Serialize)]
// 新しいフィールドを追加したとき、古い状態を復元する際にデフォルト値を使う
#[serde(default)]
pub struct App {
    #[getset(get = "pub", set = "pub")]
    current_hero: CurrentHero,

    knight: Hero,
    ranger: Hero,
    sorcerer: Hero,
    priest: Hero,
    hunter: Hero,
    slayer: Hero,

    #[serde(skip)]
    hover_skill: Option<Skill>,
    #[serde(skip)]
    click_skill: Option<Skill>,

    #[serde(skip)]
    _stream: MixerDeviceSink,
    #[serde(skip)]
    player: Player,

}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

/// App のデフォルト値を設定する。
impl App {
    pub fn new() -> Self {
        let handle = rodio::DeviceSinkBuilder::open_default_sink().unwrap();
        let player = rodio::Player::connect_new(&handle.mixer());

        Self {
            current_hero: CurrentHero::Knight,
            knight: Hero::new(&HERO_DATA[0], &knight::SKILL_DATA),
            ranger: Hero::new(&HERO_DATA[1], &ranger::SKILL_DATA),
            sorcerer: Hero::new(&HERO_DATA[2], &sorcerer::SKILL_DATA),
            priest: Hero::new(&HERO_DATA[3], &priest::SKILL_DATA),
            hunter: Hero::new(&HERO_DATA[4], &hunter::SKILL_DATA),
            slayer: Hero::new(&HERO_DATA[5], &slayer::SKILL_DATA),
            hover_skill: None,
            click_skill: None,

            _stream: handle,
            player,
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
    pub fn set_hover_skill(&mut self, skill: Option<Skill>) {
        self.hover_skill = skill;
    }

    /// クリックされたスキルの詳細を設定する。
    pub fn set_click_skill(&mut self, skill: Option<Skill>) {
        self.click_skill = skill;
    }

    /// ホバーされたスキルの詳細を返す。
    pub fn hover_skill(&self) -> Option<&Skill> {
        self.hover_skill.as_ref()
    }

    /// クリックされたスキルの詳細を返す。
    pub fn click_skill(&self) -> Option<&Skill> {
        self.click_skill.as_ref()
    }

    /// 左クリック音を再生する。
    pub fn play_left_click_sound(&self) {
        self.player.stop();
        if let Ok(file) = File::open(LEFT_CLICK_SOUND_PATH) {
            if let Ok(source) = Decoder::try_from(BufReader::new(file)) {
                self.player.append(source);
            }
        }
    }

    /// 右クリック音を再生する。
    pub fn play_right_click_sound(&self) {
        self.player.stop();
        if let Ok(file) = File::open(RIGHT_CLICK_SOUND_PATH) {
            if let Ok(source) = Decoder::try_from(BufReader::new(file)) {
                self.player.append(source);
            }
        }
    }

    /// ホバー音を再生する。
    pub fn play_hover_sound(&self) {
        self.player.stop();
        if let Ok(file) = File::open(HOVER_SOUND_PATH) {
            if let Ok(source) = Decoder::try_from(BufReader::new(file)) {
                self.player.append(source);
            }
        }
    }

    /// ビープ音を再生する。
    pub fn play_beep_sound(&self) {
        self.player.stop();
        if let Ok(file) = File::open(BEEP_SOUND_PATH) {
            if let Ok(source) = Decoder::try_from(BufReader::new(file)) {
                self.player.append(source);
            }
        }
    }
}
