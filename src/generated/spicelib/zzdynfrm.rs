//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const INERTL: i32 = 1;
const PCK: i32 = (INERTL + 1);
const CK: i32 = (PCK + 1);
const TK: i32 = (CK + 1);
const DYN: i32 = (TK + 1);
const SWTCH: i32 = (DYN + 1);
const ALL: i32 = -1;
const NABCOR: i32 = 15;
const ABATSZ: i32 = 6;
const GEOIDX: i32 = 1;
const LTIDX: i32 = (GEOIDX + 1);
const STLIDX: i32 = (LTIDX + 1);
const CNVIDX: i32 = (STLIDX + 1);
const XMTIDX: i32 = (CNVIDX + 1);
const RELIDX: i32 = (XMTIDX + 1);
const CORLEN: i32 = 5;
const KVNMLN: i32 = 32;
const KVLEN: i32 = 80;
const FRNMLN: i32 = 32;
const BDNMLN: i32 = 36;
const MAXCOF: i32 = 20;
const MXNFAC: i32 = 10;
const LBSEP: f64 = 0.001;
const QEXP: i32 = -27;
const KWBFRM: &[u8] = b"RELATIVE";
const KWSTYL: &[u8] = b"DEF_STYLE";
const KVPARM: &[u8] = b"PARAMETERIZED";
const KWFREZ: &[u8] = b"FREEZE_EPOCH";
const KWRSTA: &[u8] = b"ROTATION_STATE";
const KVROTG: &[u8] = b"ROTATING";
const KVINRT: &[u8] = b"INERTIAL";
const KWFFAM: &[u8] = b"FAMILY";
const KVMEQT: &[u8] = b"MEAN_EQUATOR_AND_EQUINOX_OF_DATE";
const KVMECL: &[u8] = b"MEAN_ECLIPTIC_AND_EQUINOX_OF_DATE";
const KVTEQT: &[u8] = b"TRUE_EQUATOR_AND_EQUINOX_OF_DATE";
const KV2VEC: &[u8] = b"TWO-VECTOR";
const KVEULR: &[u8] = b"EULER";
const KVPROD: &[u8] = b"PRODUCT";
const KWPRCM: &[u8] = b"PREC_MODEL";
const KWNUTM: &[u8] = b"NUT_MODEL";
const KWOBQM: &[u8] = b"OBLIQ_MODEL";
const KVM001: &[u8] = b"EARTH_IAU_1976";
const KVM002: &[u8] = b"EARTH_IAU_1980";
const KVM003: &[u8] = b"EARTH_IAU_1980";
const KWVAXI: &[u8] = b"AXIS";
const KVX: &[u8] = b"X";
const KVY: &[u8] = b"Y";
const KVZ: &[u8] = b"Z";
const KWPRI: &[u8] = b"PRI_";
const KWSEC: &[u8] = b"SEC_";
const KWVCDF: &[u8] = b"VECTOR_DEF";
const KVPOSV: &[u8] = b"OBSERVER_TARGET_POSITION";
const KVVELV: &[u8] = b"OBSERVER_TARGET_VELOCITY";
const KVNEAR: &[u8] = b"TARGET_NEAR_POINT";
const KVCONS: &[u8] = b"CONSTANT";
const KWVOBS: &[u8] = b"OBSERVER";
const KWVTRG: &[u8] = b"TARGET";
const KWVFRM: &[u8] = b"FRAME";
const KWVABC: &[u8] = b"ABCORR";
const KWVSPC: &[u8] = b"SPEC";
const KVRECC: &[u8] = b"RECTANGULAR";
const KVLATC: &[u8] = b"LATITUDINAL";
const KVRADC: &[u8] = b"RA/DEC";
const KWVECT: &[u8] = b"VECTOR";
const KWLAT: &[u8] = b"LATITUDE";
const KWLON: &[u8] = b"LONGITUDE";
const KWRA: &[u8] = b"RA";
const KWDEC: &[u8] = b"DEC";
const KWATOL: &[u8] = b"ANGLE_SEP_TOL";
const KWEPOC: &[u8] = b"EPOCH";
const KWEUAX: &[u8] = b"AXES";
const KWEAC1: &[u8] = b"ANGLE_1_COEFFS";
const KWEAC2: &[u8] = b"ANGLE_2_COEFFS";
const KWEAC3: &[u8] = b"ANGLE_3_COEFFS";
const KWFFRM: &[u8] = b"FROM_FRAMES";
const KWTFRM: &[u8] = b"TO_FRAMES";
const KWUNIT: &[u8] = b"UNITS";
const KVRADN: &[u8] = b"RADIANS";
const KVDEGR: &[u8] = b"DEGREES";
const RNAME: &[u8] = b"ZZDYNFRM";
const TIMLEN: i32 = 50;
const VNAMLN: i32 = 4;

struct SaveVars {
    AXES: ActualCharArray,
    ITMABC: ActualCharArray,
    ITMAXE: ActualCharArray,
    ITMCOF: ActualCharArray,
    ITMDEC: ActualCharArray,
    ITMFRM: ActualCharArray,
    ITMLAT: ActualCharArray,
    ITMLON: ActualCharArray,
    ITMOBS: ActualCharArray,
    ITMRA: ActualCharArray,
    ITMSEP: Vec<u8>,
    ITMSPC: ActualCharArray,
    ITMTRG: ActualCharArray,
    ITMUNT: ActualCharArray,
    ITMVDF: ActualCharArray,
    ITMVEC: ActualCharArray,
    VNAME: ActualCharArray,
    EARTH: i32,
    J2000: i32,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut AXES = ActualCharArray::new(1, 1..=3);
        let mut ITMABC = ActualCharArray::new(KVNMLN, 1..=2);
        let mut ITMAXE = ActualCharArray::new(KVNMLN, 1..=2);
        let mut ITMCOF = ActualCharArray::new(KVNMLN, 1..=3);
        let mut ITMDEC = ActualCharArray::new(KVNMLN, 1..=2);
        let mut ITMFRM = ActualCharArray::new(KVNMLN, 1..=2);
        let mut ITMLAT = ActualCharArray::new(KVNMLN, 1..=2);
        let mut ITMLON = ActualCharArray::new(KVNMLN, 1..=2);
        let mut ITMOBS = ActualCharArray::new(KVNMLN, 1..=2);
        let mut ITMRA = ActualCharArray::new(KVNMLN, 1..=2);
        let mut ITMSEP = vec![b' '; KVNMLN as usize];
        let mut ITMSPC = ActualCharArray::new(KVNMLN, 1..=2);
        let mut ITMTRG = ActualCharArray::new(KVNMLN, 1..=2);
        let mut ITMUNT = ActualCharArray::new(KVNMLN, 1..=2);
        let mut ITMVDF = ActualCharArray::new(KVNMLN, 1..=2);
        let mut ITMVEC = ActualCharArray::new(KVNMLN, 1..=2);
        let mut VNAME = ActualCharArray::new(VNAMLN, 1..=2);
        let mut EARTH: i32 = 0;
        let mut J2000: i32 = 0;
        let mut FIRST: bool = false;

        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(KVX), Val::C(KVY), Val::C(KVZ)].into_iter();
            AXES.iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        FIRST = true;
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(KWEAC1), Val::C(KWEAC2), Val::C(KWEAC3)].into_iter();
            ITMCOF
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        fstr::assign(&mut ITMSEP, KWATOL);
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(KWPRI), Val::C(KWSEC)].into_iter();
            VNAME
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            AXES,
            ITMABC,
            ITMAXE,
            ITMCOF,
            ITMDEC,
            ITMFRM,
            ITMLAT,
            ITMLON,
            ITMOBS,
            ITMRA,
            ITMSEP,
            ITMSPC,
            ITMTRG,
            ITMUNT,
            ITMVDF,
            ITMVEC,
            VNAME,
            EARTH,
            J2000,
            FIRST,
        }
    }
}

