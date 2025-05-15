//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const BUFSIZ: i32 = 200;
const WDSIZE: i32 = 32;
const NITEMS: i32 = 14;
const NVARS: i32 = 14;
const NDPS: i32 = 9;
const DBUFSZ: i32 = (NDPS * BUFSIZ);
const NINTS: i32 = 1;
const IBUFSZ: i32 = (NINTS * BUFSIZ);
const LBPOOL: i32 = -5;

struct SaveVars {
    AGENT: Vec<u8>,
    ALT: ActualCharArray,
    ALTNAT: Vec<u8>,
    FRNAME: Vec<u8>,
    IDSTR: Vec<u8>,
    ITEM: ActualCharArray,
    OLDAGT: Vec<u8>,
    NAME: Vec<u8>,
    SPEC: Vec<u8>,
    TYPE: Vec<u8>,
    UNITS: Vec<u8>,
    ANGLES: StackArray<f64, 3>,
    BUFFD: ActualArray2D<f64>,
    MATRIX: StackArray2D<f64, 9>,
    QUATRN: StackArray<f64, 4>,
    QTMP: StackArray<f64, 4>,
    TEMPD: f64,
    AR: i32,
    AT: i32,
    AXES: StackArray<i32, 3>,
    BUFFI: StackArray2D<i32, 200>,
    IDENTS: StackArray2D<i32, 200>,
    IDNT: StackArray<i32, 1>,
    N: i32,
    OLDID: i32,
    POOL: ActualArray2D<i32>,
    R: i32,
    TAIL: i32,
    BUFFRD: bool,
    FIRST: bool,
    FOUND1: bool,
    FOUND2: bool,
    FND: bool,
    FULL: bool,
    UPDATE: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut AGENT = vec![b' '; WDSIZE as usize];
        let mut ALT = ActualCharArray::new(WDSIZE, 1..=NITEMS);
        let mut ALTNAT = vec![b' '; WDSIZE as usize];
        let mut FRNAME = vec![b' '; WDSIZE as usize];
        let mut IDSTR = vec![b' '; WDSIZE as usize];
        let mut ITEM = ActualCharArray::new(WDSIZE, 1..=NVARS);
        let mut OLDAGT = vec![b' '; WDSIZE as usize];
        let mut NAME = vec![b' '; WDSIZE as usize];
        let mut SPEC = vec![b' '; WDSIZE as usize];
        let mut TYPE = vec![b' '; 1 as usize];
        let mut UNITS = vec![b' '; WDSIZE as usize];
        let mut ANGLES = StackArray::<f64, 3>::new(1..=3);
        let mut BUFFD = ActualArray2D::<f64>::new(1..=NDPS, 1..=BUFSIZ);
        let mut MATRIX = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut QUATRN = StackArray::<f64, 4>::new(0..=3);
        let mut QTMP = StackArray::<f64, 4>::new(0..=3);
        let mut TEMPD: f64 = 0.0;
        let mut AR: i32 = 0;
        let mut AT: i32 = 0;
        let mut AXES = StackArray::<i32, 3>::new(1..=3);
        let mut BUFFI = StackArray2D::<i32, 200>::new(1..=NINTS, 1..=BUFSIZ);
        let mut IDENTS = StackArray2D::<i32, 200>::new(1..=1, 1..=BUFSIZ);
        let mut IDNT = StackArray::<i32, 1>::new(1..=1);
        let mut N: i32 = 0;
        let mut OLDID: i32 = 0;
        let mut POOL = ActualArray2D::<i32>::new(1..=2, LBPOOL..=BUFSIZ);
        let mut R: i32 = 0;
        let mut TAIL: i32 = 0;
        let mut BUFFRD: bool = false;
        let mut FIRST: bool = false;
        let mut FOUND1: bool = false;
        let mut FOUND2: bool = false;
        let mut FND: bool = false;
        let mut FULL: bool = false;
        let mut UPDATE: bool = false;

        AT = 0;
        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::D(0.0), DBUFSZ as usize))
                .chain([]);

            BUFFD
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::I(0), IBUFSZ as usize))
                .chain([]);

            BUFFI
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        FIRST = true;
        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::I(0), BUFSIZ as usize))
                .chain([]);

            IDENTS
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            AGENT,
            ALT,
            ALTNAT,
            FRNAME,
            IDSTR,
            ITEM,
            OLDAGT,
            NAME,
            SPEC,
            TYPE,
            UNITS,
            ANGLES,
            BUFFD,
            MATRIX,
            QUATRN,
            QTMP,
            TEMPD,
            AR,
            AT,
            AXES,
            BUFFI,
            IDENTS,
            IDNT,
            N,
            OLDID,
            POOL,
            R,
            TAIL,
            BUFFRD,
            FIRST,
            FOUND1,
            FOUND2,
            FND,
            FULL,
            UPDATE,
        }
    }
}

