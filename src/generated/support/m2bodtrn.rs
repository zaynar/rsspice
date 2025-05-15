//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const MAXL: i32 = 32;
pub const MAXP: i32 = 100;
const BEG1: i32 = 1;
const END1: i32 = (BEG1 + 9);
const BEG2: i32 = (END1 + 1);
const END2: i32 = (BEG2 + 9);
const BEG3: i32 = (END2 + 1);
const END3: i32 = (BEG3 + 9);
const BEG4: i32 = (END3 + 1);
const END4: i32 = (BEG4 + 9);
const BEG5: i32 = (END4 + 1);
const END5: i32 = (BEG5 + 9);
const BEG6: i32 = (END5 + 1);
const END6: i32 = (BEG6 + 9);
const BEG7: i32 = (END6 + 1);
const END7: i32 = (BEG7 + 9);
const BEG8: i32 = (END7 + 1);
const END8: i32 = (BEG8 + 9);
const BEG9: i32 = (END8 + 1);
const END9: i32 = (BEG9 + 15);
const BEG10: i32 = (END9 + 1);
const END10: i32 = (BEG10 + 9);
const BEG11: i32 = (END10 + 1);
const END11: i32 = (BEG11 + 9);
const BEG12: i32 = (END11 + 1);
const END12: i32 = (BEG12 + 9);
const BEG13: i32 = (END12 + 1);
const END13: i32 = (BEG13 + 7);
const BEG14: i32 = (END13 + 1);
const END14: i32 = (BEG14 + 9);
const BEG15: i32 = (END14 + 1);
const END15: i32 = (BEG15 + 9);
const BEG16: i32 = (END15 + 1);
const END16: i32 = (BEG16 + 9);
const BEG17: i32 = (END16 + 1);
const END17: i32 = (BEG17 + 9);
const BEG18: i32 = (END17 + 1);
const END18: i32 = (BEG18 + 9);
const BEG19: i32 = (END18 + 1);
const END19: i32 = (BEG19 + 9);
const BEG20: i32 = (END19 + 1);
const END20: i32 = (BEG20 + 9);
const BEG21: i32 = (END20 + 1);
const END21: i32 = (BEG21 + 9);
const BEG22: i32 = (END21 + 1);
const END22: i32 = (BEG22 + 9);
const BEG23: i32 = (END22 + 1);
const END23: i32 = (BEG23 + 9);
const BEG24: i32 = (END23 + 1);
const END24: i32 = (BEG24 + 9);
const BEG25: i32 = (END24 + 1);
const END25: i32 = (BEG25 + 9);
const BEG26: i32 = (END25 + 1);
const END26: i32 = (BEG26 + 9);
const BEG27: i32 = (END26 + 1);
const END27: i32 = (BEG27 + 9);
const BEG28: i32 = (END27 + 1);
const END28: i32 = (BEG28 + 9);
const BEG29: i32 = (END28 + 1);
const END29: i32 = (BEG29 + 9);
const BEG30: i32 = (END29 + 1);
const END30: i32 = (BEG30 + 9);
const BEG31: i32 = (END30 + 1);
const END31: i32 = (BEG31 + 10);
const NPERM: i32 = END31;
const MAXE: i32 = (MAXP + NPERM);

