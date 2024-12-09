use std::collections::HashMap;

#[allow(unused_variables)]
pub fn part2(input: &str) -> usize {
    let mut it = input.chars().map(|c| c.to_digit(10).unwrap() as usize);
    let mut files = Vec::<(usize, usize, usize)>::new();
    let mut gaps = Vec::new();

    let mut id = 0;
    let mut pos = 0;
    while let Some(blocksize) = it.next() {
        files.push((pos, id, blocksize));
        id += 1;
        pos += blocksize;

        if let Some(gapsize) = it.next() {
            gaps.push((pos, gapsize));
            pos += gapsize;
        }
    }

    let mut files_by_position_after_move = HashMap::<usize, (usize, usize)>::new();

    // try to move each file exactly once
    for &(file_pos, file_id, file_size) in files.iter().rev() {
        for i in 0..gaps.len() {
            let (gap_pos, gap_size) = gaps[i];
            if gap_pos >= file_pos {
                //file remains in place
                files_by_position_after_move.insert(file_pos, (file_id, file_size));
                break;
            }
            if gap_size >= file_size {
                //move file from file_pos to gap_pos
                files_by_position_after_move.insert(gap_pos, (file_id, file_size));

                if gap_size > file_size {
                    gaps[i] = (gap_pos + file_size, gap_size - file_size);
                } else {
                    gaps.remove(i);
                }
                gaps.push((file_pos, file_size));
                break;
            }
        }
    }

    let mut sum = 0;
    for (pos, (id, len)) in files_by_position_after_move {
        for i in pos..pos + len {
            sum += i * id;
        }
    }

    sum
}
