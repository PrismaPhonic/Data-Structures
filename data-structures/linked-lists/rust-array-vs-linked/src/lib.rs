#![feature(test)]

extern crate test;



#[cfg(test)]
mod tests {
    use test::Bencher;
    use std::collections::LinkedList;

    #[bench]
    fn bench_vector_remove_front(b: &mut Bencher) {
        let mut vector = Vec::new();
        
        for i in 0..100_000 {
            vector.push(i);
        }

        b.iter(|| {
            vector.pop();
        })
    }

    #[bench]
    fn bench_linked_remove_front(b: &mut Bencher) {
        let mut linked_list = LinkedList::new();
        
        for i in 0..100_000 {
            linked_list.push_back(i);
        }


        b.iter(|| {
            linked_list.pop_front();
        })
    }
}
