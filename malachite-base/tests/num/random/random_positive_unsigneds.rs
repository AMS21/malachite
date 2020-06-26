use malachite_base_test_util::stats::common_values_map::common_values_map;

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::random::random_positive_unsigneds;
use malachite_base::random::EXAMPLE_SEED;

fn random_positive_unsigneds_helper<T: PrimitiveUnsigned>(
    values: &[T],
    common_values: &[(T, usize)],
) {
    let xs = random_positive_unsigneds::<T>(EXAMPLE_SEED);
    assert_eq!(xs.clone().take(20).collect::<Vec<T>>(), values);
    assert_eq!(common_values_map(1_000_000, 10, xs), common_values)
}

#[allow(clippy::decimal_literal_representation)]
#[test]
fn test_random_positive_unsigneds() {
    random_positive_unsigneds_helper::<u8>(
        &[
            113, 228, 87, 188, 93, 189, 117, 151, 7, 72, 233, 12, 114, 39, 104, 228, 242, 239, 235,
            200,
        ],
        &[
            (88, 4079),
            (121, 4067),
            (47, 4057),
            (173, 4056),
            (123, 4053),
            (27, 4051),
            (183, 4048),
            (74, 4044),
            (16, 4036),
            (55, 4036),
        ],
    );
    random_positive_unsigneds_helper::<u16>(
        &[
            61297, 53988, 8279, 8892, 51293, 38333, 37493, 43415, 9735, 32584, 27625, 4620, 44658,
            44583, 9576, 31460, 63730, 59887, 21995, 23240,
        ],
        &[
            (11780, 35),
            (13255, 34),
            (8969, 33),
            (65522, 33),
            (35057, 32),
            (64313, 32),
            (8247, 31),
            (24576, 31),
            (50513, 31),
            (54829, 31),
        ],
    );
    random_positive_unsigneds_helper::<u32>(
        &[
            1816522609, 2712195812, 1399726167, 3998819004, 1939195997, 3386480061, 1210028661,
            565094807, 2421237255, 2154921800, 1999530985, 4087616012, 4147883634, 3097538087,
            4234421608, 1164671716, 2394159346, 3174951407, 130045419, 2998491848,
        ],
        &[
            (20095656, 2),
            (29107328, 2),
            (83328146, 2),
            (96543416, 2),
            (109257003, 2),
            (132308363, 2),
            (140940582, 2),
            (168698132, 2),
            (182460287, 2),
            (184573980, 2),
        ],
    );
    random_positive_unsigneds_helper::<u64>(
        &[
            11648792314704686961,
            17174796846203019351,
            14544821112490281053,
            2427063716414460533,
            9255318658858690055,
            17556177092145474537,
            13303824785927286386,
            5002226935030621544,
            13636312461848344818,
            12878424424612648427,
            13573831502926905428,
            1513424385005459611,
            2484972586252155822,
            13072300245601619293,
            4344958725064805398,
            3252798961345668310,
            10520651756201345771,
            12379844438588545665,
            6654913321726770291,
            10505868200830584967,
        ],
        &[
            (26914038281329, 1),
            (32553719576594, 1),
            (53892651831494, 1),
            (66354421349686, 1),
            (86226284907602, 1),
            (89837182726049, 1),
            (95691351770484, 1),
            (166741761063383, 1),
            (171574734234584, 1),
            (212518263578065, 1),
        ],
    );
    random_positive_unsigneds_helper::<u128>(
        &[
            316819081939861044636107404782286008177,
            44771423227283929645271838218448652381,
            323854305731529921104946490224673891847,
            92274800069126412258941638023956901490,
            237564999433439714567249110498266052850,
            27917752305106984883503397141734686804,
            241141377085303586778938998836817083310,
            60003549963171791765607325839025294358,
            228367822030979405869278360636891890411,
            193799061972845222683167418018286926963,
            186696208941218647968078823625188059421,
            33018320828004690757952445968579104952,
            24887066387352849554815992782110776358,
            79085537771456044427857440036467563654,
            19637669411666889498466854442215856999,
            237587532320755783035907621678835821469,
            254983837845695498020527357238650572551,
            272337383097469374367899988789175779695,
            105189689748742230503365861545668092951,
            258427395460299182237257690021561141080,
        ],
        &[
            (68570815139656170990830410045915, 1),
            (381682482227926990846204728028719, 1),
            (565207126752383841908924745713103, 1),
            (717866653939818807939025508430762, 1),
            (775173738585689418081884794376186, 1),
            (818497230601034032775791540657915, 1),
            (1224023028796761386468452212527255, 1),
            (1379103576141836593923341631562888, 1),
            (1765193876177447622538546939111747, 1),
            (2049979073093489039458791025727172, 1),
        ],
    );
}