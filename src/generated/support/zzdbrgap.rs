//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const SRFIDX: i32 = 1;
const CTRIDX: i32 = (SRFIDX + 1);
const CLSIDX: i32 = (CTRIDX + 1);
const TYPIDX: i32 = (CLSIDX + 1);
const FRMIDX: i32 = (TYPIDX + 1);
const SYSIDX: i32 = (FRMIDX + 1);
const PARIDX: i32 = (SYSIDX + 1);
const NSYPAR: i32 = 10;
const MN1IDX: i32 = (PARIDX + NSYPAR);
const MX1IDX: i32 = (MN1IDX + 1);
const MN2IDX: i32 = (MX1IDX + 1);
const MX2IDX: i32 = (MN2IDX + 1);
const MN3IDX: i32 = (MX2IDX + 1);
const MX3IDX: i32 = (MN3IDX + 1);
const BTMIDX: i32 = (MX3IDX + 1);
const ETMIDX: i32 = (BTMIDX + 1);
const DSKDSZ: i32 = ETMIDX;
const SVFCLS: i32 = 1;
const GENCLS: i32 = 2;
const LATSYS: i32 = 1;
const CYLSYS: i32 = 2;
const RECSYS: i32 = 3;
const PDTSYS: i32 = 4;
pub const MAXC: i32 = 100000;
pub const MAXGRD: i32 = 1000000;
const LBCELL: i32 = -5;
const MAXC2: i32 = (2 * MAXC);

struct SaveVars {
    MAXLON: f64,
    MINLON: f64,
    OUTXBD: ActualArray2D<f64>,
    OUTYBD: ActualArray2D<f64>,
    CIVORX: ActualArray<i32>,
    CIVORY: ActualArray<i32>,
    CMPORX: ActualArray<i32>,
    CMPORY: ActualArray<i32>,
    H: i32,
    I: i32,
    J: i32,
    K: i32,
    MAXPXX: ActualArray<i32>,
    MAXPXY: ActualArray<i32>,
    MINPXX: ActualArray<i32>,
    MINPXY: ActualArray<i32>,
    MRKSET: ActualArray<i32>,
    NCOLS: i32,
    NR: i32,
    NROWS: i32,
    ORDX: ActualArray<i32>,
    ORDY: ActualArray<i32>,
    SRCS: ActualArray<i32>,
    TMPSET: ActualArray<i32>,
    VSET: ActualArray<i32>,
    COVERD: bool,
    GAPVAL: bool,
    GRID: ActualArray<bool>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut MAXLON: f64 = 0.0;
        let mut MINLON: f64 = 0.0;
        let mut OUTXBD = ActualArray2D::<f64>::new(1..=2, 1..=MAXC);
        let mut OUTYBD = ActualArray2D::<f64>::new(1..=2, 1..=MAXC);
        let mut CIVORX = ActualArray::<i32>::new(1..=MAXC2);
        let mut CIVORY = ActualArray::<i32>::new(1..=MAXC2);
        let mut CMPORX = ActualArray::<i32>::new(1..=MAXC2);
        let mut CMPORY = ActualArray::<i32>::new(1..=MAXC2);
        let mut H: i32 = 0;
        let mut I: i32 = 0;
        let mut J: i32 = 0;
        let mut K: i32 = 0;
        let mut MAXPXX = ActualArray::<i32>::new(1..=MAXC2);
        let mut MAXPXY = ActualArray::<i32>::new(1..=MAXC2);
        let mut MINPXX = ActualArray::<i32>::new(1..=MAXC2);
        let mut MINPXY = ActualArray::<i32>::new(1..=MAXC2);
        let mut MRKSET = ActualArray::<i32>::new(LBCELL..=MAXC2);
        let mut NCOLS: i32 = 0;
        let mut NR: i32 = 0;
        let mut NROWS: i32 = 0;
        let mut ORDX = ActualArray::<i32>::new(1..=MAXC2);
        let mut ORDY = ActualArray::<i32>::new(1..=MAXC2);
        let mut SRCS = ActualArray::<i32>::new(1..=MAXC2);
        let mut TMPSET = ActualArray::<i32>::new(LBCELL..=MAXC2);
        let mut VSET = ActualArray::<i32>::new(LBCELL..=MAXC2);
        let mut COVERD: bool = false;
        let mut GAPVAL: bool = false;
        let mut GRID = ActualArray::<bool>::new(1..=MAXGRD);

