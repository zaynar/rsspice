//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const NTERM: i32 = 106;
const ZERO: f64 = 0.0;
const REV: f64 = 360.0;
const DEG: f64 = 1.0;
const MIN: f64 = (1.0 / 60.0);
const SEC: f64 = (1.0 / 3600.0);
const CENT1: f64 = 36525.0;
const CENT2: f64 = ((CENT1 * CENT1) * 0.00000001);
const CENT3: f64 = ((CENT1 * CENT2) * 0.0001);
const L: i32 = 1;
const LP: i32 = (L + 1);
const F: i32 = (LP + 1);
const D: i32 = (F + 1);
const MG: i32 = (D + 1);

struct SaveVars {
    CE: f64,
    CL: f64,
    COSANG: f64,
    ANGLE: StackArray<f64, 5>,
    ANGRT: StackArray<f64, 5>,
    ARG: f64,
    ARGRT: f64,
    DD: f64,
    DD2: f64,
    DDDJ: f64,
    DJ: f64,
    DPI: f64,
    DTWOPI: f64,
    FACTR: f64,
    ONEDAY: f64,
    RADIAN: f64,
    RASEC: f64,
    SINANG: f64,
    T: f64,
    D0: f64,
    D1: f64,
    D2: f64,
    D3: f64,
    F0: f64,
    F1: f64,
    F2: f64,
    F3: f64,
    L0: f64,
    L1: f64,
    L2: f64,
    L3: f64,
    LP0: f64,
    LP1: f64,
    LP2: f64,
    LP3: f64,
    MG0: f64,
    MG1: f64,
    MG2: f64,
    MG3: f64,
    MATRIX: ActualArray2D<i32>,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut CE: f64 = 0.0;
        let mut CL: f64 = 0.0;
        let mut COSANG: f64 = 0.0;
        let mut ANGLE = StackArray::<f64, 5>::new(1..=5);
        let mut ANGRT = StackArray::<f64, 5>::new(1..=5);
        let mut ARG: f64 = 0.0;
        let mut ARGRT: f64 = 0.0;
        let mut DD: f64 = 0.0;
        let mut DD2: f64 = 0.0;
        let mut DDDJ: f64 = 0.0;
        let mut DJ: f64 = 0.0;
        let mut DPI: f64 = 0.0;
        let mut DTWOPI: f64 = 0.0;
        let mut FACTR: f64 = 0.0;
        let mut ONEDAY: f64 = 0.0;
        let mut RADIAN: f64 = 0.0;
        let mut RASEC: f64 = 0.0;
        let mut SINANG: f64 = 0.0;
        let mut T: f64 = 0.0;
        let mut D0: f64 = 0.0;
        let mut D1: f64 = 0.0;
        let mut D2: f64 = 0.0;
        let mut D3: f64 = 0.0;
        let mut F0: f64 = 0.0;
        let mut F1: f64 = 0.0;
        let mut F2: f64 = 0.0;
        let mut F3: f64 = 0.0;
        let mut L0: f64 = 0.0;
        let mut L1: f64 = 0.0;
        let mut L2: f64 = 0.0;
        let mut L3: f64 = 0.0;
        let mut LP0: f64 = 0.0;
        let mut LP1: f64 = 0.0;
        let mut LP2: f64 = 0.0;
        let mut LP3: f64 = 0.0;
        let mut MG0: f64 = 0.0;
        let mut MG1: f64 = 0.0;
        let mut MG2: f64 = 0.0;
        let mut MG3: f64 = 0.0;
        let mut MATRIX = ActualArray2D::<i32>::new(1..=9, 1..=NTERM);
        let mut FIRST: bool = false;

