//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const RECSYS: &[u8] = b"RECTANGULAR";
const LATSYS: &[u8] = b"LATITUDINAL";
const SPHSYS: &[u8] = b"SPHERICAL";
const RADSYS: &[u8] = b"RA/DEC";
const CYLSYS: &[u8] = b"CYLINDRICAL";
const GEOSYS: &[u8] = b"GEODETIC";
const PGRSYS: &[u8] = b"PLANETOGRAPHIC";
const XCRD: &[u8] = b"X";
const YCRD: &[u8] = b"Y";
const ZCRD: &[u8] = b"Z";
const RADCRD: &[u8] = b"RADIUS";
const LONCRD: &[u8] = b"LONGITUDE";
const LATCRD: &[u8] = b"LATITUDE";
const RACRD: &[u8] = b"RIGHT ASCENSION";
const DECCRD: &[u8] = b"DECLINATION";
const RNGCRD: &[u8] = b"RANGE";
const CLTCRD: &[u8] = b"COLATITUDE";
const ALTCRD: &[u8] = b"ALTITUDE";
const POSDEF: &[u8] = b"POSITION";
const SOBDEF: &[u8] = b"SUB-OBSERVER POINT";
const SINDEF: &[u8] = b"SURFACE INTERCEPT POINT";
const NWREL: i32 = 5;
const NWLONG: i32 = 7;
const EXWIDX: i32 = ((NWREL + NWLONG) + 1);
const MXBEGM: i32 = 55;
const MXENDM: i32 = 13;
const MXMSG: i32 = ((MXBEGM + MXENDM) + 10);
const UPIDX: i32 = 1;
const ESTIDX: i32 = 2;
const NORIDX: i32 = 3;

