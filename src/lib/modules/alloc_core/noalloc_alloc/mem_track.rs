
/// Keeps track of memory regions without EVER making a heap allocation by using a linked list structure
/// this makes it *slightly* slower but extremely reliable
pub struct MemoryTracker<'list> {
    head: RegionNode<'list>
}

struct RegionNode<'list> {
    start: u64,
    end: u64,
    pub head: Option<&'list RegionNode<'list>>,
    pub tail: Option<&'list RegionNode<'list>>
}
impl RegionNode<'_> {
    pub const fn empty() -> Self {
        Self {
            start: 0,
            end: 0,
            head: None,
            tail: None
        }
    }

    pub fn merge(&mut self) {

    }
    pub fn put(&mut self, node: RegionNode<'_>) {

    }
    pub fn allocate(&mut self, size: usize) {

    }
    pub fn allocate_aligned(&mut self, size: usize, align: usize) {

    }
    pub fn sort(&mut self) {

    }
}