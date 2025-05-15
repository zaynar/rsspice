//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const USIZE: i32 = 32;
const NUNITS: i32 = 84;
const NUMBER: i32 = 0;
const ANGLE: i32 = 1;
const LENGTH: i32 = 2;
const TIME: i32 = 3;
const MASS: i32 = 4;
const CHARGE: i32 = 5;
const KM: f64 = 1.0;
const METER: f64 = (0.001 * KM);
const CM: f64 = (0.01 * METER);
const MM: f64 = (0.1 * CM);
const INCH: f64 = (2.54 * CM);
const FOOT: f64 = (12.0 * INCH);
const YARD: f64 = (3.0 * FOOT);
const MILEST: f64 = (5280.0 * FOOT);
const MILENT: f64 = (1852.0 * METER);
const DE200V: f64 = 149597870.66;
const SECOND: f64 = 1.0;
const MINUTE: f64 = (60.0 * SECOND);
const HOUR: f64 = (60.0 * MINUTE);
const DAY: f64 = (24.0 * HOUR);
const WEEK: f64 = (7.0 * DAY);
const JYEAR: f64 = (365.25 * DAY);
const TYEAR: f64 = (DAY * 365.2421988);
const JCENT: f64 = (100.0 * JYEAR);
const RADIAN: f64 = 1.0;
const MILRAD: f64 = (0.001 * RADIAN);
const MICRAD: f64 = (0.001 * MILRAD);
const NANRAD: f64 = (0.001 * MICRAD);
const KG: f64 = 1.0;
const GRAM: f64 = (0.001 * KG);
const POUND: f64 = (0.45359237 * KG);
const OUNCE: f64 = (POUND / 16.0);
const COULOM: f64 = 1.0;
const ECHARG: f64 = (COULOM / 6241960000000000000.0);
const STATCL: f64 = (COULOM * 2997930000.0);

struct SaveVars {
    AU: f64,
    SCALE: f64,
    PARSEC: f64,
    CANDS: Vec<u8>,
    CANDP: Vec<u8>,
    ERROR: Vec<u8>,
    DEGREE: f64,
    ARCMIN: f64,
    ARCSEC: f64,
    HRANG: f64,
    LIGHT: f64,
    LSEC: f64,
    LMIN: f64,
    LHOUR: f64,
    LDAY: f64,
    LYEAR: f64,
    MINANG: f64,
    REV: f64,
    SECANG: f64,
    COUNT: i32,
    I: i32,
    J: i32,
    PTR: i32,
    NAMES: ActualCharArray,
    UPDATE: bool,
    FOUND: bool,
    IAU: i32,
    IAUS: i32,
    IPARSC: i32,
    NNAMES: i32,
    UNITS: ActualCharArray,
    UCLASS: StackArray<i32, 84>,
    UVALUE: StackArray<f64, 84>,
    ORDVEC: StackArray<i32, 84>,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut AU: f64 = 0.0;
        let mut SCALE: f64 = 0.0;
        let mut PARSEC: f64 = 0.0;
        let mut CANDS = vec![b' '; USIZE as usize];
        let mut CANDP = vec![b' '; (USIZE + 1) as usize];
        let mut ERROR = vec![b' '; USIZE as usize];
        let mut DEGREE: f64 = 0.0;
        let mut ARCMIN: f64 = 0.0;
        let mut ARCSEC: f64 = 0.0;
        let mut HRANG: f64 = 0.0;
        let mut LIGHT: f64 = 0.0;
        let mut LSEC: f64 = 0.0;
        let mut LMIN: f64 = 0.0;
        let mut LHOUR: f64 = 0.0;
        let mut LDAY: f64 = 0.0;
        let mut LYEAR: f64 = 0.0;
        let mut MINANG: f64 = 0.0;
        let mut REV: f64 = 0.0;
        let mut SECANG: f64 = 0.0;
        let mut COUNT: i32 = 0;
        let mut I: i32 = 0;
        let mut J: i32 = 0;
        let mut PTR: i32 = 0;
        let mut NAMES = ActualCharArray::new(8, 1..=1);
        let mut UPDATE: bool = false;
        let mut FOUND: bool = false;
        let mut IAU: i32 = 0;
        let mut IAUS: i32 = 0;
        let mut IPARSC: i32 = 0;
        let mut NNAMES: i32 = 0;
        let mut UNITS = ActualCharArray::new(USIZE, 1..=NUNITS);
        let mut UCLASS = StackArray::<i32, 84>::new(1..=NUNITS);
        let mut UVALUE = StackArray::<f64, 84>::new(1..=NUNITS);
        let mut ORDVEC = StackArray::<i32, 84>::new(1..=NUNITS);
        let mut FIRST: bool = false;

