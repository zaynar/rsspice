//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

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
const LBPOOL: i32 = -5;
const EQUI: i32 = 1;
const NONEQ: i32 = 2;
const NOLUCK: i32 = 3;
const EMPTY: i32 = 4;

struct SaveVars {
    ADDRSS: i32,
    BASE: i32,
    CASE: i32,
    CNSTR: i32,
    DTPTR: i32,
    J: i32,
    JBASE: i32,
    K: i32,
    LBASE: i32,
    LCOL: i32,
    LCUR: i32,
    LDSCRS: ActualArray2D<i32>,
    LELT: i32,
    LELTS: StackArray<i32, 100>,
    LHANS: StackArray<i32, 100>,
    LOCACT: StackArray<bool, 100>,
    LOVBAS: i32,
    LPTR: i32,
    LROW: i32,
    LROWS: StackArray<i32, 100>,
    LRVIDX: i32,
    LSDSC: ActualArray2D<i32>,
    LSEG: i32,
    LTAB: i32,
    MINIRV: StackArray<i32, 2>,
    OFFSET: i32,
    NR: i32,
    NT: i32,
    NT3: i32,
    RB: i32,
    RBASE: i32,
    RCOL: i32,
    RDSCRS: ActualArray2D<i32>,
    RELT: i32,
    RELTS: StackArray<i32, 100>,
    RHANS: StackArray<i32, 100>,
    ROVBAS: i32,
    RPTR: i32,
    RROW: i32,
    RROWS: StackArray<i32, 100>,
    RRVIDX: i32,
    RSDSC: ActualArray2D<i32>,
    RSEG: i32,
    RTAB: i32,
    SVBAS1: i32,
    SVBAS2: i32,
    SVCP1: StackArray<i32, 100>,
    SVCP2: StackArray<i32, 100>,
    SVOPS: StackArray<i32, 100>,
    SVNCON: i32,
    SVNR1: i32,
    SVNR2: i32,
    SVNT1: i32,
    SVNT2: i32,
    SVRB1: i32,
    SVRB2: i32,
    TAB: i32,
    TOP: i32,
    DONE: bool,
    FND: bool,
    LSMALL: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ADDRSS: i32 = 0;
        let mut BASE: i32 = 0;
        let mut CASE: i32 = 0;
        let mut CNSTR: i32 = 0;
        let mut DTPTR: i32 = 0;
        let mut J: i32 = 0;
        let mut JBASE: i32 = 0;
        let mut K: i32 = 0;
        let mut LBASE: i32 = 0;
        let mut LCOL: i32 = 0;
        let mut LCUR: i32 = 0;
        let mut LDSCRS = ActualArray2D::<i32>::new(1..=CDSCSZ, 1..=MXJCON);
        let mut LELT: i32 = 0;
        let mut LELTS = StackArray::<i32, 100>::new(1..=MXJCON);
        let mut LHANS = StackArray::<i32, 100>::new(1..=MXJCON);
        let mut LOCACT = StackArray::<bool, 100>::new(1..=MXJCON);
        let mut LOVBAS: i32 = 0;
        let mut LPTR: i32 = 0;
        let mut LROW: i32 = 0;
        let mut LROWS = StackArray::<i32, 100>::new(1..=MXJCON);
        let mut LRVIDX: i32 = 0;
        let mut LSDSC = ActualArray2D::<i32>::new(1..=SDSCSZ, 1..=MXJCON);
        let mut LSEG: i32 = 0;
        let mut LTAB: i32 = 0;
        let mut MINIRV = StackArray::<i32, 2>::new(1..=2);
        let mut OFFSET: i32 = 0;
        let mut NR: i32 = 0;
        let mut NT: i32 = 0;
        let mut NT3: i32 = 0;
        let mut RB: i32 = 0;
        let mut RBASE: i32 = 0;
        let mut RCOL: i32 = 0;
        let mut RDSCRS = ActualArray2D::<i32>::new(1..=CDSCSZ, 1..=MXJCON);
        let mut RELT: i32 = 0;
        let mut RELTS = StackArray::<i32, 100>::new(1..=MXJCON);
        let mut RHANS = StackArray::<i32, 100>::new(1..=MXJCON);
        let mut ROVBAS: i32 = 0;
        let mut RPTR: i32 = 0;
        let mut RROW: i32 = 0;
        let mut RROWS = StackArray::<i32, 100>::new(1..=MXJCON);
        let mut RRVIDX: i32 = 0;
        let mut RSDSC = ActualArray2D::<i32>::new(1..=SDSCSZ, 1..=MXJCON);
        let mut RSEG: i32 = 0;
        let mut RTAB: i32 = 0;
        let mut SVBAS1: i32 = 0;
        let mut SVBAS2: i32 = 0;
        let mut SVCP1 = StackArray::<i32, 100>::new(1..=MXJCON);
        let mut SVCP2 = StackArray::<i32, 100>::new(1..=MXJCON);
        let mut SVOPS = StackArray::<i32, 100>::new(1..=MXJCON);
        let mut SVNCON: i32 = 0;
        let mut SVNR1: i32 = 0;
        let mut SVNR2: i32 = 0;
        let mut SVNT1: i32 = 0;
        let mut SVNT2: i32 = 0;
        let mut SVRB1: i32 = 0;
        let mut SVRB2: i32 = 0;
        let mut TAB: i32 = 0;
        let mut TOP: i32 = 0;
        let mut DONE: bool = false;
        let mut FND: bool = false;
        let mut LSMALL: bool = false;