//$Procedure ZZGFCPRX ( GF, coordinate derivative proxy )
pub fn ZZGFCPRX(
    STATE: &[f64],
    CORSYS: &[u8],
    RE: f64,
    F: f64,
    SENSE: i32,
    CDSIGN: &mut [i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let STATE = DummyArray::new(STATE, 1..=6);
    let mut CDSIGN = DummyArrayMut::new(CDSIGN, 1..=3);
    let mut ALT: f64 = 0.0;
    let mut DP: f64 = 0.0;
    let mut LAT: f64 = 0.0;
    let mut LON: f64 = 0.0;
    let mut NORMAL = StackArray::<f64, 3>::new(1..=3);
    let mut RTNVEL = StackArray::<f64, 3>::new(1..=3);
    let mut VEL = StackArray::<f64, 3>::new(1..=3);
    let mut XMAT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut DPSIGN: i32 = 0;
    let mut RTNSGN = StackArray::<i32, 3>::new(1..=3);

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Local variables
    //
    // Internally, we're going to use the more
    // descriptive names EAST for the "tangential"
    // direction and NORTH for the "normal" direction.
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZGFCPRX", ctx)?;

    //
    // For planetographic coordinates, check the longitude sense.
    //
    if fstr::eq(CORSYS, PGRSYS) {
        if ((SENSE != 1) && (SENSE != -1)) {
            SETMSG(b"Longitude sense # should be 1 or -1.", ctx);
            ERRINT(b"#", SENSE, ctx);
            SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
            CHKOUT(b"ZZGFCPRX", ctx)?;
            return Ok(());
        }
    }

    //
    // If we have a zero velocity vector, just indicate that each
    // velocity coordinate isn't changing and return now. If the
    // velocity vector is non-zero, convert it to a unit vector; this
    // guarantees that overflow can't occur.

    if VZERO(STATE.subarray(4)) {
        //
        // The velocity is zero. Indicate that the coordinates are
        // not changing and return. Returning now simplifies the
        // logic of the rest of the routine, since the case of
        // zero-velocity can be ignored.
        //
        CLEARI(3, CDSIGN.as_slice_mut());

        CHKOUT(b"ZZGFCPRX", ctx)?;
        return Ok(());
    } else {
        VHAT(STATE.subarray(4), VEL.as_slice_mut());
    }

    //
    // The rectangular case is trivial; handle it now.
    //
    if fstr::eq(CORSYS, RECSYS) {
        //
        // The output system is rectangular. Just indicate the
        // signs of the input velocity.
        //
        for I in 1..=3 {
            if (VEL[I] == 0.0) {
                CDSIGN[I] = 0;
            } else {
                //
                // Use the Fortran sign transfer intrinsic function
                // to set CDSIGN(I) to 1 or -1, depending
                // on whether the corresponding velocity component
                // is positive or negative. See reference [1] for a
                // discussion of this Fortran intrinsic function.
                //
                CDSIGN[I] = intrinsics::IDNINT(f64::copysign(1.0, VEL[I]));
            }
        }
        //
        // All done.
        //
        CHKOUT(b"ZZGFCPRX", ctx)?;
        return Ok(());
    }

    //
    // There's quite a bit of common logic for the "on Z-axis" case;
    // take care of it here.
    //
    if ((STATE[1] == 0.0) && (STATE[2] == 0.0)) {
        //
        // The position lies on the Z-axis.
        //
        // For all of the coordinate systems having a longitude
        // coordinate (this includes right ascension), the derivative of
        // longitude with respect to time is undefined; we set the sign
        // of the derivative to zero.
        //
        // For all of the coordinate systems having a latitude coordinate
        // (this includes declination), if the position is not at the
        // origin, the derivative of latitude with respect to time is
        // undefined unless the input velocity is zero. At the origin,
        // the derivative of latitude with respect to time doesn't exist.
        // In both cases, we set the sign of the velocity components
        // to zero.
        //
        // For the coordinate systems that have a radius or range
        // coordinate, where distance is measured from the origin, when
        // the input position is not at the origin, distance is
        // increasing, constant, or decreasing depending on whether the
        // dot product of velocity and the position's Z-coordinate is
        // positive, zero, or negative, respectively. This dot product
        // test is valid for the derivative of altitude as well (we
        // assert this without proof for the case of positions inside
        // prolate spheroids).
        //
        // If the position is at the origin, then since range and
        // altitude are not differentiable, their signs are set to
        // zero.
        //
        // Cylindrical coordinates are a special case which we treat
        // separately.
        //
        if (STATE[3] != 0.0) {
            //
            // The position is on the Z-axis but not at the origin.
            //
            // Compute the dot product used for the range/altitude
            // derivative.
            //
            DP = VDOT(STATE.as_slice(), VEL.as_slice());

            if (DP == 0.0) {
                DPSIGN = 0;
            } else {
                DPSIGN = intrinsics::IDNINT(f64::copysign(1.0, DP));
            }
        } else {
            //
            // The position is at the origin. We know the velocity
            // is non-zero, and any movement increases radius or
            // altitude. However, neither radius nor altitude are
            // differentiable here, so we indicate no sign.
            //
            DPSIGN = 0;
        }

        //
        // Set the coordinate derivative signs for all but the
        // rectangular system, which was handled already, and
        // the cylindrical system.
        //
        //
        // Recall the coordinate systems and their coordinate orders:
        //
        // System          Coordinates
        // ----------      -----------
        // Rectangular     X, Y, Z
        // Latitudinal     Radius, Longitude, Latitude
        // Spherical       Radius, Colatitude, Longitude
        // RA/Dec          Range, Right Ascension, Declination
        // Cylindrical     Radius, Longitude, Z
        // Geodetic        Longitude, Latitude, Altitude
        // Planetographic  Longitude, Latitude, Altitude
        //
        //
        if fstr::eq(CORSYS, LATSYS) {
            //
            // The radial derivative sign was computed; the
            // other derivative signs are set to zero.
            //
            CDSIGN[1] = DPSIGN;
            CDSIGN[2] = 0;
            CDSIGN[3] = 0;
        } else if fstr::eq(CORSYS, SPHSYS) {
            //
            // The radial derivative sign was computed; the
            // longitude derivative signs is set to zero.
            //
            CDSIGN[1] = DPSIGN;
            CDSIGN[3] = 0;

            //
            // Co-latitude is a special case. Co-latitude is
            // not differentiable with respect to Cartesian
            // position for positions on the Z-axis, since
            // co-latitude is a v-shaped function of distance
            // from the Z-axis. We simply set the sign
            // of the co-latitude derivative to zero in this
            // case.
            //
            CDSIGN[2] = 0;
        } else if fstr::eq(CORSYS, RADSYS) {
            //
            // RA/Dec derivatives are assigned in the same manner
            // as latitudinal ones.
            //
            CDSIGN[1] = DPSIGN;
            CDSIGN[2] = 0;
            CDSIGN[3] = 0;
        } else if fstr::eq(CORSYS, GEOSYS) {
            //
            // Altitude plays the role of radius for this
            // system.
            //
            CDSIGN[1] = 0;
            CDSIGN[2] = 0;
            CDSIGN[3] = DPSIGN;
        } else if fstr::eq(CORSYS, PGRSYS) {
            //
            // Altitude plays the role of radius for this
            // system.
            //
            CDSIGN[1] = 0;
            CDSIGN[2] = 0;
            CDSIGN[3] = DPSIGN;
        } else if fstr::eq(CORSYS, CYLSYS) {
            CDSIGN[1] = 0;
            CDSIGN[2] = 0;
            //
            // For cylindrical coordinates, the derivative of Z with
            // respect to time is already present in VEL.
            //
            if (VEL[3] == 0.0) {
                CDSIGN[3] = 0;
            } else {
                CDSIGN[3] = intrinsics::IDNINT(f64::copysign(1.0, VEL[3]));
            }
        } else {
            //
            // If we end up here, we have an invalid coordinate system.
            //
            SETMSG(b"Coordinate system # is not supported. Verify that the coordinate system specifier matches a value from zzgf.inc.", ctx);
            ERRCH(b"#", CORSYS, ctx);
            SIGERR(b"SPICE(NOTSUPPORTED)", ctx)?;
            CHKOUT(b"ZZGFCPRX", ctx)?;
            return Ok(());
        }

        //
        // We've handled the on-Z-axis cases. Return now.
        //
        CHKOUT(b"ZZGFCPRX", ctx)?;
        return Ok(());
    }

    //
    // This is the normal case: the position is not on the Z-axis.
    //
    // The type of MRTN frame we use depends on the coordinate system.
    // Planetodetic and planetographic coordinate systems are a special
    // case.
    //
    if (fstr::eq(CORSYS, GEOSYS) || fstr::eq(CORSYS, PGRSYS)) {
        //
        // Instead of defining the MRTN frame using the input
        // position vector, we define it using an outward normal vector
        // on the reference ellipsoid at the geodetic latitude
        // and longitude of the input position.
        //
        RECGEO(STATE.as_slice(), RE, F, &mut LON, &mut LAT, &mut ALT, ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"ZZGFCPRX", ctx)?;
            return Ok(());
        }

        LATREC(1.0, LON, LAT, NORMAL.as_slice_mut());
    } else if fstr::eq(CORSYS, CYLSYS) {
        //
        // The normal vector is aligned with the local radial
        // direction; this vector is parallel to the X-Y plane.
        //
        VPACK(STATE[1], STATE[2], 0.0, NORMAL.as_slice_mut());
        VHATIP(NORMAL.as_slice_mut());
    } else {
        //
        // The position vector provides the normal direction.
        //
        VHAT(STATE.as_slice(), NORMAL.as_slice_mut());
    }

    // Obtain the matrix required to transform the velocity to the MRTN
    // frame; transform the velocity.
    //
    ZZRTNMAT(NORMAL.as_slice(), XMAT.as_slice_mut(), ctx)?;
    MXV(XMAT.as_slice(), VEL.as_slice(), RTNVEL.as_slice_mut());

    //
    // We can think of the basis vectors of the MRTN frame as local "up",
    // "East," "North" directions. Compute the signs of the up, East,
    // and North velocity components.
    //
    for I in 1..=3 {
        if (RTNVEL[I] == 0.0) {
            RTNSGN[I] = 0;
        } else {
            RTNSGN[I] = intrinsics::IDNINT(f64::copysign(1.0, RTNVEL[I]));
        }
    }

    //
    // Set the signs of the coordinate derivatives from the MRTN
    // derivative signs.
    //
    //
    //    Recall the coordinate systems and their coordinate orders:
    //
    //    System          Coordinates
    //    ----------      -----------
    //    Rectangular     X, Y, Z
    //    Latitudinal     Radius, Longitude, Latitude
    //    Spherical       Radius, Colatitude, Longitude
    //    RA/Dec          Range, Right Ascension, Declination
    //    Cylindrical     Radius, Longitude, Z
    //    Geodetic        Longitude, Latitude, Altitude
    //    Planetographic  Longitude, Latitude, Altitude
    //
    //
    if fstr::eq(CORSYS, LATSYS) {
        CDSIGN[1] = RTNSGN[UPIDX];
        CDSIGN[2] = RTNSGN[ESTIDX];
        CDSIGN[3] = RTNSGN[NORIDX];
    } else if fstr::eq(CORSYS, SPHSYS) {
        //
        // For spherical coordinate systems, the sign of the
        // derivative of co-latitude is the negative of the
        // sign of the North derivative.
        //
        CDSIGN[1] = RTNSGN[UPIDX];
        CDSIGN[2] = -RTNSGN[NORIDX];
        CDSIGN[3] = RTNSGN[ESTIDX];
    } else if fstr::eq(CORSYS, RADSYS) {
        CDSIGN[1] = RTNSGN[UPIDX];
        CDSIGN[2] = RTNSGN[ESTIDX];
        CDSIGN[3] = RTNSGN[NORIDX];
    } else if fstr::eq(CORSYS, GEOSYS) {
        CDSIGN[1] = RTNSGN[ESTIDX];
        CDSIGN[2] = RTNSGN[NORIDX];
        CDSIGN[3] = RTNSGN[UPIDX];
    } else if fstr::eq(CORSYS, PGRSYS) {
        //
        // For planetographic coordinates, altitude and latitude
        // behave identically to their geodetic counterparts. We
        // need to adjust the sign of the longitude derivative
        // according to whether longitude is positive East or West.
        //
        CDSIGN[1] = (RTNSGN[ESTIDX] * SENSE);
        CDSIGN[2] = RTNSGN[NORIDX];
        CDSIGN[3] = RTNSGN[UPIDX];
    } else if fstr::eq(CORSYS, CYLSYS) {
        CDSIGN[1] = RTNSGN[UPIDX];
        CDSIGN[2] = RTNSGN[ESTIDX];
        CDSIGN[3] = RTNSGN[NORIDX];
    } else {
        //
        // If we end up here, we have an invalid coordinate system.
        //
        SETMSG(b"Coordinate system # is not supported. Verify that the coordinate system specifier matches a value from zzgf.inc.", ctx);
        ERRCH(b"#", CORSYS, ctx);
        SIGERR(b"SPICE(NOTSUPPORTED)", ctx)?;
        CHKOUT(b"ZZGFCPRX", ctx)?;
        return Ok(());
    }

    CHKOUT(b"ZZGFCPRX", ctx)?;
    Ok(())
}
