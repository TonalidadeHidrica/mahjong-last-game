use itertools::Itertools;
use num_traits::Euclid;

pub mod agari_scores;

#[derive(Clone, Copy)]
pub struct GameConfig {
    rank_scores: [i32; 4],
}
impl GameConfig {
    pub fn new(rank_scores: [i32; 4]) -> Self {
        assert!(rank_scores.iter().sum::<i32>() == -1000);
        Self { rank_scores }
    }
    pub fn m_league() -> Self {
        GameConfig::new([200, -200, -400, -600])
    }
    pub fn to_points(self, scores: [i32; 4], deposit: i32) -> [i32; 4] {
        assert!(deposit >= 0);
        assert!(scores.iter().sum::<i32>() + deposit * 10 == 1000);

        let scores_sorted = scores.into_iter().sorted().rev().collect_vec();
        let mut points = scores;
        for &score in scores_sorted.iter().dedup() {
            let ranks = (0..4).filter(|&i| scores_sorted[i] == score).collect_vec();
            let people = (0..4).filter(|&i| scores[i] == score).collect_vec();
            let count = people.len() as i32;
            let mut distribute = |point: i32| {
                let (div, rem) = point.div_rem_euclid(&count);
                for &person in &people {
                    points[person] += div;
                }
                points[people[0]] += rem;
            };
            distribute(ranks.iter().map(|&i| self.rank_scores[i]).sum());
            if ranks[0] == 0 {
                distribute(deposit * 10);
            }
        }

        assert!(points.iter().sum::<i32>() == 0);
        points
    }
}

#[derive(Clone, Copy)]
pub struct FinalGameState {
    points: [i32; 4],
    scores: [i32; 4],
    stack: i32,
    deposit: i32,
}
impl FinalGameState {
    pub fn new(points: [i32; 4], scores: [i32; 4], stack: i32, deposit: i32) -> Self {
        assert!(points.iter().sum::<i32>() == 0);
        assert!(scores.iter().sum::<i32>() + deposit * 10 == 1000);
        Self {
            points,
            scores,
            stack,
            deposit,
        }
    }

    pub fn calc(self, config: GameConfig) {
        for i in 0..4 {
            for j in 0..4 {}
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::GameConfig;

    #[test]
    fn test_to_points() {
        let config = GameConfig::new([200, -200, -400, -600]);
        assert_eq!(
            config.to_points([453, 426, 182, -61], 0),
            [653, 226, -218, -661],
        );
        assert_eq!(
            config.to_points([383, 274, 184, 159], 0),
            [583, 74, -216, -441],
        );
        assert_eq!(
            config.to_points([237, 263, 263, 237], 0),
            // +0, +0, -500, -500
            [-263, 263, 263, -263],
        );
        assert_eq!(
            config.to_points([263, 263, 232, 232], 1),
            // +0, +0, -500, -500
            // +5, +5, 0, 0
            [268, 268, -268, -268],
        );
        assert_eq!(
            config.to_points([229, 235, 263, 263], 1),
            // +0, +0, -400, -600
            // +5, +5, 0, 0
            [229 - 600, 235 - 400, 268, 268],
        );
        assert_eq!(
            config.to_points([257, 219, 257, 257], 1),
            // -132, -134, -134, -600
            // +4, +3, +3, 0
            [257 - 132 + 4, 219 - 600, 257 - 134 + 3, 257 - 134 + 3],
        );
        assert_eq!(
            config.to_points([209, 257, 257, 257], 2),
            // +8, +6, +6, 0
            [209 - 600, 257 - 132 + 8, 257 - 134 + 6, 257 - 134 + 6],
        );
        assert_eq!(
            config.to_points([257, 257, 257, 199], 3),
            // +10, +10, +10, 0
            [257 - 132 + 10, 257 - 134 + 10, 257 - 134 + 10, 199 - 600],
        );
        assert_eq!(config.to_points([250, 250, 250, 250], 0), [0, 0, 0, 0]);
        assert_eq!(config.to_points([245, 245, 245, 245], 2), [0, 0, 0, 0]);
    }
}