        Self {
            ADDRSS,
            BASE,
            CASE,
            CNSTR,
            DTPTR,
            J,
            JBASE,
            K,
            LBASE,
            LCOL,
            LCUR,
            LDSCRS,
            LELT,
            LELTS,
            LHANS,
            LOCACT,
            LOVBAS,
            LPTR,
            LROW,
            LROWS,
            LRVIDX,
            LSDSC,
            LSEG,
            LTAB,
            MINIRV,
            OFFSET,
            NR,
            NT,
            NT3,
            RB,
            RBASE,
            RCOL,
            RDSCRS,
            RELT,
            RELTS,
            RHANS,
            ROVBAS,
            RPTR,
            RROW,
            RROWS,
            RRVIDX,
            RSDSC,
            RSEG,
            RTAB,
            SVBAS1,
            SVBAS2,
            SVCP1,
            SVCP2,
            SVOPS,
            SVNCON,
            SVNR1,
            SVNR2,
            SVNT1,
            SVNT2,
            SVRB1,
            SVRB2,
            TAB,
            TOP,
            DONE,
            FND,
            LSMALL,
        }
    }
}

//$Procedure  ZZEKJTST  ( Test join candidates )
pub fn ZZEKJTST(
    SEGVEC: &[i32],
    JBASE1: i32,
    NT1: i32,
    RB1: i32,
    NR1: i32,
    JBASE2: i32,
    NT2: i32,
    RB2: i32,
    NR2: i32,
    NJCNST: i32,
    ACTIVE: &[bool],
    CPIDX1: &[i32],
    CLIDX1: &[i32],
    ELTS1: &[i32],
    OPS: &[i32],
    CPIDX2: &[i32],
    CLIDX2: &[i32],
    ELTS2: &[i32],
    STHAN: &[i32],
    STSDSC: &[i32],
    STDTPT: &[i32],
    DTPOOL: &[i32],
    DTDSCS: &[i32],
    FOUND: bool,
    ROWVEC: &[i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    //
    // SPICELIB functions
    //

    //
    // Other functions
    //

    //
    // Local parameters
    //

    //
    // Local variables
    //

    //
    // Saved variables
    //

    CHKIN(b"ZZEKJTST", ctx)?;
    SIGERR(b"SPICE(BOGUSENTRY)", ctx)?;
    CHKOUT(b"ZZEKJTST", ctx)?;

    Ok(())
}

//$Procedure  ZZEKJPRP  ( Prepare join condition test )
pub fn ZZEKJPRP(
    SEGVEC: &[i32],
    JBASE1: i32,
    NT1: i32,
    RB1: i32,
    NR1: i32,
    JBASE2: i32,
    NT2: i32,
    RB2: i32,
    NR2: i32,
    NJCNST: i32,
    ACTIVE: &[bool],
    CPIDX1: &[i32],
    CLIDX1: &[i32],
    ELTS1: &[i32],
    OPS: &[i32],
    CPIDX2: &[i32],
    CLIDX2: &[i32],
    ELTS2: &[i32],
    STHAN: &[i32],
    STSDSC: &[i32],
    STDTPT: &[i32],
    DTPOOL: &[i32],
    DTDSCS: &[i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let SEGVEC = DummyArray::new(SEGVEC, 1..);
    let ACTIVE = DummyArray::new(ACTIVE, 1..);
    let CPIDX1 = DummyArray::new(CPIDX1, 1..);
    let CLIDX1 = DummyArray::new(CLIDX1, 1..);
    let ELTS1 = DummyArray::new(ELTS1, 1..);
    let OPS = DummyArray::new(OPS, 1..);
    let CPIDX2 = DummyArray::new(CPIDX2, 1..);
    let CLIDX2 = DummyArray::new(CLIDX2, 1..);
    let ELTS2 = DummyArray::new(ELTS2, 1..);
    let STHAN = DummyArray::new(STHAN, 1..);
    let STSDSC = DummyArray2D::new(STSDSC, 1..=SDSCSZ, 1..);
    let STDTPT = DummyArray::new(STDTPT, 1..);
    let DTPOOL = DummyArray2D::new(DTPOOL, 1..=2, LBPOOL..);
    let DTDSCS = DummyArray2D::new(DTDSCS, 1..=CDSCSZ, 1..);

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZEKJPRP", ctx)?;

    //
    // We don't validate the inputs; these must be checked by ZZEKJOIN,
    // the only routine that should call this one.
    //
    // Not much preparation is required if either input row count is
    // zero, since the cartesian product will be zero.
    //
    if ((NR1 == 0) || (NR2 == 0)) {
        save.CASE = EMPTY;

        CHKOUT(b"ZZEKJPRP", ctx)?;
        return Ok(());
    }

    //
    // Set the table count and segment vector count for the output join
    // row set.
    //
    save.NT3 = (NT1 + NT2);

    //
    // Create handle, segment base, and column descriptor
    // arrays for both sides of each active relational constraint.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = NJCNST;
        let m3__: i32 = 1;
        save.J = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            if ACTIVE[save.J] {
                save.LTAB = CPIDX1[save.J];
                save.RTAB = CPIDX2[save.J];
                save.LSEG = SEGVEC[save.LTAB];
                save.RSEG = SEGVEC[save.RTAB];
                save.LHANS[save.J] = STHAN[save.LSEG];
                save.RHANS[save.J] = STHAN[save.RSEG];

                MOVEI(
                    STSDSC.subarray([1, save.LSEG]),
                    SDSCSZ,
                    save.LSDSC.subarray_mut([1, save.J]),
                );
                MOVEI(
                    STSDSC.subarray([1, save.RSEG]),
                    SDSCSZ,
                    save.RSDSC.subarray_mut([1, save.J]),
                );

                save.DTPTR = STDTPT[save.LSEG];

                {
                    let m1__: i32 = 2;
                    let m2__: i32 = CLIDX1[save.J];
                    let m3__: i32 = 1;
                    save.K = m1__;
                    for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                        save.DTPTR = LNKNXT(save.DTPTR, DTPOOL.as_slice(), ctx)?;
                        save.K += m3__;
                    }
                }

                MOVEI(
                    DTDSCS.subarray([1, save.DTPTR]),
                    CDSCSZ,
                    save.LDSCRS.subarray_mut([1, save.J]),
                );

                save.DTPTR = STDTPT[save.RSEG];

                {
                    let m1__: i32 = 2;
                    let m2__: i32 = CLIDX2[save.J];
                    let m3__: i32 = 1;
                    save.K = m1__;
                    for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                        save.DTPTR = LNKNXT(save.DTPTR, DTPOOL.as_slice(), ctx)?;
                        save.K += m3__;
                    }
                }

                MOVEI(
                    DTDSCS.subarray([1, save.DTPTR]),
                    CDSCSZ,
                    save.RDSCRS.subarray_mut([1, save.J]),
                );
            }

            save.J += m3__;
        }
    }

    //
    // Our objective is to limit as far as possible the number of
    // row vectors that have to be tested against the join constraints.
    //
    // We break the problem down into cases as follows:
    //
    //    1)  Try to find a pair of columns related by an equi-join
    //        constraint.  If such a pair is found, sort each input
    //        join row set using the appropriate column as a key.
    //        We then can fairly rapidly compare row vectors for
    //        equality in the columns to which the equi-join constraint
    //        applies, and limit the application of the remaining tests
    //        to row vectors that satisfy the first test.
    //
    //    2)  If no equi-join constraints are available, look for
    //        join constraints using the operators LE, LT, GE, or GT.
    //        Sort as in (1); then apply the rest of the constraints.
    //
    //    3)  Hard luck:  the only constraints we have (if any) involve
    //        the operators NE, LIKE, or UNLIKE, none of which are
    //        helpful.  Test every row vector.
    //
    //
    // First step:  We try to find a pair of columns related by an
    // equi-join constraint.
    //

    save.CASE = NOLUCK;

    save.J = 1;
    save.FND = false;

    while ((save.J <= NJCNST) && !save.FND) {
        if (ACTIVE[save.J] && (OPS[save.J] == EQ)) {
            //
            // Good deal, we've got an equi-join constraint.  Save the
            // index of this constraint.
            //
            save.CASE = EQUI;
            save.CNSTR = save.J;
            save.FND = true;
        } else {
            save.J = (save.J + 1);
        }
    }

    if (save.CASE == NOLUCK) {
        save.J = 1;
        save.FND = false;

        while ((save.J <= NJCNST) && !save.FND) {
            if ACTIVE[save.J] {
                if ((((OPS[save.J] == LT) || (OPS[save.J] == LE)) || (OPS[save.J] == GE))
                    || (OPS[save.J] == GT))
                {
                    //
                    // We've got a non-equi-join constraint.  Save the
                    // index of this constraint.
                    //
                    save.CASE = NONEQ;
                    save.CNSTR = save.J;
                    save.FND = true;
                }
            }

            if !save.FND {
                save.J = (save.J + 1);
            }
        }
    }

    //
    // At this point, we know which case we've got.  If we've picked
    // a distinguished constraint, produce order vectors for each
    // set of input rows vectors, using the keys defined by the
    // join constraint.
    //
    if (save.CASE != NOLUCK) {
        //
        // Produce an order vector for the column on the left side of
        // the CNSTR constraint.  We'll do this by turning the set of
        // row vectors we want to sort into a join row set.  We'll
        // create the join row set metadata and just make it point to
        // the collection of row vectors we wish to sort.  Consult the
        // join row set include file for a picture of the data structure
        // we're creating.
        //
        ZZEKSTOP(&mut save.LBASE, ctx);

        save.LTAB = CPIDX1[save.CNSTR];
        save.LCOL = CLIDX1[save.CNSTR];
        save.LELT = ELTS1[save.CNSTR];

        //
        // Set JBASE to the base address of the join row set containing
        // the table indicated by LTAB.  Set NT, NR and RB to indicate,
        // respectively, the number of tables in this join row set, the
        // number of rows in the join row set, and the base address of the
        // relevant row vector set.  If LTAB is in the second join row
        // set, we'll adjust TAB to indicate position relative to the set
        // of tables defining the second join row set.
        //
        if (save.LTAB <= NT1) {
            save.JBASE = JBASE1;
            save.NT = NT1;
            save.NR = NR1;
            save.RB = RB1;
            save.TAB = save.LTAB;
        } else {
            save.JBASE = JBASE2;
            save.NT = NT2;
            save.NR = NR2;
            save.RB = RB2;
            save.TAB = (save.LTAB - NT1);
        }

        //
        // Save the dimensions and base addresses we'll need later.
        //
        save.SVBAS1 = save.JBASE;
        save.SVNT1 = save.NT;
        save.SVRB1 = save.RB;
        save.SVNR1 = save.NR;

        ZZEKSPSH(1, &[0], ctx)?;
        ZZEKSPSH(1, &[save.NR], ctx)?;
        ZZEKSPSH(1, &[1], ctx)?;
        ZZEKSPSH(1, &[1], ctx)?;
        ZZEKSPSH(1, SEGVEC.subarray(save.LTAB), ctx)?;
        ZZEKSPSH(1, &[7], ctx)?;
        ZZEKSPSH(1, &[save.NR], ctx)?;

        for I in 1..=save.NR {
            //
            // Grab the row pointer in position TAB from the Ith row
            // vector from the join row set containing the parent table
            // of the LHS constraint column.
            //
            save.BASE = ((save.JBASE + save.RB) + ((I - 1) * (save.NT + 1)));

            ZZEKSRD(
                (save.BASE + save.TAB),
                (save.BASE + save.TAB),
                save.MINIRV.as_slice_mut(),
                ctx,
            )?;
            //
            // Fill in the segment vector pointer for the new very
            // narrow row vector.
            //
            save.MINIRV[2] = JSVBAS;
            //
            // Append to the join row set under construction.
            //
            ZZEKSPSH(2, save.MINIRV.as_slice(), ctx)?;
        }

        ZZEKSTOP(&mut save.TOP, ctx);
        ZZEKSUPD(
            (save.LBASE + JSZIDX),
            (save.LBASE + JSZIDX),
            &[(save.TOP - save.LBASE)],
            ctx,
        )?;

        ZZEKJSRT(
            1,
            &[save.LBASE],
            1,
            &[1],
            &[save.LCOL],
            &[save.LELT],
            &[EQASND],
            STHAN.as_slice(),
            STSDSC.as_slice(),
            STDTPT.as_slice(),
            DTPOOL.as_slice(),
            DTDSCS.as_slice(),
            &mut save.LOVBAS,
            ctx,
        )?;

        //
        // Produce an order vector for the column on the right side of
        // the CNSTR constraint.
        //
        ZZEKSTOP(&mut save.RBASE, ctx);

        save.RTAB = CPIDX2[save.CNSTR];
        save.RCOL = CLIDX2[save.CNSTR];
        save.RELT = ELTS2[save.CNSTR];

        //
        // Set JBASE to the base address of the join row set containing
        // the table indicated by RTAB.  Set NT, NR and RB to indicate,
        // respectively, the number of tables in this join row set, the
        // number of rows in the join row set, and the base address of the
        // relevant row vector set.  If RTAB is in the second join row
        // set, we'll adjust TAB to indicate position relative to the set
        // of tables defining the second join row set.
        //
        if (save.RTAB <= NT1) {
            save.JBASE = JBASE1;
            save.NT = NT1;
            save.NR = NR1;
            save.RB = RB1;
            save.TAB = save.RTAB;
        } else {
            save.JBASE = JBASE2;
            save.NT = NT2;
            save.NR = NR2;
            save.RB = RB2;
            save.TAB = (save.RTAB - NT1);
        }

        //
        // Save the dimensions and base addresses we'll need later.
        //
        save.SVBAS2 = save.JBASE;
        save.SVNT2 = save.NT;
        save.SVRB2 = save.RB;
        save.SVNR2 = save.NR;

        ZZEKSPSH(1, &[0], ctx)?;
        ZZEKSPSH(1, &[save.NR], ctx)?;
        ZZEKSPSH(1, &[1], ctx)?;
        ZZEKSPSH(1, &[1], ctx)?;
        ZZEKSPSH(1, SEGVEC.subarray(save.RTAB), ctx)?;
        ZZEKSPSH(1, &[7], ctx)?;
        ZZEKSPSH(1, &[save.NR], ctx)?;

        for I in 1..=save.NR {
            //
            // Grab the row pointer in position TAB from the Ith row
            // vector from the join row set containing the parent table
            // of the RHS constraint column.
            //
            save.BASE = ((save.JBASE + save.RB) + ((I - 1) * (save.NT + 1)));

            ZZEKSRD(
                (save.BASE + save.TAB),
                (save.BASE + save.TAB),
                save.MINIRV.as_slice_mut(),
                ctx,
            )?;
            //
            // Fill in the segment vector pointer for the new very
            // narrow row vector.
            //
            save.MINIRV[2] = JSVBAS;
            //
            // Append to the join row set under construction.
            //
            ZZEKSPSH(2, save.MINIRV.as_slice(), ctx)?;
        }

        ZZEKSTOP(&mut save.TOP, ctx);
        ZZEKSUPD(
            (save.RBASE + JSZIDX),
            (save.RBASE + JSZIDX),
            &[(save.TOP - save.RBASE)],
            ctx,
        )?;

        ZZEKJSRT(
            1,
            &[save.RBASE],
            1,
            &[1],
            &[save.RCOL],
            &[save.RELT],
            &[EQASND],
            STHAN.as_slice(),
            STSDSC.as_slice(),
            STDTPT.as_slice(),
            DTPOOL.as_slice(),
            DTDSCS.as_slice(),
            &mut save.ROVBAS,
            ctx,
        )?;

        //
        // Keep a local copy of the active constraint flags, deactivating
        // the distinguished one.
        //
        for I in 1..=NJCNST {
            save.LOCACT[I] = ACTIVE[I];
        }

        save.LOCACT[save.CNSTR] = false;
    } else {
        //
        // This is the `no luck' case.  Save all of the constraints.
        //
        for I in 1..=NJCNST {
            save.LOCACT[I] = ACTIVE[I];
        }

        //
        // Save the counts pertaining to the input join row sets.
        //
        save.SVNT1 = NT1;
        save.SVNT2 = NT2;
        save.SVNR1 = NR1;
        save.SVNR2 = NR2;
        save.SVRB1 = RB1;
        save.SVRB2 = RB2;
        save.SVBAS1 = JBASE1;
        save.SVBAS2 = JBASE2;
    }

    //
    // In the non-equi-join case, record whether the join constraint
    // requires the left side to be less than, or less than or equal to,
    // the right side.
    //
    if (save.CASE == NONEQ) {
        save.LSMALL = ((OPS[save.CNSTR] == LT) || (OPS[save.CNSTR] == LE));
    }

    //
    // Keep our own copy of the relational constraints, except for the
    // column indices, which are used only in this routine.
    //
    save.SVNCON = NJCNST;

    for I in 1..=save.SVNCON {
        save.SVCP1[I] = CPIDX1[I];
        save.SVOPS[I] = OPS[I];
        save.SVCP2[I] = CPIDX2[I];
    }

    //
    // Initialize the pointers we'll use to keep track of the
    // row vectors we'll be comparing.  Initialize the DONE flag
    // as well.
    //
    save.LPTR = 1;
    save.LCUR = 1;
    save.RPTR = 1;
    save.DONE = false;

    CHKOUT(b"ZZEKJPRP", ctx)?;
    Ok(())
}

