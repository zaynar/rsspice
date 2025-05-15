//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const YES: bool = true;
const NO: bool = false;
const LOWER: i32 = 1;
const UPPER: i32 = (LOWER + 1);
const MIXED: i32 = (UPPER + 1);
const FULL: i32 = 1;
const SHORT: i32 = (FULL + 1);
const MAXTKN: i32 = 64;
const PICLEN: i32 = (MAXTKN * 5);
const WDSIZE: i32 = 32;
const NRECOG: i32 = 70;
const SMWDSZ: i32 = 12;

struct SaveVars {
    ZZTIME: bool,
    ZZCMBT: bool,
    ZZGREP: bool,
    ZZISPT: bool,
    ZZIST: bool,
    ZZNOTE: bool,
    ZZREMT: bool,
    ZZSUBT: bool,
    ZZTOKNS: bool,
    ZZUNPCK: bool,
    ZZVALT: bool,
    REP: Vec<u8>,
    SIZE: i32,
    BEGS: StackArray<i32, 64>,
    ENDS: StackArray<i32, 64>,
    PBEGS: StackArray<i32, 64>,
    PENDS: StackArray<i32, 64>,
    PICTUR: Vec<u8>,
    NAMES: ActualCharArray,
    MYERR: Vec<u8>,
    MESSGE: Vec<u8>,
    PICERR: Vec<u8>,
    TKNERR: Vec<u8>,
    MONTH: Vec<u8>,
    MONTHS: ActualCharArray,
    HMS: StackArray<f64, 3>,
    BLANK: i32,
    FROM: i32,
    I: i32,
    ITEM: i32,
    J: i32,
    K: i32,
    LAST: i32,
    NCHAR: i32,
    NEXT: i32,
    P1: i32,
    P2: i32,
    PTR: i32,
    PUT: i32,
    R: i32,
    TO: i32,
    W: i32,
    NJD: i32,
    PFROM: i32,
    PTO: i32,
    PNEXT: i32,
    CASE: i32,
    KIND: i32,
    NYEAR: i32,
    NMON: i32,
    NDAY: i32,
    NHOUR: i32,
    NMIN: i32,
    NSEC: i32,
    NDOY: i32,
    VALUE: i32,
    MNSIZE: StackArray<i32, 2>,
    WKSIZE: StackArray<i32, 2>,
    WIDTH: StackArray<i32, 70>,
    RECOG: ActualCharArray,
    SPCIAL: Vec<u8>,
    MNMRK: ActualCharArray2D,
    WKDAY: ActualCharArray2D,
    CLASS: ActualCharArray,
    THIS: Vec<u8>,
    F: StackArray<i32, 95>,
    L: StackArray<i32, 95>,
    AMPM: bool,
    DID: bool,
    GOT: bool,
    CHECK: bool,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ZZTIME: bool = false;
        let mut ZZCMBT: bool = false;
        let mut ZZGREP: bool = false;
        let mut ZZISPT: bool = false;
        let mut ZZIST: bool = false;
        let mut ZZNOTE: bool = false;
        let mut ZZREMT: bool = false;
        let mut ZZSUBT: bool = false;
        let mut ZZTOKNS: bool = false;
        let mut ZZUNPCK: bool = false;
        let mut ZZVALT: bool = false;
        let mut REP = vec![b' '; MAXTKN as usize];
        let mut SIZE: i32 = 0;
        let mut BEGS = StackArray::<i32, 64>::new(1..=MAXTKN);
        let mut ENDS = StackArray::<i32, 64>::new(1..=MAXTKN);
        let mut PBEGS = StackArray::<i32, 64>::new(1..=MAXTKN);
        let mut PENDS = StackArray::<i32, 64>::new(1..=MAXTKN);
        let mut PICTUR = vec![b' '; PICLEN as usize];
        let mut NAMES = ActualCharArray::new(WDSIZE, 32..=126);
        let mut MYERR = vec![b' '; WDSIZE as usize];
        let mut MESSGE = vec![b' '; PICLEN as usize];
        let mut PICERR = vec![b' '; PICLEN as usize];
        let mut TKNERR = vec![b' '; PICLEN as usize];
        let mut MONTH = vec![b' '; 3 as usize];
        let mut MONTHS = ActualCharArray::new(3, 1..=12);
        let mut HMS = StackArray::<f64, 3>::new(1..=3);
        let mut BLANK: i32 = 0;
        let mut FROM: i32 = 0;
        let mut I: i32 = 0;
        let mut ITEM: i32 = 0;
        let mut J: i32 = 0;
        let mut K: i32 = 0;
        let mut LAST: i32 = 0;
        let mut NCHAR: i32 = 0;
        let mut NEXT: i32 = 0;
        let mut P1: i32 = 0;
        let mut P2: i32 = 0;
        let mut PTR: i32 = 0;
        let mut PUT: i32 = 0;
        let mut R: i32 = 0;
        let mut TO: i32 = 0;
        let mut W: i32 = 0;
        let mut NJD: i32 = 0;
        let mut PFROM: i32 = 0;
        let mut PTO: i32 = 0;
        let mut PNEXT: i32 = 0;
        let mut CASE: i32 = 0;
        let mut KIND: i32 = 0;
        let mut NYEAR: i32 = 0;
        let mut NMON: i32 = 0;
        let mut NDAY: i32 = 0;
        let mut NHOUR: i32 = 0;
        let mut NMIN: i32 = 0;
        let mut NSEC: i32 = 0;
        let mut NDOY: i32 = 0;
        let mut VALUE: i32 = 0;
        let mut MNSIZE = StackArray::<i32, 2>::new(1..=SHORT);
        let mut WKSIZE = StackArray::<i32, 2>::new(1..=SHORT);
        let mut WIDTH = StackArray::<i32, 70>::new(1..=NRECOG);
        let mut RECOG = ActualCharArray::new(SMWDSZ, 1..=NRECOG);
        let mut SPCIAL = vec![b' '; SMWDSZ as usize];
        let mut MNMRK = ActualCharArray2D::new(SMWDSZ, 1..=MIXED, 1..=SHORT);
        let mut WKDAY = ActualCharArray2D::new(SMWDSZ, 1..=MIXED, 1..=SHORT);
        let mut CLASS = ActualCharArray::new(1, 1..=NRECOG);
        let mut THIS = vec![b' '; 1 as usize];
        let mut F = StackArray::<i32, 95>::new(32..=126);
        let mut L = StackArray::<i32, 95>::new(32..=126);
        let mut AMPM: bool = false;
        let mut DID: bool = false;
        let mut GOT: bool = false;
        let mut CHECK: bool = false;
        let mut FIRST: bool = false;

        SIZE = 0;
        FIRST = true;
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"JAN"),
                Val::C(b"FEB"),
                Val::C(b"MAR"),
                Val::C(b"APR"),
                Val::C(b"MAY"),
                Val::C(b"JUN"),
                Val::C(b"JUL"),
                Val::C(b"AUG"),
                Val::C(b"SEP"),
                Val::C(b"OCT"),
                Val::C(b"NOV"),
                Val::C(b"DEC"),
            ]
            .into_iter();
            MONTHS
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            ZZTIME,
            ZZCMBT,
            ZZGREP,
            ZZISPT,
            ZZIST,
            ZZNOTE,
            ZZREMT,
            ZZSUBT,
            ZZTOKNS,
            ZZUNPCK,
            ZZVALT,
            REP,
            SIZE,
            BEGS,
            ENDS,
            PBEGS,
            PENDS,
            PICTUR,
            NAMES,
            MYERR,
            MESSGE,
            PICERR,
            TKNERR,
            MONTH,
            MONTHS,
            HMS,
            BLANK,
            FROM,
            I,
            ITEM,
            J,
            K,
            LAST,
            NCHAR,
            NEXT,
            P1,
            P2,
            PTR,
            PUT,
            R,
            TO,
            W,
            NJD,
            PFROM,
            PTO,
            PNEXT,
            CASE,
            KIND,
            NYEAR,
            NMON,
            NDAY,
            NHOUR,
            NMIN,
            NSEC,
            NDOY,
            VALUE,
            MNSIZE,
            WKSIZE,
            WIDTH,
            RECOG,
            SPCIAL,
            MNMRK,
            WKDAY,
            CLASS,
            THIS,
            F,
            L,
            AMPM,
            DID,
            GOT,
            CHECK,
            FIRST,
        }
    }
}

//$Procedure ZZTIME ( Private, Time --- time parsing utilities )
pub fn ZZTIME(
    STRING: &[u8],
    TRANSL: &[u8],
    LETTER: &[u8],
    ERROR: &[u8],
    PIC: &[u8],
    TVEC: &[f64],
    B: i32,
    E: i32,
    L2R: bool,
    YABBRV: bool,
    ctx: &mut Context,
) -> f2rust_std::Result<bool> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Entry points
    //

    //
    // SPICELIB Functions
    //

    //
    // Standard Parameters
    //

    //
    // LOWER
    // UPPER
    // MIXED
    //

    //
    // FULL
    // SHORT
    //

    //
    // Maximum number of tokens that a valid time string can contain.
    //

    //
    // Length of the string buffer containing the time string picture.
    //

    //
    // Representation Variables.
    //

    //
    // Token Recognition Variables.
    //
    // At the moment there are 53 recognized substrings, we
    // make room for 70 just so we won't have to increase
    // the parameter NRECOG soon.
    //

    save.ZZTIME = false;

    CHKIN(b"ZZTIME", ctx)?;
    SIGERR(b"SPICE(BOGUSENTRY)", ctx)?;
    CHKOUT(b"ZZTIME", ctx)?;
    Ok(save.ZZTIME)
}

//$Procedure ZZCMBT ( Private, Time --- combine tokens )
pub fn ZZCMBT(STRING: &[u8], LETTER: &[u8], L2R: bool, ctx: &mut Context) -> bool {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let LETTER = &LETTER[..1 as usize];

    //
    // So far we haven't combined anything.
    //
    save.DID = NO;
    //
    // Look for the substring either looking from the
    // left (L2R is YES) or from the right (L2R is NO).
    //
    if L2R {
        save.FROM = POS(fstr::substr(&save.REP, 1..=save.SIZE), STRING, 1);
    } else {
        save.FROM = POSR(fstr::substr(&save.REP, 1..=save.SIZE), STRING, save.SIZE);
    }

    save.TO = ((save.FROM + intrinsics::LEN(STRING)) - 1);

    if (save.FROM > 0) {
        save.DID = YES;
        save.ENDS[save.FROM] = save.ENDS[save.TO];
        save.PENDS[save.FROM] = save.PENDS[save.TO];
        save.PUT = (save.FROM + 1);
        save.NEXT = (save.TO + 1);

        //
        // Perform the substitution in the representation
        //
        ZZREPSUB(
            &save.REP.to_vec(),
            save.FROM,
            save.TO,
            LETTER,
            &mut save.REP,
        );
        //
        // Now update the begins and ends of tokens in the original
        // string.
        //
        for GET in save.NEXT..=save.SIZE {
            save.BEGS[save.PUT] = save.BEGS[GET];
            save.ENDS[save.PUT] = save.ENDS[GET];
            save.PBEGS[save.PUT] = save.PBEGS[GET];
            save.PENDS[save.PUT] = save.PENDS[GET];

            save.PUT = (save.PUT + 1);
        }

        save.SIZE = ((save.SIZE - intrinsics::LEN(STRING)) + 1);
    }

    save.ZZCMBT = save.DID;
    save.ZZCMBT
}