struct SaveVars {
    NAMES: ActualCharArray,
    TMPNAM: Vec<u8>,
    CODES: ActualArray<i32>,
    NNAM: i32,
    NCOD: i32,
    I: i32,
    ORDNAM: ActualArray<i32>,
    ORDCOD: ActualArray<i32>,
    INIT: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut NAMES = ActualCharArray::new(MAXL, 1..=MAXE);
        let mut TMPNAM = vec![b' '; MAXL as usize];
        let mut CODES = ActualArray::<i32>::new(1..=MAXE);
        let mut NNAM: i32 = 0;
        let mut NCOD: i32 = 0;
        let mut I: i32 = 0;
        let mut ORDNAM = ActualArray::<i32>::new(1..=MAXE);
        let mut ORDCOD = ActualArray::<i32>::new(1..=MAXE);
        let mut INIT: bool = false;

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(199),
                Val::C(b"MERCURY"),
                Val::I(299),
                Val::C(b"VENUS"),
                Val::I(399),
                Val::C(b"EARTH"),
                Val::I(499),
                Val::C(b"MARS"),
                Val::I(599),
                Val::C(b"JUPITER"),
                Val::I(699),
                Val::C(b"SATURN"),
                Val::I(799),
                Val::C(b"URANUS"),
                Val::I(899),
                Val::C(b"NEPTUNE"),
                Val::I(999),
                Val::C(b"PLUTO"),
                Val::I(301),
                Val::C(b"MOON"),
            ]
            .into_iter();
            for I in intrinsics::range(BEG1, END1, 1) {
                CODES[I] = clist.next().unwrap().into_i32();
                fstr::assign(NAMES.get_mut(I), clist.next().unwrap().into_str());
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(401),
                Val::C(b"PHOBOS"),
                Val::I(402),
                Val::C(b"DEIMOS"),
                Val::I(501),
                Val::C(b"IO"),
                Val::I(502),
                Val::C(b"EUROPA"),
                Val::I(503),
                Val::C(b"GANYMEDE"),
                Val::I(504),
                Val::C(b"CALLISTO"),
                Val::I(505),
                Val::C(b"AMALTHEA"),
                Val::I(506),
                Val::C(b"HIMALIA"),
                Val::I(507),
                Val::C(b"ELARA"),
                Val::I(508),
                Val::C(b"PASIPHAE"),
            ]
            .into_iter();
            for I in intrinsics::range(BEG2, END2, 1) {
                CODES[I] = clist.next().unwrap().into_i32();
                fstr::assign(NAMES.get_mut(I), clist.next().unwrap().into_str());
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(509),
                Val::C(b"SINOPE"),
                Val::I(510),
                Val::C(b"LYSITHEA"),
                Val::I(511),
                Val::C(b"CARME"),
                Val::I(512),
                Val::C(b"ANANKE"),
                Val::I(513),
                Val::C(b"LEDA"),
                Val::I(514),
                Val::C(b"1979J2"),
                Val::I(514),
                Val::C(b"THEBE"),
                Val::I(515),
                Val::C(b"1979J1"),
                Val::I(515),
                Val::C(b"ADRASTEA"),
                Val::I(516),
                Val::C(b"1979J3"),
            ]
            .into_iter();
            for I in intrinsics::range(BEG3, END3, 1) {
                CODES[I] = clist.next().unwrap().into_i32();
                fstr::assign(NAMES.get_mut(I), clist.next().unwrap().into_str());
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(516),
                Val::C(b"METIS"),
                Val::I(601),
                Val::C(b"MIMAS"),
                Val::I(602),
                Val::C(b"ENCELADUS"),
                Val::I(603),
                Val::C(b"TETHYS"),
                Val::I(604),
                Val::C(b"DIONE"),
                Val::I(605),
                Val::C(b"RHEA"),
                Val::I(606),
                Val::C(b"TITAN"),
                Val::I(607),
                Val::C(b"HYPERION"),
                Val::I(608),
                Val::C(b"IAPETUS"),
                Val::I(609),
                Val::C(b"PHOEBE"),
            ]
            .into_iter();
            for I in intrinsics::range(BEG4, END4, 1) {
                CODES[I] = clist.next().unwrap().into_i32();
                fstr::assign(NAMES.get_mut(I), clist.next().unwrap().into_str());
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(610),
                Val::C(b"1980S1"),
                Val::I(610),
                Val::C(b"JANUS"),
                Val::I(611),
                Val::C(b"1980S3"),
                Val::I(611),
                Val::C(b"EPIMETHEUS"),
                Val::I(612),
                Val::C(b"1980S6"),
                Val::I(612),
                Val::C(b"HELENE"),
                Val::I(613),
                Val::C(b"1980S13"),
                Val::I(613),
                Val::C(b"TELESTO"),
                Val::I(614),
                Val::C(b"1980S25"),
                Val::I(614),
                Val::C(b"CALYPSO"),
            ]
            .into_iter();
            for I in intrinsics::range(BEG5, END5, 1) {
                CODES[I] = clist.next().unwrap().into_i32();
                fstr::assign(NAMES.get_mut(I), clist.next().unwrap().into_str());
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(615),
                Val::C(b"1980S28"),
                Val::I(615),
                Val::C(b"ATLAS"),
                Val::I(616),
                Val::C(b"1980S27"),
                Val::I(616),
                Val::C(b"PROMETHEUS"),
                Val::I(617),
                Val::C(b"1980S26"),
                Val::I(617),
                Val::C(b"PANDORA"),
                Val::I(701),
                Val::C(b"ARIEL"),
                Val::I(702),
                Val::C(b"UMBRIEL"),
                Val::I(703),
                Val::C(b"TITANIA"),
                Val::I(704),
                Val::C(b"OBERON"),
            ]
            .into_iter();
            for I in intrinsics::range(BEG6, END6, 1) {
                CODES[I] = clist.next().unwrap().into_i32();
                fstr::assign(NAMES.get_mut(I), clist.next().unwrap().into_str());
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(705),
                Val::C(b"MIRANDA"),
                Val::I(706),
                Val::C(b"1986U7"),
                Val::I(706),
                Val::C(b"CORDELIA"),
                Val::I(707),
                Val::C(b"1986U8"),
                Val::I(707),
                Val::C(b"OPHELIA"),
                Val::I(708),
                Val::C(b"1986U9"),
                Val::I(708),
                Val::C(b"BIANCA"),
                Val::I(709),
                Val::C(b"1986U4"),
                Val::I(709),
                Val::C(b"CRESSIDA"),
                Val::I(710),
                Val::C(b"1986U6"),
            ]
            .into_iter();
            for I in intrinsics::range(BEG7, END7, 1) {
                CODES[I] = clist.next().unwrap().into_i32();
                fstr::assign(NAMES.get_mut(I), clist.next().unwrap().into_str());
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(710),
                Val::C(b"DESDEMONA"),
                Val::I(711),
                Val::C(b"1986U3"),
                Val::I(711),
                Val::C(b"JULIET"),
                Val::I(712),
                Val::C(b"1986U1"),
                Val::I(712),
                Val::C(b"PORTIA"),
                Val::I(713),
                Val::C(b"1986U2"),
                Val::I(713),
                Val::C(b"ROSALIND"),
                Val::I(714),
                Val::C(b"1986U5"),
                Val::I(714),
                Val::C(b"BELINDA"),
                Val::I(715),
                Val::C(b"1985U1"),
            ]
            .into_iter();
            for I in intrinsics::range(BEG8, END8, 1) {
                CODES[I] = clist.next().unwrap().into_i32();
                fstr::assign(NAMES.get_mut(I), clist.next().unwrap().into_str());
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(715),
                Val::C(b"PUCK"),
                Val::I(801),
                Val::C(b"TRITON"),
                Val::I(802),
                Val::C(b"NEREID"),
                Val::I(803),
                Val::C(b"NAIAD"),
                Val::I(804),
                Val::C(b"THALASSA"),
                Val::I(805),
                Val::C(b"DESPINA"),
                Val::I(806),
                Val::C(b"GALATEA"),
                Val::I(807),
                Val::C(b"LARISSA"),
                Val::I(808),
                Val::C(b"PROTEUS"),
                Val::I(901),
                Val::C(b"1978P1"),
                Val::I(901),
                Val::C(b"CHARON"),
                Val::I(-12),
                Val::C(b"VENUS ORBITER"),
                Val::I(-12),
                Val::C(b"P12"),
                Val::I(-12),
                Val::C(b"PIONEER 12"),
                Val::I(-18),
                Val::C(b"MGN"),
                Val::I(-18),
                Val::C(b"MAGELLAN"),
            ]
            .into_iter();
            for I in intrinsics::range(BEG9, END9, 1) {
                CODES[I] = clist.next().unwrap().into_i32();
                fstr::assign(NAMES.get_mut(I), clist.next().unwrap().into_str());
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(-27),
                Val::C(b"VK1"),
                Val::I(-27),
                Val::C(b"VIKING 1 ORBITER"),
                Val::I(-30),
                Val::C(b"VK2"),
                Val::I(-30),
                Val::C(b"VIKING 2 ORBITER"),
                Val::I(-31),
                Val::C(b"VG1"),
                Val::I(-31),
                Val::C(b"VOYAGER 1"),
                Val::I(-32),
                Val::C(b"VG2"),
                Val::I(-32),
                Val::C(b"VOYAGER 2"),
                Val::I(-46),
                Val::C(b"MS-T5"),
                Val::I(-46),
                Val::C(b"SAKIGAKE"),
            ]
            .into_iter();
            for I in intrinsics::range(BEG10, END10, 1) {
                CODES[I] = clist.next().unwrap().into_i32();
                fstr::assign(NAMES.get_mut(I), clist.next().unwrap().into_str());
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(-47),
                Val::C(b"PLANET-A"),
                Val::I(-47),
                Val::C(b"SUISEI"),
                Val::I(-58),
                Val::C(b"VSOP"),
                Val::I(-66),
                Val::C(b"VEGA 1"),
                Val::I(-67),
                Val::C(b"VEGA 2"),
                Val::I(-77),
                Val::C(b"GLL"),
                Val::I(-77),
                Val::C(b"GALILEO ORBITER"),
                Val::I(-78),
                Val::C(b"GIOTTO"),
                Val::I(-94),
                Val::C(b"MGS"),
                Val::I(-94),
                Val::C(b"MARS GLOBAL SURVEYOR"),
            ]
            .into_iter();
            for I in intrinsics::range(BEG11, END11, 1) {
                CODES[I] = clist.next().unwrap().into_i32();
                fstr::assign(NAMES.get_mut(I), clist.next().unwrap().into_str());
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(-112),
                Val::C(b"ICE"),
                Val::I(0),
                Val::C(b"SSB"),
                Val::I(0),
                Val::C(b"SOLAR SYSTEM BARYCENTER"),
                Val::I(1),
                Val::C(b"MERCURY BARYCENTER"),
                Val::I(2),
                Val::C(b"VENUS BARYCENTER"),
                Val::I(3),
                Val::C(b"EMB"),
                Val::I(3),
                Val::C(b"EARTH MOON BARYCENTER"),
                Val::I(3),
                Val::C(b"EARTH-MOON BARYCENTER"),
                Val::I(3),
                Val::C(b"EARTH BARYCENTER"),
                Val::I(4),
                Val::C(b"MARS BARYCENTER"),
            ]
            .into_iter();
            for I in intrinsics::range(BEG12, END12, 1) {
                CODES[I] = clist.next().unwrap().into_i32();
                fstr::assign(NAMES.get_mut(I), clist.next().unwrap().into_str());
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(5),
                Val::C(b"JUPITER BARYCENTER"),
                Val::I(6),
                Val::C(b"SATURN BARYCENTER"),
                Val::I(7),
                Val::C(b"URANUS BARYCENTER"),
                Val::I(8),
                Val::C(b"NEPTUNE BARYCENTER"),
                Val::I(9),
                Val::C(b"PLUTO BARYCENTER"),
                Val::I(10),
                Val::C(b"SUN"),
                Val::I(9511010),
                Val::C(b"GASPRA"),
                Val::I(2431010),
                Val::C(b"IDA"),
            ]
            .into_iter();
            for I in intrinsics::range(BEG13, END13, 1) {
                CODES[I] = clist.next().unwrap().into_i32();
                fstr::assign(NAMES.get_mut(I), clist.next().unwrap().into_str());
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(1000001),
                Val::C(b"AREND"),
                Val::I(1000002),
                Val::C(b"AREND-RIGAUX"),
                Val::I(1000003),
                Val::C(b"ASHBROOK-JACKSON"),
                Val::I(1000004),
                Val::C(b"BOETHIN"),
                Val::I(1000005),
                Val::C(b"BORRELLY"),
                Val::I(1000006),
                Val::C(b"BOWELL-SKIFF"),
                Val::I(1000007),
                Val::C(b"BRADFIELD"),
                Val::I(1000008),
                Val::C(b"BROOKS 2"),
                Val::I(1000009),
                Val::C(b"BRORSEN-METCALF"),
                Val::I(1000010),
                Val::C(b"BUS"),
            ]
            .into_iter();
            for I in intrinsics::range(BEG14, END14, 1) {
                CODES[I] = clist.next().unwrap().into_i32();
                fstr::assign(NAMES.get_mut(I), clist.next().unwrap().into_str());
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(1000011),
                Val::C(b"CHERNYKH"),
                Val::I(1000012),
                Val::C(b"CHURYUMOV-GERASIMENKO"),
                Val::I(1000013),
                Val::C(b"CIFFREO"),
                Val::I(1000014),
                Val::C(b"CLARK"),
                Val::I(1000015),
                Val::C(b"COMAS SOLA"),
                Val::I(1000016),
                Val::C(b"CROMMELIN"),
                Val::I(1000017),
                Val::C(b"D\'ARREST"),
                Val::I(1000018),
                Val::C(b"DANIEL"),
                Val::I(1000019),
                Val::C(b"DE VICO-SWIFT"),
                Val::I(1000020),
                Val::C(b"DENNING-FUJIKAWA"),
            ]
            .into_iter();
            for I in intrinsics::range(BEG15, END15, 1) {
                CODES[I] = clist.next().unwrap().into_i32();
                fstr::assign(NAMES.get_mut(I), clist.next().unwrap().into_str());
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(1000021),
                Val::C(b"DU TOIT 1"),
                Val::I(1000022),
                Val::C(b"DU TOIT-HARTLEY"),
                Val::I(1000023),
                Val::C(b"DUTOIT-NEUJMIN-DELPORTE"),
                Val::I(1000024),
                Val::C(b"DUBIAGO"),
                Val::I(1000025),
                Val::C(b"ENCKE"),
                Val::I(1000026),
                Val::C(b"FAYE"),
                Val::I(1000027),
                Val::C(b"FINLAY"),
                Val::I(1000028),
                Val::C(b"FORBES"),
                Val::I(1000029),
                Val::C(b"GEHRELS 1"),
                Val::I(1000030),
                Val::C(b"GEHRELS 2"),
            ]
            .into_iter();
            for I in intrinsics::range(BEG16, END16, 1) {
                CODES[I] = clist.next().unwrap().into_i32();
                fstr::assign(NAMES.get_mut(I), clist.next().unwrap().into_str());
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(1000031),
                Val::C(b"GEHRELS 3"),
                Val::I(1000032),
                Val::C(b"GIACOBINI-ZINNER"),
                Val::I(1000033),
                Val::C(b"GICLAS"),
                Val::I(1000034),
                Val::C(b"GRIGG-SKJELLERUP"),
                Val::I(1000035),
                Val::C(b"GUNN"),
                Val::I(1000036),
                Val::C(b"HALLEY"),
                Val::I(1000037),
                Val::C(b"HANEDA-CAMPOS"),
                Val::I(1000038),
                Val::C(b"HARRINGTON"),
                Val::I(1000039),
                Val::C(b"HARRINGTON-ABELL"),
                Val::I(1000040),
                Val::C(b"HARTLEY 1"),
            ]
            .into_iter();
            for I in intrinsics::range(BEG17, END17, 1) {
                CODES[I] = clist.next().unwrap().into_i32();
                fstr::assign(NAMES.get_mut(I), clist.next().unwrap().into_str());
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(1000041),
                Val::C(b"HARTLEY 2"),
                Val::I(1000042),
                Val::C(b"HARTLEY-IRAS"),
                Val::I(1000043),
                Val::C(b"HERSCHEL-RIGOLLET"),
                Val::I(1000044),
                Val::C(b"HOLMES"),
                Val::I(1000045),
                Val::C(b"HONDA-MRKOS-PAJDUSAKOVA"),
                Val::I(1000046),
                Val::C(b"HOWELL"),
                Val::I(1000047),
                Val::C(b"IRAS"),
                Val::I(1000048),
                Val::C(b"JACKSON-NEUJMIN"),
                Val::I(1000049),
                Val::C(b"JOHNSON"),
                Val::I(1000050),
                Val::C(b"KEARNS-KWEE"),
            ]
            .into_iter();
            for I in intrinsics::range(BEG18, END18, 1) {
                CODES[I] = clist.next().unwrap().into_i32();
                fstr::assign(NAMES.get_mut(I), clist.next().unwrap().into_str());
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(1000051),
                Val::C(b"KLEMOLA"),
                Val::I(1000052),
                Val::C(b"KOHOUTEK"),
                Val::I(1000053),
                Val::C(b"KOJIMA"),
                Val::I(1000054),
                Val::C(b"KOPFF"),
                Val::I(1000055),
                Val::C(b"KOWAL 1"),
                Val::I(1000056),
                Val::C(b"KOWAL 2"),
                Val::I(1000057),
                Val::C(b"KOWAL-MRKOS"),
                Val::I(1000058),
                Val::C(b"KOWAL-VAVROVA"),
                Val::I(1000059),
                Val::C(b"LONGMORE"),
                Val::I(1000060),
                Val::C(b"LOVAS 1"),
            ]
            .into_iter();
            for I in intrinsics::range(BEG19, END19, 1) {
                CODES[I] = clist.next().unwrap().into_i32();
                fstr::assign(NAMES.get_mut(I), clist.next().unwrap().into_str());
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(1000061),
                Val::C(b"MACHHOLZ"),
                Val::I(1000062),
                Val::C(b"MAURY"),
                Val::I(1000063),
                Val::C(b"NEUJMIN 1"),
                Val::I(1000064),
                Val::C(b"NEUJMIN 2"),
                Val::I(1000065),
                Val::C(b"NEUJMIN 3"),
                Val::I(1000066),
                Val::C(b"OLBERS"),
                Val::I(1000067),
                Val::C(b"PETERS-HARTLEY"),
                Val::I(1000068),
                Val::C(b"PONS-BROOKS"),
                Val::I(1000069),
                Val::C(b"PONS-WINNECKE"),
                Val::I(1000070),
                Val::C(b"REINMUTH 1"),
            ]
            .into_iter();
            for I in intrinsics::range(BEG20, END20, 1) {
                CODES[I] = clist.next().unwrap().into_i32();
                fstr::assign(NAMES.get_mut(I), clist.next().unwrap().into_str());
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(1000071),
                Val::C(b"REINMUTH 2"),
                Val::I(1000072),
                Val::C(b"RUSSELL 1"),
                Val::I(1000073),
                Val::C(b"RUSSELL 2"),
                Val::I(1000074),
                Val::C(b"RUSSELL 3"),
                Val::I(1000075),
                Val::C(b"RUSSELL 4"),
                Val::I(1000076),
                Val::C(b"SANGUIN"),
                Val::I(1000077),
                Val::C(b"SCHAUMASSE"),
                Val::I(1000078),
                Val::C(b"SCHUSTER"),
                Val::I(1000079),
                Val::C(b"SCHWASSMANN-WACHMANN 1"),
                Val::I(1000080),
                Val::C(b"SCHWASSMANN-WACHMANN 2"),
            ]
            .into_iter();
            for I in intrinsics::range(BEG21, END21, 1) {
                CODES[I] = clist.next().unwrap().into_i32();
                fstr::assign(NAMES.get_mut(I), clist.next().unwrap().into_str());
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(1000081),
                Val::C(b"SCHWASSMANN-WACHMANN 3"),
                Val::I(1000082),
                Val::C(b"SHAJN-SCHALDACH"),
                Val::I(1000083),
                Val::C(b"SHOEMAKER 1"),
                Val::I(1000084),
                Val::C(b"SHOEMAKER 2"),
                Val::I(1000085),
                Val::C(b"SHOEMAKER 3"),
                Val::I(1000086),
                Val::C(b"SINGER-BREWSTER"),
                Val::I(1000087),
                Val::C(b"SLAUGHTER-BURNHAM"),
                Val::I(1000088),
                Val::C(b"SMIRNOVA-CHERNYKH"),
                Val::I(1000089),
                Val::C(b"STEPHAN-OTERMA"),
                Val::I(1000090),
                Val::C(b"SWIFT-GEHRELS"),
            ]
            .into_iter();
            for I in intrinsics::range(BEG22, END22, 1) {
                CODES[I] = clist.next().unwrap().into_i32();
                fstr::assign(NAMES.get_mut(I), clist.next().unwrap().into_str());
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(1000091),
                Val::C(b"TAKAMIZAWA"),
                Val::I(1000092),
                Val::C(b"TAYLOR"),
                Val::I(1000093),
                Val::C(b"TEMPEL 1"),
                Val::I(1000094),
                Val::C(b"TEMPEL 2"),
                Val::I(1000095),
                Val::C(b"TEMPEL-TUTTLE"),
                Val::I(1000096),
                Val::C(b"TRITTON"),
                Val::I(1000097),
                Val::C(b"TSUCHINSHAN 1"),
                Val::I(1000098),
                Val::C(b"TSUCHINSHAN 2"),
                Val::I(1000099),
                Val::C(b"TUTTLE"),
                Val::I(1000100),
                Val::C(b"TUTTLE-GIACOBINI-KRESAK"),
            ]
            .into_iter();
            for I in intrinsics::range(BEG23, END23, 1) {
                CODES[I] = clist.next().unwrap().into_i32();
                fstr::assign(NAMES.get_mut(I), clist.next().unwrap().into_str());
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(1000101),
                Val::C(b"VAISALA 1"),
                Val::I(1000102),
                Val::C(b"VAN BIESBROECK"),
                Val::I(1000103),
                Val::C(b"VAN HOUTEN"),
                Val::I(1000104),
                Val::C(b"WEST-KOHOUTEK-IKEMURA"),
                Val::I(1000105),
                Val::C(b"WHIPPLE"),
                Val::I(1000106),
                Val::C(b"WILD 1"),
                Val::I(1000107),
                Val::C(b"WILD 2"),
                Val::I(1000108),
                Val::C(b"WILD 3"),
                Val::I(1000109),
                Val::C(b"WIRTANEN"),
                Val::I(1000110),
                Val::C(b"WOLF"),
            ]
            .into_iter();
            for I in intrinsics::range(BEG24, END24, 1) {
                CODES[I] = clist.next().unwrap().into_i32();
                fstr::assign(NAMES.get_mut(I), clist.next().unwrap().into_str());
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(1000111),
                Val::C(b"WOLF-HARRINGTON"),
                Val::I(1000112),
                Val::C(b"LOVAS 2"),
                Val::I(1000113),
                Val::C(b"URATA-NIIJIMA"),
                Val::I(1000114),
                Val::C(b"WISEMAN-SKIFF"),
                Val::I(1000115),
                Val::C(b"HELIN"),
                Val::I(1000116),
                Val::C(b"MUELLER"),
                Val::I(1000117),
                Val::C(b"SHOEMAKER-HOLT 1"),
                Val::I(1000118),
                Val::C(b"HELIN-ROMAN-CROCKETT"),
                Val::I(1000119),
                Val::C(b"HARTLEY 3"),
                Val::I(1000120),
                Val::C(b"PARKER-HARTLEY"),
            ]
            .into_iter();
            for I in intrinsics::range(BEG25, END25, 1) {
                CODES[I] = clist.next().unwrap().into_i32();
                fstr::assign(NAMES.get_mut(I), clist.next().unwrap().into_str());
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(1000121),
                Val::C(b"HELIN-ROMAN-ALU 1"),
                Val::I(1000122),
                Val::C(b"WILD 4"),
                Val::I(1000123),
                Val::C(b"MUELLER 2"),
                Val::I(1000124),
                Val::C(b"MUELLER 3"),
                Val::I(1000125),
                Val::C(b"SHOEMAKER-LEVY 1"),
                Val::I(1000126),
                Val::C(b"SHOEMAKER-LEVY 2"),
                Val::I(1000127),
                Val::C(b"HOLT-OLMSTEAD"),
                Val::I(1000128),
                Val::C(b"METCALF-BREWINGTON"),
                Val::I(1000129),
                Val::C(b"LEVY"),
                Val::I(1000130),
                Val::C(b"SHOEMAKER-LEVY 9"),
            ]
            .into_iter();
            for I in intrinsics::range(BEG26, END26, 1) {
                CODES[I] = clist.next().unwrap().into_i32();
                fstr::assign(NAMES.get_mut(I), clist.next().unwrap().into_str());
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(50000001),
                Val::C(b"SHOEMAKER-LEVY 9-W"),
                Val::I(50000002),
                Val::C(b"SHOEMAKER-LEVY 9-V"),
                Val::I(50000003),
                Val::C(b"SHOEMAKER-LEVY 9-U"),
                Val::I(50000004),
                Val::C(b"SHOEMAKER-LEVY 9-T"),
                Val::I(50000005),
                Val::C(b"SHOEMAKER-LEVY 9-S"),
                Val::I(50000006),
                Val::C(b"SHOEMAKER-LEVY 9-R"),
                Val::I(50000007),
                Val::C(b"SHOEMAKER-LEVY 9-Q"),
                Val::I(50000008),
                Val::C(b"SHOEMAKER-LEVY 9-P"),
                Val::I(50000009),
                Val::C(b"SHOEMAKER-LEVY 9-N"),
                Val::I(50000010),
                Val::C(b"SHOEMAKER-LEVY 9-M"),
            ]
            .into_iter();
            for I in intrinsics::range(BEG27, END27, 1) {
                CODES[I] = clist.next().unwrap().into_i32();
                fstr::assign(NAMES.get_mut(I), clist.next().unwrap().into_str());
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(50000011),
                Val::C(b"SHOEMAKER-LEVY 9-L"),
                Val::I(50000012),
                Val::C(b"SHOEMAKER-LEVY 9-K"),
                Val::I(50000013),
                Val::C(b"SHOEMAKER-LEVY 9-J"),
                Val::I(50000014),
                Val::C(b"SHOEMAKER-LEVY 9-H"),
                Val::I(50000015),
                Val::C(b"SHOEMAKER-LEVY 9-G"),
                Val::I(50000016),
                Val::C(b"SHOEMAKER-LEVY 9-F"),
                Val::I(50000017),
                Val::C(b"SHOEMAKER-LEVY 9-E"),
                Val::I(50000018),
                Val::C(b"SHOEMAKER-LEVY 9-D"),
                Val::I(50000019),
                Val::C(b"SHOEMAKER-LEVY 9-C"),
                Val::I(50000020),
                Val::C(b"SHOEMAKER-LEVY 9-B"),
            ]
            .into_iter();
            for I in intrinsics::range(BEG28, END28, 1) {
                CODES[I] = clist.next().unwrap().into_i32();
                fstr::assign(NAMES.get_mut(I), clist.next().unwrap().into_str());
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(50000021),
                Val::C(b"SHOEMAKER-LEVY 9-A"),
                Val::I(50000022),
                Val::C(b"SHOEMAKER-LEVY 9-Q1"),
                Val::I(50000023),
                Val::C(b"SHOEMAKER-LEVY 9-P2"),
                Val::I(-40),
                Val::C(b"CLEMENTINE"),
                Val::I(-344),
                Val::C(b"GLL PROBE"),
                Val::I(-344),
                Val::C(b"GALILEO PROBE"),
                Val::I(2000433),
                Val::C(b"EROS"),
                Val::I(2000253),
                Val::C(b"MATHILDE"),
                Val::I(618),
                Val::C(b"PAN"),
                Val::I(-59),
                Val::C(b"RADIOASTRON"),
            ]
            .into_iter();
            for I in intrinsics::range(BEG29, END29, 1) {
                CODES[I] = clist.next().unwrap().into_i32();
                fstr::assign(NAMES.get_mut(I), clist.next().unwrap().into_str());
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(-53),
                Val::C(b"MARS PATHFINDER"),
                Val::I(-53),
                Val::C(b"MPF"),
                Val::I(-93),
                Val::C(b"NEAR"),
                Val::I(-93),
                Val::C(b"NEAR EARTH ASTEROID RENDEZVOUS"),
                Val::I(-82),
                Val::C(b"CASSINI"),
                Val::I(-82),
                Val::C(b"CAS"),
                Val::I(-150),
                Val::C(b"CASSINI HUYGENS PROBE"),
                Val::I(-55),
                Val::C(b"ULYSSES"),
                Val::I(399001),
                Val::C(b"GOLDSTONE"),
                Val::I(399002),
                Val::C(b"CANBERRA"),
            ]
            .into_iter();
            for I in intrinsics::range(BEG30, END30, 1) {
                CODES[I] = clist.next().unwrap().into_i32();
                fstr::assign(NAMES.get_mut(I), clist.next().unwrap().into_str());
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(399003),
                Val::C(b"MADRID"),
                Val::I(399004),
                Val::C(b"USUDA"),
                Val::I(1000131),
                Val::C(b"HYAKUTAKE"),
                Val::I(1000132),
                Val::C(b"HALE-BOPP"),
                Val::I(-550),
                Val::C(b"MARS-96"),
                Val::I(-550),
                Val::C(b"M96"),
                Val::I(-550),
                Val::C(b"MARS 96"),
                Val::I(-550),
                Val::C(b"MARS96"),
                Val::I(-90),
                Val::C(b"CASSINI SIMULATION"),
                Val::I(-95),
                Val::C(b"MGS SIMULATION"),
                Val::I(-81),
                Val::C(b"CASSINI ITL"),
            ]
            .into_iter();
            for I in intrinsics::range(BEG31, END31, 1) {
                CODES[I] = clist.next().unwrap().into_i32();
                fstr::assign(NAMES.get_mut(I), clist.next().unwrap().into_str());
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        INIT = true;
        NNAM = NPERM;

        Self {
            NAMES,
            TMPNAM,
            CODES,
            NNAM,
            NCOD,
            I,
            ORDNAM,
            ORDCOD,
            INIT,
        }
    }
}

