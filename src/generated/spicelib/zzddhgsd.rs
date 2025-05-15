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
const LOCSIZ: i32 = 8;
const NUMCLS: i32 = 3;
const METHOD: i32 = 1;
const ARCH: i32 = (METHOD + 1);
const BFF: i32 = (ARCH + 1);

struct SaveVars {
    CLSLST: ActualCharArray,
    STRAMH: ActualCharArray,
    STRARC: ActualCharArray,
    STRBFF: ActualCharArray,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut CLSLST = ActualCharArray::new(LOCSIZ, 1..=NUMCLS);
        let mut STRAMH = ActualCharArray::new(STRSIZ, 1..=NUMAMH);
        let mut STRARC = ActualCharArray::new(STRSIZ, 1..=NUMARC);
        let mut STRBFF = ActualCharArray::new(STRSIZ, 1..=NUMBFF);

        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"METHOD"), Val::C(b"ARCH"), Val::C(b"BFF")].into_iter();
            CLSLST
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"READ"),
                Val::C(b"WRITE"),
                Val::C(b"SCRATCH"),
                Val::C(b"NEW"),
            ]
            .into_iter();
            STRAMH
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"DAF"), Val::C(b"DAS")].into_iter();
            STRARC
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"BIG-IEEE"),
                Val::C(b"LTL-IEEE"),
                Val::C(b"VAX-GFLT"),
                Val::C(b"VAX-DFLT"),
            ]
            .into_iter();
            STRBFF
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            CLSLST,
            STRAMH,
            STRARC,
            STRBFF,
        }
    }
}

//$Procedure ZZDDHGSD ( Private --- DDH Get String Definitions )
pub fn ZZDDHGSD(CLASS: &[u8], ID: i32, LABEL: &mut [u8], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut TMPSTR = [b' '; LOCSIZ as usize];
    let mut CLSID: i32 = 0;

    //
    // SPICELIB Functions
    //

    //
    // Local Parameters
    //

    //
    // Number of classes of label/ID pairs of which this routine is
    // aware.
    //

    //
    // Integer codes for the currently supported classes.
    //

    //
    // Local Variables
    //

    //
    // Saved Variables
    //

    //
    // Data Statements
    //

    //
    // Left justify and convert the input to upper case.
    //
    LJUST(CLASS, &mut TMPSTR);
    UCASE(&TMPSTR.clone(), &mut TMPSTR, ctx);

    CLSID = ISRCHC(&TMPSTR, NUMCLS, save.CLSLST.as_arg());

    //
    // Initialize LABEL to the default response.
    //
    fstr::assign(LABEL, b" ");

    //
    // Branch on CLSID and return the appropriate label as requested
    // by ID.
    //
    if (((CLSID == METHOD) && (ID >= 1)) && (ID <= NUMAMH)) {
        fstr::assign(LABEL, save.STRAMH.get(ID));
    } else if (((CLSID == ARCH) && (ID >= 1)) && (ID <= NUMARC)) {
        fstr::assign(LABEL, save.STRARC.get(ID));
    } else if (((CLSID == BFF) && (ID >= 1)) && (ID <= NUMBFF)) {
        fstr::assign(LABEL, save.STRBFF.get(ID));
    }
}
