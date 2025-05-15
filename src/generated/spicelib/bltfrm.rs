//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const INERTL: i32 = 1;
const PCK: i32 = (INERTL + 1);
const CK: i32 = (PCK + 1);
const TK: i32 = (CK + 1);
const DYN: i32 = (TK + 1);
const SWTCH: i32 = (DYN + 1);
const ALL: i32 = -1;
const NINERT: i32 = 21;
const NNINRT: i32 = 124;
const LBCELL: i32 = -5;
const NFRAME: i32 = (NINERT + NNINRT);
const FRNMLN: i32 = 32;
const LBPOOL: i32 = -5;
const MAXBFR: i32 = (NFRAME + 1);

struct SaveVars {
    FRNAME: ActualCharArray,
    CTRORD: StackArray<i32, 145>,
    CENTER: StackArray<i32, 145>,
    CORDER: StackArray<i32, 145>,
    FCLASS: StackArray<i32, 145>,
    FCLSID: StackArray<i32, 145>,
    FCODE: StackArray<i32, 145>,
    J: i32,
    TO: i32,
    PASS1: bool,
    BNMLST: StackArray<i32, 146>,
    BNMPOL: StackArray<i32, 152>,
    BNMNMS: ActualCharArray,
    BNMIDX: StackArray<i32, 146>,
    BIDLST: StackArray<i32, 146>,
    BIDPOL: StackArray<i32, 152>,
    BIDIDS: StackArray<i32, 146>,
    BIDIDX: StackArray<i32, 146>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut FRNAME = ActualCharArray::new(FRNMLN, 1..=NFRAME);
        let mut CTRORD = StackArray::<i32, 145>::new(1..=NFRAME);
        let mut CENTER = StackArray::<i32, 145>::new(1..=NFRAME);
        let mut CORDER = StackArray::<i32, 145>::new(1..=NFRAME);
        let mut FCLASS = StackArray::<i32, 145>::new(1..=NFRAME);
        let mut FCLSID = StackArray::<i32, 145>::new(1..=NFRAME);
        let mut FCODE = StackArray::<i32, 145>::new(1..=NFRAME);
        let mut J: i32 = 0;
        let mut TO: i32 = 0;
        let mut PASS1: bool = false;
        let mut BNMLST = StackArray::<i32, 146>::new(1..=MAXBFR);
        let mut BNMPOL = StackArray::<i32, 152>::new(LBPOOL..=MAXBFR);
        let mut BNMNMS = ActualCharArray::new(FRNMLN, 1..=MAXBFR);
        let mut BNMIDX = StackArray::<i32, 146>::new(1..=MAXBFR);
        let mut BIDLST = StackArray::<i32, 146>::new(1..=MAXBFR);
        let mut BIDPOL = StackArray::<i32, 152>::new(LBPOOL..=MAXBFR);
        let mut BIDIDS = StackArray::<i32, 146>::new(1..=MAXBFR);
        let mut BIDIDX = StackArray::<i32, 146>::new(1..=MAXBFR);

        PASS1 = true;

        Self {
            FRNAME,
            CTRORD,
            CENTER,
            CORDER,
            FCLASS,
            FCLSID,
            FCODE,
            J,
            TO,
            PASS1,
            BNMLST,
            BNMPOL,
            BNMNMS,
            BNMIDX,
            BIDLST,
            BIDPOL,
            BIDIDS,
            BIDIDX,
        }
    }
}

