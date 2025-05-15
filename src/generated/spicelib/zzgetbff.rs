//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const FTSIZE: i32 = 5000;
const RSVUNT: i32 = 2;
const SCRUNT: i32 = 1;
const UTSIZE: i32 = ((20 + SCRUNT) + RSVUNT);
const READ: i32 = 1;
const WRITE: i32 = 2;
const SCRTCH: i32 = 3;
const NEW: i32 = 4;
const NUMAMH: i32 = 4;
const BIGI3E: i32 = 1;
const LTLI3E: i32 = 2;
const VAXGFL: i32 = 3;
const VAXDFL: i32 = 4;
const NUMBFF: i32 = 4;
const STRSIZ: i32 = 8;
const STRLEN: i32 = ((STRSIZ + 1) * NUMBFF);
const DAF: i32 = 1;
const DAS: i32 = 2;
const NUMARC: i32 = 2;
const RECL: i32 = 1024;
const FILEN: i32 = 255;
const CBFSIZ: i32 = 1024;
const DPVALU: f64 = 7.0;

struct SaveVars {
    INT1ST: StackArray<i32, 4>,
    INT2ND: StackArray<i32, 4>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut INT1ST = StackArray::<i32, 4>::new(1..=NUMBFF);
        let mut INT2ND = StackArray::<i32, 4>::new(1..=NUMBFF);

        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(1075576832)].into_iter();
            INT1ST[BIGI3E] = clist.next().unwrap().into_i32();

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(0)].into_iter();
            INT2ND[BIGI3E] = clist.next().unwrap().into_i32();

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(0)].into_iter();
            INT1ST[LTLI3E] = clist.next().unwrap().into_i32();

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(1075576832)].into_iter();
            INT2ND[LTLI3E] = clist.next().unwrap().into_i32();

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(16444)].into_iter();
            INT1ST[VAXGFL] = clist.next().unwrap().into_i32();

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(0)].into_iter();
            INT2ND[VAXGFL] = clist.next().unwrap().into_i32();

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(16864)].into_iter();
            INT1ST[VAXDFL] = clist.next().unwrap().into_i32();

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(0)].into_iter();
            INT2ND[VAXDFL] = clist.next().unwrap().into_i32();

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self { INT1ST, INT2ND }
    }
}

//$Procedure ZZGETBFF ( Private --- Get Binary File Format )
pub fn ZZGETBFF(BFFID: &mut i32, ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut DEQUIV: f64 = 0.0;
    let IEQUIV = StackArray::<i32, 2>::new(1..=2);

    //
    // Local Parameters
    //

    //
    // Local Variables
    //

    //
    // Copy DPVALU into the equivalenced DP, DEQUIV.
    //
    DEQUIV = DPVALU;

    //
    // Examine the integer pairs, to identify the binary
    // file format.
    //
    *BFFID = 0;

    for I in 1..=NUMBFF {
        if ((DummyArray::<i32>::from_equiv(&[DEQUIV], 1..=2)[1] == save.INT1ST[I])
            && (DummyArray::<i32>::from_equiv(&[DEQUIV], 1..=2)[2] == save.INT2ND[I]))
        {
            *BFFID = I;
        }
    }
}
