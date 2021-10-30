use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;

type RcRefCell<T> = Rc<RefCell<T>>;

#[derive(Debug)]
enum NodeData {
    Inner {
        left: RcRefCell<Node>,
        right: RcRefCell<Node>,
    },
    Leaf(u8),
}

struct Node {
    parent: Option<RcRefCell<Node>>,
    subsize: u16,
    data: Box<NodeData>,
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("subsize", &self.subsize)
            .field("data", &self.data)
            .finish()
    }
}

pub struct MoveToFrontTree {
    root: RcRefCell<Node>,
    node_map: HashMap<u8, RcRefCell<Node>>,
}

impl<'a> MoveToFrontTree {
    fn construct(
        parent: Option<RcRefCell<Node>>,
        bytes: &[u8],
        map: &mut HashMap<u8, RcRefCell<Node>>,
    ) -> RcRefCell<Node> {
        if bytes.len() == 1 {
            let byte = bytes[0];
            let node = Node {
                parent,
                subsize: 1,
                data: Box::new(NodeData::Leaf(byte)),
            };
            let node = Rc::new(RefCell::new(node));
            map.insert(byte, node.clone());
            return node;
        }

        let left = MoveToFrontTree::construct(None, &bytes[..bytes.len() / 2], map);
        let right = MoveToFrontTree::construct(None, &bytes[bytes.len() / 2..], map);

        let inner = Node {
            parent,
            subsize: bytes.len() as u16,
            data: Box::new(NodeData::Inner { left, right }),
        };

        let inner = Rc::new(RefCell::new(inner));
        let inner_clone = inner.clone();
        let mut inner_clone = inner_clone.borrow_mut();

        let node_data = inner_clone.data.as_mut();
        let (left, right) = match node_data {
            NodeData::Inner { left, right } => (left, right),
            _ => panic!(),
        };

        let mut left = left.borrow_mut();
        let mut right = right.borrow_mut();

        left.parent = Some(inner.clone());
        right.parent = Some(inner.clone());

        inner
    }

    pub fn new() -> MoveToFrontTree {
        let mut node_map = HashMap::with_capacity(256);
        let root = MoveToFrontTree::construct(None, &(0..255).collect::<Vec<u8>>(), &mut node_map);
        MoveToFrontTree { node_map, root }
    }

    fn count_index(node: RcRefCell<Node>) -> u8 {
        let node = node.borrow();

        if let Some(ref parent) = node.parent {
            let data = &parent.borrow().data;
            let (left, _) = match **data {
                NodeData::Inner {
                    ref left,
                    ref right,
                } => (left, right),
                _ => panic!(),
            };

            let subsize = if &*left.borrow() as *const Node == &*node as *const Node {
                0
            } else {
                // println!("{:#?}", parent);
                left.borrow().subsize
            };
            return subsize as u8 + MoveToFrontTree::count_index(parent.clone());
        }

        return 0;
    }

    fn recursive_decrement(node: RcRefCell<Node>) {
        let mut node = node.borrow_mut();
        node.subsize -= 1;
        if let Some(ref parent) = node.parent {
            MoveToFrontTree::recursive_decrement(parent.clone());
        }
    }

    fn remove_node(node: RcRefCell<Node>) {
        // TODO DEBUG
        let node = node.borrow();

        if let Some(ref parent) = node.parent {
            let data = &parent.borrow().data;
            let (left, right) = match **data {
                NodeData::Inner {
                    ref left,
                    ref right,
                } => (left, right),
                _ => panic!(),
            };

            let other = if &*left.borrow() as *const Node == &*node as *const Node {
                right
            } else {
                left
            };

            let parent_borrowed = parent.borrow();

            if let Some(ref grand) = parent_borrowed.parent {
                let mut grand_borrowed = grand.borrow_mut();
                grand_borrowed.data = match *grand_borrowed.data {
                    NodeData::Inner {
                        ref left,
                        ref right,
                    } => {
                        let node_data =
                            if &*left.borrow() as *const Node == &*parent_borrowed as *const Node {
                                NodeData::Inner {
                                    left: other.clone(),
                                    right: right.clone(),
                                }
                            } else {
                                NodeData::Inner {
                                    left: left.clone(),
                                    right: other.clone(),
                                }
                            };
                        Box::new(node_data)
                    }
                    _ => panic!(),
                };
                std::mem::drop(grand_borrowed);
                MoveToFrontTree::recursive_decrement(grand.clone());
            }
        }
    }

    fn insert_front(node: RcRefCell<Node>, byte: u8) -> RcRefCell<Node> {
        let cloned_node = node.clone();
        let mut borrowed_node = cloned_node.borrow_mut();
        match &*borrowed_node.data {
            NodeData::Inner { ref left, .. } => {
                let left = left.clone();
                borrowed_node.subsize += 1;
                std::mem::drop(borrowed_node);
                MoveToFrontTree::insert_front(left, byte)
            }
            NodeData::Leaf(_) => {
                if let Some(ref parent) = borrowed_node.parent {
                    let parent = parent.clone();
                    let new_leaf = Node {
                        parent: None,
                        subsize: 1,
                        data: Box::new(NodeData::Leaf(byte)),
                    };
                    let new_leaf = Rc::new(RefCell::new(new_leaf));

                    let new_inner = Node {
                        parent: Some(parent.clone()),
                        subsize: 2,
                        data: Box::new(NodeData::Inner {
                            left: new_leaf.clone(),
                            right: node,
                        }),
                    };

                    let new_inner = Rc::new(RefCell::new(new_inner));
                    borrowed_node.parent = Some(new_inner.clone());

                    {
                        let mut new_leaf_borrowed = new_leaf.borrow_mut();
                        new_leaf_borrowed.parent = Some(new_inner.clone());

                        let mut parent = parent.borrow_mut();
                        parent.data = match &*parent.data {
                            NodeData::Inner { right, .. } => Box::new(NodeData::Inner {
                                right: right.clone(),
                                left: new_inner.clone(),
                            }),
                            _ => panic!(),
                        }
                    }

                    new_leaf
                } else {
                    panic!()
                }
            }
        }
    }

    pub fn move_to_front(&mut self, byte: u8) -> u8 {
        let node = self.node_map[&byte].clone();
        let index = MoveToFrontTree::count_index(node.clone());

        if index != 0 {
            MoveToFrontTree::remove_node(node);
            println!("{:#?}", self.root);
            let new_node = MoveToFrontTree::insert_front(self.root.clone(), byte);
            self.node_map.insert(byte, new_node);
        }

        println!("{:#?}", self.root);

        index
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn count_index_test() {
        let mtft = MoveToFrontTree::new();
        for i in 0..255 {
            assert_eq!(i, MoveToFrontTree::count_index(mtft.node_map[&i].clone()))
        }
    }

    #[test]
    fn insert_front_test() {
        let mut mtft = MoveToFrontTree::new();
        assert_eq!(100, mtft.move_to_front(100));
        assert_eq!(0, mtft.move_to_front(100));
        assert_eq!(0, mtft.move_to_front(100));
        assert_eq!(101, mtft.move_to_front(101));
        assert_eq!(0, mtft.move_to_front(101));
        assert_eq!(1, mtft.move_to_front(100));
        assert_eq!(0, mtft.move_to_front(100));
    }

    #[test]
    fn insert_front_test_1() {
        let mut mtft = MoveToFrontTree::new();
        assert_eq!(1, mtft.move_to_front(1));
        assert_eq!(0, mtft.move_to_front(1));
        assert_eq!(0, mtft.move_to_front(1));
        assert_eq!(2, mtft.move_to_front(2));
        assert_eq!(0, mtft.move_to_front(2));
        assert_eq!(1, mtft.move_to_front(1));
        assert_eq!(0, mtft.move_to_front(1));
    }
}
