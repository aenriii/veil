use core::borrow::Borrow;


/// [StackedList] is a linked list of variable length that is stored on the stack.
/// 
/// The primary use for this, currently, is to store the list of memory regions that
/// are available for allocation so that we aren't allocating on the heap to allocate
/// on the heap.
/// 
/// As long as we can iterate over the list without consuming it, we can use it effectively.
pub struct StackedList<'a, T> {
    head: &'a mut Node<'a, T>,
    len: usize
}
struct Node<'a, T> {
    data: T,
    next: Option<&'a mut Node<'a, T>>
}
impl<'a, T> StackedList<'a, T> {
    pub fn new(head: &'a mut Node<'a, T>) -> Self {
        Self {
            head,
            len: 1
        }
    }
    pub fn push(&mut self, data: T) {
        let mut node = &mut self.head;
        while let Some(next) = &mut node.next {
            node = next;
        }
        (node.next) = Some(Node::new(data));
        self.len += 1;
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn iter(&'a self) -> Iter<'a, T> {
        Iter {
            current: Some(self.head)
        }
    }
}
impl<'a, T> Node<'a, T> {
    pub fn new(data: T) -> &'a mut Self {
        let mut node = Node {
            data,
            next: None
        };
        &mut node
    }
}
pub struct Iter<'a, T> {
    current: Option<&'a Node<'a, T>>
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current?;
        self.current = current.next.map(|x| x.borrow());
        Some(&current.data)

    }
}
