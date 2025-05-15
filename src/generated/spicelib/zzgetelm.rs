//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const START: i32 = 0;
const KNDT20: i32 = (START + 1);
const KNDD60: i32 = (KNDT20 + 1);
const KBSTAR: i32 = (KNDD60 + 1);
const KINCL: i32 = (KBSTAR + 1);
const KNODE0: i32 = (KINCL + 1);
const KECC: i32 = (KNODE0 + 1);
const KOMEGA: i32 = (KECC + 1);
const KMO: i32 = (KOMEGA + 1);
const KNO: i32 = (KMO + 1);
const KEPOCH: i32 = (KNO + 1);
const NELEMS: i32 = KEPOCH;
const WDSIZE: i32 = 32;
const LNSIZE: i32 = 160;
const MAXP: i32 = 37;
const MINP: i32 = -MAXP;
const ONE: f64 = 1.0;
const TEN: f64 = 10.0;
const MNPDAY: f64 = 1440.0;

struct SaveVars {
    ERRPRS: ActualCharArray,
    TERM: ActualCharArray,
    CYEAR: Vec<u8>,
    CDAY: Vec<u8>,
    CNDT20: Vec<u8>,
    CNDD60: Vec<u8>,
    CIEXP: Vec<u8>,
    CBSTAR: Vec<u8>,
    CIBEXP: Vec<u8>,
    CINCL: Vec<u8>,
    CNODE0: Vec<u8>,
    CECC: Vec<u8>,
    COMEGA: Vec<u8>,
    CMO: Vec<u8>,
    CNO: Vec<u8>,
    BSTAR: f64,
    DAY: f64,
    ECC: f64,
    INCL: f64,
    MO: f64,
    NDD60: f64,
    NDT20: f64,
    NO: f64,
    NODE0: f64,
    OMEGA: f64,
    D2R: f64,
    PI2: f64,
    POWER: StackArray<f64, 75>,
    TVEC: StackArray<f64, 8>,
    BEXP: i32,
    BEGYR: i32,
    NEXP: i32,
    PTR: i32,
    YEAR: i32,
    YR: i32,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ERRPRS = ActualCharArray::new(LNSIZE, 1..=13);
        let mut TERM = ActualCharArray::new(LNSIZE, 1..=13);
        let mut CYEAR = vec![b' '; WDSIZE as usize];
        let mut CDAY = vec![b' '; WDSIZE as usize];
        let mut CNDT20 = vec![b' '; WDSIZE as usize];
        let mut CNDD60 = vec![b' '; WDSIZE as usize];
        let mut CIEXP = vec![b' '; WDSIZE as usize];
        let mut CBSTAR = vec![b' '; WDSIZE as usize];
        let mut CIBEXP = vec![b' '; WDSIZE as usize];
        let mut CINCL = vec![b' '; WDSIZE as usize];
        let mut CNODE0 = vec![b' '; WDSIZE as usize];
        let mut CECC = vec![b' '; WDSIZE as usize];
        let mut COMEGA = vec![b' '; WDSIZE as usize];
        let mut CMO = vec![b' '; WDSIZE as usize];
        let mut CNO = vec![b' '; WDSIZE as usize];
        let mut BSTAR: f64 = 0.0;
        let mut DAY: f64 = 0.0;
        let mut ECC: f64 = 0.0;
        let mut INCL: f64 = 0.0;
        let mut MO: f64 = 0.0;
        let mut NDD60: f64 = 0.0;
        let mut NDT20: f64 = 0.0;
        let mut NO: f64 = 0.0;
        let mut NODE0: f64 = 0.0;
        let mut OMEGA: f64 = 0.0;
        let mut D2R: f64 = 0.0;
        let mut PI2: f64 = 0.0;
        let mut POWER = StackArray::<f64, 75>::new(MINP..=MAXP);
        let mut TVEC = StackArray::<f64, 8>::new(1..=8);
        let mut BEXP: i32 = 0;
        let mut BEGYR: i32 = 0;
        let mut NEXP: i32 = 0;
        let mut PTR: i32 = 0;
        let mut YEAR: i32 = 0;
        let mut YR: i32 = 0;
        let mut FIRST: bool = false;

        FIRST = true;

