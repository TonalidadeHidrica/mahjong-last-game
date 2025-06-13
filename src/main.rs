use mahjong_last_game::{child_rong, child_tsumo, parent_rong, parent_tsumo};

fn main() {
    child_tsumo().for_each(|x| println!("{x:?}"));
    child_rong().for_each(|x| println!("{x:?}"));
    parent_tsumo().for_each(|x| println!("{x:?}"));
    parent_rong().for_each(|x| println!("{x:?}"));
}
