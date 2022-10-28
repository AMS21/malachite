use core::hash::Hash;
use malachite_base::chars::exhaustive::exhaustive_chars;
use malachite_base::chars::random::graphic_weighted_random_char_inclusive_range;
use malachite_base::num::random::geometric::geometric_random_unsigneds;
use malachite_base::num::random::random_primitive_ints;
use malachite_base::random::{Seed, EXAMPLE_SEED};
use malachite_base::test_util::vecs::random::random_vecs_helper_helper;
use malachite_base::vecs::random::random_ordered_unique_vecs_min_length;
use std::fmt::Debug;

fn random_ordered_unique_vecs_min_length_helper<
    T: Clone + Debug + Eq + Hash + Ord,
    I: Clone + Iterator<Item = T>,
>(
    min_length: u64,
    xs_gen: &dyn Fn(Seed) -> I,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
    expected_values: &[&[T]],
    expected_common_values: &[(&[T], usize)],
    expected_median: (&[T], Option<&[T]>),
) {
    random_vecs_helper_helper(
        random_ordered_unique_vecs_min_length(
            EXAMPLE_SEED,
            min_length,
            xs_gen,
            mean_length_numerator,
            mean_length_denominator,
        ),
        expected_values,
        expected_common_values,
        expected_median,
    );
}