        Self {
            MAXLON,
            MINLON,
            OUTXBD,
            OUTYBD,
            CIVORX,
            CIVORY,
            CMPORX,
            CMPORY,
            H,
            I,
            J,
            K,
            MAXPXX,
            MAXPXY,
            MINPXX,
            MINPXY,
            MRKSET,
            NCOLS,
            NR,
            NROWS,
            ORDX,
            ORDY,
            SRCS,
            TMPSET,
            VSET,
            COVERD,
            GAPVAL,
            GRID,
        }
    }
}

//$Procedure ZZDBRGAP ( DSKBRIEF, compute gaps in coverage )
pub fn ZZDBRGAP(
    CORSYS: i32,
    NREC: i32,
    BDS1: &[f64],
    BDS2: &[f64],
    MAXN: i32,
    NCOMP: &mut i32,
    CBDS1: &mut [f64],
    CBDS2: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let BDS1 = DummyArray2D::new(BDS1, 1..=2, 1..=NREC);
    let BDS2 = DummyArray2D::new(BDS2, 1..=2, 1..=NREC);
    let mut CBDS1 = DummyArrayMut2D::new(CBDS1, 1..=2, 1..);
    let mut CBDS2 = DummyArrayMut2D::new(CBDS2, 1..=2, 1..);

    //
    // SPICELIB functions
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

    if spicelib::RETURN(ctx) {
        return Ok(());
    }

    spicelib::CHKIN(b"ZZDBRGAP", ctx)?;

    //
    // Initialize the workspace sets.
    //
    spicelib::SSIZEI(MAXC, save.MRKSET.as_slice_mut(), ctx)?;
    spicelib::SSIZEI(MAXC, save.TMPSET.as_slice_mut(), ctx)?;
    spicelib::SSIZEI(MAXC, save.VSET.as_slice_mut(), ctx)?;

    if ((CORSYS == LATSYS) || (CORSYS == PDTSYS)) {
        //
        // Adjust the input longitudes to make them usable by RC2GRD.
        //
        REGLON(
            NREC,
            BDS1.as_slice(),
            MAXN,
            &mut save.NR,
            &mut save.MINLON,
            &mut save.MAXLON,
            save.OUTXBD.as_slice_mut(),
            save.SRCS.as_slice_mut(),
            ctx,
        )?;
        //
        // Since REGLON may create new rectangles, the input set of "Y"
        // bounds (actually latitude) may not match up with the bounds in
        // OUTXBD. Create a new array of "Y" bounds parallel to the array
        // of X bounds. In this array, each rectangle has the Y bounds of
        // the source box from which it was created.
        //
        {
            let m1__: i32 = 1;
            let m2__: i32 = save.NR;
            let m3__: i32 = 1;
            save.I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                save.OUTYBD[[1, save.I]] = BDS2[[1, save.SRCS[save.I]]];
                save.OUTYBD[[2, save.I]] = BDS2[[2, save.SRCS[save.I]]];

                save.I += m3__;
            }
        }
    } else {
        //
        // Just transfer the input X and Y bounds.
        //
        spicelib::MOVED(BDS1.as_slice(), (2 * NREC), save.OUTXBD.as_slice_mut());
        spicelib::MOVED(BDS2.as_slice(), (2 * NREC), save.OUTYBD.as_slice_mut());

        save.NR = NREC;
    }

    //
    // Map the coordinate rectangles to a pixel grid. Mark
    // the coverage with the value .TRUE.
    //
    save.COVERD = true;
    save.GAPVAL = false;

    RC2GRD(
        save.NR,
        save.OUTXBD.as_slice(),
        save.OUTYBD.as_slice(),
        MAXGRD,
        MAXC,
        save.COVERD,
        save.ORDX.as_slice_mut(),
        save.ORDY.as_slice_mut(),
        save.CIVORX.as_slice_mut(),
        save.CIVORY.as_slice_mut(),
        save.CMPORX.as_slice_mut(),
        save.CMPORY.as_slice_mut(),
        &mut save.NROWS,
        &mut save.NCOLS,
        save.GRID.as_slice_mut(),
        ctx,
    )?;
    //
    // Map the gaps in the pixel grid to a set of rectangles in pixel
    // space.
    //
    save.I = (save.NROWS * save.NCOLS);
    save.K = 0;

    {
        let m1__: i32 = 1;
        let m2__: i32 = save.I;
        let m3__: i32 = 1;
        save.J = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            if save.GRID[save.J] {
                save.K = (save.K + 1);
            }

            save.J += m3__;
        }
    }

    FNDCMP(
        save.NROWS,
        save.NCOLS,
        save.GAPVAL,
        MAXN,
        save.GRID.as_slice_mut(),
        save.VSET.as_slice_mut(),
        save.MRKSET.as_slice_mut(),
        save.TMPSET.as_slice_mut(),
        NCOMP,
        save.MINPXX.as_slice_mut(),
        save.MAXPXX.as_slice_mut(),
        save.MINPXY.as_slice_mut(),
        save.MAXPXY.as_slice_mut(),
        ctx,
    )?;

    if spicelib::FAILED(ctx) {
        spicelib::CHKOUT(b"ZZDBRGAP", ctx)?;
        return Ok(());
    }

    //
    // Map the gap rectangles, which are expressed in pixel coordinates,
    // to a set of rectangles in the input coordinate system.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = *NCOMP;
        let m3__: i32 = 1;
        save.I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            //
            // If the range of a pixel coordinate is
            //
            //    A:B
            //
            // then the range of the indices of the corresponding bounds in
            // the compressed, ordered set of bounds is
            //
            //    A : B+1
            //
            // Map each bound index to the corresponding value using the
            // mappings output by RC2GRD.
            //
            // We need to deal with the fact that the arrays CMPORX and
            // CMPORY treat the bounds arrays OUTXBD and OUTYBD as
            // one-dimensional.
            //
            save.J = save.CMPORX[save.MINPXX[save.I]];

            save.K = (1 + ((save.J - 1) / 2));
            save.H = (save.J - ((save.K - 1) * 2));

            CBDS1[[1, save.I]] = save.OUTXBD[[save.H, save.K]];

            save.J = save.CMPORX[(save.MAXPXX[save.I] + 1)];

            save.K = (1 + ((save.J - 1) / 2));
            save.H = (save.J - ((save.K - 1) * 2));

            CBDS1[[2, save.I]] = save.OUTXBD[[save.H, save.K]];

            save.J = save.CMPORY[save.MINPXY[save.I]];

            save.K = (1 + ((save.J - 1) / 2));
            save.H = (save.J - ((save.K - 1) * 2));

            CBDS2[[1, save.I]] = save.OUTYBD[[save.H, save.K]];

            save.J = save.CMPORY[(save.MAXPXY[save.I] + 1)];

            save.K = (1 + ((save.J - 1) / 2));
            save.H = (save.J - ((save.K - 1) * 2));

            CBDS2[[2, save.I]] = save.OUTYBD[[save.H, save.K]];

            save.I += m3__;
        }
    }

    spicelib::CHKOUT(b"ZZDBRGAP", ctx)?;
    Ok(())
}
