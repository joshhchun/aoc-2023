use std::{fs, collections::{HashMap, VecDeque}};

#[derive(Debug, PartialEq, Eq)]
struct Node<'a> {
    left: &'a str,
    right: &'a str
}

fn get_start_nodes<'a, 'b>(node_map: &'a HashMap<&'b str, Node>) -> VecDeque<&'b str> {
    let mut queue = VecDeque::new();
    node_map
        .keys()
        .for_each(|&key| {
            // if key ends with 'A' then push the key to the queue
            if key.ends_with('A') {
                queue.push_back(key);
            }
        });

    queue
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

    let queue = get_start_nodes(&node_map);
    let cycle_lengths: Vec<u64> = queue
        .iter()
        .map(|&node| {
            let mut curr_node      = node;
            let mut len            = 0;
            let mut curr_instr_idx = 0;
            while !curr_node.ends_with('Z') {
                len += 1;
                let curr_instr = instr[curr_instr_idx % instr.len()];
                curr_node = match curr_instr {
                    'R' => node_map[&curr_node].right,
                    'L' => node_map[&curr_node].left,
                    _   => unreachable!("should only contain R or L directions")
                };
                curr_instr_idx += 1;
            }
            len
        })
        .collect();

    println!("cycle lengths: {:?}", cycle_lengths);

    // find LCM of all numbers in cycle_lenths
    let res = cycle_lengths
        .iter()
        .skip(0)
        .fold(cycle_lengths[0], |acc, &len| {
            acc * len / gcd(acc, len)
        });

    println!("{res}");
        
}

fn gcd(res: u64, i: u64) -> u64 {
    match i {
        0 => res,
        _ => gcd(i, res % i)
    }
}
