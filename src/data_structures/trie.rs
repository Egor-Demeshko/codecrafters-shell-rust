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

    pub fn get_first_command(&self, line: &str) -> String {
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
                return String::new();
            }
        }
        if command.is_empty() {
            return command;
        }

        if current_node.children.contains_key(&'*') {
            return command;
        }

        if Self::till_end(current_node, &mut command) {
            return command;
        }

        String::new()
    }

    pub fn till_end(node: &TrieNode, buffer: &mut String) -> bool {
        if node.end {
            return true;
        }

        for (char, child_node) in node.children.iter() {
            if *char == '*' {
                return true;
            }
            buffer.push(*char);
            if Self::till_end(child_node, buffer) {
                return true;
            }
        }
        return false;
    }
}
