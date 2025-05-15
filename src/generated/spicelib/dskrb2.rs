//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
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
const THIRD: f64 = (1.0 / 3.0);

struct SaveVars {
    ORIGIN: StackArray<f64, 3>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ORIGIN = StackArray::<f64, 3>::new(1..=3);

        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::D(0.0), 3 as usize))
                .chain([]);

            ORIGIN
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self { ORIGIN }
    }
}

/// DSK, determine range bounds for plate set
///
/// Determine range bounds for a set of triangular plates to
/// be stored in a type 2 DSK segment.
///
/// # Required Reading
///
/// * [DSK](crate::required_reading::dsk)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  NV         I   Number of vertices.
///  VRTCES     I   Vertices.
///  NP         I   Number of plates.
///  PLATES     I   Plates.
///  CORSYS     I   DSK coordinate system code.
///  CORPAR     I   DSK coordinate system parameters.
///  MNCOR3     O   Lower bound on range of third coordinate.
///  MXCOR3     O   Upper bound on range of third coordinate.
/// ```
///
/// # Detailed Input
///
/// ```text
///  NV       is the number of vertices belonging to the input
///           set of plates.
///
///
///  VRTCES   is an array of coordinates of the vertices. The Ith
///           vertex occupies elements (1:3,I) of this array.
///
///
///  NP       is the number of plates in the input plate set.
///
///
///  PLATES   is an array representing the triangular plates of a
///           shape model. The elements of PLATES are vertex
///           indices; vertex indices are 1-based. The vertex
///           indices of the Ith plate occupy elements (1:3,I) of
///           this array.
///
///  CORSYS   is an integer parameter identifying the coordinate
///           system in which the bounds are to be computed. The
///           bounds apply to the third coordinate in each system:
///
///              Latitudinal:           radius
///              Planetodetic:          altitude
///              Rectangular:           Z
///
///
///  CORPAR   is an array of parameters associated with the
///             coordinate system. Currently the only supported system
///             that has associated parameters is the planetodetic
///             system. For planetodetic coordinates,
///
///             CORPAR(1) is the equatorial radius
///
///             CORPAR(2) is the flattening coefficient. Let RE and
///             RP represent, respectively, the equatorial and
///             polar radii of the reference ellipsoid of the
///             system. Then
///
///                 CORPAR(2) = ( RE - RP ) / RE
/// ```
///
/// # Detailed Output
///
/// ```text
///  MNCOR3   is a lower bound on the range of the third coordinate
///           of the system identified by CORSYS and CORPAR, taken
///           over all plates.
///
///           For latitudinal and rectangular coordinates, MNCOR3
///           is the greatest lower bound of the third coordinate.
///
///           For planetodetic coordinates, MNCOR3 is an
///           approximation: it is less than or equal to the greatest
///           lower bound.
///
///  MXCOR3   is the least upper bound on the range of the third
///           coordinate of the system identified by CORSYS and
///           CORPAR, taken over all plates.
/// ```
///
/// # Parameters
///
/// ```text
///  See the include file dskdsc.inc for declarations of the public DSK
///  type 2 parameters used by this routine.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the input coordinate system is not recognized, the
///      error SPICE(NOTSUPPORTED) is signaled.
///
///  2)  If a conversion from rectangular to planetodetic coordinates
///      fails, an error is signaled by a routine in the call
///      tree of this routine.
/// ```
///
/// # Particulars
///
/// ```text
///  Users planning to create DSK files should consider whether the
///  SPICE DSK creation utility MKDSK may be suitable for their needs.
///
///  This routine supports use of the DSK type 2 segment writer DSKW02
///  by computing bounds on the range of the third coordinates of
///  the input plate set.
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
///  1) Create a three-segment DSK file using plate model data for
///     Phobos. Use latitudinal, rectangular, and planetodetic
///     coordinates in the respective segments. This is not a
///     realistic example, but it serves to demonstrate use of
///     the supported coordinate systems.
///
///     Use the DSK kernel below to provide, for simplicity, the
///     input plate and vertex data. The selected input file has one
///     segment.
///
///        phobos_3_3.bds
///
///
///     Example code begins here.
///
///
///     C
///     C     Example program for DSKW02, DSKMI2, and DSKRB2
///     C
///     C        Create a three-segment DSK file using plate model
///     C        data for Phobos. Use latitudinal, rectangular, and
///     C        planetodetic coordinates in the respective segments.
///     C
///     C        For simplicity, use an existing DSK file to provide
///     C        the input plate and vertex data. The selected input
///     C        file has one segment.
///     C
///     C           Version 1.0.0 22-JAN-2016 (NJB)
///     C
///           PROGRAM DSKRB2_EX1
///           IMPLICIT NONE
///
///           INCLUDE 'dla.inc'
///           INCLUDE 'dskdsc.inc'
///           INCLUDE 'dsk02.inc'
///
///     C
///     C     SPICELIB functions
///     C
///           DOUBLE PRECISION      JYEAR
///           DOUBLE PRECISION      PI
///     C
///     C     Local parameters
///     C
///           INTEGER               FRNMLN
///           PARAMETER           ( FRNMLN = 32 )
///
///           INTEGER               NSEG
///           PARAMETER           ( NSEG   = 3 )
///
///           INTEGER               NAMLEN
///           PARAMETER           ( NAMLEN = 20 )
///
///           INTEGER               FILSIZ
///           PARAMETER           ( FILSIZ = 255 )
///
///           INTEGER               LNSIZE
///           PARAMETER           ( LNSIZE = 80 )
///
///           INTEGER               NCOR
///           PARAMETER           ( NCOR   = 4 )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(NAMLEN)    CORNAM ( NCOR )
///           CHARACTER*(FILSIZ)    DSK
///           CHARACTER*(FRNMLN)    FRAME
///           CHARACTER*(FILSIZ)    INDSK
///           CHARACTER*(LNSIZE)    LINE
///     C
///     C     Note: the values of MAXVRT and MAXPLT declared
///     C     in dsk02.inc, and the integer spatial index
///     C     dimension SPAISZ are very large. Smaller buffers
///     C     can be used for most applications.
///     C
///           DOUBLE PRECISION      CORPAR ( NSYPAR )
///           DOUBLE PRECISION      F
///           DOUBLE PRECISION      FINSCL
///           DOUBLE PRECISION      FIRST
///           DOUBLE PRECISION      LAST
///           DOUBLE PRECISION      MNCOR1
///           DOUBLE PRECISION      MNCOR2
///           DOUBLE PRECISION      MNCOR3
///           DOUBLE PRECISION      MXCOR1
///           DOUBLE PRECISION      MXCOR2
///           DOUBLE PRECISION      MXCOR3
///           DOUBLE PRECISION      RE
///           DOUBLE PRECISION      RP
///           DOUBLE PRECISION      SPAIXD ( IXDFIX )
///           DOUBLE PRECISION      VRTCES ( 3, MAXVRT )
///
///           INTEGER               CENTER
///           INTEGER               CORSCL
///           INTEGER               CORSYS
///           INTEGER               DCLASS
///           INTEGER               DLADSC ( DLADSZ )
///           INTEGER               HANDLE
///           INTEGER               INHAN
///           INTEGER               NP
///           INTEGER               NV
///           INTEGER               PLATES ( 3, MAXPLT )
///           INTEGER               SEGNO
///           INTEGER               SPAIXI ( SPAISZ )
///           INTEGER               SURFID
///           INTEGER               VOXPSZ
///           INTEGER               VOXLSZ
///           INTEGER               WORK   ( 2, MAXCEL )
///           INTEGER               WORKSZ
///
///           LOGICAL               FOUND
///     C
///     C     Saved variables
///     C
///     C     Save all large arrays to avoid stack problems.
///     C
///           SAVE
///     C
///     C     Initial values
///     C
///           DATA                  CORNAM / 'radius',
///          .                               'Z-coordinate',
///          .                               'Z-coordinate',
///          .                               'altitude'     /
///
///     C
///     C     Assign names of input and output DSK files.
///     C
///           INDSK = 'phobos_3_3.bds'
///           DSK   = 'phobos_3_3_3seg.bds'
///     C
///     C     Open input DSK for read access; find first segment.
///     C
///           CALL DASOPR ( INDSK, INHAN )
///           CALL DLABFS ( INHAN, DLADSC, FOUND )
///     C
///     C     Fetch vertices and plates from input DSK file.
///     C
///           WRITE (*,*) 'Reading input data...'
///
///           CALL DSKV02 ( INHAN, DLADSC, 1, MAXVRT, NV, VRTCES )
///           CALL DSKP02 ( INHAN, DLADSC, 1, MAXPLT, NP, PLATES )
///
///           WRITE (*,*) 'Done.'
///     C
///     C     Set input array sizes required by DSKMI2.
///     C
///           VOXPSZ = MAXVXP
///           VOXLSZ = MXNVLS
///           WORKSZ = MAXCEL
///     C
///     C     Set fine and coarse voxel scales. (These usually
///     C     need to determined by experimentation.)
///     C
///           FINSCL = 5.D0
///           CORSCL = 4
///     C
///     C     Open a new DSK file.
///     C
///           CALL DSKOPN ( DSK, DSK, 0, HANDLE )
///     C
///     C     Create three segments and add them to the file.
///     C
///           DO SEGNO = 1, NSEG
///     C
///     C        Create spatial index.
///     C
///              WRITE (*,*) 'Creating segment ', SEGNO
///              WRITE (*,*) 'Creating spatial index...'
///
///              CALL DSKMI2 ( NV,     VRTCES, NP,     PLATES, FINSCL,
///          .                 CORSCL, WORKSZ, VOXPSZ, VOXLSZ, .TRUE.,
///          .                 SPAISZ, WORK,   SPAIXD, SPAIXI        )
///
///              WRITE (*,*) 'Done.'
///     C
///     C        Set up inputs describing segment attributes:
///     C
///     C        - Central body: Phobos
///     C        - Surface ID code: user's choice.
///     C          We use the segment number here.
///     C        - Data class: general (arbitrary) shape
///     C        - Body-fixed reference frame
///     C        - Time coverage bounds (TBD)
///     C
///              CENTER = 401
///              SURFID = SEGNO
///              DCLASS = GENCLS
///              FRAME  = 'IAU_PHOBOS'
///
///              FIRST = -50 * JYEAR()
///              LAST  =  50 * JYEAR()
///     C
///     C        Set the coordinate system and coordinate system
///     C        bounds based on the segment index.
///     C
///     C        Zero out the coordinate parameters to start.
///     C
///              CALL CLEARD ( NSYPAR, CORPAR )
///
///              IF ( SEGNO .EQ. 1 ) THEN
///     C
///     C           Use planetocentric latitudinal coordinates. Set
///     C           the longitude and latitude bounds.
///     C
///                 CORSYS = LATSYS
///
///                 MNCOR1 = -PI()
///                 MXCOR1 =  PI()
///                 MNCOR2 = -PI()/2
///                 MXCOR2 =  PI()/2
///
///              ELSE IF ( SEGNO .EQ. 2 ) THEN
///     C
///     C           Use rectangular coordinates. Set the
///     C           X and Y bounds.
///     C
///     C           The bounds shown here were derived from
///     C           the plate data. They lie slightly outside
///     C           of the range spanned by the plates.
///     C
///                 CORSYS = RECSYS
///
///                 MNCOR1 = -1.3D0
///                 MXCOR1 =  1.31D0
///                 MNCOR2 = -1.21D0
///                 MXCOR2 =  1.2D0
///
///              ELSE
///     C
///     C           Set the coordinate system to planetodetic.
///     C
///                 CORSYS    = PDTSYS
///
///                 MNCOR1    = -PI()
///                 MXCOR1    =  PI()
///                 MNCOR2    = -PI()/2
///                 MXCOR2    =  PI()/2
///     C
///     C           We'll use equatorial and polar radii from
///     C           pck00010.tpc. These normally would be fetched
///     C           at run time, but for simplicity, we'll use
///     C           hard-coded values.
///
///                 RE        = 13.0D0
///                 RP        =  9.1D0
///                 F         = ( RE - RP ) / RE
///
///                 CORPAR(1) = RE
///                 CORPAR(2) = F
///
///              END IF
///     C
///     C        Compute plate model radius bounds.
///     C
///              LINE = 'Computing # bounds of plate set...'
///
///              CALL REPMC ( LINE, '#', CORNAM(CORSYS), LINE )
///              WRITE (*,*) LINE
///
///              CALL DSKRB2 ( NV,     VRTCES, NP,     PLATES,
///          .                 CORSYS, CORPAR, MNCOR3, MXCOR3 )
///
///              WRITE (*,*) 'Done.'
///     C
///     C        Write the segment to the file.
///     C
///              WRITE (*,*) 'Writing segment...'
///
///              CALL DSKW02 ( HANDLE,
///          .                 CENTER, SURFID, DCLASS, FRAME,  CORSYS,
///          .                 CORPAR, MNCOR1, MXCOR1, MNCOR2, MXCOR2,
///          .                 MNCOR3, MXCOR3, FIRST,  LAST,   NV,
///          .                 VRTCES, NP,     PLATES, SPAIXD, SPAIXI )
///
///              WRITE (*,*) 'Done.'
///
///           END DO
///     C
///     C     Segregate the data records in the DSK file and
///     C     close the file.
///     C
///           WRITE (*,*) 'Segregating and closing DSK file...'
///
///           CALL DSKCLS ( HANDLE, .TRUE. )
///
///           WRITE (*,*) 'Done.'
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      Reading input data...
///      Done.
///      Creating segment            1
///      Creating spatial index...
///      Done.
///      Computing radius bounds of plate set...
///      Done.
///      Writing segment...
///      Done.
///      Creating segment            2
///      Creating spatial index...
///      Done.
///      Computing Z-coordinate bounds of plate set...
///      Done.
///      Writing segment...
///      Done.
///      Creating segment            3
///      Creating spatial index...
///      Done.
///      Computing altitude bounds of plate set...
///      Done.
///      Writing segment...
///      Done.
///      Segregating and closing DSK file...
///      Done.
///
///
///     Note that after run completion, a new DSK exists in the output
///     directory.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  For planetodetic coordinates, the computation of the lower
///      altitude bound requires that the surface at altitude MNCOR3 be
///      convex. This is the case for realistic geometries, but can
///      be false if a plate is very large compared to the overall
///      shape model.
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
/// -    SPICELIB Version 1.0.1, 08-JUL-2020 (JDR)
///
///         Edited the header to comply with NAIF standard.
///         Added solution to code example.
///
/// -    SPICELIB Version 1.0.0, 04-APR-2017 (NJB)
///
///         22-JAN-2016 (NJB)
///
///          Original version.
/// ```
pub fn dskrb2(
    ctx: &mut SpiceContext,
    nv: i32,
    vrtces: &[[f64; 3]],
    np: i32,
    plates: &[[i32; 3]],
    corsys: i32,
    corpar: &[f64],
    mncor3: &mut f64,
    mxcor3: &mut f64,
) -> crate::Result<()> {
    DSKRB2(
        nv,
        vrtces.as_flattened(),
        np,
        plates.as_flattened(),
        corsys,
        corpar,
        mncor3,
        mxcor3,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DSKRB2 ( DSK, determine range bounds for plate set )
pub fn DSKRB2(
    NV: i32,
    VRTCES: &[f64],
    NP: i32,
    PLATES: &[i32],
    CORSYS: i32,
    CORPAR: &[f64],
    MNCOR3: &mut f64,
    MXCOR3: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let VRTCES = DummyArray2D::new(VRTCES, 1..=3, 1..);
    let PLATES = DummyArray2D::new(PLATES, 1..=3, 1..);
    let CORPAR = DummyArray::new(CORPAR, 1..);
    let mut ALT: f64 = 0.0;
    let mut CENTER = StackArray::<f64, 3>::new(1..=3);
    let mut DIST: f64 = 0.0;
    let mut F: f64 = 0.0;
    let mut LAT: f64 = 0.0;
    let mut LON: f64 = 0.0;
    let mut MAXD: f64 = 0.0;
    let mut PNEAR = StackArray::<f64, 3>::new(1..=3);
    let mut RE: f64 = 0.0;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //
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

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"DSKRB2", ctx)?;

    if (CORSYS == LATSYS) {
        //
        // The coordinate system is latitudinal.
        //
        // Compute radius bounds. Start with the maximum radius.
        // This is simply the maximum norm of the vertices.
        //
        *MXCOR3 = 0.0;

        for I in 1..=NV {
            *MXCOR3 = intrinsics::DMAX1(&[VNORM(VRTCES.subarray([1, I])), *MXCOR3]);
        }

        //
        // Compute the minimum radius of the plate set.
        //
        *MNCOR3 = DPMAX();

        for I in 1..=NP {
            PLTNP(
                save.ORIGIN.as_slice(),
                VRTCES.subarray([1, PLATES[[1, I]]]),
                VRTCES.subarray([1, PLATES[[2, I]]]),
                VRTCES.subarray([1, PLATES[[3, I]]]),
                PNEAR.as_slice_mut(),
                &mut DIST,
                ctx,
            )?;

            *MNCOR3 = intrinsics::DMIN1(&[DIST, *MNCOR3]);
        }
    } else if (CORSYS == RECSYS) {
        //
        // The coordinate system is rectangular. Compute the range
        // of Z-coordinates of the plates.
        //
        *MNCOR3 = DPMAX();
        *MXCOR3 = DPMIN();

        for I in 1..=NV {
            *MNCOR3 = intrinsics::DMIN1(&[*MNCOR3, VRTCES[[3, I]]]);
            *MXCOR3 = intrinsics::DMAX1(&[*MXCOR3, VRTCES[[3, I]]]);
        }
    } else if (CORSYS == PDTSYS) {
        //
        // The coordinate system is planetodetic. Compute the range
        // of altitudes of the plates.
        //
        RE = CORPAR[1];
        F = CORPAR[2];

        *MXCOR3 = DPMIN();
        *MNCOR3 = DPMAX();

        //
        // The maximum altitude is attained at a plate vertex.
        //
        for I in 1..=NV {
            RECGEO(
                VRTCES.subarray([1, I]),
                RE,
                F,
                &mut LON,
                &mut LAT,
                &mut ALT,
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(b"DSKRB2", ctx)?;
                return Ok(());
            }

            *MXCOR3 = intrinsics::DMAX1(&[*MXCOR3, ALT]);
        }

        //
        // For the Ith plate, let DMAX(I) be the maximum distance between
        // the plate's center and any of the plate's vertices.
        //
        // The minimum altitude is greater than or equal to
        // the minimum of
        //
        //    {altitude of the Ith plate's center - DMAX(I)}
        //
        // taken over all plates.
        //
        for I in 1..=NP {
            VLCOM3(
                THIRD,
                VRTCES.subarray([1, PLATES[[1, I]]]),
                THIRD,
                VRTCES.subarray([1, PLATES[[2, I]]]),
                THIRD,
                VRTCES.subarray([1, PLATES[[3, I]]]),
                CENTER.as_slice_mut(),
            );

            MAXD = intrinsics::DMAX1(&[
                VDIST(VRTCES.subarray([1, PLATES[[1, I]]]), CENTER.as_slice()),
                VDIST(VRTCES.subarray([1, PLATES[[2, I]]]), CENTER.as_slice()),
                VDIST(VRTCES.subarray([1, PLATES[[3, I]]]), CENTER.as_slice()),
            ]);

            RECGEO(CENTER.as_slice(), RE, F, &mut LON, &mut LAT, &mut ALT, ctx)?;

            if FAILED(ctx) {
                CHKOUT(b"DSKRB2", ctx)?;
                return Ok(());
            }

            *MNCOR3 = intrinsics::DMIN1(&[*MNCOR3, (ALT - MAXD)]);
        }
    } else {
        SETMSG(b"Coordinate system # is not supported.", ctx);
        ERRINT(b"#", CORSYS, ctx);
        SIGERR(b"SPICE(NOTSUPPORTED)", ctx)?;
        CHKOUT(b"DSKRB2", ctx)?;
        return Ok(());
    }

    CHKOUT(b"DSKRB2", ctx)?;
    Ok(())
}