//$Procedure ZZGREP ( Private, Time --- get representation )
pub fn ZZGREP(STRING: &mut [u8], ctx: &mut Context) -> bool {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    fstr::assign(
        STRING,
        fstr::substr(&save.REP, 1..=intrinsics::MAX0(&[1, save.SIZE])),
    );
    save.ZZGREP = true;

    save.ZZGREP
}

//$Procedure ZZISPT ( Private, Time --- is pair of tokens )
pub fn ZZISPT(STRING: &[u8], B: &mut i32, E: &mut i32, ctx: &mut Context) -> bool {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    save.DID = NO;
    save.FROM = CPOS(&save.REP, STRING, 1);

    while (save.FROM > 0) {
        if (save.FROM < save.SIZE) {
            save.TO = (save.FROM + 1);
            save.DID = (intrinsics::INDEX(STRING, fstr::substr(&save.REP, save.TO..=save.TO)) > 0);
        } else {
            *B = 0;
            *E = 0;
            save.ZZISPT = false;
            return save.ZZISPT;
        }

        if save.DID {
            *B = save.BEGS[save.FROM];
            *E = save.ENDS[save.TO];
            save.ZZISPT = true;
            return save.ZZISPT;
        }

        save.FROM = CPOS(&save.REP, STRING, save.TO);
    }

    *B = 0;
    *E = 0;
    save.ZZISPT = false;

    save.ZZISPT
}

//$Procedure ZZIST ( Private, Time --- is there a token )
pub fn ZZIST(LETTER: &[u8], ctx: &mut Context) -> bool {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let LETTER = &LETTER[..1 as usize];

    save.ZZIST = (intrinsics::INDEX(fstr::substr(&save.REP, 1..=save.SIZE), LETTER) > 0);
    save.ZZIST
}

//$Procedure ZZNOTE ( Private, Time --- note the existence and remove )
pub fn ZZNOTE(LETTER: &[u8], B: &mut i32, E: &mut i32, ctx: &mut Context) -> bool {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let LETTER = &LETTER[..1 as usize];

    save.PUT = intrinsics::INDEX(&save.REP, LETTER);

    if (save.PUT > 0) {
        *B = save.BEGS[save.PUT];
        *E = save.ENDS[save.PUT];

        save.NEXT = (save.PUT + 1);

        for GET in save.NEXT..=save.SIZE {
            save.BEGS[save.PUT] = save.BEGS[GET];
            save.ENDS[save.PUT] = save.ENDS[GET];
            save.PBEGS[save.PUT] = save.PBEGS[GET];
            save.PENDS[save.PUT] = save.PENDS[GET];
            let val = fstr::substr(&save.REP, GET..=GET).to_vec();
            fstr::assign(fstr::substr_mut(&mut save.REP, save.PUT..=save.PUT), &val);

            save.PUT = (save.PUT + 1);
        }

        fstr::assign(fstr::substr_mut(&mut save.REP, save.SIZE..), b" ");
        save.SIZE = (save.SIZE - 1);
        save.DID = YES;
    } else {
        *B = 0;
        *E = 0;
        save.DID = NO;
    }

    save.ZZNOTE = save.DID;
    save.ZZNOTE
}

//$Procedure ZZREMT ( Private, Time --- remove token )
pub fn ZZREMT(LETTER: &[u8], ctx: &mut Context) -> bool {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let LETTER = &LETTER[..1 as usize];

    save.PUT = 0;
    save.DID = NO;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.SIZE;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            if (intrinsics::ICHAR(fstr::substr(&save.REP, save.I..=save.I))
                != intrinsics::ICHAR(LETTER))
            {
                save.PUT = (save.PUT + 1);
                let val = fstr::substr(&save.REP, save.I..=save.I).to_vec();
                fstr::assign(fstr::substr_mut(&mut save.REP, save.PUT..=save.PUT), &val);
                save.BEGS[save.PUT] = save.BEGS[save.I];
                save.ENDS[save.PUT] = save.ENDS[save.I];
                save.PBEGS[save.PUT] = save.PBEGS[save.I];
                save.PENDS[save.PUT] = save.PENDS[save.I];
            } else {
                save.DID = YES;
            }

            save.I += m3__;
        }
    }

    save.SIZE = save.PUT;

    if (save.PUT == 0) {
        fstr::assign(&mut save.REP, b" ");
    } else if (save.PUT < intrinsics::LEN(&save.REP)) {
        fstr::assign(fstr::substr_mut(&mut save.REP, (save.PUT + 1)..), b" ");
    }

    save.ZZREMT = save.DID;
    save.ZZREMT
}

//$Procedure ZZSUBT ( Private, Time --- substitute tokens )
pub fn ZZSUBT(STRING: &[u8], TRANSL: &[u8], L2R: bool, ctx: &mut Context) -> bool {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // So far we haven't combined anything.
    //
    save.DID = NO;
    save.K = intrinsics::LEN(STRING);

    //
    // We have two special cases to deal with.
    //
    if ((intrinsics::ICHAR(fstr::substr(STRING, 1..=1)) == intrinsics::ICHAR(b"<")) && (save.K > 1))
    {
        save.TO = intrinsics::MIN0(&[(save.K - 1), save.SIZE]);
        save.FROM = 1;

        if fstr::eq(
            fstr::substr(STRING, 2..=save.K),
            fstr::substr(&save.REP, save.FROM..=save.TO),
        ) {
            fstr::assign(fstr::substr_mut(&mut save.REP, save.FROM..=save.TO), TRANSL);
            save.ZZSUBT = YES;
        } else {
            save.ZZSUBT = NO;
        }

        return save.ZZSUBT;
    } else if ((intrinsics::ICHAR(fstr::substr(STRING, save.K..=save.K))
        == intrinsics::ICHAR(b">"))
        && (save.K > 1))
    {
        save.FROM = intrinsics::MAX0(&[1, ((save.SIZE - save.K) + 2)]);
        save.TO = save.SIZE;

        if fstr::eq(
            fstr::substr(STRING, 1..=(save.K - 1)),
            fstr::substr(&save.REP, save.FROM..=save.TO),
        ) {
            fstr::assign(fstr::substr_mut(&mut save.REP, save.FROM..=save.TO), TRANSL);
            save.ZZSUBT = YES;
        } else {
            save.ZZSUBT = NO;
        }

        return save.ZZSUBT;
    }

    //
    // Look for the substring either looking from the
    // left (L2R is YES) or from the right (L2R is NO).
    //
    if L2R {
        save.FROM = POS(&save.REP, STRING, 1);
    } else {
        save.FROM = POSR(&save.REP, STRING, save.SIZE);
    }

    save.TO = ((save.FROM + intrinsics::LEN(TRANSL)) - 1);

    if (save.FROM > 0) {
        save.DID = YES;
        fstr::assign(fstr::substr_mut(&mut save.REP, save.FROM..=save.TO), TRANSL);
    }

    save.ZZSUBT = save.DID;
    save.ZZSUBT
}

