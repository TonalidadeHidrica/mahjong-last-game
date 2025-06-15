use itertools::{Itertools, join};
use lazy_format::lazy_format;
use mahjong_last_game::{FinalGameState, GameConfig, GameEnd};

fn main() {
    let results = FinalGameState::new(
        [226, -218, 653, -661],
        [154, 349, 199, 298],
        [false; 4],
        0,
        0,
    )
    .calc(GameConfig::m_league());

    let winners_str = |ranks: [usize; 4]| {
        (0..4)
            .map(|i| lazy_format!(if ranks[i] < 2 => "{i}" else => " "))
            .join("")
    };
    for i in 0..4 {
        for j in 0..4 {
            let mut previous = None;
            for result in &results.agari[i][j] {
                let ranks = result.ranks();
                let winners_str = winners_str(ranks);
                if previous.as_ref() != Some(&winners_str) {
                    let label = lazy_format!(
                        if i != j => "{i}<-{j}"
                        else => "{i}   "
                    );
                    let self_win = lazy_format!(
                        if ranks[i] < 2 => "o"
                        else => " "
                    );
                    let sashikomi = lazy_format!(
                        if i != 3 && i != j && ranks[j] < 2 => "{j}"
                        else => " "
                    );
                    println!(
                        "{label} {} => {winners_str}   {self_win}   {sashikomi}  {:?} {:?}",
                        result.end,
                        result.scores.map(|x| x * 100),
                        result.points,
                    );
                    previous = Some(winners_str);
                }
            }
            println!();
        }
    }
    for result in &results.no_agari {
        let ranks = result.ranks();
        let winners_str = winners_str(ranks);
        let GameEnd::Ryukyoku(tempai) = result.end else {
            panic!()
        };
        let tempai_str = tempai.iter().map(|&x| if x { "o" } else { "_" }).join("");
        println!(
            "{tempai_str} => {winners_str}       {:?} {:?}",
            result.scores.map(|x| x * 100),
            result.points,
        );
    }
}
