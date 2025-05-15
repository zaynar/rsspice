//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const NDC: i32 = 2;
const NIC: i32 = 6;
const NC: i32 = (NDC + ((NIC + 1) / 2));
const IDLEN: i32 = (NC * 8);

/// CK frame, find position rotation
///
/// Find the position rotation matrix from a C-kernel (CK) frame with
/// the specified frame class ID (CK ID) to the base frame of the
/// highest priority CK segment containing orientation data for this
/// CK frame at the time requested.
///
/// # Required Reading
///
/// * [CK](crate::required_reading::ck)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  INST       I   Frame class ID (CK ID) of a CK frame.
///  ET         I   Epoch measured in seconds past J2000 TDB.
///  ROTATE     O   Rotation matrix from CK frame to frame REF.
///  REF        O   Frame ID of the base reference.
///  FOUND      O   .TRUE. when requested pointing is available.
/// ```
///
/// # Detailed Input
///
/// ```text
///  INST     is the unique frame class ID (CK ID) of the CK frame for
///           which data is being requested.
///
///  ET       is the epoch for which the position rotation is desired.
///           ET should be given in seconds past the epoch of J2000
///           TDB.
/// ```
///
/// # Detailed Output
///
/// ```text
///  ROTATE   is a position rotation matrix that converts positions
///           relative to the CK frame given by its frame class ID,
///           INST, to positions relative to the base frame given by
///           its frame ID, REF.
///
///           Thus, if a position S has components x,y,z in the CK
///           frame, then S has components x', y', z' in the base
///           frame.
///
///              .-  -.     .-        -. .- -.
///              | x' |     |          | | x |
///              | y' |  =  |  ROTATE  | | y |
///              | z' |     |          | | z |
///              `-  -'     `-        -' `- -'
///
///
///  REF      is the ID code of the base reference frame to which
///           ROTATE will transform positions.
///
///  FOUND    is .TRUE. if a record was found to satisfy the pointing
///           request. FOUND will be .FALSE. otherwise.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If no CK files were loaded prior to calling this routine, an
///      error is signaled by a routine in the call tree of this
///      routine.
///
///  2)  If no SCLK correlation data needed to read CK files were
///      loaded prior to calling this routine, an error is signaled by
///      a routine in the call tree of this routine.
///
///  3)  If the input time ET cannot be converted to an encoded SCLK
///      time, using SCLK data associated with INST, an error is
///      signaled by a routine in the call tree of this routine.
/// ```
///
/// # Files
///
/// ```text
///  CKFROT searches through loaded CK files to locate a segment that
///  can satisfy the request for position rotation data for the CK
///  frame with the specified frame class ID at time ET. You must load
///  a CK file containing such data before calling this routine. You
///  must also load SCLK and possibly LSK files needed to convert the
///  input ET time to the encoded SCLK time with which the orientation
///  data stored inside that CK is tagged.
/// ```
///
/// # Particulars
///
/// ```text
///  CKFROT searches through loaded CK files to satisfy a pointing
///  request. Last-loaded files are searched first, and individual
///  files are searched in backwards order, giving priority to
///  segments that were added to a file later than the others.
///
///  The search ends when a segment is found that can give pointing
///  for the specified CK frame at the request time.
///
///  Segments with and without angular velocities are considered by
///  this routine.
///
///  This routine uses the CKMETA routine to determine the SCLK ID
///  used to convert the input ET time to the encoded SCLK time used
///  to look up pointing data in loaded CK files.
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
///  1) Use CKFROT to compute the instantaneous angular velocity
///     vector for the Mars Global Surveyor (MGS) spacecraft frame,
///     'MGS_SPACECRAFT', relative to the inertial frame used as the
///     base frame in CK files containing MGS spacecraft orientation
///     at 2003-JUL-25 13:00:00. The frame class ID (CK ID) for the
///     'MGS_SPACECRAFT' frame is -94000.
///
///
///     Suppose that R(t) is the rotation matrix whose columns
///     represent the inertial pointing vectors of the MGS spacecraft
///     axes at time `t'.
///
///     Then the angular velocity vector points along the vector given
///     by:
///
///                             T
///         limit  AXIS( R(t+h)R )
///         h-->0
///
///
///     And the magnitude of the angular velocity at time `t' is given
///     by:
///
///                             T
///         d ANGLE ( R(t+h)R(t) )
///        ------------------------   at   h = 0
///                   dh
///
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File name: ckfrot_ex1.tm
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
///           File name                     Contents
///           ---------                     --------
///           naif0012.tls                  Leapseconds
///           mgs_sclkscet_00061.tsc        MGS SCLK coefficients
///           mgs_sc_ext12.bc               MGS s/c bus attitude
///
///        \begindata
///
///        KERNELS_TO_LOAD = ( 'naif0012.tls',
///                            'mgs_sclkscet_00061.tsc',
///                            'mgs_sc_ext12.bc' )
///
///        \begintext
///
///        End of meta-kernel
///
///
///     Example code begins here.
///
///
///           PROGRAM CKFROT_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local parameters.
///     C
///           CHARACTER*(*)         EPOCH
///           PARAMETER           ( EPOCH  = '2003-JUL-25 13:00:00' )
///
///           INTEGER               INST
///           PARAMETER           ( INST   = -94000 )
///
///     C
///     C     Local variables.
///     C
///           DOUBLE PRECISION      ANGLE
///           DOUBLE PRECISION      ANGVEL ( 3    )
///           DOUBLE PRECISION      AXIS   ( 3    )
///           DOUBLE PRECISION      ET
///           DOUBLE PRECISION      INFROT ( 3, 3 )
///           DOUBLE PRECISION      H
///           DOUBLE PRECISION      RET    ( 3, 3 )
///           DOUBLE PRECISION      RETH   ( 3, 3 )
///
///           INTEGER               REF
///           INTEGER               REFH
///
///           LOGICAL               FOUND
///           LOGICAL               FOUNDH
///
///     C
///     C     Load the required LSK, SCLK and CK. Use a
///     C     meta-kernel for convenience.
///     C
///           CALL FURNSH ( 'ckfrot_ex1.tm' )
///
///     C
///     C     First convert the time to seconds past J2000. Set the
///     C     delta time (1 ms).
///     C
///           CALL STR2ET ( EPOCH, ET )
///           H = 1.D-3
///
///     C
///     C     Now, look up the rotation from the MGS spacecraft
///     C     frame specified by its frame class ID (CK ID) to a
///     C     base reference frame (returned by CKFROT), at ET
///     C     and ET+H.
///     C
///           CALL CKFROT ( INST, ET,   RET,  REF,  FOUND  )
///           CALL CKFROT ( INST, ET+H, RETH, REFH, FOUNDH )
///
///     C
///     C     If both rotations were computed and if the base
///     C     reference frames are the same, compute the
///     C     instantaneous angular velocity vector.
///     C
///           IF ( FOUND .AND. FOUNDH .AND. REF .EQ. REFH ) THEN
///
///     C
///     C        Compute the infinitesimal rotation R(t+h)R(t)**T.
///     C
///              CALL MXMT ( RETH, RET, INFROT )
///
///     C
///     C        Compute the AXIS and ANGLE of the infinitesimal
///     C        rotation.
///     C
///              CALL RAXISA ( INFROT, AXIS, ANGLE )
///
///     C
///     C        Scale AXIS to get the angular velocity vector.
///     C
///              CALL VSCL ( ANGLE/H, AXIS, ANGVEL )
///
///     C
///     C        Output the results.
///     C
///              WRITE(*,'(A)')
///          .           'Instantaneous angular velocity vector:'
///              WRITE(*,'(3F15.10)') ANGVEL
///              WRITE(*,'(A,I5)') 'Reference frame ID:', REF
///
///           ELSE
///
///              WRITE(*,*) 'ERROR: data not found or frame mismatch.'
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
///     Instantaneous angular velocity vector:
///        0.0001244121   0.0008314866   0.0003028634
///     Reference frame ID:    1
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  A CK file must be loaded prior to calling this routine.
///
///  2)  LSK and SCLK files needed for time conversions must be loaded
///      prior to calling this routine.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.3.0, 13-DEC-2021 (JDR) (BVS) (NJB)
///
///         Edited the header to comply with NAIF standard and modern
///         SPICE CK and frames terminology. Added initialization of local
///         variable SFND.
///
///         Added complete code example.
///
/// -    SPICELIB Version 1.2.0, 17-FEB-2000 (WLT)
///
///         The routine now checks to make sure convert ET to TICKS
///         and that at least one C-kernel is loaded before trying
///         to look up the transformation. Also the routine now calls
///         SCE2C instead of SCE2T.
///
/// -    SPICELIB Version 1.0.0, 03-MAR-1999 (WLT)
/// ```
pub fn ckfrot(
    ctx: &mut SpiceContext,
    inst: i32,
    et: f64,
    rotate: &mut [[f64; 3]; 3],
    ref_: &mut i32,
    found: &mut bool,
) -> crate::Result<()> {
    CKFROT(
        inst,
        et,
        rotate.as_flattened_mut(),
        ref_,
        found,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure CKFROT ( CK frame, find position rotation )
pub fn CKFROT(
    INST: i32,
    ET: f64,
    ROTATE: &mut [f64],
    REF: &mut i32,
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut ROTATE = DummyArrayMut2D::new(ROTATE, 1..=3, 1..=3);
    let mut HANDLE: i32 = 0;
    let mut ICD = StackArray::<i32, 6>::new(1..=NIC);
    let mut SCLKID: i32 = 0;
    let mut AV = StackArray::<f64, 3>::new(1..=3);
    let mut CLKOUT: f64 = 0.0;
    let mut DESCR = StackArray::<f64, 5>::new(1..=NC);
    let mut DCD = StackArray::<f64, 2>::new(1..=NDC);
    let mut ROT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut TIME: f64 = 0.0;
    let mut TOL: f64 = 0.0;
    let mut SEGID = [b' '; IDLEN as usize];
    let mut NEEDAV: bool = false;
    let mut SFND: bool = false;
    let mut PFND: bool = false;
    let mut HAVE: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //
    //    NDC        is the number of double precision components in an
    //               unpacked C-kernel segment descriptor.
    //
    //    NIC        is the number of integer components in an unpacked
    //               C-kernel segment descriptor.
    //
    //    NC         is the number of components in a packed C-kernel
    //               descriptor.  All DAF summaries have this formulaic
    //               relationship between the number of its integer and
    //               double precision components and the number of packed
    //               components.
    //
    //    IDLEN      is the length of the C-kernel segment identifier.
    //               All DAF names have this formulaic relationship
    //               between the number of summary components and
    //               the length of the name (You will notice that
    //               a name and a summary have the same length in bytes.)
    //

    //
    // Local variables
    //

    //
    // Set FOUND to .FALSE. right now in case we end up
    // returning before doing any work.
    //
    *FOUND = false;
    *REF = 0;

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"CKFROT", ctx)?;

    //
    // We don't need angular velocity data.
    // Assume the segment won't be found until it really is.
    //
    NEEDAV = false;
    TOL = 0.0;

    //
    // Begin a search for this instrument and time, and get the first
    // applicable segment.
    //
    CKHAVE(&mut HAVE, ctx);
    CKMETA(INST, b"SCLK", &mut SCLKID, ctx)?;

    if !HAVE {
        CHKOUT(b"CKFROT", ctx)?;
        return Ok(());
    } else if !ZZSCLK(INST, SCLKID, ctx)? {
        CHKOUT(b"CKFROT", ctx)?;
        return Ok(());
    }

    //
    // Initialize SFND here in case an error occurs before CKSNS can
    // set its value.
    //
    SFND = false;

    SCE2C(SCLKID, ET, &mut TIME, ctx)?;
    CKBSS(INST, TIME, TOL, NEEDAV, ctx)?;
    CKSNS(
        &mut HANDLE,
        DESCR.as_slice_mut(),
        &mut SEGID,
        &mut SFND,
        ctx,
    )?;

    //
    // Keep trying candidate segments until a segment can produce a
    // pointing instance within the specified time tolerance of the
    // input time.
    //
    // Check FAILED to prevent an infinite loop if an error is detected
    // by a SPICELIB routine and the error handling is not set to abort.
    //
    while (SFND && !FAILED(ctx)) {
        CKPFS(
            HANDLE,
            DESCR.as_slice(),
            TIME,
            TOL,
            NEEDAV,
            ROT.as_slice_mut(),
            AV.as_slice_mut(),
            &mut CLKOUT,
            &mut PFND,
            ctx,
        )?;

        if PFND {
            //
            // Found one. Fetch the ID code of the reference frame
            // from the descriptor.
            //
            DAFUS(
                DESCR.as_slice(),
                NDC,
                NIC,
                DCD.as_slice_mut(),
                ICD.as_slice_mut(),
            );
            *REF = ICD[2];
            *FOUND = true;
            //
            // We now have the rotation matrix from
            // REF to INS. We invert ROT to get the rotation
            // from INST to REF.
            //
            XPOSE(ROT.as_slice(), ROTATE.as_slice_mut());

            CHKOUT(b"CKFROT", ctx)?;
            return Ok(());
        }

        CKSNS(
            &mut HANDLE,
            DESCR.as_slice_mut(),
            &mut SEGID,
            &mut SFND,
            ctx,
        )?;
    }

    CHKOUT(b"CKFROT", ctx)?;
    Ok(())
}
