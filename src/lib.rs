use itertools::Itertools;

pub fn tsumo_han_fu() -> impl Iterator<Item = (u32, i32)> {
    let mut ret = vec![];
    for fu in (30..=100).step_by(10) {
        ret.push((1, fu));
    }
    for fu in (20..=110).step_by(10) {
        ret.push((2, fu));
    }
    for fu in (20..=50).step_by(10) {
        ret.push((3, fu));
    }
    ret.push((3, 25));
    ret.push((4, 20));
    ret.push((4, 25));
    ret.into_iter()
}
pub fn rong_han_fu() -> impl Iterator<Item = (u32, i32)> {
    let mut ret = vec![];
    for fu in (30..=100).step_by(10) {
        ret.push((1, fu));
    }
    for fu in (30..=110).step_by(10) {
        ret.push((2, fu));
    }
    for fu in (30..=50).step_by(10) {
        ret.push((3, fu));
    }
    for han in 2..=4 {
        ret.push((han, 25));
    }
    ret.into_iter()
}
pub fn to_base_score((han, fu): (u32, i32)) -> i32 {
    2i32.pow(han + 2) * fu
}

pub fn mangan_over_base_scores() -> impl Iterator<Item = i32> {
    [2, 3, 4, 6, 8, 16, 24, 32].into_iter().map(|x| x * 1000)
}
pub fn tsumo_base_scores() -> impl Iterator<Item = i32> {
    tsumo_han_fu()
        .map(to_base_score)
        .chain(mangan_over_base_scores())
}
pub fn rong_base_scores() -> impl Iterator<Item = i32> {
    rong_han_fu()
        .map(to_base_score)
        .chain(mangan_over_base_scores())
}

pub fn round(x: i32) -> i32 {
    (x + 99) / 100 * 100
}
pub fn child_tsumo() -> impl Iterator<Item = (i32, i32)> {
    tsumo_base_scores().map(|base| (round(base), round(base * 2))).sorted().dedup()
}
pub fn child_rong() -> impl Iterator<Item = i32> {
    rong_base_scores().map(|base| round(base * 4)).sorted().dedup()
}
pub fn parent_tsumo() -> impl Iterator<Item = i32> {
    tsumo_base_scores().map(|base| round(base * 2)).sorted().dedup()
}
pub fn parent_rong() -> impl Iterator<Item = i32> {
    rong_base_scores().map(|base| round(base * 6)).sorted().dedup()
}
