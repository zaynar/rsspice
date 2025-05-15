//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const ITRUE: i32 = 1;
const IFALSE: i32 = -1;
const CTRUE: &[u8] = b"T";
const CFALSE: &[u8] = b"F";
const CDOFF: i32 = 24;
const CDSCSZ: i32 = 11;
const CLSIDX: i32 = 1;
const TYPIDX: i32 = (CLSIDX + 1);
const LENIDX: i32 = (TYPIDX + 1);
const SIZIDX: i32 = (LENIDX + 1);
const NAMIDX: i32 = (SIZIDX + 1);
const IXTIDX: i32 = (NAMIDX + 1);
const IXPIDX: i32 = (IXTIDX + 1);
const NFLIDX: i32 = (IXPIDX + 1);
const ORDIDX: i32 = (NFLIDX + 1);
const METIDX: i32 = (ORDIDX + 1);
const MXJRS: i32 = 200;
const JSZIDX: i32 = 1;
const JRCIDX: i32 = 2;
const JTCIDX: i32 = 3;
const JSCIDX: i32 = 4;
const JSVBAS: i32 = 4;
const EQ: i32 = 1;
const GE: i32 = (EQ + 1);
const GT: i32 = (GE + 1);
const LE: i32 = (GT + 1);
const LT: i32 = (LE + 1);
const NE: i32 = (LT + 1);
const LIKE: i32 = (NE + 1);
const UNLIKE: i32 = (LIKE + 1);
const ISNULL: i32 = (UNLIKE + 1);
const NOTNUL: i32 = (ISNULL + 1);
const MAXQRY: i32 = 2000;
const MAXSEL: i32 = 50;
const MAXTAB: i32 = 10;
const MAXCON: i32 = 1000;
const MXJOIN: i32 = 10;
const MXJCON: i32 = 100;
const MAXORD: i32 = 10;
const MAXTOK: i32 = 500;
const MAXQNM: i32 = 100;
const MAXCLN: i32 = MAXQRY;
const MAXSTR: i32 = 1024;
const EQARCH: i32 = 2;
const EQINIT: i32 = (EQARCH + 1);
const EQPARS: i32 = (EQINIT + 1);
const EQNRES: i32 = (EQPARS + 1);
const EQTRES: i32 = (EQNRES + 1);
const EQSCHK: i32 = (EQTRES + 1);
const EQNTAB: i32 = (EQSCHK + 1);
const EQNCNS: i32 = (EQNTAB + 1);
const EQMXML: i32 = -1;
const EQNCNJ: i32 = (EQNCNS + 1);
const EQNORD: i32 = (EQNCNJ + 1);
const EQNSEL: i32 = (EQNORD + 1);
const EQNSIZ: i32 = (EQNSEL + 1);
const EQNPTR: i32 = (EQNSIZ + 1);
const EQCSIZ: i32 = (EQNPTR + 1);
const EQCPTR: i32 = (EQCSIZ + 1);
const EQBSEL: i32 = (EQCPTR + 1);
const EQBCON: i32 = (EQBSEL + 1);
const EQBCNJ: i32 = (EQBCON + 1);
const EQBORD: i32 = (EQBCON + 1);
const EQVBAS: i32 = EQBORD;
const EQDTYP: i32 = 1;
const EQBLEX: i32 = (EQDTYP + 1);
const EQELEX: i32 = (EQBLEX + 1);
const EQBSTR: i32 = (EQELEX + 1);
const EQESTR: i32 = (EQBSTR + 1);
const EQVPTR: i32 = (EQELEX + 1);
const EQVDSZ: i32 = 6;
const EQBCOL: i32 = 1;
const EQCIDX: i32 = EQVDSZ;
const EQBTAB: i32 = 1;
const EQTORD: i32 = EQVDSZ;
const EQCTYP: i32 = 1;
const EQCOL: i32 = 1;
const EQVAL: i32 = 2;
const EQLTAB: i32 = (EQCTYP + 1);
const EQLCOL: i32 = (EQLTAB + EQVDSZ);
const EQOPCD: i32 = (EQLCOL + EQVDSZ);
const EQRTAB: i32 = (EQOPCD + 1);
const EQRCOL: i32 = (EQRTAB + EQVDSZ);
const EQBVAL: i32 = (EQOPCD + 1);
const EQCDSZ: i32 = (2 + (4 * EQVDSZ));
const EQOTAB: i32 = 1;
const EQOCOL: i32 = (EQOTAB + EQVDSZ);
const EQODIR: i32 = (EQOCOL + EQVDSZ);
const EQODSZ: i32 = (1 + (2 * EQVDSZ));
const EQASND: i32 = 0;
const EQDSND: i32 = 1;
const EQSTAB: i32 = 1;
const EQSCOL: i32 = (EQSTAB + EQVDSZ);
const EQSDSZ: i32 = (2 * EQVDSZ);
const EQIMIN: i32 =
    (((((EQVBAS + ((10 * EQVDSZ) * 2)) + (1000 * EQCDSZ)) + 1000) + (10 * EQODSZ)) + (50 * EQSDSZ));
