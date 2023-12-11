use std::{fs, collections::HashMap};

#[derive(Debug, PartialEq, Eq)]
struct Node<'a> {
    left: &'a str,
    right: &'a str
}

fn main() {
    let file = fs::read_to_string("input.txt")
        .expect("should read file");
    let content = file
        .split("\n\n")
        .collect::<Vec<&str>>();

    let mut node_map: HashMap<&str, Node> = HashMap::new();
    let (instr, nodes) = (content[0].chars().collect::<Vec<char>>(), content[1]);
    nodes
        .trim()
        .split("\n")
        .map(|node_l| node_l.split(" = ").collect::<Vec<&str>>())
        .for_each(|node| {
            let (node_name, node_dir) = (node[0], node[1].strip_prefix("(")
                                         .unwrap()
                                         .strip_suffix(")")
                                         .unwrap()
                                         .split(", ")
                                         .collect::<Vec<&str>>());
            node_map.insert(node_name, Node {left: node_dir[0], right: node_dir[1]});
        });

    let mut curr_node      = "AAA";
    let target_node        = "ZZZ";
    let mut curr_instr_idx = 0;
    let mut res            = 0;
    while curr_node != target_node {
        res += 1;
        let curr_instr = instr[curr_instr_idx % instr.len()];
        curr_node = match curr_instr {
            'R' => node_map[&curr_node].right,
            'L' => node_map[&curr_node].left,
            _   => unreachable!("should only contain R or L directions")
        };
        curr_instr_idx += 1;
    }
    println!("{res}");
}
