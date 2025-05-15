//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const EBIG: f64 = 1000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0;
const EBIGN: f64 = -EBIG;
const EBIG2: f64 = 2000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0;
const ESMALL: f64 = 0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001;
const ESMALN: f64 = -0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001;
const ESMAL2: f64 = 0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002;
const EMAX: i32 = 306;
const MSGLEN: i32 = 240;
const NBASIC: i32 = 11;
const NRAND: i32 = 5000;
const NCASE: i32 = (NBASIC + NRAND);
const LINLEN: i32 = 79;
const UBPL: i32 = 4;
const MAXRE: f64 = 0.00000000001;

struct SaveVars {
    DIR: StackArray2D<f64, 33>,
    EXPXPT: StackArray2D<f64, 33>,
    PLCONS: StackArray<f64, 11>,
    PLNORM: StackArray2D<f64, 33>,
    VERTEX: StackArray2D<f64, 33>,
    EXPNO: StackArray<i32, 11>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut DIR = StackArray2D::<f64, 33>::new(1..=3, 1..=NBASIC);
        let mut EXPXPT = StackArray2D::<f64, 33>::new(1..=3, 1..=NBASIC);
        let mut PLCONS = StackArray::<f64, 11>::new(1..=NBASIC);
        let mut PLNORM = StackArray2D::<f64, 33>::new(1..=3, 1..=NBASIC);
        let mut VERTEX = StackArray2D::<f64, 33>::new(1..=3, 1..=NBASIC);
        let mut EXPNO = StackArray::<i32, 11>::new(1..=NBASIC);

        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(0.0), Val::D(0.0), Val::D(0.0), Val::D(1.0)].into_iter();
            PLCONS[1] = clist.next().unwrap().into_f64();
            for I in intrinsics::range(1, 3, 1) {
                PLNORM[[I, 1]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::D(0.0),
                Val::D(0.0),
                Val::D(2.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(-1.0),
            ]
            .into_iter();
            for I in intrinsics::range(1, 3, 1) {
                VERTEX[[I, 1]] = clist.next().unwrap().into_f64();
            }
            for I in intrinsics::range(1, 3, 1) {
                DIR[[I, 1]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(1), Val::D(0.0), Val::D(0.0), Val::D(0.0)].into_iter();
            EXPNO[1] = clist.next().unwrap().into_i32();
            for I in intrinsics::range(1, 3, 1) {
                EXPXPT[[I, 1]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(10.0), Val::D(0.0), Val::D(0.0), Val::D(1.0)].into_iter();
            PLCONS[2] = clist.next().unwrap().into_f64();
            for I in intrinsics::range(1, 3, 1) {
                PLNORM[[I, 2]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::D(0.0),
                Val::D(0.0),
                Val::D(20.0),
                Val::D(0.0),
                Val::D(3.0),
                Val::D(-1.0),
            ]
            .into_iter();
            for I in intrinsics::range(1, 3, 1) {
                VERTEX[[I, 2]] = clist.next().unwrap().into_f64();
            }
            for I in intrinsics::range(1, 3, 1) {
                DIR[[I, 2]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(1), Val::D(0.0), Val::D(30.0), Val::D(10.0)].into_iter();
            EXPNO[2] = clist.next().unwrap().into_i32();
            for I in intrinsics::range(1, 3, 1) {
                EXPXPT[[I, 2]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(0.0), Val::D(0.0), Val::D(0.0), Val::D(1.0)].into_iter();
            PLCONS[3] = clist.next().unwrap().into_f64();
            for I in intrinsics::range(1, 3, 1) {
                PLNORM[[I, 3]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(-1.0),
            ]
            .into_iter();
            for I in intrinsics::range(1, 3, 1) {
                VERTEX[[I, 3]] = clist.next().unwrap().into_f64();
            }
            for I in intrinsics::range(1, 3, 1) {
                DIR[[I, 3]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(1), Val::D(0.0), Val::D(0.0), Val::D(0.0)].into_iter();
            EXPNO[3] = clist.next().unwrap().into_i32();
            for I in intrinsics::range(1, 3, 1) {
                EXPXPT[[I, 3]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(0.0), Val::D(0.0), Val::D(1.0), Val::D(0.0)].into_iter();
            PLCONS[4] = clist.next().unwrap().into_f64();
            for I in intrinsics::range(1, 3, 1) {
                PLNORM[[I, 4]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(1.0),
            ]
            .into_iter();
            for I in intrinsics::range(1, 3, 1) {
                VERTEX[[I, 4]] = clist.next().unwrap().into_f64();
            }
            for I in intrinsics::range(1, 3, 1) {
                DIR[[I, 4]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(-1), Val::D(0.0), Val::D(0.0), Val::D(0.0)].into_iter();
            EXPNO[4] = clist.next().unwrap().into_i32();
            for I in intrinsics::range(1, 3, 1) {
                EXPXPT[[I, 4]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(0.0), Val::D(0.0), Val::D(0.0), Val::D(1.0)].into_iter();
            PLCONS[5] = clist.next().unwrap().into_f64();
            for I in intrinsics::range(1, 3, 1) {
                PLNORM[[I, 5]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::D(1.0),
                Val::D(1.0),
                Val::D(1.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(0.0),
            ]
            .into_iter();
            for I in intrinsics::range(1, 3, 1) {
                VERTEX[[I, 5]] = clist.next().unwrap().into_f64();
            }
            for I in intrinsics::range(1, 3, 1) {
                DIR[[I, 5]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(0), Val::D(0.0), Val::D(0.0), Val::D(0.0)].into_iter();
            EXPNO[5] = clist.next().unwrap().into_i32();
            for I in intrinsics::range(1, 3, 1) {
                EXPXPT[[I, 5]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(0.0), Val::D(0.0), Val::D(0.0), Val::D(1.0)].into_iter();
            PLCONS[6] = clist.next().unwrap().into_f64();
            for I in intrinsics::range(1, 3, 1) {
                PLNORM[[I, 6]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::D(1.0),
                Val::D(1.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(10000000000000000.0),
                Val::D(-1.0),
            ]
            .into_iter();
            for I in intrinsics::range(1, 3, 1) {
                VERTEX[[I, 6]] = clist.next().unwrap().into_f64();
            }
            for I in intrinsics::range(1, 3, 1) {
                DIR[[I, 6]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(1),
                Val::D(1.0),
                Val::D(10000000000000000.0),
                Val::D(0.0),
            ]
            .into_iter();
            EXPNO[6] = clist.next().unwrap().into_i32();
            for I in intrinsics::range(1, 3, 1) {
                EXPXPT[[I, 6]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(0.0), Val::D(0.0), Val::D(0.0), Val::D(1.0)].into_iter();
            PLCONS[7] = clist.next().unwrap().into_f64();
            for I in intrinsics::range(1, 3, 1) {
                PLNORM[[I, 7]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
    Val::D(1.0),
    Val::D(1.0),
    Val::D(1.0),
    Val::D(0.0),
    Val::D(100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0),
    Val::D(-1.0),
  ].into_iter();
            for I in intrinsics::range(1, 3, 1) {
                VERTEX[[I, 7]] = clist.next().unwrap().into_f64();
            }
            for I in intrinsics::range(1, 3, 1) {
                DIR[[I, 7]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(0), Val::D(0.0), Val::D(0.0), Val::D(0.0)].into_iter();
            EXPNO[7] = clist.next().unwrap().into_i32();
            for I in intrinsics::range(1, 3, 1) {
                EXPXPT[[I, 7]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(EBIG), Val::D(0.0), Val::D(0.0), Val::D(1.0)].into_iter();
            PLCONS[8] = clist.next().unwrap().into_f64();
            for I in intrinsics::range(1, 3, 1) {
                PLNORM[[I, 8]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::D(1.0),
                Val::D(1.0),
                Val::D(EBIG2),
                Val::D(0.0),
                Val::D(100.0),
                Val::D(-1.0),
            ]
            .into_iter();
            for I in intrinsics::range(1, 3, 1) {
                VERTEX[[I, 8]] = clist.next().unwrap().into_f64();
            }
            for I in intrinsics::range(1, 3, 1) {
                DIR[[I, 8]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(0), Val::D(0.0), Val::D(0.0), Val::D(0.0)].into_iter();
            EXPNO[8] = clist.next().unwrap().into_i32();
            for I in intrinsics::range(1, 3, 1) {
                EXPXPT[[I, 8]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(EBIGN), Val::D(0.0), Val::D(0.0), Val::D(1.0)].into_iter();
            PLCONS[9] = clist.next().unwrap().into_f64();
            for I in intrinsics::range(1, 3, 1) {
                PLNORM[[I, 9]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::D(EBIG),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(-1.0),
            ]
            .into_iter();
            for I in intrinsics::range(1, 3, 1) {
                VERTEX[[I, 9]] = clist.next().unwrap().into_f64();
            }
            for I in intrinsics::range(1, 3, 1) {
                DIR[[I, 9]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(1), Val::D(EBIG2), Val::D(0.0), Val::D(EBIGN)].into_iter();
            EXPNO[9] = clist.next().unwrap().into_i32();
            for I in intrinsics::range(1, 3, 1) {
                EXPXPT[[I, 9]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(ESMALN), Val::D(0.0), Val::D(0.0), Val::D(1.0)].into_iter();
            PLCONS[10] = clist.next().unwrap().into_f64();
            for I in intrinsics::range(1, 3, 1) {
                PLNORM[[I, 10]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::D(ESMALL),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(-1.0),
            ]
            .into_iter();
            for I in intrinsics::range(1, 3, 1) {
                VERTEX[[I, 10]] = clist.next().unwrap().into_f64();
            }
            for I in intrinsics::range(1, 3, 1) {
                DIR[[I, 10]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(1), Val::D(ESMAL2), Val::D(0.0), Val::D(ESMALN)].into_iter();
            EXPNO[10] = clist.next().unwrap().into_i32();
            for I in intrinsics::range(1, 3, 1) {
                EXPXPT[[I, 10]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(1), Val::D(0.0), Val::D(0.0), Val::D(1.0)].into_iter();
            PLCONS[11] = clist.next().unwrap().into_f64();
            for I in intrinsics::range(1, 3, 1) {
                PLNORM[[I, 11]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(-1.0),
            ]
            .into_iter();
            for I in intrinsics::range(1, 3, 1) {
                VERTEX[[I, 11]] = clist.next().unwrap().into_f64();
            }
            for I in intrinsics::range(1, 3, 1) {
                DIR[[I, 11]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(0), Val::D(0.0), Val::D(0.0), Val::D(0.0)].into_iter();
            EXPNO[11] = clist.next().unwrap().into_i32();
            for I in intrinsics::range(1, 3, 1) {
                EXPXPT[[I, 11]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            DIR,
            EXPXPT,
            PLCONS,
            PLNORM,
            VERTEX,
            EXPNO,
        }
    }
}

//$Procedure F_INRYPL ( INRYPL tests )
pub fn F_INRYPL(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut TITLE = [b' '; MSGLEN as usize];
    let mut C: f64 = 0.0;
    let mut CONS: f64 = 0.0;
    let mut D = StackArray::<f64, 3>::new(1..=3);
    let mut DIFF = StackArray::<f64, 3>::new(1..=3);
    let mut ERRC: f64 = 0.0;
    let mut ERRD = StackArray::<f64, 3>::new(1..=3);
    let mut ERRN = StackArray::<f64, 3>::new(1..=3);
    let mut ERROR = StackArray::<f64, 3>::new(1..=3);
    let mut ERRV = StackArray::<f64, 3>::new(1..=3);
    let mut N = StackArray::<f64, 3>::new(1..=3);
    let mut NORMAL = StackArray::<f64, 3>::new(1..=3);
    let mut PLANE = StackArray::<f64, 4>::new(1..=UBPL);
    let mut SCALE: f64 = 0.0;
    let mut SEP: f64 = 0.0;
    let mut TOOBIG: f64 = 0.0;
    let mut V = StackArray::<f64, 3>::new(1..=3);
    let mut V2 = StackArray::<f64, 3>::new(1..=3);
    let mut VPRJ = StackArray::<f64, 3>::new(1..=3);
    let mut XPT = StackArray::<f64, 3>::new(1..=3);
    let mut NXPTS: i32 = 0;
    let mut SEED: i32 = 0;
    let mut SEPOK: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Other functions
    //

    //
    // Local Parameters
    //

    //
    // Local Variables
    //

    //
    // Saved variables
    //

    //
    // Initial values
    //

    //
    // Normal test cases:
    //

    //
    // 1)
    //
    // Planes' normal vector:                  ( 0,   0,   1 )
    // Plane constant:                           0
    // Ray's vertex:                           ( 0,   0,   2 )
    // Ray's direction vector:                 ( 0,   0,  -1 )
    // Expected number of intercept points:      1
    // Expected intercept point:               ( 0,   0,   0 )
    //
    //

    //
    // 2)
    //
    // Planes' normal vector:                  ( 0,     0,     1     )
    // Plane constant:                           10
    // Ray's vertex:                           ( 0,     0,     20    )
    // Ray's direction vector:                 ( 0,     3,    -1     )
    // Expected number of intercept points:      1
    // Expected intercept point:               ( 0,     30     10    )
    //
    //

    //
    // 3)
    //
    // Planes' normal vector:                  ( 0,   0,   1 )
    // Plane constant:                           0
    // Ray's vertex:                           ( 1,   1,   0 )
    // Ray's direction vector:                 ( 0,   0,  -1 )
    // Expected number of intercept points:      1
    // Expected intercept point:               ( 1,   1,   0 )
    //
    //

    //
    // 4)
    //
    // Planes' normal vector:                  ( 0,   1,   0 )
    // Plane constant:                           0
    // Ray's vertex:                           ( 0,   0,   0 )
    // Ray's direction vector:                 ( 1,   0,   0 )
    // Expected number of intercept points:     -1 (infinite)
    // Expected intercept point:               ( 0,   0,   0 )
    //
    //

    //
    // 5)
    //
    // Planes' normal vector:                  ( 0,   0,   1 )
    // Plane constant:                           0
    // Ray's vertex:                           ( 1,   1,   1 )
    // Ray's direction vector:                 ( 1,   0,   0 )
    // Expected number of intercept points:      0
    // Expected intercept point:               ( 0,   0,   0 )
    //
    //

    //
    // 6)
    //
    // Planes' normal vector:                  ( 0,     0,       1 )
    // Plane constant:                           0
    // Ray's vertex:                           ( 1,     1,       1 )
    // Ray's direction vector:                 ( 0,     1.D16,  -1 )
    // Expected number of intercept points:      1
    // Expected intercept point:               ( 1,     1.D16,   0 )
    //
    //

    //
    // 7)
    //
    // Planes' normal vector:                  ( 0,     0,       1 )
    // Plane constant:                           0
    // Ray's vertex:                           ( 1,     1,       1 )
    // Ray's direction vector:                 ( 0,     1.D308, -1 )
    // Expected number of intercept points:      0
    // Expected intercept point:               ( 0,     0,       0 )
    //
    //

    //
    // 8)
    //
    // Planes' normal vector:                  ( 0,     0,       1     )
    // Plane constant:                           EBIG
    // Ray's vertex:                           ( 1,     1,      2*EBIG )
    // Ray's direction vector:                 ( 0,     100,    -1     )
    // Expected number of intercept points:      0
    // Expected intercept point:               ( 0,     0,       0 )
    //
    //

    //
    // 9)
    //
    // Planes' normal vector:                  ( 0,      0,       1    )
    // Plane constant:                          -1.D306
    // Ray's vertex:                           ( 1.D306,  0,      0    )
    // Ray's direction vector:                 ( 1,      0,      -1    )
    // Expected number of intercept points:      1
    // Expected intercept point:               ( 2.D306,  0,   -1.D306 )
    //
    //

    //
    // 10)
    //
    // Planes' normal vector:                  ( 0,       0,       1    )
    // Plane constant:                          -1.D-306
    // Ray's vertex:                           ( 1.D-306,  0,      0    )
    // Ray's direction vector:                 ( 1,       0,      -1    )
    // Expected number of intercept points:      1
    // Expected intercept point:               ( 2.D-306,  0,  -1.D-306 )
    //
    //

    //
    // 11)
    //
    // Planes' normal vector:                  ( 0,       0,       1    )
    // Plane constant:                           1
    // Ray's vertex:                           ( 0     ,  0,       0    )
    // Ray's direction vector:                 ( 1,       0,      -1    )
    // Expected number of intercept points:      0
    // Expected intercept point:               ( 0,       0,       0    )
    //
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_INRYPL", ctx)?;

    //
    // We'll start out with some easy-to-check cases (11 of them).
    //
    for I in 1..=NBASIC {
        //
        // --- Case: ------------------------------------------------------
        //
        fstr::assign(&mut TITLE, b"Basic case #");
        spicelib::REPMI(&TITLE.clone(), b"#", I, &mut TITLE, ctx);
        testutil::TCASE(&TITLE, ctx)?;

        //
        // Make a SPICELIB plane from the plane constant and normal
        // vector.
        //
        spicelib::NVC2PL(
            save.PLNORM.subarray([1, I]),
            save.PLCONS[I],
            PLANE.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::PL2NVC(PLANE.as_slice(), NORMAL.as_slice_mut(), &mut CONS);
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        //
        // Call the routine to be tested.
        //
        spicelib::INRYPL(
            save.VERTEX.subarray([1, I]),
            save.DIR.subarray([1, I]),
            PLANE.as_slice(),
            &mut NXPTS,
            XPT.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSI(
            b"NXPTS---number of intercept points",
            NXPTS,
            b"=",
            save.EXPNO[I],
            0,
            OK,
            ctx,
        )?;

        testutil::CHCKAD(
            b"XPT---intercept",
            XPT.as_slice(),
            b"~~/",
            save.EXPXPT.subarray([1, I]),
            3,
            MAXRE,
            OK,
            ctx,
        )?;
    }

    //
    // Random cases follow.
    //
    SEED = -1;

    for I in 1..=NRAND {
        //
        // --- Case: ------------------------------------------------------
        //
        fstr::assign(&mut TITLE, b"Random case #");
        spicelib::REPMI(&TITLE.clone(), b"#", I, &mut TITLE, ctx);
        testutil::TCASE(&TITLE, ctx)?;
        //
        // Generate a random scale factor.
        //
        SCALE = f64::powf(
            10.0,
            testutil::T_RANDD(-(EMAX as f64), (EMAX as f64), &mut SEED, ctx)?,
        );

        //
        // Generate a normal vector and plane constant, and from these
        // a SPICELIB plane.
        //
        N[1] = testutil::T_RANDD(-2.0, 2.0, &mut SEED, ctx)?;
        N[2] = testutil::T_RANDD(-2.0, 2.0, &mut SEED, ctx)?;
        N[3] = testutil::T_RANDD(-2.0, 2.0, &mut SEED, ctx)?;

        spicelib::VHATIP(N.as_slice_mut());
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        C = (testutil::T_RANDD(-2.0, 2.0, &mut SEED, ctx)? * SCALE);

        testutil::CHCKXC(false, b" ", OK, ctx)?;
        spicelib::NVC2PL(N.as_slice(), C, PLANE.as_slice_mut(), ctx)?;

        //
        // Now generate a random ray vertex and ray direction vector.
        //
        V[1] = testutil::T_RANDD(-2.0, 2.0, &mut SEED, ctx)?;
        V[2] = testutil::T_RANDD(-2.0, 2.0, &mut SEED, ctx)?;
        V[3] = testutil::T_RANDD(-2.0, 2.0, &mut SEED, ctx)?;

        spicelib::VSCLIP(SCALE, V.as_slice_mut());
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        D[1] = testutil::T_RANDD(-2.0, 2.0, &mut SEED, ctx)?;
        D[2] = testutil::T_RANDD(-2.0, 2.0, &mut SEED, ctx)?;
        D[3] = testutil::T_RANDD(-2.0, 2.0, &mut SEED, ctx)?;

        //
        // The call.
        //
        spicelib::INRYPL(
            V.as_slice(),
            D.as_slice(),
            PLANE.as_slice(),
            &mut NXPTS,
            XPT.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // We can pretty safely assume that we won't see a value of
        // -1 for NXPTS.  If the value is 1, we'll try to get back from
        // XPT to the ray's vertex.
        //
        if (NXPTS == 1) {
            spicelib::VSUB(V.as_slice(), XPT.as_slice(), DIFF.as_slice_mut());

            spicelib::VHATIP(D.as_slice_mut());
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::VLCOM(
                1.0,
                XPT.as_slice(),
                -spicelib::VNORM(DIFF.as_slice()),
                D.as_slice(),
                V2.as_slice_mut(),
            );

            testutil::CHCKAD(
                b"V2",
                V2.as_slice(),
                b"~~/",
                V.as_slice(),
                3,
                MAXRE,
                OK,
                ctx,
            )?;

            spicelib::VSUB(V.as_slice(), V2.as_slice(), ERROR.as_slice_mut());
        } else {
            //
            // Check the angular separation between the ray
            // and the vector from the ray's vertex to its orthogonal
            // projection to the plane.
            //
            spicelib::VPRJP(V.as_slice(), PLANE.as_slice(), VPRJ.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;
            spicelib::VSUB(VPRJ.as_slice(), V.as_slice(), DIFF.as_slice_mut());

            SEP = spicelib::VSEP(DIFF.as_slice(), D.as_slice(), ctx);

            TOOBIG = (spicelib::DPMAX() / 3 as f64);

            if ((SEP * spicelib::DPR(ctx)) >= 90.0) {
                //
                // The ray is parallel to or points away from the plane.
                //
                SEPOK = true;
            } else if (SEP > f64::atan2(TOOBIG, spicelib::VNORM(DIFF.as_slice()))) {
                //
                // It doesn't happen often, but we might have a case
                // where the ray is too close to being parallel with
                // the plane for an intersection to occur.
                //
                SEPOK = true;
            } else {
                //
                // This shouldn't happen.
                //
                SEPOK = false;
            }

            testutil::CHCKSL(
                b"SEPOK---is angular separation of ray and plane consistent with return code?",
                SEPOK,
                true,
                OK,
                ctx,
            )?;
        }
    }

    //
    //     Now for the exceptions.
    //
    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Exception:  ray\'s direction is the zero vector.", ctx)?;

    spicelib::CLEARD(3, ERRV.as_slice_mut());
    spicelib::CLEARD(3, ERRD.as_slice_mut());
    spicelib::CLEARD(3, ERRN.as_slice_mut());

    ERRN[3] = 1.0;
    ERRC = 0.0;

    spicelib::NVC2PL(ERRN.as_slice(), ERRC, PLANE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::INRYPL(
        ERRV.as_slice(),
        ERRD.as_slice(),
        PLANE.as_slice(),
        &mut NXPTS,
        XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(ZEROVECTOR)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Exception:  ray\'s vertex is just too big.", ctx)?;

    spicelib::CLEARD(3, ERRV.as_slice_mut());
    spicelib::CLEARD(3, ERRD.as_slice_mut());
    spicelib::CLEARD(3, ERRN.as_slice_mut());

    ERRV[3] = 100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0;
    ERRD[3] = -1.0;
    ERRN[3] = 1.0;
    ERRC = 0.0;

    spicelib::NVC2PL(ERRN.as_slice(), ERRC, PLANE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::INRYPL(
        ERRV.as_slice(),
        ERRD.as_slice(),
        PLANE.as_slice(),
        &mut NXPTS,
        XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VECTORTOOBIG)", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Exception:  Plane is too far from the origin.", ctx)?;

    spicelib::CLEARD(3, ERRV.as_slice_mut());
    spicelib::CLEARD(3, ERRD.as_slice_mut());
    spicelib::CLEARD(3, ERRN.as_slice_mut());

    ERRV[3] = 1.0;
    ERRD[3] = -1.0;
    ERRN[3] = 1.0;
    ERRC = 100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0;

    spicelib::NVC2PL(ERRN.as_slice(), ERRC, PLANE.as_slice_mut(), ctx)?;
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    spicelib::INRYPL(
        ERRV.as_slice(),
        ERRD.as_slice(),
        PLANE.as_slice(),
        &mut NXPTS,
        XPT.as_slice_mut(),
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(VECTORTOOBIG)", OK, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
