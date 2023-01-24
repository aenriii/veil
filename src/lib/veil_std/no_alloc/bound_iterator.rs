use core::{mem::MaybeUninit, borrow::BorrowMut};


pub struct BoundedIter<T, const N: usize> {
    index: usize,
    capacity: usize,
    items: [MaybeUninit<T>; N]
}
impl <T, const N: usize> Iterator for BoundedIter<T, N> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.capacity {
            unsafe {
                let item = self.items.get_unchecked(self.index).as_ptr().read();
                self.index += 1;
                Some(item)
            }
        } else {
            None
        }
    }
}
impl <T : Copy, const N: usize> BoundedIter<T, N> {
    pub fn new() -> Self {
        Self {
          index: 0,
          capacity: 0,
          items: [MaybeUninit::uninit(); N],
        }
      }
    
    pub fn push(&mut self, x: T) -> Result<(), ()> {
        if (self.capacity + 1) > N {
          return Err(());
        }
        self.items[self.capacity].write(x);
        self.capacity += 1;
        Ok(())
      }
}