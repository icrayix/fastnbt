use bit_field::BitArray;

/// Expand blockstate data so each block is an element of a `Vec`.
///
/// This requires the number of items in the palette of the section the blockstates came from. This is because
/// blockstate is packed with as few bits as possible. If the maximum index in the palette fits in 5 bits, then
/// every 5 bits of the blockstates will represent a block. Blocks bleed into one another, so remainder bits
/// are tracked and handled for you.
///
/// This works for Minecraft 1.15. This format due to change in 1.16 so that bits do not bleed into other longs.
/// This function will not work for 1.16 blockstates yet.
pub fn expand_blockstates(state: &[i64], palette_len: usize) -> Vec<u16> {
    expand_generic(state, bits_per_block(palette_len))
}

/// Expand heightmap data. This is equivalent to `expand_generic(data, 9)`.
pub fn expand_heightmap(data: &[i64]) -> Vec<u16> {
    expand_generic(data, 9)
}

/// Expand data into individual items. Currently a copy of data is made here to convert to unsigned integers
/// to make bit operations more tractable.
pub fn expand_generic(data: &[i64], bits_per_item: usize) -> Vec<u16> {
    let bits = bits_per_item;
    let mut result: Vec<u16> = vec![0; (data.len() * 64) / bits];

    // Unfortunely make a copy here in order to treat the data as u64 rather than i64.
    // At some point we will change the parser to let us take the data as u64 rather than i64.
    let copy: Vec<_> = data.iter().map(|i| *i as u64).collect();

    for i in 0..result.len() {
        let begin = i * bits;
        let end = begin + bits;
        result[i] = copy.get_bits(begin..end) as u16;
    }

    result
}

/// Get the number of bits that will be used in `Blockstates` per block.
///
/// See `anvil::expand_blockstates` for more information.
pub fn bits_per_block(palette_len: usize) -> usize {
    if palette_len < 16 {
        4
    } else {
        std::cmp::max((palette_len as f64).log2().ceil() as usize, 4)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nether_heightmap_v1_15_2() {
        let input: Vec<i64> = vec![
            2310355422147575936,
            1155177711073787968,
            577588855536893984,
            288794427768446992,
            144397213884223496,
            72198606942111748,
            36099303471055874,
            -9205322385119247871,
            4620710844295151872,
            2310355422147575936,
            1155177711073787968,
            577588855536893984,
            288794427768446992,
            144397213884223496,
            72198606942111748,
            36099303471055874,
            -9205322385119247871,
            4620710844295151872,
            2310355422147575936,
            1155177711073787968,
            577588855536893984,
            288794427768446992,
            144397213884223496,
            72198606942111748,
            36099303471055874,
            -9205322385119247871,
            4620710844295151872,
            2310355422147575936,
            1155177711073787968,
            577588855536893984,
            288794427768446992,
            144397213884223496,
            72198606942111748,
            36099303471055874,
            -9205322385119247871,
            4620710844295151872,
        ];

        let actual = expand_heightmap(&input[..]);
        assert_eq!(&[128; 16 * 16][..], actual.as_slice());
    }

    #[test]
    fn heightmap_overworld_v1_15_2() {
        let input: Vec<i64> = vec![
            1299610109330100808,
            649787462479005732,
            329397330866873490,
            -9060925171218247159,
            4692909455540619556,
            2346453626107004050,
            -8050144124289646015,
            5198158688002654496,
            2599149849916022926,
            -7941846763497811896,
            649769835865982755,
            -1985452877601561582,
            8230641191400739272,
            4692909451237263588,
            2057661397361812594,
            -7906029485971705287,
            5126101092889936160,
            2599079343463931022,
            -7941846763497811896,
            -3970923381816146141,
            -6606172535203224687,
            8230641191400739144,
            2960142884643391716,
            2057660297841779794,
            -8483335214034816455,
            5126100816936184084,
            2526951243511307406,
            -7941882016858338234,
            -8591634191684319453,
            -4295817113055649007,
            7075463488933695880,
            3537731740163475652,
            1768865870081737826,
            -8338939101813906895,
            5053902485947822360,
            2526951242973911180,
        ];

        let actual = expand_heightmap(&input[..]);
        assert_eq!(
            &[
                72, 73, 72, 72, 72, 73, 72, 72, 72, 72, 72, 72, 72, 72, 72, 73, 72, 72, 72, 72, 73,
                72, 72, 72, 73, 72, 72, 72, 72, 73, 72, 73, 73, 72, 72, 72, 73, 72, 72, 72, 71, 72,
                72, 72, 72, 72, 72, 73, 72, 72, 72, 72, 72, 73, 71, 71, 72, 71, 72, 72, 72, 72, 72,
                72, 72, 72, 72, 72, 71, 71, 71, 71, 71, 71, 71, 71, 71, 72, 72, 72, 72, 72, 72, 72,
                71, 71, 71, 71, 71, 71, 71, 71, 71, 71, 72, 72, 72, 72, 72, 72, 71, 71, 71, 71, 72,
                71, 71, 71, 71, 71, 72, 72, 72, 73, 72, 72, 71, 71, 71, 71, 71, 71, 71, 71, 71, 71,
                71, 72, 72, 72, 72, 72, 71, 71, 71, 71, 71, 71, 71, 71, 71, 71, 71, 71, 69, 69, 69,
                69, 69, 69, 69, 71, 71, 71, 71, 71, 71, 71, 71, 71, 69, 69, 69, 69, 69, 69, 69, 71,
                71, 71, 71, 71, 71, 71, 71, 71, 69, 69, 69, 69, 69, 69, 70, 71, 71, 71, 71, 71, 72,
                70, 70, 70, 70, 70, 70, 70, 70, 70, 71, 71, 71, 71, 71, 71, 70, 70, 70, 70, 70, 70,
                70, 70, 70, 70, 70, 71, 71, 71, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70,
                70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70,
                70, 70, 70, 70
            ][..],
            &actual[..]
        );
    }

    #[test]
    fn size_one_palette_still_requires_one_bit() {
        // With a palette size of 1, we don't really need to store the
        // blockstates (it will be all zeroes), Minecraft does anyway,
        // storing each block as a single bit.
        assert_eq!(4, bits_per_block(1));
    }

    #[test]
    fn palette_size_checks() {
        assert_eq!(4, bits_per_block(2));

        assert_eq!(4, bits_per_block(3));
        assert_eq!(4, bits_per_block(4));

        assert_eq!(4, bits_per_block(5));
        assert_eq!(4, bits_per_block(7));
        assert_eq!(4, bits_per_block(8));

        assert_eq!(4, bits_per_block(9));
        assert_eq!(4, bits_per_block(16));
        assert_eq!(10, bits_per_block(1 << 10));
    }
}