//$Procedure ZZTOKNS ( Private, Time --- Time Tokens )
pub fn ZZTOKNS(STRING: &[u8], ERROR: &mut [u8], ctx: &mut Context) -> bool {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // The first time in this routine we initialize our "tokenizing"
    // table.
    //
    save.ZZTOKNS = NO;

    if save.FIRST {
        save.FIRST = NO;
        save.BLANK = intrinsics::ICHAR(b" ");

        //
        // These are the error message templates for errors generated
        // for input time strings that have too many recognizable tokens
        // or are too long for their pictures to fit in the internal
        // picture buffer.
        //
        fstr::assign(&mut save.TKNERR, b"The input time string \'#\' cannot be processed because it contains more than @ recognizable tokens. The token that could not be processed was \'#\'.");
        REPMI(&save.TKNERR.to_vec(), b"@", MAXTKN, &mut save.TKNERR, ctx);

        fstr::assign(&mut save.PICERR, b"The input time string \'#\' cannot be processed because the internal picture describing it requires more than @ characters. The token that could not be processed was \'#\'.");
        REPMI(&save.PICERR.to_vec(), b"@", PICLEN, &mut save.PICERR, ctx);

        //
        // Below is the list of recognized substrings. The basic
        // pattern here is to find the block of special tokens
        // that begin with a particular character. Insert into
        // that block the lines of code below
        //
        // I              =  I + 1
        // F( ICHAR('letter')) =  I
        // RECOG(I)       = 'the full substring that's recognized '
        // WIDTH(I)       =  number of characters required to match
        // CLASS(I)       = 'the classification of this substring'
        // L( ICHAR('b')) =  I
        //
        // Note matching is performed from the first string in the
        // group to the last.
        //
        //

        {
            let m1__: i32 = 32;
            let m2__: i32 = 126;
            let m3__: i32 = 1;
            save.I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                save.F[save.I] = 0;
                save.L[save.I] = -1;
                fstr::assign(save.NAMES.get_mut(save.I), b"substring");
                save.I += m3__;
            }
        }

        fstr::assign(
            save.NAMES.get_mut(intrinsics::ICHAR(b"\'")),
            b"\"Year Abbreviation Mark\"",
        );
        fstr::assign(save.NAMES.get_mut(intrinsics::ICHAR(b",")), b"comma");
        fstr::assign(save.NAMES.get_mut(intrinsics::ICHAR(b"-")), b"dash");
        fstr::assign(save.NAMES.get_mut(intrinsics::ICHAR(b".")), b"period");
        fstr::assign(save.NAMES.get_mut(intrinsics::ICHAR(b"/")), b"slash");
        fstr::assign(save.NAMES.get_mut(intrinsics::ICHAR(b":")), b"colon");
        fstr::assign(save.NAMES.get_mut(intrinsics::ICHAR(b"D")), b"Day of Month");
        fstr::assign(save.NAMES.get_mut(intrinsics::ICHAR(b"H")), b"Hour");
        fstr::assign(save.NAMES.get_mut(intrinsics::ICHAR(b"M")), b"Minute");
        fstr::assign(
            save.NAMES.get_mut(intrinsics::ICHAR(b"N")),
            b"AM/PM indicator",
        );
        fstr::assign(
            save.NAMES.get_mut(intrinsics::ICHAR(b"O")),
            b"UTC-Offset indicator",
        );
        fstr::assign(save.NAMES.get_mut(intrinsics::ICHAR(b"S")), b"Second");
        fstr::assign(save.NAMES.get_mut(intrinsics::ICHAR(b"Y")), b"Year");
        fstr::assign(
            save.NAMES.get_mut(intrinsics::ICHAR(b"Z")),
            b"Time-Zone indicator",
        );
        fstr::assign(
            save.NAMES.get_mut(intrinsics::ICHAR(b"[")),
            b"Left Parenthesis",
        );
        fstr::assign(
            save.NAMES.get_mut(intrinsics::ICHAR(b"]")),
            b"Right Parenthesis",
        );
        fstr::assign(save.NAMES.get_mut(intrinsics::ICHAR(b"b")), b"White Space");
        fstr::assign(
            save.NAMES.get_mut(intrinsics::ICHAR(b"d")),
            b"Day-of-Year indicator",
        );
        fstr::assign(save.NAMES.get_mut(intrinsics::ICHAR(b"e")), b"Era");
        fstr::assign(save.NAMES.get_mut(intrinsics::ICHAR(b"i")), b"Integer");
        fstr::assign(
            save.NAMES.get_mut(intrinsics::ICHAR(b"j")),
            b"Julian Date indicator",
        );
        fstr::assign(save.NAMES.get_mut(intrinsics::ICHAR(b"m")), b"Month");
        fstr::assign(
            save.NAMES.get_mut(intrinsics::ICHAR(b"n")),
            b"Decimal Number",
        );
        fstr::assign(
            save.NAMES.get_mut(intrinsics::ICHAR(b"o")),
            b"UTC-Offset indicator",
        );
        fstr::assign(
            save.NAMES.get_mut(intrinsics::ICHAR(b"s")),
            b"Time System specification",
        );
        fstr::assign(
            save.NAMES.get_mut(intrinsics::ICHAR(b"t")),
            b"ISO Time Separator",
        );
        fstr::assign(save.NAMES.get_mut(intrinsics::ICHAR(b"w")), b"Weekday");
        fstr::assign(save.NAMES.get_mut(intrinsics::ICHAR(b"y")), b"Day of Year");

        fstr::assign(save.MNMRK.get_mut([LOWER, FULL]), b"month");
        fstr::assign(save.MNMRK.get_mut([UPPER, FULL]), b"MONTH");
        fstr::assign(save.MNMRK.get_mut([MIXED, FULL]), b"Month");
        fstr::assign(save.MNMRK.get_mut([LOWER, SHORT]), b"mon");
        fstr::assign(save.MNMRK.get_mut([UPPER, SHORT]), b"MON");
        fstr::assign(save.MNMRK.get_mut([MIXED, SHORT]), b"Mon");

        fstr::assign(save.WKDAY.get_mut([LOWER, FULL]), b"weekday");
        fstr::assign(save.WKDAY.get_mut([UPPER, FULL]), b"WEEKDAY");
        fstr::assign(save.WKDAY.get_mut([MIXED, FULL]), b"Weekday");
        fstr::assign(save.WKDAY.get_mut([LOWER, SHORT]), b"wkd");
        fstr::assign(save.WKDAY.get_mut([UPPER, SHORT]), b"WKD");
        fstr::assign(save.WKDAY.get_mut([MIXED, SHORT]), b"Wkd");
        //
        // Length of the items Month, Mon, weekday, wkd
        //
        save.WKSIZE[FULL] = 7;
        save.WKSIZE[SHORT] = 3;
        save.MNSIZE[FULL] = 5;
        save.MNSIZE[SHORT] = 3;

        save.I = 0;
        //
        // Tokens beginning with ' '
        //
        save.I = (save.I + 1);
        save.F[intrinsics::ICHAR(b" ")] = save.I;
        fstr::assign(save.RECOG.get_mut(save.I), b" ");
        save.WIDTH[save.I] = 1;
        fstr::assign(save.CLASS.get_mut(save.I), b"b");
        save.L[intrinsics::ICHAR(b" ")] = save.I;

        //
        // Tokens beginning with '('
        //
        save.I = (save.I + 1);
        save.F[intrinsics::ICHAR(b"(")] = save.I;
        fstr::assign(save.RECOG.get_mut(save.I), b"(");
        save.WIDTH[save.I] = 1;
        fstr::assign(save.CLASS.get_mut(save.I), b"[");
        save.L[intrinsics::ICHAR(b"(")] = save.I;
        //
        // Tokens beginning with ')'
        //
        save.I = (save.I + 1);
        save.F[intrinsics::ICHAR(b")")] = save.I;
        fstr::assign(save.RECOG.get_mut(save.I), b")");
        save.WIDTH[save.I] = 1;
        fstr::assign(save.CLASS.get_mut(save.I), b"]");
        save.L[intrinsics::ICHAR(b")")] = save.I;
        //
        // Tokens beginning with ','
        //
        save.I = (save.I + 1);
        save.F[intrinsics::ICHAR(b",")] = save.I;
        fstr::assign(save.RECOG.get_mut(save.I), b",");
        save.WIDTH[save.I] = 1;
        fstr::assign(save.CLASS.get_mut(save.I), b",");
        save.L[intrinsics::ICHAR(b",")] = save.I;

        //
        // Tokens beginning with '-'
        //
        save.I = (save.I + 1);
        save.F[intrinsics::ICHAR(b"-")] = save.I;
        fstr::assign(save.RECOG.get_mut(save.I), b"-");
        save.WIDTH[save.I] = 1;
        fstr::assign(save.CLASS.get_mut(save.I), b"-");
        save.L[intrinsics::ICHAR(b"-")] = save.I;

        //
        // Tokens beginning with '.'
        //
        save.I = (save.I + 1);
        save.F[intrinsics::ICHAR(b".")] = save.I;
        fstr::assign(save.RECOG.get_mut(save.I), b".");
        save.WIDTH[save.I] = 1;
        fstr::assign(save.CLASS.get_mut(save.I), b".");
        save.L[intrinsics::ICHAR(b".")] = save.I;

        //
        // Tokens beginning with '/'
        //
        save.I = (save.I + 1);
        save.F[intrinsics::ICHAR(b"/")] = save.I;
        fstr::assign(save.RECOG.get_mut(save.I), b"//");
        save.WIDTH[save.I] = 2;
        fstr::assign(save.CLASS.get_mut(save.I), b"d");
        save.L[intrinsics::ICHAR(b"/")] = save.I;

        save.I = (save.I + 1);
        fstr::assign(save.RECOG.get_mut(save.I), b"/");
        save.WIDTH[save.I] = 1;
        fstr::assign(save.CLASS.get_mut(save.I), b"/");
        save.L[intrinsics::ICHAR(b"/")] = save.I;
        //
        // Tokens beginning with ':'
        //
        save.I = (save.I + 1);
        save.F[intrinsics::ICHAR(b":")] = save.I;
        fstr::assign(save.RECOG.get_mut(save.I), b"::");
        save.WIDTH[save.I] = 2;
        fstr::assign(save.CLASS.get_mut(save.I), b"d");
        save.L[intrinsics::ICHAR(b":")] = save.I;

        save.I = (save.I + 1);
        fstr::assign(save.RECOG.get_mut(save.I), b":");
        save.WIDTH[save.I] = 1;
        fstr::assign(save.CLASS.get_mut(save.I), b":");
        save.L[intrinsics::ICHAR(b":")] = save.I;

        //
        // Tokens beginning with 'A'
        //
        save.I = (save.I + 1);
        save.F[intrinsics::ICHAR(b"A")] = save.I;
        fstr::assign(save.RECOG.get_mut(save.I), b"A.D.");
        save.WIDTH[save.I] = 4;
        fstr::assign(save.CLASS.get_mut(save.I), b"e");
        save.L[intrinsics::ICHAR(b"A")] = save.I;

        save.I = (save.I + 1);
        fstr::assign(save.RECOG.get_mut(save.I), b"AD");
        save.WIDTH[save.I] = 2;
        fstr::assign(save.CLASS.get_mut(save.I), b"e");
        save.L[intrinsics::ICHAR(b"A")] = save.I;

        save.I = (save.I + 1);
        fstr::assign(save.RECOG.get_mut(save.I), b"A.M.");
        save.WIDTH[save.I] = 4;
        fstr::assign(save.CLASS.get_mut(save.I), b"N");
        save.L[intrinsics::ICHAR(b"A")] = save.I;

        save.I = (save.I + 1);
        fstr::assign(save.RECOG.get_mut(save.I), b"AM");
        save.WIDTH[save.I] = 2;
        fstr::assign(save.CLASS.get_mut(save.I), b"N");
        save.L[intrinsics::ICHAR(b"A")] = save.I;

        save.I = (save.I + 1);
        fstr::assign(save.RECOG.get_mut(save.I), b"APRIL");
        save.WIDTH[save.I] = 3;
        fstr::assign(save.CLASS.get_mut(save.I), b"m");
        save.L[intrinsics::ICHAR(b"A")] = save.I;

        save.I = (save.I + 1);
        fstr::assign(save.RECOG.get_mut(save.I), b"AUGUST");
        save.WIDTH[save.I] = 3;
        fstr::assign(save.CLASS.get_mut(save.I), b"m");
        save.L[intrinsics::ICHAR(b"A")] = save.I;

        //
        // Tokens beginning with 'B'
        //
        save.I = (save.I + 1);
        save.F[intrinsics::ICHAR(b"B")] = save.I;
        fstr::assign(save.RECOG.get_mut(save.I), b"B.C.");
        save.WIDTH[save.I] = 4;
        fstr::assign(save.CLASS.get_mut(save.I), b"e");
        save.L[intrinsics::ICHAR(b"B")] = save.I;

        save.I = (save.I + 1);
        fstr::assign(save.RECOG.get_mut(save.I), b"BC");
        save.WIDTH[save.I] = 2;
        fstr::assign(save.CLASS.get_mut(save.I), b"e");
        save.L[intrinsics::ICHAR(b"B")] = save.I;

        //
        // Tokens beginning with 'C'
        //
        save.I = (save.I + 1);
        save.F[intrinsics::ICHAR(b"C")] = save.I;
        fstr::assign(save.RECOG.get_mut(save.I), b"CDT");
        save.WIDTH[save.I] = 3;
        fstr::assign(save.CLASS.get_mut(save.I), b"Z");
        save.L[intrinsics::ICHAR(b"C")] = save.I;

        save.I = (save.I + 1);
        fstr::assign(save.RECOG.get_mut(save.I), b"CST");
        save.WIDTH[save.I] = 3;
        fstr::assign(save.CLASS.get_mut(save.I), b"Z");
        save.L[intrinsics::ICHAR(b"C")] = save.I;

        //
        // Tokens beginning with 'D'
        //
        save.I = (save.I + 1);
        save.F[intrinsics::ICHAR(b"D")] = save.I;
        fstr::assign(save.RECOG.get_mut(save.I), b"DECEMBER");
        save.WIDTH[save.I] = 3;
        fstr::assign(save.CLASS.get_mut(save.I), b"m");
        save.L[intrinsics::ICHAR(b"D")] = save.I;

        save.I = (save.I + 1);
        fstr::assign(save.RECOG.get_mut(save.I), b"D+");
        save.WIDTH[save.I] = 2;
        fstr::assign(save.CLASS.get_mut(save.I), b"E");
        save.L[intrinsics::ICHAR(b"D")] = save.I;

        save.I = (save.I + 1);
        fstr::assign(save.RECOG.get_mut(save.I), b"D-");
        save.WIDTH[save.I] = 2;
        fstr::assign(save.CLASS.get_mut(save.I), b"E");
        save.L[intrinsics::ICHAR(b"D")] = save.I;

        save.I = (save.I + 1);
        fstr::assign(save.RECOG.get_mut(save.I), b"D");
        save.WIDTH[save.I] = 1;
        fstr::assign(save.CLASS.get_mut(save.I), b"E");
        save.L[intrinsics::ICHAR(b"D")] = save.I;

        //
        // Tokens beginning with 'E'
        //
        save.I = (save.I + 1);
        save.F[intrinsics::ICHAR(b"E")] = save.I;
        fstr::assign(save.RECOG.get_mut(save.I), b"EDT");
        save.WIDTH[save.I] = 3;
        fstr::assign(save.CLASS.get_mut(save.I), b"Z");
        save.L[intrinsics::ICHAR(b"E")] = save.I;

        save.I = (save.I + 1);
        fstr::assign(save.RECOG.get_mut(save.I), b"EST");
        save.WIDTH[save.I] = 3;
        fstr::assign(save.CLASS.get_mut(save.I), b"Z");
        save.L[intrinsics::ICHAR(b"E")] = save.I;

        save.I = (save.I + 1);
        fstr::assign(save.RECOG.get_mut(save.I), b"E+");
        save.WIDTH[save.I] = 2;
        fstr::assign(save.CLASS.get_mut(save.I), b"E");
        save.L[intrinsics::ICHAR(b"E")] = save.I;

        save.I = (save.I + 1);
        fstr::assign(save.RECOG.get_mut(save.I), b"E-");
        save.WIDTH[save.I] = 2;
        fstr::assign(save.CLASS.get_mut(save.I), b"E");
        save.L[intrinsics::ICHAR(b"E")] = save.I;

        save.I = (save.I + 1);
        fstr::assign(save.RECOG.get_mut(save.I), b"E");
        save.WIDTH[save.I] = 1;
        fstr::assign(save.CLASS.get_mut(save.I), b"E");
        save.L[intrinsics::ICHAR(b"E")] = save.I;

        //
        // Tokens beginning with 'F'
        //
        save.I = (save.I + 1);
        save.F[intrinsics::ICHAR(b"F")] = save.I;
        fstr::assign(save.RECOG.get_mut(save.I), b"FEBRUARY");
        save.WIDTH[save.I] = 3;
        fstr::assign(save.CLASS.get_mut(save.I), b"m");
        save.L[intrinsics::ICHAR(b"F")] = save.I;

        save.I = (save.I + 1);
        fstr::assign(save.RECOG.get_mut(save.I), b"FRIDAY");
        save.WIDTH[save.I] = 3;
        fstr::assign(save.CLASS.get_mut(save.I), b"w");
        save.L[intrinsics::ICHAR(b"F")] = save.I;

        //
        // Tokens beginning with 'J'
        //
        save.I = (save.I + 1);
        save.F[intrinsics::ICHAR(b"J")] = save.I;
        fstr::assign(save.RECOG.get_mut(save.I), b"JANUARY");
        save.WIDTH[save.I] = 3;
        fstr::assign(save.CLASS.get_mut(save.I), b"m");
        save.L[intrinsics::ICHAR(b"J")] = save.I;

        save.I = (save.I + 1);
        fstr::assign(save.RECOG.get_mut(save.I), b"JD");
        save.WIDTH[save.I] = 2;
        fstr::assign(save.CLASS.get_mut(save.I), b"j");
        save.L[intrinsics::ICHAR(b"J")] = save.I;

        save.I = (save.I + 1);
        fstr::assign(save.RECOG.get_mut(save.I), b"JULY");
        save.WIDTH[save.I] = 3;
        fstr::assign(save.CLASS.get_mut(save.I), b"m");
        save.L[intrinsics::ICHAR(b"J")] = save.I;

        save.I = (save.I + 1);
        fstr::assign(save.RECOG.get_mut(save.I), b"JUNE");
        save.WIDTH[save.I] = 3;
        fstr::assign(save.CLASS.get_mut(save.I), b"m");
        save.L[intrinsics::ICHAR(b"J")] = save.I;

        //
        // Tokens beginning with 'M'
        //
        save.I = (save.I + 1);
        save.F[intrinsics::ICHAR(b"M")] = save.I;
        fstr::assign(save.RECOG.get_mut(save.I), b"MARCH");
        save.WIDTH[save.I] = 3;
        fstr::assign(save.CLASS.get_mut(save.I), b"m");
        save.L[intrinsics::ICHAR(b"M")] = save.I;

        save.I = (save.I + 1);
        fstr::assign(save.RECOG.get_mut(save.I), b"MAY");
        save.WIDTH[save.I] = 3;
        fstr::assign(save.CLASS.get_mut(save.I), b"m");
        save.L[intrinsics::ICHAR(b"M")] = save.I;

        save.I = (save.I + 1);
        fstr::assign(save.RECOG.get_mut(save.I), b"MDT");
        save.WIDTH[save.I] = 3;
        fstr::assign(save.CLASS.get_mut(save.I), b"Z");
        save.L[intrinsics::ICHAR(b"M")] = save.I;

        save.I = (save.I + 1);
        fstr::assign(save.RECOG.get_mut(save.I), b"MONDAY");
        save.WIDTH[save.I] = 3;
        fstr::assign(save.CLASS.get_mut(save.I), b"w");
        save.L[intrinsics::ICHAR(b"M")] = save.I;

        save.I = (save.I + 1);
        fstr::assign(save.RECOG.get_mut(save.I), b"MST");
        save.WIDTH[save.I] = 3;
        fstr::assign(save.CLASS.get_mut(save.I), b"Z");
        save.L[intrinsics::ICHAR(b"M")] = save.I;

        //
        // Tokens beginning with 'N'
        //
        save.I = (save.I + 1);
        save.F[intrinsics::ICHAR(b"N")] = save.I;
        fstr::assign(save.RECOG.get_mut(save.I), b"NOVEMBER");
        save.WIDTH[save.I] = 3;
        fstr::assign(save.CLASS.get_mut(save.I), b"m");
        save.L[intrinsics::ICHAR(b"N")] = save.I;

        //
        // Tokens beginning with 'O'
        //
        save.I = (save.I + 1);
        save.F[intrinsics::ICHAR(b"O")] = save.I;
        fstr::assign(save.RECOG.get_mut(save.I), b"OCTOBER");
        save.WIDTH[save.I] = 3;
        fstr::assign(save.CLASS.get_mut(save.I), b"m");
        save.L[intrinsics::ICHAR(b"O")] = save.I;

        //
        // Tokens beginning with 'P'
        //
        save.I = (save.I + 1);
        save.F[intrinsics::ICHAR(b"P")] = save.I;
        fstr::assign(save.RECOG.get_mut(save.I), b"P.M.");
        save.WIDTH[save.I] = 4;
        fstr::assign(save.CLASS.get_mut(save.I), b"N");
        save.L[intrinsics::ICHAR(b"P")] = save.I;

        save.I = (save.I + 1);
        fstr::assign(save.RECOG.get_mut(save.I), b"PDT");
        save.WIDTH[save.I] = 3;
        fstr::assign(save.CLASS.get_mut(save.I), b"Z");
        save.L[intrinsics::ICHAR(b"P")] = save.I;

        save.I = (save.I + 1);
        fstr::assign(save.RECOG.get_mut(save.I), b"PM");
        save.WIDTH[save.I] = 2;
        fstr::assign(save.CLASS.get_mut(save.I), b"N");
        save.L[intrinsics::ICHAR(b"P")] = save.I;

        save.I = (save.I + 1);
        fstr::assign(save.RECOG.get_mut(save.I), b"PST");
        save.WIDTH[save.I] = 3;
        fstr::assign(save.CLASS.get_mut(save.I), b"Z");
        save.L[intrinsics::ICHAR(b"P")] = save.I;

        //
        // Tokens beginning with 'S'
        //
        save.I = (save.I + 1);
        save.F[intrinsics::ICHAR(b"S")] = save.I;
        fstr::assign(save.RECOG.get_mut(save.I), b"SATURDAY");
        save.WIDTH[save.I] = 3;
        fstr::assign(save.CLASS.get_mut(save.I), b"w");
        save.L[intrinsics::ICHAR(b"S")] = save.I;

        save.I = (save.I + 1);
        fstr::assign(save.RECOG.get_mut(save.I), b"SEPTEMBER");
        save.WIDTH[save.I] = 3;
        fstr::assign(save.CLASS.get_mut(save.I), b"m");
        save.L[intrinsics::ICHAR(b"S")] = save.I;

        save.I = (save.I + 1);
        fstr::assign(save.RECOG.get_mut(save.I), b"SUNDAY");
        save.WIDTH[save.I] = 3;
        fstr::assign(save.CLASS.get_mut(save.I), b"w");
        save.L[intrinsics::ICHAR(b"S")] = save.I;

        //
        // Tokens beginning with 'T'
        //
        save.I = (save.I + 1);
        save.F[intrinsics::ICHAR(b"T")] = save.I;
        fstr::assign(save.RECOG.get_mut(save.I), b"TDB");
        save.WIDTH[save.I] = 3;
        fstr::assign(save.CLASS.get_mut(save.I), b"s");
        save.L[intrinsics::ICHAR(b"T")] = save.I;

        save.I = (save.I + 1);
        fstr::assign(save.RECOG.get_mut(save.I), b"TDT");
        save.WIDTH[save.I] = 3;
        fstr::assign(save.CLASS.get_mut(save.I), b"s");
        save.L[intrinsics::ICHAR(b"T")] = save.I;

        save.I = (save.I + 1);
        fstr::assign(save.RECOG.get_mut(save.I), b"THURSDAY");
        save.WIDTH[save.I] = 3;
        fstr::assign(save.CLASS.get_mut(save.I), b"w");
        save.L[intrinsics::ICHAR(b"T")] = save.I;

        save.I = (save.I + 1);
        fstr::assign(save.RECOG.get_mut(save.I), b"TUESDAY");
        save.WIDTH[save.I] = 3;
        fstr::assign(save.CLASS.get_mut(save.I), b"w");
        save.L[intrinsics::ICHAR(b"T")] = save.I;

        save.I = (save.I + 1);
        fstr::assign(save.RECOG.get_mut(save.I), b"T");
        save.WIDTH[save.I] = 1;
        fstr::assign(save.CLASS.get_mut(save.I), b"t");
        save.L[intrinsics::ICHAR(b"T")] = save.I;

        //
        // Tokens beginning with 'U'
        //
        save.I = (save.I + 1);
        save.F[intrinsics::ICHAR(b"U")] = save.I;
        fstr::assign(save.RECOG.get_mut(save.I), b"UTC+");
        save.WIDTH[save.I] = 4;
        fstr::assign(save.CLASS.get_mut(save.I), b"O");
        save.L[intrinsics::ICHAR(b"U")] = save.I;

        save.I = (save.I + 1);
        fstr::assign(save.RECOG.get_mut(save.I), b"UTC-");
        save.WIDTH[save.I] = 4;
        fstr::assign(save.CLASS.get_mut(save.I), b"o");
        save.L[intrinsics::ICHAR(b"U")] = save.I;

        save.I = (save.I + 1);
        fstr::assign(save.RECOG.get_mut(save.I), b"UTC");
        save.WIDTH[save.I] = 3;
        fstr::assign(save.CLASS.get_mut(save.I), b"s");
        save.L[intrinsics::ICHAR(b"U")] = save.I;
        //
        // Tokens beginning with ''''
        //
        save.I = (save.I + 1);
        save.F[intrinsics::ICHAR(b"\'")] = save.I;
        fstr::assign(save.RECOG.get_mut(save.I), b"\'");
        save.WIDTH[save.I] = 1;
        fstr::assign(save.CLASS.get_mut(save.I), b"\'");
        save.L[intrinsics::ICHAR(b"\'")] = save.I;
        //
        // Tokens beginning with 'W'
        //
        save.I = (save.I + 1);
        save.F[intrinsics::ICHAR(b"W")] = save.I;
        fstr::assign(save.RECOG.get_mut(save.I), b"WEDNESDAY");
        save.WIDTH[save.I] = 3;
        fstr::assign(save.CLASS.get_mut(save.I), b"w");
        save.L[intrinsics::ICHAR(b"W")] = save.I;
        //
        // Tokens beginning with 'Z'
        //
        save.I = (save.I + 1);
        save.F[intrinsics::ICHAR(b"Z")] = save.I;
        fstr::assign(save.RECOG.get_mut(save.I), b"Z");
        save.WIDTH[save.I] = 1;
        fstr::assign(save.CLASS.get_mut(save.I), b"x");
        save.L[intrinsics::ICHAR(b"Z")] = save.I;
    }

    //
    // If the input string is blank, return with an error message.
    //
    if fstr::eq(STRING, b" ") {
        fstr::assign(ERROR, b"The input time string is blank.");
        save.ZZTOKNS = NO;
        return save.ZZTOKNS;
    }

    //
    // OK. Initializations are out of the way. We now take
    // apart the string.
    //
    save.DID = NO;
    fstr::assign(ERROR, b" ");
    fstr::assign(&mut save.REP, b" ");
    fstr::assign(&mut save.PICTUR, b" ");
    save.SIZE = 0;
    save.NEXT = 1;
    save.PNEXT = 1;
    save.PUT = 0;
    save.AMPM = false;
    save.LAST = RTRIM(STRING);

    while (save.NEXT <= save.LAST) {
        //
        // FROM and NEXT point to parts of the string, PFROM and PNEXT
        // point to parts of the picture we will construct.
        //
        save.FROM = save.NEXT;
        save.PFROM = save.PNEXT;
        save.ITEM = intrinsics::ICHAR(fstr::substr(STRING, save.NEXT..=save.NEXT));
        //
        // First we try to find an unsigned integer in the string.
        //
        LX4UNS(
            fstr::substr(STRING, 1..=save.LAST),
            save.FROM,
            &mut save.TO,
            &mut save.NCHAR,
            ctx,
        );

        if (save.NCHAR > 0) {
            //
            // We found an unsigned integer, add a letter to the
            // internal representation, note the begin and end
            // of the token and set NEXT to the first character
            // beyond this token.
            //
            save.PUT = (save.PUT + 1);

            if (save.PUT > MAXTKN) {
                ZZTKNERR(
                    &save.TKNERR,
                    STRING,
                    fstr::substr(STRING, save.FROM..=save.TO),
                    ERROR,
                    &mut save.ZZTOKNS,
                );
                return save.ZZTOKNS;
            }

            fstr::assign(fstr::substr_mut(&mut save.REP, save.PUT..=save.PUT), b"i");
            save.BEGS[save.PUT] = save.FROM;
            save.ENDS[save.PUT] = save.TO;
            save.NEXT = (save.TO + 1);
            save.PTO = ((save.PFROM + save.NCHAR) - 1);

            if (save.PTO > PICLEN) {
                ZZTKNERR(
                    &save.PICERR,
                    STRING,
                    fstr::substr(STRING, save.FROM..=save.TO),
                    ERROR,
                    &mut save.ZZTOKNS,
                );
                return save.ZZTOKNS;
            }

            save.PNEXT = (save.PTO + 1);
            fstr::assign(
                fstr::substr_mut(&mut save.PICTUR, save.PFROM..=save.PTO),
                fstr::substr(STRING, save.FROM..=save.TO),
            );
            save.PBEGS[save.PUT] = save.PFROM;
            save.PENDS[save.PUT] = save.PTO;
        } else if (save.ITEM == save.BLANK) {
            //
            // We have a blank. We lump all consecutive
            // blanks together as one big fat blank.
            //
            save.PUT = (save.PUT + 1);

            if (save.PUT > MAXTKN) {
                ZZTKNERR(&save.TKNERR, STRING, b" ", ERROR, &mut save.ZZTOKNS);
                return save.ZZTOKNS;
            }

            save.TO = save.FROM;
            save.BEGS[save.PUT] = save.FROM;
            fstr::assign(fstr::substr_mut(&mut save.REP, save.PUT..=save.PUT), b"b");

            while ((save.ITEM == save.BLANK) && (save.TO <= save.LAST)) {
                save.TO = (save.TO + 1);
                if (save.TO <= save.LAST) {
                    save.ITEM = intrinsics::ICHAR(fstr::substr(STRING, save.TO..=save.TO));
                }
            }

            save.NEXT = save.TO;
            save.TO = (save.TO - 1);
            save.ENDS[save.PUT] = save.TO;

            save.PTO = ((save.PFROM + save.TO) - save.FROM);

            if (save.PTO > PICLEN) {
                ZZTKNERR(&save.PICERR, STRING, b" ", ERROR, &mut save.ZZTOKNS);
                return save.ZZTOKNS;
            }

            save.PNEXT = (save.PTO + 1);
            fstr::assign(
                fstr::substr_mut(&mut save.PICTUR, save.PFROM..=save.PTO),
                fstr::substr(STRING, save.FROM..=save.TO),
            );
            save.PBEGS[save.PUT] = save.PFROM;
            save.PENDS[save.PUT] = save.PTO;
        } else if (save.ITEM == 9) {
            //
            // We've got a tab character, we treat tabs as
            // blanks.
            //
            save.PUT = (save.PUT + 1);

            if (save.PUT > MAXTKN) {
                ZZTKNERR(&save.TKNERR, STRING, b"<TAB>", ERROR, &mut save.ZZTOKNS);
                return save.ZZTOKNS;
            }

            fstr::assign(fstr::substr_mut(&mut save.REP, save.PUT..=save.PUT), b"b");
            save.BEGS[save.PUT] = save.FROM;
            save.ENDS[save.PUT] = save.FROM;
            save.NEXT = (save.NEXT + 1);

            save.PTO = save.PFROM;

            if (save.PTO > PICLEN) {
                ZZTKNERR(&save.PICERR, STRING, b"<TAB>", ERROR, &mut save.ZZTOKNS);
                return save.ZZTOKNS;
            }

            save.PNEXT = (save.PTO + 1);
            fstr::assign(
                fstr::substr_mut(&mut save.PICTUR, save.PFROM..=save.PTO),
                b" ",
            );
            save.PBEGS[save.PUT] = save.PFROM;
            save.PENDS[save.PUT] = save.PFROM;
        } else if ((save.ITEM < 32) || (save.ITEM > 126)) {
            //
            // This is a non-printing character. This is
            // regarded as an error.
            //
            fstr::assign(ERROR, STRING);

            ZZINSSUB(&ERROR.to_vec(), b"<", save.NEXT, ERROR);

            //
            // Overwrite the non-printing character with a
            // closing angle bracket.
            //
            if (save.NEXT < intrinsics::LEN(ERROR)) {
                fstr::assign(
                    fstr::substr_mut(ERROR, (save.NEXT + 1)..=(save.NEXT + 1)),
                    b">",
                );
            }

            PREFIX(b"There is a non-printing, non-tab character (ASCII #) at position # of the time string: ", 1, ERROR);

            REPMI(&ERROR.to_vec(), b"#", save.ITEM, ERROR, ctx);
            REPMI(&ERROR.to_vec(), b"#", save.NEXT, ERROR, ctx);

            save.ZZTOKNS = NO;
            return save.ZZTOKNS;
        } else {
            //
            // This has to be one of the known types or we
            // have an unknown component in the string. We've constructed
            // a "parsing" table for handling these special cases.
            // This table uses the first letter of the string
            // to begin a search. We get that code and force it
            // into a suitable range.
            //
            UCASE(
                fstr::substr(STRING, save.NEXT..=save.NEXT),
                &mut save.THIS,
                ctx,
            );
            save.ITEM = intrinsics::ICHAR(&save.THIS);
            save.FROM = save.NEXT;
            save.CHECK = YES;
            save.I = save.F[save.ITEM];

            while (save.CHECK && (save.I <= save.L[save.ITEM])) {
                save.W = save.WIDTH[save.I];
                save.TO = ((save.FROM + save.W) - 1);

                save.GOT = SAMSBI(
                    STRING,
                    save.FROM,
                    save.TO,
                    &save.RECOG[save.I],
                    1,
                    save.W,
                    ctx,
                );

                if save.GOT {
                    //
                    // We have a match. If it is the match of a month
                    // or day of the week, we keep looking for the
                    // end of the match.
                    //
                    if (fstr::eq(save.CLASS.get(save.I), b"m")
                        || fstr::eq(save.CLASS.get(save.I), b"w"))
                    {
                        fstr::assign(&mut save.SPCIAL, save.RECOG.get(save.I));
                        save.R = RTRIM(&save.SPCIAL);
                        save.W = (save.W + 1);
                        save.TO = (save.TO + 1);

                        while SAMCHI(
                            STRING,
                            save.TO,
                            fstr::substr(&save.SPCIAL, 1..=save.R),
                            save.W,
                            ctx,
                        ) {
                            save.W = (save.W + 1);
                            save.TO = (save.TO + 1);
                        }

                        save.TO = (save.TO - 1);

                        if (save.W > save.R) {
                            save.KIND = FULL;
                        } else {
                            save.KIND = SHORT;
                        }

                        if fstr::ne(&save.THIS, fstr::substr(STRING, save.NEXT..=save.NEXT)) {
                            save.CASE = LOWER;
                        } else if fstr::eq(
                            fstr::substr(STRING, save.NEXT..=(save.NEXT + 2)),
                            fstr::substr(&save.SPCIAL, 1..=3),
                        ) {
                            save.CASE = UPPER;
                        } else {
                            save.CASE = MIXED;
                        }

                        if fstr::eq(save.CLASS.get(save.I), b"m") {
                            save.PTO = ((save.PFROM + save.MNSIZE[save.KIND]) - 1);

                            if (save.PTO > PICLEN) {
                                ZZTKNERR(
                                    &save.PICERR,
                                    STRING,
                                    fstr::substr(STRING, save.FROM..=save.TO),
                                    ERROR,
                                    &mut save.ZZTOKNS,
                                );
                                return save.ZZTOKNS;
                            }

                            save.PNEXT = (save.PTO + 1);
                            fstr::assign(
                                fstr::substr_mut(&mut save.PICTUR, save.PFROM..=save.PTO),
                                save.MNMRK.get([save.CASE, save.KIND]),
                            );
                        } else {
                            save.PTO = ((save.PFROM + save.WKSIZE[save.KIND]) - 1);

                            if (save.PTO > PICLEN) {
                                ZZTKNERR(
                                    &save.PICERR,
                                    STRING,
                                    fstr::substr(STRING, save.FROM..=save.TO),
                                    ERROR,
                                    &mut save.ZZTOKNS,
                                );
                                return save.ZZTOKNS;
                            }

                            save.PNEXT = (save.PTO + 1);
                            fstr::assign(
                                fstr::substr_mut(&mut save.PICTUR, save.PFROM..=save.PTO),
                                save.WKDAY.get([save.CASE, save.KIND]),
                            );
                        }
                    } else if fstr::eq(save.CLASS.get(save.I), b"e") {
                        save.PTO = (save.PFROM + 2);

                        if (save.PTO > PICLEN) {
                            ZZTKNERR(
                                &save.PICERR,
                                STRING,
                                fstr::substr(STRING, save.FROM..=save.TO),
                                ERROR,
                                &mut save.ZZTOKNS,
                            );
                            return save.ZZTOKNS;
                        }

                        save.PNEXT = (save.PTO + 1);

                        if fstr::eq(fstr::substr(STRING, save.FROM..=save.FROM), &save.THIS) {
                            fstr::assign(
                                fstr::substr_mut(&mut save.PICTUR, save.PFROM..=save.PTO),
                                b"ERA",
                            );
                        } else {
                            fstr::assign(
                                fstr::substr_mut(&mut save.PICTUR, save.PFROM..=save.PTO),
                                b"era",
                            );
                        }
                    } else if fstr::eq(save.CLASS.get(save.I), b"N") {
                        save.PTO = (save.PFROM + 3);

                        if (save.PTO > PICLEN) {
                            ZZTKNERR(
                                &save.PICERR,
                                STRING,
                                fstr::substr(STRING, save.FROM..=save.TO),
                                ERROR,
                                &mut save.ZZTOKNS,
                            );
                            return save.ZZTOKNS;
                        }

                        save.PNEXT = (save.PTO + 1);
                        if fstr::eq(fstr::substr(STRING, save.FROM..=save.FROM), &save.THIS) {
                            fstr::assign(
                                fstr::substr_mut(&mut save.PICTUR, save.PFROM..=save.PTO),
                                b"AMPM",
                            );
                        } else {
                            fstr::assign(
                                fstr::substr_mut(&mut save.PICTUR, save.PFROM..=save.PTO),
                                b"ampm",
                            );
                        }

                        save.AMPM = true;
                    } else if fstr::eq(save.CLASS.get(save.I), b"x") {

                        //
                        // Recognized token, but ignored.
                        //
                    } else {
                        save.PTO = ((save.PFROM + save.TO) - save.FROM);

                        if (save.PTO > PICLEN) {
                            ZZTKNERR(
                                &save.PICERR,
                                STRING,
                                fstr::substr(STRING, save.FROM..=save.TO),
                                ERROR,
                                &mut save.ZZTOKNS,
                            );
                            return save.ZZTOKNS;
                        }

                        save.PNEXT = (save.PTO + 1);
                        fstr::assign(
                            fstr::substr_mut(&mut save.PICTUR, save.PFROM..=save.PTO),
                            fstr::substr(STRING, save.FROM..=save.TO),
                        );
                    }

                    save.PUT = (save.PUT + 1);

                    if (save.PUT > MAXTKN) {
                        ZZTKNERR(
                            &save.TKNERR,
                            STRING,
                            fstr::substr(STRING, save.FROM..=save.TO),
                            ERROR,
                            &mut save.ZZTOKNS,
                        );
                        return save.ZZTOKNS;
                    }

                    fstr::assign(
                        fstr::substr_mut(&mut save.REP, save.PUT..=save.PUT),
                        fstr::substr(save.CLASS.get(save.I), 1..=1),
                    );
                    save.BEGS[save.PUT] = save.FROM;
                    save.ENDS[save.PUT] = save.TO;
                    save.PBEGS[save.PUT] = save.PFROM;
                    save.PENDS[save.PUT] = save.PTO;
                    save.CHECK = NO;
                    save.NEXT = (save.TO + 1);
                }

                save.I = (save.I + 1);
            }
            //
            // If we reach the end of the loop and CHECK is still
            // set to TRUE, we have a bit of unrecognizable string.
            //
            if save.CHECK {
                fstr::assign(ERROR, STRING);

                ZZINSSUB(&ERROR.to_vec(), b">", (save.FROM + 1), ERROR);
                ZZINSSUB(&ERROR.to_vec(), b"<", save.FROM, ERROR);

                PREFIX(b"The input string contains an unrecognizable substring beginning at the character marked by <#>: \"", 0, ERROR);
                SUFFIX(b"\"", 0, ERROR);

                REPMC(
                    &ERROR.to_vec(),
                    b"#",
                    fstr::substr(STRING, save.FROM..=save.FROM),
                    ERROR,
                );
                save.ZZTOKNS = NO;
                return save.ZZTOKNS;
            }
        }
    }

    save.SIZE = save.PUT;
    save.ZZTOKNS = YES;
    save.ZZTOKNS
}

