use std::cmp;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
struct Node {
    key: i8, 
    height: i8,
    left: Box<Option<Node>>,
    right: Box<Option<Node>>
    //right: Node
    /*width: 5,
    child1: Node
    child2: Node*/
}

fn update_height(node: Node) -> i8 {
    let mut left_height = -1; 
    let mut right_height = -1; 
    if node.left !=  Box::new(None) {
        left_height = node.left.unwrap().height; 
    } 
    if node.right !=  Box::new(None) {
        right_height  = node.right.unwrap().height; 
    }
    return height_of_node(left_height, right_height)
}

fn height_of_node(left_height : i8, right_height: i8) -> i8 {
    return cmp::max(left_height, right_height) + 1;  // it will return 0 if no nodes, right... 
}

// starts from the root node 
fn insert(option_node: Box<Option<Node>>, key: i8) -> Node {
    // base case, if you've reached the end of the recursive function 
    if option_node == Box::new(None) {
        return Node {key: key, height: 0, left: Box::new(None), right: Box::new(None)};
    } 
    let mut node = option_node.unwrap();
    if key < node.key {
        node.left = Box::new(Some(insert(node.left, key))) // recursive call
    } else {
        node.right = Box::new(Some(insert(node.right, key))) // recursive call
    }

    node.height = update_height(node.clone()); 
    println!("key: {:?}", key);  
    return node; 
}

fn balance(option_node: Box<Option<Node>>) -> Box<Option<Node>> {
    let node = option_node.clone().unwrap();
    if balance_of_node(node.clone()) <= -2 {
        if balance_of_node(node.clone().left.unwrap()) >= 0 {
            // return left_left_rotation(option_node)
        }
        else {
            // return left_right_rotation(option_node)
        }
    }
    else if balance_of_node(node.clone()) >= 2 {
        if balance_of_node(node.clone().right.unwrap()) >= 0 {
            // return right_right_rotation(option_node)
        }
        else {
            // return right_left_rotation(option_node)
        }
    }

    return option_node
}

fn rightRotation(node: Node) -> Box<Option<Node>> {
    // Rotation 
    let right = node.left;
    node.left = right.unwrap().right; 
    right.unwrap().right = Box::new(Some(node.left.unwrap()));

    // Update height and balance 
    node.height = update_height(node.clone()); 
    node = balance(right.clone().unwrap()); //Box::new(Some(right.unwrap().clone()))); //right.unwrap());  //Box::new(Some(right.unwrap()))
    return right 
}

fn balance_of_node(node : Node) -> i8 {
    return node.left.unwrap().height - node.right.unwrap().height; 
}

// stay within balance of 1, so you get logarithmic search 
// all binary search trees have left nodes < root, right nodes >= root 
// the key is the actual value the node stores 

fn main() {
    let node1 = Node {key: 1, height: 10, left: Box::new(None), right: Box::new(None)};
    let node2 = Node {key: 2, height: 5, left: Box::new(Some(node1.clone())), right: Box::new(Some(node1.clone()))};
    println!("node height: {}, node left: {:?}", node1.height, node1.left);
    println!("node2: {:?}", node2);  
    // node.height = cmp::max(node.left.height, node.right.height) + 1;  
    insert(Box::new(Some(node1)), 3);
    println!("{:?}", balance_of_node(node2.clone()));
}