//$Procedure ZZDYNFRM ( Dynamic state transformation evaluation )
pub fn ZZDYNFRM(
    INFRAM: i32,
    CENTER: i32,
    ET: f64,
    XFORM: &mut [f64],
    BASFRM: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut XFORM = DummyArrayMut2D::new(XFORM, 1..=6, 1..=6);
    let mut ABCORR = [b' '; CORLEN as usize];
    let mut AXNAME = [b' '; KVLEN as usize];
    let mut BASNAM = [b' '; FRNMLN as usize];
    let mut CFRMNM = [b' '; FRNMLN as usize];
    let mut CTRNAM = [b' '; BDNMLN as usize];
    let mut CVCORR = [b' '; CORLEN as usize];
    let mut DYNSTL = [b' '; KVLEN as usize];
    let mut DYNFAM = [b' '; KVLEN as usize];
    let mut FFRAMS = ActualCharArray::new(FRNMLN, 1..=MXNFAC);
    let mut INNAME = [b' '; FRNMLN as usize];
    let mut NUTMOD = [b' '; KVLEN as usize];
    let mut OBLMOD = [b' '; KVLEN as usize];
    let mut PRCMOD = [b' '; KVLEN as usize];
    let mut ROTSTA = [b' '; KVLEN as usize];
    let mut SPEC = [b' '; KVLEN as usize];
    let mut TFRAMS = ActualCharArray::new(FRNMLN, 1..=MXNFAC);
    let mut TIMSTR = [b' '; TIMLEN as usize];
    let mut TMPFAM = [b' '; KVLEN as usize];
    let mut UNITS = [b' '; KVLEN as usize];
    let mut VECDEF = ActualCharArray::new(KVLEN, 1..=2);
    let mut VELFRM = [b' '; FRNMLN as usize];
    let mut ACC = StackArray::<f64, 3>::new(1..=3);
    let mut ANGLES = StackArray::<f64, 2>::new(1..=2);
    let mut COEFFS = StackArray2D::<f64, 60>::new(1..=MAXCOF, 1..=3);
    let mut CTRPOS = StackArray::<f64, 3>::new(1..=3);
    let mut DEC: f64 = 0.0;
    let mut DELTA: f64 = 0.0;
    let mut DIRVEC = StackArray::<f64, 3>::new(1..=3);
    let mut DMOB: f64 = 0.0;
    let mut EPOCH: f64 = 0.0;
    let mut EULANG = StackArray::<f64, 6>::new(1..=6);
    let mut FET: f64 = 0.0;
    let mut LAT: f64 = 0.0;
    let mut LON: f64 = 0.0;
    let mut LT: f64 = 0.0;
    let mut MINSEP: f64 = 0.0;
    let mut MOB: f64 = 0.0;
    let mut NUTXF = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut OBLXF = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut POLY = StackArray::<f64, 2>::new(0..=1);
    let mut PRECXF = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut RA: f64 = 0.0;
    let mut RADII = StackArray::<f64, 3>::new(1..=3);
    let mut S2 = StackArray2D::<f64, 12>::new(1..=6, 1..=2);
    let mut SEP: f64 = 0.0;
    let mut STALT = StackArray::<f64, 2>::new(1..=2);
    let mut STEMP = StackArray::<f64, 6>::new(1..=6);
    let mut STNEAR = StackArray::<f64, 6>::new(1..=6);
    let mut STOBS = StackArray::<f64, 6>::new(1..=6);
    let mut TARRAY = StackArray::<f64, 3>::new(1..=3);
    let mut T0: f64 = 0.0;
    let mut VARRAY = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut VET: f64 = 0.0;
    let mut VFLT: f64 = 0.0;
    let mut XF2000 = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut XFINV = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut XFTEMP = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut XIPM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut XOUT = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut AXIS = StackArray::<i32, 2>::new(1..=2);
    let mut CFRMID: i32 = 0;
    let mut CVOBS: i32 = 0;
    let mut DEGS = StackArray::<i32, 3>::new(1..=3);
    let mut FRCID: i32 = 0;
    let mut FRCLS: i32 = 0;
    let mut FRCTR: i32 = 0;
    let mut FRID: i32 = 0;
    let mut FROMID: i32 = 0;
    let mut IAXES = StackArray::<i32, 3>::new(1..=3);
    let mut J: i32 = 0;
    let mut M: i32 = 0;
    let mut N: i32 = 0;
    let mut OBS: i32 = 0;
    let mut TARG: i32 = 0;
    let mut TOID: i32 = 0;
    let mut CORBLK = StackArray::<bool, 15>::new(1..=NABCOR);
    let mut FND: bool = false;
    let mut FROZEN: bool = false;
    let mut INERT: bool = false;
    let mut MEANEC: bool = false;
    let mut MEANEQ: bool = false;
    let mut NEGATE: bool = false;
    let mut OFDATE: bool = false;
    let mut TRUEEQ: bool = false;

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

    //
    // Initial values
    //

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(RNAME, ctx)?;

    if save.FIRST {
        //
        // Get the ID code for the J2000 frame.
        //
        IRFNUM(b"J2000", &mut save.J2000, ctx);

        //
        // Get the ID code for the earth (we needn't check the found
        // flag).
        //
        BODN2C(b"EARTH", &mut save.EARTH, &mut FND, ctx)?;

        //
        // Initialize "item" strings used to create kernel variable
        // names.
        //
        for I in 1..=2 {
            //
            // Vector axis:
            //
            fstr::assign(
                save.ITMAXE.get_mut(I),
                &fstr::concat(save.VNAME.get(I), KWVAXI),
            );
            //
            // Vector definition:
            //
            fstr::assign(
                save.ITMVDF.get_mut(I),
                &fstr::concat(save.VNAME.get(I), KWVCDF),
            );
            //
            // Vector aberration correction:
            //
            fstr::assign(
                save.ITMABC.get_mut(I),
                &fstr::concat(save.VNAME.get(I), KWVABC),
            );
            //
            // Vector frame:
            //
            fstr::assign(
                save.ITMFRM.get_mut(I),
                &fstr::concat(save.VNAME.get(I), KWVFRM),
            );
            //
            // Vector observer:
            //
            fstr::assign(
                save.ITMOBS.get_mut(I),
                &fstr::concat(save.VNAME.get(I), KWVOBS),
            );
            //
            // Vector target:
            //
            fstr::assign(
                save.ITMTRG.get_mut(I),
                &fstr::concat(save.VNAME.get(I), KWVTRG),
            );
            //
            // Vector longitude:
            //
            fstr::assign(
                save.ITMLON.get_mut(I),
                &fstr::concat(save.VNAME.get(I), KWLON),
            );
            //
            // Vector latitude:
            //
            fstr::assign(
                save.ITMLAT.get_mut(I),
                &fstr::concat(save.VNAME.get(I), KWLAT),
            );
            //
            // Vector right ascension:
            //
            fstr::assign(
                save.ITMRA.get_mut(I),
                &fstr::concat(save.VNAME.get(I), KWRA),
            );
            //
            // Vector declination:
            //
            fstr::assign(
                save.ITMDEC.get_mut(I),
                &fstr::concat(save.VNAME.get(I), KWDEC),
            );
            //
            // Vector units:
            //
            fstr::assign(
                save.ITMUNT.get_mut(I),
                &fstr::concat(save.VNAME.get(I), KWUNIT),
            );
            //
            // Constant vector coordinate specification:
            //
            fstr::assign(
                save.ITMSPC.get_mut(I),
                &fstr::concat(save.VNAME.get(I), KWVSPC),
            );
            //
            // Constant vector in Cartesian coordinates, literal value:
            //
            fstr::assign(
                save.ITMVEC.get_mut(I),
                &fstr::concat(save.VNAME.get(I), KWVECT),
            );
        }

        save.FIRST = false;
    }

    //
    // Initialize the output arguments.
    //
    CLEARD(36, XFORM.as_slice_mut());

    *BASFRM = 0;

    //
    // Initialize certain variables to ensure that we don't do
    // arithmetic operations using bogus, possibly large,
    // undefined values.
    //
    CLEARD(36, NUTXF.as_slice_mut());
    CLEARD(36, OBLXF.as_slice_mut());
    CLEARD(36, PRECXF.as_slice_mut());
    CLEARD(36, XF2000.as_slice_mut());
    CLEARD(36, XFINV.as_slice_mut());
    CLEARD(36, XIPM.as_slice_mut());

    MOB = 0.0;
    DMOB = 0.0;
    T0 = 0.0;
    FROZEN = false;

    //
    // Get the input frame name.
    //
    FRMNAM(INFRAM, &mut INNAME, ctx)?;

    //
    // We need the name of the base frame.
    //
    ZZDYNFID(&INNAME, INFRAM, KWBFRM, BASFRM, ctx)?;

    if FAILED(ctx) {
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    FRMNAM(*BASFRM, &mut BASNAM, ctx)?;

    //
    // The output frame code and name are set.
    //
    // Look up the dynamic frame definition style from the kernel pool.
    // The kernel variable's name might be specified by name or ID.
    //
    ZZDYNVAC(
        &INNAME,
        INFRAM,
        KWSTYL,
        1,
        &mut N,
        CharArrayMut::from_mut(&mut DYNSTL),
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // At this time, the only supported dynamic frame definition style is
    // PARAMETERIZED.
    //
    if EQSTR(&DYNSTL, KVPARM) {
        //
        // Parameterized dynamic frames belong to families.  Look up
        // the family for this frame.
        //
        ZZDYNVAC(
            &INNAME,
            INFRAM,
            KWFFAM,
            1,
            &mut N,
            CharArrayMut::from_mut(&mut DYNFAM),
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        CMPRSS(b" ", 0, &DYNFAM, &mut TMPFAM);
        UCASE(&TMPFAM, &mut DYNFAM, ctx);

        //
        // Determine whether we have an "of-date" frame family.
        // The logical flags used here and respective meanings are:
        //
        //    MEANEQ   Mean equator and equinox of date
        //    TRUEEQ   True equator and equinox of date
        //    MEANEC   Mean ecliptic and equinox of date
        //
        MEANEQ = fstr::eq(&DYNFAM, KVMEQT);
        TRUEEQ = fstr::eq(&DYNFAM, KVTEQT);
        MEANEC = fstr::eq(&DYNFAM, KVMECL);

        OFDATE = ((MEANEQ || MEANEC) || TRUEEQ);

        //
        // Set the evaluation epoch T0.  Normally this epoch is ET,
        // but if the frame is frozen, the freeze epoch from the
        // frame definition is used.
        //
        // Read the freeze epoch into T0 if a freeze epoch was
        // specified; let FROZEN receive the FOUND flag value
        // returned by ZZDYNOAD.
        //
        ZZDYNOAD(
            &INNAME,
            INFRAM,
            KWFREZ,
            1,
            &mut N,
            std::slice::from_mut(&mut T0),
            &mut FROZEN,
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        if !FROZEN {
            //
            // Normal case:  just use the input epoch.
            //
            T0 = ET;
        }

        //
        // Look up the rotation state keyword.  Rather than checking
        // FAILED() after every call, we'll do it after we're
        // done with processing the rotation state.
        //
        ZZDYNOAC(
            &INNAME,
            INFRAM,
            KWRSTA,
            1,
            &mut N,
            CharArrayMut::from_mut(&mut ROTSTA),
            &mut FND,
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        //
        // If the frame is frozen, the rotation state keyword *must be
        // absent*.
        //
        if (FROZEN && FND) {
            SETMSG(b"Definition of frame # contains both # and # keywords; at most one of these must be present in the frame definition. This situation is usually caused by an error in a frame kernel in which the frame is defined.", ctx);
            ERRCH(b"#", &INNAME, ctx);
            ERRCH(b"#", KWFREZ, ctx);
            ERRCH(b"#", KWRSTA, ctx);
            SIGERR(b"SPICE(FRAMEDEFERROR)", ctx)?;
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        //
        // If the frame belongs to an "of date" family, either the
        // rotation state must be specified or the frame must be
        // frozen.
        //
        if ((OFDATE && !FROZEN) && !FND) {
            SETMSG(b"Definition of frame #, which belongs to parameterized dynamic frame family #, contains neither # nor # keywords; frames in this family require exactly one of these in their frame definitions. This situation is usually caused by an error in a frame kernel in which the frame is defined.", ctx);
            ERRCH(b"#", &INNAME, ctx);
            ERRCH(b"#", &DYNFAM, ctx);
            ERRCH(b"#", KWFREZ, ctx);
            ERRCH(b"#", KWRSTA, ctx);
            SIGERR(b"SPICE(FRAMEDEFERROR)", ctx)?;
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }

        //
        // Set the rotation state logical flag indicating whether
        // the state is 'INERTIAL'.
        //
        if FND {
            //
            // A rotation state keyword was found.
            //
            // We know the state is not frozen if we arrive here.
            //
            INERT = EQSTR(&ROTSTA, KVINRT);

            if !INERT {
                //
                // Catch invalid rotation states here.
                //
                if !EQSTR(&ROTSTA, KVROTG) {
                    SETMSG(b"Definition of frame # contains # specification #. The only valid rotation states are # or #. This situation is usually caused by an error in a frame kernel in which the frame is defined.", ctx);
                    ERRCH(b"#", &INNAME, ctx);
                    ERRCH(b"#", KWRSTA, ctx);
                    ERRCH(b"#", &ROTSTA, ctx);
                    ERRCH(b"#", KVROTG, ctx);
                    ERRCH(b"#", KVINRT, ctx);
                    SIGERR(b"SPICE(NOTSUPPORTED)", ctx)?;
                    CHKOUT(RNAME, ctx)?;
                    return Ok(());
                }
            }
        } else {
            //
            // The state is not inertial unless there's a ROTATION_STATE
            // keyword assignment telling us it is.
            //
            INERT = false;
        }

        //
        // INERT and FROZEN are both set. The evaluation epoch T0 is also
        // set.
        //
        // The following code block performs actions specific to
        // the various dynamic frame families.
        //
        if OFDATE {
            //
            // Fetch the name of the true equator and equinox of date
            // precession model.
            //
            ZZDYNVAC(
                &INNAME,
                INFRAM,
                KWPRCM,
                1,
                &mut N,
                CharArrayMut::from_mut(&mut PRCMOD),
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(RNAME, ctx)?;
                return Ok(());
            }

            //
            // Get the precession transformation.
            //
            if EQSTR(&PRCMOD, KVM001) {
                //
                // This is the 1976 IAU earth precession model.
                //
                // Make sure the center of the input frame is the earth.
                //
                if (CENTER != save.EARTH) {
                    BODC2N(CENTER, &mut CTRNAM, &mut FND, ctx)?;

                    if !FND {
                        INTSTR(CENTER, &mut CTRNAM, ctx);
                    }

                    SETMSG(b"Definition of frame # specifies frame center # and precession model #. This precession model is not applicable to body #. This situation is usually caused by an error in a frame kernel in which the frame is defined.", ctx);
                    ERRCH(b"#", &INNAME, ctx);
                    ERRCH(b"#", &CTRNAM, ctx);
                    ERRCH(b"#", KVM001, ctx);
                    ERRCH(b"#", &CTRNAM, ctx);
                    SIGERR(b"SPICE(INVALIDSELECTION)", ctx)?;
                    CHKOUT(RNAME, ctx)?;
                    return Ok(());
                }
                //
                // Look up the precession transformation.
                //
                ZZEPRC76(T0, PRECXF.as_slice_mut(), ctx)?;

                //
                // If we're in the mean-of-date case, invert this
                // transformation to obtain the mapping from the
                // mean-of-date frame to J2000.
                //
                if MEANEQ {
                    INVSTM(PRECXF.as_slice(), XFTEMP.as_slice_mut(), ctx)?;
                }
            } else {
                SETMSG(b"Definition of frame # specifies precession model #, which is not recognized. This situation is usually caused by an error in a frame kernel in which the frame is defined.", ctx);
                ERRCH(b"#", &INNAME, ctx);
                ERRCH(b"#", &PRCMOD, ctx);
                SIGERR(b"SPICE(NOTSUPPORTED)", ctx)?;
                CHKOUT(RNAME, ctx)?;
                return Ok(());
            }

            //
            // At this point the precession transformation PRECXF is set.
            // If INFRAM is a mean equator and equinox of date frame, the
            // inverse of PRECXF is currently stored in XFTEMP.

            if TRUEEQ {
                //
                // We need a nutation transformation as well. Get the name
                // of the nutation model.
                //
                ZZDYNVAC(
                    &INNAME,
                    INFRAM,
                    KWNUTM,
                    1,
                    &mut N,
                    CharArrayMut::from_mut(&mut NUTMOD),
                    ctx,
                )?;

                if FAILED(ctx) {
                    CHKOUT(RNAME, ctx)?;
                    return Ok(());
                }

                //
                // Get the nutation transformation.
                //
                if EQSTR(&NUTMOD, KVM002) {
                    //
                    // This is the 1980 IAU earth nutation model.
                    //
                    // Make sure the center is the earth.
                    //
                    if (CENTER != save.EARTH) {
                        BODC2N(CENTER, &mut CTRNAM, &mut FND, ctx)?;

                        if !FND {
                            INTSTR(CENTER, &mut CTRNAM, ctx);
                        }

                        SETMSG(b"Definition of frame # specifies frame center # and nutation model #. This nutation model is not applicable to body #.  This situation is usually caused by an error in a frame kernel in which the frame is defined.", ctx);
                        ERRCH(b"#", &INNAME, ctx);
                        ERRCH(b"#", &CTRNAM, ctx);
                        ERRCH(b"#", KVM002, ctx);
                        ERRCH(b"#", &CTRNAM, ctx);
                        SIGERR(b"SPICE(INVALIDSELECTION)", ctx)?;
                        CHKOUT(RNAME, ctx)?;
                        return Ok(());
                    }

                    //
                    // Look up the nutation transformation.
                    //
                    ZZENUT80(T0, NUTXF.as_slice_mut(), ctx)?;

                    //
                    // Find the transformation from the J2000 frame to the
                    // earth true of date frame.  Invert.
                    //
                    MXMG(
                        NUTXF.as_slice(),
                        PRECXF.as_slice(),
                        6,
                        6,
                        6,
                        XFINV.as_slice_mut(),
                    );
                    INVSTM(XFINV.as_slice(), XFTEMP.as_slice_mut(), ctx)?;
                } else {
                    SETMSG(b"Definition of frame # specifies nutation model #, which is not recognized. This situation is usually caused by an error in a frame kernel in which the frame is defined.", ctx);
                    ERRCH(b"#", &INNAME, ctx);
                    ERRCH(b"#", &NUTMOD, ctx);
                    SIGERR(b"SPICE(NOTSUPPORTED)", ctx)?;
                    CHKOUT(RNAME, ctx)?;
                    return Ok(());
                }
            } else if MEANEC {
                //
                // We need a mean obliquity transformation as well.
                // Get the name of the obliquity model.
                //
                ZZDYNVAC(
                    &INNAME,
                    INFRAM,
                    KWOBQM,
                    1,
                    &mut N,
                    CharArrayMut::from_mut(&mut OBLMOD),
                    ctx,
                )?;

                if FAILED(ctx) {
                    CHKOUT(RNAME, ctx)?;
                    return Ok(());
                }

                //
                // Get the obliquity transformation.
                //
                if EQSTR(&OBLMOD, KVM003) {
                    //
                    // This is the 1980 IAU earth mean obliquity of
                    // date model.
                    //
                    // Make sure the center is the earth.
                    //
                    if (CENTER != save.EARTH) {
                        BODC2N(CENTER, &mut CTRNAM, &mut FND, ctx)?;

                        if !FND {
                            INTSTR(CENTER, &mut CTRNAM, ctx);
                        }

                        SETMSG(b"Definition of frame # specifies frame center # and obliquity model #.  This obliquity model is not applicable to body #. This situation is usually caused by an error in a frame kernel in which the frame is defined.", ctx);
                        ERRCH(b"#", &INNAME, ctx);
                        ERRCH(b"#", &CTRNAM, ctx);
                        ERRCH(b"#", KVM003, ctx);
                        ERRCH(b"#", &CTRNAM, ctx);
                        SIGERR(b"SPICE(INVALIDSELECTION)", ctx)?;
                        CHKOUT(RNAME, ctx)?;
                        return Ok(());
                    }

                    //
                    // Create the obliquity transformation. First look up
                    // the obliquity state (angle and angular rate).
                    //
                    ZZMOBLIQ(T0, &mut MOB, &mut DMOB, ctx);

                    //
                    // The obliquity rotation is about the mean-of-date
                    // x-axis.  The other Euler angles are identically
                    // zero; the axes are arbitrary, as long as the
                    // middle axis is distinct from the other two.
                    //
                    CLEARD(6, EULANG.as_slice_mut());
                    EULANG[3] = MOB;
                    EULANG[6] = DMOB;

                    EUL2XF(EULANG.as_slice(), 1, 3, 1, OBLXF.as_slice_mut(), ctx)?;

                    //
                    // Find the transformation from the J2000 to the
                    // earth mean ecliptic of date frame.  Invert.
                    //
                    MXMG(
                        OBLXF.as_slice(),
                        PRECXF.as_slice(),
                        6,
                        6,
                        6,
                        XFINV.as_slice_mut(),
                    );
                    INVSTM(XFINV.as_slice(), XFTEMP.as_slice_mut(), ctx)?;
                } else {
                    SETMSG(b"Definition of frame # specifies obliquity model #, which is not recognized. This situation is usually caused by an error in a frame kernel in which the frame is defined.", ctx);
                    ERRCH(b"#", &INNAME, ctx);
                    ERRCH(b"#", &OBLMOD, ctx);
                    SIGERR(b"SPICE(NOTSUPPORTED)", ctx)?;
                    CHKOUT(RNAME, ctx)?;
                    return Ok(());
                }
            }

            //
            // At this point, XFTEMP contains the mapping from the
            // specified mean of date or true of date frame to J2000.
            //
            // If the base frame is not J2000, we must find the
            // transformation from J2000 to the base frame.
            //
            if (*BASFRM != save.J2000) {
                ZZFRMCH0(save.J2000, *BASFRM, T0, XF2000.as_slice_mut(), ctx)?;
                MXMG(
                    XF2000.as_slice(),
                    XFTEMP.as_slice(),
                    6,
                    6,
                    6,
                    XFORM.as_slice_mut(),
                );
            } else {
                //
                // Otherwise, XFTEMP is the matrix we want.
                //
                MOVED(XFTEMP.as_slice(), 36, XFORM.as_slice_mut());
            }

        //
        // Now XFORM is the state transformation mapping from
        // the input frame INFRAM to the base frame BASFRM.
        //
        // This is the end of the work specific to "of-date" frames.
        // From here we drop out of the IF block.  At the end of this
        // routine, the derivative block of XFORM will be zeroed out
        // if the frame is frozen.  If the rotation state is
        // 'INERTIAL', we will make sure the transformation between
        // the defined frame and the J2000 frame has time derivative
        // zero.
        //
        } else if fstr::eq(&DYNFAM, KV2VEC) {
            //
            // The frame belongs to the TWO-VECTOR family.
            //
            // Initialize the array S2.
            //
            CLEARD(12, S2.as_slice_mut());

            //
            // Fetch the specifications of the primary and secondary
            // axes.
            //
            for I in 1..=2 {
                //
                // Get the name of the axis associated with the Ith
                // defining vector.
                //
                ZZDYNVAC(
                    &INNAME,
                    INFRAM,
                    &save.ITMAXE[I],
                    1,
                    &mut N,
                    CharArrayMut::from_mut(&mut AXNAME),
                    ctx,
                )?;

                if FAILED(ctx) {
                    CHKOUT(RNAME, ctx)?;
                    return Ok(());
                }

                CMPRSS(b" ", 0, &AXNAME.clone(), &mut AXNAME);
                UCASE(&AXNAME.clone(), &mut AXNAME, ctx);

                //
                // Set the sign flag associated with the axis.
                //
                NEGATE = fstr::eq(fstr::substr(&AXNAME, 1..=1), b"-");

                CMPRSS(b"-", 0, &AXNAME.clone(), &mut AXNAME);
                CMPRSS(b"+", 0, &AXNAME.clone(), &mut AXNAME);

                AXIS[I] = ISRCHC(&AXNAME, 3, save.AXES.as_arg());

                if (AXIS[I] == 0) {
                    SETMSG(b"Definition of frame # associates vector # with axis #.  The only valid axis values are { X, -X, Y, -Y, Z, -Z }. This situation is usually caused by an error in a frame kernel in which the frame is defined.", ctx);
                    ERRCH(b"#", &INNAME, ctx);
                    ERRINT(b"#", I, ctx);
                    ERRCH(b"#", &AXNAME, ctx);
                    SIGERR(b"SPICE(INVALIDAXIS)", ctx)?;
                    CHKOUT(RNAME, ctx)?;
                    return Ok(());
                }

                //
                // Find out how the vector is defined:
                //
                //    - Observer-target position vector
                //    - Observer-target velocity vector
                //    - Observer-target near point vector
                //    - Constant vector
                //
                // VECDEF(I) indicates the vector definition method
                // for the Ith vector.
                //
                ZZDYNVAC(
                    &INNAME,
                    INFRAM,
                    &save.ITMVDF[I],
                    1,
                    &mut N,
                    VECDEF.subarray_mut(I),
                    ctx,
                )?;

                if FAILED(ctx) {
                    CHKOUT(RNAME, ctx)?;
                    return Ok(());
                }

                CMPRSS(b" ", 0, &VECDEF[I].to_vec(), &mut VECDEF[I]);
                UCASE(&VECDEF[I].to_vec(), &mut VECDEF[I], ctx);

                if fstr::eq(VECDEF.get(I), KVPOSV) {
                    //
                    // The vector is the position of a target relative
                    // to an observer.
                    //
                    // We need a target, observer, and aberration correction.
                    //
                    ZZDYNBID(&INNAME, INFRAM, &save.ITMTRG[I], &mut TARG, ctx)?;

                    ZZDYNBID(&INNAME, INFRAM, &save.ITMOBS[I], &mut OBS, ctx)?;

                    ZZDYNVAC(
                        &INNAME,
                        INFRAM,
                        &save.ITMABC[I],
                        1,
                        &mut N,
                        CharArrayMut::from_mut(&mut ABCORR),
                        ctx,
                    )?;

                    //
                    // Look up the Ith state vector in the J2000 frame.
                    //
                    ZZSPKEZ0(
                        TARG,
                        T0,
                        b"J2000",
                        &ABCORR,
                        OBS,
                        S2.subarray_mut([1, I]),
                        &mut LT,
                        ctx,
                    )?;

                    if FAILED(ctx) {
                        CHKOUT(RNAME, ctx)?;
                        return Ok(());
                    }

                //
                // At this point, S2(*,I) contains position and
                // velocity relative to frame J2000.
                //
                } else if fstr::eq(VECDEF.get(I), KVVELV) {
                    //
                    // The vector is the velocity of a target relative
                    // to an observer.
                    //
                    // We need a target, observer, and aberration correction.
                    //
                    ZZDYNBID(&INNAME, INFRAM, &save.ITMTRG[I], &mut TARG, ctx)?;

                    ZZDYNBID(&INNAME, INFRAM, &save.ITMOBS[I], &mut OBS, ctx)?;

                    ZZDYNVAC(
                        &INNAME,
                        INFRAM,
                        &save.ITMABC[I],
                        1,
                        &mut N,
                        CharArrayMut::from_mut(&mut ABCORR),
                        ctx,
                    )?;

                    //
                    // We need to know the frame in which the velocity is
                    // defined.
                    //
                    ZZDYNFID(&INNAME, INFRAM, &save.ITMFRM[I], &mut FRID, ctx)?;
                    FRMNAM(FRID, &mut VELFRM, ctx)?;

                    if FAILED(ctx) {
                        CHKOUT(RNAME, ctx)?;
                        return Ok(());
                    }

                    //
                    // Obtain the velocity vector in the specified velocity
                    // frame.  Also obtain bracketing vectors to support
                    // discrete differentiation.  (See notes in zzdyn.inc
                    // regarding definition of DELTA.)
                    //
                    DELTA = intrinsics::DMAX1(&[1.0, (f64::powi(2.0, QEXP) * T0)]);

                    VPACK((T0 - DELTA), T0, (T0 + DELTA), TARRAY.as_slice_mut());

                    {
                        let m1__: i32 = 1;
                        let m2__: i32 = 3;
                        let m3__: i32 = 1;
                        J = m1__;
                        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                            ZZSPKEZ0(
                                TARG,
                                TARRAY[J],
                                &VELFRM,
                                &ABCORR,
                                OBS,
                                STEMP.as_slice_mut(),
                                &mut LT,
                                ctx,
                            )?;
                            //
                            // We compute the derivative using unit
                            // velocity vectors.
                            //
                            if FAILED(ctx) {
                                CHKOUT(RNAME, ctx)?;
                                return Ok(());
                            }

                            VHAT(STEMP.subarray(4), VARRAY.subarray_mut([1, J]));

                            J += m3__;
                        }
                    }

                    //
                    // Compute acceleration and fill in the velocity state
                    // vector S2(*,I).
                    //
                    QDERIV(
                        3,
                        VARRAY.subarray([1, 1]),
                        VARRAY.subarray([1, 3]),
                        DELTA,
                        ACC.as_slice_mut(),
                        ctx,
                    )?;

                    VEQU(VARRAY.subarray([1, 2]), S2.subarray_mut([1, I]));
                    VEQU(ACC.as_slice(), S2.subarray_mut([4, I]));

                    //
                    // We need the epoch VET at which VELFRM is evaluated.
                    // This epoch will be used to transform the velocity's
                    // "state" vector from VELFRM to J2000.
                    //
                    // Set the default value of VET here.
                    //
                    VET = T0;

                    //
                    // Parse the aberration correction.  Find the epoch used
                    // to evaluate the velocity vector's frame.
                    //
                    ZZVALCOR(&ABCORR, CORBLK.as_slice_mut(), ctx)?;

                    if CORBLK[LTIDX] {
                        //
                        // Light time correction is used.  The epoch used
                        // to evaluate the velocity vector's frame depends
                        // on the frame's observer and center.
                        //
                        // Look up the velocity frame's center.
                        //
                        FRINFO(FRID, &mut FRCTR, &mut FRCLS, &mut FRCID, &mut FND, ctx)?;

                        if !FND {
                            SETMSG(b"In definition of frame #, the frame associated with a velocity vector has frame ID code #, but no frame center, frame class, or frame class ID was found by FRINFO.  This situation MAY be caused by an error in a frame kernel in which the frame is defined. The problem also could be indicative of a SPICELIB bug.", ctx);
                            ERRCH(b"#", &INNAME, ctx);
                            ERRINT(b"#", FRID, ctx);
                            SIGERR(b"SPICE(FRAMEDATANOTFOUND)", ctx)?;
                            CHKOUT(RNAME, ctx)?;
                            return Ok(());
                        }

                        //
                        // If the velocity frame is non-inertial, we'll need
                        // to compute the evaluation epoch for this frame.
                        //
                        if (FRCLS != INERTL) {
                            //
                            // Obtain light time from the observer to the
                            // frame's center; find the evaluation epoch VET
                            // for the frame.

                            ZZSPKZP0(
                                FRCTR,
                                T0,
                                b"J2000",
                                &ABCORR,
                                OBS,
                                CTRPOS.as_slice_mut(),
                                &mut VFLT,
                                ctx,
                            )?;
                            ZZCOREPC(&ABCORR, T0, VFLT, &mut VET, ctx)?;

                            if FAILED(ctx) {
                                CHKOUT(RNAME, ctx)?;
                                return Ok(());
                            }
                        }
                    }

                    //
                    // The velocity frame's evaluation epoch VET is now set.
                    //
                    // We must rotate the velocity vector and transform the
                    // acceleration from the velocity frame (evaluated at
                    // VET) to the output frame at T0.  We'll do this in two
                    // stages, first mapping velocity and acceleration into
                    // the J2000 frame.
                    //
                    if (FRID != save.J2000) {
                        ZZFRMCH0(FRID, save.J2000, VET, XF2000.as_slice_mut(), ctx)?;

                        if FAILED(ctx) {
                            CHKOUT(RNAME, ctx)?;
                            return Ok(());
                        }

                        MXVG(
                            XF2000.as_slice(),
                            S2.subarray([1, I]),
                            6,
                            6,
                            STEMP.as_slice_mut(),
                        );
                        MOVED(STEMP.as_slice(), 6, S2.subarray_mut([1, I]));
                    }

                //
                // At this point, S2(*,I) contains velocity and
                // acceleration relative to frame J2000.
                //
                } else if fstr::eq(VECDEF.get(I), KVNEAR) {
                    //
                    // The vector points from an observer to the
                    // sub-observer point (nearest point to the observer) on
                    // the target body.
                    //
                    // We need a target, observer, and aberration correction.
                    //
                    ZZDYNBID(&INNAME, INFRAM, &save.ITMTRG[I], &mut TARG, ctx)?;

                    ZZDYNBID(&INNAME, INFRAM, &save.ITMOBS[I], &mut OBS, ctx)?;

                    ZZDYNVAC(
                        &INNAME,
                        INFRAM,
                        &save.ITMABC[I],
                        1,
                        &mut N,
                        CharArrayMut::from_mut(&mut ABCORR),
                        ctx,
                    )?;

                    //
                    // The vector points from the observer to the nearest
                    // point on the target. We need the state of the near
                    // point relative to the observer.
                    //
                    // We'll look up the state of the target center relative
                    // to the observer and the state of the near point
                    // relative to the target center, both in the body-fixed
                    // frame associated with the target.
                    //
                    // Look up the body-fixed frame associated with the
                    // target body.
                    //
                    CIDFRM(TARG, &mut CFRMID, &mut CFRMNM, &mut FND, ctx)?;

                    if !FND {
                        SETMSG(b"Definition of frame # requires definition of body-fixed frame associated with target body #. A call to CIDFRM indicated no body-fixed frame is associated with the target body.  This situation can arise when a frame kernel defining the target\'s body-fixed frame  lacks the OBJECT_<ID>_FRAME or OBJECT_<name>_FRAME keywords.  The problem also could be caused by an error in a frame kernel in which the parameterized two-vector dynamic frame # is defined.", ctx);
                        ERRCH(b"#", &INNAME, ctx);
                        ERRINT(b"#", TARG, ctx);
                        ERRCH(b"#", &INNAME, ctx);
                        SIGERR(b"SPICE(FRAMEDATANOTFOUND)", ctx)?;
                        CHKOUT(RNAME, ctx)?;
                        return Ok(());
                    }

                    //
                    // Get the radii of the target body.
                    //
                    ZZGFTREB(TARG, RADII.as_slice_mut(), ctx)?;

                    //
                    // Look up the Ith state vector in the target-fixed
                    // frame.  Negate the vector to obtain the target-to-
                    // observer vector.
                    //
                    ZZSPKEZ0(
                        TARG,
                        T0,
                        &CFRMNM,
                        &ABCORR,
                        OBS,
                        STEMP.as_slice_mut(),
                        &mut LT,
                        ctx,
                    )?;

                    //
                    // We check FAILED() here because VMINUG is a simple
                    // arithmetic routine that doesn't return on entry.
                    //
                    if FAILED(ctx) {
                        CHKOUT(RNAME, ctx)?;
                        return Ok(());
                    }

                    VMINUG(STEMP.as_slice(), 6, STOBS.as_slice_mut());

                    DNEARP(
                        STOBS.as_slice(),
                        RADII[1],
                        RADII[2],
                        RADII[3],
                        STNEAR.as_slice_mut(),
                        STALT.as_slice_mut(),
                        &mut FND,
                        ctx,
                    )?;

                    if FAILED(ctx) {
                        CHKOUT(RNAME, ctx)?;
                        return Ok(());
                    }

                    if !FND {
                        SETMSG(b"In definition of frame #, vector # is defined by the near point on body # as seen from body #.  The state of this near point was not found. See the routine DNEARP for an explanation.  This situation MAY be caused by an error in a frame kernel in which the frame is defined. The problem also could be indicative of a SPICELIB bug.", ctx);
                        ERRCH(b"#", &INNAME, ctx);
                        ERRINT(b"#", I, ctx);
                        ERRINT(b"#", TARG, ctx);
                        ERRINT(b"#", OBS, ctx);
                        SIGERR(b"SPICE(DEGENERATECASE)", ctx)?;
                        CHKOUT(RNAME, ctx)?;
                        return Ok(());
                    }

                    //
                    // Find the observer-near point state in the target
                    // body-fixed frame.
                    //
                    VSUBG(STNEAR.as_slice(), STOBS.as_slice(), 6, STEMP.as_slice_mut());

                    //
                    // Transform the state to frame J2000.  To get the
                    // required transformation matrix, we'll need to obtain
                    // the epoch associated with CNMFRM.  Parse the
                    // aberration correction and adjust the frame evaluation
                    // epoch as needed.
                    //
                    ZZCOREPC(&ABCORR, T0, LT, &mut FET, ctx)?;

                    //
                    // Obtain the matrix for transforming state vectors
                    // from the target center frame to the J2000 frame and
                    // apply it to the observer-to-near point state vector.
                    //
                    ZZFRMCH0(CFRMID, save.J2000, FET, XIPM.as_slice_mut(), ctx)?;

                    if FAILED(ctx) {
                        CHKOUT(RNAME, ctx)?;
                        return Ok(());
                    }

                    MXVG(
                        XIPM.as_slice(),
                        STEMP.as_slice(),
                        6,
                        6,
                        S2.subarray_mut([1, I]),
                    );

                //
                // At this point, S2(*,I) contains position and
                // velocity of the near point on the target as
                // seen by the observer, relative to frame J2000.
                //
                } else if fstr::eq(VECDEF.get(I), KVCONS) {
                    //
                    // The vector is constant in a specified frame.
                    //
                    //
                    // We need a 3-vector and an associated reference
                    // frame relative to which the vector is specified.
                    //
                    // Look up the ID of the frame first.
                    //
                    ZZDYNFID(&INNAME, INFRAM, &save.ITMFRM[I], &mut FRID, ctx)?;

                    //
                    // Let FET ("frame ET") be the evaluation epoch for
                    // the constant vector's frame.  By default, this
                    // frame is just T0, but if we're using light time
                    // corrections, FET must be adjusted for one-way
                    // light time between the frame's center and the
                    // observer.
                    //
                    // Set the default value of FET here.
                    //
                    FET = T0;

                    //
                    // Optionally, there is an aberration correction
                    // associated with the constant vector's frame.
                    // If so, an observer must be associated with the
                    // frame.  Look up the correction first.
                    //
                    ZZDYNOAC(
                        &INNAME,
                        INFRAM,
                        &save.ITMABC[I],
                        1,
                        &mut N,
                        CharArrayMut::from_mut(&mut CVCORR),
                        &mut FND,
                        ctx,
                    )?;

                    if !FND {
                        fstr::assign(&mut CVCORR, b"NONE");
                    }

                    ZZPRSCOR(&CVCORR, CORBLK.as_slice_mut(), ctx)?;

                    if FAILED(ctx) {
                        CHKOUT(RNAME, ctx)?;
                        return Ok(());
                    }

                    if !CORBLK[GEOIDX] {
                        //
                        // We need to apply an aberration correction to
                        // the constant vector.
                        //
                        // Check for errors in the aberration correction
                        // specification.
                        //
                        //    - Light time and stellar aberration corrections
                        //      are mutually exclusive.
                        //
                        if (CORBLK[LTIDX] && CORBLK[STLIDX]) {
                            SETMSG(b"Definition of frame # specifies aberration correction # for constant vector.  Light time and stellar aberration corrections are mutually exclusive for constant vectors used in two-vector parameterized dynamic frame definitions.  This situation is usually caused by an error in a frame kernel in which the frame is defined.", ctx);
                            ERRCH(b"#", &INNAME, ctx);
                            ERRCH(b"#", &CVCORR, ctx);
                            SIGERR(b"SPICE(INVALIDOPTION)", ctx)?;
                            CHKOUT(RNAME, ctx)?;
                            return Ok(());
                        }

                        if CORBLK[LTIDX] {
                            //
                            // Light time correction is used.  The epoch used
                            // to evaluate the constant vector's frame depends
                            // on the frame's observer and center.
                            //
                            // Look up the constant vector frame's center.
                            //
                            FRINFO(FRID, &mut FRCTR, &mut FRCLS, &mut FRCID, &mut FND, ctx)?;

                            if !FND {
                                SETMSG(b"In definition of frame #, the frame associated with a constant vector has frame ID code #, but no frame center, frame class, or frame class ID was found by FRINFO.  This situation MAY be caused by an error in a frame kernel in which the frame is defined. The problem also could be indicative of a SPICELIB bug.", ctx);
                                ERRCH(b"#", &INNAME, ctx);
                                ERRINT(b"#", FRID, ctx);
                                SIGERR(b"SPICE(FRAMEDATANOTFOUND)", ctx)?;
                                CHKOUT(RNAME, ctx)?;
                                return Ok(());
                            }
                            //
                            // If the constant vector frame is non-inertial,
                            // we'll need to compute the evaluation epoch for
                            // this frame.
                            //
                            if (FRCLS != INERTL) {
                                //
                                // Look up the observer associated with the
                                // constant vector's frame.  This observer,
                                // together with the frame's center, determines
                                // the evaluation epoch for the frame.
                                //
                                ZZDYNBID(&INNAME, INFRAM, &save.ITMOBS[I], &mut CVOBS, ctx)?;

                                //
                                // Obtain light time from the observer to the
                                // frame's center.
                                //
                                ZZSPKZP0(
                                    FRCTR,
                                    T0,
                                    b"J2000",
                                    &CVCORR,
                                    CVOBS,
                                    CTRPOS.as_slice_mut(),
                                    &mut LT,
                                    ctx,
                                )?;

                                //
                                // Re-set the evaluation epoch for the frame.
                                //
                                ZZCOREPC(&CVCORR, T0, LT, &mut FET, ctx)?;

                                if FAILED(ctx) {
                                    CHKOUT(RNAME, ctx)?;
                                    return Ok(());
                                }
                            }
                        //
                        // The constant vector frame's evaluation epoch
                        // FET has been set.
                        //
                        } else if CORBLK[STLIDX] {
                            //
                            // Stellar aberration case.
                            //
                            // The constant vector must be corrected for
                            // stellar aberration induced by the observer's
                            // velocity relative to the solar system
                            // barycenter.  First, find this velocity in
                            // the J2000 frame.  We'll apply the correction
                            // later, when the constant vector has been
                            // transformed to the J2000 frame.
                            //
                            ZZDYNBID(&INNAME, INFRAM, &save.ITMOBS[I], &mut CVOBS, ctx)?;

                            ZZSPKSB0(CVOBS, T0, b"J2000", STOBS.as_slice_mut(), ctx)?;

                            if FAILED(ctx) {
                                CHKOUT(RNAME, ctx)?;
                                return Ok(());
                            }
                        }
                    }

                    //
                    // At this point FET is the frame evaluation epoch
                    // for the frame associated with the constant vector.
                    //
                    // If stellar aberration correction has been specified,
                    // STOBS is the state of the observer relative to the
                    // solar system barycenter, expressed in the J2000
                    // frame.
                    //
                    // Get the constant vector specification.
                    //
                    ZZDYNVAC(
                        &INNAME,
                        INFRAM,
                        &save.ITMSPC[I],
                        1,
                        &mut N,
                        CharArrayMut::from_mut(&mut SPEC),
                        ctx,
                    )?;

                    if FAILED(ctx) {
                        CHKOUT(RNAME, ctx)?;
                        return Ok(());
                    }

                    CMPRSS(b" ", 0, &SPEC.clone(), &mut SPEC);
                    UCASE(&SPEC.clone(), &mut SPEC, ctx);

                    if fstr::eq(&SPEC, KVRECC) {
                        //
                        // The coordinate system is rectangular.
                        //
                        // Look up the constant vector.
                        //
                        ZZDYNVAD(
                            &INNAME,
                            INFRAM,
                            &save.ITMVEC[I],
                            3,
                            &mut N,
                            DIRVEC.as_slice_mut(),
                            ctx,
                        )?;
                    } else if (fstr::eq(&SPEC, KVLATC) || fstr::eq(&SPEC, KVRADC)) {
                        //
                        // The coordinate system is latitudinal or RA/DEC.
                        //
                        // Look up the units associated with the angles.
                        //
                        ZZDYNVAC(
                            &INNAME,
                            INFRAM,
                            &save.ITMUNT[I],
                            1,
                            &mut N,
                            CharArrayMut::from_mut(&mut UNITS),
                            ctx,
                        )?;

                        if fstr::eq(&SPEC, KVLATC) {
                            //
                            // Look up longitude and latitude.
                            //
                            ZZDYNVAD(
                                &INNAME,
                                INFRAM,
                                &save.ITMLON[I],
                                1,
                                &mut N,
                                std::slice::from_mut(&mut LON),
                                ctx,
                            )?;

                            ZZDYNVAD(
                                &INNAME,
                                INFRAM,
                                &save.ITMLAT[I],
                                1,
                                &mut N,
                                std::slice::from_mut(&mut LAT),
                                ctx,
                            )?;

                            //
                            // Convert angles from input units to radians.
                            //
                            CONVRT(LON, &UNITS, KVRADN, &mut ANGLES[1], ctx)?;
                            CONVRT(LAT, &UNITS, KVRADN, &mut ANGLES[2], ctx)?;
                        } else {
                            //
                            // Look up RA and DEC.
                            //
                            ZZDYNVAD(
                                &INNAME,
                                INFRAM,
                                &save.ITMRA[I],
                                1,
                                &mut N,
                                std::slice::from_mut(&mut RA),
                                ctx,
                            )?;

                            ZZDYNVAD(
                                &INNAME,
                                INFRAM,
                                &save.ITMDEC[I],
                                1,
                                &mut N,
                                std::slice::from_mut(&mut DEC),
                                ctx,
                            )?;

                            //
                            // Convert angles from input units to radians.
                            //
                            CONVRT(RA, &UNITS, KVRADN, &mut ANGLES[1], ctx)?;
                            CONVRT(DEC, &UNITS, KVRADN, &mut ANGLES[2], ctx)?;
                        }

                        if FAILED(ctx) {
                            CHKOUT(RNAME, ctx)?;
                            return Ok(());
                        }

                        //
                        // Now  produce a direction vector.
                        //
                        LATREC(1.0, ANGLES[1], ANGLES[2], DIRVEC.as_slice_mut());
                    } else {
                        SETMSG(b"Definition of two-vector parameterized dynamic frame # includes constant vector specification #, which is not supported.  This situation is usually caused by an error in a frame kernel in which the frame is defined.", ctx);
                        ERRCH(b"#", &INNAME, ctx);
                        ERRCH(b"#", &SPEC, ctx);
                        SIGERR(b"SPICE(NOTSUPPORTED)", ctx)?;
                        CHKOUT(RNAME, ctx)?;
                        return Ok(());
                    }

                    //
                    // At this point, the Cartesian coordinates of the
                    // vector relative to the constant vector frame
                    // are stored in DIRVEC.
                    //
                    // Convert the direction vector to the J2000 frame.
                    // Fill in the state vector.  The velocity in the
                    // constant vector's frame is zero.
                    //
                    VEQU(DIRVEC.as_slice(), S2.subarray_mut([1, I]));
                    CLEARD(3, S2.subarray_mut([4, I]));

                    if (FRID != save.J2000) {
                        ZZFRMCH0(FRID, save.J2000, FET, XIPM.as_slice_mut(), ctx)?;

                        if FAILED(ctx) {
                            CHKOUT(RNAME, ctx)?;
                            return Ok(());
                        }

                        MXVG(
                            XIPM.as_slice(),
                            S2.subarray([1, I]),
                            6,
                            6,
                            STEMP.as_slice_mut(),
                        );
                        MOVED(STEMP.as_slice(), 6, S2.subarray_mut([1, I]));
                    }

                    //
                    // The state of the constant vector is now represented
                    // in the J2000 frame, but we may still need to
                    // apply a stellar aberration correction.
                    //
                    if CORBLK[STLIDX] {
                        //
                        // Perform the stellar aberration correction
                        // appropriate to the radiation travel sense.

                        if CORBLK[XMTIDX] {
                            //
                            // The correction is for transmission.
                            //
                            STLABX(
                                S2.subarray([1, I]),
                                STOBS.subarray(4),
                                STEMP.as_slice_mut(),
                                ctx,
                            )?;
                        } else {
                            //
                            // The correction is for reception.
                            //
                            STELAB(
                                S2.subarray([1, I]),
                                STOBS.subarray(4),
                                STEMP.as_slice_mut(),
                                ctx,
                            )?;
                        }

                        if FAILED(ctx) {
                            CHKOUT(RNAME, ctx)?;
                            return Ok(());
                        }

                        //
                        // Update the position portion of S2(*,I).
                        //
                        VEQU(STEMP.as_slice(), S2.subarray_mut([1, I]));
                    }

                //
                // At this point, S2(*,I) contains position and velocity
                // of the constant (constant relative to its associated
                // frame, that is) vector as seen by the observer,
                // relative to frame J2000.
                //
                } else {
                    SETMSG(b"Definition of two-vector parameterized dynamic frame # includes vector definition #, which is not supported.  This situation is usually caused by an error in a frame kernel in which the frame is defined.", ctx);
                    ERRCH(b"#", &INNAME, ctx);
                    ERRCH(b"#", &VECDEF[I], ctx);
                    SIGERR(b"SPICE(NOTSUPPORTED)", ctx)?;
                    CHKOUT(RNAME, ctx)?;
                    return Ok(());
                }

                //
                // Negate the state vector if the axis has negative sign.
                //
                if NEGATE {
                    VMINUG(S2.subarray([1, I]), 6, STEMP.as_slice_mut());
                    MOVED(STEMP.as_slice(), 6, S2.subarray_mut([1, I]));
                }
            }

            //
            // Look up the lower bound for the angular separation of
            // the defining vectors.  Use the default value if none
            // was supplied.
            //
            ZZDYNOAD(
                &INNAME,
                INFRAM,
                &save.ITMSEP,
                1,
                &mut N,
                std::slice::from_mut(&mut MINSEP),
                &mut FND,
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(RNAME, ctx)?;
                return Ok(());
            }

            if !FND {
                MINSEP = LBSEP;
            }

            //
            // Now use our states to compute our state transformation
            // matrix.
            //
            // Check the angular separation of the defining vectors. We
            // want to ensure that the vectors are not too close to being
            // linearly dependent.  We can handle both cases---separation
            // close to 0 or separation close to Pi---by comparing the
            // sine of the separation to the sine of the separation limit.
            //
            SEP = VSEP(S2.subarray([1, 1]), S2.subarray([1, 2]), ctx);

            if (f64::sin(SEP) < f64::sin(MINSEP)) {
                ETCAL(T0, &mut TIMSTR, ctx);

                SETMSG(b"Angular separation of vectors defining two-vector parameterized dynamic frame # is # (radians); minimum allowed difference of separation from 0 or Pi is # radians.  Evaluation epoch is #.  Extreme loss of precision can occur when defining vectors are nearly linearly dependent.  This type of error can be due to using a dynamic frame outside of the time range for which it is meant. It also can be due to a conceptual error pertaining to the frame\'s definition, or to an implementation error in the frame kernel containing the frame definition. However, if you wish to proceed with this computation, the # keyword can be used in the frame definition to adjust the separation limit.", ctx);
                ERRCH(b"#", &INNAME, ctx);
                ERRDP(b"#", SEP, ctx);
                ERRDP(b"#", MINSEP, ctx);
                ERRCH(b"#", &TIMSTR, ctx);
                ERRCH(b"#", KWATOL, ctx);
                SIGERR(b"SPICE(DEGENERATECASE)", ctx)?;
                CHKOUT(RNAME, ctx)?;
                return Ok(());
            }

            //
            // We have both states expressed relative to frame J2000
            // at this point.  Find the transformation from INNAME to
            // the frame J2000, then from J2000 to frame BASNAM.
            //
            ZZTWOVXF(
                S2.subarray([1, 1]),
                AXIS[1],
                S2.subarray([1, 2]),
                AXIS[2],
                XFORM.as_slice_mut(),
                ctx,
            )?;

            if (*BASFRM != save.J2000) {
                MOVED(XFORM.as_slice(), 36, XFTEMP.as_slice_mut());

                ZZFRMCH0(save.J2000, *BASFRM, T0, XF2000.as_slice_mut(), ctx)?;

                if FAILED(ctx) {
                    CHKOUT(RNAME, ctx)?;
                    return Ok(());
                }

                MXMG(
                    XF2000.as_slice(),
                    XFTEMP.as_slice(),
                    6,
                    6,
                    6,
                    XFORM.as_slice_mut(),
                );
            }

        //
        // This is the end of the work specific to two-vector frames.
        // From here we drop out of the IF block.  At the end of this
        // routine, the derivative block of XFORM will be zeroed out
        // if the frame is frozen.  If the rotation state is
        // 'INERTIAL', we will make sure the transformation between
        // the defined frame and the J2000 frame has time derivative
        // zero.
        //
        } else if fstr::eq(&DYNFAM, KVEULR) {
            //
            // The frame belongs to the Euler family.
            //
            // We expect to see specifications of an axis sequence, units,
            // and angles via polynomial coefficients.  We also expect
            // to see an ET epoch.
            //
            // Look up the epoch first.  Let DELTA represent the offset
            // of T0 relative to the epoch.
            //
            // Initialize EPOCH so subtraction doesn't overflow if EPOCH
            // is invalid due to a lookup error.
            //
            EPOCH = 0.0;

            ZZDYNVAD(
                &INNAME,
                INFRAM,
                KWEPOC,
                1,
                &mut N,
                std::slice::from_mut(&mut EPOCH),
                ctx,
            )?;

            DELTA = (T0 - EPOCH);

            //
            // Now the axis sequence.
            //
            ZZDYNVAI(
                &INNAME,
                INFRAM,
                KWEUAX,
                3,
                &mut N,
                IAXES.as_slice_mut(),
                ctx,
            )?;

            //
            // Now the coefficients for the angles.
            //
            for I in 1..=3 {
                //
                // Initialize N so subtraction doesn't overflow if N
                // is invalid due to a lookup error.
                //
                N = 0;

                ZZDYNVAD(
                    &INNAME,
                    INFRAM,
                    &save.ITMCOF[I],
                    MAXCOF,
                    &mut N,
                    COEFFS.subarray_mut([1, I]),
                    ctx,
                )?;

                //
                // Set the polynomial degree for the Ith angle.
                //
                DEGS[I] = (N - 1);
            }

            //
            // Look up the units associated with the angles.
            //
            ZZDYNVAC(
                &INNAME,
                INFRAM,
                KWUNIT,
                1,
                &mut N,
                CharArrayMut::from_mut(&mut UNITS),
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(RNAME, ctx)?;
                return Ok(());
            }

            //
            // Evaluate the angles and their derivatives at DELTA.  Convert
            // angles from input units to radians and radians/sec.
            //
            for I in 1..=3 {
                POLYDS(
                    COEFFS.subarray([1, I]),
                    DEGS[I],
                    1,
                    DELTA,
                    POLY.as_slice_mut(),
                );
                //
                // Convert units.  Fill in the Euler angle state vector.
                //
                CONVRT(POLY[0], &UNITS, KVRADN, &mut EULANG[I], ctx)?;
                CONVRT(POLY[1], &UNITS, KVRADN, &mut EULANG[(I + 3)], ctx)?;
            }

            //
            // Produce a state transformation matrix that maps from
            // the defined frame to the base frame.
            //
            EUL2XF(
                EULANG.as_slice(),
                IAXES[1],
                IAXES[2],
                IAXES[3],
                XFORM.as_slice_mut(),
                ctx,
            )?;

        //
        // This is the end of the work specific to Euler frames.
        // From here we drop out of the IF block.  At the end of this
        // routine, the derivative block of XFORM will be zeroed out
        // if the frame is frozen.  If the rotation state is
        // 'INERTIAL', we will make sure the transformation between
        // the defined frame and the J2000 frame has time derivative
        // zero.
        //
        } else if fstr::eq(&DYNFAM, KVPROD) {
            //
            // The frame belongs to the product family.
            //
            // We expect to see specifications of "from" and "to" frames
            // that make up the product.
            //
            // Look up the "from" and "to" frames that define the product.
            //
            ZZDYNVAC(
                &INNAME,
                INFRAM,
                KWFFRM,
                MXNFAC,
                &mut M,
                FFRAMS.as_arg_mut(),
                ctx,
            )?;
            ZZDYNVAC(
                &INNAME,
                INFRAM,
                KWTFRM,
                MXNFAC,
                &mut N,
                TFRAMS.as_arg_mut(),
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(RNAME, ctx)?;
                return Ok(());
            }

            if (N != M) {
                SETMSG(b"Definition of product parameterized dynamic frame # has # \"from\" frames and # \"to\" frames. These counts must match.", ctx);
                ERRCH(b"#", &INNAME, ctx);
                ERRINT(b"#", M, ctx);
                ERRINT(b"#", N, ctx);
                SIGERR(b"SPICE(BADFRAMECOUNT)", ctx)?;
                CHKOUT(RNAME, ctx)?;
                return Ok(());
            }

            //
            // The product of the factors is the transformation from the
            // base frame to the output frame. We need the inverse of
            // this transformation.
            //
            // Compute the inverse of the product of the factors; place
            // the result in XFORM.
            //
            NAMFRM(&FFRAMS[1], &mut FROMID, ctx)?;
            NAMFRM(&TFRAMS[1], &mut TOID, ctx)?;

            ZZFRMCH0(TOID, FROMID, T0, XFORM.as_slice_mut(), ctx)?;

            if FAILED(ctx) {
                CHKOUT(RNAME, ctx)?;
                return Ok(());
            }

            for I in 2..=N {
                MOVED(XFORM.as_slice(), 36, XFTEMP.as_slice_mut());

                NAMFRM(&FFRAMS[I], &mut FROMID, ctx)?;
                NAMFRM(&TFRAMS[I], &mut TOID, ctx)?;

                ZZFRMCH0(TOID, FROMID, T0, XFORM.as_slice_mut(), ctx)?;

                if FAILED(ctx) {
                    CHKOUT(RNAME, ctx)?;
                    return Ok(());
                }

                MXMG(
                    XFORM.as_slice(),
                    XFTEMP.as_slice(),
                    6,
                    6,
                    6,
                    XOUT.as_slice_mut(),
                );
                MOVED(XOUT.as_slice(), 36, XFORM.as_slice_mut());
            }
        } else {
            SETMSG(b"Dynamic frame family # (in definition of frame #) is not supported. This situation is usually caused by an error in a frame kernel in which the frame is defined.", ctx);
            ERRCH(b"#", &DYNFAM, ctx);
            ERRCH(b"#", &INNAME, ctx);
            SIGERR(b"SPICE(NOTSUPPORTED)", ctx)?;
            CHKOUT(RNAME, ctx)?;
            return Ok(());
        }
    //
    // This is the end of the IF block that processes the
    // parameterized dynamic frame families.
    //
    } else {
        SETMSG(b"Dynamic frame style # (in definition of frame #) is not supported. This situation is usually caused by an error in a frame kernel in which the frame is defined.", ctx);
        ERRCH(b"#", &DYNSTL, ctx);
        ERRCH(b"#", &INNAME, ctx);
        SIGERR(b"SPICE(NOTSUPPORTED)", ctx)?;
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    if FAILED(ctx) {
        CHKOUT(RNAME, ctx)?;
        return Ok(());
    }

    //
    // At this point XFORM is the state transformation matrix mapping
    // from the input frame INFRAM to the base frame BASFRM.
    //
    // If the frame has rotation state 'INERTIAL', the frame must have
    // zero derivative with respect to any inertial frame. Set the
    // derivative block accordingly.
    //
    if INERT {
        //
        // See whether the base frame is inertial.
        //
        IRFNUM(&BASNAM, &mut J, ctx);

        if (J > 0) {
            //
            // The base frame is  a recognized inertial frame.  Zero
            // out the derivative block.
            //
            for I in 1..=3 {
                CLEARD(3, XFORM.subarray_mut([4, I]));
            }
        } else {
            //
            // The base frame is *not* a recognized inertial frame.
            //
            // Create the state transformation matrix that maps from the
            // defined frame to J2000.  Zero out the derivative block of
            // this matrix.  Convert the resulting matrix to the state
            // transformation from the defined frame to the output frame.
            //
            ZZFRMCH0(*BASFRM, save.J2000, T0, XF2000.as_slice_mut(), ctx)?;

            if FAILED(ctx) {
                CHKOUT(RNAME, ctx)?;
                return Ok(());
            }

            MXMG(
                XF2000.as_slice(),
                XFORM.as_slice(),
                6,
                6,
                6,
                XFTEMP.as_slice_mut(),
            );

            for I in 1..=3 {
                CLEARD(3, XFTEMP.subarray_mut([4, I]));
            }

            //
            // XFTEMP now represents the transformation from a
            // constant frame matching the defined frame at T0 to the
            // J2000 frame. Produce the transformation from this constant
            // frame to the output frame.
            //
            // To avoid introducing additional round-off error into
            // the rotation blocks of XFORM, we overwrite only the
            // derivative block of XFORM with the derivative block
            // of the "inertial" transformation.
            //
            INVSTM(XF2000.as_slice(), XFINV.as_slice_mut(), ctx)?;
            MXMG(
                XFINV.as_slice(),
                XFTEMP.as_slice(),
                6,
                6,
                6,
                XOUT.as_slice_mut(),
            );

            for I in 1..=3 {
                VEQU(XOUT.subarray([4, I]), XFORM.subarray_mut([4, I]));
            }
        }
    }

    //
    // If the frame is frozen, zero out the derivative block of the
    // transformation matrix.
    //
    if FROZEN {
        for I in 1..=3 {
            CLEARD(3, XFORM.subarray_mut([4, I]));
        }
    }

    //
    // XFORM and BASFRM are set.
    //

    CHKOUT(RNAME, ctx)?;
    Ok(())
}