//$Procedure ZZUNPCK ( Private, Time --- Unpack a time string )
pub fn ZZUNPCK(
    STRING: &[u8],
    YABBRV: bool,
    TVEC: &mut [f64],
    E: &mut i32,
    TRANSL: &mut [u8],
    PIC: &mut [u8],
    ERROR: &mut [u8],
    ctx: &mut Context,
) -> bool {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut TVEC = DummyArrayMut::new(TVEC, 1..);

    save.NYEAR = 0;
    save.NMON = 0;
    save.NDAY = 0;
    save.NHOUR = 0;
    save.NMIN = 0;
    save.NSEC = 0;
    save.NDOY = 0;
    save.NJD = 0;
    *E = 0;
    fstr::assign(TRANSL, b" ");

    save.HMS[1] = 0.0;
    save.HMS[2] = 0.0;
    save.HMS[3] = 0.0;

    {
        let m1__: i32 = save.SIZE;
        let m2__: i32 = 1;
        let m3__: i32 = -1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.ITEM = intrinsics::ICHAR(fstr::substr(&save.REP, save.I..=save.I));

            //
            // Confirm ITEM range [32,126].
            //
            if ((save.ITEM < 32) || (save.ITEM > 126)) {
                //
                // A non-printing character found in REP. This is
                // an error.
                //
                fstr::assign(ERROR, b"A character at location #1 does not have ASCII value [32,126] for REP string.");

                REPMI(&ERROR.to_vec(), b"#1", save.I, ERROR, ctx);

                //
                // Error condition, return.
                //
                save.ZZUNPCK = NO;
                return save.ZZUNPCK;
            }

            save.J = save.BEGS[save.I];
            save.K = save.ENDS[save.I];

            if (save.ITEM == intrinsics::ICHAR(b"Y")) {
                save.NYEAR = (save.NYEAR + 1);
                *E = (*E + 1);
                NPARSD(
                    fstr::substr(STRING, save.J..=save.K),
                    &mut TVEC[1],
                    ERROR,
                    &mut save.PTR,
                    ctx,
                );

                if YABBRV {
                    ZZREPSUB(
                        &save.PICTUR.to_vec(),
                        save.PBEGS[save.I],
                        save.PENDS[save.I],
                        b"YR",
                        &mut save.PICTUR,
                    );
                } else {
                    ZZREPSUB(
                        &save.PICTUR.to_vec(),
                        save.PBEGS[save.I],
                        save.PENDS[save.I],
                        b"YYYY",
                        &mut save.PICTUR,
                    );
                }
            } else if (save.ITEM == intrinsics::ICHAR(b"m")) {
                save.NMON = (save.NMON + 1);
                *E = (*E + 1);

                UCASE(fstr::substr(STRING, save.J..=save.K), &mut save.MONTH, ctx);
                save.VALUE = ISRCHC(&save.MONTH, 12, save.MONTHS.as_arg());

                if (save.VALUE == 0) {
                    NPARSD(
                        fstr::substr(STRING, save.J..=save.K),
                        &mut TVEC[2],
                        ERROR,
                        &mut save.PTR,
                        ctx,
                    );
                    ZZREPSUB(
                        &save.PICTUR.to_vec(),
                        save.PBEGS[save.I],
                        save.PENDS[save.I],
                        b"MM",
                        &mut save.PICTUR,
                    );
                } else {
                    TVEC[2] = (save.VALUE as f64);
                }
            } else if (save.ITEM == intrinsics::ICHAR(b"D")) {
                save.NDAY = (save.NDAY + 1);
                *E = (*E + 1);

                NPARSD(
                    fstr::substr(STRING, save.J..=save.K),
                    &mut TVEC[3],
                    ERROR,
                    &mut save.PTR,
                    ctx,
                );

                ZZMKPC(
                    &mut save.PICTUR,
                    save.PBEGS[save.I],
                    save.PENDS[save.I],
                    b"DD",
                    fstr::substr(STRING, save.J..=save.K),
                );
            } else if (save.ITEM == intrinsics::ICHAR(b"y")) {
                save.NDOY = (save.NDOY + 1);
                *E = (*E + 1);

                NPARSD(
                    fstr::substr(STRING, save.J..=save.K),
                    &mut TVEC[2],
                    ERROR,
                    &mut save.PTR,
                    ctx,
                );

                ZZMKPC(
                    &mut save.PICTUR,
                    save.PBEGS[save.I],
                    save.PENDS[save.I],
                    b"DOY",
                    fstr::substr(STRING, save.J..=save.K),
                );
            } else if (save.ITEM == intrinsics::ICHAR(b"H")) {
                save.NHOUR = (save.NHOUR + 1);
                *E = (*E + 1);

                NPARSD(
                    fstr::substr(STRING, save.J..=save.K),
                    &mut save.HMS[1],
                    ERROR,
                    &mut save.PTR,
                    ctx,
                );
                //
                // We have to handle the hour component based on the
                // presence of the AM/PM mark in the picture. We earlier
                // set up the logical AMPM to indicate its presence.
                //
                if save.AMPM {
                    ZZMKPC(
                        &mut save.PICTUR,
                        save.PBEGS[save.I],
                        save.PENDS[save.I],
                        b"AP",
                        fstr::substr(STRING, save.J..=save.K),
                    );
                } else {
                    ZZMKPC(
                        &mut save.PICTUR,
                        save.PBEGS[save.I],
                        save.PENDS[save.I],
                        b"HR",
                        fstr::substr(STRING, save.J..=save.K),
                    );
                }
            } else if (save.ITEM == intrinsics::ICHAR(b"M")) {
                save.NMIN = (save.NMIN + 1);
                *E = (*E + 1);

                NPARSD(
                    fstr::substr(STRING, save.J..=save.K),
                    &mut save.HMS[2],
                    ERROR,
                    &mut save.PTR,
                    ctx,
                );

                ZZMKPC(
                    &mut save.PICTUR,
                    save.PBEGS[save.I],
                    save.PENDS[save.I],
                    b"MN",
                    fstr::substr(STRING, save.J..=save.K),
                );
            } else if (save.ITEM == intrinsics::ICHAR(b"S")) {
                save.NSEC = (save.NSEC + 1);
                *E = (*E + 1);

                NPARSD(
                    fstr::substr(STRING, save.J..=save.K),
                    &mut save.HMS[3],
                    ERROR,
                    &mut save.PTR,
                    ctx,
                );

                ZZMKPC(
                    &mut save.PICTUR,
                    save.PBEGS[save.I],
                    save.PENDS[save.I],
                    b"SC",
                    fstr::substr(STRING, save.J..=save.K),
                );
            } else if (save.ITEM == intrinsics::ICHAR(b"J")) {
                save.NJD = (save.NJD + 1);
                *E = (*E + 1);

                NPARSD(
                    fstr::substr(STRING, save.J..=save.K),
                    &mut TVEC[1],
                    ERROR,
                    &mut save.PTR,
                    ctx,
                );

                ZZMKPC(
                    &mut save.PICTUR,
                    save.PBEGS[save.I],
                    save.PENDS[save.I],
                    b"JULIAND",
                    fstr::substr(STRING, save.J..=save.K),
                );
            } else if (save.ITEM == intrinsics::ICHAR(b"i")) {
                fstr::assign(ERROR, STRING);

                ZZINSSUB(&ERROR.to_vec(), b">", (save.K + 1), ERROR);
                ZZINSSUB(&ERROR.to_vec(), b"<", save.J, ERROR);

                PREFIX(
                    b"The meaning of the integer <#> could not be determined: \'",
                    1,
                    ERROR,
                );
                SUFFIX(b"\'", 0, ERROR);

                REPMC(
                    &ERROR.to_vec(),
                    b"#",
                    fstr::substr(STRING, save.J..=save.K),
                    ERROR,
                );

                *E = 0;
                fstr::assign(PIC, b" ");
                save.ZZUNPCK = NO;
                return save.ZZUNPCK;
            } else if (save.ITEM == intrinsics::ICHAR(b"n")) {
                fstr::assign(ERROR, STRING);

                ZZINSSUB(&ERROR.to_vec(), b">", (save.K + 1), ERROR);
                ZZINSSUB(&ERROR.to_vec(), b"<", save.J, ERROR);

                PREFIX(
                    b"The meaning of the decimal number <#> could not be determined: ",
                    1,
                    ERROR,
                );

                REPMC(
                    &ERROR.to_vec(),
                    b"#",
                    fstr::substr(STRING, save.J..=save.K),
                    ERROR,
                );

                *E = 0;
                fstr::assign(PIC, b" ");
                save.ZZUNPCK = NO;
                return save.ZZUNPCK;
            } else {
                fstr::assign(ERROR, STRING);

                ZZINSSUB(&ERROR.to_vec(), b">", (save.K + 1), ERROR);
                ZZINSSUB(&ERROR.to_vec(), b"<", save.J, ERROR);

                PREFIX(
                    b"An unexpected # (\"#\") was encountered in the time string: ",
                    1,
                    ERROR,
                );
                REPMC(&ERROR.to_vec(), b"#", &save.NAMES[save.ITEM], ERROR);
                REPMC(
                    &ERROR.to_vec(),
                    b"#",
                    fstr::substr(STRING, save.J..=save.K),
                    ERROR,
                );

                fstr::assign(PIC, b" ");
                *E = 0;
                save.ZZUNPCK = NO;
                return save.ZZUNPCK;
            }

            save.I += m3__;
        }
    }

    //
    // Ok. Check the counts of substrings to make sure everything
    // looks ok. If so move the HMS into the appropriate slots
    // in TVEC, set the kind of TVEC, set the function value to YES,
    // and RETURN. Note regardless of the correctness of the parsing
    // we have a legitimate format picture at this point so we keep it.
    //
    fstr::assign(PIC, &save.PICTUR);

    if ((((((((save.NYEAR == 1) && (save.NMON == 1)) && (save.NDAY == 1)) && (save.NDOY == 0))
        && (save.NJD == 0))
        && (save.NHOUR <= 1))
        && (save.NMIN <= save.NHOUR))
        && (save.NSEC <= save.NMIN))
    {
        TVEC[4] = save.HMS[1];
        TVEC[5] = save.HMS[2];
        TVEC[6] = save.HMS[3];

        fstr::assign(TRANSL, b"YMD");
        save.ZZUNPCK = YES;
        return save.ZZUNPCK;
    } else if ((((((((save.NYEAR == 1) && (save.NMON == 0)) && (save.NDAY == 0))
        && (save.NJD == 0))
        && (save.NDOY == 1))
        && (save.NHOUR <= 1))
        && (save.NMIN <= save.NHOUR))
        && (save.NSEC <= save.NMIN))
    {
        TVEC[3] = save.HMS[1];
        TVEC[4] = save.HMS[2];
        TVEC[5] = save.HMS[3];

        fstr::assign(TRANSL, b"YD");
        save.ZZUNPCK = YES;
        return save.ZZUNPCK;
    } else if ((((((((save.NYEAR == 0) && (save.NMON == 0)) && (save.NDAY == 0))
        && (save.NJD == 1))
        && (save.NDOY == 0))
        && (save.NHOUR <= 0))
        && (save.NMIN <= 0))
        && (save.NSEC <= 0))
    {
        fstr::assign(TRANSL, b"JD");
        save.ZZUNPCK = YES;
        return save.ZZUNPCK;
    }
    //
    // If we're still here, there is some kind of an error
    // in the input string. There are a lot of possible
    // problems.
    //
    *E = 0;

    if (((((((save.NYEAR == 0) && (save.NDAY == 0)) && (save.NJD == 0)) && (save.NDOY == 0))
        && (save.NHOUR == 0))
        && (save.NMIN == 0))
        && (save.NSEC == 0))
    {
        fstr::assign(
            ERROR,
            b"No numeric components were supplied in the time string. ",
        );
    } else if (save.NJD == 1) {
        fstr::assign(
            ERROR,
            b"The string possesses calendar components in addition to Julian Date specifier. ",
        );
    } else if (save.NJD > 1) {
        fstr::assign(
            ERROR,
            b"There is more than one Julian Date specified in the epoch string. ",
        );
    } else if (save.NYEAR == 0) {
        fstr::assign(
            ERROR,
            b"The year associated with the calendar string \"#\" could not be identified. ",
        );

        REPMC(&ERROR.to_vec(), b"#", STRING, ERROR);
    } else if (save.NYEAR > 1) {
        fstr::assign(ERROR, STRING);
        fstr::assign(&mut save.MESSGE, b"Two substrings indicating a calendar year were identified in the input time string <#> and <#>: \"");

        save.P1 = POS(&save.REP, b"Y", 1);
        save.P2 = POS(&save.REP, b"Y", (save.P1 + 1));

        save.J = save.BEGS[save.P2];
        save.K = save.ENDS[save.P2];

        ZZINSSUB(&ERROR.to_vec(), b">", (save.K + 1), ERROR);
        ZZINSSUB(&ERROR.to_vec(), b"<", save.J, ERROR);
        REPMC(
            &save.MESSGE.to_vec(),
            b"#",
            fstr::substr(STRING, save.J..=save.K),
            &mut save.MESSGE,
        );

        save.J = save.BEGS[save.P1];
        save.K = save.ENDS[save.P1];

        ZZINSSUB(&ERROR.to_vec(), b">", (save.K + 1), ERROR);
        ZZINSSUB(&ERROR.to_vec(), b"<", save.J, ERROR);
        REPMC(
            &save.MESSGE.to_vec(),
            b"#",
            fstr::substr(STRING, save.J..=save.K),
            &mut save.MESSGE,
        );

        PREFIX(&save.MESSGE, 1, ERROR);
        SUFFIX(b"\"", 0, ERROR);
    } else if ((save.NMON > 0) && (save.NDOY > 0)) {
        fstr::assign(ERROR, STRING);
        fstr::assign(
            &mut save.MESSGE,
            b"Both a day of year and month were identified in the input string. \"",
        );

        save.P2 = intrinsics::MAX0(&[POS(&save.REP, b"m", 1), POS(&save.REP, b"y", 1)]);

        save.P1 = intrinsics::MIN0(&[POS(&save.REP, b"m", 1), POS(&save.REP, b"y", 1)]);

        save.J = save.BEGS[save.P2];
        save.K = save.ENDS[save.P2];

        ZZINSSUB(&ERROR.to_vec(), b">", (save.K + 1), ERROR);
        ZZINSSUB(&ERROR.to_vec(), b"<", save.J, ERROR);

        save.J = save.BEGS[save.P1];
        save.K = save.ENDS[save.P1];

        ZZINSSUB(&ERROR.to_vec(), b">", (save.K + 1), ERROR);
        ZZINSSUB(&ERROR.to_vec(), b"<", save.J, ERROR);

        PREFIX(&save.MESSGE, 1, ERROR);
        SUFFIX(b"\"", 0, ERROR);
    } else if (save.NMON > 1) {
        fstr::assign(ERROR, STRING);
        fstr::assign(&mut save.MESSGE, b"Two substrings indicating a calendar month were identified in the input time string <#> and <#>: \"");

        save.P1 = POS(&save.REP, b"m", 1);
        save.P2 = POS(&save.REP, b"m", (save.P1 + 1));

        save.J = save.BEGS[save.P2];
        save.K = save.ENDS[save.P2];

        ZZINSSUB(&ERROR.to_vec(), b">", (save.K + 1), ERROR);
        ZZINSSUB(&ERROR.to_vec(), b"<", save.J, ERROR);
        REPMC(
            &save.MESSGE.to_vec(),
            b"#",
            fstr::substr(STRING, save.J..=save.K),
            &mut save.MESSGE,
        );

        save.J = save.BEGS[save.P1];
        save.K = save.ENDS[save.P1];

        ZZINSSUB(&ERROR.to_vec(), b">", (save.K + 1), ERROR);
        ZZINSSUB(&ERROR.to_vec(), b"<", save.J, ERROR);
        REPMC(
            &save.MESSGE.to_vec(),
            b"#",
            fstr::substr(STRING, save.J..=save.K),
            &mut save.MESSGE,
        );

        PREFIX(&save.MESSGE, 1, ERROR);
        SUFFIX(b"\"", 0, ERROR);
    } else if (save.NDOY > 1) {
        fstr::assign(ERROR, STRING);
        fstr::assign(&mut save.MESSGE, b"Two substrings indicating a day of year were identified in the input time string <#> and <#>: \"");

        save.P1 = POS(&save.REP, b"y", 1);
        save.P2 = POS(&save.REP, b"y", (save.P1 + 1));

        save.J = save.BEGS[save.P2];
        save.K = save.ENDS[save.P2];

        ZZINSSUB(&ERROR.to_vec(), b">", (save.K + 1), ERROR);
        ZZINSSUB(&ERROR.to_vec(), b"<", save.J, ERROR);
        REPMC(
            &save.MESSGE.to_vec(),
            b"#",
            fstr::substr(STRING, save.J..=save.K),
            &mut save.MESSGE,
        );

        save.J = save.BEGS[save.P1];
        save.K = save.ENDS[save.P1];

        ZZINSSUB(&ERROR.to_vec(), b">", (save.K + 1), ERROR);
        ZZINSSUB(&ERROR.to_vec(), b"<", save.J, ERROR);
        REPMC(
            &save.MESSGE.to_vec(),
            b"#",
            fstr::substr(STRING, save.J..=save.K),
            &mut save.MESSGE,
        );

        PREFIX(&save.MESSGE, 1, ERROR);
        SUFFIX(b"\"", 0, ERROR);
    } else if (save.NDAY > 1) {
        fstr::assign(ERROR, STRING);
        fstr::assign(&mut save.MESSGE, b"Two substrings indicating a day of month were identified in the input time string <#> and <#>: \"");

        save.P1 = POS(&save.REP, b"D", 1);
        save.P2 = POS(&save.REP, b"D", (save.P1 + 1));

        save.J = save.BEGS[save.P2];
        save.K = save.ENDS[save.P2];

        ZZINSSUB(&ERROR.to_vec(), b">", (save.K + 1), ERROR);
        ZZINSSUB(&ERROR.to_vec(), b"<", save.J, ERROR);
        REPMC(
            &save.MESSGE.to_vec(),
            b"#",
            fstr::substr(STRING, save.J..=save.K),
            &mut save.MESSGE,
        );

        save.J = save.BEGS[save.P1];
        save.K = save.ENDS[save.P1];

        ZZINSSUB(&ERROR.to_vec(), b">", (save.K + 1), ERROR);
        ZZINSSUB(&ERROR.to_vec(), b"<", save.J, ERROR);
        REPMC(
            &save.MESSGE.to_vec(),
            b"#",
            fstr::substr(STRING, save.J..=save.K),
            &mut save.MESSGE,
        );

        PREFIX(&save.MESSGE, 1, ERROR);
        SUFFIX(b"\"", 0, ERROR);
    } else if (save.NHOUR > 1) {
        fstr::assign(ERROR, STRING);
        fstr::assign(&mut save.MESSGE, b"Two substrings representing an hour of the day were identified in the input time string <#> and <#>: \"");

        save.P1 = POS(&save.REP, b"H", 1);
        save.P2 = POS(&save.REP, b"H", (save.P1 + 1));

        save.J = save.BEGS[save.P2];
        save.K = save.ENDS[save.P2];

        ZZINSSUB(&ERROR.to_vec(), b">", (save.K + 1), ERROR);
        ZZINSSUB(&ERROR.to_vec(), b"<", save.J, ERROR);
        REPMC(
            &save.MESSGE.to_vec(),
            b"#",
            fstr::substr(STRING, save.J..=save.K),
            &mut save.MESSGE,
        );

        save.J = save.BEGS[save.P1];
        save.K = save.ENDS[save.P1];

        ZZINSSUB(&ERROR.to_vec(), b">", (save.K + 1), ERROR);
        ZZINSSUB(&ERROR.to_vec(), b"<", save.J, ERROR);
        REPMC(
            &save.MESSGE.to_vec(),
            b"#",
            fstr::substr(STRING, save.J..=save.K),
            &mut save.MESSGE,
        );

        PREFIX(&save.MESSGE, 1, ERROR);
        SUFFIX(b"\"", 0, ERROR);
    } else if (save.NMIN > 1) {
        fstr::assign(ERROR, STRING);
        fstr::assign(&mut save.MESSGE, b"Two substrings representing minutes of the hour were identified in the input time string <#> and <#>: \"");

        save.P1 = POS(&save.REP, b"M", 1);
        save.P2 = POS(&save.REP, b"M", (save.P1 + 1));

        save.J = save.BEGS[save.P2];
        save.K = save.ENDS[save.P2];

        ZZINSSUB(&ERROR.to_vec(), b">", (save.K + 1), ERROR);
        ZZINSSUB(&ERROR.to_vec(), b"<", save.J, ERROR);
        REPMC(
            &save.MESSGE.to_vec(),
            b"#",
            fstr::substr(STRING, save.J..=save.K),
            &mut save.MESSGE,
        );

        save.J = save.BEGS[save.P1];
        save.K = save.ENDS[save.P1];

        ZZINSSUB(&ERROR.to_vec(), b">", (save.K + 1), ERROR);
        ZZINSSUB(&ERROR.to_vec(), b"<", save.J, ERROR);
        REPMC(
            &save.MESSGE.to_vec(),
            b"#",
            fstr::substr(STRING, save.J..=save.K),
            &mut save.MESSGE,
        );

        PREFIX(&save.MESSGE, 1, ERROR);
        SUFFIX(b"\"", 0, ERROR);
    } else if (save.NSEC > 1) {
        fstr::assign(ERROR, STRING);
        fstr::assign(&mut save.MESSGE, b"Two substrings representing seconds were identified in the input time string <#> and <#>: \"");

        save.P1 = POS(&save.REP, b"S", 1);
        save.P2 = POS(&save.REP, b"S", (save.P1 + 1));

        save.J = save.BEGS[save.P2];
        save.K = save.ENDS[save.P2];

        ZZINSSUB(&ERROR.to_vec(), b">", (save.K + 1), ERROR);
        ZZINSSUB(&ERROR.to_vec(), b"<", save.J, ERROR);
        REPMC(
            &save.MESSGE.to_vec(),
            b"#",
            fstr::substr(STRING, save.J..=save.K),
            &mut save.MESSGE,
        );

        save.J = save.BEGS[save.P1];
        save.K = save.ENDS[save.P1];

        ZZINSSUB(&ERROR.to_vec(), b">", (save.K + 1), ERROR);
        ZZINSSUB(&ERROR.to_vec(), b"<", save.J, ERROR);
        REPMC(
            &save.MESSGE.to_vec(),
            b"#",
            fstr::substr(STRING, save.J..=save.K),
            &mut save.MESSGE,
        );

        PREFIX(&save.MESSGE, 1, ERROR);
        SUFFIX(b"\"", 0, ERROR);
    } else if ((save.NDOY == 0) && (save.NMON == 0)) {
        fstr::assign(
            ERROR,
            b"Neither a month nor day of year could be identified in the input time string: \"#\" ",
        );

        REPMC(&ERROR.to_vec(), b"#", STRING, ERROR);
    } else if ((save.NMON == 1) && (save.NDAY == 0)) {
        fstr::assign(ERROR, b"A month was identified in the time string \"#\", but a day of month could not be identified. ");

        REPMC(&ERROR.to_vec(), b"#", STRING, ERROR);
    } else if ((save.NMON == 0) && (save.NDAY == 1)) {
        fstr::assign(ERROR, b"A day of month was identified in the time string \"#\", but the month it belongs to could not be identified. ");

        REPMC(&ERROR.to_vec(), b"#", STRING, ERROR);
    } else if (save.NMIN > save.NHOUR) {
        fstr::assign(ERROR, b"A minutes components of the time  was identified in the time string \"#\", but the hours component could not be identified. ");

        REPMC(&ERROR.to_vec(), b"#", STRING, ERROR);
    } else if (save.NSEC > save.NMIN) {
        fstr::assign(ERROR, b"A seconds components of the time was identified in the time string \"#\", but the minutes component could not be identified. ");

        REPMC(&ERROR.to_vec(), b"#", STRING, ERROR);
    }

    save.ZZUNPCK = NO;
    save.ZZUNPCK
}

