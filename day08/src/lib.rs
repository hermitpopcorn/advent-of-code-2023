use std::{
    cell::RefCell,
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    rc::Rc,
};

pub type MapNodePointer = Rc<RefCell<MapNode>>;

#[derive(Debug, Eq, Clone)]
pub struct MapNode {
    pub label: String,
    pub left: Option<MapNodePointer>,
    pub right: Option<MapNodePointer>,
}

impl PartialEq for MapNode {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label
    }
}

impl std::fmt::Display for MapNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let left_label = match &self.left {
            Some(node) => node.as_ref().borrow().label.clone(),
            None => String::from("None"),
        };

        let right_label = match &self.right {
            Some(node) => node.as_ref().borrow().label.clone(),
            None => String::from("None"),
        };

        f.write_str(&format!(
            "Node [{}] L: [{}] R: [{}]",
            self.label, left_label, right_label
        ))
    }
}

pub fn parse_input_into_instruction_and_map_nodes(
    path: &str,
) -> (String, HashMap<String, MapNodePointer>) {
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let line = lines.next().unwrap().unwrap();
    let instruction = line;

    let _ = lines.next().unwrap(); // skip empty line

    let mut node_notations = HashMap::new();

    for line in lines {
        let line = line.unwrap();
        if line.is_empty() {
            continue;
        }

        let split_left_right: Vec<&str> = line.split(" = ").collect();
        let label = String::from(split_left_right[0]);
        let node = String::from(split_left_right[1]);
        let node_contents = (&node[1..&node.len() - 1])
            .split(',')
            .collect::<Vec<&str>>();
        let left_label = String::from(node_contents[0].trim());
        let right_label = String::from(node_contents[1].trim());

        node_notations.insert(label, [left_label, right_label]);
    }

    let mut nodes = HashMap::new();

    for (node_label, _) in &node_notations {
        nodes.insert(
            node_label.clone(),
            Rc::new(RefCell::new(MapNode {
                label: node_label.clone(),
                left: None,
                right: None,
            })),
        );
    }

    for (node_label, node) in nodes.iter() {
        let notation = node_notations.get(node_label).unwrap();

        let left_node = nodes.get(&notation[0]).unwrap().clone();
        let right_node = nodes.get(&notation[1]).unwrap().clone();

        node.as_ref().borrow_mut().left = Some(left_node.clone());
        node.as_ref().borrow_mut().right = Some(right_node.clone());
    }

    (instruction, nodes)
}
