use std::collections::{HashMap, HashSet};

fn parse(data: &str) -> Vec<(&str, &str)> {
    data.trim()
        .split("\n")
        .map(|line| line.split_once("-").unwrap())
        .collect()
}

pub fn part1(data: &str) -> i64 {
    let links = parse(data);
    let mut connection_sets: HashMap<&str, Vec<&str>> = HashMap::new();

    for (from, to) in links {
        connection_sets
            .entry(from)
            .or_insert_with(Vec::new)
            .push(to);

        connection_sets
            .entry(to)
            .or_insert_with(Vec::new)
            .push(from);
    }

    let mut triples: HashSet<(&str, &str, &str)> = HashSet::new();

    for (a, a_set) in &connection_sets {
        for b in a_set {
            for c in connection_sets.get(b).unwrap() {
                if a_set.contains(c) {
                    let mut triple = [a, b, c];
                    triple.sort();
                    triples.insert((triple[0], triple[1], triple[2]));
                }
            }
        }
    }

    triples
        .iter()
        .filter(|(a, b, c)| a.starts_with('t') || b.starts_with('t') || c.starts_with('t'))
        .count() as i64
}

fn bron_kerbosch<'a>(
    r: &HashSet<&'a str>,
    p: &HashSet<&'a str>,
    x: &HashSet<&'a str>,
    connection_sets: &'a HashMap<&str, HashSet<&str>>,
) -> HashSet<&'a str> {
    // I am bad at rust
    // Implementation from here: https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm
    if p.len() == 0 && x.len() == 0 {
        return r.clone();
    }

    let mut maximum_clique: HashSet<&str> = HashSet::new();
    let mut new_p = p.clone();
    let mut new_x = x.clone();

    for &v in p {
        let neighbours = connection_sets.get(v).unwrap();
        let mut new_r = r.clone();
        new_r.insert(v);

        let current_clique = bron_kerbosch(
            &new_r,
            &new_p.intersection(neighbours).cloned().collect(),
            &new_x.intersection(neighbours).cloned().collect(),
            connection_sets,
        );

        if current_clique.len() > maximum_clique.len() {
            maximum_clique = current_clique;
        }

        new_p.remove(v);
        new_x.insert(v);
    }

    maximum_clique
}

pub fn part2(data: &str) -> String {
    // needed hint for this (obv)...
    let links = parse(data);
    let mut connection_sets: HashMap<&str, HashSet<&str>> = HashMap::new();

    for (from, to) in links {
        connection_sets
            .entry(from)
            .or_insert_with(HashSet::new)
            .insert(to);

        connection_sets
            .entry(to)
            .or_insert_with(HashSet::new)
            .insert(from);
    }

    let r: HashSet<&str> = HashSet::new();
    let p: HashSet<&str> = connection_sets.keys().cloned().collect();
    let x: HashSet<&str> = HashSet::new();

    let mut max_clique: Vec<&str> = bron_kerbosch(&r, &p, &x, &connection_sets)
        .iter()
        .cloned()
        .collect();
    max_clique.sort();

    max_clique.join(",").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let data = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";
        assert_eq!(part1(data), 7);
    }

    #[test]
    fn part2_works() {
        let data = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";
        assert_eq!(part2(data), "co,de,ka,ta");
    }
}