//$Procedure ZZVALT ( Private, Time --- Value Based Tokens )
pub fn ZZVALT(STRING: &[u8], B: i32, E: i32, LETTER: &[u8], ctx: &mut Context) -> bool {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let LETTER = &LETTER[..1 as usize];

    //
    // So far no translations have been performed.
    //
    save.DID = NO;
    //
    // Examine each token to see if it is an integer.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = save.SIZE;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.ITEM = intrinsics::ICHAR(fstr::substr(&save.REP, save.I..=save.I));

            if (save.ITEM == intrinsics::ICHAR(b"i")) {
                //
                // We've got an integer. Parse it to see if it
                // is in the specified range.
                //
                save.J = save.BEGS[save.I];
                save.K = save.ENDS[save.I];

                NPARSI(
                    fstr::substr(STRING, save.J..=save.K),
                    &mut save.VALUE,
                    &mut save.MYERR,
                    &mut save.PTR,
                    ctx,
                );

                if (((save.PTR == 0) && (save.VALUE >= B)) && (save.VALUE <= E)) {
                    fstr::assign(fstr::substr_mut(&mut save.REP, save.I..=save.I), LETTER);
                    save.DID = YES;
                }
            }

            save.I += m3__;
        }
    }

    save.ZZVALT = save.DID;
    save.ZZVALT
}
