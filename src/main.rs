use mahjong_last_game::{
    FinalGameState, GameConfig,
    agari_scores::{child_rong, child_tsumo, parent_rong, parent_tsumo},
};

fn main() {
    FinalGameState::new([226, -218, 653, -661], [154, 349, 199, 298], 0, 0)
        .calc(GameConfig::m_league());
}