//$Procedure  ZZEKJNXT  ( Return next join row vector )
pub fn ZZEKJNXT(FOUND: &mut bool, ROWVEC: &mut [i32], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut ROWVEC = DummyArrayMut::new(ROWVEC, 1..);

    //
    // No row vector found to start with.
    //
    *FOUND = false;

    //
    // The action we take depends on the join constraint situation.
    // Handle the "empty" case first.
    //
    if (save.CASE == EMPTY) {
        return Ok(());
    } else if (save.CASE == EQUI) {
        while (!save.DONE && !*FOUND) {
            //
            // At this point, LCUR and RPTR should point to the current
            // pair of order vector entries to use.  We should always have
            //
            //    1     <  LPTR  <  SVNR1
            //          -        -
            //
            //    LPTR  <  LCUR  <  SVNR1
            //          -        -
            //
            //    1     <  RPTR  <  SVNR2
            //          -        -
            //
            // here.
            //
            // Look up the next set of row vector indices.  Get the row
            // numbers in the join columns for each order vector in our
            // mini-join row sets that we created for sorting.
            //
            ZZEKSRD(
                (save.LOVBAS + save.LCUR),
                (save.LOVBAS + save.LCUR),
                std::slice::from_mut(&mut save.LRVIDX),
                ctx,
            )?;
            ZZEKSRD(
                (save.ROVBAS + save.RPTR),
                (save.ROVBAS + save.RPTR),
                std::slice::from_mut(&mut save.RRVIDX),
                ctx,
            )?;

            save.ADDRSS = (((save.LBASE + 7) + (2 * (save.LRVIDX - 1))) + 1);
            ZZEKSRD(
                save.ADDRSS,
                save.ADDRSS,
                std::slice::from_mut(&mut save.LROW),
                ctx,
            )?;

            save.ADDRSS = (((save.RBASE + 7) + (2 * (save.RRVIDX - 1))) + 1);
            ZZEKSRD(
                save.ADDRSS,
                save.ADDRSS,
                std::slice::from_mut(&mut save.RROW),
                ctx,
            )?;

            //
            // Compare column entries, and advance the pointers as
            // required.
            //
            if ZZEKRCMP(
                LT,
                1,
                save.LHANS[save.CNSTR],
                save.LSDSC.subarray([1, save.CNSTR]),
                save.LDSCRS.subarray([1, save.CNSTR]),
                save.LROW,
                &[save.LELT],
                save.RHANS[save.CNSTR],
                save.RSDSC.subarray([1, save.CNSTR]),
                save.RDSCRS.subarray([1, save.CNSTR]),
                save.RROW,
                &[save.RELT],
                ctx,
            )? {
                //
                //
                // The `left' key entry is smaller.  Advance the bottom
                // pointer on the left side.
                //
                if (save.LPTR < save.SVNR1) {
                    save.LPTR = (save.LPTR + 1);
                    save.LCUR = save.LPTR;
                } else {
                    save.DONE = true;
                }
            } else if ZZEKRCMP(
                EQ,
                1,
                save.LHANS[save.CNSTR],
                save.LSDSC.subarray([1, save.CNSTR]),
                save.LDSCRS.subarray([1, save.CNSTR]),
                save.LROW,
                &[save.LELT],
                save.RHANS[save.CNSTR],
                save.RSDSC.subarray([1, save.CNSTR]),
                save.RDSCRS.subarray([1, save.CNSTR]),
                save.RROW,
                &[save.RELT],
                ctx,
            )? {
                //
                //
                // The `left' key entry is equal.  Form a composite
                // row vector and test it against the full set of active
                // constraints.
                //
                if (save.SVCP1[save.CNSTR] <= save.SVNT1) {
                    //
                    // The parent table of the column on the LHS of our
                    // equi-join constraint belongs to the first join
                    // row set.
                    //
                    save.J = 1;
                    save.K = (save.SVNT1 + 1);
                } else {
                    save.J = (save.SVNT2 + 1);
                    save.K = 1;
                }

                save.OFFSET = (save.SVRB1 + ((save.LRVIDX - 1) * (save.SVNT1 + 1)));

                ZZEKSRD(
                    ((save.SVBAS1 + save.OFFSET) + 1),
                    ((save.SVBAS1 + save.OFFSET) + save.SVNT1),
                    ROWVEC.subarray_mut(save.J),
                    ctx,
                )?;

                save.OFFSET = (save.SVRB2 + ((save.RRVIDX - 1) * (save.SVNT2 + 1)));

                ZZEKSRD(
                    ((save.SVBAS2 + save.OFFSET) + 1),
                    ((save.SVBAS2 + save.OFFSET) + save.SVNT2),
                    ROWVEC.subarray_mut(save.K),
                    ctx,
                )?;

                //
                // Create row arrays for both sides of each active
                // relational constraint.
                //
                {
                    let m1__: i32 = 1;
                    let m2__: i32 = save.SVNCON;
                    let m3__: i32 = 1;
                    save.J = m1__;
                    for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                        if save.LOCACT[save.J] {
                            save.LTAB = save.SVCP1[save.J];
                            save.RTAB = save.SVCP2[save.J];
                            save.LROWS[save.J] = ROWVEC[save.LTAB];
                            save.RROWS[save.J] = ROWVEC[save.RTAB];
                        }

                        save.J += m3__;
                    }
                }

                *FOUND = ZZEKVMCH(
                    save.SVNCON,
                    save.LOCACT.as_slice(),
                    save.LHANS.as_slice(),
                    save.LSDSC.as_slice(),
                    save.LDSCRS.as_slice(),
                    save.LROWS.as_slice(),
                    save.LELTS.as_slice(),
                    save.SVOPS.as_slice(),
                    save.RHANS.as_slice(),
                    save.RSDSC.as_slice(),
                    save.RDSCRS.as_slice(),
                    save.RROWS.as_slice(),
                    save.RELTS.as_slice(),
                    ctx,
                )?;

                //
                // Update the pointers.
                //
                if (save.LCUR < save.SVNR1) {
                    save.LCUR = (save.LCUR + 1);
                } else if ((save.LCUR == save.SVNR1) && (save.RPTR < save.SVNR2)) {
                    //
                    // We've compared every left hand entry from RPTR
                    // upwards to the right hand entry.  Time to work on
                    // the next right hand entry.
                    //
                    save.RPTR = (save.RPTR + 1);
                    save.LCUR = save.LPTR;
                } else {
                    //
                    // LCUR and RPTR point to the last entries in their
                    // respective row sets.
                    //
                    save.DONE = true;
                }
            } else {
                //
                // The current left key entry is greater than that
                // on the right.  It's time to look at the next entry
                // on the right, if possible.
                //
                if (save.RPTR < save.SVNR2) {
                    save.RPTR = (save.RPTR + 1);
                    save.LCUR = save.LPTR;
                } else {
                    save.DONE = true;
                }
            }

            //
            // At this point, we've advanced at least one of LPTR, RPTR,
            // or LCUR, or else we've set DONE to .TRUE.
            //
        }
    } else if (save.CASE == NONEQ) {
        //
        // This is the non-equi-join case.
        //
        while (!save.DONE && !*FOUND) {
            //
            // At this point, LPTR and RPTR should point to the current
            // pair of order vector entries to use.  We should always have
            //
            //    1     <  LPTR  <  SVNR1
            //          -        -
            //
            //    1     <  RPTR  <  SVNR2
            //          -        -
            //
            // here.
            //
            // Look up the next set of row vector indices.  Get the row
            // numbers in the join columns for each order vector in our
            // mini-join row sets that we created for sorting.
            //
            ZZEKSRD(
                (save.LOVBAS + save.LPTR),
                (save.LOVBAS + save.LPTR),
                std::slice::from_mut(&mut save.LRVIDX),
                ctx,
            )?;
            ZZEKSRD(
                (save.ROVBAS + save.RPTR),
                (save.ROVBAS + save.RPTR),
                std::slice::from_mut(&mut save.RRVIDX),
                ctx,
            )?;

            save.ADDRSS = (((save.LBASE + 7) + (2 * (save.LRVIDX - 1))) + 1);
            ZZEKSRD(
                save.ADDRSS,
                save.ADDRSS,
                std::slice::from_mut(&mut save.LROW),
                ctx,
            )?;

            save.ADDRSS = (((save.RBASE + 7) + (2 * (save.RRVIDX - 1))) + 1);
            ZZEKSRD(
                save.ADDRSS,
                save.ADDRSS,
                std::slice::from_mut(&mut save.RROW),
                ctx,
            )?;

            //
            // Compare column entries, and advance the pointers as
            // required.
            //
            if ZZEKRCMP(
                save.SVOPS[save.CNSTR],
                1,
                save.LHANS[save.CNSTR],
                save.LSDSC.subarray([1, save.CNSTR]),
                save.LDSCRS.subarray([1, save.CNSTR]),
                save.LROW,
                &[save.LELT],
                save.RHANS[save.CNSTR],
                save.RSDSC.subarray([1, save.CNSTR]),
                save.RDSCRS.subarray([1, save.CNSTR]),
                save.RROW,
                &[save.RELT],
                ctx,
            )? {
                //
                //
                // This pair of row vectors satisfies the join constraint.
                // Form a composite row vector and test it against the full
                // set of active constraints.
                //
                if (save.SVCP1[save.CNSTR] <= save.SVNT1) {
                    //
                    // The parent table of the column on the LHS of our
                    // equi-join constraint belongs to the first join
                    // row set.
                    //
                    save.J = 1;
                    save.K = (save.SVNT1 + 1);
                } else {
                    save.J = (save.SVNT2 + 1);
                    save.K = 1;
                }

                save.OFFSET = (save.SVRB1 + ((save.LRVIDX - 1) * (save.SVNT1 + 1)));

                ZZEKSRD(
                    ((save.SVBAS1 + save.OFFSET) + 1),
                    ((save.SVBAS1 + save.OFFSET) + save.SVNT1),
                    ROWVEC.subarray_mut(save.J),
                    ctx,
                )?;

                save.OFFSET = (save.SVRB2 + ((save.RRVIDX - 1) * (save.SVNT2 + 1)));

                ZZEKSRD(
                    ((save.SVBAS2 + save.OFFSET) + 1),
                    ((save.SVBAS2 + save.OFFSET) + save.SVNT2),
                    ROWVEC.subarray_mut(save.K),
                    ctx,
                )?;

                //
                // Create row arrays for both sides of each active
                // relational constraint.
                //
                {
                    let m1__: i32 = 1;
                    let m2__: i32 = save.SVNCON;
                    let m3__: i32 = 1;
                    save.J = m1__;
                    for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                        if save.LOCACT[save.J] {
                            save.LTAB = save.SVCP1[save.J];
                            save.RTAB = save.SVCP2[save.J];
                            save.LROWS[save.J] = ROWVEC[save.LTAB];
                            save.RROWS[save.J] = ROWVEC[save.RTAB];
                        }

                        save.J += m3__;
                    }
                }

                *FOUND = ZZEKVMCH(
                    save.SVNCON,
                    save.LOCACT.as_slice(),
                    save.LHANS.as_slice(),
                    save.LSDSC.as_slice(),
                    save.LDSCRS.as_slice(),
                    save.LROWS.as_slice(),
                    save.LELTS.as_slice(),
                    save.SVOPS.as_slice(),
                    save.RHANS.as_slice(),
                    save.RSDSC.as_slice(),
                    save.RDSCRS.as_slice(),
                    save.RROWS.as_slice(),
                    save.RELTS.as_slice(),
                    ctx,
                )?;

                if save.LSMALL {
                    //
                    // The `left' key entry is smaller.  All higher-indexed
                    // rows on the right side also satisfy the join
                    // constraint, combined with the current left hand side.
                    //
                    if (save.RPTR < save.SVNR2) {
                        save.RPTR = (save.RPTR + 1);
                    } else if (save.LPTR < save.SVNR1) {
                        save.LPTR = (save.LPTR + 1);
                        save.RPTR = 1;
                    } else {
                        save.DONE = true;
                    }
                } else {
                    //
                    // The `right' key entry is smaller.  All higher-indexed
                    // rows on the left side also satisfy the join
                    // constraint, combined with the current right hand side.
                    //
                    if (save.LPTR < save.SVNR1) {
                        save.LPTR = (save.LPTR + 1);
                    } else if (save.RPTR < save.SVNR2) {
                        save.RPTR = (save.RPTR + 1);
                        save.LPTR = 1;
                    } else {
                        save.DONE = true;
                    }
                }
            //
            // We incremented LPTR or RPTR, or else we set DONE to
            // .TRUE.
            //
            } else {
                //
                // The constraint was not met by the rows under
                // consideration.
                //
                if save.LSMALL {
                    //
                    // If the right side can be incremented, there's a
                    // chance of a match.
                    //
                    if (save.RPTR < save.SVNR2) {
                        save.RPTR = (save.RPTR + 1);
                    } else {
                        save.DONE = true;
                    }
                } else {
                    //
                    // If the left side can be incremented, there's a
                    // chance of a match.
                    //
                    if (save.LPTR < save.SVNR1) {
                        save.LPTR = (save.LPTR + 1);
                    } else {
                        save.DONE = true;
                    }
                }
                //
                // We incremented LPTR or RPTR, or else we set DONE to
                // .TRUE.
                //
            }
        }
    } else {
        //
        // We have no order vectors to help us out, so we just loop
        // through every possible combination.  When we find a match,
        // we return immediately, leaving the pointers set to enable
        // continuation of our search when we drop back into the loop
        // on a subsequent call.
        //
        while (save.LPTR <= save.SVNR1) {
            while (save.RPTR <= save.SVNR2) {
                //
                // Form a composite row vector and test it against the full
                // set of active constraints.
                //
                save.OFFSET = (save.SVRB1 + ((save.LPTR - 1) * (save.SVNT1 + 1)));

                ZZEKSRD(
                    ((save.SVBAS1 + save.OFFSET) + 1),
                    ((save.SVBAS1 + save.OFFSET) + save.SVNT1),
                    ROWVEC.as_slice_mut(),
                    ctx,
                )?;

                save.OFFSET = (save.SVRB2 + ((save.RPTR - 1) * (save.SVNT2 + 1)));

                ZZEKSRD(
                    ((save.SVBAS2 + save.OFFSET) + 1),
                    ((save.SVBAS2 + save.OFFSET) + save.SVNT2),
                    ROWVEC.subarray_mut((save.SVNT1 + 1)),
                    ctx,
                )?;

                //
                // Create row arrays for both sides of each active
                // relational constraint.
                //
                {
                    let m1__: i32 = 1;
                    let m2__: i32 = save.SVNCON;
                    let m3__: i32 = 1;
                    save.J = m1__;
                    for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                        if save.LOCACT[save.J] {
                            save.LTAB = save.SVCP1[save.J];
                            save.RTAB = save.SVCP2[save.J];
                            save.LROWS[save.J] = ROWVEC[save.LTAB];
                            save.RROWS[save.J] = ROWVEC[save.RTAB];
                        }

                        save.J += m3__;
                    }
                }

                *FOUND = ZZEKVMCH(
                    save.SVNCON,
                    save.LOCACT.as_slice(),
                    save.LHANS.as_slice(),
                    save.LSDSC.as_slice(),
                    save.LDSCRS.as_slice(),
                    save.LROWS.as_slice(),
                    save.LELTS.as_slice(),
                    save.SVOPS.as_slice(),
                    save.RHANS.as_slice(),
                    save.RSDSC.as_slice(),
                    save.RDSCRS.as_slice(),
                    save.RROWS.as_slice(),
                    save.RELTS.as_slice(),
                    ctx,
                )?;

                save.RPTR = (save.RPTR + 1);

                if *FOUND {
                    return Ok(());
                }
            }

            save.LPTR = (save.LPTR + 1);
            save.RPTR = 1;
        }
    }

    Ok(())
}