/// TK frame, find position rotation
///
/// Find the position rotation matrix from a Text Kernel (TK) frame
/// with the specified frame class ID to its base frame.
///
/// # Required Reading
///
/// * [FRAMES](crate::required_reading::frames)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  ----------------------------------------------
///  FRCODE     I   Frame class ID of a TK frame.
///  ROT        O   Rotation matrix from TK frame to frame FRAME.
///  FRAME      O   Frame ID of the base reference.
///  FOUND      O   .TRUE. if the rotation could be determined.
/// ```
///
/// # Detailed Input
///
/// ```text
///  FRCODE   is the unique frame class ID of the TK frame for which
///           data is being requested. For TK frames the frame class
///           ID is always equal to the frame ID.
/// ```
///
/// # Detailed Output
///
/// ```text
///  ROT      is a position rotation matrix that converts positions
///           relative to the TK frame given by its frame class ID,
///           FRCODE, to positions relative to the base frame given by
///           its frame ID, FRAME.
///
///           Thus, if a position S has components x,y,z in the TK
///           frame, then S has components x', y', z' in the base
///           frame.
///
///              .-  -.     .-     -. .- -.
///              | x' |     |       | | x |
///              | y' |  =  |  ROT  | | y |
///              | z' |     |       | | z |
///              `-  -'     `-     -' `- -'
///
///
///  FRAME    is the ID code of the base reference frame to which ROT
///           will transform positions.
///
///  FOUND    is a logical indicating whether or not a frame definition
///           for the TK frame with the frame class ID, FRCODE, was
///           constructed from kernel pool data. If ROT and FRAME were
///           constructed, FOUND will be returned with the value .TRUE.
///           Otherwise it will be returned with the value .FALSE.
/// ```
///
/// # Parameters
///
/// ```text
///  BUFSIZ   is the number of rotation, frame class ID pairs that can
///           have their instance data buffered for the sake of
///           improving run-time performance. This value MUST be
///           positive and should probably be at least 10.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If some kernel variable associated with this frame is not
///      present in the kernel pool, or does not have the proper type
///      or dimension, an error is signaled by a routine in the call
///      tree of this routine. In such a case FOUND will be set to
///      .FALSE.
///
///  2)  If the input FRCODE has the value 0, the error
///      SPICE(ZEROFRAMEID) is signaled. FOUND will be set to .FALSE.
///
///  3)  If the name of the frame corresponding to FRCODE cannot be
///      determined, the error SPICE(INCOMPLETEFRAME) is signaled.
///
///  4)  If the frame given by FRCODE is defined relative to a frame
///      that is unrecognized, the error SPICE(BADFRAMESPEC) is
///      signaled. FOUND will be set to .FALSE.
///
///  5)  If the kernel pool specification for the frame given by
///      FRCODE is not one of 'MATRIX', 'ANGLES' or 'QUATERNION',
///      the error SPICE(UNKNOWNFRAMESPEC) is signaled. FOUND will be
///      set to .FALSE.
///
///  6)  If the frame FRCODE is equal to the relative frame ID (i.e.
///      the frame is defined relative to itself), the error
///      SPICE(BADFRAMESPEC2) is signaled. FOUND will be set to .FALSE.
///
///  7)  If name-based and ID-based forms of any TKFRAME_ keyword
///      are detected in the kernel pool at the same time, the error
///      SPICE(COMPETINGFRAMESPEC) is signaled. FOUND will be set to
///      .FALSE.
/// ```
///
/// # Files
///
/// ```text
///  This routine makes use of the loaded text kernels to determine
///  the rotation from a constant offset TK frame to its base frame.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine is used to construct the rotation from some frame
///  that is a constant rotation offset from some other reference
///  frame. This rotation is derived from data stored in the kernel
///  pool.
///
///  This routine is intended to be used as a low level routine by the
///  frame system software. However, you could use this routine to
///  directly retrieve the rotation from an fixed offset TK frame to
///  its base frame.
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
///  1) Compute the rotation from the DSS-34 topocentric frame to
///     its base Earth body-fixed frame and use it to determine the
///     geodetic latitude and longitude of the DSS-34 site.
///
///
///     Use the FK kernel below to load the required topocentric
///     reference frame definition for the DSS-34 site.
///
///        earth_topo_050714.tf
///
///
///     Example code begins here.
///
///
///           PROGRAM TKFRAM_EX1
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions.
///     C
///           DOUBLE PRECISION      DPR
///
///     C
///     C     Local parameters
///     C
///           CHARACTER*(*)         MYTOPO
///           PARAMETER           ( MYTOPO = 'DSS-34_TOPO' )
///
///           INTEGER               MXFRLN
///           PARAMETER           ( MXFRLN = 26 )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(MXFRLN)    FRNAME
///
///           DOUBLE PRECISION      LAT
///           DOUBLE PRECISION      LON
///           DOUBLE PRECISION      RAD
///           DOUBLE PRECISION      ROT   ( 3, 3 )
///           DOUBLE PRECISION      Z     ( 3    )
///
///           INTEGER               FRAME
///           INTEGER               FRCODE
///
///           LOGICAL               FOUND
///
///     C
///     C     Load the FK that contains the topocentric reference
///     C     frame definition for DSS-34.
///     C
///           CALL FURNSH ( 'earth_topo_050714.tf' )
///
///     C
///     C     The name of the topocentric frame is MYTOPO.
///     C     First we get the ID code of the topocentric frame.
///     C
///           CALL NAMFRM ( MYTOPO, FRCODE )
///
///     C
///     C     Next get the rotation from the topocentric frame to
///     C     the body-fixed frame. We can use the TK frame ID in
///     C     place of the TK frame class ID in this call because
///     C     for TK frames these IDs are identical.
///     C
///           CALL TKFRAM ( FRCODE, ROT, FRAME, FOUND )
///
///     C
///     C     Make sure the topocentric frame is relative to one of
///     C     the Earth fixed frames.
///     C
///           CALL FRMNAM( FRAME, FRNAME )
///
///           IF (       FRNAME .NE. 'IAU_EARTH'
///          .     .AND. FRNAME .NE. 'EARTH_FIXED'
///          .     .AND. FRNAME .NE. 'ITRF93'  ) THEN
///
///              WRITE (*,*) 'The frame ', MYTOPO,
///          .               ' does not appear to be '
///              WRITE (*,*) 'defined relative to an '
///          .            // 'Earth fixed frame.'
///              STOP
///
///           END IF
///
///     C
///     C     Things look ok. Get the location of the Z-axis in the
///     C     topocentric frame.
///     C
///           Z(1) = ROT(1,3)
///           Z(2) = ROT(2,3)
///           Z(3) = ROT(3,3)
///
///     C
///     C     Convert the Z vector to latitude, longitude and radius.
///     C
///           CALL RECLAT ( Z, RAD, LAT, LON )
///
///           WRITE (*,'(A)') 'The geodetic coordinates of the center'
///           WRITE (*,'(A)') 'of the topographic frame are:'
///           WRITE (*,*)
///           WRITE (*,'(A,F20.13)') '   Latitude  (deg): ', LAT*DPR()
///           WRITE (*,'(A,F20.13)') '   Longitude (deg): ', LON*DPR()
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     The geodetic coordinates of the center
///     of the topographic frame are:
///
///        Latitude  (deg):    148.9819650021110
///        Longitude (deg):    -35.3984778756552
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
///  F.S. Turner        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.3.0, 20-AUG-2021 (JDR) (BVS) (NJB)
///
///         BUG FIX: the routine now signals an error if it detects
///         name-based and ID-based forms of any TKFRAME_ keyword present
///         in the POOL at the same time. This prevents name-based
///         keywords from frame definitions loaded with lower priority
///         from being used instead of ID-based keywords from frame
///         definitions loaded with higher priority.
///
///         BUG FIX: when failing to fetch any frame keywords from the
///         POOL or for any other reason, the routine now always returns
///         FOUND = .FALSE. Previously FOUND could be set to .TRUE. by a
///         DTPOOL call preceding the failure.
///
///         BUG FIX: when failing due to a frame defined relative to
///         itself or due to an unrecognized _SPEC, the routine now always
///         returns FRAME = 0. Previously FRAME was set to the _RELATIVE
///         keyword.
///
///         BUG FIX: the misspelled short error message
///         SPICE(INCOMPLETEFRAME) was corrected. The message had been
///         spelled correctly in header comments but not in the code.
///
///         Changed to return ROT as identity for all failures; previously
///         it was returned this way only for some failures.
///
///         Changed the input argument name ID to FRCODE for consistency
///         with other routines.
///
///         Fixed minor typo on the UNKNOWNFRAMESPEC long error message.
///
///         Edited the header to comply with NAIF standard and modern
///         SPICE CK and frames terminology.
///
///         Added complete code example based on existing fragments.
///
///         Construction of kernel variable names now uses trimmed
///         strings in order to suppress gfortran compile warnings.
///
///         Added DATA statements to initialize BUFFI, BUFFD, and IDENTS.
///         This change suppresses ftnchek warnings for variables possibly
///         not initialized before use. It is not a bug fix.
///
///         Minor inline comment typos were corrected.
///
/// -    SPICELIB Version 2.2.0, 08-JAN-2014 (BVS)
///
///         Added an error check for frames defined relative to
///         themselves.
///
///         Increased BUFSIZ from 20 to 200.
///
/// -    SPICELIB Version 2.1.0, 23-APR-2009 (NJB)
///
///         Bug fix: watch is deleted only for frames
///         that are deleted from the buffer.
///
/// -    SPICELIB Version 2.0.0, 19-MAR-2009 (NJB)
///
///         Bug fix: this routine now deletes watches set on
///         kernel variables of frames that are discarded from
///         the local buffering system.
///
/// -    SPICELIB Version 1.2.0, 09-SEP-2005 (NJB)
///
///         Updated to remove non-standard use of duplicate arguments
///         in CONVRT, UCRSS, VHATG and VSCL calls.
///
/// -    SPICELIB Version 1.1.0, 21-NOV-2001 (FST)
///
///         Updated this routine to dump the buffer of frame ID codes
///         it saves when it or one of the modules in its call tree
///         signals an error. This fixes a bug where a frame's ID code is
///         buffered, but the matrix and kernel pool watcher were not set
///         properly.
///
/// -    SPICELIB Version 1.0.0, 18-NOV-1996 (WLT)
/// ```
pub fn tkfram(
    ctx: &mut SpiceContext,
    frcode: i32,
    rot: &mut [[f64; 3]; 3],
    frame: &mut i32,
    found: &mut bool,
) -> crate::Result<()> {
    TKFRAM(
        frcode,
        rot.as_flattened_mut(),
        frame,
        found,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure TKFRAM ( TK frame, find position rotation )
pub fn TKFRAM(
    FRCODE: i32,
    ROT: &mut [f64],
    FRAME: &mut i32,
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut ROT = DummyArrayMut2D::new(ROT, 1..=3, 1..=3);

    //
    // Spicelib Functions
    //

    //
    // Local Parameters
    //

    //
    // Local Variables
    //

    //
    // Saved variables
    //

    //
    // Initial values
    //

    //
    // Programmer's note: this routine makes use of the *implementation*
    // of LOCATI. If that routine is changed, the logic this routine
    // uses to locate buffered, old frame IDs may need to change as well.
    //

    //
    // Before we even check in, if N is less than 1 we can
    // just return.
    //
    //
    // Perform any initializations that might be needed for this
    // routine.
    //
    if save.FIRST {
        save.FIRST = false;

        LNKINI(BUFSIZ, save.POOL.as_slice_mut(), ctx)?;
    }

    //
    // Now do the standard SPICE error handling.  Sure this is
    // a bit unconventional, but nothing will be hurt by doing
    // the stuff above first.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"TKFRAM", ctx)?;

    //
    // So far, we've not FOUND the rotation to the specified frame.
    //
    *FOUND = false;

    //
    // Check the ID to make sure it is non-zero.
    //
    if (FRCODE == 0) {
        LNKINI(BUFSIZ, save.POOL.as_slice_mut(), ctx)?;

        SETMSG(b"Frame identification codes are required to be non-zero.  You\'ve specified a frame with ID value zero. ", ctx);
        SIGERR(b"SPICE(ZEROFRAMEID)", ctx)?;
        CHKOUT(b"TKFRAM", ctx)?;
        return Ok(());
    }

    //
    // Find out whether our linked list pool is already full.
    // We'll use this information later to decide whether we're
    // going to have to delete a watcher.
    //
    save.FULL = (LNKNFN(save.POOL.as_slice()) == 0);

    if save.FULL {
        //
        // If the input frame ID is not buffered, we'll need to
        // overwrite an existing buffer entry. In this case
        // the call to LOCATI we're about to make will overwrite
        // the ID code in the slot we're about to use. We need
        // this ID code, so extract it now while we have the
        // opportunity. The old ID sits at the tail of the list
        // whose head node is AT.
        //
        save.TAIL = LNKTL(save.AT, save.POOL.as_slice(), ctx)?;

        save.OLDID = save.IDENTS[[1, save.TAIL]];

        //
        // Create the name of the agent associated with the old
        // frame.
        //
        fstr::assign(&mut save.OLDAGT, b"TKFRAME_#");
        REPMI(
            &save.OLDAGT.to_vec(),
            b"#",
            save.OLDID,
            &mut save.OLDAGT,
            ctx,
        );
    }

    //
    // Look up the address of the instance data.
    //
    save.IDNT[1] = FRCODE;
    LOCATI(
        save.IDNT.as_slice(),
        1,
        save.IDENTS.as_slice_mut(),
        save.POOL.as_slice_mut(),
        &mut save.AT,
        &mut save.BUFFRD,
        ctx,
    )?;

    if (save.FULL && !save.BUFFRD) {
        //
        // Since the buffer is already full, we'll delete the watcher for
        // the kernel variables associated with OLDID, since there's no
        // longer a need for that watcher.
        //
        // First clear the update status of the old agent; DWPOOL won't
        // delete an agent with a unchecked update.
        //
        CVPOOL(&save.OLDAGT, &mut save.UPDATE, ctx)?;
        DWPOOL(&save.OLDAGT, ctx)?;
    }

    //
    // Until we have better information we put the identity matrix
    // into the output rotation and set FRAME to zero.
    //
    IDENT(ROT.as_slice_mut());
    *FRAME = 0;

    //
    // If we have to look up the data for our frame, we do
    // it now and perform any conversions and computations that
    // will be needed when it's time to convert coordinates to
    // directions.
    //
    // Construct the name of the agent associated with the
    // requested frame.  (Each frame has its own agent).
    //
    INTSTR(FRCODE, &mut save.IDSTR, ctx);
    FRMNAM(FRCODE, &mut save.FRNAME, ctx)?;

    if fstr::eq(&save.FRNAME, b" ") {
        LNKINI(BUFSIZ, save.POOL.as_slice_mut(), ctx)?;

        SETMSG(
            b"The Text Kernel (TK) frame with ID code # does not have a recognized name. ",
            ctx,
        );
        ERRINT(b"#", FRCODE, ctx);
        SIGERR(b"SPICE(INCOMPLETEFRAME)", ctx)?;
        CHKOUT(b"TKFRAM", ctx)?;
        return Ok(());
    }

    fstr::assign(
        &mut save.AGENT,
        &fstr::concat(
            b"TKFRAME_",
            fstr::substr(&save.IDSTR, 1..=RTRIM(&save.IDSTR)),
        ),
    );
    save.R = RTRIM(&save.AGENT);

    fstr::assign(
        &mut save.ALTNAT,
        &fstr::concat(
            b"TKFRAME_",
            fstr::substr(&save.FRNAME, 1..=RTRIM(&save.FRNAME)),
        ),
    );
    save.AR = RTRIM(&save.ALTNAT);

    //
    // If the frame is buffered, we check the kernel pool to
    // see if there has been an update to this frame.
    //
    if save.BUFFRD {
        CVPOOL(fstr::substr(&save.AGENT, 1..=save.R), &mut save.UPDATE, ctx)?;
    } else {
        //
        // If the frame is not buffered we definitely need to update
        // things.

        save.UPDATE = true;
    }

    if !save.UPDATE {
        //
        // Just look up the rotation matrix and relative-to
        // information from the local buffer.
        //
        ROT[[1, 1]] = save.BUFFD[[1, save.AT]];
        ROT[[2, 1]] = save.BUFFD[[2, save.AT]];
        ROT[[3, 1]] = save.BUFFD[[3, save.AT]];
        ROT[[1, 2]] = save.BUFFD[[4, save.AT]];
        ROT[[2, 2]] = save.BUFFD[[5, save.AT]];
        ROT[[3, 2]] = save.BUFFD[[6, save.AT]];
        ROT[[1, 3]] = save.BUFFD[[7, save.AT]];
        ROT[[2, 3]] = save.BUFFD[[8, save.AT]];
        ROT[[3, 3]] = save.BUFFD[[9, save.AT]];

        *FRAME = save.BUFFI[[1, save.AT]];
    } else {
        //
        // Determine how the frame is specified and what it
        // is relative to.  The variables that specify
        // how the frame is represented and what it is relative to
        // are TKFRAME_#_SPEC and TKFRAME_#_RELATIVE where # is
        // replaced by the text value of ID or the frame name.
        //
        fstr::assign(
            save.ITEM.get_mut(1),
            &fstr::concat(fstr::substr(&save.AGENT, 1..=save.R), b"_SPEC"),
        );
        fstr::assign(
            save.ITEM.get_mut(2),
            &fstr::concat(fstr::substr(&save.AGENT, 1..=save.R), b"_RELATIVE"),
        );

        fstr::assign(
            save.ALT.get_mut(1),
            &fstr::concat(fstr::substr(&save.ALTNAT, 1..=save.AR), b"_SPEC"),
        );
        fstr::assign(
            save.ALT.get_mut(2),
            &fstr::concat(fstr::substr(&save.ALTNAT, 1..=save.AR), b"_RELATIVE"),
        );
        //
        // See if the friendlier version of the kernel pool variables
        // are available.
        //
        // If both forms are present, we signal an error.
        //
        for I in 1..=2 {
            DTPOOL(
                &save.ITEM[I],
                &mut save.FOUND1,
                &mut save.N,
                &mut save.TYPE,
                ctx,
            )?;
            DTPOOL(
                &save.ALT[I],
                &mut save.FOUND2,
                &mut save.N,
                &mut save.TYPE,
                ctx,
            )?;

            if (save.FOUND1 && save.FOUND2) {
                LNKINI(BUFSIZ, save.POOL.as_slice_mut(), ctx)?;
                *FRAME = 0;
                IDENT(ROT.as_slice_mut());
                SETMSG(b"Frame name-based and frame ID-based text kernel (fixed-offset) frame definition keywords \'#\' and \'#\' are both present in the POOL. Most likely this is because loaded text kernels contain competing definitions of the \'#\' frame using different keyword styles, which is not allowed. ", ctx);
                ERRCH(b"#", &save.ITEM[I], ctx);
                ERRCH(b"#", &save.ALT[I], ctx);
                ERRCH(b"#", &save.FRNAME, ctx);
                SIGERR(b"SPICE(COMPETINGFRAMESPEC)", ctx)?;
                CHKOUT(b"TKFRAM", ctx)?;
                return Ok(());
            }

            if save.FOUND2 {
                fstr::assign(save.ITEM.get_mut(I), save.ALT.get(I));
            }
        }

        //
        // If either the SPEC or RELATIVE frame are missing from
        // the kernel pool, we simply return.
        //
        if (BADKPV(b"TKFRAM", &save.ITEM[1], b"=", 1, 1, b"C", ctx)?
            || BADKPV(b"TKFRAM", &save.ITEM[2], b"=", 1, 1, b"C", ctx)?)
        {
            LNKINI(BUFSIZ, save.POOL.as_slice_mut(), ctx)?;
            *FRAME = 0;
            IDENT(ROT.as_slice_mut());
            CHKOUT(b"TKFRAM", ctx)?;
            return Ok(());
        }

        //
        // If we make it this far, look up the SPEC and RELATIVE frame.
        //
        GCPOOL(
            &save.ITEM[1],
            1,
            1,
            &mut save.N,
            CharArrayMut::from_mut(&mut save.SPEC),
            &mut save.FND,
            ctx,
        )?;
        GCPOOL(
            &save.ITEM[2],
            1,
            1,
            &mut save.N,
            CharArrayMut::from_mut(&mut save.NAME),
            &mut save.FND,
            ctx,
        )?;

        //
        // Look up the ID code for this frame.
        //
        NAMFRM(&save.NAME, FRAME, ctx)?;

        if (*FRAME == 0) {
            LNKINI(BUFSIZ, save.POOL.as_slice_mut(), ctx)?;
            IDENT(ROT.as_slice_mut());
            SETMSG(b"The frame to which frame # is relatively defined is not recognized. The kernel pool specification of the relative frame is \'#\'.  This is not a recognized frame. ", ctx);
            ERRINT(b"#", FRCODE, ctx);
            ERRCH(b"#", &save.NAME, ctx);
            SIGERR(b"SPICE(BADFRAMESPEC)", ctx)?;
            CHKOUT(b"TKFRAM", ctx)?;
            return Ok(());
        }

        //
        // Make sure that the RELATIVE frame ID is distinct from the
        // frame ID. If they are the same, SPICE will go into an
        // indefinite loop.
        //
        if (*FRAME == FRCODE) {
            LNKINI(BUFSIZ, save.POOL.as_slice_mut(), ctx)?;
            *FRAME = 0;
            IDENT(ROT.as_slice_mut());
            SETMSG(b"Bad fixed offset frame specification: the frame \'#\' (frame ID #) is defined relative to itself. SPICE cannot work with such frames. ", ctx);
            ERRCH(b"#", &save.NAME, ctx);
            ERRINT(b"#", FRCODE, ctx);
            SIGERR(b"SPICE(BADFRAMESPEC2)", ctx)?;
            CHKOUT(b"TKFRAM", ctx)?;
            return Ok(());
        }

        //
        // Convert SPEC to upper case so that we can easily check
        // to see if this is one of the expected specification types.
        //
        UCASE(&save.SPEC.to_vec(), &mut save.SPEC, ctx);

        if fstr::eq(&save.SPEC, b"MATRIX") {
            //
            // This is the easiest case.  Just grab the matrix
            // from the kernel pool (and polish it up a bit just
            // to make sure we have a rotation matrix).
            //
            // We give preference to the kernel pool variable
            // TKFRAME_<name>_MATRIX if it is available.
            //
            // If both forms are present, we signal an error.
            //
            fstr::assign(
                save.ITEM.get_mut(3),
                &fstr::concat(fstr::substr(&save.AGENT, 1..=save.R), b"_MATRIX"),
            );
            fstr::assign(
                save.ALT.get_mut(3),
                &fstr::concat(fstr::substr(&save.ALTNAT, 1..=save.AR), b"_MATRIX"),
            );

            DTPOOL(
                &save.ITEM[3],
                &mut save.FOUND1,
                &mut save.N,
                &mut save.TYPE,
                ctx,
            )?;
            DTPOOL(
                &save.ALT[3],
                &mut save.FOUND2,
                &mut save.N,
                &mut save.TYPE,
                ctx,
            )?;

            if (save.FOUND1 && save.FOUND2) {
                LNKINI(BUFSIZ, save.POOL.as_slice_mut(), ctx)?;
                *FRAME = 0;
                IDENT(ROT.as_slice_mut());
                SETMSG(b"Frame name-based and frame ID-based text kernel (fixed-offset) frame definition keywords \'#\' and \'#\' are both present in the POOL. Most likely this is because loaded text kernels contain competing definitions of the \'#\' frame using different keyword styles, which is not allowed. ", ctx);
                ERRCH(b"#", &save.ITEM[3], ctx);
                ERRCH(b"#", &save.ALT[3], ctx);
                ERRCH(b"#", &save.FRNAME, ctx);
                SIGERR(b"SPICE(COMPETINGFRAMESPEC)", ctx)?;
                CHKOUT(b"TKFRAM", ctx)?;
                return Ok(());
            }

            if save.FOUND2 {
                fstr::assign(save.ITEM.get_mut(3), save.ALT.get(3));
            }

            if BADKPV(b"TKFRAM", &save.ITEM[3], b"=", 9, 1, b"N", ctx)? {
                LNKINI(BUFSIZ, save.POOL.as_slice_mut(), ctx)?;
                *FRAME = 0;
                IDENT(ROT.as_slice_mut());
                CHKOUT(b"TKFRAM", ctx)?;
                return Ok(());
            }

            //
            // The variable meets current expectations, look it up
            // from the kernel pool.
            //
            GDPOOL(
                &save.ITEM[3],
                1,
                9,
                &mut save.N,
                save.MATRIX.as_slice_mut(),
                &mut save.FND,
                ctx,
            )?;

            //
            // In this case the full transformation matrix has been
            // specified.  We simply polish it up a bit.
            //
            MOVED(save.MATRIX.as_slice(), 9, ROT.as_slice_mut());
            SHARPR(ROT.as_slice_mut());

            //
            // The matrix might not be right-handed, so correct
            // the sense of the second and third columns if necessary.
            //
            if (VDOT(ROT.subarray([1, 2]), save.MATRIX.subarray([1, 2])) < 0.0) {
                VSCLIP(-1.0, ROT.subarray_mut([1, 2]));
            }

            if (VDOT(ROT.subarray([1, 3]), save.MATRIX.subarray([1, 3])) < 0.0) {
                VSCLIP(-1.0, ROT.subarray_mut([1, 3]));
            }
        } else if fstr::eq(&save.SPEC, b"ANGLES") {
            //
            // Look up the angles, their units and axes for the
            // frame specified by ID. (Note that UNITS are optional).
            // As in the previous case we give preference to the
            // form TKFRAME_<name>_<item> over TKFRAME_<id>_<item>.
            //
            fstr::assign(
                save.ITEM.get_mut(3),
                &fstr::concat(fstr::substr(&save.AGENT, 1..=save.R), b"_ANGLES"),
            );
            fstr::assign(
                save.ITEM.get_mut(4),
                &fstr::concat(fstr::substr(&save.AGENT, 1..=save.R), b"_AXES"),
            );
            fstr::assign(
                save.ITEM.get_mut(5),
                &fstr::concat(fstr::substr(&save.AGENT, 1..=save.R), b"_UNITS"),
            );

            fstr::assign(
                save.ALT.get_mut(3),
                &fstr::concat(fstr::substr(&save.ALTNAT, 1..=save.AR), b"_ANGLES"),
            );
            fstr::assign(
                save.ALT.get_mut(4),
                &fstr::concat(fstr::substr(&save.ALTNAT, 1..=save.AR), b"_AXES"),
            );
            fstr::assign(
                save.ALT.get_mut(5),
                &fstr::concat(fstr::substr(&save.ALTNAT, 1..=save.AR), b"_UNITS"),
            );
            //
            // Again, we give preference to the more friendly form
            // of TKFRAME specification.
            //
            // If both forms are present, we signal an error.
            //
            for I in 3..=5 {
                DTPOOL(
                    &save.ITEM[I],
                    &mut save.FOUND1,
                    &mut save.N,
                    &mut save.TYPE,
                    ctx,
                )?;
                DTPOOL(
                    &save.ALT[I],
                    &mut save.FOUND2,
                    &mut save.N,
                    &mut save.TYPE,
                    ctx,
                )?;

                if (save.FOUND1 && save.FOUND2) {
                    LNKINI(BUFSIZ, save.POOL.as_slice_mut(), ctx)?;
                    *FRAME = 0;
                    IDENT(ROT.as_slice_mut());
                    SETMSG(b"Frame name-based and frame ID-based text kernel (fixed-offset) frame definition keywords \'#\' and \'#\' are both present in the POOL. Most likely this is because loaded text kernels contain competing definitions of the \'#\' frame using different keyword styles, which is not allowed. ", ctx);
                    ERRCH(b"#", &save.ITEM[I], ctx);
                    ERRCH(b"#", &save.ALT[I], ctx);
                    ERRCH(b"#", &save.FRNAME, ctx);
                    SIGERR(b"SPICE(COMPETINGFRAMESPEC)", ctx)?;
                    CHKOUT(b"TKFRAM", ctx)?;
                    return Ok(());
                }

                if save.FOUND2 {
                    fstr::assign(save.ITEM.get_mut(I), save.ALT.get(I));
                }
            }

            if (BADKPV(b"TKFRAM", &save.ITEM[3], b"=", 3, 1, b"N", ctx)?
                || BADKPV(b"TKFRAM", &save.ITEM[4], b"=", 3, 1, b"N", ctx)?)
            {
                LNKINI(BUFSIZ, save.POOL.as_slice_mut(), ctx)?;
                *FRAME = 0;
                IDENT(ROT.as_slice_mut());
                CHKOUT(b"TKFRAM", ctx)?;
                return Ok(());
            }

            fstr::assign(&mut save.UNITS, b"RADIANS");

            GDPOOL(
                &save.ITEM[3],
                1,
                3,
                &mut save.N,
                save.ANGLES.as_slice_mut(),
                &mut save.FND,
                ctx,
            )?;
            GIPOOL(
                &save.ITEM[4],
                1,
                3,
                &mut save.N,
                save.AXES.as_slice_mut(),
                &mut save.FND,
                ctx,
            )?;
            GCPOOL(
                &save.ITEM[5],
                1,
                1,
                &mut save.N,
                CharArrayMut::from_mut(&mut save.UNITS),
                &mut save.FND,
                ctx,
            )?;

            //
            // Convert angles to radians.
            //
            for I in 1..=3 {
                CONVRT(
                    save.ANGLES[I],
                    &save.UNITS,
                    b"RADIANS",
                    &mut save.TEMPD,
                    ctx,
                )?;
                save.ANGLES[I] = save.TEMPD;
            }

            //
            // Compute the rotation from instrument frame to CK frame.
            //
            EUL2M(
                save.ANGLES[1],
                save.ANGLES[2],
                save.ANGLES[3],
                save.AXES[1],
                save.AXES[2],
                save.AXES[3],
                ROT.as_slice_mut(),
                ctx,
            )?;

            if FAILED(ctx) {
                LNKINI(BUFSIZ, save.POOL.as_slice_mut(), ctx)?;
                *FRAME = 0;
                IDENT(ROT.as_slice_mut());
                CHKOUT(b"TKFRAM", ctx)?;
                return Ok(());
            }
        } else if fstr::eq(&save.SPEC, b"QUATERNION") {
            //
            // Look up the quaternion and convert it to a rotation
            // matrix. Again there are two possible variables that
            // may point to the quaternion. We give preference to
            // the form TKFRAME_<name>_Q over the form TKFRAME_<id>_Q.
            //
            // If both forms are present, we signal an error.
            //
            fstr::assign(
                save.ITEM.get_mut(3),
                &fstr::concat(fstr::substr(&save.AGENT, 1..=save.R), b"_Q"),
            );
            fstr::assign(
                save.ALT.get_mut(3),
                &fstr::concat(fstr::substr(&save.ALTNAT, 1..=save.AR), b"_Q"),
            );

            DTPOOL(
                &save.ITEM[3],
                &mut save.FOUND1,
                &mut save.N,
                &mut save.TYPE,
                ctx,
            )?;
            DTPOOL(
                &save.ALT[3],
                &mut save.FOUND2,
                &mut save.N,
                &mut save.TYPE,
                ctx,
            )?;

            if (save.FOUND1 && save.FOUND2) {
                LNKINI(BUFSIZ, save.POOL.as_slice_mut(), ctx)?;
                *FRAME = 0;
                IDENT(ROT.as_slice_mut());
                SETMSG(b"Frame name-based and frame ID-based text kernel (fixed-offset) frame definition keywords \'#\' and \'#\' are both present in the POOL. Most likely this is because loaded text kernels contain competing definitions of the \'#\' frame using different keyword styles, which is not allowed. ", ctx);
                ERRCH(b"#", &save.ITEM[3], ctx);
                ERRCH(b"#", &save.ALT[3], ctx);
                ERRCH(b"#", &save.FRNAME, ctx);
                SIGERR(b"SPICE(COMPETINGFRAMESPEC)", ctx)?;
                CHKOUT(b"TKFRAM", ctx)?;
                return Ok(());
            }

            if save.FOUND2 {
                fstr::assign(save.ITEM.get_mut(3), save.ALT.get(3));
            }

            if BADKPV(b"TKFRAM", &save.ITEM[3], b"=", 4, 1, b"N", ctx)? {
                LNKINI(BUFSIZ, save.POOL.as_slice_mut(), ctx)?;
                *FRAME = 0;
                IDENT(ROT.as_slice_mut());
                CHKOUT(b"TKFRAM", ctx)?;
                return Ok(());
            }
            //
            // In this case we have the quaternion representation.
            // Again, we do a small amount of polishing of the input.
            //
            GDPOOL(
                &save.ITEM[3],
                1,
                4,
                &mut save.N,
                save.QUATRN.as_slice_mut(),
                &mut save.FND,
                ctx,
            )?;
            VHATG(save.QUATRN.as_slice(), 4, save.QTMP.as_slice_mut());
            Q2M(save.QTMP.as_slice(), ROT.as_slice_mut());
        } else {
            //
            // We don't recognize the SPEC for this frame.  Say
            // so.  Also note that perhaps the user needs to upgrade
            // the toolkit.
            //
            LNKINI(BUFSIZ, save.POOL.as_slice_mut(), ctx)?;
            *FRAME = 0;
            IDENT(ROT.as_slice_mut());
            SETMSG(b"The frame specification \"# = \'#\'\" is not one of the recognized means of specifying a text-kernel constant offset frame. This may reflect a typographical error or may indicate that you need to consider updating your version of the SPICE toolkit. ", ctx);

            ERRCH(b"#", &save.ITEM[1], ctx);
            ERRCH(b"#", &save.SPEC, ctx);
            SIGERR(b"SPICE(UNKNOWNFRAMESPEC)", ctx)?;
            CHKOUT(b"TKFRAM", ctx)?;
            return Ok(());
        }

        //
        // Buffer the identifier, relative frame and rotation matrix.
        //
        save.BUFFD[[1, save.AT]] = ROT[[1, 1]];
        save.BUFFD[[2, save.AT]] = ROT[[2, 1]];
        save.BUFFD[[3, save.AT]] = ROT[[3, 1]];
        save.BUFFD[[4, save.AT]] = ROT[[1, 2]];
        save.BUFFD[[5, save.AT]] = ROT[[2, 2]];
        save.BUFFD[[6, save.AT]] = ROT[[3, 2]];
        save.BUFFD[[7, save.AT]] = ROT[[1, 3]];
        save.BUFFD[[8, save.AT]] = ROT[[2, 3]];
        save.BUFFD[[9, save.AT]] = ROT[[3, 3]];

        save.BUFFI[[1, save.AT]] = *FRAME;

        //
        // If these were not previously buffered, we need to set
        // a watch on the various items that might be used to define
        // this frame.
        //
        if !save.BUFFRD {
            //
            // Immediately check for an update so that we will
            // not redundantly look for this item the next time this
            // routine is called.
            //
            fstr::assign(
                save.ITEM.get_mut(1),
                &fstr::concat(fstr::substr(&save.AGENT, 1..=save.R), b"_RELATIVE"),
            );
            fstr::assign(
                save.ITEM.get_mut(2),
                &fstr::concat(fstr::substr(&save.AGENT, 1..=save.R), b"_SPEC"),
            );
            fstr::assign(
                save.ITEM.get_mut(3),
                &fstr::concat(fstr::substr(&save.AGENT, 1..=save.R), b"_AXES"),
            );
            fstr::assign(
                save.ITEM.get_mut(4),
                &fstr::concat(fstr::substr(&save.AGENT, 1..=save.R), b"_MATRIX"),
            );
            fstr::assign(
                save.ITEM.get_mut(5),
                &fstr::concat(fstr::substr(&save.AGENT, 1..=save.R), b"_Q"),
            );
            fstr::assign(
                save.ITEM.get_mut(6),
                &fstr::concat(fstr::substr(&save.AGENT, 1..=save.R), b"_ANGLES"),
            );
            fstr::assign(
                save.ITEM.get_mut(7),
                &fstr::concat(fstr::substr(&save.AGENT, 1..=save.R), b"_UNITS"),
            );

            fstr::assign(
                save.ITEM.get_mut(8),
                &fstr::concat(fstr::substr(&save.ALTNAT, 1..=save.AR), b"_RELATIVE"),
            );
            fstr::assign(
                save.ITEM.get_mut(9),
                &fstr::concat(fstr::substr(&save.ALTNAT, 1..=save.AR), b"_SPEC"),
            );
            fstr::assign(
                save.ITEM.get_mut(10),
                &fstr::concat(fstr::substr(&save.ALTNAT, 1..=save.AR), b"_AXES"),
            );
            fstr::assign(
                save.ITEM.get_mut(11),
                &fstr::concat(fstr::substr(&save.ALTNAT, 1..=save.AR), b"_MATRIX"),
            );
            fstr::assign(
                save.ITEM.get_mut(12),
                &fstr::concat(fstr::substr(&save.ALTNAT, 1..=save.AR), b"_Q"),
            );
            fstr::assign(
                save.ITEM.get_mut(13),
                &fstr::concat(fstr::substr(&save.ALTNAT, 1..=save.AR), b"_ANGLES"),
            );
            fstr::assign(
                save.ITEM.get_mut(14),
                &fstr::concat(fstr::substr(&save.ALTNAT, 1..=save.AR), b"_UNITS"),
            );

            SWPOOL(&save.AGENT, 14, save.ITEM.as_arg(), ctx)?;
            CVPOOL(&save.AGENT, &mut save.UPDATE, ctx)?;
        }
    }

    if FAILED(ctx) {
        LNKINI(BUFSIZ, save.POOL.as_slice_mut(), ctx)?;
        *FRAME = 0;
        IDENT(ROT.as_slice_mut());
        CHKOUT(b"TKFRAM", ctx)?;
        return Ok(());
    }

    //
    // All errors cause the routine to exit before we get to this
    // point.  If we reach this point we didn't have an error and
    // hence did find the rotation from ID to FRAME.
    //
    *FOUND = true;

    //
    // That's it
    //
    CHKOUT(b"TKFRAM", ctx)?;
    Ok(())
}
