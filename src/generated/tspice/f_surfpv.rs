//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const TIGHT: f64 = 0.0000000001;
const MSGLEN: i32 = 400;
const NSIMPL: i32 = 8;
const NRANDM: i32 = 5000;

struct SaveVars {
    TITLE: Vec<u8>,
    DESCR: ActualCharArray,
    A: f64,
    B: f64,
    C: f64,
    LEVEL: f64,
    SFACTR: f64,
    SMPA: StackArray<f64, 8>,
    SMPB: StackArray<f64, 8>,
    SMPC: StackArray<f64, 8>,
    SMPDST: StackArray2D<f64, 48>,
    SMPVST: StackArray2D<f64, 48>,
    STDIR: StackArray<f64, 6>,
    STVRTX: StackArray<f64, 6>,
    STX: StackArray<f64, 6>,
    XSTX: StackArray<f64, 6>,
    SEED: i32,
    FOUND: bool,
    SMPFND: StackArray<bool, 8>,
    XFOUND: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut TITLE = vec![b' '; MSGLEN as usize];
        let mut DESCR = ActualCharArray::new(MSGLEN, 1..=NSIMPL);
        let mut A: f64 = 0.0;
        let mut B: f64 = 0.0;
        let mut C: f64 = 0.0;
        let mut LEVEL: f64 = 0.0;
        let mut SFACTR: f64 = 0.0;
        let mut SMPA = StackArray::<f64, 8>::new(1..=NSIMPL);
        let mut SMPB = StackArray::<f64, 8>::new(1..=NSIMPL);
        let mut SMPC = StackArray::<f64, 8>::new(1..=NSIMPL);
        let mut SMPDST = StackArray2D::<f64, 48>::new(1..=6, 1..=NSIMPL);
        let mut SMPVST = StackArray2D::<f64, 48>::new(1..=6, 1..=NSIMPL);
        let mut STDIR = StackArray::<f64, 6>::new(1..=6);
        let mut STVRTX = StackArray::<f64, 6>::new(1..=6);
        let mut STX = StackArray::<f64, 6>::new(1..=6);
        let mut XSTX = StackArray::<f64, 6>::new(1..=6);
        let mut SEED: i32 = 0;
        let mut FOUND: bool = false;
        let mut SMPFND = StackArray::<bool, 8>::new(1..=NSIMPL);
        let mut XFOUND: bool = false;

