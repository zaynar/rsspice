//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const CTRSIZ: i32 = 2;
const WDSIZE: i32 = 32;
const NCK: i32 = 30;
const SCLK: i32 = 1;
const SPK: i32 = (SCLK + 1);

struct SaveVars {
    BASE: Vec<u8>,
    MYMETA: Vec<u8>,
    LOOKUP: ActualCharArray2D,
    AGENT: ActualCharArray,
    CKS: StackArray<i32, 30>,
    CKSORD: StackArray<i32, 30>,
    CURRNT: i32,
    LAST: i32,
    N: i32,
    SCLKS: StackArray<i32, 30>,
    SPKS: StackArray<i32, 30>,
    THIS: i32,
    USRCTR: StackArray2D<i32, 60>,
    FIRST: bool,
    FOUND: StackArray<bool, 2>,
    NODATA: bool,
    UPDATE: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut BASE = vec![b' '; 7 as usize];
        let mut MYMETA = vec![b' '; 7 as usize];
        let mut LOOKUP = ActualCharArray2D::new(WDSIZE, 1..=2, 1..=NCK);
        let mut AGENT = ActualCharArray::new(WDSIZE, 1..=NCK);
        let mut CKS = StackArray::<i32, 30>::new(1..=NCK);
        let mut CKSORD = StackArray::<i32, 30>::new(1..=NCK);
        let mut CURRNT: i32 = 0;
        let mut LAST: i32 = 0;
        let mut N: i32 = 0;
        let mut SCLKS = StackArray::<i32, 30>::new(1..=NCK);
        let mut SPKS = StackArray::<i32, 30>::new(1..=NCK);
        let mut THIS: i32 = 0;
        let mut USRCTR = StackArray2D::<i32, 60>::new(1..=CTRSIZ, 1..=NCK);
        let mut FIRST: bool = false;
        let mut FOUND = StackArray::<bool, 2>::new(1..=2);
        let mut NODATA: bool = false;
        let mut UPDATE: bool = false;

        fstr::assign(&mut BASE, b"CKMETA.");
        CURRNT = 0;
        FIRST = true;
        LAST = 0;
        NODATA = true;

        Self {
            BASE,
            MYMETA,
            LOOKUP,
            AGENT,
            CKS,
            CKSORD,
            CURRNT,
            LAST,
            N,
            SCLKS,
            SPKS,
            THIS,
            USRCTR,
            FIRST,
            FOUND,
            NODATA,
            UPDATE,
        }
    }
}

