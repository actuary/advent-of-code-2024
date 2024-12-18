use std::collections::{HashMap, HashSet};

fn parse(data: &str) -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
    let parts: Vec<String> = data
        .trim()
        .split("\n\n")
        .map(|v| v.to_string())
        .collect::<Vec<String>>();

    assert!(parts.len() == 2);

    let mut ordering: HashMap<i32, Vec<i32>> = HashMap::new();

    parts[0].split("\n").for_each(|line| {
        let (l, r) = line.split_once("|").unwrap();

        let before = l.parse::<i32>().unwrap();
        let after = r.parse::<i32>().unwrap();

        match ordering.get_mut(&before) {
            Some(vec) => vec.push(after),
            None => {
                ordering.insert(before, vec![after]);
            }
        }
    });

    let updates: Vec<Vec<i32>> = parts[1]
        .split("\n")
        .map(|line| line.split(",").map(|v| v.parse().unwrap()).collect())
        .collect();

    (ordering, updates)
}

fn is_topologically_sorted(orderings: &HashMap<i32, Vec<i32>>, update: &Vec<i32>) -> bool {
    let mut found: HashSet<i32> = HashSet::new();

    let mut is_sorted: bool = true;
    for &n in update {
        found.insert(n);

        if !orderings.contains_key(&n) {
            continue;
        }

        for &m in &orderings[&n] {
            if found.contains(&m) {
                is_sorted = false;
                break;
            }
        }
    }

    is_sorted
}

pub fn part1(data: &str) -> i32 {
    let (orderings, updates) = parse(data);

    let mut result: i32 = 0;
    for update in &updates {
        if is_topologically_sorted(&orderings, &update) {
            result += update[update.len() / 2];
        }
    }

    result
}


fn visit(n: i32, graph: &HashMap<i32, Vec<i32>>, unmarked_nodes: &Vec<i32>, l: &mut Vec<i32>) { 
    if l.contains(&n) {
        return
    }

    for &m in &graph[&n] {
        visit(m, &graph, unmarked_nodes, l);
    };

    l.push(n);
}

fn topologically_sort(graph: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
    //https://en.wikipedia.org/wiki/Topological_sorting
    let mut l: Vec<i32> = Vec::new();
    let mut unmarked_nodes: Vec<i32> = graph.keys().clone().cloned().collect();

    while !unmarked_nodes.is_empty() {
        let node = unmarked_nodes.pop().unwrap();

        visit(node, &graph, &unmarked_nodes, &mut l);
    }

    //l.reverse(); // don't actually need to do this as just taking mid.
    l
}

pub fn part2(data: &str) -> i32 {
    let (orderings, updates) = parse(data);

    let broken_updates = updates
        .iter()
        .filter(|update| !is_topologically_sorted(&orderings, &update));

    let mut result: i32 = 0;

    for update in broken_updates {
        // subset the graph so we only have the values we're concerned about
        let mut new_graph: HashMap<i32, Vec<i32>> = HashMap::new();
        
        for n in update {

            let all_reachable: HashSet<i32> = if !orderings.contains_key(&n) {
                HashSet::new()
            } else {
                HashSet::from_iter(orderings[n].iter().cloned())
            };
            let all: HashSet<i32> = HashSet::from_iter(update.iter().cloned());
            let reachable: Vec<i32> = all_reachable.intersection(&all).map(|v| *v).collect();
            new_graph.insert(*n, reachable);
        }
        
        let ordered = topologically_sort(&new_graph);
        result += ordered[ordered.len() / 2];
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let data = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!(part1(data), 143);
    }

    #[test]
    fn part2_works() {
        let data = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!(part2(data), 123);
    }
}
