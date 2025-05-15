//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const IMPLE: i32 = 0;
const IMPCLS: i32 = 1;
const EXPLT: i32 = 2;
const EXPLE: i32 = 3;
const EXPCLS: i32 = 4;
const MNIDXT: i32 = 0;
const MXIDXT: i32 = 4;
const CONBAS: i32 = 1;
const NCON: i32 = (CONBAS + 1);
const RDRBAS: i32 = (NCON + 1);
const NRDR: i32 = (RDRBAS + 1);
const RDRTYP: i32 = (NRDR + 1);
const REFBAS: i32 = (RDRTYP + 1);
const NREF: i32 = (REFBAS + 1);
const PDRBAS: i32 = (NREF + 1);
const NPDR: i32 = (PDRBAS + 1);
const PDRTYP: i32 = (NPDR + 1);
const PKTBAS: i32 = (PDRTYP + 1);
const NPKT: i32 = (PKTBAS + 1);
const RSVBAS: i32 = (NPKT + 1);
const NRSV: i32 = (RSVBAS + 1);
const PKTSZ: i32 = (NRSV + 1);
const PKTOFF: i32 = (PKTSZ + 1);
const NMETA: i32 = (PKTOFF + 1);
const MXMETA: i32 = NMETA;
const MNMETA: i32 = 15;
const SPKTYP: i32 = 10;
const NCONST: i32 = 8;
const NELEMS: i32 = 10;
const NUOBL: i32 = 11;
const NULON: i32 = 12;
const DNUOBL: i32 = 13;
const DNULON: i32 = 14;
const PKTSIZ: i32 = DNULON;

