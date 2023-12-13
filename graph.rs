use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap};
use std::fmt;
pub struct Graph(BTreeMap<String, BTreeMap<String, u32>>);

impl Graph {
    pub fn new() -> Self {
        Graph(BTreeMap::new())
    }

    pub fn add_edge(&mut self, label1: &str, label2: &str) {
        if label1 == label2 {
            return;
        }

        match self.0.get_mut(label1) {
            None => {
                let mut edges = BTreeMap::new();
                edges.insert(label2.to_string(), 1);
                self.0.insert(label1.to_string(), edges);
            }
            Some(v) => {
                v.entry(label2.to_string())
                    .and_modify(|v| *v += 1)
                    .or_insert(1);
            }
        };

        match self.0.get_mut(label2) {
            None => {
                let mut edges = BTreeMap::new();
                edges.insert(label1.to_string(), 1);
                self.0.insert(label2.to_string(), edges);
            }

            Some(v) => {
                v.entry(label1.to_string())
                    .and_modify(|v| *v += 1)
                    .or_insert(1);
            }
        }
    }

    pub fn dijkstra(
        &self,
        label: &str,
    ) -> BTreeMap<String, Option<(String, u32)>> {
        let mut result = BTreeMap::new();
        let mut priority = BinaryHeap::new();

        result.insert(label.to_string(), None);

        for (new, weight) in &self.0[label] {
            result
                .insert(String::clone(new), Some((label.to_string(), *weight)));
            priority.push(Reverse((
                *weight,
                String::clone(new),
                label.to_string(),
            )));
        }

        while let Some(Reverse((dist_new, new, prev))) = priority.pop() {
            match &result[new.as_str()] {
                Some((p, d))
                    if p.as_str() == prev.as_str() && *d == dist_new => {}

                _ => continue,
            }

            for (next, weight) in &self.0[new.as_str()] {
                match result.get(next.as_str()) {
                    Some(Some((_, dist_next)))
                        if dist_new + *weight >= *dist_next => {}
                    Some(None) => {}
                    _ => {
                        result.insert(
                            next.clone(),
                            Some((new.clone(), *weight + dist_new)),
                        );
                        priority.push(Reverse((
                            *weight + dist_new,
                            next.clone(),
                            new.clone(),
                        )));
                    }
                }
            }
        }

        result
    }

    pub fn path(&self, start: &str, end: &str) -> Vec<(String, u32)> {
        let mut heap = BinaryHeap::new();
        let mut visited = BTreeSet::new();

        heap.push(Reverse((0u32, start, vec![(start.to_string(), 0u32)])));
        while let Some(Reverse((cost, key, path))) = heap.pop() {
            if visited.contains(key) {
                continue;
            }

            visited.insert(key);
            if key == end {
                return path;
            }

            for (v, c) in &self.0[key] {
                if visited.contains(v.as_str()) {
                    continue;
                }

                let next_item = cost + c;
                let mut paths = path.clone();
                paths.push((v.clone(), *c));
                heap.push(Reverse((next_item, v.as_str(), paths)));
            }
        }

        vec![]
    }
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "==========graph begin==========")?;
        for (node, edges) in &self.0 {
            let mut fms = String::with_capacity(512);
            fms.push('[');
            for (v, w) in edges {
                fms.push_str(&format!("{}:{},", v, w));
            }
            if fms.len() > 1 {
                fms.pop(); //pop last ,
            }

            fms.push(']');
            writeln!(f, "[{}: {}]", node, fms)?;
        }
        writeln!(f, "==========graph end==========")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new_graph() {
        let mut g = Graph::new();
        g.add_edge("a", "b");
        g.add_edge("c", "a");

        assert!(g.0.get("a").is_some());
        assert!(g.0.get("d").is_none());
        assert!(g.0.get("b").is_some());
        assert!(g.0.get("a").unwrap().len() == 2);
    }

    #[test]
    fn test_path() {
        let mut g = Graph::new();
        g.add_edge("a", "b");
        g.add_edge("c", "a");

        let dest = vec![("a".to_string(), 0), ("b".to_string(), 1)];
        assert_eq!(dest, g.path("a", "b"));
    }

    #[test]
    fn test_dijkstra() {
        let mut g = Graph::new();
        g.add_edge("a", "b");
        g.add_edge("c", "a");

        let paths = g.dijkstra("a");
        assert_eq!(paths["b"].as_ref().unwrap().1, 1);
    }
}
