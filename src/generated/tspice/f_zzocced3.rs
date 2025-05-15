//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const TOTAL1: i32 = -3;
const ANNLR1: i32 = -2;
const PARTL1: i32 = -1;
const NOOCC: i32 = 0;
const PARTL2: i32 = 1;
const ANNLR2: i32 = 2;
const TOTAL2: i32 = 3;
const ANGTOL: f64 = 0.000000000001;
const CNVTOL: f64 = 0.00000000000001;
const ADJTOL: f64 = 0.00000000000001;
const LNSIZE: i32 = 255;
const MAXITR: i32 = 2048;
const UBEL: i32 = 9;
const UBPL: i32 = 4;
const NCASE1: i32 = 200;
const NCASE2: i32 = 200;

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

//$Procedure      F_ZZOCCED3 ( Test ellipsoid occultation routine )
pub fn F_ZZOCCED3(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut TITLE = [b' '; LNSIZE as usize];
    let mut ALT: f64 = 0.0;
    let mut ALPHA: f64 = 0.0;
    let mut ANGLE: f64 = 0.0;
    let mut AXIS = StackArray::<f64, 3>::new(1..=3);
    let mut BASIS = StackArray3D::<f64, 18>::new(1..=3, 1..=3, 1..=2);
    let mut BETA: f64 = 0.0;
    let mut BIGSEP: f64 = 0.0;
    let mut CENTR1 = StackArray::<f64, 3>::new(1..=3);
    let mut CENTR2 = StackArray::<f64, 3>::new(1..=3);
    let mut CTRSEP: f64 = 0.0;
    let mut CHOP = StackArray::<f64, 4>::new(1..=UBPL);
    let mut DIFF: f64 = 0.0;
    let mut DIST = StackArray::<f64, 2>::new(1..=2);
    let mut GAMMA: f64 = 0.0;
    let mut HIGH: f64 = 0.0;
    let mut LAT: f64 = 0.0;
    let mut LIMB = StackArray::<f64, 9>::new(1..=UBEL);
    let mut LIMIT: f64 = 0.0;
    let mut LMBCTR = StackArray::<f64, 3>::new(1..=3);
    let mut LMBMAJ = StackArray::<f64, 3>::new(1..=3);
    let mut LMBMIN = StackArray::<f64, 3>::new(1..=3);
    let mut LON: f64 = 0.0;
    let mut LOW: f64 = 0.0;
    let mut LVIEW = StackArray::<f64, 3>::new(1..=3);
    let mut MAXANG = StackArray::<f64, 2>::new(1..=2);
    let mut MAXRAD = StackArray::<f64, 2>::new(1..=2);
    let mut MIDPT: f64 = 0.0;
    let mut MINANG = StackArray::<f64, 2>::new(1..=2);
    let mut MINRAD = StackArray::<f64, 3>::new(1..=3);
    let mut NEWCTR = StackArray::<f64, 3>::new(1..=3);
    let mut NORMAL = StackArray::<f64, 3>::new(1..=3);
    let mut R = StackArray2D::<f64, 6>::new(1..=3, 1..=2);
    let mut RADSUM: f64 = 0.0;
    let mut RMAT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut RSIGN: f64 = 0.0;
    let mut RVEC = StackArray::<f64, 3>::new(1..=3);
    let mut SCALE: f64 = 0.0;
    let mut SEMAX1 = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut SEMAX2 = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut SRFDIR = StackArray::<f64, 3>::new(1..=3);
    let mut TSTVEC = StackArray::<f64, 3>::new(1..=3);
    let mut XPT = StackArray::<f64, 3>::new(1..=3);
    let mut UVPOFF = StackArray::<f64, 3>::new(1..=3);
    let mut V1 = StackArray::<f64, 3>::new(1..=3);
    let mut V2 = StackArray::<f64, 3>::new(1..=3);
    let mut V3 = StackArray::<f64, 3>::new(1..=3);
    let mut VIEWPT = StackArray::<f64, 3>::new(1..=3);
    let mut VPCTR1 = StackArray::<f64, 3>::new(1..=3);
    let mut VPCTR2 = StackArray::<f64, 3>::new(1..=3);
    let mut VPOFF = StackArray::<f64, 3>::new(1..=3);
    let mut VPOFF2 = StackArray::<f64, 3>::new(1..=3);
    let mut VPXPT1 = StackArray::<f64, 3>::new(1..=3);
    let mut XLIMB = StackArray::<f64, 9>::new(1..=UBEL);
    let mut XPT1 = StackArray::<f64, 3>::new(1..=3);
    let mut XPT2 = StackArray::<f64, 3>::new(1..=3);
    let mut XSEP: f64 = 0.0;
    let mut Y = StackArray::<f64, 3>::new(1..=3);
    let mut Z = StackArray::<f64, 3>::new(1..=3);
    let mut CODE: i32 = 0;
    let mut NITR: i32 = 0;
    let mut NXPTS: i32 = 0;
    let mut SEED: i32 = 0;
    let mut FOUND: bool = false;

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
    // Recommendation:  increase the parameters NCASE1 and NCASE2
    // to 1000 for robust testing.
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
    // Begin every test family with an open call.
    //
    testutil::TOPEN(b"F_ZZOCCED3", ctx)?;

    //
    // The initial set of cases tests the accuracy with which
    // ZZOCCED can detect transitions between total occultation
    // and partial occultation.
    //
    // In the discussion below, we use the terms "target" and
    // "ellipsoid" interchangeably
    //
    // These cases attempt to cover a broad set of geometric cases.
    // In order to do this efficiently, random numbers are used
    // to generate most inputs:
    //
    //    - A random scale factor is applied uniformly to all
    //      participating objects.
    //
    //    - The principal axis matrices of both targets are
    //      generated from random Euler angles.
    //
    //    - The radii of each target are chosen randomly, with
    //      each radius in the range of 1:100 prior to scaling.
    //
    //    - The location of the observer's sub-point on the first
    //      target is selected using a direction vector with random
    //      components.  The ray emanating from the center of the
    //      first target and parallel to this direction vector
    //      defines the sub-point.
    //
    //    - The altitude of the observer above its sub-point on the
    //      first target is selected randomly.
    //
    //    - The original position of the center of the second target
    //      is on the ray emanating from the observer and passing
    //      through the center of the first target.  The distance
    //      between the target centers is chosen randomly.
    //
    //      The second target is scaled to make sure it is in total
    //      occultation by the first target.
    //
    //    - To search for state transitions, the second target is
    //      displaced from its original position.  The displacement
    //      is accomplished by rotating the vector from the observer
    //      to the center of the second target.  This rotated vector
    //      is constrained to lie in a plane whose normal vector is
    //      orthogonal to the vector from the observer to the center
    //      of the first target.  The orientation of this normal vector
    //      is selected randomly.
    //
    //
    //
    //
    // Initialize the random number seed.
    //
    SEED = -1;

    for I in 1..=NCASE1 {
        // WRITE (*,*) '=========================='

        //
        // --- Case: ------------------------------------------------------
        //
        fstr::assign(&mut TITLE, b"\"General\" case:  search for transitions from total to partial occultation for ellipsoids of different shape, size and orientation.  Loop iteration = #.");

        spicelib::REPMI(&TITLE.clone(), b"#", I, &mut TITLE, ctx);
        testutil::TCASE(&TITLE, ctx)?;

        //
        // Pick a scale factor; we'll scale all of the participating
        // objects using this factor.
        //
        SCALE = f64::powf(10.0, testutil::T_RANDD(-10.0, 10.0, &mut SEED, ctx)?);

        //
        // Create random orientation matrices for both ellipsoids.  We
        // start with three Euler angles.  Also pick random radii for
        // the ellipsoids.
        //
        for J in 1..=2 {
            ALPHA = testutil::T_RANDD(-spicelib::PI(ctx), spicelib::PI(ctx), &mut SEED, ctx)?;
            BETA = testutil::T_RANDD(
                -spicelib::HALFPI(ctx),
                spicelib::HALFPI(ctx),
                &mut SEED,
                ctx,
            )?;
            GAMMA = testutil::T_RANDD(-spicelib::PI(ctx), spicelib::PI(ctx), &mut SEED, ctx)?;

            spicelib::EUL2M(ALPHA, BETA, GAMMA, 1, 2, 3, RMAT.as_slice_mut(), ctx)?;

            spicelib::XPOSE(RMAT.as_slice(), BASIS.subarray_mut([1, 1, J]));

            //
            // Pick unscaled radius values in the range 1:10 for the
            // Jth ellipsoid.
            //
            R[[1, J]] = testutil::T_RANDD(1.0, 100.0, &mut SEED, ctx)?;
            R[[2, J]] = testutil::T_RANDD(1.0, 100.0, &mut SEED, ctx)?;
            R[[3, J]] = testutil::T_RANDD(1.0, 100.0, &mut SEED, ctx)?;

            //
            // Scale the radii using our global scale factor.
            //
            spicelib::VSCLIP(SCALE, R.subarray_mut([1, J]));

            //
            // Save the minimum and maximum radii of each ellipsoid.
            //
            MINRAD[J] = intrinsics::DMIN1(&[R[[1, J]], R[[2, J]], R[[3, J]]]);
            MAXRAD[J] = intrinsics::DMAX1(&[R[[1, J]], R[[2, J]], R[[3, J]]]);

            // WRITE (*,*) 'MINRAD, MAXRAD = ', MINRAD(J), MAXRAD(J)

            //
            // Create the Jth semi-axis matrix by scaling the column
            // vectors of RMAT.
            //
            for K in 1..=3 {
                if (J == 1) {
                    spicelib::VSCL(
                        R[[K, J]],
                        BASIS.subarray([1, K, 1]),
                        SEMAX1.subarray_mut([1, K]),
                    );
                } else {
                    spicelib::VSCL(
                        R[[K, J]],
                        BASIS.subarray([1, K, 2]),
                        SEMAX2.subarray_mut([1, K]),
                    );
                }
            }
        }

        //
        // Pick a center for the first ellipsoid.
        //
        for J in 1..=3 {
            CENTR1[J] = (SCALE * testutil::T_RANDD(-1000.0, 1000.0, &mut SEED, ctx)?);
        }

        //
        // Pick a random viewing point.  Start by picking longitude
        // and latitude of a surface point on the first ellipsoid.
        //
        LON = testutil::T_RANDD(-spicelib::PI(ctx), spicelib::PI(ctx), &mut SEED, ctx)?;
        LAT = testutil::T_RANDD(
            -spicelib::HALFPI(ctx),
            spicelib::HALFPI(ctx),
            &mut SEED,
            ctx,
        )?;

        spicelib::LATREC(1.0, LON, LAT, SRFDIR.as_slice_mut());

        //
        // Since the origin of this ray is the center of the
        // ellipsoid, we don't have to check the found flag.
        //
        spicelib::SURFPT(
            save.ORIGIN.as_slice(),
            SRFDIR.as_slice(),
            R[[1, 1]],
            R[[2, 1]],
            R[[3, 1]],
            XPT.as_slice_mut(),
            &mut FOUND,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

        //
        // Pick a random altitude.
        //
        ALT = (SCALE * f64::powf(10.0, testutil::T_RANDD(-1.0, 4.0, &mut SEED, ctx)?));

        //
        // Find the local outward unit surface normal, scale it by
        // the altitude, and add it to XPT to obtain the view point.
        //
        spicelib::SURFNM(
            R[[1, 1]],
            R[[2, 1]],
            R[[3, 1]],
            XPT.as_slice(),
            NORMAL.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::VLCOM(
            1.0,
            XPT.as_slice(),
            ALT,
            NORMAL.as_slice(),
            VIEWPT.as_slice_mut(),
        );

        //
        //
        // We want to find the limb of the first ellipsoid as seen
        // from the viewing point.
        //
        // Express the viewing point as an offset from the center
        // of the first ellipsoid.
        //
        spicelib::VSUB(VIEWPT.as_slice(), CENTR1.as_slice(), VPOFF.as_slice_mut());

        //
        // Transform the viewing point into the principal axis frame
        // of the first ellipsoid.
        //
        spicelib::MTXV(
            BASIS.subarray([1, 1, 1]),
            VPOFF.as_slice(),
            LVIEW.as_slice_mut(),
        );

        //
        // Find the limb of the ellipsoid.  Rotate the limb back to the
        // original reference frame and shift the limb center to reflect
        // the offset of the first ellipsoid's center from the origin.
        //
        spicelib::EDLIMB(
            R[[1, 1]],
            R[[2, 1]],
            R[[3, 1]],
            LVIEW.as_slice(),
            XLIMB.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::EL2CGV(
            XLIMB.as_slice(),
            V1.as_slice_mut(),
            V2.as_slice_mut(),
            V3.as_slice_mut(),
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::MXV(
            BASIS.subarray([1, 1, 1]),
            V1.as_slice(),
            LMBCTR.as_slice_mut(),
        );
        spicelib::MXV(
            BASIS.subarray([1, 1, 1]),
            V2.as_slice(),
            LMBMAJ.as_slice_mut(),
        );
        spicelib::MXV(
            BASIS.subarray([1, 1, 1]),
            V3.as_slice(),
            LMBMIN.as_slice_mut(),
        );

        spicelib::VADD(CENTR1.as_slice(), LMBCTR.as_slice(), V1.as_slice_mut());
        spicelib::VEQU(V1.as_slice(), LMBCTR.as_slice_mut());

        //
        // Construct the limb in the original frame.
        //
        spicelib::CGV2EL(
            LMBCTR.as_slice(),
            LMBMAJ.as_slice(),
            LMBMIN.as_slice(),
            LIMB.as_slice_mut(),
            ctx,
        )?;

        //
        // Determine the center of the second ellipsoid.  This ellipsoid
        // is centered on the ray from the view point through the center
        // of the first ellipsoid.
        //
        // The second ellipsoid must be placed far enough from the first
        // so that no overlap of the ellipsoids occurs.
        //
        DIST[1] = spicelib::VNORM(VPOFF.as_slice());

        //
        // Let UVPOFF be the unit vector pointing from the view point
        // to the center of the first ellipsoid.
        //
        spicelib::VMINUS(VPOFF.as_slice(), UVPOFF.as_slice_mut());
        spicelib::VHATIP(UVPOFF.as_slice_mut());

        //
        // Pick a random distance between the centers; the distance
        // must be at least 1.25 * the sum of the maximum radii of
        // the ellipsoids.  Here 1.25 is an arbitrary factor "slightly"
        // greater than 1.
        //
        RADSUM = (MAXRAD[1] + MAXRAD[2]);

        CTRSEP = testutil::T_RANDD((1.25 * RADSUM), (100.0 * RADSUM), &mut SEED, ctx)?;

        //
        // The second center is placed at distance CTRSEP along
        // the ray emanating from the first center in direction UVPOFF.
        //
        spicelib::VLCOM(
            1.0,
            CENTR1.as_slice(),
            CTRSEP,
            UVPOFF.as_slice(),
            CENTR2.as_slice_mut(),
        );

        //
        // Now make sure the second ellipsoid is occulted.  We'll
        // adjust its size if necessary.
        //
        // Find a lower bound on the angular radius, as seen from the
        // view point, of the first ellipsoid.
        //
        MINANG[1] = spicelib::DASINE((MINRAD[1] / DIST[1]), 0.0, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Find the maximum angular radius of the second ellipsoid,
        // based on its nominal radii.
        //
        DIST[2] = (DIST[1] + CTRSEP);

        MAXANG[2] = spicelib::DASINE((MAXRAD[2] / DIST[2]), 0.0, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        MINANG[2] = 0.0;

        // MINANG(2) = DASINE ( MINRAD(2)/DIST(2), 0.D0 )
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // If we can't guarantee the second ellipsoid is occulted based
        // on its maximum angular radius, scale it down until it "fits."
        //
        if (MAXANG[2] >= MINANG[1]) {
            //
            // Find the limit on MAXRAD(2).  We pick LIMIT so that
            //
            //    ASIN ( LIMIT/DIST(2) ) < MINANG(1)
            //
            LIMIT = ((0.5 * DIST[2]) * f64::sin(MINANG[1]));

            //
            // Scale down the radii of the second ellipsoid.
            //
            spicelib::VSCLIP((LIMIT / MAXRAD[2]), R.subarray_mut([1, 2]));

            MINRAD[2] = intrinsics::DMIN1(&[R[[1, 2]], R[[2, 2]], R[[3, 2]]]);
            MAXRAD[2] = intrinsics::DMAX1(&[R[[1, 2]], R[[2, 2]], R[[3, 2]]]);

            //
            // We must re-create the second semi-axis matrix too.
            //
            for J in 1..=3 {
                spicelib::VSCL(
                    R[[J, 2]],
                    BASIS.subarray([1, J, 2]),
                    SEMAX2.subarray_mut([1, J]),
                );
            }
        }

        //
        // Sanity check:  validate MAXANG(2).
        //
        MAXANG[2] = spicelib::DASINE((MAXRAD[2] / DIST[2]), 0.0, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSD(b"MAXANG(2)", MAXANG[2], b"<", MINANG[1], 0.0, OK, ctx)?;

        //
        // The second ellipsoid should be in total occultation
        // by the first.  Verify this.
        //
        CODE = spicelib::ZZOCCED(
            VIEWPT.as_slice(),
            CENTR1.as_slice(),
            SEMAX1.as_slice(),
            CENTR2.as_slice(),
            SEMAX2.as_slice(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSI(
            b"ZZOCCED CODE (initial total occultation)",
            CODE,
            b"=",
            TOTAL2,
            0,
            OK,
            ctx,
        )?;

        CODE = T_OCCED(
            VIEWPT.as_slice(),
            CENTR1.as_slice(),
            SEMAX1.as_slice(),
            CENTR2.as_slice(),
            SEMAX2.as_slice(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSI(
            b"T_OCCED CODE (initial total occultation)",
            CODE,
            b"=",
            TOTAL2,
            0,
            OK,
            ctx,
        )?;

        //
        // Repeat with arguments switched.
        //
        CODE = spicelib::ZZOCCED(
            VIEWPT.as_slice(),
            CENTR2.as_slice(),
            SEMAX2.as_slice(),
            CENTR1.as_slice(),
            SEMAX1.as_slice(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSI(
            b"ZZOCCED CODE (initial total occ/switched)",
            CODE,
            b"=",
            TOTAL1,
            0,
            OK,
            ctx,
        )?;

        CODE = T_OCCED(
            VIEWPT.as_slice(),
            CENTR2.as_slice(),
            SEMAX2.as_slice(),
            CENTR1.as_slice(),
            SEMAX1.as_slice(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSI(
            b"T_OCCED CODE (initial total occ/switched)",
            CODE,
            b"=",
            TOTAL1,
            0,
            OK,
            ctx,
        )?;

        //
        // Now we're going to displace the second ellipsoid in
        // a direction orthogonal to the vector UVPOFF.  We'll
        // do this by selecting a rotation axis and rotating the
        // vector from the viewpoint to the center of the second
        // ellipsoid about this axis until we detect a change of
        // occultation classification (from total to partial).
        // We'll then make sure that both ZZOCCED and our alternative
        // computation performed by T_OCCED agree that we have a
        // transition at the same angle.
        //
        // We're now going to pick a random vector orthogonal to
        // UVPOFF.  First pick an orthogonal basis, with UVPOFF the
        // first vector of the basis.
        //
        spicelib::FRAME(UVPOFF.as_slice_mut(), Y.as_slice_mut(), Z.as_slice_mut());

        //
        // Pick a random rotation angle; we'll rotate Y about X by
        // this angle to create a rotation axis.
        //
        ANGLE = testutil::T_RANDD(-spicelib::PI(ctx), spicelib::PI(ctx), &mut SEED, ctx)?;

        spicelib::VROTV(Y.as_slice(), UVPOFF.as_slice(), ANGLE, AXIS.as_slice_mut());
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // We're now going to bracket the rotation angle needed to
        // find the occultation state transition from total to partial.
        // If the vector from the view point to the center of the
        // second ellipsoid intersects the limb of the first ellipsoid,
        // we definitely have a partial occultation, so determine
        // the angular displacement required to make this happen.
        //
        // Create the plane containing the second center and normal
        // to the rotation axis.
        //
        spicelib::NVP2PL(AXIS.as_slice(), CENTR2.as_slice(), CHOP.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Chop the first limb with the plane.
        //
        spicelib::INELPL(
            LIMB.as_slice(),
            CHOP.as_slice(),
            &mut NXPTS,
            XPT1.as_slice_mut(),
            XPT2.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // If we don't have two points of intersection, something's
        // wrong.
        //
        testutil::CHCKSI(b"NXPTS", NXPTS, b"=", 2, 0, OK, ctx)?;

        //
        // Find the vector from the view point to the first
        // intersection point XPT1; find the angular separation
        // of this vector from UVPOFF.
        //
        spicelib::VSUB(XPT1.as_slice(), VIEWPT.as_slice(), VPXPT1.as_slice_mut());

        XSEP = spicelib::VSEP(VPXPT1.as_slice(), UVPOFF.as_slice(), ctx);

        //
        // Determine the sign of the rotation angle by which
        // we rotate UVPOFF to align it with VPXPT1.
        //
        spicelib::VROTV(
            UVPOFF.as_slice(),
            AXIS.as_slice(),
            XSEP,
            TSTVEC.as_slice_mut(),
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if (spicelib::VSEP(TSTVEC.as_slice(), VPXPT1.as_slice(), ctx) > ANGTOL) {
            XSEP = -XSEP;
            spicelib::VROTV(
                UVPOFF.as_slice(),
                AXIS.as_slice(),
                XSEP,
                TSTVEC.as_slice_mut(),
            );
        }

        testutil::CHCKSD(
            b"VSEP(TSTVEC, VPXPT1)",
            spicelib::VSEP(TSTVEC.as_slice(), VPXPT1.as_slice(), ctx),
            b"~",
            0.0,
            ANGTOL,
            OK,
            ctx,
        )?;

        //
        // Time for another sanity check:  rotate the vector from
        // the viewpoint to the center of the second ellipsoid about
        // AXIS by XSEP, then add this to the view point,
        // yielding a new center vector for the second ellipsoid.
        // verify that the ellipsoid is in partial occultation.
        //
        spicelib::VSUB(CENTR2.as_slice(), VIEWPT.as_slice(), VPCTR2.as_slice_mut());

        spicelib::VROTV(
            VPCTR2.as_slice(),
            AXIS.as_slice(),
            XSEP,
            RVEC.as_slice_mut(),
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Sanity check:  make sure RVEC points from the view point
        // to XPT1.
        //
        testutil::CHCKSD(
            b"VSEP(RVEC, VPXPT1)",
            spicelib::VSEP(RVEC.as_slice(), VPXPT1.as_slice(), ctx),
            b"~",
            0.0,
            ANGTOL,
            OK,
            ctx,
        )?;

        spicelib::VADD(VIEWPT.as_slice(), RVEC.as_slice(), NEWCTR.as_slice_mut());

        // CALL VSUB  ( NEWCTR, VIEWPT, TSTVEC )
        // WRITE (*,*) 'VPXPT1 sep = ', VSEP ( TSTVEC, VPXPT1 )
        // WRITE (*,*) 'VPCTR2 sep = ', VSEP ( TSTVEC, VPCTR2 )

        //
        // Check for partial occultation of the second ellipsoid.
        //
        CODE = T_OCCED(
            VIEWPT.as_slice(),
            CENTR1.as_slice(),
            SEMAX1.as_slice(),
            NEWCTR.as_slice(),
            SEMAX2.as_slice(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSI(
            b"T_OCCED CODE, (initial partial occultation)",
            CODE,
            b"=",
            PARTL2,
            0,
            OK,
            ctx,
        )?;

        CODE = spicelib::ZZOCCED(
            VIEWPT.as_slice(),
            CENTR1.as_slice(),
            SEMAX1.as_slice(),
            NEWCTR.as_slice(),
            SEMAX2.as_slice(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSI(
            b"ZZOCCED CODE, (initial partial occultation)",
            CODE,
            b"=",
            PARTL2,
            0,
            OK,
            ctx,
        )?;

        //
        // Repeat with arguments switched.
        //
        CODE = spicelib::ZZOCCED(
            VIEWPT.as_slice(),
            CENTR2.as_slice(),
            SEMAX2.as_slice(),
            CENTR1.as_slice(),
            SEMAX1.as_slice(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSI(
            b"ZZOCCED CODE (initial partial occ/switched)",
            CODE,
            b"=",
            TOTAL1,
            0,
            OK,
            ctx,
        )?;

        CODE = T_OCCED(
            VIEWPT.as_slice(),
            CENTR2.as_slice(),
            SEMAX2.as_slice(),
            CENTR1.as_slice(),
            SEMAX1.as_slice(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSI(
            b"T_OCCED CODE (initial partial occ/switched)",
            CODE,
            b"=",
            TOTAL1,
            0,
            OK,
            ctx,
        )?;

        //
        // We now know that a rotation of the vector from the
        // view point to the center of the second ellipsoid about
        // AXIS by an angle between 0 and XSEP radians should yield
        // a state transition.  Find the angle via binary search.
        //
        if (XSEP > 0.0) {
            RSIGN = 1.0;
        } else {
            RSIGN = -1.0;
        }

        LOW = 0.0;
        HIGH = f64::abs(XSEP);
        DIFF = (HIGH - LOW);
        NITR = 0;

        while ((DIFF > CNVTOL) && *OK) {
            NITR = (NITR + 1);

            testutil::CHCKSI(b"NITR", NITR, b"<", MAXITR, 0, OK, ctx)?;

            MIDPT = ((HIGH + LOW) / 2.0);

            spicelib::VROTV(
                VPCTR2.as_slice(),
                AXIS.as_slice(),
                (RSIGN * MIDPT),
                RVEC.as_slice_mut(),
            );
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::VADD(VIEWPT.as_slice(), RVEC.as_slice(), NEWCTR.as_slice_mut());

            CODE = spicelib::ZZOCCED(
                VIEWPT.as_slice(),
                CENTR1.as_slice(),
                SEMAX1.as_slice(),
                NEWCTR.as_slice(),
                SEMAX2.as_slice(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            if (CODE == PARTL2) {
                HIGH = MIDPT;
            } else {
                //
                // The code had better be TOTAL2.
                //
                testutil::CHCKSI(b"(bisection) CODE", CODE, b"=", TOTAL2, 0, OK, ctx)?;

                LOW = MIDPT;
            }

            DIFF = (HIGH - LOW);
        }

        //
        // Now that we've dropped out of the loop, verify that T_OCCED
        // says the occultation is total at rotation angle RSIGN*LOW and
        // partial at rotation angle RSIGN*HIGH.  We adjust each
        // of these angles by ADJTOL to allow for differences in
        // round-off between ZZOCCED and T_OCCED.
        //
        //
        // Verify that when we switch the order of the ellipsoids,
        // we see a transition from total occultation of ellipsoid 1
        // to partial occultation of ellipsoid 1.
        //
        //
        // Check for total occultation:
        //
        spicelib::VROTV(
            VPCTR2.as_slice(),
            AXIS.as_slice(),
            (RSIGN * (LOW - ADJTOL)),
            RVEC.as_slice_mut(),
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::VADD(VIEWPT.as_slice(), RVEC.as_slice(), NEWCTR.as_slice_mut());

        CODE = T_OCCED(
            VIEWPT.as_slice(),
            CENTR1.as_slice(),
            SEMAX1.as_slice(),
            NEWCTR.as_slice(),
            SEMAX2.as_slice(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSI(b"CODE from T_OCCED (total)", CODE, b"=", TOTAL2, 0, OK, ctx)?;

        CODE = T_OCCED(
            VIEWPT.as_slice(),
            NEWCTR.as_slice(),
            SEMAX2.as_slice(),
            CENTR1.as_slice(),
            SEMAX1.as_slice(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSI(
            b"CODE from T_OCCED (total, args switched)",
            CODE,
            b"=",
            TOTAL1,
            0,
            OK,
            ctx,
        )?;

        //
        // Check for partial occultation:
        //
        spicelib::VROTV(
            VPCTR2.as_slice(),
            AXIS.as_slice(),
            (RSIGN * (HIGH + ADJTOL)),
            RVEC.as_slice_mut(),
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::VADD(VIEWPT.as_slice(), RVEC.as_slice(), NEWCTR.as_slice_mut());

        CODE = T_OCCED(
            VIEWPT.as_slice(),
            CENTR1.as_slice(),
            SEMAX1.as_slice(),
            NEWCTR.as_slice(),
            SEMAX2.as_slice(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSI(
            b"CODE from T_OCCED (partial 0)",
            CODE,
            b"=",
            PARTL2,
            0,
            OK,
            ctx,
        )?;

        CODE = T_OCCED(
            VIEWPT.as_slice(),
            NEWCTR.as_slice(),
            SEMAX2.as_slice(),
            CENTR1.as_slice(),
            SEMAX1.as_slice(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSI(
            b"CODE from T_OCCED (partial 0, args switched)",
            CODE,
            b"=",
            PARTL1,
            0,
            OK,
            ctx,
        )?;

        //
        // --- Case: ------------------------------------------------------
        //
        fstr::assign(&mut TITLE, b"\"General\" case:  search for transitions from partial occultation to no occultation, for ellipsoids of different shape, size and orientation.  Loop iteration = #.");

        spicelib::REPMI(&TITLE.clone(), b"#", I, &mut TITLE, ctx);
        testutil::TCASE(&TITLE, ctx)?;
        //
        // The next step is to test for transitions from partial
        // occultation to no occultation.
        //
        // Let BIGSEP be a displacement angle large enough to guarantee
        // that no occultation will be found.
        //
        BIGSEP = spicelib::PI(ctx);

        spicelib::VROTV(
            VPCTR2.as_slice(),
            AXIS.as_slice(),
            (RSIGN * BIGSEP),
            RVEC.as_slice_mut(),
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::VADD(VIEWPT.as_slice(), RVEC.as_slice(), NEWCTR.as_slice_mut());

        CODE = spicelib::ZZOCCED(
            VIEWPT.as_slice(),
            CENTR1.as_slice(),
            SEMAX1.as_slice(),
            NEWCTR.as_slice(),
            SEMAX2.as_slice(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSI(b"CODE", CODE, b"=", NOOCC, 0, OK, ctx)?;

        //
        // We now know that a rotation of the vector from the
        // view point to the center of the second ellipsoid about
        // AXIS by an angle between 0 and XSEP radians should yield
        // a state transition.  Find the angle via binary search.
        //
        LOW = f64::abs(XSEP);
        HIGH = BIGSEP;
        DIFF = (HIGH - LOW);
        NITR = 0;

        while ((DIFF > CNVTOL) && *OK) {
            NITR = (NITR + 1);

            testutil::CHCKSI(b"NITR", NITR, b"<", MAXITR, 0, OK, ctx)?;

            MIDPT = ((HIGH + LOW) / 2.0);

            spicelib::VROTV(
                VPCTR2.as_slice(),
                AXIS.as_slice(),
                (RSIGN * MIDPT),
                RVEC.as_slice_mut(),
            );
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::VADD(VIEWPT.as_slice(), RVEC.as_slice(), NEWCTR.as_slice_mut());

            CODE = spicelib::ZZOCCED(
                VIEWPT.as_slice(),
                CENTR1.as_slice(),
                SEMAX1.as_slice(),
                NEWCTR.as_slice(),
                SEMAX2.as_slice(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            if (CODE == NOOCC) {
                HIGH = MIDPT;
            } else {
                //
                // The code had better be PARTL2.
                //
                testutil::CHCKSI(b"(bisection) CODE", CODE, b"=", PARTL2, 0, OK, ctx)?;

                LOW = MIDPT;
            }

            DIFF = (HIGH - LOW);
        }

        //
        // Now that we've dropped out of the loop, verify that T_OCCED
        // says the occultation is partial at rotation angle RSIGN*LOW and
        // "none" at rotation angle RSIGN*HIGH.  We adjust each
        // of these angles by CNVTOL to allow for differences in
        // round-off between ZZOCCED and T_OCCED.
        //
        //
        // Check for partial occultation:
        //
        spicelib::VROTV(
            VPCTR2.as_slice(),
            AXIS.as_slice(),
            (RSIGN * (LOW - ADJTOL)),
            RVEC.as_slice_mut(),
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::VADD(VIEWPT.as_slice(), RVEC.as_slice(), NEWCTR.as_slice_mut());

        CODE = T_OCCED(
            VIEWPT.as_slice(),
            CENTR1.as_slice(),
            SEMAX1.as_slice(),
            NEWCTR.as_slice(),
            SEMAX2.as_slice(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSI(
            b"CODE from T_OCCED (partial 1)",
            CODE,
            b"=",
            PARTL2,
            0,
            OK,
            ctx,
        )?;

        //
        // Check for no occultation:
        //
        spicelib::VROTV(
            VPCTR2.as_slice(),
            AXIS.as_slice(),
            (RSIGN * (HIGH + ADJTOL)),
            RVEC.as_slice_mut(),
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::VADD(VIEWPT.as_slice(), RVEC.as_slice(), NEWCTR.as_slice_mut());

        CODE = T_OCCED(
            VIEWPT.as_slice(),
            CENTR1.as_slice(),
            SEMAX1.as_slice(),
            NEWCTR.as_slice(),
            SEMAX2.as_slice(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSI(b"CODE from T_OCCED (none)", CODE, b"=", NOOCC, 0, OK, ctx)?;
    }

    //*****************************************************************
    //*****************************************************************
    //*****************************************************************
    //*****************************************************************
    //*****************************************************************
    //*****************************************************************
    //*****************************************************************
    //*****************************************************************
    //*****************************************************************
    //*****************************************************************
    //
    //
    //         Transit cases follow.
    //
    //
    //*****************************************************************
    //*****************************************************************
    //*****************************************************************
    //*****************************************************************
    //*****************************************************************
    //*****************************************************************
    //*****************************************************************
    //*****************************************************************
    //*****************************************************************

    //
    //
    // The following tests are quite similar to the preceding ones,
    // with the difference that now we're going to have the first
    // ellipsoid start out in annular transit across the second one.
    //
    for I in 1..=NCASE2 {
        // WRITE (*,*) '=========================='

        //
        // --- Case: ------------------------------------------------------
        //
        fstr::assign(&mut TITLE, b"\"General\" case:  search for transitions from annular to partial transit for ellipsoids of different shape, size and orientation.  Loop iteration = #.");

        spicelib::REPMI(&TITLE.clone(), b"#", I, &mut TITLE, ctx);
        testutil::TCASE(&TITLE, ctx)?;

        //
        // Pick a scale factor; we'll scale all of the participating
        // objects using this factor.
        //
        SCALE = f64::powf(10.0, testutil::T_RANDD(-10.0, 10.0, &mut SEED, ctx)?);

        //
        // Create random orientation matrices for both ellipsoids.  We
        // start with three Euler angles.  Also pick random radii for
        // the ellipsoids.
        //
        for J in 1..=2 {
            ALPHA = testutil::T_RANDD(-spicelib::PI(ctx), spicelib::PI(ctx), &mut SEED, ctx)?;
            BETA = testutil::T_RANDD(
                -spicelib::HALFPI(ctx),
                spicelib::HALFPI(ctx),
                &mut SEED,
                ctx,
            )?;
            GAMMA = testutil::T_RANDD(-spicelib::PI(ctx), spicelib::PI(ctx), &mut SEED, ctx)?;

            spicelib::EUL2M(ALPHA, BETA, GAMMA, 1, 2, 3, RMAT.as_slice_mut(), ctx)?;

            spicelib::XPOSE(RMAT.as_slice(), BASIS.subarray_mut([1, 1, J]));

            //
            // Pick unscaled radius values in the range 1:10 for the
            // Jth ellipsoid.
            //
            R[[1, J]] = testutil::T_RANDD(1.0, 100.0, &mut SEED, ctx)?;
            R[[2, J]] = testutil::T_RANDD(1.0, 100.0, &mut SEED, ctx)?;
            R[[3, J]] = testutil::T_RANDD(1.0, 100.0, &mut SEED, ctx)?;

            //
            // Scale the radii using our global scale factor.
            //
            spicelib::VSCLIP(SCALE, R.subarray_mut([1, J]));

            //
            // Save the minimum and maximum radii of each ellipsoid.
            //
            MINRAD[J] = intrinsics::DMIN1(&[R[[1, J]], R[[2, J]], R[[3, J]]]);
            MAXRAD[J] = intrinsics::DMAX1(&[R[[1, J]], R[[2, J]], R[[3, J]]]);

            // WRITE (*,*) 'MINRAD, MAXRAD = ', MINRAD(J), MAXRAD(J)

            //
            // Create the Jth semi-axis matrix by scaling the column
            // vectors of RMAT.
            //
            for K in 1..=3 {
                if (J == 1) {
                    spicelib::VSCL(
                        R[[K, J]],
                        BASIS.subarray([1, K, 1]),
                        SEMAX1.subarray_mut([1, K]),
                    );
                } else {
                    spicelib::VSCL(
                        R[[K, J]],
                        BASIS.subarray([1, K, 2]),
                        SEMAX2.subarray_mut([1, K]),
                    );
                }
            }
        }

        //
        // Pick a center for the first ellipsoid.
        //
        for J in 1..=3 {
            CENTR1[J] = (SCALE * testutil::T_RANDD(-1000.0, 1000.0, &mut SEED, ctx)?);
        }

        //
        // Pick a random viewing point.  Start by picking longitude
        // and latitude of a surface point on the first ellipsoid.
        //
        LON = testutil::T_RANDD(-spicelib::PI(ctx), spicelib::PI(ctx), &mut SEED, ctx)?;
        LAT = testutil::T_RANDD(
            -spicelib::HALFPI(ctx),
            spicelib::HALFPI(ctx),
            &mut SEED,
            ctx,
        )?;

        spicelib::LATREC(1.0, LON, LAT, SRFDIR.as_slice_mut());

        //
        // Since the origin of this ray is the center of the
        // ellipsoid, we don't have to check the found flag.
        //
        spicelib::SURFPT(
            save.ORIGIN.as_slice(),
            SRFDIR.as_slice(),
            R[[1, 1]],
            R[[2, 1]],
            R[[3, 1]],
            XPT.as_slice_mut(),
            &mut FOUND,
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

        //
        // Pick a random altitude.  Note the we move the observer
        // farther away from the target than in the total occultation
        // cases.  For those, the minimum exponent was -1.
        //
        ALT = (SCALE * f64::powf(10.0, testutil::T_RANDD(1.0, 4.0, &mut SEED, ctx)?));

        //
        // Find the local outward unit surface normal, scale it by
        // the altitude, and add it to XPT to obtain the view point.
        //
        spicelib::SURFNM(
            R[[1, 1]],
            R[[2, 1]],
            R[[3, 1]],
            XPT.as_slice(),
            NORMAL.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::VLCOM(
            1.0,
            XPT.as_slice(),
            ALT,
            NORMAL.as_slice(),
            VIEWPT.as_slice_mut(),
        );

        //
        // Find the offset from the viewing point to the center of the
        // first ellipsoid.
        //
        spicelib::VSUB(VIEWPT.as_slice(), CENTR1.as_slice(), VPOFF.as_slice_mut());
        DIST[1] = spicelib::VNORM(VPOFF.as_slice());

        //
        // Let UVPOFF be the unit vector pointing from the view point
        // to the center of the first ellipsoid.
        //
        spicelib::VMINUS(VPOFF.as_slice(), UVPOFF.as_slice_mut());
        spicelib::VHATIP(UVPOFF.as_slice_mut());

        // Determine the center of the second ellipsoid.  This ellipsoid
        // is centered on the ray from the view point through the center
        // of the first ellipsoid.
        //
        // The second ellipsoid must be placed far enough from the first
        // so that no overlap of the ellipsoids occurs.
        //
        // Pick a random distance between the centers; the distance
        // must be at least 1.25 * the sum of the maximum radii of
        // the ellipsoids.  Here 1.25 is an arbitrary factor "slightly"
        // larger than 1.
        //
        RADSUM = (MAXRAD[1] + MAXRAD[2]);

        CTRSEP = testutil::T_RANDD((1.25 * RADSUM), (100.0 * RADSUM), &mut SEED, ctx)?;

        //
        // The second center is placed at distance CTRSEP along
        // the ray emanating from the first center in direction UVPOFF.
        //
        spicelib::VLCOM(
            1.0,
            CENTR1.as_slice(),
            CTRSEP,
            UVPOFF.as_slice(),
            CENTR2.as_slice_mut(),
        );

        //
        // Now make sure the second ellipsoid is in annular occultation
        // by the first.  We'll adjust the size of the first
        // ellipsoid if necessary.
        //
        // Find a upper bound on the angular radius, as seen from the
        // view point, of the first ellipsoid.
        //
        MAXANG[1] = spicelib::DASINE((MAXRAD[1] / DIST[1]), 0.0, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Find the minimum angular radius of the second ellipsoid,
        // based on its nominal radii.
        //
        DIST[2] = (DIST[1] + CTRSEP);

        MAXANG[2] = 0.0;

        // MAXANG(2) = DASINE ( MAXRAD(2)/DIST(2), 0.D0 )
        // CALL CHCKXC ( .FALSE., ' ', OK )

        MINANG[1] = 0.0;
        MINANG[2] = spicelib::DASINE((MINRAD[2] / DIST[2]), 0.0, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // If we can't guarantee the second ellipsoid is in annular
        // occultation by the first, based on the second ellipsoid's
        // minimum angular radius, shrink the first ellipsoid until this
        // condition is met.
        //
        if (MINANG[2] <= MAXANG[1]) {
            //
            // Find the limit on MAXRAD(1).  We pick LIMIT so that
            //
            //    ASIN ( LIMIT/DIST(1) ) = MINANG(2)
            //
            LIMIT = (DIST[1] * f64::sin(MINANG[2]));

            //
            // Scale down the radii of the first ellipsoid.  Use the
            // arbitrary safety factor 0.75.
            //
            spicelib::VSCLIP(((0.75 * LIMIT) / MAXRAD[1]), R.subarray_mut([1, 1]));

            MINRAD[1] = intrinsics::DMIN1(&[R[[1, 1]], R[[2, 1]], R[[3, 1]]]);
            MAXRAD[1] = intrinsics::DMAX1(&[R[[1, 1]], R[[2, 1]], R[[3, 1]]]);

            //
            // We must re-create the first semi-axis matrix too.
            //
            for J in 1..=3 {
                spicelib::VSCL(
                    R[[J, 1]],
                    BASIS.subarray([1, J, 1]),
                    SEMAX1.subarray_mut([1, J]),
                );
            }
        }

        //
        // Sanity check:  validate MAXANG(1).
        //
        MAXANG[1] = spicelib::DASINE((MAXRAD[1] / DIST[1]), 0.0, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSD(b"MAXANG(1)", MAXANG[1], b"<", MINANG[2], 0.0, OK, ctx)?;

        //
        //
        // We want to find the limb of the *second* ellipsoid as seen
        // from the viewing point.
        //
        // Express the viewing point as an offset from the center
        // of the first ellipsoid.
        //
        spicelib::VSUB(VIEWPT.as_slice(), CENTR2.as_slice(), VPOFF2.as_slice_mut());

        //
        // Transform the viewing point into the principal axis frame
        // of the second ellipsoid.
        //
        spicelib::MTXV(
            BASIS.subarray([1, 1, 2]),
            VPOFF2.as_slice(),
            LVIEW.as_slice_mut(),
        );

        //
        // Find the limb of the second ellipsoid. Rotate the limb back to
        // the original reference frame and shift the limb center to
        // reflect the offset of the second ellipsoid's center from the
        // origin.
        //
        spicelib::EDLIMB(
            R[[1, 2]],
            R[[2, 2]],
            R[[3, 2]],
            LVIEW.as_slice(),
            XLIMB.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::EL2CGV(
            XLIMB.as_slice(),
            V1.as_slice_mut(),
            V2.as_slice_mut(),
            V3.as_slice_mut(),
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::MXV(
            BASIS.subarray([1, 1, 2]),
            V1.as_slice(),
            LMBCTR.as_slice_mut(),
        );
        spicelib::MXV(
            BASIS.subarray([1, 1, 2]),
            V2.as_slice(),
            LMBMAJ.as_slice_mut(),
        );
        spicelib::MXV(
            BASIS.subarray([1, 1, 2]),
            V3.as_slice(),
            LMBMIN.as_slice_mut(),
        );

        spicelib::VADD(CENTR2.as_slice(), LMBCTR.as_slice(), V1.as_slice_mut());
        spicelib::VEQU(V1.as_slice(), LMBCTR.as_slice_mut());

        //
        // Construct the limb of the second ellipsoid
        // in the original frame.
        //
        spicelib::CGV2EL(
            LMBCTR.as_slice(),
            LMBMAJ.as_slice(),
            LMBMIN.as_slice(),
            LIMB.as_slice_mut(),
            ctx,
        )?;

        //
        // The second ellipsoid should be in annular occultation
        // by the first.  Verify this.
        //
        CODE = spicelib::ZZOCCED(
            VIEWPT.as_slice(),
            CENTR1.as_slice(),
            SEMAX1.as_slice(),
            CENTR2.as_slice(),
            SEMAX2.as_slice(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSI(
            b"ZZOCCED CODE (initial annular occultation)",
            CODE,
            b"=",
            ANNLR2,
            0,
            OK,
            ctx,
        )?;

        CODE = T_OCCED(
            VIEWPT.as_slice(),
            CENTR1.as_slice(),
            SEMAX1.as_slice(),
            CENTR2.as_slice(),
            SEMAX2.as_slice(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSI(
            b"T_OCCED CODE (initial annular occultation)",
            CODE,
            b"=",
            ANNLR2,
            0,
            OK,
            ctx,
        )?;

        //
        // Repeat with arguments switched.
        //
        CODE = spicelib::ZZOCCED(
            VIEWPT.as_slice(),
            CENTR2.as_slice(),
            SEMAX2.as_slice(),
            CENTR1.as_slice(),
            SEMAX1.as_slice(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSI(
            b"ZZOCCED CODE (initial annular occ/switched)",
            CODE,
            b"=",
            ANNLR1,
            0,
            OK,
            ctx,
        )?;

        CODE = T_OCCED(
            VIEWPT.as_slice(),
            CENTR2.as_slice(),
            SEMAX2.as_slice(),
            CENTR1.as_slice(),
            SEMAX1.as_slice(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSI(
            b"T_OCCED CODE (initial annular occ/switched)",
            CODE,
            b"=",
            ANNLR1,
            0,
            OK,
            ctx,
        )?;

        //
        // Now we're going to displace the first ellipsoid in
        // a direction orthogonal to the vector UVPOFF.  We'll
        // do this by selecting a rotation axis and rotating the
        // vector from the viewpoint to the center of the first
        // ellipsoid about this axis until we detect a change of
        // transit classification (from annular to partial).
        // We'll then make sure that both ZZOCCED and our alternative
        // computation performed by T_OCCED agree that we have a
        // transition at the same angle.
        //
        // We're now going to pick a random vector orthogonal to
        // UVPOFF.  First pick an orthogonal basis, with UVPOFF the
        // first vector of the basis.
        //
        spicelib::FRAME(UVPOFF.as_slice_mut(), Y.as_slice_mut(), Z.as_slice_mut());

        //
        // Pick a random rotation angle; we'll rotate Y about X by
        // this angle to create a rotation axis.
        //
        ANGLE = testutil::T_RANDD(-spicelib::PI(ctx), spicelib::PI(ctx), &mut SEED, ctx)?;

        spicelib::VROTV(Y.as_slice(), UVPOFF.as_slice(), ANGLE, AXIS.as_slice_mut());
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // We're now going to bracket the rotation angle needed to find
        // the transit state transition from annular to partial. If
        // the vector from the view point to the center of the *first*
        // ellipsoid intersects the limb of the *second* ellipsoid, we
        // definitely have a partial transit, so determine the
        // angular displacement required to make this happen.
        //
        // Create the plane containing the second center and normal
        // to the rotation axis.
        //
        spicelib::NVP2PL(AXIS.as_slice(), CENTR2.as_slice(), CHOP.as_slice_mut(), ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Chop the second limb with the plane.
        //
        spicelib::INELPL(
            LIMB.as_slice(),
            CHOP.as_slice(),
            &mut NXPTS,
            XPT1.as_slice_mut(),
            XPT2.as_slice_mut(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // If we don't have two points of intersection, something's
        // wrong.
        //
        testutil::CHCKSI(b"NXPTS", NXPTS, b"=", 2, 0, OK, ctx)?;

        //
        // Find the vector from the view point to the first
        // intersection point XPT1; find the angular separation
        // of this vector from UVPOFF.
        //
        spicelib::VSUB(XPT1.as_slice(), VIEWPT.as_slice(), VPXPT1.as_slice_mut());

        XSEP = spicelib::VSEP(VPXPT1.as_slice(), UVPOFF.as_slice(), ctx);

        //
        // Determine the sign of the rotation angle by which
        // we rotate UVPOFF to align it with VPXPT1.
        //
        spicelib::VROTV(
            UVPOFF.as_slice(),
            AXIS.as_slice(),
            XSEP,
            TSTVEC.as_slice_mut(),
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        if (spicelib::VSEP(TSTVEC.as_slice(), VPXPT1.as_slice(), ctx) > ANGTOL) {
            XSEP = -XSEP;
            spicelib::VROTV(
                UVPOFF.as_slice(),
                AXIS.as_slice(),
                XSEP,
                TSTVEC.as_slice_mut(),
            );
        }

        testutil::CHCKSD(
            b"VSEP(TSTVEC, VPXPT1)",
            spicelib::VSEP(TSTVEC.as_slice(), VPXPT1.as_slice(), ctx),
            b"~",
            0.0,
            ANGTOL,
            OK,
            ctx,
        )?;

        //
        // Time for another sanity check:  rotate the vector from
        // the viewpoint to the center of the first ellipsoid about
        // AXIS by XSEP, then add this to the view point,
        // yielding a new center vector for the first ellipsoid.
        // Verify that the first ellipsoid is in partial transit.
        //
        spicelib::VSUB(CENTR1.as_slice(), VIEWPT.as_slice(), VPCTR1.as_slice_mut());

        spicelib::VROTV(
            VPCTR1.as_slice(),
            AXIS.as_slice(),
            XSEP,
            RVEC.as_slice_mut(),
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        //
        // Sanity check:  make sure RVEC points from the view point
        // to XPT1.
        //
        testutil::CHCKSD(
            b"VSEP(RVEC, VPXPT1)",
            spicelib::VSEP(RVEC.as_slice(), VPXPT1.as_slice(), ctx),
            b"~",
            0.0,
            ANGTOL,
            OK,
            ctx,
        )?;

        spicelib::VADD(VIEWPT.as_slice(), RVEC.as_slice(), NEWCTR.as_slice_mut());

        // CALL VSUB  ( NEWCTR, VIEWPT, TSTVEC )
        // WRITE (*,*) 'VPXPT1 sep = ', VSEP ( TSTVEC, VPXPT1 )
        // WRITE (*,*) 'VPCTR2 sep = ', VSEP ( TSTVEC, VPCTR2 )

        //
        // Check for partial transit of the first ellipsoid across
        // the second.
        //
        CODE = T_OCCED(
            VIEWPT.as_slice(),
            NEWCTR.as_slice(),
            SEMAX1.as_slice(),
            CENTR2.as_slice(),
            SEMAX2.as_slice(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;
        testutil::CHCKSI(
            b"T_OCCED CODE, (initial partial transit)",
            CODE,
            b"=",
            PARTL2,
            0,
            OK,
            ctx,
        )?;

        CODE = spicelib::ZZOCCED(
            VIEWPT.as_slice(),
            NEWCTR.as_slice(),
            SEMAX1.as_slice(),
            CENTR2.as_slice(),
            SEMAX2.as_slice(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSI(
            b"ZZOCCED CODE, (initial partial transit)",
            CODE,
            b"=",
            PARTL2,
            0,
            OK,
            ctx,
        )?;

        //
        // Repeat with arguments switched.
        //
        CODE = spicelib::ZZOCCED(
            VIEWPT.as_slice(),
            CENTR2.as_slice(),
            SEMAX2.as_slice(),
            NEWCTR.as_slice(),
            SEMAX1.as_slice(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSI(
            b"ZZOCCED CODE (initial partial occ/switched)",
            CODE,
            b"=",
            PARTL1,
            0,
            OK,
            ctx,
        )?;

        CODE = T_OCCED(
            VIEWPT.as_slice(),
            CENTR2.as_slice(),
            SEMAX2.as_slice(),
            NEWCTR.as_slice(),
            SEMAX1.as_slice(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSI(
            b"T_OCCED CODE (initial partial occ/switched)",
            CODE,
            b"=",
            PARTL1,
            0,
            OK,
            ctx,
        )?;

        //
        // We now know that a rotation of the vector from the
        // view point to the center of the second ellipsoid about
        // AXIS by an angle between 0 and XSEP radians should yield
        // a state transition.  Find the angle via binary search.
        //
        if (XSEP > 0.0) {
            RSIGN = 1.0;
        } else {
            RSIGN = -1.0;
        }

        LOW = 0.0;
        HIGH = f64::abs(XSEP);
        DIFF = (HIGH - LOW);
        NITR = 0;

        while ((DIFF > CNVTOL) && *OK) {
            NITR = (NITR + 1);

            testutil::CHCKSI(b"NITR", NITR, b"<", MAXITR, 0, OK, ctx)?;

            MIDPT = ((HIGH + LOW) / 2.0);

            spicelib::VROTV(
                VPCTR1.as_slice(),
                AXIS.as_slice(),
                (RSIGN * MIDPT),
                RVEC.as_slice_mut(),
            );
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::VADD(VIEWPT.as_slice(), RVEC.as_slice(), NEWCTR.as_slice_mut());

            //XXX
            CODE = spicelib::ZZOCCED(
                VIEWPT.as_slice(),
                NEWCTR.as_slice(),
                SEMAX1.as_slice(),
                CENTR2.as_slice(),
                SEMAX2.as_slice(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            if (CODE == PARTL2) {
                HIGH = MIDPT;
            } else {
                //
                // The code had better be ANNLR2.
                //
                testutil::CHCKSI(b"(bisection) CODE", CODE, b"=", ANNLR2, 0, OK, ctx)?;

                LOW = MIDPT;
            }

            DIFF = (HIGH - LOW);
        }

        //
        // Now that we've dropped out of the loop, verify that T_OCCED
        // says the transit is total at rotation angle RSIGN*LOW and
        // partial at rotation angle RSIGN*HIGH.  We adjust each
        // of these angles by ADJTOL to allow for differences in
        // round-off between ZZOCCED and T_OCCED.
        //
        //
        // Verify that when we switch the order of the ellipsoids,
        // we see a transition from total transit of ellipsoid 1
        // to partial transit of ellipsoid 1.
        //
        //
        // Check for annular transit:
        //
        spicelib::VROTV(
            VPCTR1.as_slice(),
            AXIS.as_slice(),
            (RSIGN * (LOW - ADJTOL)),
            RVEC.as_slice_mut(),
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::VADD(VIEWPT.as_slice(), RVEC.as_slice(), NEWCTR.as_slice_mut());

        CODE = T_OCCED(
            VIEWPT.as_slice(),
            NEWCTR.as_slice(),
            SEMAX1.as_slice(),
            CENTR2.as_slice(),
            SEMAX2.as_slice(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSI(
            b"CODE from T_OCCED (annular)",
            CODE,
            b"=",
            ANNLR2,
            0,
            OK,
            ctx,
        )?;

        //
        // IF ( .NOT. OK ) THEN
        //
        //    WRITE (*,*) '============='
        //    WRITE (*,*) 'XSEP = ', XSEP
        //    WRITE (*,*) 'MINANG: ', MINANG
        //    WRITE (*,*) 'MAXANG: ', MAXANG
        //    WRITE (*,*) 'LIMB   = ', LIMB
        //    WRITE (*,*) 'VIEWPT = ', VIEWPT
        //    WRITE (*,*) 'LOW, HIGH = ', LOW, HIGH
        // END IF

        CODE = T_OCCED(
            VIEWPT.as_slice(),
            CENTR2.as_slice(),
            SEMAX2.as_slice(),
            NEWCTR.as_slice(),
            SEMAX1.as_slice(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSI(
            b"CODE from T_OCCED (annular, args switched)",
            CODE,
            b"=",
            ANNLR1,
            0,
            OK,
            ctx,
        )?;

        //
        // Check for partial transit:
        //
        spicelib::VROTV(
            VPCTR1.as_slice(),
            AXIS.as_slice(),
            (RSIGN * (HIGH + ADJTOL)),
            RVEC.as_slice_mut(),
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::VADD(VIEWPT.as_slice(), RVEC.as_slice(), NEWCTR.as_slice_mut());

        CODE = T_OCCED(
            VIEWPT.as_slice(),
            NEWCTR.as_slice(),
            SEMAX1.as_slice(),
            CENTR2.as_slice(),
            SEMAX2.as_slice(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSI(
            b"CODE from T_OCCED (partial 0)",
            CODE,
            b"=",
            PARTL2,
            0,
            OK,
            ctx,
        )?;

        //
        // IF ( .NOT. OK ) THEN
        //
        //    WRITE (*,*) '============='
        //    WRITE (*,*) 'XSEP = ', XSEP
        //    WRITE (*,*) 'MINANG: ', MINANG
        //    WRITE (*,*) 'MAXANG: ', MAXANG
        //    WRITE (*,*) 'LIMB   = ', LIMB
        //    WRITE (*,*) 'VIEWPT = ', VIEWPT
        //    WRITE (*,*) 'LOW, HIGH = ', LOW, HIGH
        // END IF
        //
        CODE = T_OCCED(
            VIEWPT.as_slice(),
            CENTR2.as_slice(),
            SEMAX2.as_slice(),
            NEWCTR.as_slice(),
            SEMAX1.as_slice(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSI(
            b"CODE from T_OCCED (partial 0, args switched)",
            CODE,
            b"=",
            PARTL1,
            0,
            OK,
            ctx,
        )?;

        //
        // --- Case: ------------------------------------------------------
        //
        fstr::assign(&mut TITLE, b"\"General\" case:  search for transitions from partial transit to no occultation, for ellipsoids of different shape, size and orientation.  Loop iteration = #.");

        spicelib::REPMI(&TITLE.clone(), b"#", I, &mut TITLE, ctx);
        testutil::TCASE(&TITLE, ctx)?;
        //
        // The next step is to test for transitions from partial
        // transit to no occultation.
        //
        // Let BIGSEP be a displacement angle large enough to guarantee
        // that no occultation will be found.
        //
        BIGSEP = spicelib::PI(ctx);

        spicelib::VROTV(
            VPCTR1.as_slice(),
            AXIS.as_slice(),
            (RSIGN * BIGSEP),
            RVEC.as_slice_mut(),
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::VADD(VIEWPT.as_slice(), RVEC.as_slice(), NEWCTR.as_slice_mut());

        CODE = spicelib::ZZOCCED(
            VIEWPT.as_slice(),
            NEWCTR.as_slice(),
            SEMAX1.as_slice(),
            CENTR2.as_slice(),
            SEMAX2.as_slice(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSI(b"CODE", CODE, b"=", NOOCC, 0, OK, ctx)?;

        //
        // We now know that a rotation of the vector from the
        // view point to the center of the first ellipsoid about
        // AXIS by an angle between 0 and XSEP radians should yield
        // a state transition.  Find the angle via binary search.
        //
        LOW = f64::abs(XSEP);
        HIGH = BIGSEP;
        DIFF = (HIGH - LOW);
        NITR = 0;

        while ((DIFF > CNVTOL) && *OK) {
            NITR = (NITR + 1);

            testutil::CHCKSI(b"NITR", NITR, b"<", MAXITR, 0, OK, ctx)?;

            MIDPT = ((HIGH + LOW) / 2.0);

            spicelib::VROTV(
                VPCTR1.as_slice(),
                AXIS.as_slice(),
                (RSIGN * MIDPT),
                RVEC.as_slice_mut(),
            );
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::VADD(VIEWPT.as_slice(), RVEC.as_slice(), NEWCTR.as_slice_mut());

            CODE = spicelib::ZZOCCED(
                VIEWPT.as_slice(),
                NEWCTR.as_slice(),
                SEMAX1.as_slice(),
                CENTR2.as_slice(),
                SEMAX2.as_slice(),
                ctx,
            )?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            if (CODE == NOOCC) {
                HIGH = MIDPT;
            } else {
                //
                // The code had better be PARTL2.
                //
                testutil::CHCKSI(b"(bisection) CODE", CODE, b"=", PARTL2, 0, OK, ctx)?;

                LOW = MIDPT;
            }

            DIFF = (HIGH - LOW);
        }

        //
        // Now that we've dropped out of the loop, verify that T_OCCED
        // says the transit is partial at rotation angle RSIGN*LOW and
        // "none" at rotation angle RSIGN*HIGH.  We adjust each
        // of these angles by CNVTOL to allow for differences in
        // round-off between ZZOCCED and T_OCCED.
        //
        //
        // Check for partial transit:
        //
        spicelib::VROTV(
            VPCTR1.as_slice(),
            AXIS.as_slice(),
            (RSIGN * (LOW - ADJTOL)),
            RVEC.as_slice_mut(),
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::VADD(VIEWPT.as_slice(), RVEC.as_slice(), NEWCTR.as_slice_mut());

        CODE = T_OCCED(
            VIEWPT.as_slice(),
            NEWCTR.as_slice(),
            SEMAX1.as_slice(),
            CENTR2.as_slice(),
            SEMAX2.as_slice(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSI(
            b"CODE from T_OCCED (partial 1)",
            CODE,
            b"=",
            PARTL2,
            0,
            OK,
            ctx,
        )?;

        //
        // Check for no occultation:
        //
        spicelib::VROTV(
            VPCTR1.as_slice(),
            AXIS.as_slice(),
            (RSIGN * (HIGH + ADJTOL)),
            RVEC.as_slice_mut(),
        );
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        spicelib::VADD(VIEWPT.as_slice(), RVEC.as_slice(), NEWCTR.as_slice_mut());

        CODE = T_OCCED(
            VIEWPT.as_slice(),
            NEWCTR.as_slice(),
            SEMAX1.as_slice(),
            CENTR2.as_slice(),
            SEMAX2.as_slice(),
            ctx,
        )?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        testutil::CHCKSI(b"CODE from T_OCCED (none)", CODE, b"=", NOOCC, 0, OK, ctx)?;
    }

    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