        FIRST = true;
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(1),
                Val::I(-171996),
                Val::I(-1742),
                Val::I(92025),
                Val::I(89),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(2),
                Val::I(2062),
                Val::I(2),
                Val::I(-895),
                Val::I(5),
                Val::I(-2),
                Val::I(0),
                Val::I(2),
                Val::I(0),
                Val::I(1),
                Val::I(46),
                Val::I(0),
                Val::I(-24),
                Val::I(0),
                Val::I(2),
                Val::I(0),
                Val::I(-2),
                Val::I(0),
                Val::I(0),
                Val::I(11),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(-2),
                Val::I(0),
                Val::I(2),
                Val::I(0),
                Val::I(2),
                Val::I(-3),
                Val::I(0),
                Val::I(1),
                Val::I(0),
                Val::I(1),
                Val::I(-1),
                Val::I(0),
                Val::I(-1),
                Val::I(0),
                Val::I(-3),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(-2),
                Val::I(2),
                Val::I(-2),
                Val::I(1),
                Val::I(-2),
                Val::I(0),
                Val::I(1),
                Val::I(0),
                Val::I(2),
                Val::I(0),
                Val::I(-2),
                Val::I(0),
                Val::I(1),
                Val::I(1),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(2),
                Val::I(-2),
                Val::I(2),
                Val::I(-13187),
                Val::I(-16),
                Val::I(5736),
                Val::I(-31),
                Val::I(0),
                Val::I(1),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(1426),
                Val::I(-34),
                Val::I(54),
                Val::I(-1),
                Val::I(0),
                Val::I(1),
                Val::I(2),
                Val::I(-2),
                Val::I(2),
                Val::I(-517),
                Val::I(12),
                Val::I(224),
                Val::I(-6),
                Val::I(0),
                Val::I(-1),
                Val::I(2),
                Val::I(-2),
                Val::I(2),
                Val::I(217),
                Val::I(-5),
                Val::I(-95),
                Val::I(3),
                Val::I(0),
                Val::I(0),
                Val::I(2),
                Val::I(-2),
                Val::I(1),
                Val::I(129),
                Val::I(1),
                Val::I(-70),
                Val::I(0),
                Val::I(2),
                Val::I(0),
                Val::I(0),
                Val::I(-2),
                Val::I(0),
                Val::I(48),
                Val::I(0),
                Val::I(1),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(2),
                Val::I(-2),
                Val::I(0),
                Val::I(-22),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(2),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(17),
                Val::I(-1),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(1),
                Val::I(0),
                Val::I(0),
                Val::I(1),
                Val::I(-15),
                Val::I(0),
                Val::I(9),
                Val::I(0),
                Val::I(0),
                Val::I(2),
                Val::I(2),
                Val::I(-2),
                Val::I(2),
                Val::I(-16),
                Val::I(1),
                Val::I(7),
                Val::I(0),
                Val::I(0),
                Val::I(-1),
                Val::I(0),
                Val::I(0),
                Val::I(1),
                Val::I(-12),
                Val::I(0),
                Val::I(6),
                Val::I(0),
            ]
            .into_iter();
            for J in intrinsics::range(1, 19, 1) {
                for I in intrinsics::range(1, 9, 1) {
                    MATRIX[[I, J]] = clist.next().unwrap().into_i32();
                }
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(-2),
                Val::I(0),
                Val::I(0),
                Val::I(2),
                Val::I(1),
                Val::I(-6),
                Val::I(0),
                Val::I(3),
                Val::I(0),
                Val::I(0),
                Val::I(-1),
                Val::I(2),
                Val::I(-2),
                Val::I(1),
                Val::I(-5),
                Val::I(0),
                Val::I(3),
                Val::I(0),
                Val::I(2),
                Val::I(0),
                Val::I(0),
                Val::I(-2),
                Val::I(1),
                Val::I(4),
                Val::I(0),
                Val::I(-2),
                Val::I(0),
                Val::I(0),
                Val::I(1),
                Val::I(2),
                Val::I(-2),
                Val::I(1),
                Val::I(4),
                Val::I(0),
                Val::I(-2),
                Val::I(0),
                Val::I(1),
                Val::I(0),
                Val::I(0),
                Val::I(-1),
                Val::I(0),
                Val::I(-4),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(2),
                Val::I(1),
                Val::I(0),
                Val::I(-2),
                Val::I(0),
                Val::I(1),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(-2),
                Val::I(2),
                Val::I(1),
                Val::I(1),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(1),
                Val::I(-2),
                Val::I(2),
                Val::I(0),
                Val::I(-1),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(1),
                Val::I(0),
                Val::I(0),
                Val::I(2),
                Val::I(1),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(-1),
                Val::I(0),
                Val::I(0),
                Val::I(1),
                Val::I(1),
                Val::I(1),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(1),
                Val::I(2),
                Val::I(-2),
                Val::I(0),
                Val::I(-1),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(2),
                Val::I(0),
                Val::I(2),
                Val::I(-2274),
                Val::I(-2),
                Val::I(977),
                Val::I(-5),
                Val::I(1),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(712),
                Val::I(1),
                Val::I(-7),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(2),
                Val::I(0),
                Val::I(1),
                Val::I(-386),
                Val::I(-4),
                Val::I(200),
                Val::I(0),
                Val::I(1),
                Val::I(0),
                Val::I(2),
                Val::I(0),
                Val::I(2),
                Val::I(-301),
                Val::I(0),
                Val::I(129),
                Val::I(-1),
                Val::I(1),
                Val::I(0),
                Val::I(0),
                Val::I(-2),
                Val::I(0),
                Val::I(-158),
                Val::I(0),
                Val::I(-1),
                Val::I(0),
                Val::I(-1),
                Val::I(0),
                Val::I(2),
                Val::I(0),
                Val::I(2),
                Val::I(123),
                Val::I(0),
                Val::I(-53),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(2),
                Val::I(0),
                Val::I(63),
                Val::I(0),
                Val::I(-2),
                Val::I(0),
                Val::I(1),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(1),
                Val::I(63),
                Val::I(1),
                Val::I(-33),
                Val::I(0),
            ]
            .into_iter();
            for J in intrinsics::range(20, 38, 1) {
                for I in intrinsics::range(1, 9, 1) {
                    MATRIX[[I, J]] = clist.next().unwrap().into_i32();
                }
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(-1),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(1),
                Val::I(-58),
                Val::I(-1),
                Val::I(32),
                Val::I(0),
                Val::I(-1),
                Val::I(0),
                Val::I(2),
                Val::I(2),
                Val::I(2),
                Val::I(-59),
                Val::I(0),
                Val::I(26),
                Val::I(0),
                Val::I(1),
                Val::I(0),
                Val::I(2),
                Val::I(0),
                Val::I(1),
                Val::I(-51),
                Val::I(0),
                Val::I(27),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(2),
                Val::I(2),
                Val::I(2),
                Val::I(-38),
                Val::I(0),
                Val::I(16),
                Val::I(0),
                Val::I(2),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(29),
                Val::I(0),
                Val::I(-1),
                Val::I(0),
                Val::I(1),
                Val::I(0),
                Val::I(2),
                Val::I(-2),
                Val::I(2),
                Val::I(29),
                Val::I(0),
                Val::I(-12),
                Val::I(0),
                Val::I(2),
                Val::I(0),
                Val::I(2),
                Val::I(0),
                Val::I(2),
                Val::I(-31),
                Val::I(0),
                Val::I(13),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(2),
                Val::I(0),
                Val::I(0),
                Val::I(26),
                Val::I(0),
                Val::I(-1),
                Val::I(0),
                Val::I(-1),
                Val::I(0),
                Val::I(2),
                Val::I(0),
                Val::I(1),
                Val::I(21),
                Val::I(0),
                Val::I(-10),
                Val::I(0),
                Val::I(-1),
                Val::I(0),
                Val::I(0),
                Val::I(2),
                Val::I(1),
                Val::I(16),
                Val::I(0),
                Val::I(-8),
                Val::I(0),
                Val::I(1),
                Val::I(0),
                Val::I(0),
                Val::I(-2),
                Val::I(1),
                Val::I(-13),
                Val::I(0),
                Val::I(7),
                Val::I(0),
                Val::I(-1),
                Val::I(0),
                Val::I(2),
                Val::I(2),
                Val::I(1),
                Val::I(-10),
                Val::I(0),
                Val::I(5),
                Val::I(0),
                Val::I(1),
                Val::I(1),
                Val::I(0),
                Val::I(-2),
                Val::I(0),
                Val::I(-7),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(1),
                Val::I(2),
                Val::I(0),
                Val::I(2),
                Val::I(7),
                Val::I(0),
                Val::I(-3),
                Val::I(0),
                Val::I(0),
                Val::I(-1),
                Val::I(2),
                Val::I(0),
                Val::I(2),
                Val::I(-7),
                Val::I(0),
                Val::I(3),
                Val::I(0),
                Val::I(1),
                Val::I(0),
                Val::I(2),
                Val::I(2),
                Val::I(2),
                Val::I(-8),
                Val::I(0),
                Val::I(3),
                Val::I(0),
                Val::I(1),
                Val::I(0),
                Val::I(0),
                Val::I(2),
                Val::I(0),
                Val::I(6),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(2),
                Val::I(0),
                Val::I(2),
                Val::I(-2),
                Val::I(2),
                Val::I(6),
                Val::I(0),
                Val::I(-3),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(2),
                Val::I(1),
                Val::I(-6),
                Val::I(0),
                Val::I(3),
                Val::I(0),
            ]
            .into_iter();
            for J in intrinsics::range(39, 57, 1) {
                for I in intrinsics::range(1, 9, 1) {
                    MATRIX[[I, J]] = clist.next().unwrap().into_i32();
                }
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(0),
                Val::I(0),
                Val::I(2),
                Val::I(2),
                Val::I(1),
                Val::I(-7),
                Val::I(0),
                Val::I(3),
                Val::I(0),
                Val::I(1),
                Val::I(0),
                Val::I(2),
                Val::I(-2),
                Val::I(1),
                Val::I(6),
                Val::I(0),
                Val::I(-3),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(-2),
                Val::I(1),
                Val::I(-5),
                Val::I(0),
                Val::I(3),
                Val::I(0),
                Val::I(1),
                Val::I(-1),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(5),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(2),
                Val::I(0),
                Val::I(2),
                Val::I(0),
                Val::I(1),
                Val::I(-5),
                Val::I(0),
                Val::I(3),
                Val::I(0),
                Val::I(0),
                Val::I(1),
                Val::I(0),
                Val::I(-2),
                Val::I(0),
                Val::I(-4),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(1),
                Val::I(0),
                Val::I(-2),
                Val::I(0),
                Val::I(0),
                Val::I(4),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(1),
                Val::I(0),
                Val::I(-4),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(1),
                Val::I(1),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(-3),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(1),
                Val::I(0),
                Val::I(2),
                Val::I(0),
                Val::I(0),
                Val::I(3),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(1),
                Val::I(-1),
                Val::I(2),
                Val::I(0),
                Val::I(2),
                Val::I(-3),
                Val::I(0),
                Val::I(1),
                Val::I(0),
                Val::I(-1),
                Val::I(-1),
                Val::I(2),
                Val::I(2),
                Val::I(2),
                Val::I(-3),
                Val::I(0),
                Val::I(1),
                Val::I(0),
                Val::I(-2),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(1),
                Val::I(-2),
                Val::I(0),
                Val::I(1),
                Val::I(0),
                Val::I(3),
                Val::I(0),
                Val::I(2),
                Val::I(0),
                Val::I(2),
                Val::I(-3),
                Val::I(0),
                Val::I(1),
                Val::I(0),
                Val::I(0),
                Val::I(-1),
                Val::I(2),
                Val::I(2),
                Val::I(2),
                Val::I(-3),
                Val::I(0),
                Val::I(1),
                Val::I(0),
                Val::I(1),
                Val::I(1),
                Val::I(2),
                Val::I(0),
                Val::I(2),
                Val::I(2),
                Val::I(0),
                Val::I(-1),
                Val::I(0),
                Val::I(-1),
                Val::I(0),
                Val::I(2),
                Val::I(-2),
                Val::I(1),
                Val::I(-2),
                Val::I(0),
                Val::I(1),
                Val::I(0),
                Val::I(2),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(1),
                Val::I(2),
                Val::I(0),
                Val::I(-1),
                Val::I(0),
                Val::I(1),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(2),
                Val::I(-2),
                Val::I(0),
                Val::I(1),
                Val::I(0),
            ]
            .into_iter();
            for J in intrinsics::range(58, 76, 1) {
                for I in intrinsics::range(1, 9, 1) {
                    MATRIX[[I, J]] = clist.next().unwrap().into_i32();
                }
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(3),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(2),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(2),
                Val::I(1),
                Val::I(2),
                Val::I(2),
                Val::I(0),
                Val::I(-1),
                Val::I(0),
                Val::I(-1),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(2),
                Val::I(1),
                Val::I(0),
                Val::I(-1),
                Val::I(0),
                Val::I(1),
                Val::I(0),
                Val::I(0),
                Val::I(-4),
                Val::I(0),
                Val::I(-1),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(-2),
                Val::I(0),
                Val::I(2),
                Val::I(2),
                Val::I(2),
                Val::I(1),
                Val::I(0),
                Val::I(-1),
                Val::I(0),
                Val::I(-1),
                Val::I(0),
                Val::I(2),
                Val::I(4),
                Val::I(2),
                Val::I(-2),
                Val::I(0),
                Val::I(1),
                Val::I(0),
                Val::I(2),
                Val::I(0),
                Val::I(0),
                Val::I(-4),
                Val::I(0),
                Val::I(-1),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(1),
                Val::I(1),
                Val::I(2),
                Val::I(-2),
                Val::I(2),
                Val::I(1),
                Val::I(0),
                Val::I(-1),
                Val::I(0),
                Val::I(1),
                Val::I(0),
                Val::I(2),
                Val::I(2),
                Val::I(1),
                Val::I(-1),
                Val::I(0),
                Val::I(1),
                Val::I(0),
                Val::I(-2),
                Val::I(0),
                Val::I(2),
                Val::I(4),
                Val::I(2),
                Val::I(-1),
                Val::I(0),
                Val::I(1),
                Val::I(0),
                Val::I(-1),
                Val::I(0),
                Val::I(4),
                Val::I(0),
                Val::I(2),
                Val::I(1),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(1),
                Val::I(-1),
                Val::I(0),
                Val::I(-2),
                Val::I(0),
                Val::I(1),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(2),
                Val::I(0),
                Val::I(2),
                Val::I(-2),
                Val::I(1),
                Val::I(1),
                Val::I(0),
                Val::I(-1),
                Val::I(0),
                Val::I(2),
                Val::I(0),
                Val::I(2),
                Val::I(2),
                Val::I(2),
                Val::I(-1),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(1),
                Val::I(0),
                Val::I(0),
                Val::I(2),
                Val::I(1),
                Val::I(-1),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(4),
                Val::I(-2),
                Val::I(2),
                Val::I(1),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(3),
                Val::I(0),
                Val::I(2),
                Val::I(-2),
                Val::I(2),
                Val::I(1),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(1),
                Val::I(0),
                Val::I(2),
                Val::I(-2),
                Val::I(0),
                Val::I(-1),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(1),
                Val::I(2),
                Val::I(0),
                Val::I(1),
                Val::I(1),
                Val::I(0),
                Val::I(0),
                Val::I(0),
            ]
            .into_iter();
            for J in intrinsics::range(77, 95, 1) {
                for I in intrinsics::range(1, 9, 1) {
                    MATRIX[[I, J]] = clist.next().unwrap().into_i32();
                }
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(-1),
                Val::I(-1),
                Val::I(0),
                Val::I(2),
                Val::I(1),
                Val::I(1),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(-2),
                Val::I(0),
                Val::I(1),
                Val::I(-1),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(2),
                Val::I(-1),
                Val::I(2),
                Val::I(-1),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(1),
                Val::I(0),
                Val::I(2),
                Val::I(0),
                Val::I(-1),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(1),
                Val::I(0),
                Val::I(-2),
                Val::I(-2),
                Val::I(0),
                Val::I(-1),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(-1),
                Val::I(2),
                Val::I(0),
                Val::I(1),
                Val::I(-1),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(1),
                Val::I(1),
                Val::I(0),
                Val::I(-2),
                Val::I(1),
                Val::I(-1),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(1),
                Val::I(0),
                Val::I(-2),
                Val::I(2),
                Val::I(0),
                Val::I(-1),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(2),
                Val::I(0),
                Val::I(0),
                Val::I(2),
                Val::I(0),
                Val::I(1),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(2),
                Val::I(4),
                Val::I(2),
                Val::I(-1),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(0),
                Val::I(1),
                Val::I(0),
                Val::I(1),
                Val::I(0),
                Val::I(1),
                Val::I(0),
                Val::I(0),
                Val::I(0),
            ]
            .into_iter();
            for J in intrinsics::range(96, NTERM, 1) {
                for I in intrinsics::range(1, 9, 1) {
                    MATRIX[[I, J]] = clist.next().unwrap().into_i32();
                }
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            CE,
            CL,
            COSANG,
            ANGLE,
            ANGRT,
            ARG,
            ARGRT,
            DD,
            DD2,
            DDDJ,
            DJ,
            DPI,
            DTWOPI,
            FACTR,
            ONEDAY,
            RADIAN,
            RASEC,
            SINANG,
            T,
            D0,
            D1,
            D2,
            D3,
            F0,
            F1,
            F2,
            F3,
            L0,
            L1,
            L2,
            L3,
            LP0,
            LP1,
            LP2,
            LP3,
            MG0,
            MG1,
            MG2,
            MG3,
            MATRIX,
            FIRST,
        }
    }
}