const SDSCSZ: i32 = 24;
const EKTIDX: i32 = 1;
const SNOIDX: i32 = (EKTIDX + 1);
const IMDIDX: i32 = (SNOIDX + 1);
const TNMIDX: i32 = (IMDIDX + 1);
const NCIDX: i32 = (TNMIDX + 1);
const NRIDX: i32 = (NCIDX + 1);
const RTIDX: i32 = (NRIDX + 1);
const CPTIDX: i32 = (RTIDX + 1);
const DPTIDX: i32 = (CPTIDX + 1);
const IPTIDX: i32 = (DPTIDX + 1);
const MFLIDX: i32 = (IPTIDX + 1);
const IFLIDX: i32 = (MFLIDX + 1);
const SHDIDX: i32 = (IFLIDX + 1);
const CFHIDX: i32 = (SHDIDX + 1);
const CSNIDX: i32 = (CFHIDX + 1);
const LCPIDX: i32 = (CSNIDX + 1);
const LDPIDX: i32 = (LCPIDX + 1);
const LIPIDX: i32 = (LDPIDX + 1);
const LCWIDX: i32 = (LIPIDX + 1);
const LDWIDX: i32 = (LCWIDX + 1);
const LIWIDX: i32 = (LDWIDX + 1);
const NMLIDX: i32 = (LIWIDX + 1);
const CHR: i32 = 1;
const DP: i32 = 2;
const INT: i32 = 3;
const TIME: i32 = 4;
const LBPOOL: i32 = -5;
const INISUB: i32 = 32;
const LIMIT1: i32 = 250000;

struct SaveVars {
    CDAT: ActualCharArray,
    NF: ActualCharArray,
    DDAT: ActualArray<f64>,
    IDAT: ActualArray<i32>,
    ORDVEC: ActualArray<i32>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut CDAT = ActualCharArray::new(INISUB, 1..=LIMIT1);
        let mut NF = ActualCharArray::new(1, 1..=LIMIT1);
        let mut DDAT = ActualArray::<f64>::new(1..=LIMIT1);
        let mut IDAT = ActualArray::<i32>::new(1..=LIMIT1);
        let mut ORDVEC = ActualArray::<i32>::new(1..=LIMIT1);

        Self {
            CDAT,
            NF,
            DDAT,
            IDAT,
            ORDVEC,
        }
    }
}

fn INTEQ(N1: bool, N2: bool, I1: i32, I2: i32) -> bool {
    ((N1 && N2) || (!(N1 || N2) && (I1 == I2)))
}

fn DPEQ(N1: bool, N2: bool, D1: f64, D2: f64) -> bool {
    ((N1 && N2) || (!(N1 || N2) && (D1 == D2)))
}

fn CHREQ(N1: bool, N2: bool, C1: &[u8], C2: &[u8]) -> bool {
    let C1 = &C1[..INISUB as usize];
    let C2 = &C2[..INISUB as usize];
    ((N1 && N2) || (!(N1 || N2) && fstr::eq(C1, C2)))
}