//$Procedure      M2BODTRN ( Body name and code translation )
pub fn M2BODTRN(NAME: &[u8], CODE: i32, FOUND: bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    //
    // SPICELIB functions
    //

    //
    // Functions
    //

    //
    // The parameters here are for ease in maintaining the
    // large collection of automatic names that are stored
    // in data statements.  To insert a name/code pair in the
    // block from BEGx to ENDx, redefine ENDx to be
    // one larger than its current definition.  Recompiling
    // will automatically modify all the other parameters.
    //

    //
    // Local variables
    //

    //
    // Introducing the permanent collection.
    //

    //
    // The 851, 852, ... codes are temporary codes for the newly-
    // discovered satellites of Neptune.  These will go away when
    // the official codes are assigned.  The codes listed above
    // do not include these temporary assignments.
    //
    // The proposed names are the following:
    //
    //    1989N1 = Proteus
    //    1989N2 = Larissa
    //    1989N3 = Despina
    //    1989N4 = Galatea
    //    1989N5 = Thalassa
    //    1989N6 = Naiad
    //

    //
    // Standard SPICELIB error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"M2BODTRN", ctx)?;
    }

    //
    // This routine should never be called. If it is called,
    // an error is signalled.
    //
    spicelib::SETMSG(b"M2BODTRN: You have called an entry which performs no run-time function. This may indicate a bug. Please check the documentation for the subroutine M2BODTRN.", ctx);

    spicelib::SIGERR(b"SPICE(BOGUSENTRY)", ctx)?;

    spicelib::CHKOUT(b"M2BODTRN", ctx)?;
    Ok(())
}

