use super::get_input_vec;

// 43821 - too low
// 43827 - too high
// 43824 - too low
// 43825 - correct
pub fn problem_eight_part_one() -> u32 {
    let input = get_input_vec("eight.txt");
    let tree = input
        .first()
        .unwrap()
        .split(' ')
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let node = get_node(&tree);
    calculate_sum_of_metadata_for_node(&node)
}

// 19276 - correct
pub fn problem_eight_part_two() -> u32 {
    let input = get_input_vec("eight.txt");
    let tree = input
        .first()
        .unwrap()
        .split(' ')
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let node = get_node(&tree);
    node.get_value()
}

fn calculate_sum_of_metadata_for_node(node: &Node) -> u32 {
    let metadata_sum: u32 = node.metadata.iter().sum();
    if node.child_nodes.is_empty() {
        metadata_sum
    } else {
        let child_sum = node
            .child_nodes
            .iter()
            .map(|n| calculate_sum_of_metadata_for_node(n))
            .sum::<u32>();

        child_sum + metadata_sum
    }
}

#[derive(Debug)]
struct Node {
    length: usize,
    child_nodes: Vec<Node>,
    metadata: Vec<u32>,
}

impl Node {
    fn get_value(&self) -> u32 {
        if self.child_nodes.is_empty() {
            self.metadata.iter().sum()
        } else {
            self.metadata
                .iter()
                .filter_map(|m| self.child_nodes.get((*m - 1) as usize))
                .map(|c| c.get_value())
                .sum()
        }
    }
}

fn get_node(tree: &[u32]) -> Node {
    let header_length = 2 as usize;
    let number_of_child_nodes = tree[0] as usize;
    let number_of_metadata_entries = tree[1] as usize;

    if number_of_child_nodes == 0 {
        return Node {
            length: number_of_metadata_entries + header_length,
            child_nodes: Vec::new(),
            metadata: tree[header_length..(number_of_metadata_entries + header_length)].to_vec(),
        };
    }

    let mut child_nodes: Vec<Node> = Vec::new();
    for i in 0..number_of_child_nodes {
        let node_tree = if i == 0 {
            tree[header_length..].to_vec()
        } else {
            let length = get_nodes_length(&child_nodes) + header_length;
            tree[length..].to_vec()
        };
        let node = get_node(&node_tree);
        child_nodes.push(node);
    }

    let child_nodes_length = get_nodes_length(&child_nodes);
    let metadata_start = header_length + child_nodes_length;
    let metadata_end = metadata_start + number_of_metadata_entries;

    Node {
        length: header_length + child_nodes_length + number_of_metadata_entries,
        child_nodes: child_nodes,
        metadata: tree[metadata_start..metadata_end].to_vec(),
    }
}

fn get_nodes_length(nodes: &[Node]) -> usize {
    nodes.iter().map(|s| s.length).sum()
}
