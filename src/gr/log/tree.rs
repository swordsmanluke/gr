use std::fmt::{Display, Formatter};
use colored::{Color, Colorize};
use itertools::Itertools;
use crate::gr::log::color_cycle::ColorCycle;
use crate::indent::Indentable;

/* Top down glyphs */
const SPACE: &str   = "  ";
const PIPE: &str    = "| ";
const MSG_BAR: &str = "║ ";
const HAS_NEIGHBOR: &str = "├─";
const NO_NEIGHBOR: &str  = "└─";

/* Bottom up glyphs */
const NO_NEIGHBOR_B: &str = "┌─";

pub trait WithChildren {
    fn children(&self) -> Vec<Self>
    where Self: Sized
    {
        Vec::new()
    }
}

pub struct Node<T>
where T: Display + Clone + WithChildren
{
    pub data: T,
    pub children: Vec<Node<T>>
}

pub struct Tree<T>
where T: Display + Clone + WithChildren
{
    pub root: Node<T>,
    color_cycle: ColorCycle
}

impl<T: Display + Clone + WithChildren> From<T> for Node<T> {
    fn from(data: T) -> Self {
        let children = data.children().into_iter().map(Node::from).collect_vec();
        Self { data, children }
    }
}

impl<T: Display + Clone + WithChildren> Display for Node<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
}

impl<T: Display + Clone + WithChildren> Tree<T> {
    pub fn new(root: T) -> Self {
        let root = Node::from(root);
        let color_cycle = ColorCycle::new();
        Self { root, color_cycle }
    }

    pub fn to_string(&self) -> String {
        // Output should be a right-side-up tree,
        // with roots at the bottom and leaves at the top
        self.dfs_stringify(&self.root)
    }

    fn dfs_stringify(&self, root: &Node<T>) -> String {
        let mut color_cycle = ColorCycle::new();
        let mut out = root.to_string();
        let last_sib = root.children.len() - 1;
        let color = self.color_cycle.color();
        for i in 0..root.children.len() {
            let c = &root.children[i];
            out = format!("{}\n{}", self.dfs_traverse_node(c, "", i < last_sib, &mut color_cycle).iter().rev().join("\n"), out);
        };
        out
    }

    fn dfs_traverse_node(&self, node: &Node<T>, padding: &str, has_right_sib: bool, mut color_cycle: &mut ColorCycle) -> Vec<String> {
        let pointer = if has_right_sib { HAS_NEIGHBOR } else { NO_NEIGHBOR_B }.to_string();

        let raw_str = node.to_string();
        let mut node_lines = raw_str.split("\n");
        let firstline = format!("{}{}", pointer, node_lines.next().unwrap().to_owned());
        let mut lines = vec![firstline.clone()];
        lines.append(&mut node_lines
            .map(|s| s.indent_with(&MSG_BAR.color(color_cycle.color()).to_string(), 0)
                            .indent_with(&PIPE.color(color_cycle.color()).to_string(), 1)
                            ).collect_vec());
        let node_string = lines.join("\n").color(color_cycle.color());

        let mut my_block = vec![node_string.indent_with(&padding, 1)];

        // Increase padding for children
        let padding = format!("{}{}", padding, if has_right_sib { PIPE.color(color_cycle.color()).to_string() } else { SPACE.to_owned() });

        // Concat children to the block
        for i in 0..node.children.len() {
            let c = &node.children[i];
            let last_sib = node.children.len() - 1;
            if i > 0 { color_cycle.advance(); }
            self.dfs_traverse_node(c, &padding, i < last_sib, &mut color_cycle).into_iter().for_each(|s| my_block.push(s));
        };

        my_block
    }

    fn dfs_split_width(&self, node: &Node<T>) -> usize {
        match node.children.len() {
            0 => 1,  // leaves don't increase the number of splits
            _ => node.children.iter().map(|c| self.dfs_split_width(c)).collect_vec().iter().sum()
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[derive(Clone)]
    struct TreeNode {
        name: String,
        children: Vec<TreeNode>
    }

    impl Display for TreeNode {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.name)
        }
    }

    impl WithChildren for TreeNode {
        fn children(&self) -> Vec<TreeNode> {
            self.children.clone()
        }
    }

    fn test_tree() -> Tree<TreeNode>{
        Tree::new(TreeNode {
            name: String::from("root"),
            children: vec![
                TreeNode {
                    name: String::from("a"),
                    children: vec![
                        TreeNode {
                            name: String::from("aa"),
                            children: vec![]
                        },
                        TreeNode {
                            name: String::from("ab\nababab\nXyzzzay\ndfkai"),
                            children: vec![
                                TreeNode {
                                    name: String::from("aba"),
                                    children: vec![]
                                }
                            ]
                        },
                        TreeNode {
                            name: String::from("ac"),
                            children: vec![]
                        }
                    ]
                },
                TreeNode {
                    name: String::from("b"),
                    children: vec![
                        TreeNode {
                            name: String::from("ba"),
                            children: vec![]
                        }
                    ]
                }
            ]
        })
    }

    #[test]
    fn test_log_generates_tree() {
        let t = test_tree();
        assert!(t.to_string().len() > 0);
    }

    #[test]
    fn test_log_generates_expected_tree() {
        let t = test_tree();
        let expected = "\
        root\
        | a\
        | | aa\
        | | ab\
        | b\
        | | ba";

        println!("{}", t.to_string().trim());

        assert_eq!(t.to_string().trim(), expected.trim());
    }
}