#[test]
fn test_random_ordered_unique_vecs_min_length() {
    random_ordered_unique_vecs_min_length_helper(
        0,
        &random_primitive_ints::<u8>,
        4,
        1,
        &[
            &[],
            &[11, 32, 38, 85, 134, 136, 162, 166, 177, 200, 203, 217, 223, 235],
            &[30, 90, 218, 234],
            &[9, 106, 204, 216],
            &[151],
            &[],
            &[78, 91, 97, 213, 253],
            &[39, 191],
            &[170, 175, 232, 233],
            &[],
            &[2, 22, 35, 114, 198, 217],
            &[],
            &[],
            &[17, 25, 32, 65, 79, 114, 121, 144, 148, 173, 222],
            &[52, 69, 73, 91, 115, 137, 153, 178],
            &[],
            &[34, 95, 112],
            &[],
            &[106, 130, 167, 168, 197],
            &[86, 101, 122, 150, 172, 177, 207, 218, 221],
        ],
        &[
            (&[], 199913),
            (&[7], 705),
            (&[25], 689),
            (&[184], 681),
            (&[213], 681),
            (&[255], 676),
            (&[215], 675),
            (&[54], 673),
            (&[122], 672),
            (&[207], 672),
        ],
        (&[27, 31, 211, 238], Some(&[27, 31, 247, 251])),
    );
    random_ordered_unique_vecs_min_length_helper(
        3,
        &random_primitive_ints::<u8>,
        7,
        1,
        &[
            &[11, 85, 136],
            &[9, 30, 32, 38, 90, 106, 134, 162, 166, 177, 200, 203, 217, 218, 223, 234, 235],
            &[78, 97, 151, 204, 213, 216, 253],
            &[39, 91, 170, 175, 191, 232, 233],
            &[2, 22, 35, 217],
            &[17, 114, 198],
            &[25, 32, 65, 114, 121, 144, 173, 222],
            &[52, 73, 79, 115, 148],
            &[34, 69, 91, 112, 137, 153, 178],
            &[95, 106, 167],
            &[86, 122, 130, 150, 168, 172, 177, 197, 207],
            &[101, 218, 221],
            &[9, 74, 115],
            &[40, 48, 52, 97, 104, 109, 123, 133, 159, 196, 201, 235, 247, 250],
            &[7, 11, 24, 43, 68, 103, 112, 157, 190, 216, 217],
            &[84, 135, 211],
            &[29, 55, 65, 89, 191, 206],
            &[9, 51, 79],
            &[3, 20, 22, 34, 62, 114, 118, 148],
            &[23, 32, 47, 50, 120, 166, 176, 177, 194, 204, 238, 248],
        ],
        &[
            (&[5, 128, 142], 4),
            (&[137, 145, 160], 4),
            (&[2, 4, 52], 3),
            (&[1, 5, 192], 3),
            (&[12, 41, 58], 3),
            (&[2, 95, 171], 3),
            (&[20, 86, 94], 3),
            (&[21, 43, 50], 3),
            (&[3, 81, 122], 3),
            (&[31, 54, 79], 3),
        ],
        (&[26, 138, 167], Some(&[26, 138, 167, 173, 211])),
    );
    random_ordered_unique_vecs_min_length_helper(
        0,
        &|seed| geometric_random_unsigneds::<u32>(seed, 32, 1),
        4,
        1,
        &[
            &[],
            &[1, 9, 12, 14, 16, 17, 19, 21, 41, 42, 68, 79, 124, 141],
            &[0, 1, 10, 99],
            &[2, 12, 36, 77],
            &[1],
            &[],
            &[1, 5, 9, 19, 103],
            &[6, 7],
            &[15, 18, 51, 159],
            &[],
            &[2, 26, 40, 52, 64, 75],
            &[],
            &[],
            &[3, 4, 5, 7, 30, 31, 34, 43, 49, 51, 67],
            &[1, 14, 16, 24, 29, 41, 47, 52],
            &[],
            &[11, 13, 62],
            &[],
            &[3, 14, 42, 47, 109],
            &[5, 13, 16, 25, 37, 41, 42, 86, 96],
        ],
        &[
            (&[], 199913),
            (&[0], 4861),
            (&[1], 4593),
            (&[2], 4498),
            (&[3], 4405),
            (&[4], 4330),
            (&[5], 4078),
            (&[6], 4050),
            (&[7], 3858),
            (&[8], 3848),
        ],
        (
            &[3, 9, 14, 22, 36, 56, 107],
            Some(&[3, 9, 14, 22, 42, 54, 73, 150]),
        ),
    );
    random_ordered_unique_vecs_min_length_helper(
        3,
        &|seed| geometric_random_unsigneds::<u32>(seed, 32, 1),
        7,
        1,
        &[
            &[1, 14, 42],
            &[0, 1, 9, 10, 12, 16, 17, 19, 21, 36, 41, 68, 77, 79, 99, 124, 141],
            &[1, 2, 5, 9, 12, 19, 103],
            &[6, 7, 15, 18, 51, 52, 159],
            &[2, 40, 64, 75],
            &[26, 34, 67],
            &[4, 5, 7, 30, 31, 43, 49, 51],
            &[3, 14, 16, 24, 47],
            &[1, 11, 13, 29, 41, 52, 62],
            &[3, 47, 109],
            &[13, 14, 16, 25, 37, 41, 42, 86, 96],
            &[5, 20, 42],
            &[2, 74, 82],
            &[3, 6, 7, 11, 17, 20, 36, 45, 56, 66, 76, 80, 89, 127],
            &[1, 6, 10, 13, 19, 23, 25, 32, 41, 43, 97],
            &[7, 41, 134],
            &[9, 10, 25, 26, 47, 105],
            &[68, 94, 109],
            &[1, 3, 9, 13, 28, 43, 44, 84],
            &[0, 4, 5, 6, 7, 13, 31, 32, 37, 42, 50, 75],
        ],
        &[
            (&[0, 2, 5], 42),
            (&[0, 1, 8], 39),
            (&[0, 3, 4], 38),
            (&[1, 3, 9], 38),
            (&[0, 1, 7], 35),
            (&[0, 2, 8], 34),
            (&[1, 2, 12], 34),
            (&[0, 1, 2], 33),
            (&[1, 2, 3], 33),
            (&[1, 3, 4], 33),
        ],
        (
            &[3, 8, 14, 19, 25, 36, 52, 64, 71],
            Some(&[3, 8, 14, 19, 25, 38, 58, 61]),
        ),
    );
    random_ordered_unique_vecs_min_length_helper(
        0,
        &random_primitive_ints::<u8>,
        1,
        4,
        &[
            &[],
            &[],
            &[85],
            &[11],
            &[136, 200],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[134, 235],
            &[203],
            &[],
            &[38, 223],
            &[],
            &[],
            &[],
            &[],
        ],
        &[
            (&[], 800023),
            (&[162], 692),
            (&[235], 690),
            (&[90], 688),
            (&[65], 687),
            (&[249], 686),
            (&[175], 684),
            (&[108], 683),
            (&[211], 682),
            (&[237], 680),
        ],
        (&[], None),
    );
    random_ordered_unique_vecs_min_length_helper(
        3,
        &random_primitive_ints::<u8>,
        13,
        4,
        &[
            &[11, 85, 136],
            &[134, 200, 235],
            &[38, 203, 223, 235],
            &[32, 162, 177, 217],
            &[30, 90, 166, 218, 234],
            &[9, 106, 216],
            &[151, 204, 213],
            &[78, 97, 253],
            &[39, 91, 191],
            &[170, 175, 232],
            &[2, 35, 233],
            &[22, 198, 217],
            &[17, 32, 65, 114, 173],
            &[25, 121, 173, 222],
            &[79, 144, 148],
            &[52, 69, 73, 115, 137],
            &[91, 153, 178],
            &[34, 95, 112],
            &[106, 167, 197],
            &[122, 130, 168],
        ],
        &[
            (&[10, 87, 204], 6),
            (&[15, 40, 115], 6),
            (&[108, 193, 199], 6),
            (&[1, 22, 70], 5),
            (&[1, 8, 212], 5),
            (&[2, 40, 169], 5),
            (&[2, 58, 211], 5),
            (&[3, 29, 186], 5),
            (&[3, 97, 112], 5),
            (&[11, 66, 140], 5),
        ],
        (&[49, 78, 193], Some(&[49, 78, 193, 215])),
    );
    random_ordered_unique_vecs_min_length_helper(
        0,
        &|seed| {
            graphic_weighted_random_char_inclusive_range(
                seed,
                'a',
                exhaustive_chars().nth(200).unwrap(),
                1,
                1,
            )
        },
        4,
        1,
        &[
            &[],
            &['g', 'q', '³', '»', 'À', 'Á', 'Ã', 'È', 'á', 'â', 'ì', 'ñ', 'Ā', 'ą'],
            &['ª', '´', 'Ã', 'ä'],
            &['½', 'Á', 'Ï', 'ý'],
            &['j'],
            &[],
            &['u', '½', 'Â', 'Ñ', 'ï'],
            &['x', 'õ'],
            &['¡', 'Â', 'ù', 'Ċ'],
            &[],
            &['b', 'r', 's', '¬', 'Â', 'Ñ'],
            &[],
            &[],
            &['j', 'n', 't', '¬', 'º', '¿', 'Á', 'Ø', 'Þ', 'ô', 'ü'],
            &['b', 'k', '±', 'Î', 'Ü', 'æ', 'è', 'ā'],
            &[],
            &['«', '¹', 'Î'],
            &[],
            &['~', '¯', '´', 'Ý', 'â'],
            &['g', '¼', 'Ç', 'Î', 'Ü', 'Þ', 'æ', 'é', 'ö'],
        ],
        &[
            (&[][..], 199913),
            (&['Ó'], 1270),
            (&['Â'], 1249),
            (&['§'], 1244),
            (&['¿'], 1243),
            (&['õ'], 1241),
            (&['ĉ'], 1234),
            (&['¤'], 1232),
            (&['¼'], 1232),
            (&['Ì'], 1229),
        ],
        (
            &['o', 'v', '¢', '±', 'Ä', 'Ć'],
            Some(&['o', 'v', '¢', '³', 'ã']),
        ),
    );
    random_ordered_unique_vecs_min_length_helper(
        3,
        &|seed| {
            graphic_weighted_random_char_inclusive_range(
                seed,
                'a',
                exhaustive_chars().nth(200).unwrap(),
                1,
                1,
            )
        },
        7,
        1,
        &[
            &['g', 'q', 'á'],
            &['g', 'ª', '³', '´', '»', '½', 'À', 'Á', 'Ã', 'È', 'Ï', 'â', 'ä', 'ì', 'ñ', 'Ā', 'ą'],
            &['j', 'u', '½', 'Â', 'Ñ', 'ï', 'ý'],
            &['x', '¡', '¬', 'Â', 'õ', 'ù', 'Ċ'],
            &['b', 's', '¬', 'Ñ'],
            &['n', 'r', 'Â'],
            &['t', '¬', 'º', '¿', 'Ø', 'Þ', 'ô', 'ü'],
            &['j', 'k', '±', 'Á', 'è'],
            &['b', '«', '¹', 'Î', 'Ü', 'æ', 'ā'],
            &['~', '´', 'Î'],
            &['g', '¯', 'Î', 'Ý', 'Þ', 'â', 'æ', 'é', 'ö'],
            &['¼', 'Ç', 'Ü'],
            &['¡', '§', 'Ì'],
            &['d', 'm', 'z', '{', '¨', '®', '±', '¼', 'Ë', 'Ü', 'ê', 'ì', 'ý', 'þ'],
            &['x', 'ª', '½', 'À', 'Õ', 'ì', 'ï', 'û', 'ă', 'Ą', 'ċ'],
            &['¢', '«', 'Ć'],
            &['{', '¢', '½', 'È', 'ä', 'ÿ'],
            &['Ë', 'Õ', 'ê'],
            &['p', '¨', '°', 'º', 'Å', 'Ó', '×', 'ü'],
            &['d', 'k', 'o', 'v', '¥', '±', 'Ä', 'È', 'Ê', 'ß', 'æ', 'Ć'],
        ],
        &[
            (&['m', 'u', 'w'], 6),
            (&['b', 'n', 'Ã'], 6),
            (&['g', '®', 'Ý'], 6),
            (&['x', 'Ä', 'î'], 6),
            (&['º', 'Ú', '÷'], 6),
            (&['a', 'w', 'ø'], 5),
            (&['c', 'e', 'Þ'], 5),
            (&['d', 't', 'Ã'], 5),
            (&['m', 'r', 'È'], 5),
            (&['w', '{', '³'], 5),
        ],
        (
            &['o', 's', '×', 'Ý', 'Þ', 'ß', 'î', 'ù'],
            Some(&['o', 's', '×', 'à', 'ã', 'ò', 'ċ']),
        ),
    );
}

#[test]
#[should_panic]
fn random_ordered_unique_vecs_min_length_fail_1() {
    random_ordered_unique_vecs_min_length(EXAMPLE_SEED, 3, &random_primitive_ints::<u32>, 3, 1);
}

#[test]
#[should_panic]
fn random_ordered_unique_vecs_min_length_fail_2() {
    random_ordered_unique_vecs_min_length(EXAMPLE_SEED, 1, &random_primitive_ints::<u32>, 1, 0);
}

#[test]
#[should_panic]
fn random_ordered_unique_vecs_min_length_fail_3() {
    random_ordered_unique_vecs_min_length(
        EXAMPLE_SEED,
        0,
        &random_primitive_ints::<u32>,
        u64::MAX,
        u64::MAX - 1,
    );
}
