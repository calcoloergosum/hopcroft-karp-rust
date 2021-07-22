use std::collections::LinkedList;
use std::iter::repeat;

pub struct HopcroftKarp {
    cur_d: Option<i32>,
    l2d: Vec<Option<i32>>,
    matching_l2r: Vec<Option<usize>>,
    matching_r2l: Vec<Option<usize>>,
}

impl HopcroftKarp {
    pub fn new(n: usize) -> Self {
        Self {
            cur_d: None,
            l2d: repeat(None).take(n).collect(),
            matching_l2r: repeat(Option::None).take(n).collect(),
            matching_r2l: repeat(Option::None).take(n).collect(),
        }
    }

    pub fn get_maximum_matching(&mut self, graph_l2r: &[Vec<usize>]) -> &Vec<Option<usize>> {
        self.run(graph_l2r);
        &self.matching_l2r
    }

    fn run(&mut self, graph_l2r: &[Vec<usize>]) -> usize {
        let mut matchings = 0;
        while self.bfs(&graph_l2r) {
            for l in 0..graph_l2r.len() {
                match self.matching_l2r[l] {
                    Some(_) => continue,
                    None => {
                        if self.dfs(graph_l2r, l) {
                            matchings += 1
                        }
                    }
                }
            }
        }
        matchings
    }

    fn bfs(&mut self, graph_l2r: &[Vec<usize>]) -> bool {
        let mut vertex_queue = LinkedList::new();
        for l in 0..graph_l2r.len() {
            match self.matching_l2r[l] {
                None => {
                    vertex_queue.push_back(l);
                    self.l2d[l] = Some(0);
                }
                Some(_) => {
                    self.l2d[l] = None;
                }
            }
        }
        self.cur_d = None;
        while let Some(l) = vertex_queue.pop_front() {
            match (self.l2d[l], self.cur_d) {
                (None, None) => continue,
                (Some(d), Some(_d)) => {
                    if d >= _d {
                        continue;
                    }
                }
                _ => {}
            }
            let d_new = self.l2d[l].unwrap_or(-1) + 1;
            for r in graph_l2r[l].iter() {
                match self.matching_r2l[*r] {
                    None => {
                        if self.cur_d.is_none() {
                            self.cur_d = Some(d_new);
                        }
                    }
                    Some(l2) => {
                        if self.l2d[l2].is_none() {
                            self.l2d[l2] = Some(d_new);
                            vertex_queue.push_back(l2);
                        }
                    }
                }
            }
        }
        self.cur_d.is_some()
    }

    fn dfs(&mut self, graph_l2r: &[Vec<usize>], l: usize) -> bool {
        for r in graph_l2r[l].iter() {
            match self.matching_r2l[*r] {
                None => {
                    if self.cur_d == Some(self.l2d[l].unwrap_or(-1) + 1) {
                        self.update_match(l, *r);
                        return true;
                    }
                }
                Some(l2) => {
                    if self.l2d[l2] == Some(self.l2d[l].unwrap_or(-1) + 1)
                        && self.dfs(graph_l2r, l2)
                    {
                        self.update_match(l, *r);
                        return true;
                    }
                }
            }
        }
        self.l2d[l] = None;
        false
    }

    fn update_match(&mut self, l: usize, r: usize) {
        self.matching_l2r[l] = Some(r);
        self.matching_r2l[r] = Some(l);
    }
}

#[cfg(test)]
mod tests {
    use super::HopcroftKarp;
    #[test]
    fn testcase1() {
        let mut hopkarp = HopcroftKarp::new(2);
        let graph = vec![vec![0, 1], vec![0]];
        let expected: Vec<Option<usize>> = vec![Some(1), Some(0)];
        assert_eq!(&expected, hopkarp.get_maximum_matching(&graph));
    }
    #[test]
    fn testcase2() {
        let mut hopkarp = HopcroftKarp::new(5);
        let graph = vec![vec![0, 1], vec![0, 4], vec![2, 3], vec![0, 4], vec![0, 3]];
        let expected: Vec<Option<usize>> = vec![Some(1), Some(4), Some(2), Some(0), Some(3)];
        assert_eq!(&expected, hopkarp.get_maximum_matching(&graph));
    }
    #[test]
    fn testcase3() {
        let mut hopkarp = HopcroftKarp::new(7);
        let graph = vec![
            vec![0, 1],
            vec![1, 2],
            vec![1],
            vec![2, 3, 4, 5],
            vec![3, 6],
            vec![6],
            vec![6],
        ];
        let expected: Vec<Option<usize>> =
            vec![Some(0), Some(2), Some(1), Some(4), Some(3), Some(6), None];
        assert_eq!(expected, *hopkarp.get_maximum_matching(&graph));
    }
    #[test]
    fn testcase4() {
        let mut hopkarp = HopcroftKarp::new(5);
        let graph = vec![vec![0, 2], vec![0, 2], vec![2, 1], vec![4]];
        let expected: Vec<Option<usize>> = vec![Some(0), Some(2), Some(1), Some(4), None];
        assert_eq!(expected, *hopkarp.get_maximum_matching(&graph));
    }
    #[test]
    fn testcase5() {
        let mut hopkarp = HopcroftKarp::new(8);
        let graph = vec![
            vec![2, 3],
            vec![2, 3],
            vec![2],
            vec![0, 4, 6],
            vec![0, 1, 6],
            vec![1, 7],
            vec![5],
            vec![1, 3, 7],
        ];
        let expected: Vec<Option<usize>> = vec![
            Some(2),
            Some(3),
            None,
            Some(0),
            Some(6),
            Some(7),
            Some(5),
            Some(1),
        ];
        assert_eq!(expected, *hopkarp.get_maximum_matching(&graph));
    }
}