fn INTGE(N1: bool, N2: bool, I1: i32, I2: i32) -> bool {
    (N2 || (!(N1 || N2) && (I1 >= I2)))
}

fn INTLE(N1: bool, N2: bool, I1: i32, I2: i32) -> bool {
    (N1 || (!(N1 || N2) && (I1 <= I2)))
}

fn DPGE(N1: bool, N2: bool, D1: f64, D2: f64) -> bool {
    (N2 || (!(N1 || N2) && (D1 >= D2)))
}

fn DPLE(N1: bool, N2: bool, D1: f64, D2: f64) -> bool {
    (N1 || (!(N1 || N2) && (D1 <= D2)))
}

fn CHRGE(N1: bool, N2: bool, C1: &[u8], C2: &[u8]) -> bool {
    let C1 = &C1[..INISUB as usize];
    let C2 = &C2[..INISUB as usize];
    (N2 || (!(N1 || N2) && fstr::ge(C1, C2)))
}

fn CHRLE(N1: bool, N2: bool, C1: &[u8], C2: &[u8]) -> bool {
    let C1 = &C1[..INISUB as usize];
    let C2 = &C2[..INISUB as usize];
    (N1 || (!(N1 || N2) && fstr::le(C1, C2)))
}

//$Procedure      ZZEKJSRT ( EK, join row set union sort )
pub fn ZZEKJSRT(
    NJRS: i32,
    UBASES: &[i32],
    NORDER: i32,
    OTABS: &[i32],
    OCOLS: &[i32],
    OELTS: &[i32],
    SENSES: &[i32],
    STHAN: &[i32],
    STSDSC: &[i32],
    STDTPT: &[i32],
    DTPOOL: &[i32],
    DTDSCS: &[i32],
    ORDBAS: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let UBASES = DummyArray::new(UBASES, 1..);
    let OTABS = DummyArray::new(OTABS, 1..);
    let OCOLS = DummyArray::new(OCOLS, 1..);
    let OELTS = DummyArray::new(OELTS, 1..);
    let SENSES = DummyArray::new(SENSES, 1..);
    let STHAN = DummyArray::new(STHAN, 1..);
    let STSDSC = DummyArray2D::new(STSDSC, 1..=SDSCSZ, 1..);
    let STDTPT = DummyArray::new(STDTPT, 1..);
    let DTPOOL = DummyArray2D::new(DTPOOL, 1..=2, LBPOOL..);
    let DTDSCS = DummyArray2D::new(DTDSCS, 1..=CDSCSZ, 1..);
    let mut ADDRJ: i32 = 0;
    let mut ADDRJG: i32 = 0;
    let mut CPRIME: i32 = 0;
    let mut COLPTR: i32 = 0;
    let mut CVLEN: i32 = 0;
    let mut DTYPE: i32 = 0;
    let mut ELTIDX: i32 = 0;
    let mut GAP: i32 = 0;
    let mut HANDLE: i32 = 0;
    let mut J: i32 = 0;
    let mut JG: i32 = 0;
    let mut NR: i32 = 0;
    let mut NRLOC: i32 = 0;
    let mut NROWS: i32 = 0;
    let mut NTAB: i32 = 0;
    let mut PRVBAS: i32 = 0;
    let mut ROW: i32 = 0;
    let mut RJ: i32 = 0;
    let mut RJG: i32 = 0;
    let mut ROWVEC = StackArray::<i32, 11>::new(1..=(MAXTAB + 1));
    let mut RVECJ = StackArray::<i32, 11>::new(1..=(MAXTAB + 1));
    let mut RVECJG = StackArray::<i32, 11>::new(1..=(MAXTAB + 1));
    let mut RVSIZE: i32 = 0;
    let mut RWVBAS: i32 = 0;
    let mut SEG: i32 = 0;
    let mut SEGVEC = StackArray::<i32, 10>::new(1..=MAXTAB);
    let mut SGVBAS: i32 = 0;
    let mut SVECJ = StackArray::<i32, 10>::new(1..=MAXTAB);
    let mut SVECJG = StackArray::<i32, 10>::new(1..=MAXTAB);
    let mut SVSIZE: i32 = 0;
    let mut TABLOC: i32 = 0;
    let mut TPRIME: i32 = 0;
    let mut BRUTE: bool = false;
    let mut FOUND: bool = false;
    let mut JLE: bool = false;
    let mut NFJ: bool = false;
    let mut NFJG: bool = false;
    let mut NULL: bool = false;
    let mut TRUNC: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Other functions
    //

    //
    // Other local parameters
    //

    //
    // Local variables
    //

    //
    // Saved variables
    //
    // The following variables are saved in order to prevent
    // memory errors under Cygwin and in shared object libraries
    // under various Unix systems.
    //

    //
    // Statement functions
    //

    //
    //
    // The following functions test whether two column entries
    // are equal.  In the integer and d.p. cases, the test is conclusive.
    // In the character case, the test indicates whether the initial
    // substrings consisting of the first INISUB characters of each of
    // the two entries are equal.
    //

    //
    // The following functions indicate whether the first of two column
    // entries is less than or equal to the second.  In the integer and
    // d.p. cases, the test is conclusive.  In the character case, the
    // test indicates whether the initial substring consisting of the
    // first INISUB characters of the first entry is less than or equal
    // to the corresponding initial substring of length INISUB of the
    // second entry.
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"ZZEKJSRT", ctx)?;
    }

    //
    // If there are no order-by columns, that's an error.
    //
    if (NORDER < 1) {
        SETMSG(
            b"Number of order-by columns must be positive but was #.",
            ctx,
        );
        ERRINT(b"#", NORDER, ctx);
        SIGERR(b"SPICE(INVALIDCOUNT)", ctx)?;
        CHKOUT(b"ZZEKJSRT", ctx)?;
        return Ok(());
    }

    //
    // We split the sorting job up into two cases:
    //
    //    1)  If the number of rows to be sorted is not too large,
    //        we can gain speed by reading data from the primary
    //        order-by column into memory and sorting the row number
    //        array in memory.
    //
    //    2)  If there's too much data for option (1) to handle,
    //        we just read data from the order-by columns as needed.
    //        This algorithm is simple, but very slow, since many
    //        DAS reads of individual column entries are required.
    //
    //
    // Find out how many rows are in the join row set union.
    //
    NROWS = 0;

    for I in 1..=NJRS {
        NRLOC = (UBASES[I] + JRCIDX);

        ZZEKSRD(NRLOC, NRLOC, std::slice::from_mut(&mut NR), ctx)?;

        NROWS = (NROWS + NR);
    }

    //
    // Get the number of tables in the cartesian product represented
    // by the join row set union.  The number of tables in the first
    // join row set suffices.
    //
    TABLOC = (UBASES[1] + JTCIDX);

    ZZEKSRD(TABLOC, TABLOC, std::slice::from_mut(&mut NTAB), ctx)?;

    SVSIZE = NTAB;
    RVSIZE = (NTAB + 1);

    //
    // We can get the data types of the order-by columns from the
    // segment vector of the first row vector in the first join row set.
    // Initialize addressing in the join row set union so we can look up
    // the locations of these vectors.
    //
    ZZEKVSET(NJRS, UBASES.as_slice(), ctx)?;
    ZZEKVCAL(1, &mut RWVBAS, &mut SGVBAS, ctx)?;
    ZZEKSRD((SGVBAS + 1), (SGVBAS + SVSIZE), SEGVEC.as_slice_mut(), ctx)?;

    TPRIME = OTABS[1];
    CPRIME = OCOLS[1];
    SEG = SEGVEC[TPRIME];
    COLPTR = STDTPT[SEG];

    for I in 2..=CPRIME {
        COLPTR = LNKNXT(COLPTR, DTPOOL.as_slice(), ctx)?;
    }

    DTYPE = DTDSCS[[TYPIDX, COLPTR]];

    if (NROWS <= LIMIT1) {
        //
        // Case 1.
        //
        // We have a small enough quantity of data that we may be able
        // to speed up sorting by using memory.  Here's the plan:
        //
        // We'll read data for the primary order-by column into memory.
        // The `primary' column is the one whose index appears first
        // in the input list of column indices.  We'll also maintain a
        // null flag array for the primary column.  If we can figure out
        // the order relation between two rows by looking at entries in
        // the primary order-by column, fine.  Otherwise, we let ZZEKVCMP
        // perform the comparison.
        //
        // We'll sort the set of row vector numbers of the matching rows
        // in parallel with our data sort.
        //
        // Character columns present a special case:  their string length
        // can get pretty big, and it could take a lot of memory to store
        // their column entries.  We compromise here:  we store only the
        // first INISUB characters of each character column entry.  If
        // we can't decide the order of two strings based on these initial
        // substrings, we let ZZEKVCMP handle the matter.
        //
        // Read the primary column data.  Keep track of whether we've
        // truncated any strings.
        //
        TRUNC = false;
        PRVBAS = -1;

        for I in 1..=NROWS {
            ZZEKVCAL(I, &mut RWVBAS, &mut SGVBAS, ctx)?;

            if ((I == 1) || (SGVBAS != PRVBAS)) {
                ZZEKSRD((SGVBAS + 1), (SGVBAS + SVSIZE), SEGVEC.as_slice_mut(), ctx)?;

                SEG = SEGVEC[TPRIME];
                HANDLE = STHAN[SEG];
                COLPTR = STDTPT[SEG];

                {
                    let m1__: i32 = 2;
                    let m2__: i32 = CPRIME;
                    let m3__: i32 = 1;
                    J = m1__;
                    for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                        COLPTR = LNKNXT(COLPTR, DTPOOL.as_slice(), ctx)?;
                        J += m3__;
                    }
                }
            }

            ZZEKSRD((RWVBAS + 1), (RWVBAS + RVSIZE), ROWVEC.as_slice_mut(), ctx)?;

            ROW = ROWVEC[TPRIME];
            ELTIDX = OELTS[CPRIME];

            if (DTYPE == CHR) {
                ZZEKRSC(
                    HANDLE,
                    STSDSC.subarray([1, SEG]),
                    DTDSCS.subarray([1, COLPTR]),
                    ROW,
                    ELTIDX,
                    &mut CVLEN,
                    &mut save.CDAT[I],
                    &mut NULL,
                    &mut FOUND,
                    ctx,
                )?;

                if !FOUND {
                    SETMSG(b"EK = #; SEG = #; ROW = #; COLIDX = #; ELT = #; column entry elt was not found.", ctx);
                    ERRHAN(b"#", HANDLE, ctx)?;
                    ERRINT(b"#", SEG, ctx);
                    ERRINT(b"#", ROW, ctx);
                    ERRINT(b"#", DTDSCS[[ORDIDX, COLPTR]], ctx);
                    ERRINT(b"#", ELTIDX, ctx);
                    SIGERR(b"SPICE(BUG)", ctx)?;
                    CHKOUT(b"ZZEKJSRT", ctx)?;
                    return Ok(());
                }

                TRUNC = (TRUNC || (CVLEN > INISUB));
            } else if ((DTYPE == DP) || (DTYPE == TIME)) {
                ZZEKRSD(
                    HANDLE,
                    STSDSC.subarray([1, SEG]),
                    DTDSCS.subarray([1, COLPTR]),
                    ROW,
                    ELTIDX,
                    &mut save.DDAT[I],
                    &mut NULL,
                    &mut FOUND,
                    ctx,
                )?;

                if !FOUND {
                    SETMSG(b"EK = #; SEG = #; ROW = #; COLIDX = #; ELT = #; column entry elt was not found.", ctx);
                    ERRHAN(b"#", HANDLE, ctx)?;
                    ERRINT(b"#", SEG, ctx);
                    ERRINT(b"#", ROW, ctx);
                    ERRINT(b"#", DTDSCS[[ORDIDX, COLPTR]], ctx);
                    ERRINT(b"#", ELTIDX, ctx);
                    SIGERR(b"SPICE(BUG)", ctx)?;
                    CHKOUT(b"ZZEKJSRT", ctx)?;
                    return Ok(());
                }
            } else if (DTYPE == INT) {
                ZZEKRSI(
                    HANDLE,
                    STSDSC.subarray([1, SEG]),
                    DTDSCS.subarray([1, COLPTR]),
                    ROW,
                    ELTIDX,
                    &mut save.IDAT[I],
                    &mut NULL,
                    &mut FOUND,
                    ctx,
                )?;

                if !FOUND {
                    SETMSG(b"EK = #; SEG = #; ROW = #; COLIDX = #; ELT = #; column entry elt was not found.", ctx);
                    ERRHAN(b"#", HANDLE, ctx)?;
                    ERRINT(b"#", SEG, ctx);
                    ERRINT(b"#", ROW, ctx);
                    ERRINT(b"#", DTDSCS[[ORDIDX, COLPTR]], ctx);
                    ERRINT(b"#", ELTIDX, ctx);
                    SIGERR(b"SPICE(BUG)", ctx)?;
                    CHKOUT(b"ZZEKJSRT", ctx)?;
                    return Ok(());
                }
            } else {
                //
                // We must have a bogus column descriptor.
                //
                SETMSG(b"Unrecognized data type # for first column.", ctx);
                ERRINT(b"#", DTYPE, ctx);
                SIGERR(b"SPICE(INVALIDTYPE)", ctx)?;
                CHKOUT(b"ZZEKJSRT", ctx)?;
                return Ok(());
            }

            //
            // Set the character null flag for the current column entry.
            //
            if NULL {
                fstr::assign(save.NF.get_mut(I), CTRUE);
            } else {
                fstr::assign(save.NF.get_mut(I), CFALSE);
            }

            PRVBAS = SGVBAS;
        }

        //
        // Initialize the order vector.
        //
        for I in 1..=NROWS {
            save.ORDVEC[I] = I;
        }

        //
        // At this point, we've read in the data for the primary order-by
        // column, and also have set the null flag array for the column.
        // We're ready to proceed with our sort.
        //
        GAP = (NROWS / 2);

        while (GAP > 0) {
            for I in (GAP + 1)..=NROWS {
                J = (I - GAP);

                while (J > 0) {
                    JG = (J + GAP);
                    //
                    // Compare the Jth and JGth rows of the row set.  The
                    // logical JLE is TRUE when the Jth element is less than
                    // or equal to the JGth.  If the Jth and JGth elements
                    // compare equal, and there is more than one order-by
                    // column or if we've truncated string data, we'll have
                    // to go on and make a conclusive test.  Otherwise, we
                    // can set JLE based on the data we've read.
                    //
                    // Set the data array indices of the Jth and JGth
                    // elements, as indicated by the order vector.
                    //
                    RJ = save.ORDVEC[J];
                    RJG = save.ORDVEC[JG];

                    NFJ = fstr::eq(save.NF.get(RJ), CTRUE);
                    NFJG = fstr::eq(save.NF.get(RJG), CTRUE);

                    //
                    // Start out hoping for the best:  that we won't have
                    // to do a brute-force comparison.
                    //
                    BRUTE = false;

                    if (DTYPE == INT) {
                        if (NORDER == 1) {
                            //
                            // We can make a decision based on the data in
                            // memory.
                            //
                            if (SENSES[1] == EQASND) {
                                JLE = INTLE(NFJ, NFJG, save.IDAT[RJ], save.IDAT[RJG]);
                            } else {
                                JLE = INTGE(NFJ, NFJG, save.IDAT[RJ], save.IDAT[RJG]);
                            }
                        } else if !INTEQ(NFJ, NFJG, save.IDAT[RJ], save.IDAT[RJG]) {
                            //
                            // If the items we're comparing are unequal, we can
                            // still make a decision.
                            //
                            if (SENSES[1] == EQASND) {
                                JLE = INTLE(NFJ, NFJG, save.IDAT[RJ], save.IDAT[RJG]);
                            } else {
                                JLE = INTGE(NFJ, NFJG, save.IDAT[RJ], save.IDAT[RJG]);
                            }
                        } else {
                            //
                            // Otherwise, we'll have to look at values in the
                            // other order-by columns.  Get the segment and
                            // row vectors to be compared.
                            //
                            BRUTE = true;
                        }
                    } else if ((DTYPE == DP) || (DTYPE == TIME)) {
                        //
                        // The D.P. case parallels the integer case.
                        //
                        if (NORDER == 1) {
                            if (SENSES[1] == EQASND) {
                                JLE = DPLE(NFJ, NFJG, save.DDAT[RJ], save.DDAT[RJG]);
                            } else {
                                JLE = DPGE(NFJ, NFJG, save.DDAT[RJ], save.DDAT[RJG]);
                            }
                        } else if !DPEQ(NFJ, NFJG, save.DDAT[RJ], save.DDAT[RJG]) {
                            if (SENSES[1] == EQASND) {
                                JLE = DPLE(NFJ, NFJG, save.DDAT[RJ], save.DDAT[RJG]);
                            } else {
                                JLE = DPGE(NFJ, NFJG, save.DDAT[RJ], save.DDAT[RJG]);
                            }
                        } else {
                            //
                            // Otherwise, we'll have to look at values in the
                            // other order-by columns.  Get the segment and
                            // row vectors to be compared.
                            //
                            BRUTE = true;
                        }
                    } else {
                        //
                        // In the character case where there is one order-by
                        // column, equality is a problem unless no truncation
                        // occurred.
                        //
                        if ((NORDER == 1) && !TRUNC) {
                            if (SENSES[1] == EQASND) {
                                JLE = CHRLE(NFJ, NFJG, &save.CDAT[RJ], &save.CDAT[RJG]);
                            } else {
                                JLE = CHRGE(NFJ, NFJG, &save.CDAT[RJ], &save.CDAT[RJG]);
                            }
                        } else if !CHREQ(NFJ, NFJG, &save.CDAT[RJ], &save.CDAT[RJG]) {
                            //
                            // If the items we're comparing are unequal, we can
                            // still make a decision.
                            //
                            if (SENSES[1] == EQASND) {
                                JLE = CHRLE(NFJ, NFJG, &save.CDAT[RJ], &save.CDAT[RJG]);
                            } else {
                                JLE = CHRGE(NFJ, NFJG, &save.CDAT[RJ], &save.CDAT[RJG]);
                            }
                        } else {
                            //
                            // Otherwise, we'll have to look at values in the
                            // other order-by columns.  Get the segment and
                            // row vectors to be compared.
                            //
                            BRUTE = true;
                        }
                    }

                    if BRUTE {
                        ZZEKVCAL(RJ, &mut RWVBAS, &mut SGVBAS, ctx)?;
                        ZZEKSRD((SGVBAS + 1), (SGVBAS + SVSIZE), SVECJ.as_slice_mut(), ctx)?;
                        ZZEKSRD((RWVBAS + 1), (RWVBAS + RVSIZE), RVECJ.as_slice_mut(), ctx)?;

                        ZZEKVCAL(RJG, &mut RWVBAS, &mut SGVBAS, ctx)?;
                        ZZEKSRD((SGVBAS + 1), (SGVBAS + SVSIZE), SVECJG.as_slice_mut(), ctx)?;
                        ZZEKSRD((RWVBAS + 1), (RWVBAS + RVSIZE), RVECJG.as_slice_mut(), ctx)?;

                        JLE = ZZEKVCMP(
                            LE,
                            NORDER,
                            OTABS.as_slice(),
                            OCOLS.as_slice(),
                            OELTS.as_slice(),
                            SENSES.as_slice(),
                            STHAN.as_slice(),
                            STSDSC.as_slice(),
                            STDTPT.as_slice(),
                            DTPOOL.as_slice(),
                            DTDSCS.as_slice(),
                            SVECJ.as_slice(),
                            RVECJ.as_slice(),
                            SVECJG.as_slice(),
                            RVECJG.as_slice(),
                            ctx,
                        )?;
                    }

                    //
                    // At this point, JLE is set.
                    //
                    if JLE {
                        J = 0;
                    } else {
                        //
                        // Swap the Jth and JGth elements of the order vector.
                        //
                        SWAPI_ARRAY(
                            save.ORDVEC.subscript(J),
                            save.ORDVEC.subscript(JG),
                            save.ORDVEC.as_slice_mut(),
                        );
                    }

                    J = (J - GAP);
                }
            }

            //
            // The following division guarantees loop termination, even
            // if a DAS error occurs.
            //
            GAP = (GAP / 2);
        }

        //
        // We've sorted the row numbers in Case 1.  Push the order vector
        // onto the scratch area stack.
        //
        ZZEKSTOP(ORDBAS, ctx);

        ZZEKSPSH(NROWS, save.ORDVEC.as_slice(), ctx)?;
    } else {
        //
        // Case 2.
        //
        // Well, we really have a lot of data.  Don't try to read it into
        // memory.  Build the order vector in the scratch area.
        //
        ZZEKSTOP(ORDBAS, ctx);

        for I in 1..=NROWS {
            ZZEKSPSH(1, &[I], ctx)?;
        }

        //
        // Re-order the order vector elements to reflect the order of the
        // corresponding rows. This uses the Shell Sort algorithm, but
        // swaps the elements of the order vector instead of the rows
        // themselves.
        //
        GAP = (NROWS / 2);

        while (GAP > 0) {
            for I in (GAP + 1)..=NROWS {
                J = (I - GAP);

                while (J > 0) {
                    JG = (J + GAP);
                    //
                    // Set the indices of the Jth and JGth
                    // row vectors, as indicated by the order vector.
                    //
                    ZZEKSRD(
                        (*ORDBAS + J),
                        (*ORDBAS + J),
                        std::slice::from_mut(&mut RJ),
                        ctx,
                    )?;
                    ZZEKSRD(
                        (*ORDBAS + JG),
                        (*ORDBAS + JG),
                        std::slice::from_mut(&mut RJG),
                        ctx,
                    )?;

                    //
                    // Compare the two row vectors.
                    //
                    ZZEKVCAL(RJ, &mut RWVBAS, &mut SGVBAS, ctx)?;
                    ZZEKSRD((SGVBAS + 1), (SGVBAS + SVSIZE), SVECJ.as_slice_mut(), ctx)?;
                    ZZEKSRD((RWVBAS + 1), (RWVBAS + RVSIZE), RVECJ.as_slice_mut(), ctx)?;

                    ZZEKVCAL(RJG, &mut RWVBAS, &mut SGVBAS, ctx)?;
                    ZZEKSRD((SGVBAS + 1), (SGVBAS + SVSIZE), SVECJG.as_slice_mut(), ctx)?;
                    ZZEKSRD((RWVBAS + 1), (RWVBAS + RVSIZE), RVECJG.as_slice_mut(), ctx)?;

                    if ZZEKVCMP(
                        LE,
                        NORDER,
                        OTABS.as_slice(),
                        OCOLS.as_slice(),
                        OELTS.as_slice(),
                        SENSES.as_slice(),
                        STHAN.as_slice(),
                        STSDSC.as_slice(),
                        STDTPT.as_slice(),
                        DTPOOL.as_slice(),
                        DTDSCS.as_slice(),
                        SVECJ.as_slice(),
                        RVECJ.as_slice(),
                        SVECJG.as_slice(),
                        RVECJG.as_slice(),
                        ctx,
                    )? {
                        J = 0;
                    } else {
                        //
                        // Swap the order vector's Jth and JGth elements.
                        //
                        ADDRJ = (*ORDBAS + J);
                        ADDRJG = (*ORDBAS + JG);

                        ZZEKSUPD(ADDRJ, ADDRJ, &[RJG], ctx)?;
                        ZZEKSUPD(ADDRJG, ADDRJG, &[RJ], ctx)?;
                    }

                    J = (J - GAP);
                }
            }

            //
            // The following division guarantees loop termination, even
            // if a DAS error occurs.
            //
            GAP = (GAP / 2);
        }
        //
        // We've sorted the row numbers for case (2).
        //
    }
    //
    // We've sorted the row numbers, no matter how many there were.
    //

    CHKOUT(b"ZZEKJSRT", ctx)?;
    Ok(())
}
