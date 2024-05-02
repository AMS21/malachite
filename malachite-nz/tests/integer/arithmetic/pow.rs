// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    CheckedRoot, Parity, Pow, PowAssign, PowerOf2, Square,
};
use malachite_base::num::basic::traits::{NegativeOne, One, Two, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::{signed_unsigned_pair_gen_var_15, unsigned_gen_var_5};
use malachite_nz::integer::Integer;
use malachite_nz::platform::SignedLimb;
use malachite_nz::test_util::generators::{
    integer_gen, integer_integer_unsigned_triple_gen_var_1, integer_unsigned_pair_gen_var_2,
    integer_unsigned_unsigned_triple_gen_var_3, natural_unsigned_pair_gen_var_4,
};
use num::traits::Pow as NumPow;
use num::BigInt;
use rug::ops::Pow as RugPow;
use std::str::FromStr;

#[test]
fn test_pow() {
    let test = |s, exp, out| {
        let u = Integer::from_str(s).unwrap();

        let mut x = u.clone();
        x.pow_assign(exp);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let x = u.clone().pow(exp);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let x = (&u).pow(exp);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let x = BigInt::from_str(s).unwrap().pow(exp);
        assert_eq!(x.to_string(), out);

        let x = rug::Integer::from_str(s).unwrap().pow(u32::exact_from(exp));
        assert_eq!(x.to_string(), out);
    };
    test("0", 0, "1");
    test("1", 0, "1");
    test("2", 0, "1");
    test("1000", 0, "1");
    test("1000000000000", 0, "1");
    test("0", 1, "0");
    test("1", 1, "1");
    test("2", 1, "2");
    test("1000", 1, "1000");
    test("1000000000000", 1, "1000000000000");
    test("0", 2, "0");
    test("1", 2, "1");
    test("2", 2, "4");
    test("3", 2, "9");
    test("1000", 2, "1000000");
    test("1000000000000", 2, "1000000000000000000000000");
    test(
        "123",
        456,
        "992500687720988567008314620574696326372959408198869005198162988813828671047493990779211286\
        6142614463805542423693627187249280035274164990211814381967260156999810012079049675951763646\
        5445895625741609866209900500198407153244604778968016963028050310261417615914468729918240685\
        4878786176459769390634643579861657117309763994785076492286863414669671679101266533421349427\
        4485146389992748709248661097714611276356710167264595313219648143933987301708814041466127119\
        8500333255713096142335151414630651683065518784081203678487703002802082091236603519026256880\
        6244996817813872275740354848312715156831237421490955692604636096559777009388445806119312464\
        9516620869554031369814001163802732256625268978083813635182879531427216211122223117090171561\
        2355701347552371530013693855379834865667060014643302459100429783653966913783002290784283455\
        6282833554705299329560514844771293338811599302127586876027950885792304316616960102321873904\
        36601614145603241902386663442520160735566561",
    );
    test(
        "123",
        457,
        "122077584589681593742022698330687648143874007208460887639374047624100926538841760865842988\
        2535541579048081718114316144031661444338722293796053168981972999310976631485723110142066928\
        5249845161966218013543817761524404079849086387813066086452450188162154366757479653779943604\
        3150090699704551635048061160322983825429100971358564408551284200004369616529455783610825979\
        5761673005969108091237585315018897186991875350573545223526016721703880438110184127100333635\
        7415540990452710825507223623999570157017058810441988052453987469344656097222102232840229596\
        3168134608591106289916063646342463964290242202843387550190370239876852572154778834152675433\
        1890544366955145858487122143147736067564908084304309077127494182365547593968033443402091102\
        0319751265748941698191684344211719688477048381801126202469352863389437930395309281766466865\
        0422788527228751817535943325906869080673826714161693185751437958952453430943886092585590490\
        23701998539909198753993559603429979770474687003",
    );
    test(
        "10",
        100,
        "100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        00000000000",
    );
    test(
        "10",
        101,
        "100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000",
    );
    test("2", 100, "1267650600228229401496703205376");
    test(
        "12345678987654321",
        5,
        "286797196211272153445658333952148540084053148773966942500383143133144940612575601",
    );
    test(
        "12345678987654321",
        6,
        "354070611898367606555445954656918550154832595750628623501854823341167403514406003590115726\
        6821921",
    );

    test("-1", 0, "1");
    test("-2", 0, "1");
    test("-1000", 0, "1");
    test("-1000000000000", 0, "1");
    test("-1", 1, "-1");
    test("-2", 1, "-2");
    test("-1000", 1, "-1000");
    test("-1000000000000", 1, "-1000000000000");
    test("-1", 2, "1");
    test("-2", 2, "4");
    test("-3", 2, "9");
    test("-1000", 2, "1000000");
    test("-1000000000000", 2, "1000000000000000000000000");
    test(
        "-123",
        456,
        "992500687720988567008314620574696326372959408198869005198162988813828671047493990779211286\
        6142614463805542423693627187249280035274164990211814381967260156999810012079049675951763646\
        5445895625741609866209900500198407153244604778968016963028050310261417615914468729918240685\
        4878786176459769390634643579861657117309763994785076492286863414669671679101266533421349427\
        4485146389992748709248661097714611276356710167264595313219648143933987301708814041466127119\
        8500333255713096142335151414630651683065518784081203678487703002802082091236603519026256880\
        6244996817813872275740354848312715156831237421490955692604636096559777009388445806119312464\
        9516620869554031369814001163802732256625268978083813635182879531427216211122223117090171561\
        2355701347552371530013693855379834865667060014643302459100429783653966913783002290784283455\
        6282833554705299329560514844771293338811599302127586876027950885792304316616960102321873904\
        36601614145603241902386663442520160735566561",
    );
    test(
        "-123",
        457,
        "-12207758458968159374202269833068764814387400720846088763937404762410092653884176086584298\
        8253554157904808171811431614403166144433872229379605316898197299931097663148572311014206692\
        8524984516196621801354381776152440407984908638781306608645245018816215436675747965377994360\
        4315009069970455163504806116032298382542910097135856440855128420000436961652945578361082597\
        9576167300596910809123758531501889718699187535057354522352601672170388043811018412710033363\
        5741554099045271082550722362399957015701705881044198805245398746934465609722210223284022959\
        6316813460859110628991606364634246396429024220284338755019037023987685257215477883415267543\
        3189054436695514585848712214314773606756490808430430907712749418236554759396803344340209110\
        2031975126574894169819168434421171968847704838180112620246935286338943793039530928176646686\
        5042278852722875181753594332590686908067382671416169318575143795895245343094388609258559049\
        023701998539909198753993559603429979770474687003",
    );
    test(
        "-10",
        100,
        "100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        00000000000",
    );
    test(
        "-10",
        101,
        "-10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        0000000000000",
    );
    test("-2", 100, "1267650600228229401496703205376");
    test(
        "-12345678987654321",
        5,
        "-286797196211272153445658333952148540084053148773966942500383143133144940612575601",
    );
    test(
        "-12345678987654321",
        6,
        "354070611898367606555445954656918550154832595750628623501854823341167403514406003590115726\
        6821921",
    );
}

#[test]
fn pow_properties() {
    integer_unsigned_pair_gen_var_2().test_properties(|(x, exp)| {
        let power = (&x).pow(exp);
        assert!(power.is_valid());

        let power_alt = x.clone().pow(exp);
        assert!(power_alt.is_valid());
        assert_eq!(power_alt, power);

        let mut power_alt = x.clone();
        power_alt.pow_assign(exp);
        assert!(power_alt.is_valid());
        assert_eq!(power_alt, power);

        let power_of_neg = (-&x).pow(exp);
        if exp.even() {
            assert_eq!(power_of_neg, power);
        } else {
            assert_eq!(power_of_neg, -&power);
        }
        if exp > 0 && (x >= 0 || exp.odd()) {
            assert_eq!((&power).checked_root(exp).as_ref(), Some(&x));
        }

        assert_eq!(Integer::from(&BigInt::from(&x).pow(exp)), power);
        assert_eq!(
            Integer::from(&rug::Integer::from(&x).pow(u32::exact_from(exp))),
            power
        );
    });

    integer_gen().test_properties(|x| {
        assert_eq!((&x).pow(0), 1);
        assert_eq!((&x).pow(1), x);
        assert_eq!((&x).pow(2), x.square());
    });

    unsigned_gen_var_5().test_properties(|exp| {
        assert_eq!(Integer::ZERO.pow(exp), u64::from(exp == 0));
        assert_eq!(Integer::ONE.pow(exp), 1);
        assert_eq!(Integer::TWO.pow(exp), Integer::power_of_2(exp));

        assert_eq!(
            Integer::NEGATIVE_ONE.pow(exp),
            if exp.even() { 1 } else { -1 }
        );
    });

    integer_integer_unsigned_triple_gen_var_1().test_properties(|(x, y, exp)| {
        assert_eq!((&x * &y).pow(exp), x.pow(exp) * y.pow(exp));
    });

    integer_unsigned_unsigned_triple_gen_var_3().test_properties(|(x, e, f)| {
        assert_eq!((&x).pow(e + f), (&x).pow(e) * (&x).pow(f));
        assert_eq!((&x).pow(e * f), x.pow(e).pow(f));
    });

    natural_unsigned_pair_gen_var_4().test_properties(|(x, exp)| {
        assert_eq!((&x).pow(exp), Integer::from(x).pow(exp));
    });

    signed_unsigned_pair_gen_var_15::<SignedLimb>().test_properties(|(x, exp)| {
        assert_eq!(Pow::pow(x, exp), Integer::from(x).pow(exp));
    });
}