        {
            use f2rust_std::data::Val;

            let mut clist =
                [Val::C(b"Vertex moves; direction vector has zero velocity.")].into_iter();
            fstr::assign(DESCR.get_mut(1), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(100.0), Val::D(100.0), Val::D(100.0)].into_iter();
            SMPA[1] = clist.next().unwrap().into_f64();
            SMPB[1] = clist.next().unwrap().into_f64();
            SMPC[1] = clist.next().unwrap().into_f64();

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(200.0), Val::D(0.0), Val::D(0.0)].into_iter();
            for I in intrinsics::range(1, 3, 1) {
                SMPVST[[I, 1]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(0.0), Val::D(0.0), Val::D(1.0)].into_iter();
            for I in intrinsics::range(4, 6, 1) {
                SMPVST[[I, 1]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(-1.0), Val::D(0.0), Val::D(0.0)].into_iter();
            for I in intrinsics::range(1, 3, 1) {
                SMPDST[[I, 1]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(0.0), Val::D(0.0), Val::D(0.0)].into_iter();
            for I in intrinsics::range(4, 6, 1) {
                SMPDST[[I, 1]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::L(true)].into_iter();
            SMPFND[1] = clist.next().unwrap().into_bool();

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(
                b"Direction vector scans upward; vertex has zero velocity.",
            )]
            .into_iter();
            fstr::assign(DESCR.get_mut(2), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(100.0), Val::D(200.0), Val::D(300.0)].into_iter();
            SMPA[2] = clist.next().unwrap().into_f64();
            SMPB[2] = clist.next().unwrap().into_f64();
            SMPC[2] = clist.next().unwrap().into_f64();

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(200.0), Val::D(0.0), Val::D(0.0)].into_iter();
            for I in intrinsics::range(1, 3, 1) {
                SMPVST[[I, 2]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(0.0), Val::D(0.0), Val::D(0.0)].into_iter();
            for I in intrinsics::range(4, 6, 1) {
                SMPVST[[I, 2]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(-1.0), Val::D(0.0), Val::D(0.0)].into_iter();
            for I in intrinsics::range(1, 3, 1) {
                SMPDST[[I, 2]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(0.0), Val::D(0.0), Val::D(1.0)].into_iter();
            for I in intrinsics::range(4, 6, 1) {
                SMPDST[[I, 2]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::L(true)].into_iter();
            SMPFND[2] = clist.next().unwrap().into_bool();

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"No intersection; ray passes above target.")].into_iter();
            fstr::assign(DESCR.get_mut(3), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(100.0), Val::D(200.0), Val::D(300.0)].into_iter();
            SMPA[3] = clist.next().unwrap().into_f64();
            SMPB[3] = clist.next().unwrap().into_f64();
            SMPC[3] = clist.next().unwrap().into_f64();

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(200.0), Val::D(0.0), Val::D(310.0)].into_iter();
            for I in intrinsics::range(1, 3, 1) {
                SMPVST[[I, 3]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(0.0), Val::D(0.0), Val::D(0.0)].into_iter();
            for I in intrinsics::range(4, 6, 1) {
                SMPVST[[I, 3]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(-1.0), Val::D(0.0), Val::D(0.0)].into_iter();
            for I in intrinsics::range(1, 3, 1) {
                SMPDST[[I, 3]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(0.0), Val::D(0.0), Val::D(1.0)].into_iter();
            for I in intrinsics::range(4, 6, 1) {
                SMPDST[[I, 3]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::L(false)].into_iter();
            SMPFND[3] = clist.next().unwrap().into_bool();

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"No intersection; ray points away from target.")].into_iter();
            fstr::assign(DESCR.get_mut(4), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(100.0), Val::D(200.0), Val::D(300.0)].into_iter();
            SMPA[4] = clist.next().unwrap().into_f64();
            SMPB[4] = clist.next().unwrap().into_f64();
            SMPC[4] = clist.next().unwrap().into_f64();

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(200.0), Val::D(0.0), Val::D(0.0)].into_iter();
            for I in intrinsics::range(1, 3, 1) {
                SMPVST[[I, 4]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(0.0), Val::D(0.0), Val::D(0.0)].into_iter();
            for I in intrinsics::range(4, 6, 1) {
                SMPVST[[I, 4]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(1.0), Val::D(0.0), Val::D(0.0)].into_iter();
            for I in intrinsics::range(1, 3, 1) {
                SMPDST[[I, 4]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(0.0), Val::D(0.0), Val::D(1.0)].into_iter();
            for I in intrinsics::range(4, 6, 1) {
                SMPDST[[I, 4]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::L(false)].into_iter();
            SMPFND[4] = clist.next().unwrap().into_bool();

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(
                b"Velocity not computable: ray scans upward too fast.",
            )]
            .into_iter();
            fstr::assign(DESCR.get_mut(5), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(1.0), Val::D(2.0), Val::D(3.0)].into_iter();
            SMPA[5] = clist.next().unwrap().into_f64();
            SMPB[5] = clist.next().unwrap().into_f64();
            SMPC[5] = clist.next().unwrap().into_f64();

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(2.0), Val::D(0.0), Val::D(0.0)].into_iter();
            for I in intrinsics::range(1, 3, 1) {
                SMPVST[[I, 5]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(0.0), Val::D(0.0), Val::D(0.0)].into_iter();
            for I in intrinsics::range(4, 6, 1) {
                SMPVST[[I, 5]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(-1.0), Val::D(0.0), Val::D(0.0)].into_iter();
            for I in intrinsics::range(1, 3, 1) {
                SMPDST[[I, 5]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
    Val::D(0.0),
    Val::D(0.0),
    Val::D(100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0),
  ].into_iter();
            for I in intrinsics::range(4, 6, 1) {
                SMPDST[[I, 5]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::L(false)].into_iter();
            SMPFND[5] = clist.next().unwrap().into_bool();

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(
                b"Velocity not computable: vertex moves upward too fast.",
            )]
            .into_iter();
            fstr::assign(DESCR.get_mut(6), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(1.0), Val::D(2.0), Val::D(3.0)].into_iter();
            SMPA[6] = clist.next().unwrap().into_f64();
            SMPB[6] = clist.next().unwrap().into_f64();
            SMPC[6] = clist.next().unwrap().into_f64();

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(2.0), Val::D(0.0), Val::D(2.0)].into_iter();
            for I in intrinsics::range(1, 3, 1) {
                SMPVST[[I, 6]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
    Val::D(0.0),
    Val::D(0.0),
    Val::D(150000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0),
  ].into_iter();
            for I in intrinsics::range(4, 6, 1) {
                SMPVST[[I, 6]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(-1.0), Val::D(0.0), Val::D(0.0)].into_iter();
            for I in intrinsics::range(1, 3, 1) {
                SMPDST[[I, 6]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(0.0), Val::D(0.0), Val::D(0.0)].into_iter();
            for I in intrinsics::range(4, 6, 1) {
                SMPDST[[I, 6]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::L(false)].into_iter();
            SMPFND[6] = clist.next().unwrap().into_bool();

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"Interior vertex; vertex not at origin.")].into_iter();
            fstr::assign(DESCR.get_mut(7), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(100.0), Val::D(200.0), Val::D(300.0)].into_iter();
            SMPA[7] = clist.next().unwrap().into_f64();
            SMPB[7] = clist.next().unwrap().into_f64();
            SMPC[7] = clist.next().unwrap().into_f64();

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(10.0), Val::D(0.0), Val::D(0.0)].into_iter();
            for I in intrinsics::range(1, 3, 1) {
                SMPVST[[I, 7]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(0.0), Val::D(0.0), Val::D(0.0)].into_iter();
            for I in intrinsics::range(4, 6, 1) {
                SMPVST[[I, 7]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(1.0), Val::D(0.0), Val::D(0.0)].into_iter();
            for I in intrinsics::range(1, 3, 1) {
                SMPDST[[I, 7]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(0.0), Val::D(0.0), Val::D(1.0)].into_iter();
            for I in intrinsics::range(4, 6, 1) {
                SMPDST[[I, 7]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::L(true)].into_iter();
            SMPFND[7] = clist.next().unwrap().into_bool();

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"Interior vertex; vertex is at origin.")].into_iter();
            fstr::assign(DESCR.get_mut(8), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(100.0), Val::D(200.0), Val::D(300.0)].into_iter();
            SMPA[8] = clist.next().unwrap().into_f64();
            SMPB[8] = clist.next().unwrap().into_f64();
            SMPC[8] = clist.next().unwrap().into_f64();

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(0.0), Val::D(0.0), Val::D(0.0)].into_iter();
            for I in intrinsics::range(1, 3, 1) {
                SMPVST[[I, 8]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(0.0), Val::D(0.0), Val::D(0.0)].into_iter();
            for I in intrinsics::range(4, 6, 1) {
                SMPVST[[I, 8]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(1.0), Val::D(0.0), Val::D(0.0)].into_iter();
            for I in intrinsics::range(1, 3, 1) {
                SMPDST[[I, 8]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(0.0), Val::D(0.0), Val::D(1.0)].into_iter();
            for I in intrinsics::range(4, 6, 1) {
                SMPDST[[I, 8]] = clist.next().unwrap().into_f64();
            }

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::L(true)].into_iter();
            SMPFND[8] = clist.next().unwrap().into_bool();

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            TITLE,
            DESCR,
            A,
            B,
            C,
            LEVEL,
            SFACTR,
            SMPA,
            SMPB,
            SMPC,
            SMPDST,
            SMPVST,
            STDIR,
            STVRTX,
            STX,
            XSTX,
            SEED,
            FOUND,
            SMPFND,
            XFOUND,
        }
    }
}

//$Procedure F_SURFPV ( SURFPV tests )
pub fn F_SURFPV(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

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
    // Saved everything.
    //

    //
    // Initial values
    //
    // The simple test cases.
    //

    //
    // Open the test family.
    //
    testutil::TOPEN(b"F_SURFPV", ctx)?;

    //
    // *****************************************************************
    //
    // Error cases: SURFPV
    //
    // *****************************************************************
    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"SURFPV: zero direction vector,", ctx)?;

    spicelib::FILLD(10.0, 6, save.STVRTX.as_slice_mut());
    spicelib::CLEARD(6, save.STDIR.as_slice_mut());
    spicelib::SURFPV(
        save.STVRTX.as_slice(),
        save.STDIR.as_slice(),
        save.A,
        save.B,
        save.C,
        save.STX.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;

    testutil::CHCKXC(true, b"SPICE(ZEROVECTOR)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"SURFPV: ellipsoid has one zero-length axis.", ctx)?;

    spicelib::FILLD(10.0, 6, save.STVRTX.as_slice_mut());
    spicelib::FILLD(-1.0, 6, save.STDIR.as_slice_mut());

    spicelib::SURFPV(
        save.STVRTX.as_slice(),
        save.STDIR.as_slice(),
        0.0,
        1.0,
        1.0,
        save.STX.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADAXISLENGTH)", OK, ctx)?;

    spicelib::SURFPV(
        save.STVRTX.as_slice(),
        save.STDIR.as_slice(),
        1.0,
        0.0,
        1.0,
        save.STX.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADAXISLENGTH)", OK, ctx)?;

    spicelib::SURFPV(
        save.STVRTX.as_slice(),
        save.STDIR.as_slice(),
        1.0,
        1.0,
        0.0,
        save.STX.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADAXISLENGTH)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"SURFPV: ellipsoid has one negative axis.", ctx)?;

    spicelib::FILLD(10.0, 6, save.STVRTX.as_slice_mut());
    spicelib::FILLD(-1.0, 6, save.STDIR.as_slice_mut());

    spicelib::SURFPV(
        save.STVRTX.as_slice(),
        save.STDIR.as_slice(),
        -1.0,
        1.0,
        1.0,
        save.STX.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADAXISLENGTH)", OK, ctx)?;

    spicelib::SURFPV(
        save.STVRTX.as_slice(),
        save.STDIR.as_slice(),
        1.0,
        -1.0,
        1.0,
        save.STX.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADAXISLENGTH)", OK, ctx)?;

    spicelib::SURFPV(
        save.STVRTX.as_slice(),
        save.STDIR.as_slice(),
        1.0,
        1.0,
        -1.0,
        save.STX.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(BADAXISLENGTH)", OK, ctx)?;

    //
    // --- Case: -------------------------------------------------------
    //
    testutil::TCASE(b"SURFPV: ray\'s vertex is on the ellipsoid", ctx)?;

    spicelib::FILLD(0.0, 3, save.STVRTX.as_slice_mut());
    save.STVRTX[1] = 1.0;
    spicelib::FILLD(1.0, 3, save.STVRTX.subarray_mut(4));

    spicelib::FILLD(-1.0, 6, save.STDIR.as_slice_mut());

    spicelib::SURFPV(
        save.STVRTX.as_slice(),
        save.STDIR.as_slice(),
        1.0,
        1.0,
        1.0,
        save.STX.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDVERTEX)", OK, ctx)?;

    testutil::T_SURFPV(
        save.STVRTX.as_slice(),
        save.STDIR.as_slice(),
        1.0,
        1.0,
        1.0,
        save.STX.as_slice_mut(),
        &mut save.FOUND,
        ctx,
    )?;
    testutil::CHCKXC(true, b"SPICE(INVALIDVERTEX)", OK, ctx)?;

    //
    // *****************************************************************
    //
    // Normal cases: SURFPV
    //
    // *****************************************************************
    //
    // Run some simple tests where the correct results can be
    // determined by inspection.
    //
    for I in 1..=NSIMPL {
        //
        // --- Case: ----------------------------------------------------
        //
        testutil::TCASE(&save.DESCR[I], ctx)?;

        spicelib::SURFPV(
            save.SMPVST.subarray([1, I]),
            save.SMPDST.subarray([1, I]),
            save.SMPA[I],
            save.SMPB[I],
            save.SMPC[I],
            save.STX.as_slice_mut(),
            &mut save.FOUND,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(b"FOUND (0)", save.FOUND, save.SMPFND[I], OK, ctx)?;

        // IF ( FOUND ) THEN
        //    WRITE (*,*) 'STX from SURFPV: ', STX
        // END IF

        //
        // Repeat the computation with T_SURFPV.
        //
        testutil::T_SURFPV(
            save.SMPVST.subarray([1, I]),
            save.SMPDST.subarray([1, I]),
            save.SMPA[I],
            save.SMPB[I],
            save.SMPC[I],
            save.XSTX.as_slice_mut(),
            &mut save.FOUND,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(b"FOUND (1)", save.FOUND, save.SMPFND[I], OK, ctx)?;

        if save.SMPFND[I] {
            //
            // Check the intercept state against that found
            // by T_SURFPV.
            //
            testutil::CHCKAD(
                b"STX(1:3)",
                save.STX.as_slice(),
                b"~~/",
                save.XSTX.as_slice(),
                3,
                TIGHT,
                OK,
                ctx,
            )?;

            testutil::CHCKAD(
                b"STX(4:6)",
                save.STX.subarray(4),
                b"~~/",
                save.XSTX.subarray(4),
                3,
                TIGHT,
                OK,
                ctx,
            )?;
        }
    }

    //
    // Now for some more difficult cases.  We'll generate the ellipsoids
    // and lines using random numbers.  There are 16 components to
    // generate:
    //
    //    - random ray vertices and vertex velocities
    //    - random ray directions and direction velocities
    //    - random ellipsoid axis lengths
    //    - random scale factors for the ellipsoid and ray
    //
    save.SEED = -1;

    for I in 1..=NRANDM {
        //
        // --- Case: ---------------------------------------------------
        //
        //
        // Get a scale factor.
        //
        save.SFACTR = f64::powf(10.0, testutil::T_RANDD(-290.0, 290.0, &mut save.SEED, ctx)?);

        //
        // Make up ellipsoid axis lengths.
        //
        save.A = (save.SFACTR * testutil::T_RANDD(1.0, 2.0, &mut save.SEED, ctx)?);
        save.B = (save.SFACTR * testutil::T_RANDD(1.0, 2.0, &mut save.SEED, ctx)?);
        save.C = (save.SFACTR * testutil::T_RANDD(1.0, 2.0, &mut save.SEED, ctx)?);

        //
        // We gotta have a ray vertex and vertex velocity.
        //
        for J in 1..=6 {
            save.STVRTX[J] =
                (((10 as f64) * save.SFACTR) * testutil::T_RANDD(-1.0, 1.0, &mut save.SEED, ctx)?);
        }

        //
        // Compute the level surface parameter of the vertex.
        //
        save.LEVEL = ((f64::powi((save.STVRTX[1] / save.A), 2)
            + f64::powi((save.STVRTX[2] / save.B), 2))
            + f64::powi((save.STVRTX[3] / save.C), 2));

        //
        // Alternate between exterior and interior cases.
        //
        if spicelib::ODD(I) {
            //
            // If necessary, scale up the vertex to take it outside the
            // ellipsoid.
            //
            if (save.LEVEL < 1.0) {
                if (save.LEVEL > 0.0) {
                    spicelib::VSCLIP((2.0 / f64::sqrt(save.LEVEL)), save.STVRTX.as_slice_mut());
                } else {
                    //
                    // It's unlikely that we get here: the vertex is at
                    // the origin. Pick an exterior vertex.
                    //
                    spicelib::VPACK(
                        (2.0 * save.A),
                        (2.0 * save.B),
                        (2.0 * save.C),
                        save.STVRTX.as_slice_mut(),
                    );
                }
            }
        } else {
            //
            // Ensure the vertex is inside the ellipsoid.
            //
            if (save.LEVEL >= 1.0) {
                spicelib::VSCLIP((0.5 / f64::sqrt(save.LEVEL)), save.STVRTX.as_slice_mut());
            }
        }

        //
        // Generate the ray's direction vector by perturbing the
        // inverse of the ray's vertex.
        //
        for J in 1..=3 {
            save.STDIR[J] = (-save.STVRTX[J]
                + (save.SFACTR * testutil::T_RANDD(-1.0, 1.0, &mut save.SEED, ctx)?));
        }

        for J in 4..=6 {
            save.STDIR[J] = (save.SFACTR * testutil::T_RANDD(-1.0, 1.0, &mut save.SEED, ctx)?);
        }

        fstr::assign(&mut save.TITLE, b"SURFPV Random case #.  A, B, C = # # #; STVRTX = (#, #, #, #, #, #) STDIR  = (#, #, #, #, #, #)");
        spicelib::REPMI(&save.TITLE.to_vec(), b"#", I, &mut save.TITLE, ctx);
        spicelib::REPMD(&save.TITLE.to_vec(), b"#", save.A, 14, &mut save.TITLE, ctx);
        spicelib::REPMD(&save.TITLE.to_vec(), b"#", save.B, 14, &mut save.TITLE, ctx);
        spicelib::REPMD(&save.TITLE.to_vec(), b"#", save.C, 14, &mut save.TITLE, ctx);
        spicelib::REPMD(
            &save.TITLE.to_vec(),
            b"#",
            save.STVRTX[1],
            14,
            &mut save.TITLE,
            ctx,
        );
        spicelib::REPMD(
            &save.TITLE.to_vec(),
            b"#",
            save.STVRTX[2],
            14,
            &mut save.TITLE,
            ctx,
        );
        spicelib::REPMD(
            &save.TITLE.to_vec(),
            b"#",
            save.STVRTX[3],
            14,
            &mut save.TITLE,
            ctx,
        );
        spicelib::REPMD(
            &save.TITLE.to_vec(),
            b"#",
            save.STVRTX[4],
            14,
            &mut save.TITLE,
            ctx,
        );
        spicelib::REPMD(
            &save.TITLE.to_vec(),
            b"#",
            save.STVRTX[5],
            14,
            &mut save.TITLE,
            ctx,
        );
        spicelib::REPMD(
            &save.TITLE.to_vec(),
            b"#",
            save.STVRTX[6],
            14,
            &mut save.TITLE,
            ctx,
        );
        spicelib::REPMD(
            &save.TITLE.to_vec(),
            b"#",
            save.STDIR[1],
            14,
            &mut save.TITLE,
            ctx,
        );
        spicelib::REPMD(
            &save.TITLE.to_vec(),
            b"#",
            save.STDIR[2],
            14,
            &mut save.TITLE,
            ctx,
        );
        spicelib::REPMD(
            &save.TITLE.to_vec(),
            b"#",
            save.STDIR[3],
            14,
            &mut save.TITLE,
            ctx,
        );
        spicelib::REPMD(
            &save.TITLE.to_vec(),
            b"#",
            save.STDIR[4],
            14,
            &mut save.TITLE,
            ctx,
        );
        spicelib::REPMD(
            &save.TITLE.to_vec(),
            b"#",
            save.STDIR[5],
            14,
            &mut save.TITLE,
            ctx,
        );
        spicelib::REPMD(
            &save.TITLE.to_vec(),
            b"#",
            save.STDIR[6],
            14,
            &mut save.TITLE,
            ctx,
        );

        testutil::TCASE(&save.TITLE, ctx)?;

        //
        // Cross our fingers and toes and let 'er rip.
        //
        spicelib::SURFPV(
            save.STVRTX.as_slice(),
            save.STDIR.as_slice(),
            save.A,
            save.B,
            save.C,
            save.STX.as_slice_mut(),
            &mut save.FOUND,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::T_SURFPV(
            save.STVRTX.as_slice(),
            save.STDIR.as_slice(),
            save.A,
            save.B,
            save.C,
            save.XSTX.as_slice_mut(),
            &mut save.XFOUND,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(b"FOUND (0)", save.FOUND, save.XFOUND, OK, ctx)?;

        if save.XFOUND {
            //
            // Check the intercept state against that found
            // by T_SURFPV.
            //
            testutil::CHCKAD(
                b"STX(1:3)",
                save.STX.as_slice(),
                b"~~/",
                save.XSTX.as_slice(),
                3,
                TIGHT,
                OK,
                ctx,
            )?;

            testutil::CHCKAD(
                b"STX(4:6)",
                save.STX.subarray(4),
                b"~~/",
                save.XSTX.subarray(4),
                3,
                TIGHT,
                OK,
                ctx,
            )?;

            // WRITE (*,*) '-----------------'
            // WRITE (*,*) 'Case ', I
            // WRITE (*,*) 'STX:  ', STX
            // WRITE (*,*) 'XSTX: ', XSTX
        }
    }

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