//$Procedure      ZZWAHR ( SPICELIB private version of Newhalls' WAHR )
pub fn ZZWAHR(ET: f64, DVNUT: &mut [f64], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut DVNUT = DummyArrayMut::new(DVNUT, 1..=4);

    //
    // SPICELIB Functions
    //

    //
    // Parameters
    //
    // NTERM is the number of SIN and COSINE terms used in the
    // computation of Delta Psi and Delta epsilon
    //

    //
    // The parameters below stand for
    //
    //    revolutions
    //    degrees
    //    minutes
    //    seconds
    //    julian century
    //    julian century ** 2
    //    julian century ** 3
    //
    // These parameters are needed for converting the quantities
    // on page 114 of the Explanatory supplement from revolutions,
    // degrees, minutes and seconds / century, century**2 and century**3
    // to degrees, degrees/day, degrees/(0.0001 days)**2 and
    // degrees/(0.0001 days)**3.
    //

    //
    // The next set of parameters is an enumeration of the various
    // angles needed in the computation of nutations.
    //

    //
    // Local Variables.
    //

    //
    //     Below are the coefficients for the various periods of the
    //     nutation model.  There does not appear to be any particular reason
    //     for the ordering selected.  The n'th row corresponds to the n'th
    //     period listed above each data statement.
    //
    //>> Periods: 6798.4, 3399.2, 1305.5, 1095.2, 1615.7, 3232.9, 6786.3,
    //             943.2,  182.6,  365.3,  121.7,  365.2,  177.8,  205.9,
    //             173.3,  182.6,  386.0,   91.3,  346.6
    //

    //
    // Periods: 199.8, 346.6, 212.3, 119.6, 411.8, 131.7, 169.0, 329.8,
    //          409.2, 388.3, 117.5,  13.7,  27.6,  13.6,   9.1,  31.8,
    //           27.1,  14.8,  27.7
    //
    //
    // Periods: 27.4, 9.6,  9.1,  7.1, 13.8, 23.9, 6.9, 13.6, 27.0, 32.0,
    //          31.7, 9.5, 34.8, 13.2, 14.2,  5.6, 9.6, 12.8, 14.8
    //

    //
    // Periods: 7.1, 23.9, 14.7, 29.8, 6.9, 15.4, 26.9, 29.5, 25.6, 9.1,
    //          9.4,  9.8, 13.7,  5.5, 7.2,  8.9, 32.6, 13.8, 27.8

    //
    // Periods: 9.2,  9.3, 27.3, 10.1, 14.6,  5.8, 15.9, 22.5,  5.6,
    //          7.3,  9.1, 29.3, 12.8,  4.7,  9.6, 12.7,  8.7, 23.8,
    //         13.1

    //
    // Periods: 35.0, 13.6, 25.4, 14.2, 9.5, 14.2, 34.7, 32.8, 7.1, 4.8,
    //          27.3

    if save.FIRST {
        save.FIRST = false;
        save.DPI = PI(ctx);
        save.DTWOPI = TWOPI(ctx);
        save.RADIAN = (180.0 / save.DPI);
        save.RASEC = (3600.0 * save.RADIAN);
        save.FACTR = (10000.0 * save.RASEC);
        save.ONEDAY = SPD();

        //
        // The following values are direct conversions to degrees from
        // page 114 of the Explanatory Supplement to the Astronomical
        // Almanac.
        //
        // L0 through L3 are the coefficients for l---the mean longitude
        // of the Moon minus the mean longitude of the Moon's perigee.
        // Units for the various terms:
        //
        //    L0      degrees
        //    L1      degrees/day
        //    L2      degrees/(0.0001 days)**2
        //    L3      degrees/(0.0001 days)**3
        //
        save.L0 = (((134.0 * DEG) + (57.0 * MIN)) + (46.733 * SEC));
        save.L1 = (((((1325.0 * REV) + (198.0 * DEG)) + (52.0 * MIN)) + (2.633 * SEC)) / CENT1);
        save.L2 = ((31.31 * SEC) / CENT2);
        save.L3 = ((0.064 * SEC) / CENT3);

        //
        // LP0 through LP3 are the coefficients for l'---the mean
        // longitude of the Sun minus the mean longitude of the Sun's
        // perigee. Units for the various terms:
        //
        //    LP0      degrees
        //    LP1      degrees/day
        //    LP2      degrees/(0.0001 days)**2
        //    LP3      degrees/(0.0001 days)**3
        //
        save.LP0 = (((357.0 * DEG) + (31.0 * MIN)) + (39.804 * SEC));
        save.LP1 = (((((99.0 * REV) + (359.0 * DEG)) + (3.0 * MIN)) + (1.224 * SEC)) / CENT1);
        save.LP2 = -((0.577 * SEC) / CENT2);
        save.LP3 = -((0.012 * SEC) / CENT3);

        //
        // F0 through F3 are the coefficients for F---the mean longitude
        // of the Moon minus the mean longitude of the Moon's node. Units
        // for the various terms:
        //
        //    F0      degrees
        //    F1      degrees/day
        //    F2      degrees/(0.0001 days)**2
        //    F3      degrees/(0.0001 days)**3
        //
        save.F0 = (((93.0 * DEG) + (16.0 * MIN)) + (18.877 * SEC));
        save.F1 = (((((1342.0 * REV) + (82.0 * DEG)) + (1.0 * MIN)) + (3.137 * SEC)) / CENT1);
        save.F2 = -((13.257 * SEC) / CENT2);
        save.F3 = ((0.011 * SEC) / CENT3);

        //
        // D0 through D3 are the coefficients for D---the mean longitude
        // of the Moon minus the mean longitude of the Sun. Units
        // for the various terms:
        //
        //    D0      degrees
        //    D1      degrees/day
        //    D2      degrees/(0.0001 days)**2
        //    D3      degrees/(0.0001 days)**3
        //
        save.D0 = (((297.0 * DEG) + (51.0 * MIN)) + (1.307 * SEC));
        save.D1 = (((((1236.0 * REV) + (307.0 * DEG)) + (6.0 * MIN)) + (41.328 * SEC)) / CENT1);
        save.D2 = -((6.891 * SEC) / CENT2);
        save.D3 = ((0.019 * SEC) / CENT3);

        //
        // MG0 through MG3 are the coefficients for Omega---the longitude
        // of the mean ascending node of the lunar orbit on the ecliptic
        // measured from the mean equinox of date.  NOTE: The constant
        // term MG0 is correct.  The value
        //        o
        //     135 02' 40".280
        //
        // given in the Explanatory Supplement page 114 has a typo.  The
        // correct value is the one used here:
        //
        //        o
        //     125 02' 40".280
        //
        //    MG0      degrees
        //    MG1      degrees/day
        //    MG2      degrees/(0.0001 days)**2
        //    MG3      degrees/(0.0001 days)**3
        //
        save.MG0 = (((125.0 * DEG) + (2.0 * MIN)) + (40.28 * SEC));
        save.MG1 = -(((((5.0 * REV) + (134.0 * DEG)) + (8.0 * MIN)) + (10.539 * SEC)) / CENT1);
        save.MG2 = ((7.455 * SEC) / CENT2);
        save.MG3 = ((0.008 * SEC) / CENT3);
    }

    //
    // Compute all of the various time components.  DJ is the delta
    // in the Julian date from the J2000 epoch.
    //
    save.DJ = (ET / save.ONEDAY);
    save.DD = (save.DJ / 10000.0);
    save.DDDJ = (save.DD / 10000.0);
    save.DD2 = (save.DD * save.DD);
    save.T = (save.DJ / 365250.0);

    //
    // Now compute all of the various angles and their rates
    // at the current epoch
    //
    save.ANGLE[L] =
        ((save.L0 + (save.DJ * save.L1)) + ((save.L2 + (save.DD * save.L3)) * save.DD2));
    save.ANGLE[LP] =
        ((save.LP0 + (save.DJ * save.LP1)) + ((save.LP2 + (save.DD * save.LP3)) * save.DD2));
    save.ANGLE[F] =
        ((save.F0 + (save.DJ * save.F1)) + ((save.F2 + (save.DD * save.F3)) * save.DD2));
    save.ANGLE[D] =
        ((save.D0 + (save.DJ * save.D1)) + ((save.D2 + (save.DD * save.D3)) * save.DD2));
    save.ANGLE[MG] =
        ((save.MG0 + (save.DJ * save.MG1)) + ((save.MG2 + (save.DD * save.MG3)) * save.DD2));

    save.ANGRT[L] = (save.L1 + (save.DDDJ * ((2.0 * save.L2) + ((3.0 * save.DD) * save.L3))));
    save.ANGRT[LP] = (save.LP1 + (save.DDDJ * ((2.0 * save.LP2) + ((3.0 * save.DD) * save.LP3))));
    save.ANGRT[F] = (save.F1 + (save.DDDJ * ((2.0 * save.F2) + ((3.0 * save.DD) * save.F3))));
    save.ANGRT[D] = (save.D1 + (save.DDDJ * ((2.0 * save.D2) + ((3.0 * save.DD) * save.D3))));
    save.ANGRT[MG] = (save.MG1 + (save.DDDJ * ((2.0 * save.MG2) + ((3.0 * save.DD) * save.MG3))));

    //
    // Wrap all of the angles and rates to range from 0 to 360, then
    // convert to radians.
    //
    for J in 1..=5 {
        save.ANGLE[J] = intrinsics::DMOD(save.ANGLE[J], 360.0);
        save.ANGRT[J] = intrinsics::DMOD(save.ANGRT[J], 360.0);

        save.ANGLE[J] = (save.ANGLE[J] / save.RADIAN);
        save.ANGRT[J] = (save.ANGRT[J] / save.RADIAN);
    }
    //
    // Zero out the components of the nutation array
    //
    for J in 1..=4 {
        DVNUT[J] = ZERO;
    }
    //
    // Now we accumulate the various terms of Delta Psi and Delta
    // epsilon as expressed on page 115 of the Green Book
    // (Explanatory Supplement to the Astronomical Almanac).
    //
    for I in 1..=NTERM {
        save.ARG = ZERO;
        save.ARGRT = ZERO;

        for J in 1..=5 {
            if (save.MATRIX[[J, I]] != 0) {
                save.ARG = (save.ARG + ((save.MATRIX[[J, I]] as f64) * save.ANGLE[J]));
                save.ARGRT = (save.ARGRT + ((save.MATRIX[[J, I]] as f64) * save.ANGRT[J]));
                save.ARG = intrinsics::DMOD(save.ARG, save.DTWOPI);
            }
        }

        save.CL = save.MATRIX[[6, I]] as f64;

        if (save.MATRIX[[7, I]] != 0) {
            save.CL = (save.CL + ((save.MATRIX[[7, I]] as f64) * save.T));
        }

        save.CE = save.MATRIX[[8, I]] as f64;

        if (save.MATRIX[[9, I]] != 0) {
            save.CE = (save.CE + ((save.MATRIX[[9, I]] as f64) * save.T));
        }

        save.COSANG = f64::cos(save.ARG);
        save.SINANG = f64::sin(save.ARG);

        DVNUT[1] = (DVNUT[1] + ((save.CL * save.SINANG) / save.FACTR));
        DVNUT[2] = (DVNUT[2] + ((save.CE * save.COSANG) / save.FACTR));
        DVNUT[3] = (DVNUT[3] + (((save.CL * save.COSANG) * save.ARGRT) / save.FACTR));
        DVNUT[4] = (DVNUT[4] - (((save.CE * save.SINANG) * save.ARGRT) / save.FACTR));
    }

    //
    // Finally convert DVNUT(3) and DVNUT(4) to radians/second
    //
    DVNUT[3] = (DVNUT[3] / save.ONEDAY);
    DVNUT[4] = (DVNUT[4] / save.ONEDAY);
}
