use std::str::FromStr;

use malachite_base::num::arithmetic::traits::{Pow, PowAssign};
use malachite_base::num::conversion::traits::ExactFrom;
use num::traits::Pow as NumPow;
use num::BigInt;
use rug::ops::Pow as RugPow;

use malachite_nz::integer::Integer;

#[test]
fn test_pow() {
    let test = |u, exp, out| {
        let mut x = Integer::from_str(u).unwrap();
        x.pow_assign(exp);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let x = Integer::from_str(u).unwrap().pow(exp);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let x = (&Integer::from_str(u).unwrap()).pow(exp);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let x = BigInt::from_str(u).unwrap().pow(exp);
        assert_eq!(x.to_string(), out);

        let x = rug::Integer::from_str(u).unwrap().pow(u32::exact_from(exp));
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
        36601614145603241902386663442520160735566561"
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
        23701998539909198753993559603429979770474687003"
    );
    test(
        "10",
        100,
        "100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        00000000000"
    );
    test(
        "10",
        101,
        "100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000"
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
        6821921"
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
        36601614145603241902386663442520160735566561"
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
        023701998539909198753993559603429979770474687003"
    );
    test(
        "-10",
        100,
        "100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        00000000000"
    );
    test(
        "-10",
        101,
        "-10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        0000000000000"
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
        6821921"
    );
}