/// Built-in frame IDs
///
/// Return a set containing the frame IDs of all built-in frames of a
/// specified class.
///
/// # Required Reading
///
/// * [CELLS](crate::required_reading::cells)
/// * [FRAMES](crate::required_reading::frames)
/// * [NAIF_IDS](crate::required_reading::naif_ids)
/// * [SETS](crate::required_reading::sets)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  FRMCLS     I   Frame class.
///  IDSET      O   Set of ID codes of frames of the specified class.
/// ```
///
/// # Detailed Input
///
/// ```text
///  FRMCLS   is an integer code specifying the frame class or
///           classes for which built-in frame ID codes are
///           requested. FRMCLS may designate a single class or
///           "all classes."
///
///           The include file frmtyp.inc declares parameters
///           identifying frame classes. The supported values
///           and corresponding meanings of FRMCLS are
///
///              Parameter      Value    Meaning
///              =========      =====    =================
///              ALL              -1     All frame classes
///              INERTL            1     Built-in inertial
///              PCK               2     PCK-based frame
///              CK                3     CK-based frame
///              TK                4     Fixed offset ("text
///                                      kernel") frame
///              DYN               5     Dynamic frame
///              SWTCH             6     Switch frame
/// ```
///
/// # Detailed Output
///
/// ```text
///  IDSET    is a SPICE set containing the ID codes of all
///           built-in reference frames of the specified class
///           or classes.
///
///           If IDSET is non-empty on input, its contents will be
///           discarded.
///
///           IDSET must be initialized by the caller via the
///           SPICELIB routine SSIZEI.
/// ```
///
/// # Parameters
///
/// ```text
///  See the INCLUDE file frmtyp.inc.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the input frame class argument is not defined in
///      frmtyp.inc, the error SPICE(BADFRAMECLASS) is signaled.
///
///  2)  If the size of IDSET is too small to hold the requested
///      frame ID set, the error SPICE(SETTOOSMALL) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine has a counterpart
///
///     KPLFRM
///
///  which fetches the frame IDs of all frames specified in the kernel
///  pool.
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
///  1) Display the IDs and names of all SPICE built-in frames.
///     Group the outputs by frame class. Also fetch and display
///     the entire set of IDs and names using the parameter ALL.
///
///     Example code begins here.
///
///
///           PROGRAM BLTFRM_EX1
///           IMPLICIT NONE
///
///           INCLUDE 'ninert.inc'
///           INCLUDE 'nninrt.inc'
///           INCLUDE 'frmtyp.inc'
///     C
///     C     SPICELIB functions
///     C
///           INTEGER               CARDI
///     C
///     C     Local parameters
///     C
///           INTEGER               NFRAME
///           PARAMETER           ( NFRAME = NINERT + NNINRT )
///
///           INTEGER               LBCELL
///           PARAMETER           ( LBCELL = -5 )
///
///           INTEGER               LNSIZE
///           PARAMETER           ( LNSIZE = 80 )
///
///           INTEGER               FRNMLN
///           PARAMETER           ( FRNMLN = 32 )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(FRNMLN)    FRNAME
///           CHARACTER*(LNSIZE)    OUTLIN
///           CHARACTER*(LNSIZE)    VERSN
///
///           INTEGER               I
///           INTEGER               IDSET ( LBCELL : NFRAME )
///           INTEGER               NFRMS
///           INTEGER               J
///
///     C
///     C     Get the Toolkit version number and display it.
///     C
///           CALL TKVRSN ( 'TOOLKIT', VERSN )
///           CALL TOSTDO ( 'Toolkit version: ' // VERSN )
///
///     C
///     C     Initialize the frame set.
///     C
///           CALL SSIZEI ( NFRAME, IDSET )
///
///     C
///     C     Fetch and display the frames of each class.
///     C
///           DO I = 1, 7
///
///              IF ( I .LT. 7 ) THEN
///     C
///     C           Fetch the frames of class I.
///     C
///                 CALL BLTFRM ( I, IDSET )
///
///                 OUTLIN = 'Number of frames of class #: #'
///                 CALL REPMI ( OUTLIN, '#', I,            OUTLIN )
///                 CALL REPMI ( OUTLIN, '#', CARDI(IDSET), OUTLIN )
///
///              ELSE
///     C
///     C           Fetch IDs of all built-in frames.
///     C
///                 CALL BLTFRM ( ALL, IDSET )
///
///                 OUTLIN = 'Number of built-in frames: #'
///                 CALL REPMI ( OUTLIN, '#', CARDI(IDSET), OUTLIN )
///
///              END IF
///
///              CALL TOSTDO ( ' '    )
///              CALL TOSTDO ( OUTLIN )
///              CALL TOSTDO ( '   Frame IDs and names' )
///
///     C
///     C        Display the NAIF ID and name of a maximum of 5 frames
///     C        per family.
///     C
///              NFRMS = MIN( 5, CARDI(IDSET) )
///
///              DO J = 1, NFRMS
///                 CALL FRMNAM ( IDSET(J), FRNAME )
///                 WRITE (*,*) IDSET(J), '  ', FRNAME
///              END DO
///
///           END DO
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     Toolkit version: N0067
///
///     Number of frames of class 1: 21
///        Frame IDs and names
///                1   J2000
///                2   B1950
///                3   FK4
///                4   DE-118
///                5   DE-96
///
///     Number of frames of class 2: 105
///        Frame IDs and names
///            10001   IAU_MERCURY_BARYCENTER
///            10002   IAU_VENUS_BARYCENTER
///            10003   IAU_EARTH_BARYCENTER
///            10004   IAU_MARS_BARYCENTER
///            10005   IAU_JUPITER_BARYCENTER
///
///     Number of frames of class 3: 0
///        Frame IDs and names
///
///     Number of frames of class 4: 1
///        Frame IDs and names
///            10081   EARTH_FIXED
///
///     Number of frames of class 5: 0
///        Frame IDs and names
///
///     Number of frames of class 6: 0
///        Frame IDs and names
///
///     Number of built-in frames: 127
///        Frame IDs and names
///                1   J2000
///                2   B1950
///                3   FK4
///                4   DE-118
///                5   DE-96
///
///
///     Note that the set of built-in frames, particularly the
///     non-inertial ones, will grow over time, so the output
///     shown here may be out of sync with that produced by a
///     current SPICE Toolkit. Only the first 5 frames of each
///     family are presented in the output.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.0.0, 08-AUG-2021 (JDR) (NJB)
///
///         Updated to account for switch frame class.
///
///         Edited the header to comply with NAIF standard. Updated
///         code example to limit the number of frames presented in the
///         output.
///
///         Extended IDSET description to indicate that the set must
///         be declared and initialized before calling this routine and
///         that its contents will be discarded.
///
/// -    SPICELIB Version 1.1.0, 09-AUG-2013 (BVS)
///
///         Updated for changed ZZFDAT calling sequence.
///
/// -    SPICELIB Version 1.0.0, 21-MAY-2012 (NJB)
/// ```
pub fn bltfrm(ctx: &mut SpiceContext, frmcls: i32, idset: &mut [i32]) -> crate::Result<()> {
    BLTFRM(frmcls, idset, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure BLTFRM ( Built-in frame IDs )
pub fn BLTFRM(FRMCLS: i32, IDSET: &mut [i32], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut IDSET = DummyArrayMut::new(IDSET, LBCELL..);

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
    // Built-in frame hashes returned by ZZFDAT.
    //

    //
    // Saved variables
    //
    //
    // Save all variables in order to avoid problems
    // in code translated by f2c.
    //

    //
    // Initial values
    //

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"BLTFRM", ctx)?;
    //
    // The output set starts out empty.
    //
    SCARDI(0, IDSET.as_slice_mut(), ctx)?;

    //
    // On the first pass, fetch all data for the
    // built-in frames.
    //
    if save.PASS1 {
        ZZFDAT(
            NFRAME,
            MAXBFR,
            save.FRNAME.as_arg_mut(),
            save.FCODE.as_slice_mut(),
            save.CENTER.as_slice_mut(),
            save.FCLASS.as_slice_mut(),
            save.FCLSID.as_slice_mut(),
            save.CTRORD.as_slice_mut(),
            save.BNMLST.as_slice_mut(),
            save.BNMPOL.as_slice_mut(),
            save.BNMNMS.as_arg_mut(),
            save.BNMIDX.as_slice_mut(),
            save.BIDLST.as_slice_mut(),
            save.BIDPOL.as_slice_mut(),
            save.BIDIDS.as_slice_mut(),
            save.BIDIDX.as_slice_mut(),
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"BLTFRM", ctx)?;
            return Ok(());
        }

        save.PASS1 = false;
    }

    //
    // Check the input frame class.
    //
    // This block of code must be kept in sync with frmtyp.inc.
    //
    if (((FRMCLS > SWTCH) || (FRMCLS == 0)) || (FRMCLS < ALL)) {
        SETMSG(
            b"Frame class specifier FRMCLS was #; this value is not supported.",
            ctx,
        );
        ERRINT(b"#", FRMCLS, ctx);
        SIGERR(b"SPICE(BADFRAMECLASS)", ctx)?;
        CHKOUT(b"BLTFRM", ctx)?;
        return Ok(());
    }

    //
    // Make sure the set is large enough to hold all of
    // the IDs of the built-in frames.
    //
    if (SIZEI(IDSET.as_slice(), ctx)? < NFRAME) {
        SETMSG(
            b"Frame ID set argument IDSET has size #; required size is at least #.",
            ctx,
        );
        ERRINT(b"#", SIZEI(IDSET.as_slice(), ctx)?, ctx);
        ERRINT(b"#", NFRAME, ctx);
        SIGERR(b"SPICE(SETTOOSMALL)", ctx)?;
        CHKOUT(b"BLTFRM", ctx)?;
        return Ok(());
    }

    //
    // Transfer ID codes of all frames of the specified class
    // to the output set. First, generate an order vector for
    // the ID codes.
    //
    ORDERI(save.FCODE.as_slice(), NFRAME, save.CORDER.as_slice_mut());

    save.TO = 0;

    for I in 1..=NFRAME {
        //
        // Get the index J in the parallel data arrays of
        // the Ith frame, ordered by ID code.
        //
        save.J = save.CORDER[I];

        if ((save.FCLASS[save.J] == FRMCLS) || (FRMCLS == ALL)) {
            //
            // The frame at index J belongs to the
            // requested class. Append the frame's ID
            // code to the set.
            //
            save.TO = (save.TO + 1);
            IDSET[save.TO] = save.FCODE[save.J];
        }
    }

    //
    // Set the cardinality of the output set.
    //
    // Note that we populated the set using an order vector, so sorting
    // the elements is not necessary. We rely on ZZFDAT to not give us
    // duplicate frame specifications.
    //
    SCARDI(save.TO, IDSET.as_slice_mut(), ctx)?;

    CHKOUT(b"BLTFRM", ctx)?;
    Ok(())
}
