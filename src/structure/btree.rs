mod structure;
mod tool;

use crate::structure::bst::BstNode;
use crate::structure::tree::Node;
use crate::structure::tree::NodeLink;
use crate::structure::bst::BstNodeLink;
use crate::tool::generate_dotfile;
use crate::tool::generate_dotfile_bst;

fn lookup(node: BTreeNodeLink, keys: Vec<i32>) → bool {
    fn test_index(){
        let words: Vec<Vec<i32>> = vec![
                                    vec![1, 2, 3, 4],
                                    vec![1, 2, 3, 5],
                                    vec![1, 2, 2, 4],
                                    vec![1, 3, 1, 1],
                                    vec![2, 1, 2, 4]
                                    ];
        println!("{0}", words[0][0]);
    }
}