/// CK ID to associated SCLK
///
/// Return (depending upon the user's request) the ID code of either
/// the spacecraft or spacecraft clock associated with a C-Kernel ID
/// code.
///
/// # Required Reading
///
/// * [CK](crate::required_reading::ck)
/// * [FRAMES](crate::required_reading::frames)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  CKID       I   The ID code for some C kernel object.
///  META       I   The kind of meta data requested 'SPK' or 'SCLK'
///  IDCODE     O   The requested SCLK or spacecraft ID code.
/// ```
///
/// # Detailed Input
///
/// ```text
///  CKID     is the ID code for some object whose attitude
///           and possibly angular velocity are stored in
///           some C-kernel.
///
///  META     is a character string that indicates which piece
///           of meta data to fetch. Acceptable values are
///           'SCLK' and 'SPK'. The routine is case insensitive.
///           Leading and trailing blanks are insignificant.
///           However, blanks between characters are regarded
///           as being significant.
/// ```
///
/// # Detailed Output
///
/// ```text
///  IDCODE   if META is 'SCLK' then the value returned in IDCODE
///           is the ID code of the spacecraft clock used for
///           converting ET to TICKS and TICKS to ET for the
///           C-kernel used to represent the attitude of the
///           object with ID code CKID.
///
///           If META is 'SPK' then the value returned in IDCODE is the
///           ID code of the spacecraft on which the platform indicated
///           by CKID is mounted.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the variable META is not recognized to be one of the
///      inputs 'SPK' or 'SCLK', the error SPICE(UNKNOWNCKMETA)
///      is signaled.
///
///  2)  If CKID is greater than -1000, the associated SCLK and SPK
///      IDs must be in the kernel pool. If they are not present
///      a value of zero is returned for the requested item. Zero
///      is never the valid ID of a spacecraft clock.
/// ```
///
/// # Particulars
///
/// ```text
///  This is a utility routine for mapping C-kernels to associated
///  spacecraft clocks.
///
///  An association of an SCLK ID and spacecraft ID with a CK frame
///  class ID may be made by placing in a text kernel the kernel
///  variable assignments
///
///     CK_<ck_frame_class_ID_code>_SCLK = <ID code of SCLK>
///     CK_<ck_frame_class_ID_code>_SPK  = <SPK ID code>
///
///  See the Frames Required Reading section on CK frames.
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for this example may differ across
///  platforms. The results depend on the SPICE kernels used as
///  input, the compiler and supporting libraries, and the machine
///  specific arithmetic implementation.
///
///  1) Suppose you would like to look up the attitude of an object
///     in a C-kernel but have ET and seconds as your input time and
///     tolerance.
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File name: ckmeta_ex1.tm
///
///        This meta-kernel is intended to support operation of SPICE
///        example programs. The kernels shown here should not be
///        assumed to contain adequate or correct versions of data
///        required by SPICE-based user applications.
///
///        In order for an application to use this meta-kernel, the
///        kernels referenced here must be present in the user's
///        current working directory.
///
///        The names and contents of the kernels referenced
///        by this meta-kernel are as follows:
///
///           File name              Contents
///           --------------------   -----------------------
///           cas00071.tsc           CASSINI SCLK
///           naif0012.tls           Leapseconds
///           04153_04182ca_ISS.bc   CASSINI image navigated
///                                  spacecraft CK
///
///
///        \begindata
///
///           KERNELS_TO_LOAD = ( 'naif0012.tls',
///                               'cas00071.tsc'
///                               '04153_04182ca_ISS.bc' )
///
///        \begintext
///
///        End of meta-kernel
///
///
///     Example code begins here.
///
///
///           PROGRAM CKMETA_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local parameters.
///     C
///     C     -- The code for CASSINI spacecraft reference frame is
///     C        -82000.
///     C
///     C     -- The reference frame we want is J2000.
///     C
///           CHARACTER*(*)         REF
///           PARAMETER           ( REF = 'J2000' )
///
///           INTEGER               CKID
///           PARAMETER           ( CKID = -82000  )
///
///     C
///     C     Local variables.
///     C
///           DOUBLE PRECISION      AV     ( 3 )
///           DOUBLE PRECISION      CLKOUT
///           DOUBLE PRECISION      CMAT   ( 3, 3 )
///           DOUBLE PRECISION      ET
///           DOUBLE PRECISION      ETOUT
///           DOUBLE PRECISION      SECTOL
///           DOUBLE PRECISION      TICKS
///           DOUBLE PRECISION      TICK2
///           DOUBLE PRECISION      TOL
///
///           INTEGER               IDCODE
///
///           LOGICAL               FOUND
///
///     C
///     C     Initial values.
///     C
///           DATA                  ET      / 141162208.034340D0 /
///           DATA                  SECTOL  / 0.5D0 /
///
///
///     C
///     C     First load the CK, LSK and SCLK files.
///     C
///           CALL FURNSH ( 'ckmeta_ex1.tm' )
///
///     C
///     C     Get the SCLK identifier of the spacecraft clock required
///     C     to convert from ET to TICKS.
///     C
///           CALL CKMETA ( CKID, 'SCLK', IDCODE )
///
///     C
///     C     Convert ET and ET+SECTOL to spacecraft clock ticks.
///     C
///           CALL SCE2C  ( IDCODE, ET,        TICKS  )
///           CALL SCE2C  ( IDCODE, ET+SECTOL, TICK2  )
///
///
///     C
///     C     Compute the tolerance in spacecraft clock ticks.
///     C
///           TOL = TICK2 - TICKS
///
///     C
///     C     Look the attitude up.
///     C
///           CALL CKGPAV ( CKID, TICKS, TOL,    REF,
///          .              CMAT, AV,    CLKOUT, FOUND )
///
///           WRITE(*,'(A,F20.6)')    'Input ET:            ', ET
///
///           IF ( FOUND ) THEN
///
///              CALL SCT2E ( IDCODE, CLKOUT, ETOUT )
///              WRITE(*,'(A,F20.6)') 'Attitude found at ET:', ETOUT
///
///           ELSE
///
///              WRITE(*,'(A)') 'No attitude found at ET.'
///
///           END IF
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     Input ET:                141162208.034340
///     Attitude found at ET:    141162208.034586
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.2.1, 07-JUN-2021 (JDR) (NJB)
///
///         Edited the header to comply with NAIF standard. Added complete
///         code example based on existing fragment.
///
/// -    SPICELIB Version 1.2.0, 06-SEP-2013 (BVS)
///
///         BUG FIX: the POOL agents now watch both variables --
///         CK_<ID>_SCLK and CK_<ID>_SPK. Before they watched only
///         CK_<ID>_SCLK.
///
///         BUG FIX: if a previously available CK_<ID>_SCLK or CK_<ID>_SPK
///         variable that was used to populate a saved value disappears,
///         the routine now resets and returns the value based on the
///         default rule rather than keeping and returning the stale
///         POOL-based saved value.
///
///         BUG FIX: the routine now deletes watchers for the CK IDs that
///         were bumped from the local buffer.
///
///         Updated to keep track of agent-specific POOL counters and call
///         ZZCVPOOL to make use of them.
///
/// -    SPICELIB Version 1.1.0, 05-MAR-2009 (NJB)
///
///         This routine now keeps track of whether its kernel pool
///         look-up failed. If so, a kernel pool lookup is attempted on
///         the next call to this routine. This change is an enhancement,
///         not a bug fix (unlike similar modifications in SCLK routines).
///
///         Header sections were put in correct order.
///
/// -    SPICELIB Version 1.0.1, 09-MAR-1999 (NJB)
///
///         Comments referring to SCE2T have been updated to refer to
///         SCE2C. Occurrences of "id" replaced by "ID."
///
/// -    SPICELIB Version 1.0.0, 04-OCT-1994 (WLT)
/// ```
pub fn ckmeta(
    ctx: &mut SpiceContext,
    ckid: i32,
    meta: &str,
    idcode: &mut i32,
) -> crate::Result<()> {
    CKMETA(ckid, meta.as_bytes(), idcode, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure CKMETA ( CK ID to associated SCLK )
pub fn CKMETA(
    CKID: i32,
    META: &[u8],
    IDCODE: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB Functions
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

    //
    // Initial values
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"CKMETA", ctx)?;

    if save.FIRST {
        //
        // Initialize all agent-specific POOL counters to user value.
        //
        {
            let m1__: i32 = 1;
            let m2__: i32 = NCK;
            let m3__: i32 = 1;
            save.N = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                ZZCTRUIN(save.USRCTR.subarray_mut([1, save.N]), ctx);
                save.N += m3__;
            }
        }

        //
        // Clear AGENTS array. We will use a non-blank AGENT value as the
        // flag to delete previously set watchers.
        //
        CLEARC(NCK, save.AGENT.as_arg_mut());

        save.FIRST = false;
    }

    //
    // Get an upper-case, left-justified copy of the metadata
    // type ('SCLK' or 'SPK').
    //
    LJUCRS(1, META, &mut save.MYMETA, ctx);

    //
    // See if we already have this CK ID in hand.
    //
    save.THIS = BSCHOI(
        CKID,
        save.CURRNT,
        save.CKS.as_slice(),
        save.CKSORD.as_slice(),
    );

    if (save.THIS > 0) {
        //
        // We've got it.  Check to see if its value has been updated.
        // (Note that every CK ID  has its own agent and saved POOL
        // counter.)
        //
        ZZCVPOOL(
            &save.AGENT[save.THIS],
            save.USRCTR.subarray_mut([1, save.THIS]),
            &mut save.UPDATE,
            ctx,
        )?;

        if (save.UPDATE || save.NODATA) {
            GIPOOL(
                &save.LOOKUP[[SCLK, save.THIS]],
                1,
                1,
                &mut save.N,
                save.SCLKS.subarray_mut(save.THIS),
                &mut save.FOUND[SCLK],
                ctx,
            )?;

            GIPOOL(
                &save.LOOKUP[[SPK, save.THIS]],
                1,
                1,
                &mut save.N,
                save.SPKS.subarray_mut(save.THIS),
                &mut save.FOUND[SPK],
                ctx,
            )?;

            if FAILED(ctx) {
                save.NODATA = true;

                CHKOUT(b"CKMETA", ctx)?;
                return Ok(());
            }

            //
            // Note that failure to find data is not an error in this
            // routine; it's just SPICE errors that are a problem.
            //
            save.NODATA = false;
        } else {
            //
            // The POOL variables did not change since the last check and
            // we have already buffered IDs for this CK ID. Set found
            // flags to make use of saved values.
            //
            save.FOUND[SCLK] = true;
            save.FOUND[SPK] = true;
        }
    } else {
        //
        // We don't have this on our handy list. Find a place to put it.
        //
        if (save.CURRNT < NCK) {
            save.CURRNT = (save.CURRNT + 1);
            save.LAST = save.CURRNT;
        } else {
            save.LAST = (save.LAST + 1);

            if (save.LAST > NCK) {
                save.LAST = 1;
            }
        }

        save.THIS = save.LAST;

        //
        // If we already have a watcher at this index, delete it. Note
        // we may have an update pending for this watcher, so we will
        // check it first to clear it.
        //
        if fstr::ne(save.AGENT.get(save.THIS), b" ") {
            CVPOOL(&save.AGENT[save.THIS], &mut save.UPDATE, ctx)?;
            DWPOOL(&save.AGENT[save.THIS], ctx)?;
        }

        //
        // Recompute the order vector for the CKS; construct the
        // kernel pool variable names and the agent name.
        //
        save.CKS[save.THIS] = CKID;

        ORDERI(save.CKS.as_slice(), save.CURRNT, save.CKSORD.as_slice_mut());

        INTSTR(CKID, &mut save.LOOKUP[[SCLK, save.THIS]], ctx);
        PREFIX(b"CK_", 0, &mut save.LOOKUP[[SCLK, save.THIS]]);

        fstr::assign(
            save.AGENT.get_mut(save.THIS),
            &fstr::concat(&save.BASE, save.LOOKUP.get([SCLK, save.THIS])),
        );
        let val = save.LOOKUP.get([SCLK, save.THIS]).to_vec();
        fstr::assign(save.LOOKUP.get_mut([SPK, save.THIS]), &val);

        SUFFIX(b"_SCLK", 0, &mut save.LOOKUP[[SCLK, save.THIS]]);
        SUFFIX(b"_SPK", 0, &mut save.LOOKUP[[SPK, save.THIS]]);

        //
        // Set a watch for this item and fetch the current value
        // from the kernel pool (if there is a value there).
        //
        SWPOOL(
            &save.AGENT[save.THIS],
            2,
            save.LOOKUP.subarray([1, save.THIS]),
            ctx,
        )?;

        CVPOOL(&save.AGENT[save.THIS], &mut save.UPDATE, ctx)?;

        GIPOOL(
            &save.LOOKUP[[SCLK, save.THIS]],
            1,
            1,
            &mut save.N,
            save.SCLKS.subarray_mut(save.THIS),
            &mut save.FOUND[SCLK],
            ctx,
        )?;

        GIPOOL(
            &save.LOOKUP[[SPK, save.THIS]],
            1,
            1,
            &mut save.N,
            save.SPKS.subarray_mut(save.THIS),
            &mut save.FOUND[SPK],
            ctx,
        )?;

        if FAILED(ctx) {
            save.NODATA = true;

            CHKOUT(b"CKMETA", ctx)?;
            return Ok(());
        }

        //
        // Note that failure to find data is not an error in this
        // routine; it's just SPICE errors that are a problem.
        //
        // At this point, kernel data checks are done.
        //
        save.NODATA = false;
    }

    //
    // If we didn't find either _SCLK or _SPK variable, we manufacture
    // an ID code based upon the "convention" used for all CKS so far.
    // However, the convention assumes that the CK ID will be less than
    // -1000 if it's not there is no sensible ID to return.  We return
    // zero in that case.
    //
    if !save.FOUND[SCLK] {
        if (save.CKS[save.THIS] <= -1000) {
            save.SCLKS[save.THIS] = (save.CKS[save.THIS] / 1000);
        } else {
            save.SCLKS[save.THIS] = 0;
        }
    }

    if !save.FOUND[SPK] {
        if (save.CKS[save.THIS] <= -1000) {
            save.SPKS[save.THIS] = (save.CKS[save.THIS] / 1000);
        } else {
            save.SPKS[save.THIS] = 0;
        }
    }

    //
    // Set output ID.
    //
    if fstr::eq(&save.MYMETA, b"SPK") {
        *IDCODE = save.SPKS[save.THIS];
    } else if fstr::eq(&save.MYMETA, b"SCLK") {
        *IDCODE = save.SCLKS[save.THIS];
    } else {
        *IDCODE = 0;

        SETMSG(b"The CK meta data item \"#\" is not a recognized meta data item for the routine CKMETA. The recognized value are \"SPK\" and \"SCLK\". ", ctx);

        ERRCH(b"#", META, ctx);
        SIGERR(b"SPICE(UNKNOWNCKMETA)", ctx)?;
        CHKOUT(b"CKMETA", ctx)?;
        return Ok(());
    }

    CHKOUT(b"CKMETA", ctx)?;

    Ok(())
}
