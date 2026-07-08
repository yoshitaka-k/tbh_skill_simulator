/// 埋め込み効果音データ。
pub struct SoundData {
    pub bytes: &'static [u8],
}

impl SoundData {
    /// 新しい効果音データを作成する。
    pub const fn new(bytes: &'static [u8]) -> Self {
        Self { bytes }
    }
}

include!(concat!(env!("OUT_DIR"), "/sounds_generated.rs"));
