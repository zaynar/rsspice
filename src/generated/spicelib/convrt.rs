//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const SECPJY: f64 = (86400.0 * 365.25);
const SECPTY: f64 = (86400.0 * 365.2421988);
const LTYEAR: f64 = ((86400.0 * 365.25) * 299792458.0);
const DPM: f64 = (1.0 / 60.0);
const DPS: f64 = (1.0 / 3600.0);
const DPSA: f64 = (15.0 / 3600.0);
const AU: f64 = 149597870613.68887;
const SCALE: f64 = 206264.80624790437;
const PARSEC: f64 = (AU * SCALE);
const ANG: i32 = 0;
const DIST: i32 = (ANG + 7);
const TME: i32 = (DIST + 17);
const TOTAL: i32 = (TME + 7);

struct SaveVars {
    UNITS: ActualCharArray,
    CNVRTN: StackArray<f64, 31>,
    TYPE: ActualCharArray,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut UNITS = ActualCharArray::new(16, 1..=TOTAL);
        let mut CNVRTN = StackArray::<f64, 31>::new(1..=TOTAL);
        let mut TYPE = ActualCharArray::new(8, 1..=TOTAL);
        let mut FIRST: bool = false;

        FIRST = true;
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"RADIANS")].into_iter();
            fstr::assign(UNITS.get_mut((ANG + 1)), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"DEGREES")].into_iter();
            fstr::assign(UNITS.get_mut((ANG + 2)), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"ARCMINUTES")].into_iter();
            fstr::assign(UNITS.get_mut((ANG + 3)), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"ARCSECONDS")].into_iter();
            fstr::assign(UNITS.get_mut((ANG + 4)), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"HOURANGLE")].into_iter();
            fstr::assign(UNITS.get_mut((ANG + 5)), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"MINUTEANGLE")].into_iter();
            fstr::assign(UNITS.get_mut((ANG + 6)), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"SECONDANGLE")].into_iter();
            fstr::assign(UNITS.get_mut((ANG + 7)), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(1.0)].into_iter();
            CNVRTN[(ANG + 2)] = clist.next().unwrap().into_f64();

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(DPM)].into_iter();
            CNVRTN[(ANG + 3)] = clist.next().unwrap().into_f64();

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(DPS)].into_iter();
            CNVRTN[(ANG + 4)] = clist.next().unwrap().into_f64();

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(15.0)].into_iter();
            CNVRTN[(ANG + 5)] = clist.next().unwrap().into_f64();

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(0.25)].into_iter();
            CNVRTN[(ANG + 6)] = clist.next().unwrap().into_f64();

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(DPSA)].into_iter();
            CNVRTN[(ANG + 7)] = clist.next().unwrap().into_f64();

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"ANGLE")].into_iter();
            fstr::assign(TYPE.get_mut((ANG + 1)), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"ANGLE")].into_iter();
            fstr::assign(TYPE.get_mut((ANG + 2)), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"ANGLE")].into_iter();
            fstr::assign(TYPE.get_mut((ANG + 3)), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"ANGLE")].into_iter();
            fstr::assign(TYPE.get_mut((ANG + 4)), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"ANGLE")].into_iter();
            fstr::assign(TYPE.get_mut((ANG + 5)), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"ANGLE")].into_iter();
            fstr::assign(TYPE.get_mut((ANG + 6)), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"ANGLE")].into_iter();
            fstr::assign(TYPE.get_mut((ANG + 7)), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"METERS")].into_iter();
            fstr::assign(UNITS.get_mut((DIST + 1)), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"KM")].into_iter();
            fstr::assign(UNITS.get_mut((DIST + 2)), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"CM")].into_iter();
            fstr::assign(UNITS.get_mut((DIST + 3)), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"MM")].into_iter();
            fstr::assign(UNITS.get_mut((DIST + 4)), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"LIGHTSECS")].into_iter();
            fstr::assign(UNITS.get_mut((DIST + 5)), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"AU")].into_iter();
            fstr::assign(UNITS.get_mut((DIST + 6)), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"M")].into_iter();
            fstr::assign(UNITS.get_mut((DIST + 7)), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"KILOMETERS")].into_iter();
            fstr::assign(UNITS.get_mut((DIST + 8)), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"CENTIMETERS")].into_iter();
            fstr::assign(UNITS.get_mut((DIST + 9)), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"MILLIMETERS")].into_iter();
            fstr::assign(UNITS.get_mut((DIST + 10)), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(1.0)].into_iter();
            CNVRTN[(DIST + 1)] = clist.next().unwrap().into_f64();

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(1000.0)].into_iter();
            CNVRTN[(DIST + 2)] = clist.next().unwrap().into_f64();

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(0.01)].into_iter();
            CNVRTN[(DIST + 3)] = clist.next().unwrap().into_f64();

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(0.001)].into_iter();
            CNVRTN[(DIST + 4)] = clist.next().unwrap().into_f64();

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(299792458.0)].into_iter();
            CNVRTN[(DIST + 5)] = clist.next().unwrap().into_f64();

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(AU)].into_iter();
            CNVRTN[(DIST + 6)] = clist.next().unwrap().into_f64();

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(1.0)].into_iter();
            CNVRTN[(DIST + 7)] = clist.next().unwrap().into_f64();

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(1000.0)].into_iter();
            CNVRTN[(DIST + 8)] = clist.next().unwrap().into_f64();

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(0.01)].into_iter();
            CNVRTN[(DIST + 9)] = clist.next().unwrap().into_f64();

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(0.001)].into_iter();
            CNVRTN[(DIST + 10)] = clist.next().unwrap().into_f64();

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"DISTANCE")].into_iter();
            fstr::assign(TYPE.get_mut((DIST + 1)), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"DISTANCE")].into_iter();
            fstr::assign(TYPE.get_mut((DIST + 2)), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"DISTANCE")].into_iter();
            fstr::assign(TYPE.get_mut((DIST + 3)), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"DISTANCE")].into_iter();
            fstr::assign(TYPE.get_mut((DIST + 4)), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"DISTANCE")].into_iter();
            fstr::assign(TYPE.get_mut((DIST + 5)), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"DISTANCE")].into_iter();
            fstr::assign(TYPE.get_mut((DIST + 6)), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"DISTANCE")].into_iter();
            fstr::assign(TYPE.get_mut((DIST + 7)), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"DISTANCE")].into_iter();
            fstr::assign(TYPE.get_mut((DIST + 8)), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"DISTANCE")].into_iter();
            fstr::assign(TYPE.get_mut((DIST + 9)), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"DISTANCE")].into_iter();
            fstr::assign(TYPE.get_mut((DIST + 10)), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"FEET")].into_iter();
            fstr::assign(UNITS.get_mut((DIST + 11)), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"INCHES")].into_iter();
            fstr::assign(UNITS.get_mut((DIST + 12)), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"STATUTE_MILES")].into_iter();
            fstr::assign(UNITS.get_mut((DIST + 13)), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"NAUTICAL_MILES")].into_iter();
            fstr::assign(UNITS.get_mut((DIST + 14)), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"YARDS")].into_iter();
            fstr::assign(UNITS.get_mut((DIST + 15)), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(0.3048)].into_iter();
            CNVRTN[(DIST + 11)] = clist.next().unwrap().into_f64();

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(0.0254)].into_iter();
            CNVRTN[(DIST + 12)] = clist.next().unwrap().into_f64();

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(1609.344)].into_iter();
            CNVRTN[(DIST + 13)] = clist.next().unwrap().into_f64();

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(1852.0)].into_iter();
            CNVRTN[(DIST + 14)] = clist.next().unwrap().into_f64();

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(0.9144)].into_iter();
            CNVRTN[(DIST + 15)] = clist.next().unwrap().into_f64();

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"DISTANCE")].into_iter();
            fstr::assign(TYPE.get_mut((DIST + 11)), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"DISTANCE")].into_iter();
            fstr::assign(TYPE.get_mut((DIST + 12)), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"DISTANCE")].into_iter();
            fstr::assign(TYPE.get_mut((DIST + 13)), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"DISTANCE")].into_iter();
            fstr::assign(TYPE.get_mut((DIST + 14)), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"DISTANCE")].into_iter();
            fstr::assign(TYPE.get_mut((DIST + 15)), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"LIGHTYEARS")].into_iter();
            fstr::assign(UNITS.get_mut((DIST + 16)), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"PARSECS")].into_iter();
            fstr::assign(UNITS.get_mut((DIST + 17)), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(LTYEAR)].into_iter();
            CNVRTN[(DIST + 16)] = clist.next().unwrap().into_f64();

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(PARSEC)].into_iter();
            CNVRTN[(DIST + 17)] = clist.next().unwrap().into_f64();

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"DISTANCE")].into_iter();
            fstr::assign(TYPE.get_mut((DIST + 16)), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"DISTANCE")].into_iter();
            fstr::assign(TYPE.get_mut((DIST + 17)), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"SECONDS")].into_iter();
            fstr::assign(UNITS.get_mut((TME + 1)), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"MINUTES")].into_iter();
            fstr::assign(UNITS.get_mut((TME + 2)), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"HOURS")].into_iter();
            fstr::assign(UNITS.get_mut((TME + 3)), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"DAYS")].into_iter();
            fstr::assign(UNITS.get_mut((TME + 4)), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"JULIAN_YEARS")].into_iter();
            fstr::assign(UNITS.get_mut((TME + 5)), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"TROPICAL_YEARS")].into_iter();
            fstr::assign(UNITS.get_mut((TME + 6)), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"YEARS")].into_iter();
            fstr::assign(UNITS.get_mut((TME + 7)), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(1.0)].into_iter();
            CNVRTN[(TME + 1)] = clist.next().unwrap().into_f64();

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(60.0)].into_iter();
            CNVRTN[(TME + 2)] = clist.next().unwrap().into_f64();

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(3600.0)].into_iter();
            CNVRTN[(TME + 3)] = clist.next().unwrap().into_f64();

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(86400.0)].into_iter();
            CNVRTN[(TME + 4)] = clist.next().unwrap().into_f64();

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(SECPJY)].into_iter();
            CNVRTN[(TME + 5)] = clist.next().unwrap().into_f64();

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(SECPTY)].into_iter();
            CNVRTN[(TME + 6)] = clist.next().unwrap().into_f64();

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(SECPJY)].into_iter();
            CNVRTN[(TME + 7)] = clist.next().unwrap().into_f64();

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"TIME")].into_iter();
            fstr::assign(TYPE.get_mut((TME + 1)), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"TIME")].into_iter();
            fstr::assign(TYPE.get_mut((TME + 2)), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"TIME")].into_iter();
            fstr::assign(TYPE.get_mut((TME + 3)), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"TIME")].into_iter();
            fstr::assign(TYPE.get_mut((TME + 4)), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"TIME")].into_iter();
            fstr::assign(TYPE.get_mut((TME + 5)), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"TIME")].into_iter();
            fstr::assign(TYPE.get_mut((TME + 6)), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::C(b"TIME")].into_iter();
            fstr::assign(TYPE.get_mut((TME + 7)), clist.next().unwrap().into_str());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            UNITS,
            CNVRTN,
            TYPE,
            FIRST,
        }
    }
}