        Self {
            ERRPRS,
            TERM,
            CYEAR,
            CDAY,
            CNDT20,
            CNDD60,
            CIEXP,
            CBSTAR,
            CIBEXP,
            CINCL,
            CNODE0,
            CECC,
            COMEGA,
            CMO,
            CNO,
            BSTAR,
            DAY,
            ECC,
            INCL,
            MO,
            NDD60,
            NDT20,
            NO,
            NODE0,
            OMEGA,
            D2R,
            PI2,
            POWER,
            TVEC,
            BEXP,
            BEGYR,
            NEXP,
            PTR,
            YEAR,
            YR,
            FIRST,
        }
    }
}

//$Procedure ZZGETELM ( Get the components from two-line elements)
pub fn ZZGETELM(
    FRSTYR: i32,
    LINES: CharArray,
    EPOCH: &mut f64,
    ELEMS: &mut [f64],
    OK: &mut bool,
    ERROR: &mut [u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let LINES = DummyCharArray::new(LINES, None, 1..=2);
    let mut ELEMS = DummyArrayMut::new(ELEMS, 1..);

    //
    // Spicelib functions
    //

    //
    // An enumeration of the various components of the
    // elements array---ELEMS
    //
    //    KNDT20
    //    KNDD60
    //    KBSTAR
    //    KINCL
    //    KNODE0
    //    KECC
    //    KOMEGA
    //    KMO
    //    KNO
    //

    //
    // Character string lengths
    //

    //
    // Maximum exponent (base 10)
    //

    //
    // Double precision constants.
    //

    //
    // Minutes/day
    //

    //
    // Local variables
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZGETELM", ctx)?;

    //
    // Initialize the error indicators and the elements to zero.
    //
    *OK = true;
    fstr::assign(ERROR, b" ");
    ELEMS[KNDT20] = 0.0;
    ELEMS[KNDD60] = 0.0;
    ELEMS[KBSTAR] = 0.0;
    ELEMS[KINCL] = 0.0;
    ELEMS[KNODE0] = 0.0;
    ELEMS[KECC] = 0.0;
    ELEMS[KOMEGA] = 0.0;
    ELEMS[KMO] = 0.0;
    ELEMS[KNO] = 0.0;
    ELEMS[KEPOCH] = 0.0;
    *EPOCH = 0.0;

    //
    // First entry initialization.
    //
    if save.FIRST {
        //
        // Define two constants. This initialization proves the most
        // useful when processing thousands of TLE sets.
        //
        save.D2R = RPD(ctx);
        save.PI2 = TWOPI(ctx);

        save.FIRST = false;

        save.POWER[0] = ONE;

        for I in 1..=MAXP {
            save.POWER[I] = (TEN * save.POWER[(I - 1)]);
            save.POWER[-I] = (ONE / save.POWER[I]);
        }

        fstr::assign(
            save.TERM.get_mut(1),
            b"\"YEAR\" (characters 19 to 20 of the first line of a two-line element set)",
        );

        fstr::assign(
            save.TERM.get_mut(2),
            b"\"DAY\" (characters 21 to 32 of the first line of a two-line element set)",
        );

        fstr::assign(
            save.TERM.get_mut(3),
            b"\"NDT20\" (characters 34 to 43 of the first line of a two-line element set)",
        );

        fstr::assign(
            save.TERM.get_mut(4),
            b"\"NDD60\" (characters 45 to 45 of the first line of a two-line element set)",
        );

        fstr::assign(
            save.TERM.get_mut(5),
            b"\"IEXP\" (characters 51 to 52 of the first line of a two-line element set)",
        );

        fstr::assign(
            save.TERM.get_mut(6),
            b"\"BSTAR\" (characters 54 to 54 of the first line of a two-line element set)",
        );

        fstr::assign(
            save.TERM.get_mut(7),
            b"\"IBEXP\" (characters 60 to 61 of the first line of a two-line element set)",
        );

        fstr::assign(
            save.TERM.get_mut(8),
            b"\"INCL\" (characters 9 to 16 of the second line of a two-line element set)",
        );

        fstr::assign(
            save.TERM.get_mut(9),
            b"\"NODE0\" (characters 18 to 25 of the second line of a two-line element set)",
        );

        fstr::assign(
            save.TERM.get_mut(10),
            b"\"ECC\" (characters 27 to 33 of the second line of a two-line element set)",
        );

        fstr::assign(
            save.TERM.get_mut(11),
            b"\"OMEGA\" (characters 35 to 42 of the second line of a two-line element set)",
        );

        fstr::assign(
            save.TERM.get_mut(12),
            b"\"MO\" (characters 44 to 51 of the second line of a two-line element set)",
        );

        fstr::assign(
            save.TERM.get_mut(13),
            b"\"NO\" (characters 53 to 63 of the second line of a two-line element set)",
        );
    }

    //
    // Ensure the vehicle IDs match in each line.
    //
    if fstr::ne(
        fstr::substr(LINES.get(1), 2..=7),
        fstr::substr(LINES.get(2), 2..=7),
    ) {
        //
        // Vehicle IDs do not match. Flag an error.
        //
        fstr::assign(ERROR, b"Line 1 of the TLE pair tagged with vehicle ID #1,  line 2 of TLE pair tagged with vehicle ID #2");
        REPMC(
            &ERROR.to_vec(),
            b"#1",
            fstr::substr(&LINES[1], 2..=7),
            ERROR,
        );
        REPMC(
            &ERROR.to_vec(),
            b"#2",
            fstr::substr(&LINES[2], 2..=7),
            ERROR,
        );

        *OK = false;

        CHKOUT(b"ZZGETELM", ctx)?;
        return Ok(());
    }

    //
    // Check line format and length.
    //
    for K in 1..=2 {
        if ((LASTNB(&LINES[K]) != 68) && (LASTNB(&LINES[K]) != 69)) {
            //
            // The TLE data line was not 68 or 69 characters long (ignoring
            // trailing whitespace). Flag an error.
            //
            fstr::assign(ERROR, b"Line #1 of the TLE has incorrect data length. Expected length 68 or 69 elements, actual length: #2. TLE line value: #3");
            REPMI(&ERROR.to_vec(), b"#1", K, ERROR, ctx);
            REPMI(&ERROR.to_vec(), b"#2", LASTNB(&LINES[K]), ERROR, ctx);
            REPMC(&ERROR.to_vec(), b"#3", &LINES[K], ERROR);

            *OK = false;

            CHKOUT(b"ZZGETELM", ctx)?;
            return Ok(());
        }
    }

    //
    // This isn't particularly pretty, but it is straight
    // forward.  According to the documentation on the two line
    // element sets, (as well as what's indicated by the program
    // driver that is documented in SPACETRACK REPORT NO.3
    // we can simply pick out the various components of the
    // elements from the input lines 1 and 2.
    //
    // For the record we include the DECODE statement in DRIVER
    // for fetching the data out of lines one and two (after a bit
    // of pretty printing).  Note that some of these formats have
    // and implied decimal point.  In particular f6.5 and f7.7  in
    // all other cases the decimal points seem to be given explicitely.
    //
    // decode (abuf,702) epoch,     xndt20,                       ...
    //                   xndd60,    iexp,        bstar,    ibexp, ...
    //                   xincl,     xnodeo,                       ...
    //                   eo,        omegao,     xmo,       xno
    // format(      18x, d14.8, 1x, f10.8,
    //               1x, f6.5,      i2,     1x, f6.5,      i2,    /,
    //               8x, f8.4,  1x, f8.4,
    //               1x, f7.7,  1x, f8.4,   1x, f8.4   1x, f11.8 )
    //
    // Note that in the two-line element sets, the epoch is read
    // as a single number.  However the documentation that describes
    // this data (as well as the code in THETAG) show that it's a lot
    // easier to capture the year and day of year separately.
    //
    fstr::assign(&mut save.CYEAR, fstr::substr(LINES.get(1), 19..=20));
    fstr::assign(&mut save.CDAY, fstr::substr(LINES.get(1), 21..=32));
    fstr::assign(&mut save.CNDT20, fstr::substr(LINES.get(1), 34..=43));
    fstr::assign(
        &mut save.CNDD60,
        &fstr::concat(
            &fstr::concat(fstr::substr(LINES.get(1), 45..=45), b"."),
            fstr::substr(LINES.get(1), 46..=50),
        ),
    );
    fstr::assign(&mut save.CIEXP, fstr::substr(LINES.get(1), 51..=52));
    fstr::assign(
        &mut save.CBSTAR,
        &fstr::concat(
            &fstr::concat(fstr::substr(LINES.get(1), 54..=54), b"."),
            fstr::substr(LINES.get(1), 55..=59),
        ),
    );
    fstr::assign(&mut save.CIBEXP, fstr::substr(LINES.get(1), 60..=61));

    fstr::assign(&mut save.CINCL, fstr::substr(LINES.get(2), 9..=16));
    fstr::assign(&mut save.CNODE0, fstr::substr(LINES.get(2), 18..=25));
    fstr::assign(
        &mut save.CECC,
        &fstr::concat(b"0.", fstr::substr(LINES.get(2), 27..=33)),
    );
    fstr::assign(&mut save.COMEGA, fstr::substr(LINES.get(2), 35..=42));
    fstr::assign(&mut save.CMO, fstr::substr(LINES.get(2), 44..=51));
    fstr::assign(&mut save.CNO, fstr::substr(LINES.get(2), 53..=63));

    //
    // Parse the numerical values from the data string.
    //
    NPARSI(
        &save.CYEAR,
        &mut save.YR,
        &mut save.ERRPRS[1],
        &mut save.PTR,
        ctx,
    );
    NPARSD(
        &save.CDAY,
        &mut save.DAY,
        &mut save.ERRPRS[2],
        &mut save.PTR,
        ctx,
    );
    NPARSD(
        &save.CNDT20,
        &mut save.NDT20,
        &mut save.ERRPRS[3],
        &mut save.PTR,
        ctx,
    );
    NPARSD(
        &save.CNDD60,
        &mut save.NDD60,
        &mut save.ERRPRS[4],
        &mut save.PTR,
        ctx,
    );
    NPARSI(
        &save.CIEXP,
        &mut save.NEXP,
        &mut save.ERRPRS[5],
        &mut save.PTR,
        ctx,
    );
    NPARSD(
        &save.CBSTAR,
        &mut save.BSTAR,
        &mut save.ERRPRS[6],
        &mut save.PTR,
        ctx,
    );
    NPARSI(
        &save.CIBEXP,
        &mut save.BEXP,
        &mut save.ERRPRS[7],
        &mut save.PTR,
        ctx,
    );
    NPARSD(
        &save.CINCL,
        &mut save.INCL,
        &mut save.ERRPRS[8],
        &mut save.PTR,
        ctx,
    );
    NPARSD(
        &save.CNODE0,
        &mut save.NODE0,
        &mut save.ERRPRS[9],
        &mut save.PTR,
        ctx,
    );
    NPARSD(
        &save.CECC,
        &mut save.ECC,
        &mut save.ERRPRS[10],
        &mut save.PTR,
        ctx,
    );
    NPARSD(
        &save.COMEGA,
        &mut save.OMEGA,
        &mut save.ERRPRS[11],
        &mut save.PTR,
        ctx,
    );
    NPARSD(
        &save.CMO,
        &mut save.MO,
        &mut save.ERRPRS[12],
        &mut save.PTR,
        ctx,
    );
    NPARSD(
        &save.CNO,
        &mut save.NO,
        &mut save.ERRPRS[13],
        &mut save.PTR,
        ctx,
    );

    //
    // Check for parse errors.
    //
    for I in 1..=13 {
        if fstr::ne(save.ERRPRS.get(I), b" ") {
            //
            // Something could not parse. Set the error message then
            // return.
            //
            fstr::assign(
                ERROR,
                b"An error occurred while trying to parse the term #. The diagnostic was:  # ",
            );
            REPMC(&ERROR.to_vec(), b"#", &save.TERM[I], ERROR);
            REPMC(&ERROR.to_vec(), b"#", &save.ERRPRS[I], ERROR);

            *OK = false;

            CHKOUT(b"ZZGETELM", ctx)?;
            return Ok(());
        }
    }

    //
    // Check for reasonable exponets; a single digit. These should
    // probably be LE 0.
    //
    if (i32::abs(save.NEXP) > 9) {
        fstr::assign(
            ERROR,
            b"NEXP (exponent) not a single digit. Actual value #1",
        );
        REPMI(&ERROR.to_vec(), b"#1", save.NEXP, ERROR, ctx);

        *OK = false;

        CHKOUT(b"ZZGETELM", ctx)?;
        return Ok(());
    }

    if (i32::abs(save.BEXP) > 9) {
        fstr::assign(
            ERROR,
            b"BEXP (exponent) not a single digit. Actual value #1",
        );
        REPMI(&ERROR.to_vec(), b"#1", save.BEXP, ERROR, ctx);

        *OK = false;

        CHKOUT(b"ZZGETELM", ctx)?;
        return Ok(());
    }

    //
    // Confirm correct bounds on angular values.
    //
    // NODE0 - right ascension of the ascending node, [0,360)
    //
    if ((save.NODE0 < 0.0) || (save.NODE0 >= 360.0)) {
        fstr::assign(
            ERROR,
            b"NODE0 (RA acend node) expected bounds [0,360). Actual value #1",
        );
        REPMD(&ERROR.to_vec(), b"#1", save.NODE0, 4, ERROR, ctx);

        *OK = false;

        CHKOUT(b"ZZGETELM", ctx)?;
        return Ok(());
    }

    //
    // OMEAGA - argument of the periapsis, [0,360)
    //
    if ((save.OMEGA < 0.0) || (save.OMEGA >= 360.0)) {
        fstr::assign(
            ERROR,
            b"OMEGA (arg periap) expected bounds [0,360). Actual value #1",
        );
        REPMD(&ERROR.to_vec(), b"#1", save.OMEGA, 4, ERROR, ctx);

        *OK = false;

        CHKOUT(b"ZZGETELM", ctx)?;
        return Ok(());
    }

    //
    // MO - mean anomoly, [0,360)
    //
    if ((save.MO < 0.0) || (save.MO >= 360.0)) {
        fstr::assign(
            ERROR,
            b"MO (mean anomoly) expected bounds [0,360). Actual value #1",
        );
        REPMD(&ERROR.to_vec(), b"#1", save.MO, 4, ERROR, ctx);

        *OK = false;

        CHKOUT(b"ZZGETELM", ctx)?;
        return Ok(());
    }

    //
    // INCL - inclination, [0,180]
    //
    if ((save.INCL < 0.0) || (save.INCL > 180.0)) {
        fstr::assign(
            ERROR,
            b"INCL (inclination) expected bounds [0,180). Actual value #1",
        );
        REPMD(&ERROR.to_vec(), b"#1", save.INCL, 4, ERROR, ctx);

        *OK = false;

        CHKOUT(b"ZZGETELM", ctx)?;
        return Ok(());
    }

    //
    // NO - mean motion (0,20) (Earth orbiter).
    //
    if ((save.NO > 20.0) || (save.NO < 0.0)) {
        fstr::assign(
            ERROR,
            b"NO (mean motion) expected bounds (0,20). Actual value #1",
        );
        REPMD(&ERROR.to_vec(), b"#1", save.NO, 4, ERROR, ctx);

        *OK = false;

        CHKOUT(b"ZZGETELM", ctx)?;
        return Ok(());
    }

    //
    // Finish up the computation of NDD60 and BSTAR
    //
    save.NDD60 = (save.NDD60 * save.POWER[save.NEXP]);
    save.BSTAR = (save.BSTAR * save.POWER[save.BEXP]);

    //
    // Convert everything from degrees to radians ...
    //
    save.NODE0 = (save.NODE0 * save.D2R);
    save.OMEGA = (save.OMEGA * save.D2R);
    save.MO = (save.MO * save.D2R);
    save.INCL = (save.INCL * save.D2R);

    //
    // ... and from revolutions/day**n to radians/minutes**n
    //
    save.NO = ((save.NO * save.PI2) / MNPDAY);
    save.NDT20 = (((save.NDT20 * save.PI2) / MNPDAY) / MNPDAY);
    save.NDD60 = ((((save.NDD60 * save.PI2) / MNPDAY) / MNPDAY) / MNPDAY);

    //
    // Finally, we need to convert the input epoch to
    // seconds past 2000. First let's adjust the year.
    // Add to YR the largest multiple of 100 that is
    // less than or equal to FRSTYR
    //
    save.BEGYR = ((FRSTYR / 100) * 100);
    save.YEAR = (save.BEGYR + save.YR);

    if (save.YEAR < FRSTYR) {
        save.YEAR = (save.YEAR + 100);
    }

    //
    // Compute the epoch of the year and date.
    //
    save.TVEC[1] = (save.YEAR as f64);
    save.TVEC[2] = save.DAY;

    TTRANS(b"YD.D", b"TDB", save.TVEC.as_slice_mut(), ctx)?;

    *EPOCH = save.TVEC[1];

    //
    // That's it.  Load ELEMS with the elements and ship them
    // back to the calling routine.
    //
    ELEMS[KNDT20] = save.NDT20;
    ELEMS[KNDD60] = save.NDD60;
    ELEMS[KBSTAR] = save.BSTAR;
    ELEMS[KINCL] = save.INCL;
    ELEMS[KNODE0] = save.NODE0;
    ELEMS[KECC] = save.ECC;
    ELEMS[KOMEGA] = save.OMEGA;
    ELEMS[KMO] = save.MO;
    ELEMS[KNO] = save.NO;
    ELEMS[KEPOCH] = *EPOCH;

    CHKOUT(b"ZZGETELM", ctx)?;
    Ok(())
}
