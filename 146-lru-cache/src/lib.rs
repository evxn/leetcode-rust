use std::{
    collections::{HashMap, LinkedList as LL},
    ptr::NonNull,
};

struct LinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
}

#[derive(Clone)]
struct Node<T> {
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
    elem: T,
}

impl<T> Node<T> {
    fn new(elem: T) -> Self {
        Self {
            next: None,
            prev: None,
            elem,
        }
    }
}

impl<T: Clone> LinkedList<T> {
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }

    fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    fn len(&self) -> usize {
        self.len
    }

    fn front(&self) -> Option<&T> {
        unsafe { self.head.as_ref().map(|node| &node.as_ref().elem) }
    }

    fn back(&self) -> Option<&T> {
        unsafe { self.tail.as_ref().map(|node| &node.as_ref().elem) }
    }

    unsafe fn push_back_node(&mut self, node: NonNull<Node<T>>) {
        // This method takes care not to create mutable references to whole nodes,
        // to maintain validity of aliasing pointers into `element`.
        unsafe {
            (*node.as_ptr()).next = None;
            (*node.as_ptr()).prev = self.tail;
            let node = Some(node);

            match self.tail {
                None => self.head = node,
                // Not creating new mutable (unique!) references overlapping `element`.
                Some(tail) => (*tail.as_ptr()).next = node,
            }

            self.tail = node;
            self.len += 1;
        }
    }

    fn push_back(&mut self, elt: T) {
        let node = Box::new(Node::new(elt));
        let node = Box::into_raw(node);
        let node_ptr = NonNull::new(node).unwrap();
        // SAFETY: node_ptr is a unique pointer to a node we boxed with self.alloc and leaked
        unsafe {
            self.push_back_node(node_ptr);
        }
    }

    fn pop_back_node(&mut self) -> Option<Box<Node<T>>> {
        // This method takes care not to create mutable references to whole nodes,
        // to maintain validity of aliasing pointers into `element`.
        self.tail.map(|node| unsafe {
            let node = Box::from_raw(node.as_ptr());
            self.tail = node.prev;

            match self.tail {
                None => self.head = None,
                // Not creating new mutable (unique!) references overlapping `element`.
                Some(tail) => (*tail.as_ptr()).next = None,
            }

            self.len -= 1;
            node
        })
    }

    unsafe fn push_front_node(&mut self, node: NonNull<Node<T>>) {
        // This method takes care not to create mutable references to whole nodes,
        // to maintain validity of aliasing pointers into `element`.
        unsafe {
            (*node.as_ptr()).next = self.head;
            (*node.as_ptr()).prev = None;
            let node = Some(node);

            match self.head {
                None => self.tail = node,
                // Not creating new mutable (unique!) references overlapping `element`.
                Some(head) => (*head.as_ptr()).prev = node,
            }

            self.head = node;
            self.len += 1;
        }
    }

    fn push_front(&mut self, elt: T) {
        let node = Box::new(Node::new(elt));
        let node_ptr = NonNull::from(Box::leak(node));
        // SAFETY: node_ptr is a unique pointer to a node we boxed with self.alloc and leaked
        unsafe {
            self.push_front_node(node_ptr);
        }
    }

    fn pop_front_node(&mut self) -> Option<Box<Node<T>>> {
        // This method takes care not to create mutable references to whole nodes,
        // to maintain validity of aliasing pointers into `element`.
        self.head.map(|node| unsafe {
            let node = Box::from_raw(node.as_ptr());
            self.head = node.next;

            match self.head {
                None => self.tail = None,
                // Not creating new mutable (unique!) references overlapping `element`.
                Some(head) => (*head.as_ptr()).prev = None,
            }

            self.len -= 1;
            node
        })
    }

    fn pop_front(&mut self) -> Option<T> {
        self.pop_front_node().map(|node| node.elem)
    }

    unsafe fn unlink_node(&mut self, mut node_ptr: NonNull<Node<T>>) -> Box<Node<T>> {
        {
            let node = unsafe { node_ptr.as_mut() }; // this one is ours now, we can create an &mut.

            // Not creating new mutable (unique!) references overlapping `element`.
            match node.prev {
                Some(prev) => unsafe { (*prev.as_ptr()).next = node.next },
                // this node is the head node
                None => self.head = node.next,
            };

            match node.next {
                Some(next) => unsafe { (*next.as_ptr()).prev = node.prev },
                // this node is the tail node
                None => self.tail = node.prev,
            };

            self.len -= 1;
        }

        unsafe { Box::from_raw(node_ptr.as_ptr()) }
    }

    fn remove_node(&mut self, node: NonNull<Node<T>>) -> T {
        unsafe { self.unlink_node(node) }.elem
    }
}

type Key = i32;
type Val = i32;

pub struct LRUCache {
    capacity: usize,
    cache: HashMap<Key, Box<Node<(Key, Val)>>>,
    lru_order: LinkedList<(Key, Val)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    fn new(capacity: i32) -> Self {
        todo!()
    }

    fn get(&self, key: i32) -> i32 {
        0
    }

    fn put(&self, key: i32, value: i32) {}
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linked_list_1() {
        let mut list = LinkedList::new();
        assert_eq!(list.len, 0);
        list.push_back(0);
        list.push_back(1);
        list.push_back(2);
        assert_eq!(list.len, 3);
        let zero = list.pop_front().unwrap();
        assert_eq!(zero, 0);
        assert_eq!(list.len, 2);
        let one = list.pop_front().unwrap();
        assert_eq!(one, 1);
        assert_eq!(list.len, 1);
        let tail = list.tail.unwrap();
        let two = list.remove_node(tail);
        assert_eq!(two, 2);
        assert_eq!(list.len, 0);
    }

    #[test]
    fn linked_list_2() {
        let mut list = LinkedList::new();
        assert_eq!(list.len, 0);
        list.push_back(0);
        list.push_back(1);
        assert_eq!(list.len, 2);
        let tail = list.tail.unwrap();
        let removed = list.remove_node(tail);
        assert_eq!(removed, 1);
        assert_eq!(list.len, 1);
    }

    #[test]
    fn linked_list_3() {
        let mut list = LinkedList::new();
        assert_eq!(list.len, 0);
        list.push_back(0);
        list.push_back(1);
        list.push_back(2);
        assert_eq!(list.len, 3);
        let head_next = unsafe { Box::from_raw(list.head.unwrap().as_ptr()) }
            .next
            .unwrap();
        let removed = list.remove_node(head_next);
        assert_eq!(removed, 1);
        assert_eq!(list.len, 2);
    }

    #[test]
    fn example1() {
        // let lru_cache = LRUCache::new(2);
        // lru_cache.put(1, 1); // cache is {1=1}
        // lru_cache.put(2, 2); // cache is {1=1, 2=2}
        // assert_eq!(lru_cache.get(1), 1);
        // lru_cache.put(3, 3); // LRU key was 2, evicts key 2, cache is {1=1, 3=3}
        // assert_eq!(lru_cache.get(2), -1);
        // lru_cache.put(4, 4); // LRU key was 1, evicts key 1, cache is {4=4, 3=3}
        // assert_eq!(lru_cache.get(1), -1);
        // assert_eq!(lru_cache.get(3), 3);
        // assert_eq!(lru_cache.get(4), 4);
    }
}
