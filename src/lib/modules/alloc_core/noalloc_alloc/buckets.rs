use core::sync::atomic::AtomicUsize;
#[derive(Debug)]
pub struct BucketQueue {
    pub start: AtomicUsize,
    pub end: AtomicUsize,
}


pub static BUCKETS: [BucketQueue; super::size::BUCKETS_AVAIL.len() + 1] = [
    BucketQueue {
        start: AtomicUsize::new(0),
        end: AtomicUsize::new(0),
    },
    BucketQueue {
        start: AtomicUsize::new(0),
        end: AtomicUsize::new(0),
    },
    BucketQueue {
        start: AtomicUsize::new(0),
        end: AtomicUsize::new(0),
    },
    BucketQueue {
        start: AtomicUsize::new(0),
        end: AtomicUsize::new(0),
    },
    BucketQueue {
        start: AtomicUsize::new(0),
        end: AtomicUsize::new(0),
    },
    BucketQueue {
        start: AtomicUsize::new(0),
        end: AtomicUsize::new(0),
    },
    BucketQueue {
        start: AtomicUsize::new(0),
        end: AtomicUsize::new(0),
    },
    BucketQueue {
        start: AtomicUsize::new(0),
        end: AtomicUsize::new(0),
    },
    BucketQueue {
        start: AtomicUsize::new(0),
        end: AtomicUsize::new(0),
    },
    BucketQueue {
        start: AtomicUsize::new(0),
        end: AtomicUsize::new(0),
    },
    BucketQueue {
        start: AtomicUsize::new(0),
        end: AtomicUsize::new(0),
    },
    BucketQueue {
        start: AtomicUsize::new(0),
        end: AtomicUsize::new(0),
    },
    BucketQueue {
        start: AtomicUsize::new(0),
        end: AtomicUsize::new(0),
    },
    BucketQueue {
        start: AtomicUsize::new(0),
        end: AtomicUsize::new(0),
    },
    BucketQueue {
        start: AtomicUsize::new(0),
        end: AtomicUsize::new(0),
    },
    BucketQueue {
        start: AtomicUsize::new(0),
        end: AtomicUsize::new(0),
    },
    BucketQueue {
        start: AtomicUsize::new(0),
        end: AtomicUsize::new(0),
    }    
];