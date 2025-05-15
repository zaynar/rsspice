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
const FRNMLN: i32 = 32;
const KVNMLN: i32 = 32;
const BUFSIZ: i32 = 100;

/// Kernel pool frame IDs
///
/// Return a SPICE set containing the frame IDs of all reference
/// frames of a given class having specifications in the kernel pool.
///
/// # Required Reading
///
/// * [CELLS](crate::required_reading::cells)
/// * [FRAMES](crate::required_reading::frames)
/// * [KERNEL](crate::required_reading::kernel)
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
///           classes for which frame ID codes are requested.
///           The applicable reference frames are those having
///           specifications present in the kernel pool.
///
///           FRMCLS may designate a single class or "all
///           classes."
///
///           The include file frmtyp.inc declares parameters
///           identifying frame classes. The supported values
///           and corresponding meanings of FRMCLS are
///
///              Parameter      Value    Meaning
///              =========      =====    =================
///              ALL              -1     All frame classes
///                                      specified in the
///                                      kernel pool. Class 1
///                                      is not included.
///
///              INERTL            1     Built-in inertial.
///                                      No frames will be
///                                      returned in the
///                                      output set.
///
///              PCK               2     PCK-based frame
///
///              CK                3     CK-based frame
///
///              TK                4     Fixed rotational
///                                      offset ("text
///                                      kernel") frame
///
///              DYN               5     Dynamic frame
///
///              SWTCH             6     Switch frame
/// ```
///
/// # Detailed Output
///
/// ```text
///  IDSET    is a SPICE set containing the ID codes of all
///           reference frames having specifications present in
///           the kernel pool and belonging to the specified
///           class or classes.
///
///           Note that if FRMCLS is set to INERTL, IDSET
///           will be empty on output.
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
///  2)  If the size of IDSET is too small to hold the requested frame
///      ID set, the error SPICE(SETTOOSMALL) is signaled.
///
///  3)  Frames of class 1 may not be specified in the kernel pool.
///      However, for the convenience of users, this routine does not
///      signal an error if the input class is set to INERTL. In this
///      case the output set will be empty.
///
///  4)  This routine relies on the presence of just three kernel
///      variable assignments for a reference frame in order to
///      determine that that reference frame has been specified:
///
///         FRAME_<frame name>       = <ID code>
///         FRAME_<ID code>_NAME     = <frame name>
///
///      and either
///
///         FRAME_<ID code>_CLASS    = <class>
///
///      or
///
///         FRAME_<frame name>_CLASS = <class>
///
///      It is possible for the presence of an incomplete frame
///      specification to trick this routine into incorrectly
///      deciding that a frame has been specified. This routine
///      does not attempt to diagnose this problem.
/// ```
///
/// # Files
///
/// ```text
///  Reference frame specifications for frames that are not built in
///  are typically established by loading frame kernels.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine enables SPICE-based applications to conveniently
///  find the frame ID codes of reference frames having specifications
///  present in the kernel pool. Such frame specifications are
///  introduced into the kernel pool either by loading frame kernels
///  or by means of calls to the kernel pool "put" API routines
///
///     PCPOOL
///     PDPOOL
///     PIPOOL
///
///  Given a reference frame's ID code, other attributes of the
///  frame can be obtained via calls to the SPICELIB routines
///
///     FRMNAM {Return a frame's name}
///     FRINFO {Return a frame's center, class, and class ID}
///
///  This routine has a counterpart
///
///     BLTFRM
///
///  which fetches the frame IDs of all built-in reference frames.
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
///  1) Display the IDs and names of all reference frames having
///     specifications present in the kernel pool. Group the outputs
///     by frame class. Also fetch and display the entire set of IDs
///     and names using the parameter ALL.
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File: kplfrm_ex1.tm
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
///           File name            Contents
///           --------------       --------------------------
///           clem_v20.tf          Clementine FK
///           moon_060721.tf       Generic Lunar SPICE frames
///
///
///        \begindata
///
///           KERNELS_TO_LOAD = ( 'clem_v20.tf'
///                               'moon_060721.tf' )
///        \begintext
///
///        End of meta-kernel
///
///
///     Example code begins here.
///
///
///           PROGRAM KPLFRM_EX1
///           IMPLICIT NONE
///
///           INCLUDE 'frmtyp.inc'
///     C
///     C     SPICELIB functions
///     C
///           INTEGER               CARDI
///     C
///     C     Local parameters
///     C
///           CHARACTER*(*)         META
///           PARAMETER           ( META   = 'kplfrm_ex1.tm' )
///
///           INTEGER               NFRAME
///           PARAMETER           ( NFRAME = 1000 )
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
///
///           INTEGER               I
///           INTEGER               IDSET ( LBCELL : NFRAME )
///           INTEGER               J
///
///     C
///     C     Initialize the frame set.
///     C
///           CALL SSIZEI ( NFRAME, IDSET )
///
///     C
///     C     Load kernels that contain frame specifications.
///     C
///           CALL FURNSH ( META )
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
///                 CALL KPLFRM ( I, IDSET )
///
///                 OUTLIN = 'Number of frames of class #: #'
///                 CALL REPMI ( OUTLIN, '#', I,            OUTLIN )
///                 CALL REPMI ( OUTLIN, '#', CARDI(IDSET), OUTLIN )
///
///              ELSE
///     C
///     C           Fetch IDs of all frames specified in the kernel
///     C           pool.
///     C
///                 CALL KPLFRM ( ALL, IDSET )
///
///                 OUTLIN = 'Number of frames in the kernel pool: #'
///                 CALL REPMI ( OUTLIN, '#', CARDI(IDSET), OUTLIN )
///
///              END IF
///
///              CALL TOSTDO ( ' '    )
///              CALL TOSTDO ( OUTLIN )
///              CALL TOSTDO ( '   Frame IDs and names' )
///
///              DO J = 1, CARDI(IDSET)
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
///     Number of frames of class 1: 0
///        Frame IDs and names
///
///     Number of frames of class 2: 1
///        Frame IDs and names
///            31002   MOON_PA_DE403
///
///     Number of frames of class 3: 1
///        Frame IDs and names
///           -40000   CLEM_SC_BUS
///
///     Number of frames of class 4: 11
///        Frame IDs and names
///           -40008   CLEM_CPT
///           -40007   CLEM_BSTAR
///           -40006   CLEM_ASTAR
///           -40005   CLEM_LIDAR
///           -40004   CLEM_LWIR
///           -40003   CLEM_NIR
///           -40002   CLEM_UVVIS
///           -40001   CLEM_HIRES
///            31000   MOON_PA
///            31001   MOON_ME
///            31003   MOON_ME_DE403
///
///     Number of frames of class 5: 0
///        Frame IDs and names
///
///     Number of frames of class 6: 0
///        Frame IDs and names
///
///     Number of frames in the kernel pool: 13
///        Frame IDs and names
///           -40008   CLEM_CPT
///           -40007   CLEM_BSTAR
///           -40006   CLEM_ASTAR
///           -40005   CLEM_LIDAR
///           -40004   CLEM_LWIR
///           -40003   CLEM_NIR
///           -40002   CLEM_UVVIS
///           -40001   CLEM_HIRES
///           -40000   CLEM_SC_BUS
///            31000   MOON_PA
///            31001   MOON_ME
///            31002   MOON_PA_DE403
///            31003   MOON_ME_DE403
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  This routine will work correctly if the kernel pool contains
///      no invalid frame specifications. See the description of
///      exception 4 above. Users must ensure that no invalid frame
///      specifications are introduced into the kernel pool, either by
///      loaded kernels or by means of the kernel pool "put" APIs.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.0.0, 08-AUG-2021 (JDR) (NJB)
///
///         Updated to account for switch frame class.
///
///         Edited the header to comply with NAIF standard.
///
///         Updated Example's kernels set to use Clementine PDS archived
///         data.
///
/// -    SPICELIB Version 1.1.0, 18-JUN-2015 (NJB)
///
///         Bug fix: previous algorithm failed if the number of frame
///         names fetched from the kernel pool on a single call exceeded
///         twice the kernel variable buffer size. The count of
///         fetched names is now maintained correctly.
///
/// -    SPICELIB Version 1.0.0, 22-MAY-2012 (NJB)
/// ```
pub fn kplfrm(ctx: &mut SpiceContext, frmcls: i32, idset: &mut [i32]) -> crate::Result<()> {
    KPLFRM(frmcls, idset, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure KPLFRM ( Kernel pool frame IDs )
pub fn KPLFRM(FRMCLS: i32, IDSET: &mut [i32], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut IDSET = DummyArrayMut::new(IDSET, LBCELL..);
    let mut FRNAME = [b' '; FRNMLN as usize];
    let mut KVBUFF = ActualCharArray::new(KVNMLN, 1..=BUFSIZ);
    let mut KVNAME = [b' '; KVNMLN as usize];
    let mut KVTEMP = [b' '; KVNMLN as usize];
    let mut KVCODE = [b' '; KVNMLN as usize];
    let mut KVCLAS = [b' '; KVNMLN as usize];
    let mut TMPNAM = [b' '; FRNMLN as usize];
    let mut FCLASS: i32 = 0;
    let mut IDCODE: i32 = 0;
    let mut L: i32 = 0;
    let mut M: i32 = 0;
    let mut N: i32 = 0;
    let mut TO: i32 = 0;
    let mut TOTAL: i32 = 0;
    let mut W: i32 = 0;
    let mut FOUND: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Local variables
    //

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"KPLFRM", ctx)?;

    //
    // The output set starts out empty.
    //
    SCARDI(0, IDSET.as_slice_mut(), ctx)?;

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
        CHKOUT(b"KPLFRM", ctx)?;
        return Ok(());
    }

    //
    // Initialize the output buffer index. The
    // index is to be incremented prior to each
    // write to the buffer.
    //
    TO = 0;

    //
    // Find all of the kernel variables having names
    // that could correspond to frame name assignments.
    //
    // We expect that all frame specifications will
    // include assignments of the form
    //
    //     FRAME_<ID code>_NAME = <frame name>
    //
    // We may pick up some additional assignments that are not part of
    // frame specifications; we plan to filter out as many as possible
    // by looking the corresponding frame ID and frame class
    // assignments.
    //
    fstr::assign(&mut KVTEMP, b"FRAME_*_NAME");

    GNPOOL(
        &KVTEMP,
        1,
        BUFSIZ,
        &mut N,
        KVBUFF.as_arg_mut(),
        &mut FOUND,
        ctx,
    )?;

    TOTAL = 0;

    while (N > 0) {
        TOTAL = (TOTAL + N);

        //
        // At least one kernel variable was found by the last
        // GNPOOL call. Each of these variables is a possible
        // frame name. Look up each of these candidate names.
        //
        for I in 1..=N {
            //
            // Attempt to fetch the right hand side value for
            // the Ith kernel variable found on the previous
            // GNPOOL call.
            //
            GCPOOL(
                &KVBUFF[I],
                1,
                1,
                &mut M,
                CharArrayMut::from_mut(&mut FRNAME),
                &mut FOUND,
                ctx,
            )?;

            if FOUND {
                //
                // We found a possible frame name. Attempt to look
                // up an ID code variable for the name. The assignment
                // for the ID code, if present, will have the form
                //
                //    FRAME_<name> = <ID code>
                //
                // Create the kernel variable name on the left hand
                // side of the assignment.
                //
                fstr::assign(&mut KVCODE, b"FRAME_<name>");

                REPMC(&KVCODE.clone(), b"<name>", &FRNAME, &mut KVCODE);

                //
                // Try to fetch the ID code.
                //
                GIPOOL(
                    &KVCODE,
                    1,
                    1,
                    &mut L,
                    std::slice::from_mut(&mut IDCODE),
                    &mut FOUND,
                    ctx,
                )?;

                if FOUND {
                    //
                    // We found an integer on the right hand side
                    // of the assignment. We probably have a
                    // frame specification at this point. Check that
                    // the variable
                    //
                    //    FRAME_<ID code>_NAME
                    //
                    // is present in the kernel pool and maps to
                    // the name FRNAME.
                    //
                    fstr::assign(&mut KVNAME, b"FRAME_<code>_NAME");

                    REPMI(&KVNAME.clone(), b"<code>", IDCODE, &mut KVNAME, ctx);

                    GCPOOL(
                        &KVNAME,
                        1,
                        1,
                        &mut W,
                        CharArrayMut::from_mut(&mut TMPNAM),
                        &mut FOUND,
                        ctx,
                    )?;

                    if FOUND {
                        //
                        // Try to look up the frame class using a
                        // kernel variable name of the form
                        //
                        //    FRAME_<integer ID code>_CLASS
                        //
                        // Create the kernel variable name on the left
                        // hand side of the frame class assignment.
                        //
                        fstr::assign(&mut KVCLAS, b"FRAME_<integer>_CLASS");

                        REPMI(&KVCLAS.clone(), b"<integer>", IDCODE, &mut KVCLAS, ctx);

                        //
                        // Look for the frame class.
                        //
                        GIPOOL(
                            &KVCLAS,
                            1,
                            1,
                            &mut W,
                            std::slice::from_mut(&mut FCLASS),
                            &mut FOUND,
                            ctx,
                        )?;

                        if !FOUND {
                            //
                            // Try to look up the frame class using a kernel
                            // variable name of the form
                            //
                            //    FRAME_<frame name>_CLASS
                            //
                            fstr::assign(&mut KVCLAS, b"FRAME_<name>_CLASS");

                            REPMC(&KVCLAS.clone(), b"<name>", &FRNAME, &mut KVCLAS);

                            GIPOOL(
                                &KVCLAS,
                                1,
                                1,
                                &mut W,
                                std::slice::from_mut(&mut FCLASS),
                                &mut FOUND,
                                ctx,
                            )?;
                        }

                        //
                        // At this point FOUND indicates whether we found
                        // the frame class.
                        //
                        if FOUND {
                            //
                            // Check whether the frame class is one
                            // we want.
                            //
                            if ((FRMCLS == ALL) || (FRMCLS == FCLASS)) {
                                //
                                // We have a winner. Add it to the output set.
                                //
                                // First make sure the set is large enough to
                                // hold another element.
                                //
                                if (TO == SIZEI(IDSET.as_slice(), ctx)?) {
                                    SETMSG(b"Frame ID set argument IDSET has size #; required size is at least #. Make sure that the caller of this routine has initialized IDSET via SSIZEI.", ctx);
                                    ERRINT(b"#", SIZEI(IDSET.as_slice(), ctx)?, ctx);
                                    ERRINT(b"#", (TO + 1), ctx);
                                    SIGERR(b"SPICE(SETTOOSMALL)", ctx)?;
                                    CHKOUT(b"KPLFRM", ctx)?;
                                    return Ok(());
                                }

                                TO = (TO + 1);
                                IDSET[TO] = IDCODE;
                            }
                            //
                            // End of IF block for processing a frame having
                            // a frame class matching the request.
                            //
                        }
                        //
                        // End of IF block for finding the frame class.
                        //
                    }
                    //
                    // End of IF block for finding the frame name.
                    //
                }
                //
                // End of IF block for finding the frame ID.
                //
            }
            //
            // End of IF block for finding string value corresponding to
            // the Ith kernel variable matching the name template.
            //
        }
        //
        // End of loop for processing last batch of potential
        // frame names.
        //
        // Fetch next batch of potential frame names.
        //
        GNPOOL(
            &KVTEMP,
            (TOTAL + 1),
            BUFSIZ,
            &mut N,
            KVBUFF.as_arg_mut(),
            &mut FOUND,
            ctx,
        )?;
    }
    //
    // At this point all kernel variables that matched the frame name
    // keyword template have been processed. All frames of the specified
    // class or classes have had their ID codes appended to IDSET. In
    // general IDSET is not yet a SPICELIB set, since it's not sorted
    // and it may contain duplicate values.
    //
    // Turn IDSET into a set. VALIDI sorts and removes duplicates.
    //
    VALIDI(SIZEI(IDSET.as_slice(), ctx)?, TO, IDSET.as_slice_mut(), ctx)?;

    CHKOUT(b"KPLFRM", ctx)?;
    Ok(())
}
