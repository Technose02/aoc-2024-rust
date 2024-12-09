struct BlocksToMoveIterator {
    files: Vec<usize>,
    free_space: usize,
}

impl BlocksToMoveIterator {
    pub fn new(files: Vec<usize>, free_space: usize) -> Self {
        BlocksToMoveIterator { files, free_space }
    }
}

impl Iterator for BlocksToMoveIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.free_space > 0 {
            loop {
                let last_block = self.files.pop();
                if let Some(blocksize) = last_block {
                    if blocksize > 0 {
                        self.free_space -= 1;
                        self.files.push(blocksize - 1);
                        return Some(self.files.len() - 1); // the 'file-id'
                    }
                }
            }
        }
        None
    }
}

#[allow(unused_variables)]
pub fn part1(input: &str) -> usize {
    let mut it = input.chars().map(|c| c.to_digit(10).unwrap() as usize);
    let mut files = Vec::<usize>::new();
    let mut gaps = Vec::<usize>::new();
    let mut total_size = 0;
    let mut total_freespace = 0_usize;

    while let Some(blocksize) = it.next() {
        total_size += blocksize;
        files.push(blocksize);

        if let Some(gap) = it.next() {
            total_freespace += gap;
            gaps.push(gap);
        }
    }

    let mut btmiter = BlocksToMoveIterator::new(files.clone(), total_freespace);

    let mut gaps = gaps.iter().copied();
    let mut files = files.iter().copied();

    let mut id = 0;
    let mut k = 0;
    let mut sum = 0;
    let mut space_occupied = 0_usize;

    loop {
        let blocksize = files.next().unwrap();
        for i in k..k + blocksize {
            sum += id * i;
            space_occupied += 1;
            if space_occupied >= total_size {
                return sum;
            }
        }
        k += blocksize;
        id += 1;

        let gapsize = gaps.next().unwrap();
        for i in k..k + gapsize {
            sum += btmiter.next().unwrap() * i;
            space_occupied += 1;
            if space_occupied >= total_size {
                return sum;
            }
        }
        k += gapsize;
    }
}