/// Convert Units
///
/// Take a measurement X, the units associated with
/// X, and units to which X should be converted; return Y ---
/// the value of the measurement in the output units.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  -------------------------------------------------
///  X          I   Number representing a measurement in some units.
///  IN         I   The units in which X is measured.
///  OUT        I   Desired units for the measurement.
///  Y          O   The measurement in the desired units.
/// ```
///
/// # Detailed Input
///
/// ```text
///  X        is a number representing a measurement in the units
///           specified by IN.
///
///  IN       is the identifier of the units associated with the
///           measurement X. Acceptable units are:
///
///              Angles:                 'RADIANS'
///                                      'DEGREES'
///                                      'ARCMINUTES'
///                                      'ARCSECONDS'
///                                      'HOURANGLE'
///                                      'MINUTEANGLE'
///                                      'SECONDANGLE'
///
///              Metric Distances:       'M'
///                                      'METERS'
///                                      'KM'
///                                      'KILOMETERS'
///                                      'CM'
///                                      'CENTIMETERS'
///                                      'MM'
///                                      'MILLIMETERS'
///
///              English Distances:      'FEET'
///                                      'INCHES'
///                                      'YARDS'
///                                      'STATUTE_MILES'
///                                      'NAUTICAL_MILES'
///
///              Astrometric Distances:  'AU'
///                                      'PARSECS'
///                                      'LIGHTSECS'
///                                      'LIGHTYEARS' julian lightyears
///
///              Time:                   'SECONDS'
///                                      'MINUTES'
///                                      'HOURS'
///                                      'DAYS'
///                                      'JULIAN_YEARS'
///                                      'TROPICAL_YEARS'
///                                      'YEARS' (same as julian years)
///
///  OUT      is the identifier of the units desired for the
///           measurement X. See the description of IN.
/// ```
///
/// # Detailed Output
///
/// ```text
///  Y        is the input measurement converted to the desired
///           units.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the input units, output units, or both input and
///      output units are not recognized, the error
///      SPICE(UNITSNOTREC) is signaled.
///
///  2)  If the units being converted between are incompatible, the
///      error SPICE(INCOMPATIBLEUNITS) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine converts a measurement X given in units specified by
///  IN to the equivalent value Y in units specified by OUT.
///
///  If a unit is not recognized, an error message is produced that
///  indicates which one was not recognized.
///
///  If input and output units are incompatible (for example ANGLE
///  and DISTANCE units) and error message will be produced stating
///  the requested units and associated types.
/// ```
///
/// # Examples
///
/// ```text
///  To convert 1 meter to statute miles and feet you could
///
///     CALL CONVRT ( 1.0D0, 'METERS',        'STATUTE_MILES', MILES )
///     CALL CONVRT ( MILES, 'STATUTE_MILES', 'FEET',          FEET  )
///
///  or
///
///     CALL CONVRT ( 1.0D0, 'METERS', 'STATUTE_MILES', MILES )
///     CALL CONVRT ( 1.0D0, 'METERS', 'FEET',          FEET  )
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  This routine does not do any checking for overflow. The caller
///      is required to make sure that the units used for the
///      measurement are such that no floating point overflow will
///      occur when the conversion is performed.
///
///  2)  Some of the units are not "defined" quantities. In such a case
///      a best estimate is provided as of the date of the current
///      version of this routine. Those estimated quantities are:
///
///         AU               The astronomical unit. The value was taken
///                          from the JPL ephemeris DE125. This value
///                          is an approximation and should not be used
///                          for high-accuracy work. It agrees with the
///                          value used in the JPL planetary ephemeris
///                          DE430 (149597870.700 km) at the 100m
///                          level.
///
///         TROPICAL_YEARS   The tropical year is the time from equinox
///                          to equinox. This varies slightly with
///                          time.
///
///         PARSECS          The parsec is the distance to an object
///                          whose parallax angle is one arcsecond. Its
///                          value is dependent upon the value of the
///                          astronomical unit.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  C.A. Curzon        (JPL)
///  J. Diaz del Rio    (ODC Space)
///  W.M. Owen          (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.1.0, 06-JUL-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Removed
///         unnecessary $Revisions section.
///
///         Corrected a typo in $Restrictions section.
///
/// -    SPICELIB Version 2.0.0, 12-MAY-2015 (NJB)
///
///         Added support for full names of metric distance units. Added
///         support for the abbreviation 'M' indicating meters.
///
/// -    SPICELIB Version 1.0.2, 01-JUL-2014 (NJB)
///
///         Updated the description of the AU in the $Restrictions
///         section.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (CAC) (WMO) (WLT) (IMU)
/// ```
pub fn convrt(
    ctx: &mut SpiceContext,
    x: f64,
    in_: &str,
    out: &str,
    y: &mut f64,
) -> crate::Result<()> {
    CONVRT(x, in_.as_bytes(), out.as_bytes(), y, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure CONVRT ( Convert Units )
pub fn CONVRT(
    X: f64,
    IN: &[u8],
    OUT: &[u8],
    Y: &mut f64,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut INU = [b' '; 16];
    let mut OUTU = [b' '; 16];
    let mut I: i32 = 0;
    let mut J: i32 = 0;
    let mut TEMP: f64 = 0.0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // 1.0d0 divided by the sin of 1 arc second
    //

    //
    // Angular Conversions:
    //
    //             (1)  Degrees/Radians
    //             (2)  Degrees/Degrees
    //             (3)  Degrees/ARCMINUTES
    //             (4)  Degrees/ARCSECONDS
    //
    //             ()   Degrees/HOURANGLE
    //             ()   Degrees/MINUTEANGLE
    //             ()   Degrees/SECONDANGLE
    //
    //
    // DATA CNVRTN (ANG + 1)  /      DPR()   /
    //
    // This value will be loaded using the SPICELIB function DPR()
    // on the first execution of this routine.
    //

    //
    // Distance Conversions ( 8 through 17 )
    //
    //             (+  1) Meters/Meter
    //             (+  2) Meters/Km
    //             (+  3) Meters/Cm
    //             (+  4) Meters/mm
    //             (+  5) Meters/Lightsecs
    //             (+  6) Meters/AU
    //             (+  7) Meters/Meter
    //             (+  8) Meters/Km
    //             (+  9) Meters/cm
    //             (+ 10) Meters/mm
    //

    //
    // Distance Conversions
    //
    //             (+ 11) Meters/Foot
    //             (+ 12) Meters/inch
    //             (+ 13) Meters/Statute Mile
    //             (+ 14) Meters/Nautical Mile
    //             (+ 15) Meters/Yard
    //

    //
    // Distance Conversions
    //
    //             (+ 16) Meters/LightYear
    //             (+ 17) Meters/Parsec
    //

    //
    // Time Conversions
    //
    //             (+ 1 ) seconds / second
    //             (+ 2 ) seconds / minute
    //             (+ 3 ) seconds / hour
    //             (+ 4 ) seconds / day
    //             (+ 5 ) Seconds / Julian year
    //             (+ 6 ) Seconds / Tropical year
    //             (+ 7 ) Seconds / year          --- same as Julian year
    //

    //
    // Set up the error processing.
    //
    if RETURN(ctx) {
        return Ok(());
    }
    CHKIN(b"CONVRT", ctx)?;

    if save.FIRST {
        save.CNVRTN[(ANG + 1)] = DPR(ctx);
        save.FIRST = false;
    }

    UCASE(IN, &mut INU, ctx);
    UCASE(OUT, &mut OUTU, ctx);

    I = ISRCHC(&INU, TOTAL, save.UNITS.as_arg());
    J = ISRCHC(&OUTU, TOTAL, save.UNITS.as_arg());

    if ((I == 0) || (J == 0)) {
        if ((I == 0) && (J == 0)) {
            SETMSG(
                &fstr::concat(
                    &fstr::concat(
                        &fstr::concat(
                            &fstr::concat(b"CONVRT: Neither the input units ", &INU),
                            b"nor the output units ",
                        ),
                        &OUTU,
                    ),
                    b"were recognized.",
                ),
                ctx,
            );

            SIGERR(b"SPICE(UNITSNOTREC)", ctx)?;

            CHKOUT(b"CONVRT", ctx)?;
            return Ok(());
        } else if (I == 0) {
            SETMSG(
                &fstr::concat(
                    &fstr::concat(b"CONVRT: Input units ", &INU),
                    b" were not recognized",
                ),
                ctx,
            );

            SIGERR(b"SPICE(UNITSNOTREC)", ctx)?;

            CHKOUT(b"CONVRT", ctx)?;
            return Ok(());
        } else if (J == 0) {
            SETMSG(
                &fstr::concat(
                    &fstr::concat(b"CONVRT: Output units ", &OUTU),
                    b" were not recognized",
                ),
                ctx,
            );

            SIGERR(b"SPICE(UNITSNOTREC)", ctx)?;

            CHKOUT(b"CONVRT", ctx)?;
            return Ok(());
        }
    }

    if fstr::ne(save.TYPE.get(I), save.TYPE.get(J)) {
        SETMSG(&fstr::concat(&fstr::concat(&fstr::concat(&fstr::concat(&fstr::concat(&fstr::concat(&fstr::concat(&fstr::concat(b"CONVRT: Incompatible units. You are attempting to convert ", &INU), b"type: "), save.TYPE.get(I)), b" to "), &OUTU), b"type: "), save.TYPE.get(J)), b"."), ctx);

        SIGERR(b"SPICE(INCOMPATIBLEUNITS)", ctx)?;

        CHKOUT(b"CONVRT", ctx)?;
        return Ok(());
    }

    TEMP = (X * save.CNVRTN[I]);
    *Y = (TEMP / save.CNVRTN[J]);

    CHKOUT(b"CONVRT", ctx)?;
    Ok(())
}
