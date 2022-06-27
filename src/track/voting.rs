use anyhow::Result;
use itertools::Itertools;

pub trait Voting {
    fn find_merge_candidates(&self, distances: Vec<(u64, Result<f32>)>) -> Vec<(u64, usize)>;
}

pub struct DefaultVoting {
    count: usize,
    max_distance: f32,
    min_votes: usize,
}

impl Voting for DefaultVoting {
    fn find_merge_candidates(&self, distances: Vec<(u64, Result<f32>)>) -> Vec<(u64, usize)> {
        let mut tracks: Vec<_> = distances
            .into_iter()
            .filter(|(_, e)| match e {
                Ok(e) => *e <= self.max_distance,
                _ => false,
            })
            .map(|(track, _)| track)
            .collect();
        tracks.sort();
        let mut counts = tracks
            .into_iter()
            .counts()
            .into_iter()
            .filter(|(_, count)| *count >= self.min_votes)
            .collect::<Vec<(_, _)>>();
        counts.sort_by(|(_, c1), (_, c2)| c2.partial_cmp(c1).unwrap());
        counts.truncate(self.count);
        counts
    }
}

#[cfg(test)]
mod tests {
    use crate::track::voting::{DefaultVoting, Voting};

    #[test]
    fn default_voting() {
        let v = DefaultVoting {
            count: 5,
            max_distance: 0.32,
            min_votes: 1,
        };

        let candidates = v.find_merge_candidates(vec![(1, Ok(0.2))]);
        assert_eq!(candidates, vec![(1, 1)]);

        let candidates = v.find_merge_candidates(vec![(1, Ok(0.2)), (1, Ok(0.3))]);
        assert_eq!(candidates, vec![(1, 2)]);

        let candidates = v.find_merge_candidates(vec![(1, Ok(0.2)), (1, Ok(0.4))]);
        assert_eq!(candidates, vec![(1, 1)]);

        let candidates = v.find_merge_candidates(vec![(1, Ok(0.2)), (2, Ok(0.2))]);
        assert_eq!(candidates, vec![(1, 1), (2, 1)]);

        let candidates = v.find_merge_candidates(vec![
            (1, Ok(0.2)),
            (2, Ok(0.21)),
            (3, Ok(0.22)),
            (4, Ok(0.23)),
            (5, Ok(0.24)),
            (6, Ok(0.25)),
        ]);
        assert_eq!(candidates.len(), 5);
    }
}