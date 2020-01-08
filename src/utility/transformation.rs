pub fn transpose(input: &[u8], blocks: usize) -> Vec<Vec<u8>> {
    let chunked = input.chunks_exact(blocks);

    let mut output: Vec<Vec<u8>> = Vec::with_capacity(input.len() / blocks);

    for _ in 0..blocks {
        output.push(vec![]);
    }

    // For each chunk of the expected block size...
    for chunk in chunked {
        // We place each item in order in the right vector in the output vectors
        // vector. so [1, 2, 3, 4, 5, 6, 7, 8] with 2 blocks becomes
        // [[1, 3, 5, 7], [2, 4, 6, 8]]
        for (i, &item) in chunk.iter().enumerate() {
            output[i].push(item);
        }
    }

    output
}