        FIRST = true;

        Self {
            AU,
            SCALE,
            PARSEC,
            CANDS,
            CANDP,
            ERROR,
            DEGREE,
            ARCMIN,
            ARCSEC,
            HRANG,
            LIGHT,
            LSEC,
            LMIN,
            LHOUR,
            LDAY,
            LYEAR,
            MINANG,
            REV,
            SECANG,
            COUNT,
            I,
            J,
            PTR,
            NAMES,
            UPDATE,
            FOUND,
            IAU,
            IAUS,
            IPARSC,
            NNAMES,
            UNITS,
            UCLASS,
            UVALUE,
            ORDVEC,
            FIRST,
        }
    }
}

//$Procedure      FNDUCV ( Find unit, class and value. )
pub fn FNDUCV(
    UNIN: &[u8],
    KNOWN: &mut bool,
    CLASS: &mut i32,
    VALUE: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB Functions
    //

    //
    //
    // Local parameters
    //

    //
    // These are the various classes of recognized objects.
    //

    //
    // The reference values for length will be kilometers
    //                      for time   will be seconds
    //                      for angles will be radians
    //                      for mass   will be kilograms
    //                      for charge will be coulombs
    //

    //
    // This value will be computed at run time or default to the
    // value given here.
    //

    //
    //  Some of the units are not "defined" quantities.  In such a case
    //  a best estimate is provided as of the date of the current version
    //  of this routine.  Those estimated quantities are:
    //
    //     1 AU    --- the astronomical unit  is taken from the JPL
    //                 ephemeris DE200.  It is believed to be accurate to
    //                 about 40 meters.
    //
    //     The tropical year is the time from equinox to equinox.  This
    //     varies slightly with time.
    //
    //     1 PARSEC --- is dependent upon the value of the astronomical
    //                  unit.
    //
    //
    // 1.0d0 divided by the sin of 1 arc second
    //

    //
    // Local variables
    //

    //
    // Conversion values.
    //

    //
    // Initial values
    //

    //
    // This next block of code sets up the constants, names, values
    // and classes for all the recognized strings.  We do this here
    // because FORTRAN just doesn't do this kind of stuff in a
    // convenient manner.
    //
    if save.FIRST {
        save.FIRST = false;

        save.DEGREE = (spicelib::PI(ctx) / 180.0);
        save.ARCMIN = (save.DEGREE / 60.0);
        save.ARCSEC = (save.ARCMIN / 60.0);
        save.SCALE = (1.0 / f64::sin(save.ARCSEC));

        save.SECANG = (save.ARCSEC * 15.0);
        save.MINANG = (save.ARCMIN * 15.0);
        save.HRANG = (save.DEGREE * 15.0);

        save.REV = spicelib::TWOPI(ctx);

        save.LIGHT = spicelib::CLIGHT();
        save.LSEC = (SECOND * save.LIGHT);
        save.LMIN = (MINUTE * save.LIGHT);
        save.LHOUR = (HOUR * save.LIGHT);
        save.LDAY = (DAY * save.LIGHT);
        save.LYEAR = (JYEAR * save.LIGHT);

        save.NNAMES = 1;
        fstr::assign(save.NAMES.get_mut(1), b"AU");

        //
        // If available and the value of the AU is reasonable, we fetch
        // it from the kernel pool.  Otherwise we use the value in
        // DE200.
        //
        spicelib::SWPOOL(b"FNDUCV", save.NNAMES, save.NAMES.as_arg(), ctx)?;
        spicelib::CVPOOL(b"FNDUCV", &mut save.UPDATE, ctx)?;
        spicelib::RTPOOL(
            b"AU",
            &mut save.I,
            std::slice::from_mut(&mut save.AU),
            &mut save.FOUND,
            ctx,
        )?;

        if !save.FOUND {
            save.AU = DE200V;
        } else if (f64::abs((save.AU - DE200V)) > 10.0) {
            save.AU = DE200V;
        }

        save.PARSEC = (save.SCALE * save.AU);

        save.I = 0;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"METERS");
        save.UVALUE[save.I] = METER;
        save.UCLASS[save.I] = LENGTH;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"CM");
        save.UVALUE[save.I] = CM;
        save.UCLASS[save.I] = LENGTH;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"KM");
        save.UVALUE[save.I] = KM;
        save.UCLASS[save.I] = LENGTH;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"KMS");
        save.UVALUE[save.I] = KM;
        save.UCLASS[save.I] = LENGTH;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"CENTIMETERS");
        save.UVALUE[save.I] = CM;
        save.UCLASS[save.I] = LENGTH;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"KILOMETERS");
        save.UVALUE[save.I] = KM;
        save.UCLASS[save.I] = LENGTH;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"INCH");
        save.UVALUE[save.I] = INCH;
        save.UCLASS[save.I] = LENGTH;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"INCHES");
        save.UVALUE[save.I] = INCH;
        save.UCLASS[save.I] = LENGTH;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"FOOT");
        save.UVALUE[save.I] = FOOT;
        save.UCLASS[save.I] = LENGTH;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"FEET");
        save.UVALUE[save.I] = FOOT;
        save.UCLASS[save.I] = LENGTH;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"YARDS");
        save.UVALUE[save.I] = YARD;
        save.UCLASS[save.I] = LENGTH;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"AU");
        save.UVALUE[save.I] = save.AU;
        save.UCLASS[save.I] = LENGTH;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"AUS");
        save.UVALUE[save.I] = save.AU;
        save.UCLASS[save.I] = LENGTH;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"MILES");
        save.UVALUE[save.I] = MILEST;
        save.UCLASS[save.I] = LENGTH;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"STATUTE_MILES");
        save.UVALUE[save.I] = MILEST;
        save.UCLASS[save.I] = LENGTH;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"LIGHTSECONDS");
        save.UVALUE[save.I] = save.LSEC;
        save.UCLASS[save.I] = LENGTH;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"LIGHTYEAR");
        save.UVALUE[save.I] = save.LYEAR;
        save.UCLASS[save.I] = LENGTH;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"SECS");
        save.UVALUE[save.I] = SECOND;
        save.UCLASS[save.I] = TIME;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"SECONDS");
        save.UVALUE[save.I] = SECOND;
        save.UCLASS[save.I] = TIME;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"MINS");
        save.UVALUE[save.I] = MINUTE;
        save.UCLASS[save.I] = TIME;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"MINUTES");
        save.UVALUE[save.I] = MINUTE;
        save.UCLASS[save.I] = TIME;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"HRS");
        save.UVALUE[save.I] = HOUR;
        save.UCLASS[save.I] = TIME;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"HOURS");
        save.UVALUE[save.I] = HOUR;
        save.UCLASS[save.I] = TIME;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"DAYS");
        save.UVALUE[save.I] = DAY;
        save.UCLASS[save.I] = TIME;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"WEEKS");
        save.UVALUE[save.I] = WEEK;
        save.UCLASS[save.I] = TIME;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"JYEARS");
        save.UVALUE[save.I] = JYEAR;
        save.UCLASS[save.I] = TIME;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"JULIAN_YEARS");
        save.UVALUE[save.I] = JYEAR;
        save.UCLASS[save.I] = TIME;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"CENTURY");
        save.UVALUE[save.I] = JCENT;
        save.UCLASS[save.I] = TIME;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"CENTURIES");
        save.UVALUE[save.I] = JCENT;
        save.UCLASS[save.I] = TIME;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"JULIAN_CENTURIES");
        save.UVALUE[save.I] = JCENT;
        save.UCLASS[save.I] = TIME;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"JULIAN_CENTURY");
        save.UVALUE[save.I] = JCENT;
        save.UCLASS[save.I] = TIME;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"LIGHTDAYS");
        save.UVALUE[save.I] = save.LDAY;
        save.UCLASS[save.I] = LENGTH;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"LIGHTYEARS");
        save.UVALUE[save.I] = save.LYEAR;
        save.UCLASS[save.I] = LENGTH;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"RADIANS");
        save.UVALUE[save.I] = RADIAN;
        save.UCLASS[save.I] = ANGLE;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"MILLIRADIANS");
        save.UVALUE[save.I] = MILRAD;
        save.UCLASS[save.I] = ANGLE;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"MICRORADIANS");
        save.UVALUE[save.I] = MICRAD;
        save.UCLASS[save.I] = ANGLE;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"NANORADIANS");
        save.UVALUE[save.I] = NANRAD;
        save.UCLASS[save.I] = ANGLE;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"DEGREES");
        save.UVALUE[save.I] = save.DEGREE;
        save.UCLASS[save.I] = ANGLE;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"DEGS");
        save.UVALUE[save.I] = save.DEGREE;
        save.UCLASS[save.I] = ANGLE;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"ARCSECONDS");
        save.UVALUE[save.I] = save.ARCSEC;
        save.UCLASS[save.I] = ANGLE;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"ARCMINUTES");
        save.UVALUE[save.I] = save.ARCMIN;
        save.UCLASS[save.I] = ANGLE;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"SECONDANGLES");
        save.UVALUE[save.I] = save.SECANG;
        save.UCLASS[save.I] = ANGLE;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"MINUTEANGLES");
        save.UVALUE[save.I] = save.MINANG;
        save.UCLASS[save.I] = ANGLE;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"HOURANGLES");
        save.UVALUE[save.I] = save.HRANG;
        save.UCLASS[save.I] = ANGLE;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"KILOGRAMS");
        save.UVALUE[save.I] = KG;
        save.UCLASS[save.I] = MASS;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"KGS");
        save.UVALUE[save.I] = KG;
        save.UCLASS[save.I] = MASS;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"GRAMS");
        save.UVALUE[save.I] = GRAM;
        save.UCLASS[save.I] = MASS;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"POUNDS");
        save.UVALUE[save.I] = POUND;
        save.UCLASS[save.I] = MASS;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"OUNCES");
        save.UVALUE[save.I] = OUNCE;
        save.UCLASS[save.I] = MASS;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"PARSECS");
        save.UVALUE[save.I] = save.PARSEC;
        save.UCLASS[save.I] = LENGTH;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"YEARS");
        save.UVALUE[save.I] = JYEAR;
        save.UCLASS[save.I] = TIME;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"JULIANYEARS");
        save.UVALUE[save.I] = JYEAR;
        save.UCLASS[save.I] = TIME;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"TROPICALYEARS");
        save.UVALUE[save.I] = TYEAR;
        save.UCLASS[save.I] = TIME;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"TROPICAL_YEARS");
        save.UVALUE[save.I] = TYEAR;
        save.UCLASS[save.I] = TIME;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"STATUTEMILES");
        save.UVALUE[save.I] = MILEST;
        save.UCLASS[save.I] = LENGTH;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"NAUTICALMILES");
        save.UVALUE[save.I] = MILENT;
        save.UCLASS[save.I] = LENGTH;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"NAUTICAL_MILES");
        save.UVALUE[save.I] = MILENT;
        save.UCLASS[save.I] = LENGTH;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"MMS");
        save.UVALUE[save.I] = MM;
        save.UCLASS[save.I] = LENGTH;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"MILLIMETERS");
        save.UVALUE[save.I] = MM;
        save.UCLASS[save.I] = LENGTH;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"REVOLUTIONS");
        save.UVALUE[save.I] = save.REV;
        save.UCLASS[save.I] = ANGLE;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"REVS");
        save.UVALUE[save.I] = save.REV;
        save.UCLASS[save.I] = ANGLE;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"LIGHTHOURS");
        save.UVALUE[save.I] = save.LHOUR;
        save.UCLASS[save.I] = LENGTH;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"LIGHTMINUTES");
        save.UVALUE[save.I] = save.LMIN;
        save.UCLASS[save.I] = LENGTH;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"COULOMBS");
        save.UVALUE[save.I] = COULOM;
        save.UCLASS[save.I] = CHARGE;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"ELECTRON_CHARGES");
        save.UVALUE[save.I] = ECHARG;
        save.UCLASS[save.I] = CHARGE;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"STATCOULOMBS");
        save.UVALUE[save.I] = STATCL;
        save.UCLASS[save.I] = CHARGE;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"PI");
        save.UVALUE[save.I] = spicelib::PI(ctx);
        save.UCLASS[save.I] = NUMBER;

        save.I = (save.I + 1);
        fstr::assign(save.UNITS.get_mut(save.I), b"-PI");
        save.UVALUE[save.I] = -spicelib::PI(ctx);
        save.UCLASS[save.I] = NUMBER;
        //
        // I         = I + 1
        // UNITS(I)  =
        // UVALUE(I) =
        // UCLASS(I) =
        //
        save.COUNT = save.I;

        //
        // Sort everything for quick lookup.
        //
        spicelib::ORDERC(save.UNITS.as_arg(), save.COUNT, save.ORDVEC.as_slice_mut());
        spicelib::REORDC(
            save.ORDVEC.as_slice_mut(),
            save.COUNT,
            save.UNITS.as_arg_mut(),
        );
        spicelib::REORDD(
            save.ORDVEC.as_slice_mut(),
            save.COUNT,
            save.UVALUE.as_slice_mut(),
        );
        spicelib::REORDI(
            save.ORDVEC.as_slice_mut(),
            save.COUNT,
            save.UCLASS.as_slice_mut(),
        );
    }

    spicelib::CVPOOL(b"FNDUCV", &mut save.UPDATE, ctx)?;

    if save.UPDATE {
        save.IAU = spicelib::BSRCHC(b"AU", save.COUNT, save.UNITS.as_arg());
        save.IAUS = spicelib::BSRCHC(b"AUS", save.COUNT, save.UNITS.as_arg());
        save.IPARSC = spicelib::BSRCHC(b"PARSECS", save.COUNT, save.UNITS.as_arg());

        spicelib::RTPOOL(
            b"AU",
            &mut save.I,
            std::slice::from_mut(&mut save.AU),
            &mut save.FOUND,
            ctx,
        )?;

        if (f64::abs((save.AU - DE200V)) < 10.0) {
            save.UVALUE[save.IAU] = save.AU;
            save.UVALUE[save.IAUS] = save.AU;
            save.UVALUE[save.IPARSC] = (save.SCALE * save.AU);
        }
    }

    //
    // Left justify, convert to upper case and form a "plural" version
    // of UNIN
    //
    spicelib::LJUST(UNIN, &mut save.CANDS);
    spicelib::UCASE(&save.CANDS.to_vec(), &mut save.CANDS, ctx);

    fstr::assign(&mut save.CANDP, &save.CANDS);
    spicelib::SUFFIX(b"S", 0, &mut save.CANDP);

    //
    // Look for the "singular" version first.
    //
    save.J = spicelib::BSRCHC(&save.CANDS, save.COUNT, save.UNITS.as_arg());

    //
    // If we didn't have any luck with the singular version,
    // look for the plural form.
    //
    if (save.J == 0) {
        save.J = spicelib::BSRCHC(&save.CANDP, save.COUNT, save.UNITS.as_arg());
    }

    //
    // If we got something, just copy the class and value.
    //
    if (save.J > 0) {
        *KNOWN = true;
        *CLASS = save.UCLASS[save.J];
        *VALUE = save.UVALUE[save.J];
    } else {
        //
        // We don't have a unit.  Get ready to return...
        //
        *KNOWN = false;
        *CLASS = -1;
        *VALUE = 0.0;

        //
        // ... but before we do, see if we've got a number.
        //
        if spicelib::BENUM(&save.CANDS) {
            spicelib::NPARSD(&save.CANDS, VALUE, &mut save.ERROR, &mut save.PTR, ctx);

            if fstr::eq(&save.ERROR, b" ") {
                *KNOWN = true;
                *CLASS = NUMBER;
            }
        }
    }

    //
    // Since the user can potentially enter a bad value for the AU
    // via the kernel pool, we will signal an error.  However we
    // wait until this point so that routines that need to have
    // an AU value in order to continue functioning,
    //
    if (f64::abs((save.AU - DE200V)) > 10.0) {
        spicelib::CHKIN(b"FNDUCV", ctx)?;
        spicelib::SETMSG(b"The value of the astronomical unit extracted from the kernel pool varies from the well trusted value used in DE200 (149,597,870.660 km) by more than 10 km. The value in DE200 is believed to be good to 60 meters or so.  The value in the kernel pool was #. ", ctx);
        spicelib::ERRDP(b"#", save.AU, ctx);
        spicelib::SIGERR(b"SPICE(BADAUVALUE)", ctx)?;
        spicelib::CHKOUT(b"FNDUCV", ctx)?;
        //
        // Reset the value of the AU back to the DE200 value so that
        // the next time we hit this without doing a kernel pool read
        // we will not get this error message again.
        //
        save.AU = DE200V;
        return Ok(());
    }

    Ok(())
}