/// SPK - write a type 10 segment
///
/// Write an SPK type 10 segment to the file specified by
/// the input HANDLE.
///
/// # Required Reading
///
/// * [NAIF_IDS](crate::required_reading::naif_ids)
/// * [SPK](crate::required_reading::spk)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   The handle of a DAF file open for writing.
///  BODY       I   The NAIF ID code for the body of the segment.
///  CENTER     I   The center of motion for BODY.
///  FRAME      I   The reference frame for this segment.
///  FIRST      I   The first epoch for which the segment is valid.
///  LAST       I   The last  epoch for which the segment is valid.
///  SEGID      I   The string to use for segment identifier.
///  CONSTS     I   Array of geophysical constants for the segment.
///  N          I   The number of element/epoch pairs to be stored
///  ELEMS      I   The collection of "two-line" element sets.
///  EPOCHS     I   The epochs associated with the element sets.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the file handle of an SPK file that has been opened
///           for writing.
///
///  BODY     is the NAIF ID for the body whose states are
///           to be recorded in an SPK file.
///
///  CENTER   is the NAIF ID for the center of motion associated
///           with BODY.
///
///  FRAME    is the reference frame that states are referenced to,
///           for example 'J2000'.
///
///  FIRST,
///  LAST     are the bounds on the ephemeris times, expressed as
///           seconds past J2000, for which the states can be used
///           to interpolate a state for BODY.
///
///  SEGID    is the segment identifier. An SPK segment identifier
///           may contain up to 40 characters.
///
///  CONSTS   are the geophysical constants needed for evaluation
///           of the two line elements sets. The order of these
///           constants must be:
///
///              CONSTS(1) = J2 gravitational harmonic for Earth.
///              CONSTS(2) = J3 gravitational harmonic for Earth.
///              CONSTS(3) = J4 gravitational harmonic for Earth.
///
///           These first three constants are dimensionless.
///
///              CONSTS(4) = KE: Square root of the GM for Earth where
///                          GM is expressed in Earth radii cubed
///                          per minutes squared.
///
///              CONSTS(5) = QO: High altitude bound for atmospheric
///                          model in km.
///
///              CONSTS(6) = SO: Low altitude bound for atmospheric
///                          model in km.
///
///              CONSTS(7) = RE: Equatorial radius of the earth in km.
///
///              CONSTS(8) = AE: Distance units/earth radius
///                          (normally 1).
///
///           Below are currently recommended values for these
///           items:
///
///              J2 =    1.082616D-3
///              J3 =   -2.53881D-6
///              J4 =   -1.65597D-6
///
///           The next item is the square root of GM for the Earth
///           given in units of earth-radii**1.5/Minute
///
///              KE =    7.43669161D-2
///
///           The next two items define the top and bottom of the
///           atmospheric drag model used by the type 10 ephemeris
///           type. Don't adjust these unless you understand the full
///           implications of such changes.
///
///              QO =  120.0D0
///              SO =   78.0D0
///
///           The ER value is the equatorial radius in km of the Earth
///           as used by NORAD.
///
///              ER = 6378.135D0
///
///           The value of AE is the number of distance units per
///           Earth radii used by the NORAD state propagation
///           software. The value should be 1 unless you've got a very
///           good understanding of the NORAD routine SGP4 and the
///           affect of changing this value.
///
///              AE =    1.0D0
///
///  N        is the number of "two-line" element sets and epochs
///           to be stored in the segment.
///
///  ELEMS    is a time-ordered array of two-line elements as supplied
///           in NORAD two-line element files. The I'th set of
///           elements should be stored as shown here:
///
///              BASE = (I-1)*10
///
///              ELEMS( BASE + 1  )  = NDT2O in radians/minute**2
///              ELEMS( BASE + 2  )  = NDD6O in radians/minute**3
///              ELEMS( BASE + 3  )  = BSTAR
///              ELEMS( BASE + 4  )  = INCL  in radians
///              ELEMS( BASE + 5  )  = NODE0 in radians
///              ELEMS( BASE + 6  )  = ECC
///              ELEMS( BASE + 7  )  = OMEGA in radians
///              ELEMS( BASE + 8  )  = M0    in radians
///              ELEMS( BASE + 9  )  = N0    in radians/minute
///              ELEMS( BASE + 10 )  = EPOCH of the elements in seconds
///                                    past ephemeris epoch J2000.
///
///           The meaning of these variables is defined by the
///           format of the two-line element files available from
///           NORAD.
///
///  EPOCHS   is an n-dimensional array that contains the epochs
///           (ephemeris seconds past J2000) corresponding to the
///           elements in ELEMS. The I'th epoch must equal the epoch
///           of the I'th element set. EPOCHS must form a strictly
///           increasing sequence.
/// ```
///
/// # Detailed Output
///
/// ```text
///  None.
///
///  The routine writes an SPK type 10 segment to the file attached to
///  HANDLE.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the structure or content of the inputs are invalid, an
///      error is signaled by a routine in the call tree of this
///      routine.
///
///  2)  If any file access error occurs, the error is signaled by a
///      routine in the call tree of this routine.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine writes a type 10 SPK segment to the SPK file open
///  for writing that is attached to HANDLE.
///
///  The routine GETELM reads two-line element sets, as those
///  distributed by NORAD, and converts them to the elements in units
///  suitable for use in this routine.
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
///  1) Suppose that you have collected the two-line element data
///     for a spacecraft with NORAD ID 18123. The following example
///     code demonstrates how you could go about creating a type 10
///     SPK segment.
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File name: spkw10_ex1.tm
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
///           File name           Contents
///           ---------           ------------------------------------
///           naif0012.tls        Leapseconds
///           geophysical.ker     geophysical constants for evaluation
///                               of two-line element sets.
///
///        \begindata
///
///           KERNELS_TO_LOAD = ( 'naif0012.tls',
///                               'geophysical.ker'  )
///
///        \begintext
///
///        The geophysical.ker is a PCK file that is provided with the
///        SPICE toolkit under the "/data" directory.
///
///        End of meta-kernel
///
///
///     Example code begins here.
///
///
///           PROGRAM SPKW10_EX1
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions.
///     C
///           DOUBLE PRECISION      SPD
///
///     C
///     C     Local parameters.
///     C
///           INTEGER               NAMLEN
///           PARAMETER           ( NAMLEN = 40               )
///
///           INTEGER               PNAMLN
///           PARAMETER           ( PNAMLN = 2                )
///
///           CHARACTER*(*)         SPK10
///           PARAMETER           ( SPK10  = 'spkw10_ex1.bsp' )
///
///           INTEGER               TLELLN
///           PARAMETER           ( TLELLN = 69               )
///
///     C
///     C     The SPK type 10 segment will contain 18 two-line
///     C     elements sets for the NORAD spacecraft 18123 with
///     C     respect to the Earth (ID 399) in the J2000 reference
///     C     frame.
///     C
///     C     As stated in the naif_ids required reading, for Earth
///     C     orbiting spacecraft lacking a DSN identification code,
///     C     the NAIF ID is derived from the tracking ID assigned to
///     C     it by NORAD via:
///     C
///     C        NAIF ID = -100000 - NORAD ID code
///     C
///           INTEGER               TLESSZ
///           PARAMETER           ( TLESSZ = 9       )
///
///           INTEGER               BODY
///           PARAMETER           ( BODY   = -118123 )
///
///           INTEGER               CENTER
///           PARAMETER           ( CENTER = 399     )
///
///           CHARACTER*(*)         FRMNAM
///           PARAMETER           ( FRMNAM = 'J2000' )
///
///     C
///     C     Local variables.
///     C
///           CHARACTER*(NAMLEN)    IFNAME
///           CHARACTER*(PNAMLN)    NOADPN ( 8           )
///           CHARACTER*(NAMLEN)    SEGID
///           CHARACTER*(TLELLN)    TLE    ( 2  * TLESSZ )
///
///           DOUBLE PRECISION      CONSTS ( 8           )
///           DOUBLE PRECISION      ELEMS  ( 10 * TLESSZ )
///           DOUBLE PRECISION      EPOCHS (      TLESSZ )
///           DOUBLE PRECISION      FIRST
///           DOUBLE PRECISION      LAST
///
///           INTEGER               HANDLE
///           INTEGER               I
///           INTEGER               N
///           INTEGER               NCOMCH
///
///     C
///     C     These are the variables that will hold the constants
///     C     required by SPK type 10. These constants are available
///     C     from the loaded PCK file, which provides the actual
///     C     values and units as used by NORAD propagation model.
///     C
///     C        Constant   Meaning
///     C        --------   ------------------------------------------
///     C        J2         J2 gravitational harmonic for Earth.
///     C        J3         J3 gravitational harmonic for Earth.
///     C        J4         J4 gravitational harmonic for Earth.
///     C        KE         Square root of the GM for Earth.
///     C        QO         High altitude bound for atmospheric model.
///     C        SO         Low altitude bound for atmospheric model.
///     C        ER         Equatorial radius of the Earth.
///     C        AE         Distance units/earth radius.
///     C
///           DATA          NOADPN  /  'J2', 'J3', 'J4', 'KE',
///          .                         'QO', 'SO', 'ER', 'AE'  /
///
///     C
///     C     Define the Two-Line Element sets.
///     C
///           TLE(1)  = '1 18123U 87 53  A 87324.61041692 -.00000023'
///          .      //                   '  00000-0 -75103-5 0 00675'
///           TLE(2)  = '2 18123  98.8296 152.0074 0014950 168.7820 '
///          .      //                   '191.3688 14.12912554 21686'
///           TLE(3)  = '1 18123U 87 53  A 87326.73487726  .00000045'
///          .      //                   '  00000-0  28709-4 0 00684'
///           TLE(4)  = '2 18123  98.8335 154.1103 0015643 163.5445 '
///          .      //                   '196.6235 14.12912902 21988'
///           TLE(5)  = '1 18123U 87 53  A 87331.40868801  .00000104'
///          .      //                   '  00000-0  60183-4 0 00690'
///           TLE(6)  = '2 18123  98.8311 158.7160 0015481 149.9848 '
///          .      //                   '210.2220 14.12914624 22644'
///           TLE(7)  = '1 18123U 87 53  A 87334.24129978  .00000086'
///          .      //                   '  00000-0  51111-4 0 00702'
///           TLE(8)  = '2 18123  98.8296 161.5054 0015372 142.4159 '
///          .      //                   '217.8089 14.12914879 23045'
///           TLE(9)  = '1 18123U 87 53  A 87336.93227900 -.00000107'
///          .      //                   '  00000-0 -52860-4 0 00713'
///           TLE(10) = '2 18123  98.8317 164.1627 0014570 135.9191 '
///          .      //                   '224.2321 14.12910572 23425'
///           TLE(11) = '1 18123U 87 53  A 87337.28635487  .00000173'
///          .      //                   '  00000-0  10226-3 0 00726'
///           TLE(12) = '2 18123  98.8284 164.5113 0015289 133.5979 '
///          .      //                   '226.6438 14.12916140 23475'
///           TLE(13) = '1 18123U 87 53  A 87339.05673569  .00000079'
///          .      //                   '  00000-0  47069-4 0 00738'
///           TLE(14) = '2 18123  98.8288 166.2585 0015281 127.9985 '
///          .      //                   '232.2567 14.12916010 24908'
///           TLE(15) = '1 18123U 87 53  A 87345.43010859  .00000022'
///          .      //                   '  00000-0  16481-4 0 00758'
///           TLE(16) = '2 18123  98.8241 172.5226 0015362 109.1515 '
///          .      //                   '251.1323 14.12915487 24626'
///           TLE(17) = '1 18123U 87 53  A 87349.04167543  .00000042'
///          .      //                   '  00000-0  27370-4 0 00764'
///           TLE(18) = '2 18123  98.8301 176.1010 0015565 100.0881 '
///          .      //                   '260.2047 14.12916361 25138'
///
///     C
///     C     Load the PCK file that provides the geophysical
///     C     constants required for the evaluation of the two-line
///     C     elements sets. Load also an LSK, as it is required by
///     C     GETELM to perform time conversions. Use a metakernel for
///     C     convenience.
///     C
///           CALL FURNSH ( 'spkw10_ex1.tm' )
///
///     C
///     C     Retrieve the data from the kernel, and place it on
///     C     the CONSTS array.
///     C
///           DO I = 1, 8
///
///              CALL BODVCD ( CENTER, NOADPN(I), 1, N, CONSTS(I) )
///
///           END DO
///
///     C
///     C     Convert the Two Line Elements lines to the
///     C     element sets.
///     C
///           DO I = 1, TLESSZ
///
///              CALL GETELM ( 1950,      TLE( (I-1)*2 + 1 ),
///          .                 EPOCHS(I), ELEMS( (I-1)*10 + 1 ) )
///
///           END DO
///
///     C
///     C     Define the beginning and end of the segment to be
///     C     -/+ 12 hours from the first and last epochs,
///     C     respectively.
///     C
///           FIRST = EPOCHS(1     ) - 0.5D0 * SPD()
///           LAST  = EPOCHS(TLESSZ) + 0.5D0 * SPD()
///
///     C
///     C     NCOMCH is the number of characters to reserve for the
///     C     kernel's comment area. This example doesn't write
///     C     comments, so set to zero.
///     C
///           NCOMCH = 0
///
///     C
///     C     Internal file name and segment ID.
///     C
///           IFNAME = 'Test for type 10 SPK internal file name'
///           SEGID  = 'SPK type 10 test segment'
///
///     C
///     C     Open a new SPK file.
///     C
///           CALL SPKOPN( SPK10, IFNAME, NCOMCH, HANDLE )
///
///     C
///     C     Now add the segment.
///     C
///           CALL SPKW10 ( HANDLE, BODY,  CENTER, FRMNAM,
///          .              FIRST,  LAST,  SEGID,  CONSTS,
///          .              TLESSZ, ELEMS, EPOCHS         )
///
///     C
///     C     Close the SPK file.
///     C
///           CALL SPKCLS ( HANDLE )
///
///           END
///
///
///     When this program is executed, no output is presented on
///     screen. After run completion, a new SPK type 10 exists in
///     the output directory.
/// ```
///
/// # Literature References
///
/// ```text
///  [1]  F. Hoots and R. Roehrich, "Spacetrack Report #3: Models for
///       Propagation of the NORAD Element Sets," U.S. Air Force
///       Aerospace Defense Command, Colorado Springs, CO, 1980.
///
///  [2]  F. Hoots, "Spacetrack Report #6: Models for Propagation of
///       Space Command Element Sets,"  U.S. Air Force Aerospace
///       Defense Command, Colorado Springs, CO, 1986.
///
///  [3]  F. Hoots, P. Schumacher and R. Glover, "History of Analytical
///       Orbit Modeling in the U. S. Space Surveillance System,"
///       Journal of Guidance, Control, and Dynamics. 27(2):174-185,
///       2004.
///
///  [4]  D. Vallado, P. Crawford, R. Hujsak and T. Kelso, "Revisiting
///       Spacetrack Report #3," paper AIAA 2006-6753 presented at the
///       AIAA/AAS Astrodynamics Specialist Conference, Keystone, CO.,
///       August 21-24, 2006.
/// ```
///
/// # Author and Institution
///
/// ```text
///  M. Costa Sitja     (JPL)
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 04-NOV-2021 (JDR) (MCS)
///
///         Added IMPLICIT NONE statement.
///
///         Corrected the expected order of QO, SO and ER in the detailed
///         description of the input argument GEOPHS and the input element
///         names in ELEMS.
///
///         Added Spacetrack Report #3 to literature references and
///         NAIF_IDS to the list of required readings.
///
///         Edited the header to comply with NAIF standard. Added complete
///         code example based on existing example.
///
/// -    SPICELIB Version 1.0.2, 30-OCT-2006 (BVS)
///
///         Deleted "inertial" from the FRAME description in the $Brief_I/O
///         section of the header.
///
/// -    SPICELIB Version 1.0.1, 21-JUN-1999 (WLT)
///
///         Cleaned up the header.
///
/// -    SPICELIB Version 1.0.0, 05-JAN-1994 (WLT)
/// ```
pub fn spkw10(
    ctx: &mut SpiceContext,
    handle: i32,
    body: i32,
    center: i32,
    frame: &str,
    first: f64,
    last: f64,
    segid: &str,
    consts: &[f64],
    n: i32,
    elems: &[f64],
    epochs: &[f64],
) -> crate::Result<()> {
    SPKW10(
        handle,
        body,
        center,
        frame.as_bytes(),
        first,
        last,
        segid.as_bytes(),
        consts,
        n,
        elems,
        epochs,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPKW10 (SPK - write a type 10 segment )
pub fn SPKW10(
    HANDLE: i32,
    BODY: i32,
    CENTER: i32,
    FRAME: &[u8],
    FIRST: f64,
    LAST: f64,
    SEGID: &[u8],
    CONSTS: &[f64],
    N: i32,
    ELEMS: &[f64],
    EPOCHS: &[f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let CONSTS = DummyArray::new(CONSTS, 1..);
    let ELEMS = DummyArray::new(ELEMS, 1..);
    let EPOCHS = DummyArray::new(EPOCHS, 1..);
    let mut DESCR = StackArray::<f64, 6>::new(1..=6);
    let mut PACKET = StackArray::<f64, 14>::new(1..=PKTSIZ);
    let mut DNUT = StackArray::<f64, 4>::new(1..=4);
    let mut BASE: i32 = 0;
    let mut NPKTS: i32 = 0;
    let mut NEPOCH: i32 = 0;

    //
    // Spicelib functions
    //

    //
    // Local Variables
    //

    //
    // The type of this segment
    //
    //
    // The number of geophysical constants:
    //
    //
    // The number of elements per two-line set:
    //

    //
    // Standard SPICELIB error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"SPKW10", ctx)?;
    //
    // First we need to create a descriptor for the segment
    // we are about to write.
    //
    SPKPDS(
        BODY,
        CENTER,
        FRAME,
        SPKTYP,
        FIRST,
        LAST,
        DESCR.as_slice_mut(),
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"SPKW10", ctx)?;
        return Ok(());
    }

    //
    // We've got a valid descriptor, write the data to a DAF
    // segment using the generic segment writer.
    //
    NPKTS = N;
    NEPOCH = N;

    SGBWFS(
        HANDLE,
        DESCR.as_slice(),
        SEGID,
        NCONST,
        CONSTS.as_slice(),
        &[PKTSIZ],
        EXPCLS,
        ctx,
    )?;

    for I in 1..=NEPOCH {
        //
        // Move the elements into the next packet.
        //
        BASE = ((I - 1) * NELEMS);

        MOVED(ELEMS.subarray((BASE + 1)), 10, PACKET.as_slice_mut());
        //
        // For each epoch, we need to get the nutation in obliquity,
        // nutation in longitude and mean obliquity.
        //
        ZZWAHR(EPOCHS[I], DNUT.as_slice_mut(), ctx);

        PACKET[NULON] = DNUT[1];
        PACKET[NUOBL] = DNUT[2];
        PACKET[DNULON] = DNUT[3];
        PACKET[DNUOBL] = DNUT[4];

        //
        // Now write the packet into the generic segment.
        //
        SGWFPK(HANDLE, 1, PACKET.as_slice(), 1, EPOCHS.subarray(I), ctx)?;
    }

    SGWES(HANDLE, ctx)?;

    CHKOUT(b"SPKW10", ctx)?;

    Ok(())
}
