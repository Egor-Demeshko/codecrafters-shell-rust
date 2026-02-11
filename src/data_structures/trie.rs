use std::collections::HashMap;

pub struct TrieNode {
    children: HashMap<char, TrieNode>,
    end: bool,
}

impl TrieNode {
    pub fn new() -> Self {
        Self {
            children: HashMap::new(),
            end: false,
        }
    }
}

pub struct CommandTrie {
    root: TrieNode,
}

impl CommandTrie {
    pub fn new() -> Self {
        CommandTrie {
            root: TrieNode::new(),
        }
    }

    pub fn add_command(&mut self, name: &str) -> bool {
        let mut current_node: &mut TrieNode = &mut self.root;
        for char in name.chars() {
            if current_node.children.contains_key(&char) {
                current_node = match current_node.children.get_mut(&char) {
                    Some(node) => node,
                    None => return false,
                }
            } else {
                current_node.children.insert(char, TrieNode::new());
                current_node = current_node.children.get_mut(&char).unwrap();
            }
        }
        // для обозначения конца необходимо добавить последний эелмент
        let mut node = TrieNode::new();
        node.end = true;
        current_node.children.insert('*', node);
        true
    }

    pub fn get_all_for_prefix(&self, line: &str) -> Vec<String> {
        let (prefix, node) = self.get_prefix_node(line);
        let mut commands: Vec<String> = vec![];
        self.collect_commands(&mut commands, prefix, node);
        commands
    }

    pub fn collect_commands(
        &self,
        command_list: &mut Vec<String>,
        command: String,
        next_node: &TrieNode,
    ) -> () {
        for (key, node) in next_node.children.iter() {
            if *key == '*' || node.end == true {
                command_list.push(command.clone());
                continue;
            }

            let mut iteration_command: String = command.clone();
            iteration_command.push(*key);

            self.collect_commands(command_list, iteration_command, node);
        }
    }

    pub fn get_prefix_node(&self, line: &str) -> (String, &TrieNode) {
        let mut command = String::new();
        let mut current_node: &TrieNode = &self.root;
        for char in line.chars() {
            if current_node.children.contains_key(&char) {
                current_node = match current_node.children.get(&char) {
                    Some(node) => node,
                    None => break,
                };
                command.push(char);
            } else {
                return (String::new(), current_node);
            }
        }

        (command, current_node)
    }
}
