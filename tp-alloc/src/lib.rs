const ALLOC_BLOCK_SIZE: u8 = 64;
const ALLOC_BLOCK_NUM: usize = 16384;

struct MyAllocData {
    tab_alloc : [bool;ALLOC_BLOCK_NUM]
}

struct MyAlloc {
    memory : [u8;ALLOC_BLOCK_NUM],
    data : MyAllocData
}

impl MyAlloc {
    pub const fn new() -> Self {
        MyAlloc { memory: [0;ALLOC_BLOCK_NUM], data: MyAllocData { tab_alloc: [true;ALLOC_BLOCK_NUM] } }
    }
}

impl MyAllocData {
    ///Set the num_blocks after first_block in the state input
    fn mark_blocks(&mut self, first_block: usize, num_blocks: usize, state: bool) {

        for i in first_block..first_block+num_blocks {
            self.tab_alloc[i] = state;
        }
    }

    ///Find num_block free consecutive blocks or return None otherwise
    fn find_blocks(&self, num_blocks: usize) -> Option<usize> {
        let mut cpt: usize = 0;
        for i in 0..ALLOC_BLOCK_NUM {
            if (!self.tab_alloc[i]) {
                cpt = cpt+1;
                if (cpt == num_blocks) {
                    return Some(i);
                }
            } else {
                cpt = 0; //reinit cpt if a block is occupied
            }
        }
        None
    }
}