//$Procedure M2BODN2C ( Body name to code )
pub fn M2BODN2C(
    NAME: &[u8],
    CODE: &mut i32,
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICELIB error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"M2BODN2C", ctx)?;
    }

    *FOUND = false;

    spicelib::BODN2C(NAME, CODE, FOUND, ctx)?;

    if *FOUND {
        spicelib::CHKOUT(b"M2BODN2C", ctx)?;
        return Ok(());
    }

    //
    // Get the order vectors for the names and codes.
    //
    if save.INIT {
        save.INIT = false;

        M2BODINI(
            save.NAMES.as_arg(),
            save.NNAM,
            save.CODES.as_slice(),
            &mut save.NCOD,
            save.ORDNAM.as_slice_mut(),
            save.ORDCOD.as_slice_mut(),
        );
    }

    //
    // Return the CODE associated with the name.
    //
    spicelib::LJUST(NAME, &mut save.TMPNAM);
    spicelib::UCASE(&save.TMPNAM.to_vec(), &mut save.TMPNAM, ctx);
    spicelib::CMPRSS(b" ", 1, &save.TMPNAM.to_vec(), &mut save.TMPNAM);

    save.I = spicelib::BSCHOC(
        &save.TMPNAM,
        save.NNAM,
        save.NAMES.as_arg(),
        save.ORDNAM.as_slice(),
    );

    if (save.I != 0) {
        *CODE = save.CODES[save.I];
        *FOUND = true;
    } else {
        {
            let m1__: i32 = 1;
            let m2__: i32 = save.NNAM;
            let m3__: i32 = 1;
            save.I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                if spicelib::EQSTR(&save.TMPNAM, &save.NAMES[save.I]) {
                    *CODE = save.CODES[save.I];
                    *FOUND = true;
                    spicelib::CHKOUT(b"M2BODN2C", ctx)?;
                    return Ok(());
                }

                save.I += m3__;
            }
        }
    }

    spicelib::CHKOUT(b"M2BODN2C", ctx)?;
    Ok(())
}

//$Procedure M2BODC2N ( Body code to name )
pub fn M2BODC2N(
    CODE: i32,
    NAME: &mut [u8],
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICELIB error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"M2BODC2N", ctx)?;
    }

    *FOUND = false;

    //
    // Get the order vectors for the names and codes.
    //
    if save.INIT {
        save.INIT = false;

        M2BODINI(
            save.NAMES.as_arg(),
            save.NNAM,
            save.CODES.as_slice(),
            &mut save.NCOD,
            save.ORDNAM.as_slice_mut(),
            save.ORDCOD.as_slice_mut(),
        );
    }

    //
    // Return the name associated with the CODE.
    //
    save.I = spicelib::BSCHOI(
        CODE,
        save.NCOD,
        save.CODES.as_slice(),
        save.ORDCOD.as_slice(),
    );

    if (save.I != 0) {
        fstr::assign(NAME, save.NAMES.get(save.I));
        *FOUND = true;
    }

    spicelib::CHKOUT(b"M2BODC2N", ctx)?;
    Ok(())
}

//$Procedure M2BODDEF ( Body name/code definition )
pub fn M2BODDEF(NAME: &[u8], CODE: i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICELIB error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"M2BODDEF", ctx)?;
    }

    //
    // Initialize the order vectors if we haven't already.
    //
    if save.INIT {
        save.INIT = false;

        M2BODINI(
            save.NAMES.as_arg(),
            save.NNAM,
            save.CODES.as_slice(),
            &mut save.NCOD,
            save.ORDNAM.as_slice_mut(),
            save.ORDCOD.as_slice_mut(),
        );
    }

    //
    // Make sure the name has not already been used.
    //
    spicelib::LJUST(NAME, &mut save.TMPNAM);
    spicelib::UCASE(&save.TMPNAM.to_vec(), &mut save.TMPNAM, ctx);
    spicelib::CMPRSS(b" ", 1, &save.TMPNAM.to_vec(), &mut save.TMPNAM);

    save.I = spicelib::BSCHOC(
        &save.TMPNAM,
        save.NNAM,
        save.NAMES.as_arg(),
        save.ORDNAM.as_slice(),
    );

    if (save.I != 0) {
        spicelib::SETMSG(
            b"The name, \'#\', has already been used for body having id-code #.",
            ctx,
        );
        spicelib::ERRCH(b"#", NAME, ctx);
        spicelib::ERRINT(b"#", save.CODES[save.I], ctx);
        spicelib::SIGERR(b"SPICE(NAMENOTUNIQUE)", ctx)?;
        spicelib::CHKOUT(b"M2BODDEF", ctx)?;
        return Ok(());
    }

    //
    // Do we have room for another name/code pair?
    //

    if (save.NNAM < MAXE) {
        save.NNAM = (save.NNAM + 1);
    } else {
        spicelib::SETMSG(b"There is no room available for adding \'#\'  to the list of name/code pairs. The number of names that can be supported is #.  This number has been reached. ", ctx);

        spicelib::ERRCH(b"#", NAME, ctx);
        spicelib::ERRINT(b"#", save.NNAM, ctx);
        spicelib::SIGERR(b"SPICE(TOOMANYPAIRS)", ctx)?;
        spicelib::CHKOUT(b"M2BODDEF", ctx)?;
        return Ok(());
    }

    //
    // Add NAME and CODE and reorder the vectors.
    //
    fstr::assign(save.NAMES.get_mut(save.NNAM), &save.TMPNAM);
    save.CODES[save.NNAM] = CODE;

    M2BODINI(
        save.NAMES.as_arg(),
        save.NNAM,
        save.CODES.as_slice(),
        &mut save.NCOD,
        save.ORDNAM.as_slice_mut(),
        save.ORDCOD.as_slice_mut(),
    );

    spicelib::CHKOUT(b"M2BODDEF", ctx)?;
    Ok(())
}
