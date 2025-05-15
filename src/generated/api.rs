//
// GENERATED FILE
//

use super::spicelib::*;
use crate::{Cell, CharCell, CharVec, Result, SpiceContext, raw};
use f2rust_std::Context;

fn blank(len: i32) -> String {
    " ".repeat(len as usize)
}

fn trim(s: String) -> String {
    s.trim_end_matches(' ').to_owned()
}

const LBCELL: i32 = -5;

impl SpiceContext<'_> {
    /// All entries true?
    ///
    /// Determine if all the entries in an array of logicals are .TRUE.
    ///
    /// See [`alltru`](raw::alltru) for full documentation.
    pub fn alltru(&self, logcls: &[bool], n: i32) -> bool {
        raw::alltru(logcls, n)
    }

    /// AN or A ?
    ///
    /// Return the correct article "a" or "an" used to modify a word
    /// and return it capitalized, lower case, or upper case.
    ///
    /// Returns `ana`.
    ///
    /// See [`ana`](raw::ana) for full documentation.
    pub fn ana(&mut self, word: &str, case: &str) -> String {
        let mut ana = blank(2);
        raw::ana(self, word, case, &mut ana);
        trim(ana)
    }

    /// Append an item to a character cell
    ///
    /// Append an item to a character cell.
    ///
    /// See [`appndc`](raw::appndc) for full documentation.
    pub fn appndc(&mut self, item: &str, cell: &mut CharCell) -> Result<()> {
        raw::appndc(self, item, cell.as_arg_mut())?;
        Ok(())
    }

    /// Append an item to a double precision cell
    ///
    /// Append an item to a double precision cell.
    ///
    /// See [`appndd`](raw::appndd) for full documentation.
    pub fn appndd(&mut self, item: f64, cell: &mut Cell<f64>) -> Result<()> {
        raw::appndd(self, item, cell.as_raw_mut_slice())?;
        Ok(())
    }

    /// Append an item to an integer cell
    ///
    /// Append an item to an integer cell.
    ///
    /// See [`appndi`](raw::appndi) for full documentation.
    pub fn appndi(&mut self, item: i32, cell: &mut Cell<i32>) -> Result<()> {
        raw::appndi(self, item, cell.as_raw_mut_slice())?;
        Ok(())
    }

    /// Approximate equality
    ///
    /// Return .TRUE. if two double precision numbers are equal to
    /// within some tolerance.
    ///
    /// See [`approx`](raw::approx) for full documentation.
    pub fn approx(&self, x: f64, y: f64, tol: f64) -> bool {
        raw::approx(x, y, tol)
    }

    /// Axis and angle to rotation
    ///
    /// Construct a rotation matrix that rotates vectors by a specified
    /// angle about a specified axis.
    ///
    /// Returns `r`.
    ///
    /// See [`axisar`](raw::axisar) for full documentation.
    pub fn axisar(&self, axis: &[f64; 3], angle: f64) -> [[f64; 3]; 3] {
        let mut r: [[f64; 3]; 3] = Default::default();
        raw::axisar(axis, angle, &mut r);
        r
    }

    /// AZ/EL, constant position observer state
    ///
    /// Return the azimuth/elevation coordinates of a specified target
    /// relative to an "observer," where the observer has constant
    /// position in a specified reference frame. The observer's position
    /// is provided by the calling program rather than by loaded SPK
    /// files.
    ///
    /// Returns `(azlsta, lt)`.
    ///
    /// See [`azlcpo`](raw::azlcpo) for full documentation.
    pub fn azlcpo(
        &mut self,
        method: &str,
        target: &str,
        et: f64,
        abcorr: &str,
        azccw: bool,
        elplsz: bool,
        obspos: &[f64; 3],
        obsctr: &str,
        obsref: &str,
    ) -> Result<([f64; 6], f64)> {
        let mut azlsta: [f64; 6] = Default::default();
        let mut lt: f64 = Default::default();
        raw::azlcpo(
            self,
            method,
            target,
            et,
            abcorr,
            azccw,
            elplsz,
            obspos,
            obsctr,
            obsref,
            &mut azlsta,
            &mut lt,
        )?;
        Ok((azlsta, lt))
    }

    /// AZ/EL to rectangular coordinates
    ///
    /// Convert from range, azimuth and elevation of a point to
    /// rectangular coordinates.
    ///
    /// Returns `rectan`.
    ///
    /// See [`azlrec`](raw::azlrec) for full documentation.
    pub fn azlrec(&self, range: f64, az: f64, el: f64, azccw: bool, elplsz: bool) -> [f64; 3] {
        let mut rectan: [f64; 3] = Default::default();
        raw::azlrec(range, az, el, azccw, elplsz, &mut rectan);
        rectan
    }

    /// Besselian Date 1900.0
    ///
    /// Return the Julian Date corresponding to Besselian Date 1900.0.
    ///
    /// See [`b1900`](raw::b1900) for full documentation.
    pub fn b1900(&self) -> f64 {
        raw::b1900()
    }

    /// Besselian Date 1950.0
    ///
    /// Return the Julian Date corresponding to Besselian Date 1950.0.
    ///
    /// See [`b1950`](raw::b1950) for full documentation.
    pub fn b1950(&self) -> f64 {
        raw::b1950()
    }

    /// Bad Kernel Pool Variable
    ///
    /// Determine if a kernel pool variable is present and if so
    /// that it has the correct size and type.
    ///
    /// See [`badkpv`](raw::badkpv) for full documentation.
    pub fn badkpv(
        &mut self,
        caller: &str,
        name: &str,
        comp: &str,
        size: i32,
        divby: i32,
        type_: &str,
    ) -> Result<bool> {
        raw::badkpv(self, caller, name, comp, size, divby, type_)
    }

    /// Be a decimal number?
    ///
    /// Determine whether a string represents a decimal number.
    ///
    /// See [`bedec`](raw::bedec) for full documentation.
    pub fn bedec(&self, string: &str) -> bool {
        raw::bedec(string)
    }

    /// Be an Integer?
    ///
    /// Determine whether a string represents an integer.
    ///
    /// See [`beint`](raw::beint) for full documentation.
    pub fn beint(&self, string: &str) -> bool {
        raw::beint(string)
    }

    /// Be a number?
    ///
    /// Determine whether a string represents a number.
    ///
    /// See [`benum`](raw::benum) for full documentation.
    pub fn benum(&self, string: &str) -> bool {
        raw::benum(string)
    }

    /// Be an unsigned integer?
    ///
    /// Determine whether a string represents an unsigned integer.
    ///
    /// See [`beuns`](raw::beuns) for full documentation.
    pub fn beuns(&self, string: &str) -> bool {
        raw::beuns(string)
    }

    /// Built-in frame IDs
    ///
    /// Return a set containing the frame IDs of all built-in frames of a
    /// specified class.
    ///
    /// See [`bltfrm`](raw::bltfrm) for full documentation.
    pub fn bltfrm(&mut self, frmcls: i32, idset: &mut Cell<i32>) -> Result<()> {
        raw::bltfrm(self, frmcls, idset.as_raw_mut_slice())?;
        Ok(())
    }

    /// Body ID code to name translation
    ///
    /// Translate the SPICE integer code of a body into a common name
    /// for that body.
    ///
    /// Returns `name`.
    ///
    /// See [`bodc2n`](raw::bodc2n) for full documentation.
    pub fn bodc2n(&mut self, code: i32) -> Result<Option<String>> {
        let mut name = blank(inc::zzbodtrn::MAXL);
        let mut found: bool = Default::default();
        raw::bodc2n(self, code, &mut name, &mut found)?;
        Ok(if found { Some(trim(name)) } else { None })
    }

    /// Body ID code to string translation
    ///
    /// Translate a body ID code to either the corresponding name
    /// or if no name to ID code mapping exists, the string
    /// representation of the body ID value.
    ///
    /// Returns `name`.
    ///
    /// See [`bodc2s`](raw::bodc2s) for full documentation.
    pub fn bodc2s(&mut self, code: i32) -> Result<String> {
        let mut name = blank(inc::zzbodtrn::MAXL);
        raw::bodc2s(self, code, &mut name)?;
        Ok(trim(name))
    }

    /// Body name/ID code definition
    ///
    /// Define a body name/ID code pair for later translation via
    /// BODN2C or BODC2N.
    ///
    /// See [`boddef`](raw::boddef) for full documentation.
    pub fn boddef(&mut self, name: &str, code: i32) -> Result<()> {
        raw::boddef(self, name, code)?;
        Ok(())
    }

    /// Return Euler angles for a body
    ///
    /// Return the Euler angles needed to compute the transformation from
    /// inertial to body-fixed coordinates for any body in the kernel
    /// pool.
    ///
    /// Returns `(ra, dec, w, lambda)`.
    ///
    /// See [`bodeul`](raw::bodeul) for full documentation.
    pub fn bodeul(&mut self, body: i32, et: f64) -> Result<(f64, f64, f64, f64)> {
        let mut ra: f64 = Default::default();
        let mut dec: f64 = Default::default();
        let mut w: f64 = Default::default();
        let mut lambda: f64 = Default::default();
        raw::bodeul(self, body, et, &mut ra, &mut dec, &mut w, &mut lambda)?;
        Ok((ra, dec, w, lambda))
    }

    /// Find values from the kernel pool
    ///
    /// Determine whether values exist for some item for any body
    /// in the kernel pool.
    ///
    /// See [`bodfnd`](raw::bodfnd) for full documentation.
    pub fn bodfnd(&mut self, body: i32, item: &str) -> Result<bool> {
        raw::bodfnd(self, body, item)
    }

    /// Return transformation matrix for a body
    ///
    /// Return the J2000 to body Equator and Prime Meridian coordinate
    /// transformation matrix for a specified body.
    ///
    /// Returns `tipm`.
    ///
    /// See [`bodmat`](raw::bodmat) for full documentation.
    pub fn bodmat(&mut self, body: i32, et: f64) -> Result<[[f64; 3]; 3]> {
        let mut tipm: [[f64; 3]; 3] = Default::default();
        raw::bodmat(self, body, et, &mut tipm)?;
        Ok(tipm)
    }

    /// Body name to ID code translation
    ///
    /// Translate the name of a body or object to the corresponding SPICE
    /// integer ID code.
    ///
    /// Returns `code`.
    ///
    /// See [`bodn2c`](raw::bodn2c) for full documentation.
    pub fn bodn2c(&mut self, name: &str) -> Result<Option<i32>> {
        let mut code: i32 = Default::default();
        let mut found: bool = Default::default();
        raw::bodn2c(self, name, &mut code, &mut found)?;
        Ok(if found { Some(code) } else { None })
    }

    /// Body string to ID code translation
    ///
    /// Translate a string containing a body name or ID code to an
    /// integer code.
    ///
    /// Returns `code`.
    ///
    /// See [`bods2c`](raw::bods2c) for full documentation.
    pub fn bods2c(&mut self, name: &str) -> Result<Option<i32>> {
        let mut code: i32 = Default::default();
        let mut found: bool = Default::default();
        raw::bods2c(self, name, &mut code, &mut found)?;
        Ok(if found { Some(code) } else { None })
    }

    /// Return d.p. values from the kernel pool
    ///
    /// Fetch from the kernel pool the double precision values
    /// of an item associated with a body, where the body is
    /// specified by an integer ID code.
    ///
    /// Returns `(dim, values)`.
    ///
    /// See [`bodvcd`](raw::bodvcd) for full documentation.
    pub fn bodvcd(&mut self, bodyid: i32, item: &str, maxn: i32) -> Result<(i32, Vec<f64>)> {
        let mut dim: i32 = Default::default();
        let mut values: Vec<f64> = vec![Default::default(); maxn.max(0) as usize];
        raw::bodvcd(self, bodyid, item, maxn, &mut dim, &mut values)?;
        Ok((dim, values))
    }

    /// Return d.p. values from the kernel pool
    ///
    /// Fetch from the kernel pool the double precision values
    /// of an item associated with a body.
    ///
    /// Returns `(dim, values)`.
    ///
    /// See [`bodvrd`](raw::bodvrd) for full documentation.
    pub fn bodvrd(&mut self, bodynm: &str, item: &str, maxn: i32) -> Result<(i32, Vec<f64>)> {
        let mut dim: i32 = Default::default();
        let mut values: Vec<f64> = vec![Default::default(); maxn.max(0) as usize];
        raw::bodvrd(self, bodynm, item, maxn, &mut dim, &mut values)?;
        Ok((dim, values))
    }

    /// Bracket a d.p. value within an interval
    ///
    /// Bracket a double precision number. That is, given a number and an
    /// acceptable interval, make sure that the number is contained in the
    /// interval. (If the number is already in the interval, leave it
    /// alone. If not, set it to the nearest endpoint of the interval.)
    ///
    /// See [`brcktd`](raw::brcktd) for full documentation.
    pub fn brcktd(&self, number: f64, end1: f64, end2: f64) -> f64 {
        raw::brcktd(number, end1, end2)
    }

    /// Bracket an integer value within an interval
    ///
    /// Bracket an integer number. That is, given a number and an
    /// acceptable interval, make sure that the number is contained in the
    /// interval. (If the number is already in the interval, leave it
    /// alone. If not, set it to the nearest endpoint of the interval.)
    ///
    /// See [`brckti`](raw::brckti) for full documentation.
    pub fn brckti(&self, number: i32, end1: i32, end2: i32) -> i32 {
        raw::brckti(number, end1, end2)
    }

    /// Binary search with order vector, character
    ///
    /// Do a binary search for a given value within an array of character
    /// strings, accompanied by an order vector. Return the index of the
    /// matching array entry, or zero if the key value is not found.
    ///
    /// See [`bschoc`](raw::bschoc) for full documentation.
    pub fn bschoc(&self, value: &str, ndim: i32, array: &CharVec, order: &[i32]) -> i32 {
        raw::bschoc(value, ndim, array.as_arg(), order)
    }

    /// Binary search with order vector, integer
    ///
    /// Do a binary search for a given value within an integer array,
    /// accompanied by an order vector. Return the index of the
    /// matching array entry, or zero if the key value is not found.
    ///
    /// See [`bschoi`](raw::bschoi) for full documentation.
    pub fn bschoi(&self, value: i32, ndim: i32, array: &[i32], order: &[i32]) -> i32 {
        raw::bschoi(value, ndim, array, order)
    }

    /// Binary search for a character string
    ///
    /// Do a binary search for a given value within a character array,
    /// assumed to be in nondecreasing order. Return the index of the
    /// matching array entry, or zero if the key value is not found.
    ///
    /// See [`bsrchc`](raw::bsrchc) for full documentation.
    pub fn bsrchc(&self, value: &str, ndim: i32, array: &CharVec) -> i32 {
        raw::bsrchc(value, ndim, array.as_arg())
    }

    /// Binary search for double precision value
    ///
    /// Do a binary search for a given value within a double precision
    /// array, assumed to be in nondecreasing order. Return the index of
    /// the matching array entry, or zero if the key value is not found.
    ///
    /// See [`bsrchd`](raw::bsrchd) for full documentation.
    pub fn bsrchd(&self, value: f64, ndim: i32, array: &[f64]) -> i32 {
        raw::bsrchd(value, ndim, array)
    }

    /// Binary search for an integer value
    ///
    /// Do a binary search for a given value within an integer array,
    /// assumed to be in nondecreasing order. Return the index of the
    /// matching array entry, or zero if the key value is not found.
    ///
    /// See [`bsrchi`](raw::bsrchi) for full documentation.
    pub fn bsrchi(&self, value: i32, ndim: i32, array: &[i32]) -> i32 {
        raw::bsrchi(value, ndim, array)
    }

    /// Exit a program indicating an error status
    ///
    /// Exit an executing program returning a success or failure status
    /// to the operating system, if this capability is supported, or
    /// simply exit the program.
    ///
    /// See [`byebye`](raw::byebye) for full documentation.
    pub fn byebye(&mut self, status: &str) -> Result<()> {
        raw::byebye(self, status)?;
        Ok(())
    }

    /// Cardinality of a character cell
    ///
    /// Return the cardinality (number of elements) of a character cell.
    ///
    /// See [`cardc`](raw::cardc) for full documentation.
    pub fn cardc(&mut self, cell: &CharCell) -> Result<i32> {
        raw::cardc(self, cell.as_arg())
    }

    /// Cardinality of a double precision cell
    ///
    /// Return the cardinality (number of elements) of a double
    /// precision cell.
    ///
    /// See [`cardd`](raw::cardd) for full documentation.
    pub fn cardd(&mut self, cell: &Cell<f64>) -> Result<i32> {
        raw::cardd(self, cell.as_raw_slice())
    }

    /// Cardinality of an integer cell
    ///
    /// Return the cardinality (number of elements) of an integer cell.
    ///
    /// See [`cardi`](raw::cardi) for full documentation.
    pub fn cardi(&mut self, cell: &Cell<i32>) -> Result<i32> {
        raw::cardi(self, cell.as_raw_slice())
    }

    /// Center and generating vectors to ellipse
    ///
    /// Form a SPICE ellipse from a center vector and two generating
    /// vectors.
    ///
    /// Returns `ellips`.
    ///
    /// See [`cgv2el`](raw::cgv2el) for full documentation.
    pub fn cgv2el(
        &mut self,
        center: &[f64; 3],
        vec1: &[f64; 3],
        vec2: &[f64; 3],
    ) -> Result<[f64; 9]> {
        let mut ellips: [f64; 9] = Default::default();
        raw::cgv2el(self, center, vec1, vec2, &mut ellips)?;
        Ok(ellips)
    }

    /// Character set base
    ///
    /// Return the base value used to encode unsigned integer values
    /// in character strings.
    ///
    /// See [`chbase`](raw::chbase) for full documentation.
    pub fn chbase(&self) -> i32 {
        raw::chbase()
    }

    /// Derivatives of a Chebyshev expansion
    ///
    /// Return the value of a polynomial and its first NDERIV
    /// derivatives, evaluated at the input X, using the coefficients of
    /// the Chebyshev expansion of the polynomial.
    ///
    /// Returns `dpdxs`.
    ///
    /// See [`chbder`](raw::chbder) for full documentation.
    pub fn chbder(
        &self,
        cp: &[f64],
        degp: i32,
        x2s: &[f64; 2],
        x: f64,
        nderiv: i32,
        partdp: &mut [[f64; 3]],
    ) -> Vec<f64> {
        let mut dpdxs: Vec<f64> = vec![Default::default(); (nderiv + 1).max(0) as usize];
        raw::chbder(cp, degp, x2s, x, nderiv, partdp, &mut dpdxs);
        dpdxs
    }

    /// Chebyshev expansion integral
    ///
    /// Evaluate an indefinite integral of a Chebyshev expansion at a
    /// specified point X and return the value of the input expansion at
    /// X as well. The constant of integration is selected to make the
    /// integral zero when X equals the abscissa value X2S(1).
    ///
    /// Returns `(p, itgrlp)`.
    ///
    /// See [`chbigr`](raw::chbigr) for full documentation.
    pub fn chbigr(&mut self, degp: i32, cp: &[f64], x2s: &[f64; 2], x: f64) -> Result<(f64, f64)> {
        let mut p: f64 = Default::default();
        let mut itgrlp: f64 = Default::default();
        raw::chbigr(self, degp, cp, x2s, x, &mut p, &mut itgrlp)?;
        Ok((p, itgrlp))
    }

    /// Interpolate a Chebyshev expansion
    ///
    /// Return the value of a polynomial and its derivative, evaluated at
    /// the input X, using the coefficients of the Chebyshev expansion of
    /// the polynomial.
    ///
    /// Returns `(p, dpdx)`.
    ///
    /// See [`chbint`](raw::chbint) for full documentation.
    pub fn chbint(&self, cp: &[f64], degp: i32, x2s: &[f64; 2], x: f64) -> (f64, f64) {
        let mut p: f64 = Default::default();
        let mut dpdx: f64 = Default::default();
        raw::chbint(cp, degp, x2s, x, &mut p, &mut dpdx);
        (p, dpdx)
    }

    /// Value of a Chebyshev polynomial expansion
    ///
    /// Return the value of a polynomial evaluated at the input X using
    /// the coefficients for the Chebyshev expansion of the polynomial.
    ///
    /// Returns `p`.
    ///
    /// See [`chbval`](raw::chbval) for full documentation.
    pub fn chbval(&self, cp: &[f64], degp: i32, x2s: &[f64; 2], x: f64) -> f64 {
        let mut p: f64 = Default::default();
        raw::chbval(cp, degp, x2s, x, &mut p);
        p
    }

    /// Check ID string
    ///
    /// Validate an ID string: check for non-printing characters
    /// or excessive non-blank length.
    ///
    /// See [`chckid`](raw::chckid) for full documentation.
    pub fn chckid(&mut self, class: &str, maxlen: i32, id: &str) -> Result<()> {
        raw::chckid(self, class, maxlen, id)?;
        Ok(())
    }

    /// Inertial reference frame rotation
    ///
    /// Compute the matrix needed to rotate vectors between two
    /// standard inertial reference frames.
    ///
    /// Returns `rotab`.
    ///
    /// See [`irfrot`](raw::irfrot) for full documentation.
    pub fn irfrot(&mut self, refa: i32, refb: i32) -> Result<[[f64; 3]; 3]> {
        let mut rotab: [[f64; 3]; 3] = Default::default();
        raw::irfrot(self, refa, refb, &mut rotab)?;
        Ok(rotab)
    }

    /// Inertial reference frame number
    ///
    /// Return the index of one of the standard inertial reference
    /// frames supported by IRFROT.
    ///
    /// Returns `index`.
    ///
    /// See [`irfnum`](raw::irfnum) for full documentation.
    pub fn irfnum(&mut self, name: &str) -> i32 {
        let mut index: i32 = Default::default();
        raw::irfnum(self, name, &mut index);
        index
    }

    /// Inertial reference frame name
    ///
    /// Return the name of one of the standard inertial reference
    /// frames supported by IRFROT.
    ///
    /// Returns `name`.
    ///
    /// See [`irfnam`](raw::irfnam) for full documentation.
    pub fn irfnam(&mut self, index: i32) -> Result<String> {
        let mut name = blank(26);
        raw::irfnam(self, index, &mut name)?;
        Ok(trim(name))
    }

    /// Inertial reference frame, default
    ///
    /// Specify a standard inertial reference frame as the default
    /// frame for a program.
    ///
    /// See [`irfdef`](raw::irfdef) for full documentation.
    pub fn irfdef(&mut self, index: i32) -> Result<()> {
        raw::irfdef(self, index)?;
        Ok(())
    }

    /// CK, load pointing file
    ///
    /// Load a CK pointing file for use by the CK readers. Return that
    /// file's handle, to be used by other CK routines to refer to the
    /// file.
    ///
    /// Returns `handle`.
    ///
    /// See [`cklpf`](raw::cklpf) for full documentation.
    pub fn cklpf(&mut self, fname: &str) -> Result<i32> {
        let mut handle: i32 = Default::default();
        raw::cklpf(self, fname, &mut handle)?;
        Ok(handle)
    }

    /// CK, Unload pointing file
    ///
    /// Unload a CK pointing file so that it will no longer be searched
    /// by the readers.
    ///
    /// See [`ckupf`](raw::ckupf) for full documentation.
    pub fn ckupf(&mut self, handle: i32) -> Result<()> {
        raw::ckupf(self, handle)?;
        Ok(())
    }

    /// CK, begin search for segment
    ///
    /// Initiate search through loaded files to find segments applicable
    /// to the spacecraft instrument and time specified.
    ///
    /// See [`ckbss`](raw::ckbss) for full documentation.
    pub fn ckbss(&mut self, inst: i32, sclkdp: f64, tol: f64, needav: bool) -> Result<()> {
        raw::ckbss(self, inst, sclkdp, tol, needav)?;
        Ok(())
    }

    /// C-kernel, Select next segment
    ///
    /// Search through loaded files to find a segment matching the
    /// requested instrument, time, and need for angular velocity,
    /// buffering segment descriptors, identifiers, and handles in the
    /// process to minimize file reads.
    ///
    /// Returns `(handle, descr, segid)`.
    ///
    /// See [`cksns`](raw::cksns) for full documentation.
    pub fn cksns(&mut self) -> Result<Option<(i32, Vec<f64>, String)>> {
        let mut handle: i32 = Default::default();
        let mut descr: Vec<f64> = vec![Default::default(); 5 as usize];
        let mut segid = blank(40);
        let mut found: bool = Default::default();
        raw::cksns(self, &mut handle, &mut descr, &mut segid, &mut found)?;
        Ok(if found {
            Some((handle, descr, trim(segid)))
        } else {
            None
        })
    }

    /// CK --- Are there any loaded?
    ///
    /// Determine whether or not any C-kernels are currently loaded.
    ///
    /// See [`ckhave`](raw::ckhave) for full documentation.
    pub fn ckhave(&mut self) -> Option<()> {
        let mut found: bool = Default::default();
        raw::ckhave(self, &mut found);
        if found { Some(()) } else { None }
    }

    /// CK, Close file
    ///
    /// Close an open CK file.
    ///
    /// See [`ckcls`](raw::ckcls) for full documentation.
    pub fn ckcls(&mut self, handle: i32) -> Result<()> {
        raw::ckcls(self, handle)?;
        Ok(())
    }

    /// CK coverage
    ///
    /// Find the coverage window for a specified object in a specified CK
    /// file.
    ///
    /// See [`ckcov`](raw::ckcov) for full documentation.
    pub fn ckcov(
        &mut self,
        ckfnm: &str,
        idcode: i32,
        needav: bool,
        level: &str,
        tol: f64,
        timsys: &str,
        cover: &mut Cell<f64>,
    ) -> Result<()> {
        raw::ckcov(
            self,
            ckfnm,
            idcode,
            needav,
            level,
            tol,
            timsys,
            cover.as_raw_mut_slice(),
        )?;
        Ok(())
    }

    /// CK evaluate pointing record, data type 1
    ///
    /// Evaluate a pointing record returned by CKR01 from a CK data type 1
    /// segment. Return the C-matrix and optionally the angular velocity
    /// vector associated with the time CLKOUT.
    ///
    /// Returns `(cmat, av, clkout)`.
    ///
    /// See [`cke01`](raw::cke01) for full documentation.
    pub fn cke01(
        &mut self,
        needav: bool,
        record: &[f64],
    ) -> Result<([[f64; 3]; 3], [f64; 3], f64)> {
        let mut cmat: [[f64; 3]; 3] = Default::default();
        let mut av: [f64; 3] = Default::default();
        let mut clkout: f64 = Default::default();
        raw::cke01(self, needav, record, &mut cmat, &mut av, &mut clkout)?;
        Ok((cmat, av, clkout))
    }

    /// C-kernel, evaluate pointing record, data type 2
    ///
    /// Evaluate a pointing record returned by CKR02 from a CK data type 2
    /// segment. Return the C-matrix and angular velocity vector
    /// associated with the time CLKOUT.
    ///
    /// Returns `(cmat, av, clkout)`.
    ///
    /// See [`cke02`](raw::cke02) for full documentation.
    pub fn cke02(
        &mut self,
        needav: bool,
        record: &[f64],
    ) -> Result<([[f64; 3]; 3], [f64; 3], f64)> {
        let mut cmat: [[f64; 3]; 3] = Default::default();
        let mut av: [f64; 3] = Default::default();
        let mut clkout: f64 = Default::default();
        raw::cke02(self, needav, record, &mut cmat, &mut av, &mut clkout)?;
        Ok((cmat, av, clkout))
    }

    /// C-kernel, evaluate pointing record, data type 3
    ///
    /// Evaluate a pointing record returned by CKR03 from a CK type 3
    /// segment. Return the C-matrix and angular velocity vector
    /// associated with the time CLKOUT.
    ///
    /// Returns `(cmat, av, clkout)`.
    ///
    /// See [`cke03`](raw::cke03) for full documentation.
    pub fn cke03(
        &mut self,
        needav: bool,
        record: &[f64],
    ) -> Result<([[f64; 3]; 3], [f64; 3], f64)> {
        let mut cmat: [[f64; 3]; 3] = Default::default();
        let mut av: [f64; 3] = Default::default();
        let mut clkout: f64 = Default::default();
        raw::cke03(self, needav, record, &mut cmat, &mut av, &mut clkout)?;
        Ok((cmat, av, clkout))
    }

    /// C-kernel, evaluate pointing record, type 4
    ///
    /// Evaluate a pointing record returned by CKR04 from a CK type 4
    /// segment. Return the C-matrix and angular velocity vector
    /// associated with the time CLKOUT.
    ///
    /// Returns `(cmat, av, clkout)`.
    ///
    /// See [`cke04`](raw::cke04) for full documentation.
    pub fn cke04(&self, needav: bool, record: &[f64]) -> ([[f64; 3]; 3], [f64; 3], f64) {
        let mut cmat: [[f64; 3]; 3] = Default::default();
        let mut av: [f64; 3] = Default::default();
        let mut clkout: f64 = Default::default();
        raw::cke04(needav, record, &mut cmat, &mut av, &mut clkout);
        (cmat, av, clkout)
    }

    /// C-Kernel, evaluate, type 5
    ///
    /// Evaluate a single data record from a type 5 CK segment.
    ///
    /// Returns `(cmat, av, clkout)`.
    ///
    /// See [`cke05`](raw::cke05) for full documentation.
    pub fn cke05(
        &mut self,
        needav: bool,
        record: &mut [f64],
    ) -> Result<([[f64; 3]; 3], [f64; 3], f64)> {
        let mut cmat: [[f64; 3]; 3] = Default::default();
        let mut av: [f64; 3] = Default::default();
        let mut clkout: f64 = Default::default();
        raw::cke05(self, needav, record, &mut cmat, &mut av, &mut clkout)?;
        Ok((cmat, av, clkout))
    }

    /// C-Kernel, evaluate, type 6
    ///
    /// Evaluate a single data record from a type 6 CK segment.
    ///
    /// Returns `(cmat, av, clkout)`.
    ///
    /// See [`cke06`](raw::cke06) for full documentation.
    pub fn cke06(
        &mut self,
        needav: bool,
        record: &mut [f64],
    ) -> Result<([[f64; 3]; 3], [f64; 3], f64)> {
        let mut cmat: [[f64; 3]; 3] = Default::default();
        let mut av: [f64; 3] = Default::default();
        let mut clkout: f64 = Default::default();
        raw::cke06(self, needav, record, &mut cmat, &mut av, &mut clkout)?;
        Ok((cmat, av, clkout))
    }

    /// CK frame, find position rotation
    ///
    /// Find the position rotation matrix from a C-kernel (CK) frame with
    /// the specified frame class ID (CK ID) to the base frame of the
    /// highest priority CK segment containing orientation data for this
    /// CK frame at the time requested.
    ///
    /// Returns `(rotate, ref_)`.
    ///
    /// See [`ckfrot`](raw::ckfrot) for full documentation.
    pub fn ckfrot(&mut self, inst: i32, et: f64) -> Result<Option<([[f64; 3]; 3], i32)>> {
        let mut rotate: [[f64; 3]; 3] = Default::default();
        let mut ref_: i32 = Default::default();
        let mut found: bool = Default::default();
        raw::ckfrot(self, inst, et, &mut rotate, &mut ref_, &mut found)?;
        Ok(if found { Some((rotate, ref_)) } else { None })
    }

    /// CK frame, find state transformation
    ///
    /// Find the state transformation matrix from a C-kernel (CK) frame
    /// with the specified frame class ID (CK ID) to the base frame of
    /// the highest priority CK segment containing orientation and
    /// angular velocity data for this CK frame at the time requested.
    ///
    /// Returns `(xform, ref_)`.
    ///
    /// See [`ckfxfm`](raw::ckfxfm) for full documentation.
    pub fn ckfxfm(&mut self, inst: i32, et: f64) -> Result<Option<([[f64; 6]; 6], i32)>> {
        let mut xform: [[f64; 6]; 6] = Default::default();
        let mut ref_: i32 = Default::default();
        let mut found: bool = Default::default();
        raw::ckfxfm(self, inst, et, &mut xform, &mut ref_, &mut found)?;
        Ok(if found { Some((xform, ref_)) } else { None })
    }

    /// C-kernel, get pointing
    ///
    /// Get pointing (attitude) for a specified spacecraft clock time.
    ///
    /// Returns `(cmat, clkout)`.
    ///
    /// See [`ckgp`](raw::ckgp) for full documentation.
    pub fn ckgp(
        &mut self,
        inst: i32,
        sclkdp: f64,
        tol: f64,
        ref_: &str,
    ) -> Result<Option<([[f64; 3]; 3], f64)>> {
        let mut cmat: [[f64; 3]; 3] = Default::default();
        let mut clkout: f64 = Default::default();
        let mut found: bool = Default::default();
        raw::ckgp(
            self,
            inst,
            sclkdp,
            tol,
            ref_,
            &mut cmat,
            &mut clkout,
            &mut found,
        )?;
        Ok(if found { Some((cmat, clkout)) } else { None })
    }

    /// C-kernel, get pointing and angular velocity
    ///
    /// Get pointing (attitude) and angular velocity for a specified
    /// spacecraft clock time.
    ///
    /// Returns `(cmat, av, clkout)`.
    ///
    /// See [`ckgpav`](raw::ckgpav) for full documentation.
    pub fn ckgpav(
        &mut self,
        inst: i32,
        sclkdp: f64,
        tol: f64,
        ref_: &str,
    ) -> Result<Option<([[f64; 3]; 3], [f64; 3], f64)>> {
        let mut cmat: [[f64; 3]; 3] = Default::default();
        let mut av: [f64; 3] = Default::default();
        let mut clkout: f64 = Default::default();
        let mut found: bool = Default::default();
        raw::ckgpav(
            self,
            inst,
            sclkdp,
            tol,
            ref_,
            &mut cmat,
            &mut av,
            &mut clkout,
            &mut found,
        )?;
        Ok(if found {
            Some((cmat, av, clkout))
        } else {
            None
        })
    }

    /// C-kernel, get record, type 01
    ///
    /// Return a specified pointing instance from a CK type 01 segment.
    /// The segment is identified by a CK file handle and segment
    /// descriptor.
    ///
    /// Returns `record`.
    ///
    /// See [`ckgr01`](raw::ckgr01) for full documentation.
    pub fn ckgr01(&mut self, handle: i32, descr: &[f64], recno: i32) -> Result<Vec<f64>> {
        let mut record: Vec<f64> = vec![Default::default(); 8 as usize];
        raw::ckgr01(self, handle, descr, recno, &mut record)?;
        Ok(record)
    }

    /// C-kernel, get record, type 02
    ///
    /// Return a specified pointing instance from a CK type 02 segment.
    /// The segment is identified by a CK file handle and segment
    /// descriptor.
    ///
    /// Returns `record`.
    ///
    /// See [`ckgr02`](raw::ckgr02) for full documentation.
    pub fn ckgr02(&mut self, handle: i32, descr: &[f64], recno: i32) -> Result<Vec<f64>> {
        let mut record: Vec<f64> = vec![Default::default(); 10 as usize];
        raw::ckgr02(self, handle, descr, recno, &mut record)?;
        Ok(record)
    }

    /// C-kernel, get record, type 03
    ///
    /// Return a specified pointing instance from a CK type 03 segment.
    /// The segment is identified by a CK file handle and segment
    /// descriptor.
    ///
    /// Returns `record`.
    ///
    /// See [`ckgr03`](raw::ckgr03) for full documentation.
    pub fn ckgr03(&mut self, handle: i32, descr: &[f64], recno: i32) -> Result<Vec<f64>> {
        let mut record: Vec<f64> = vec![Default::default(); 8 as usize];
        raw::ckgr03(self, handle, descr, recno, &mut record)?;
        Ok(record)
    }

    /// C-kernel, get record, type 04
    ///
    /// Return a specified pointing record from a CK type 04 segment.
    /// The segment is identified by a CK file handle and segment
    /// descriptor.
    ///
    /// Returns `record`.
    ///
    /// See [`ckgr04`](raw::ckgr04) for full documentation.
    pub fn ckgr04(&mut self, handle: i32, descr: &[f64], recno: i32) -> Result<Vec<f64>> {
        let mut record: Vec<f64> = vec![Default::default(); inc::ckparam::CK4RSZ as usize];
        raw::ckgr04(self, handle, descr, recno, &mut record)?;
        Ok(record)
    }

    /// C-kernel, get record, type 05
    ///
    /// Return a specified pointing instance from a CK type 05 segment.
    /// The segment is identified by a CK file handle and segment
    /// descriptor.
    ///
    /// Returns `record`.
    ///
    /// See [`ckgr05`](raw::ckgr05) for full documentation.
    pub fn ckgr05(&mut self, handle: i32, descr: &[f64], recno: i32) -> Result<Vec<f64>> {
        let mut record: Vec<f64> = vec![Default::default(); 16 as usize];
        raw::ckgr05(self, handle, descr, recno, &mut record)?;
        Ok(record)
    }

    /// C-kernel, get record, type 06
    ///
    /// Return a specified pointing record from a type 6 CK segment, given
    /// the CK file's handle and the segment's descriptor.
    ///
    /// Returns `record`.
    ///
    /// See [`ckgr06`](raw::ckgr06) for full documentation.
    pub fn ckgr06(
        &mut self,
        handle: i32,
        descr: &[f64],
        msno: i32,
        recno: i32,
    ) -> Result<Vec<f64>> {
        let mut record: Vec<f64> = vec![Default::default(); 17 as usize];
        raw::ckgr06(self, handle, descr, msno, recno, &mut record)?;
        Ok(record)
    }

    /// CK ID to associated SCLK
    ///
    /// Return (depending upon the user's request) the ID code of either
    /// the spacecraft or spacecraft clock associated with a C-Kernel ID
    /// code.
    ///
    /// Returns `idcode`.
    ///
    /// See [`ckmeta`](raw::ckmeta) for full documentation.
    pub fn ckmeta(&mut self, ckid: i32, meta: &str) -> Result<i32> {
        let mut idcode: i32 = Default::default();
        raw::ckmeta(self, ckid, meta, &mut idcode)?;
        Ok(idcode)
    }

    /// C-kernel, get mini-segment parameters, type 06
    ///
    /// Return the mini-segment control parameters, mini-segment interval
    /// bounds, and last epoch for a specified mini-segment in a type 6
    /// CK segment.
    ///
    /// Returns `(rate, subtyp, winsiz, nrec, ivlbds, lstepc)`.
    ///
    /// See [`ckmp06`](raw::ckmp06) for full documentation.
    pub fn ckmp06(
        &mut self,
        handle: i32,
        descr: &[f64],
        msno: i32,
    ) -> Result<(f64, i32, i32, i32, [f64; 2], f64)> {
        let mut rate: f64 = Default::default();
        let mut subtyp: i32 = Default::default();
        let mut winsiz: i32 = Default::default();
        let mut nrec: i32 = Default::default();
        let mut ivlbds: [f64; 2] = Default::default();
        let mut lstepc: f64 = Default::default();
        raw::ckmp06(
            self,
            handle,
            descr,
            msno,
            &mut rate,
            &mut subtyp,
            &mut winsiz,
            &mut nrec,
            &mut ivlbds,
            &mut lstepc,
        )?;
        Ok((rate, subtyp, winsiz, nrec, ivlbds, lstepc))
    }

    /// C-kernel, number of mini-segments, type 06
    ///
    /// Return the number of mini-segments in a type 6 CK segment, given
    /// the handle of the type 6 CK file and the segment's descriptor.
    ///
    /// Returns `nmini`.
    ///
    /// See [`cknm06`](raw::cknm06) for full documentation.
    pub fn cknm06(&mut self, handle: i32, descr: &[f64]) -> Result<i32> {
        let mut nmini: i32 = Default::default();
        raw::cknm06(self, handle, descr, &mut nmini)?;
        Ok(nmini)
    }

    /// C-kernel, number of records, type 01
    ///
    /// Return the number of pointing instances in a CK type 01 segment.
    /// The segment is identified by a CK file handle and segment
    /// descriptor.
    ///
    /// Returns `nrec`.
    ///
    /// See [`cknr01`](raw::cknr01) for full documentation.
    pub fn cknr01(&mut self, handle: i32, descr: &[f64]) -> Result<i32> {
        let mut nrec: i32 = Default::default();
        raw::cknr01(self, handle, descr, &mut nrec)?;
        Ok(nrec)
    }

    /// C-kernel, number of records, type 02
    ///
    /// Return the number of pointing records in a CK type 02 segment.
    /// The segment is identified by a CK file handle and segment
    /// descriptor.
    ///
    /// Returns `nrec`.
    ///
    /// See [`cknr02`](raw::cknr02) for full documentation.
    pub fn cknr02(&mut self, handle: i32, descr: &[f64]) -> Result<i32> {
        let mut nrec: i32 = Default::default();
        raw::cknr02(self, handle, descr, &mut nrec)?;
        Ok(nrec)
    }

    /// C-kernel, number of records, type 03
    ///
    /// Return the number of pointing instances in a CK type 03 segment.
    /// The segment is identified by a CK file handle and segment
    /// descriptor.
    ///
    /// Returns `nrec`.
    ///
    /// See [`cknr03`](raw::cknr03) for full documentation.
    pub fn cknr03(&mut self, handle: i32, descr: &[f64]) -> Result<i32> {
        let mut nrec: i32 = Default::default();
        raw::cknr03(self, handle, descr, &mut nrec)?;
        Ok(nrec)
    }

    /// C-kernel, number of records, data type 4
    ///
    /// Return the number of pointing records in a CK type 04 segment.
    /// The segment is identified by a CK file handle and segment
    /// descriptor.
    ///
    /// Returns `nrec`.
    ///
    /// See [`cknr04`](raw::cknr04) for full documentation.
    pub fn cknr04(&mut self, handle: i32, descr: &[f64]) -> Result<i32> {
        let mut nrec: i32 = Default::default();
        raw::cknr04(self, handle, descr, &mut nrec)?;
        Ok(nrec)
    }

    /// C-kernel, number of records, type 05
    ///
    /// Return the number of pointing instances in a CK type 05 segment.
    /// The segment is identified by a CK file handle and segment
    /// descriptor.
    ///
    /// Returns `nrec`.
    ///
    /// See [`cknr05`](raw::cknr05) for full documentation.
    pub fn cknr05(&mut self, handle: i32, descr: &[f64]) -> Result<i32> {
        let mut nrec: i32 = Default::default();
        raw::cknr05(self, handle, descr, &mut nrec)?;
        Ok(nrec)
    }

    /// CK objects
    ///
    /// Find the set of ID codes of all objects in a specified CK file.
    ///
    /// See [`ckobj`](raw::ckobj) for full documentation.
    pub fn ckobj(&mut self, ckfnm: &str, ids: &mut Cell<i32>) -> Result<()> {
        raw::ckobj(self, ckfnm, ids.as_raw_mut_slice())?;
        Ok(())
    }

    /// CK, open new file.
    ///
    /// Open a new CK file, returning the handle of the opened file.
    ///
    /// Returns `handle`.
    ///
    /// See [`ckopn`](raw::ckopn) for full documentation.
    pub fn ckopn(&mut self, fname: &str, ifname: &str, ncomch: i32) -> Result<i32> {
        let mut handle: i32 = Default::default();
        raw::ckopn(self, fname, ifname, ncomch, &mut handle)?;
        Ok(handle)
    }

    /// C-kernel, get pointing from segment
    ///
    /// Evaluate pointing data from a segment for a given time.
    ///
    /// Returns `(cmat, av, clkout)`.
    ///
    /// See [`ckpfs`](raw::ckpfs) for full documentation.
    pub fn ckpfs(
        &mut self,
        handle: i32,
        descr: &[f64],
        sclkdp: f64,
        tol: f64,
        needav: bool,
    ) -> Result<Option<([[f64; 3]; 3], [f64; 3], f64)>> {
        let mut cmat: [[f64; 3]; 3] = Default::default();
        let mut av: [f64; 3] = Default::default();
        let mut clkout: f64 = Default::default();
        let mut found: bool = Default::default();
        raw::ckpfs(
            self,
            handle,
            descr,
            sclkdp,
            tol,
            needav,
            &mut cmat,
            &mut av,
            &mut clkout,
            &mut found,
        )?;
        Ok(if found {
            Some((cmat, av, clkout))
        } else {
            None
        })
    }

    /// C-kernel, read pointing record, data type 1
    ///
    /// Read a pointing record from a CK segment, data type 1.
    ///
    /// Returns `record`.
    ///
    /// See [`ckr01`](raw::ckr01) for full documentation.
    pub fn ckr01(
        &mut self,
        handle: i32,
        descr: &[f64],
        sclkdp: f64,
        tol: f64,
        needav: bool,
    ) -> Result<Option<Vec<f64>>> {
        let mut record: Vec<f64> = vec![Default::default(); inc::ckparam::CK1RSZ as usize];
        let mut found: bool = Default::default();
        raw::ckr01(
            self,
            handle,
            descr,
            sclkdp,
            tol,
            needav,
            &mut record,
            &mut found,
        )?;
        Ok(if found { Some(record) } else { None })
    }

    /// C-kernel, read pointing record, data type 2
    ///
    /// Read a pointing record from a CK segment, data type 2.
    ///
    /// Returns `record`.
    ///
    /// See [`ckr02`](raw::ckr02) for full documentation.
    pub fn ckr02(
        &mut self,
        handle: i32,
        descr: &[f64],
        sclkdp: f64,
        tol: f64,
    ) -> Result<Option<Vec<f64>>> {
        let mut record: Vec<f64> = vec![Default::default(); inc::ckparam::CK2RSZ as usize];
        let mut found: bool = Default::default();
        raw::ckr02(self, handle, descr, sclkdp, tol, &mut record, &mut found)?;
        Ok(if found { Some(record) } else { None })
    }

    /// C-kernel, read pointing record, data type 3
    ///
    /// Read a pointing record from a CK segment, data type 3.
    ///
    /// Returns `record`.
    ///
    /// See [`ckr03`](raw::ckr03) for full documentation.
    pub fn ckr03(
        &mut self,
        handle: i32,
        descr: &[f64],
        sclkdp: f64,
        tol: f64,
        needav: bool,
    ) -> Result<Option<Vec<f64>>> {
        let mut record: Vec<f64> = vec![Default::default(); inc::ckparam::CK3RSZ as usize];
        let mut found: bool = Default::default();
        raw::ckr03(
            self,
            handle,
            descr,
            sclkdp,
            tol,
            needav,
            &mut record,
            &mut found,
        )?;
        Ok(if found { Some(record) } else { None })
    }

    /// C-kernel, read pointing record, data type 4
    ///
    /// Read a single data record from a type 4 CK segment.
    ///
    /// Returns `record`.
    ///
    /// See [`ckr04`](raw::ckr04) for full documentation.
    pub fn ckr04(
        &mut self,
        handle: i32,
        descr: &[f64],
        sclkdp: f64,
        tol: f64,
        needav: bool,
    ) -> Result<Option<Vec<f64>>> {
        let mut record: Vec<f64> = vec![Default::default(); inc::ckparam::CK4RSZ as usize];
        let mut found: bool = Default::default();
        raw::ckr04(
            self,
            handle,
            descr,
            sclkdp,
            tol,
            needav,
            &mut record,
            &mut found,
        )?;
        Ok(if found { Some(record) } else { None })
    }

    /// Read CK record from segment, type 05
    ///
    /// Read a single CK data record from a segment of type 05
    /// (MEX/Rosetta Attitude file interpolation).
    ///
    /// Returns `record`.
    ///
    /// See [`ckr05`](raw::ckr05) for full documentation.
    pub fn ckr05(
        &mut self,
        handle: i32,
        descr: &[f64; 5],
        sclkdp: f64,
        tol: f64,
        needav: bool,
    ) -> Result<Option<Vec<f64>>> {
        let mut record: Vec<f64> = vec![Default::default(); inc::ckparam::CK5RSZ as usize];
        let mut found: bool = Default::default();
        raw::ckr05(
            self,
            handle,
            descr,
            sclkdp,
            tol,
            needav,
            &mut record,
            &mut found,
        )?;
        Ok(if found { Some(record) } else { None })
    }

    /// C-kernel, read record from segment, type 6
    ///
    /// Read a single CK data record from a segment of type 6
    /// (ESOC/DDID Piecewise Interpolation).
    ///
    /// Returns `record`.
    ///
    /// See [`ckr06`](raw::ckr06) for full documentation.
    pub fn ckr06(
        &mut self,
        handle: i32,
        descr: &[f64; 5],
        sclkdp: f64,
        tol: f64,
        needav: bool,
    ) -> Result<Option<Vec<f64>>> {
        let mut record: Vec<f64> = vec![Default::default(); inc::ckparam::CK6RSZ as usize];
        let mut found: bool = Default::default();
        raw::ckr06(
            self,
            handle,
            descr,
            sclkdp,
            tol,
            needav,
            &mut record,
            &mut found,
        )?;
        Ok(if found { Some(record) } else { None })
    }

    /// C-Kernel, write segment to C-kernel, data type 1
    ///
    /// Add a type 1 segment to a C-kernel.
    ///
    /// See [`ckw01`](raw::ckw01) for full documentation.
    pub fn ckw01(
        &mut self,
        handle: i32,
        begtim: f64,
        endtim: f64,
        inst: i32,
        ref_: &str,
        avflag: bool,
        segid: &str,
        nrec: i32,
        sclkdp: &[f64],
        quats: &[[f64; 4]],
        avvs: &[[f64; 3]],
    ) -> Result<()> {
        raw::ckw01(
            self, handle, begtim, endtim, inst, ref_, avflag, segid, nrec, sclkdp, quats, avvs,
        )?;
        Ok(())
    }

    /// C-Kernel, write segment to C-kernel, data type 2
    ///
    /// Write a type 2 segment to a C-kernel.
    ///
    /// See [`ckw02`](raw::ckw02) for full documentation.
    pub fn ckw02(
        &mut self,
        handle: i32,
        begtim: f64,
        endtim: f64,
        inst: i32,
        ref_: &str,
        segid: &str,
        nrec: i32,
        start: &[f64],
        stop: &[f64],
        quats: &[[f64; 4]],
        avvs: &[[f64; 3]],
        rates: &[f64],
    ) -> Result<()> {
        raw::ckw02(
            self, handle, begtim, endtim, inst, ref_, segid, nrec, start, stop, quats, avvs, rates,
        )?;
        Ok(())
    }

    /// C-Kernel, write segment to C-kernel, data type 3
    ///
    /// Add a type 3 segment to a C-kernel.
    ///
    /// See [`ckw03`](raw::ckw03) for full documentation.
    pub fn ckw03(
        &mut self,
        handle: i32,
        begtim: f64,
        endtim: f64,
        inst: i32,
        ref_: &str,
        avflag: bool,
        segid: &str,
        nrec: i32,
        sclkdp: &[f64],
        quats: &[[f64; 4]],
        avvs: &[[f64; 3]],
        nints: i32,
        starts: &[f64],
    ) -> Result<()> {
        raw::ckw03(
            self, handle, begtim, endtim, inst, ref_, avflag, segid, nrec, sclkdp, quats, avvs,
            nints, starts,
        )?;
        Ok(())
    }

    /// CK type 04: Add data to a segment
    ///
    /// Add data to a type 4 CK segment currently being written to
    /// the file associated with HANDLE. See also CKW04B and CKW04E.
    ///
    /// See [`ckw04a`](raw::ckw04a) for full documentation.
    pub fn ckw04a(
        &mut self,
        handle: i32,
        npkts: i32,
        pktsiz: &mut [i32],
        pktdat: &mut [f64],
        sclkdp: &[f64],
    ) -> Result<()> {
        raw::ckw04a(self, handle, npkts, pktsiz, pktdat, sclkdp)?;
        Ok(())
    }

    /// CK type 04: Begin a segment
    ///
    /// Begin a type CK04 segment in the DAF file associated with
    /// HANDLE. See also CKW04A and CKW04E.
    ///
    /// See [`ckw04b`](raw::ckw04b) for full documentation.
    pub fn ckw04b(
        &mut self,
        handle: i32,
        begtim: f64,
        inst: i32,
        ref_: &str,
        avflag: bool,
        segid: &str,
    ) -> Result<()> {
        raw::ckw04b(self, handle, begtim, inst, ref_, avflag, segid)?;
        Ok(())
    }

    /// CK type 04: End a segment
    ///
    /// End the type 04 CK segment currently being written to the DAF
    /// file associated with HANDLE. See also CKW04B and CKW04E.
    ///
    /// See [`ckw04e`](raw::ckw04e) for full documentation.
    pub fn ckw04e(&mut self, handle: i32, endtim: f64) -> Result<()> {
        raw::ckw04e(self, handle, endtim)?;
        Ok(())
    }

    /// Write CK segment, type 5
    ///
    /// Write a type 5 segment to a CK file.
    ///
    /// See [`ckw05`](raw::ckw05) for full documentation.
    pub fn ckw05(
        &mut self,
        handle: i32,
        subtyp: i32,
        degree: i32,
        begtim: f64,
        endtim: f64,
        inst: i32,
        ref_: &str,
        avflag: bool,
        segid: &str,
        n: i32,
        sclkdp: &[f64],
        packts: &[f64],
        rate: f64,
        nints: i32,
        starts: &[f64],
    ) -> Result<()> {
        raw::ckw05(
            self, handle, subtyp, degree, begtim, endtim, inst, ref_, avflag, segid, n, sclkdp,
            packts, rate, nints, starts,
        )?;
        Ok(())
    }

    /// CK, Write segment, type 6
    ///
    /// Write a type 6 segment to a CK file.
    ///
    /// See [`ckw06`](raw::ckw06) for full documentation.
    pub fn ckw06(
        &mut self,
        handle: i32,
        inst: i32,
        ref_: &str,
        avflag: bool,
        first: f64,
        last: f64,
        segid: &str,
        nmini: i32,
        npkts: &[i32],
        subtps: &[i32],
        degres: &[i32],
        packts: &[f64],
        rates: &[f64],
        sclkdp: &[f64],
        ivlbds: &[f64],
        sellst: bool,
    ) -> Result<()> {
        raw::ckw06(
            self, handle, inst, ref_, avflag, first, last, segid, nmini, npkts, subtps, degres,
            packts, rates, sclkdp, ivlbds, sellst,
        )?;
        Ok(())
    }

    /// C, Speed of light in a vacuum
    ///
    /// Return the speed of light in a vacuum (IAU official
    /// value, in km/sec).
    ///
    /// See [`clight`](raw::clight) for full documentation.
    pub fn clight(&self) -> f64 {
        raw::clight()
    }

    /// Determine state from conic elements
    ///
    /// Determine the state (position, velocity) of an orbiting body
    /// from a set of elliptic, hyperbolic, or parabolic orbital
    /// elements.
    ///
    /// Returns `state`.
    ///
    /// See [`conics`](raw::conics) for full documentation.
    pub fn conics(&mut self, elts: &[f64; 8], et: f64) -> Result<[f64; 6]> {
        let mut state: [f64; 6] = Default::default();
        raw::conics(self, elts, et, &mut state)?;
        Ok(state)
    }

    /// Convert Units
    ///
    /// Take a measurement X, the units associated with
    /// X, and units to which X should be converted; return Y ---
    /// the value of the measurement in the output units.
    ///
    /// Returns `y`.
    ///
    /// See [`convrt`](raw::convrt) for full documentation.
    pub fn convrt(&mut self, x: f64, in_: &str, out: &str) -> Result<f64> {
        let mut y: f64 = Default::default();
        raw::convrt(self, x, in_, out, &mut y)?;
        Ok(y)
    }

    /// Copy a character cell
    ///
    /// Copy the contents of a character cell to another cell.
    ///
    /// See [`copyc`](raw::copyc) for full documentation.
    pub fn copyc(&mut self, cell: &CharCell, copy: &mut CharCell) -> Result<()> {
        raw::copyc(self, cell.as_arg(), copy.as_arg_mut())?;
        Ok(())
    }

    /// Copy a double precision cell
    ///
    /// Copy the contents of a double precision cell to another cell.
    ///
    /// See [`copyd`](raw::copyd) for full documentation.
    pub fn copyd(&mut self, cell: &Cell<f64>, copy: &mut Cell<f64>) -> Result<()> {
        raw::copyd(self, cell.as_raw_slice(), copy.as_raw_mut_slice())?;
        Ok(())
    }

    /// Copy an integer cell
    ///
    /// Copy the contents of an integer cell to another cell.
    ///
    /// See [`copyi`](raw::copyi) for full documentation.
    pub fn copyi(&mut self, cell: &Cell<i32>, copy: &mut Cell<i32>) -> Result<()> {
        raw::copyi(self, cell.as_raw_slice(), copy.as_raw_mut_slice())?;
        Ok(())
    }

    /// Count characters in a text file
    ///
    /// Count the characters in a group of lines in a text file.
    ///
    /// See [`countc`](raw::countc) for full documentation.
    pub fn countc(&mut self, unit: i32, bline: i32, eline: i32, line: &mut str) -> Result<i32> {
        raw::countc(self, unit, bline, eline, line)
    }

    /// Character position
    ///
    /// Find the first occurrence in a string of a character belonging
    /// to a collection of characters, starting at a specified location,
    /// searching forward.
    ///
    /// See [`cpos`](raw::cpos) for full documentation.
    pub fn cpos(&self, str: &str, chars: &str, start: i32) -> i32 {
        raw::cpos(str, chars, start)
    }

    /// Character position, reverse
    ///
    /// Find the first occurrence in a string of a character belonging
    /// to a collection of characters, starting at a specified location,
    /// searching in reverse.
    ///
    /// See [`cposr`](raw::cposr) for full documentation.
    pub fn cposr(&self, str: &str, chars: &str, start: i32) -> i32 {
        raw::cposr(str, chars, start)
    }

    /// Cycle the elements of a character array
    ///
    /// Cycle the elements of a character array forward or backward
    /// in place.
    ///
    /// See [`cyacip`](raw::cyacip) for full documentation.
    pub fn cyacip(&mut self, nelt: i32, dir: char, ncycle: i32, array: &mut CharVec) -> Result<()> {
        raw::cyacip(self, nelt, dir, ncycle, array.as_arg_mut())?;
        Ok(())
    }

    /// Cycle the elements of a DP array, in place
    ///
    /// Cycle the elements of a double precision array forward
    /// or backward in place.
    ///
    /// See [`cyadip`](raw::cyadip) for full documentation.
    pub fn cyadip(&mut self, nelt: i32, dir: char, ncycle: i32, array: &mut [f64]) -> Result<()> {
        raw::cyadip(self, nelt, dir, ncycle, array)?;
        Ok(())
    }

    /// Cycle the elements of an integer array, in place
    ///
    /// Cycle the elements of an integer array forward or backward
    /// in place.
    ///
    /// See [`cyaiip`](raw::cyaiip) for full documentation.
    pub fn cyaiip(&mut self, nelt: i32, dir: char, ncycle: i32, array: &mut [i32]) -> Result<()> {
        raw::cyaiip(self, nelt, dir, ncycle, array)?;
        Ok(())
    }

    /// Cycle the elements of a character array
    ///
    /// Cycle the elements of a character array forward or backward.
    ///
    /// See [`cyclac`](raw::cyclac) for full documentation.
    pub fn cyclac(
        &mut self,
        array: &CharVec,
        nelt: i32,
        dir: char,
        ncycle: i32,
        out: &mut CharVec,
    ) -> Result<()> {
        raw::cyclac(self, array.as_arg(), nelt, dir, ncycle, out.as_arg_mut())?;
        Ok(())
    }

    /// Cycle the elements of a DP array
    ///
    /// Cycle the elements of a double precision array forward
    /// or backward.
    ///
    /// Returns `out`.
    ///
    /// See [`cyclad`](raw::cyclad) for full documentation.
    pub fn cyclad(&mut self, array: &[f64], nelt: i32, dir: char, ncycle: i32) -> Result<Vec<f64>> {
        let mut out: Vec<f64> = vec![Default::default(); nelt.max(0) as usize];
        raw::cyclad(self, array, nelt, dir, ncycle, &mut out)?;
        Ok(out)
    }

    /// Cycle the elements of an integer array
    ///
    /// Cycle the elements of an integer array forward or backward.
    ///
    /// Returns `out`.
    ///
    /// See [`cyclai`](raw::cyclai) for full documentation.
    pub fn cyclai(&mut self, array: &[i32], nelt: i32, dir: char, ncycle: i32) -> Result<Vec<i32>> {
        let mut out: Vec<i32> = vec![Default::default(); nelt.max(0) as usize];
        raw::cyclai(self, array, nelt, dir, ncycle, &mut out)?;
        Ok(out)
    }

    /// Cycle a character string
    ///
    /// Cycle the contents of a character string to the left or right.
    ///
    /// Returns `outstr`.
    ///
    /// See [`cyclec`](raw::cyclec) for full documentation.
    pub fn cyclec(&mut self, instr: &str, dir: char, ncycle: i32) -> Result<String> {
        let mut outstr = blank((instr.len() as i32));
        raw::cyclec(self, instr, dir, ncycle, &mut outstr)?;
        Ok(trim(outstr))
    }

    /// Cylindrical to latitudinal
    ///
    /// Convert from cylindrical to latitudinal coordinates.
    ///
    /// Returns `(radius, lon, lat)`.
    ///
    /// See [`cyllat`](raw::cyllat) for full documentation.
    pub fn cyllat(&self, r: f64, clon: f64, z: f64) -> (f64, f64, f64) {
        let mut radius: f64 = Default::default();
        let mut lon: f64 = Default::default();
        let mut lat: f64 = Default::default();
        raw::cyllat(r, clon, z, &mut radius, &mut lon, &mut lat);
        (radius, lon, lat)
    }

    /// Cylindrical to rectangular
    ///
    /// Convert from cylindrical to rectangular coordinates.
    ///
    /// Returns `rectan`.
    ///
    /// See [`cylrec`](raw::cylrec) for full documentation.
    pub fn cylrec(&self, r: f64, clon: f64, z: f64) -> [f64; 3] {
        let mut rectan: [f64; 3] = Default::default();
        raw::cylrec(r, clon, z, &mut rectan);
        rectan
    }

    /// Cylindrical to spherical
    ///
    /// Convert from cylindrical to spherical coordinates.
    ///
    /// Returns `(radius, colat, slon)`.
    ///
    /// See [`cylsph`](raw::cylsph) for full documentation.
    pub fn cylsph(&self, r: f64, clon: f64, z: f64) -> (f64, f64, f64) {
        let mut radius: f64 = Default::default();
        let mut colat: f64 = Default::default();
        let mut slon: f64 = Default::default();
        raw::cylsph(r, clon, z, &mut radius, &mut colat, &mut slon);
        (radius, colat, slon)
    }

    /// Double precision arc hyperbolic cosine
    ///
    /// Return the inverse hyperbolic cosine of a double precision
    /// argument.
    ///
    /// See [`dacosh`](raw::dacosh) for full documentation.
    pub fn dacosh(&mut self, x: f64) -> Result<f64> {
        raw::dacosh(self, x)
    }

    /// arc cosine of bracketed argument
    ///
    /// Compute arc cosine of a bracketed argument.
    ///
    /// This routine produces a SPICE error if the |argument| exceeds
    /// 1.D0 by more than TOL. If ARG exceeds 1.D0, the argument is
    /// evaluated as if it equaled 1.D0, if ARG is less than -1.,
    /// the argument is evaluated as if it equaled -1.D0.
    ///
    /// See [`dacosn`](raw::dacosn) for full documentation.
    pub fn dacosn(&mut self, arg: f64, tol: f64) -> Result<f64> {
        raw::dacosn(self, arg, tol)
    }

    /// DAF add comments
    ///
    /// Add comments from a buffer of character strings to the comment
    /// area of a binary DAF file, appending them to any comments which
    /// are already present in the file's comment area.
    ///
    /// See [`dafac`](raw::dafac) for full documentation.
    pub fn dafac(&mut self, handle: i32, n: i32, buffer: &CharVec) -> Result<()> {
        raw::dafac(self, handle, n, buffer.as_arg())?;
        Ok(())
    }

    /// DAF, open for read
    ///
    /// Open a DAF for subsequent read requests.
    ///
    /// Returns `handle`.
    ///
    /// See [`dafopr`](raw::dafopr) for full documentation.
    pub fn dafopr(&mut self, fname: &str) -> Result<i32> {
        let mut handle: i32 = Default::default();
        raw::dafopr(self, fname, &mut handle)?;
        Ok(handle)
    }

    /// DAF, open for write
    ///
    /// Open a DAF for subsequent write requests.
    ///
    /// Returns `handle`.
    ///
    /// See [`dafopw`](raw::dafopw) for full documentation.
    pub fn dafopw(&mut self, fname: &str) -> Result<i32> {
        let mut handle: i32 = Default::default();
        raw::dafopw(self, fname, &mut handle)?;
        Ok(handle)
    }

    /// DAF, open new
    ///
    /// Open a new DAF for subsequent write requests.
    ///
    /// Returns `handle`.
    ///
    /// See [`dafonw`](raw::dafonw) for full documentation.
    pub fn dafonw(
        &mut self,
        fname: &str,
        ftype: &str,
        nd: i32,
        ni: i32,
        ifname: &str,
        resv: i32,
    ) -> Result<i32> {
        let mut handle: i32 = Default::default();
        raw::dafonw(self, fname, ftype, nd, ni, ifname, resv, &mut handle)?;
        Ok(handle)
    }

    /// DAF, close
    ///
    /// Close the DAF associated with a given handle.
    ///
    /// See [`dafcls`](raw::dafcls) for full documentation.
    pub fn dafcls(&mut self, handle: i32) -> Result<()> {
        raw::dafcls(self, handle)?;
        Ok(())
    }

    /// DAF, handle to summary format
    ///
    /// Return the summary format associated with a handle.
    ///
    /// Returns `(nd, ni)`.
    ///
    /// See [`dafhsf`](raw::dafhsf) for full documentation.
    pub fn dafhsf(&mut self, handle: i32) -> Result<(i32, i32)> {
        let mut nd: i32 = Default::default();
        let mut ni: i32 = Default::default();
        raw::dafhsf(self, handle, &mut nd, &mut ni)?;
        Ok((nd, ni))
    }

    /// DAF, handle to logical unit
    ///
    /// Return the logical unit associated with a handle.
    ///
    /// Returns `unit`.
    ///
    /// See [`dafhlu`](raw::dafhlu) for full documentation.
    pub fn dafhlu(&mut self, handle: i32) -> Result<i32> {
        let mut unit: i32 = Default::default();
        raw::dafhlu(self, handle, &mut unit)?;
        Ok(unit)
    }

    /// DAF, logical unit to handle
    ///
    /// Return the handle associated with a logical unit.
    ///
    /// Returns `handle`.
    ///
    /// See [`dafluh`](raw::dafluh) for full documentation.
    pub fn dafluh(&mut self, unit: i32) -> Result<i32> {
        let mut handle: i32 = Default::default();
        raw::dafluh(self, unit, &mut handle)?;
        Ok(handle)
    }

    /// DAF, handle to file name
    ///
    /// Return the name of the file associated with a handle.
    ///
    /// Returns `fname`.
    ///
    /// See [`dafhfn`](raw::dafhfn) for full documentation.
    pub fn dafhfn(&mut self, handle: i32) -> Result<String> {
        let mut fname = blank(inc::zzddhman::FILEN);
        raw::dafhfn(self, handle, &mut fname)?;
        Ok(trim(fname))
    }

    /// DAF, file name to handle
    ///
    /// Return handle associated with a file name.
    ///
    /// Returns `handle`.
    ///
    /// See [`daffnh`](raw::daffnh) for full documentation.
    pub fn daffnh(&mut self, fname: &str) -> Result<i32> {
        let mut handle: i32 = Default::default();
        raw::daffnh(self, fname, &mut handle)?;
        Ok(handle)
    }

    /// DAF, handles of open files
    ///
    /// Return a SPICE set containing the handles of all currently
    /// open DAFS.
    ///
    /// See [`dafhof`](raw::dafhof) for full documentation.
    pub fn dafhof(&mut self, fhset: &mut Cell<i32>) -> Result<()> {
        raw::dafhof(self, fhset.as_raw_mut_slice())?;
        Ok(())
    }

    /// DAF, signal invalid handles
    ///
    /// Signal an error if a DAF file handle does not designate a DAF
    /// that is open for a specified type of access.
    ///
    /// See [`dafsih`](raw::dafsih) for full documentation.
    pub fn dafsih(&mut self, handle: i32, access: &str) -> Result<()> {
        raw::dafsih(self, handle, access)?;
        Ok(())
    }

    /// DAF, begin new array
    ///
    /// Begin a new array in a DAF.
    ///
    /// See [`dafbna`](raw::dafbna) for full documentation.
    pub fn dafbna(&mut self, handle: i32, sum: &[f64], name: &str) -> Result<()> {
        raw::dafbna(self, handle, sum, name)?;
        Ok(())
    }

    /// DAF, add data to array
    ///
    /// Add one or more double precision words of data to the newest
    /// array in the current DAF.
    ///
    /// See [`dafada`](raw::dafada) for full documentation.
    pub fn dafada(&mut self, data: &[f64], n: i32) -> Result<()> {
        raw::dafada(self, data, n)?;
        Ok(())
    }

    /// DAF, end new array
    ///
    /// End the addition of data to the newest array in the current DAF.
    ///
    /// See [`dafena`](raw::dafena) for full documentation.
    pub fn dafena(&mut self) -> Result<()> {
        raw::dafena(self)?;
        Ok(())
    }

    /// DAF, continue adding data
    ///
    /// Select a DAF that already has a new array in progress as the
    /// one to continue adding data to.
    ///
    /// See [`dafcad`](raw::dafcad) for full documentation.
    pub fn dafcad(&mut self, handle: i32) -> Result<()> {
        raw::dafcad(self, handle)?;
        Ok(())
    }

    /// DAF, add reserved records
    ///
    /// Add a specified number of reserved records to a Double Precision
    /// Array File (DAF).
    ///
    /// See [`dafarr`](raw::dafarr) for full documentation.
    pub fn dafarr(&mut self, handle: i32, resv: i32) -> Result<()> {
        raw::dafarr(self, handle, resv)?;
        Ok(())
    }

    /// DAF, convert binary file to transfer file
    ///
    /// Convert the contents of a binary DAF file to an equivalent DAF
    /// transfer file.
    ///
    /// See [`dafbt`](raw::dafbt) for full documentation.
    pub fn dafbt(&mut self, binfil: &str, xfrlun: i32) -> Result<()> {
        raw::dafbt(self, binfil, xfrlun)?;
        Ok(())
    }

    /// DAF delete comments
    ///
    /// Delete the entire comment area of a previously opened binary
    /// DAF attached to HANDLE.
    ///
    /// See [`dafdc`](raw::dafdc) for full documentation.
    pub fn dafdc(&mut self, handle: i32) -> Result<()> {
        raw::dafdc(self, handle)?;
        Ok(())
    }

    /// DAF extract comments
    ///
    /// Extract comments from the comment area of a binary DAF.
    ///
    /// Returns `(n, done)`.
    ///
    /// See [`dafec`](raw::dafec) for full documentation.
    pub fn dafec(&mut self, handle: i32, bufsiz: i32, buffer: &mut CharVec) -> Result<(i32, bool)> {
        let mut n: i32 = Default::default();
        let mut done: bool = Default::default();
        raw::dafec(self, handle, bufsiz, &mut n, buffer.as_arg_mut(), &mut done)?;
        Ok((n, done))
    }

    /// DAF, begin forward search
    ///
    /// Begin a forward search for arrays in a DAF.
    ///
    /// See [`dafbfs`](raw::dafbfs) for full documentation.
    pub fn dafbfs(&mut self, handle: i32) -> Result<()> {
        raw::dafbfs(self, handle)?;
        Ok(())
    }

    /// DAF, find next array
    ///
    /// Find the next (forward) array in the current DAF.
    ///
    /// See [`daffna`](raw::daffna) for full documentation.
    pub fn daffna(&mut self) -> Result<Option<()>> {
        let mut found: bool = Default::default();
        raw::daffna(self, &mut found)?;
        Ok(if found { Some(()) } else { None })
    }

    /// DAF, begin backward search
    ///
    /// Begin a backward search for arrays in a DAF.
    ///
    /// See [`dafbbs`](raw::dafbbs) for full documentation.
    pub fn dafbbs(&mut self, handle: i32) -> Result<()> {
        raw::dafbbs(self, handle)?;
        Ok(())
    }

    /// DAF, find previous array
    ///
    /// Find the previous (backward) array in the current DAF.
    ///
    /// See [`daffpa`](raw::daffpa) for full documentation.
    pub fn daffpa(&mut self) -> Result<Option<()>> {
        let mut found: bool = Default::default();
        raw::daffpa(self, &mut found)?;
        Ok(if found { Some(()) } else { None })
    }

    /// DAF, get summary
    ///
    /// Return (get) the summary for the current array in the current
    /// DAF.
    ///
    /// Returns `sum`.
    ///
    /// See [`dafgs`](raw::dafgs) for full documentation.
    pub fn dafgs(&mut self) -> Result<Vec<f64>> {
        let mut sum: Vec<f64> = vec![Default::default(); 128 as usize];
        raw::dafgs(self, &mut sum)?;
        Ok(sum)
    }

    /// DAF, get array name
    ///
    /// Return (get) the name for the current array in the current DAF.
    ///
    /// Returns `name`.
    ///
    /// See [`dafgn`](raw::dafgn) for full documentation.
    pub fn dafgn(&mut self) -> Result<String> {
        let mut name = blank(1000);
        raw::dafgn(self, &mut name)?;
        Ok(trim(name))
    }

    /// DAF, get handle
    ///
    /// Return (get) the handle of the DAF currently being searched.
    ///
    /// Returns `handle`.
    ///
    /// See [`dafgh`](raw::dafgh) for full documentation.
    pub fn dafgh(&mut self) -> Result<i32> {
        let mut handle: i32 = Default::default();
        raw::dafgh(self, &mut handle)?;
        Ok(handle)
    }

    /// DAF, replace summary
    ///
    /// Change the summary for the current array in the current DAF.
    ///
    /// See [`dafrs`](raw::dafrs) for full documentation.
    pub fn dafrs(&mut self, sum: &[f64]) -> Result<()> {
        raw::dafrs(self, sum)?;
        Ok(())
    }

    /// DAF, change array name
    ///
    /// Replace the name for the current array in the current DAF.
    ///
    /// See [`dafrn`](raw::dafrn) for full documentation.
    pub fn dafrn(&mut self, name: &str) -> Result<()> {
        raw::dafrn(self, name)?;
        Ok(())
    }

    /// DAF, write summary
    ///
    /// Write a new summary for the current array in the current DAF.
    ///
    /// See [`dafws`](raw::dafws) for full documentation.
    pub fn dafws(&mut self, sum: &[f64]) -> Result<()> {
        raw::dafws(self, sum)?;
        Ok(())
    }

    /// DAF, continue search
    ///
    /// Select a DAF that already has a search in progress as the
    /// one to continue searching.
    ///
    /// See [`dafcs`](raw::dafcs) for full documentation.
    pub fn dafcs(&mut self, handle: i32) -> Result<()> {
        raw::dafcs(self, handle)?;
        Ok(())
    }

    /// DAF, read data from address
    ///
    /// Read the double precision data bounded by two addresses within
    /// a DAF.
    ///
    /// Returns `data`.
    ///
    /// See [`dafgda`](raw::dafgda) for full documentation.
    pub fn dafgda(&mut self, handle: i32, baddr: i32, eaddr: i32) -> Result<Vec<f64>> {
        let mut data: Vec<f64> = vec![Default::default(); (eaddr + 1 - baddr).max(0) as usize];
        raw::dafgda(self, handle, baddr, eaddr, &mut data)?;
        Ok(data)
    }

    /// DAF, pack summary
    ///
    /// Pack (assemble) an array summary from its double precision and
    /// integer components.
    ///
    /// Returns `sum`.
    ///
    /// See [`dafps`](raw::dafps) for full documentation.
    pub fn dafps(&self, nd: i32, ni: i32, dc: &[f64], ic: &[i32]) -> Vec<f64> {
        let mut sum: Vec<f64> = vec![Default::default(); (nd + (ni + 1) / 2).max(0) as usize];
        raw::dafps(nd, ni, dc, ic, &mut sum);
        sum
    }

    /// DAF, unpack summary
    ///
    /// Unpack an array summary into its double precision and integer
    /// components.
    ///
    /// Returns `(dc, ic)`.
    ///
    /// See [`dafus`](raw::dafus) for full documentation.
    pub fn dafus(&self, sum: &[f64], nd: i32, ni: i32) -> (Vec<f64>, Vec<i32>) {
        let mut dc: Vec<f64> = vec![Default::default(); nd.max(0) as usize];
        let mut ic: Vec<i32> = vec![Default::default(); ni.max(0) as usize];
        raw::dafus(sum, nd, ni, &mut dc, &mut ic);
        (dc, ic)
    }

    /// DAF, Re-order arrays
    ///
    /// Reorder the arrays in a DAF according to a given order vector.
    ///
    /// See [`dafra`](raw::dafra) for full documentation.
    pub fn dafra(&mut self, handle: i32, iorder: &mut [i32], n: i32) -> Result<()> {
        raw::dafra(self, handle, iorder, n)?;
        Ok(())
    }

    /// DAF, read character record
    ///
    /// Read the contents of a character record from a DAF.
    ///
    /// Returns `crec`.
    ///
    /// See [`dafrcr`](raw::dafrcr) for full documentation.
    pub fn dafrcr(&mut self, handle: i32, recno: i32) -> Result<String> {
        let mut crec = blank(1000);
        raw::dafrcr(self, handle, recno, &mut crec)?;
        Ok(trim(crec))
    }

    /// DAF, read file record
    ///
    /// Read the contents of the file record of a DAF.
    ///
    /// Returns `(nd, ni, ifname, fward, bward, free)`.
    ///
    /// See [`dafrfr`](raw::dafrfr) for full documentation.
    pub fn dafrfr(&mut self, handle: i32) -> Result<(i32, i32, String, i32, i32, i32)> {
        let mut nd: i32 = Default::default();
        let mut ni: i32 = Default::default();
        let mut ifname = blank(60);
        let mut fward: i32 = Default::default();
        let mut bward: i32 = Default::default();
        let mut free: i32 = Default::default();
        raw::dafrfr(
            self,
            handle,
            &mut nd,
            &mut ni,
            &mut ifname,
            &mut fward,
            &mut bward,
            &mut free,
        )?;
        Ok((nd, ni, trim(ifname), fward, bward, free))
    }

    /// DAF, remove reserved records
    ///
    /// Remove a specified number of reserved records from a Double
    /// Precision Array File (DAF).
    ///
    /// See [`dafrrr`](raw::dafrrr) for full documentation.
    pub fn dafrrr(&mut self, handle: i32, resv: i32) -> Result<()> {
        raw::dafrrr(self, handle, resv)?;
        Ok(())
    }

    /// DAF, record/word to address
    ///
    /// Convert a record/word pair to its equivalent address within
    /// a DAF.
    ///
    /// Returns `addr`.
    ///
    /// See [`dafrwa`](raw::dafrwa) for full documentation.
    pub fn dafrwa(&mut self, recno: i32, wordno: i32) -> Result<i32> {
        let mut addr: i32 = Default::default();
        raw::dafrwa(self, recno, wordno, &mut addr)?;
        Ok(addr)
    }

    /// DAF, address to record/word
    ///
    /// Convert an address within a DAF to its equivalent
    /// record/word representation.
    ///
    /// Returns `(recno, wordno)`.
    ///
    /// See [`dafarw`](raw::dafarw) for full documentation.
    pub fn dafarw(&mut self, addr: i32) -> Result<(i32, i32)> {
        let mut recno: i32 = Default::default();
        let mut wordno: i32 = Default::default();
        raw::dafarw(self, addr, &mut recno, &mut wordno)?;
        Ok((recno, wordno))
    }

    /// DAF, get double precision record
    ///
    /// Read a portion of the contents of a double precision record in a
    /// DAF file.
    ///
    /// Returns `data`.
    ///
    /// See [`dafgdr`](raw::dafgdr) for full documentation.
    pub fn dafgdr(
        &mut self,
        handle: i32,
        recno: i32,
        begin: i32,
        end: i32,
    ) -> Result<Option<Vec<f64>>> {
        let mut data: Vec<f64> =
            vec![Default::default(); (end.min(128) + 1 - begin.max(1)).max(0) as usize];
        let mut found: bool = Default::default();
        raw::dafgdr(self, handle, recno, begin, end, &mut data, &mut found)?;
        Ok(if found { Some(data) } else { None })
    }

    /// DAF, get summary/descriptor record
    ///
    /// Read a portion of the contents of a summary record in a DAF file.
    ///
    /// Returns `data`.
    ///
    /// See [`dafgsr`](raw::dafgsr) for full documentation.
    pub fn dafgsr(
        &mut self,
        handle: i32,
        recno: i32,
        begin: i32,
        end: i32,
    ) -> Result<Option<Vec<f64>>> {
        let mut data: Vec<f64> =
            vec![Default::default(); (end.min(128) + 1 - begin.max(1)).max(0) as usize];
        let mut found: bool = Default::default();
        raw::dafgsr(self, handle, recno, begin, end, &mut data, &mut found)?;
        Ok(if found { Some(data) } else { None })
    }

    /// DAF, write double precision record
    ///
    /// Write or rewrite the contents of a double precision record in
    /// a DAF.
    ///
    /// See [`dafwdr`](raw::dafwdr) for full documentation.
    pub fn dafwdr(&mut self, handle: i32, recno: i32, drec: &[f64; 128]) -> Result<()> {
        raw::dafwdr(self, handle, recno, drec)?;
        Ok(())
    }

    /// DAF number of reads, requests
    ///
    /// Return the number of reads and requests fielded by DAFRDR.
    ///
    /// Returns `(reads, reqs)`.
    ///
    /// See [`dafnrr`](raw::dafnrr) for full documentation.
    pub fn dafnrr(&mut self) -> (i32, i32) {
        let mut reads: i32 = Default::default();
        let mut reqs: i32 = Default::default();
        raw::dafnrr(self, &mut reads, &mut reqs);
        (reads, reqs)
    }

    /// DAF, convert transfer file to binary file
    ///
    /// Convert the contents of an DAF transfer file into an equivalent
    /// binary DAF file.
    ///
    /// See [`daftb`](raw::daftb) for full documentation.
    pub fn daftb(&mut self, xfrlun: i32, binfil: &str) -> Result<()> {
        raw::daftb(self, xfrlun, binfil)?;
        Ok(())
    }

    /// DAF, write character record
    ///
    /// Write or rewrite the contents of a character record to
    /// a DAF.
    ///
    /// See [`dafwcr`](raw::dafwcr) for full documentation.
    pub fn dafwcr(&mut self, handle: i32, recno: i32, crec: &str) -> Result<()> {
        raw::dafwcr(self, handle, recno, crec)?;
        Ok(())
    }

    /// DAF, write data to address
    ///
    /// Write or rewrite the double precision data bounded by two
    /// addresses within a DAF.
    ///
    /// See [`dafwda`](raw::dafwda) for full documentation.
    pub fn dafwda(&mut self, handle: i32, begin: i32, end: i32, data: &[f64]) -> Result<()> {
        raw::dafwda(self, handle, begin, end, data)?;
        Ok(())
    }

    /// DAF write file record
    ///
    /// Write or rewrite the contents of the file record of a DAF.
    ///
    /// See [`dafwfr`](raw::dafwfr) for full documentation.
    pub fn dafwfr(
        &mut self,
        handle: i32,
        nd: i32,
        ni: i32,
        ifname: &str,
        fward: i32,
        bward: i32,
        free: i32,
    ) -> Result<()> {
        raw::dafwfr(self, handle, nd, ni, ifname, fward, bward, free)?;
        Ok(())
    }

    /// DAS, address to physical location
    ///
    /// Map a DAS address to a physical location in a specified DAS file.
    ///
    /// Returns `(clbase, clsize, recno, wordno)`.
    ///
    /// See [`dasa2l`](raw::dasa2l) for full documentation.
    pub fn dasa2l(&mut self, handle: i32, type_: i32, addrss: i32) -> Result<(i32, i32, i32, i32)> {
        let mut clbase: i32 = Default::default();
        let mut clsize: i32 = Default::default();
        let mut recno: i32 = Default::default();
        let mut wordno: i32 = Default::default();
        raw::dasa2l(
            self,
            handle,
            type_,
            addrss,
            &mut clbase,
            &mut clsize,
            &mut recno,
            &mut wordno,
        )?;
        Ok((clbase, clsize, recno, wordno))
    }

    /// DAS add comments
    ///
    /// Add comments from a buffer of character strings to the comment
    /// area of a binary DAS file, appending them to any comments which
    /// are already present in the file's comment area.
    ///
    /// See [`dasac`](raw::dasac) for full documentation.
    pub fn dasac(&mut self, handle: i32, n: i32, buffer: &CharVec) -> Result<()> {
        raw::dasac(self, handle, n, buffer.as_arg())?;
        Ok(())
    }

    /// DAS, add comment records
    ///
    /// Increase the size of the comment area in a DAS file to accommodate
    /// a specified number of additional comment records.
    ///
    /// See [`dasacr`](raw::dasacr) for full documentation.
    pub fn dasacr(&mut self, handle: i32, n: i32) -> Result<()> {
        raw::dasacr(self, handle, n)?;
        Ok(())
    }

    /// DAS add comments from a logical unit
    ///
    /// Add comments to a previously opened binary DAS file from a
    /// previously opened text file attached to a Fortran logical unit.
    ///
    /// See [`dasacu`](raw::dasacu) for full documentation.
    pub fn dasacu(
        &mut self,
        comlun: i32,
        begmrk: &str,
        endmrk: &str,
        insbln: bool,
        handle: i32,
    ) -> Result<()> {
        raw::dasacu(self, comlun, begmrk, endmrk, insbln, handle)?;
        Ok(())
    }

    /// DAS, add data, character
    ///
    /// Add character data to a DAS file.
    ///
    /// See [`dasadc`](raw::dasadc) for full documentation.
    pub fn dasadc(
        &mut self,
        handle: i32,
        n: i32,
        bpos: i32,
        epos: i32,
        data: &CharVec,
    ) -> Result<()> {
        raw::dasadc(self, handle, n, bpos, epos, data.as_arg())?;
        Ok(())
    }

    /// DAS, add data, double precision
    ///
    /// Add an array of double precision numbers to a DAS file.
    ///
    /// See [`dasadd`](raw::dasadd) for full documentation.
    pub fn dasadd(&mut self, handle: i32, n: i32, data: &[f64]) -> Result<()> {
        raw::dasadd(self, handle, n, data)?;
        Ok(())
    }

    /// DAS, add data, integer
    ///
    /// Add an array of integers to a DAS file.
    ///
    /// See [`dasadi`](raw::dasadi) for full documentation.
    pub fn dasadi(&mut self, handle: i32, n: i32, data: &[i32]) -> Result<()> {
        raw::dasadi(self, handle, n, data)?;
        Ok(())
    }

    /// DAS, convert binary file to transfer file
    ///
    /// Convert the contents of a binary DAS file to an equivalent DAS
    /// transfer file.
    ///
    /// See [`dasbt`](raw::dasbt) for full documentation.
    pub fn dasbt(&mut self, binfil: &str, xfrlun: i32) -> Result<()> {
        raw::dasbt(self, binfil, xfrlun)?;
        Ok(())
    }

    /// DAS, close file
    ///
    /// Close a DAS file.
    ///
    /// See [`dascls`](raw::dascls) for full documentation.
    pub fn dascls(&mut self, handle: i32) -> Result<()> {
        raw::dascls(self, handle)?;
        Ok(())
    }

    /// DAS, create or update directories
    ///
    /// Create or update directories in a DAS file to reflect addition
    /// of a specified number of words of a specified data type.
    ///
    /// See [`dascud`](raw::dascud) for full documentation.
    pub fn dascud(&mut self, handle: i32, type_: i32, nwords: i32) -> Result<()> {
        raw::dascud(self, handle, type_, nwords)?;
        Ok(())
    }

    /// DAS delete comments
    ///
    /// Delete the entire comment area of a previously opened binary
    /// DAS file.
    ///
    /// See [`dasdc`](raw::dasdc) for full documentation.
    pub fn dasdc(&mut self, handle: i32) -> Result<()> {
        raw::dasdc(self, handle)?;
        Ok(())
    }

    /// DAS extract comments
    ///
    /// Extract comments from the comment area of a binary DAS file.
    ///
    /// Returns `(n, done)`.
    ///
    /// See [`dasec`](raw::dasec) for full documentation.
    pub fn dasec(&mut self, handle: i32, bufsiz: i32, buffer: &mut CharVec) -> Result<(i32, bool)> {
        let mut n: i32 = Default::default();
        let mut done: bool = Default::default();
        raw::dasec(self, handle, bufsiz, &mut n, buffer.as_arg_mut(), &mut done)?;
        Ok((n, done))
    }

    /// DAS extract comments to a logical unit
    ///
    /// Extract comments from a previously opened binary DAS file to a
    /// previously opened text file attached to a Fortran logical unit.
    ///
    /// Returns `comnts`.
    ///
    /// See [`dasecu`](raw::dasecu) for full documentation.
    pub fn dasecu(&mut self, handle: i32, comlun: i32) -> Result<bool> {
        let mut comnts: bool = Default::default();
        raw::dasecu(self, handle, comlun, &mut comnts)?;
        Ok(comnts)
    }

    /// DAS, open for read
    ///
    /// Open a DAS file for reading.
    ///
    /// Returns `handle`.
    ///
    /// See [`dasopr`](raw::dasopr) for full documentation.
    pub fn dasopr(&mut self, fname: &str) -> Result<i32> {
        let mut handle: i32 = Default::default();
        raw::dasopr(self, fname, &mut handle)?;
        Ok(handle)
    }

    /// DAS, open for write
    ///
    /// Open a DAS file for writing.
    ///
    /// Returns `handle`.
    ///
    /// See [`dasopw`](raw::dasopw) for full documentation.
    pub fn dasopw(&mut self, fname: &str) -> Result<i32> {
        let mut handle: i32 = Default::default();
        raw::dasopw(self, fname, &mut handle)?;
        Ok(handle)
    }

    /// DAS, open new file
    ///
    /// Open a new DAS file and set the file type.
    ///
    /// Returns `handle`.
    ///
    /// See [`dasonw`](raw::dasonw) for full documentation.
    pub fn dasonw(&mut self, fname: &str, ftype: &str, ifname: &str, ncomr: i32) -> Result<i32> {
        let mut handle: i32 = Default::default();
        raw::dasonw(self, fname, ftype, ifname, ncomr, &mut handle)?;
        Ok(handle)
    }

    /// DAS, open scratch
    ///
    /// Open a scratch DAS file for writing.
    ///
    /// Returns `handle`.
    ///
    /// See [`dasops`](raw::dasops) for full documentation.
    pub fn dasops(&mut self) -> Result<i32> {
        let mut handle: i32 = Default::default();
        raw::dasops(self, &mut handle)?;
        Ok(handle)
    }

    /// DAS, low-level close
    ///
    /// Close the DAS file associated with a given handle, without
    /// flushing buffered data or segregating the file.
    ///
    /// See [`dasllc`](raw::dasllc) for full documentation.
    pub fn dasllc(&mut self, handle: i32) -> Result<()> {
        raw::dasllc(self, handle)?;
        Ok(())
    }

    /// DAS, handle to file summary
    ///
    /// Return a file summary for a specified DAS file.
    ///
    /// Returns `(nresvr, nresvc, ncomr, ncomc, free, lastla, lastrc, lastwd)`.
    ///
    /// See [`dashfs`](raw::dashfs) for full documentation.
    pub fn dashfs(
        &mut self,
        handle: i32,
    ) -> Result<(i32, i32, i32, i32, i32, [i32; 3], [i32; 3], [i32; 3])> {
        let mut nresvr: i32 = Default::default();
        let mut nresvc: i32 = Default::default();
        let mut ncomr: i32 = Default::default();
        let mut ncomc: i32 = Default::default();
        let mut free: i32 = Default::default();
        let mut lastla: [i32; 3] = Default::default();
        let mut lastrc: [i32; 3] = Default::default();
        let mut lastwd: [i32; 3] = Default::default();
        raw::dashfs(
            self,
            handle,
            &mut nresvr,
            &mut nresvc,
            &mut ncomr,
            &mut ncomc,
            &mut free,
            &mut lastla,
            &mut lastrc,
            &mut lastwd,
        )?;
        Ok((nresvr, nresvc, ncomr, ncomc, free, lastla, lastrc, lastwd))
    }

    /// DAS, update file summary
    ///
    /// Update the file summary in a specified DAS file.
    ///
    /// See [`dasufs`](raw::dasufs) for full documentation.
    pub fn dasufs(
        &mut self,
        handle: i32,
        nresvr: i32,
        nresvc: i32,
        ncomr: i32,
        ncomc: i32,
        free: i32,
        lastla: &[i32; 3],
        lastrc: &[i32; 3],
        lastwd: &[i32; 3],
    ) -> Result<()> {
        raw::dasufs(
            self, handle, nresvr, nresvc, ncomr, ncomc, free, lastla, lastrc, lastwd,
        )?;
        Ok(())
    }

    /// DAS, handle to logical unit
    ///
    /// Return the logical unit associated with a handle. The unit
    /// is "locked" to the handle by the DAF/DAS handle manager
    /// subsystem.
    ///
    /// Returns `unit`.
    ///
    /// See [`dashlu`](raw::dashlu) for full documentation.
    pub fn dashlu(&mut self, handle: i32) -> Result<i32> {
        let mut unit: i32 = Default::default();
        raw::dashlu(self, handle, &mut unit)?;
        Ok(unit)
    }

    /// DAS, logical unit to handle
    ///
    /// Return the handle associated with a logical unit.
    ///
    /// Returns `handle`.
    ///
    /// See [`dasluh`](raw::dasluh) for full documentation.
    pub fn dasluh(&mut self, unit: i32) -> Result<i32> {
        let mut handle: i32 = Default::default();
        raw::dasluh(self, unit, &mut handle)?;
        Ok(handle)
    }

    /// DAS, handle to file name
    ///
    /// Return the name of the DAS file associated with a handle.
    ///
    /// Returns `fname`.
    ///
    /// See [`dashfn`](raw::dashfn) for full documentation.
    pub fn dashfn(&mut self, handle: i32) -> Result<String> {
        let mut fname = blank(inc::zzddhman::FILEN);
        raw::dashfn(self, handle, &mut fname)?;
        Ok(trim(fname))
    }

    /// DAS, file name to handle
    ///
    /// Return handle associated with a file name.
    ///
    /// Returns `handle`.
    ///
    /// See [`dasfnh`](raw::dasfnh) for full documentation.
    pub fn dasfnh(&mut self, fname: &str) -> Result<i32> {
        let mut handle: i32 = Default::default();
        raw::dasfnh(self, fname, &mut handle)?;
        Ok(handle)
    }

    /// DAS, handles of open files
    ///
    /// Return a SPICE set containing the handles of all currently
    /// open DAS files.
    ///
    /// See [`dashof`](raw::dashof) for full documentation.
    pub fn dashof(&mut self, fhset: &mut Cell<i32>) -> Result<()> {
        raw::dashof(self, fhset.as_raw_mut_slice())?;
        Ok(())
    }

    /// DAS, signal invalid handles
    ///
    /// Signal an error if a DAS file file handle does not designate a
    /// DAS file that is open for a specified type of access.
    ///
    /// See [`dassih`](raw::dassih) for full documentation.
    pub fn dassih(&mut self, handle: i32, access: &str) -> Result<()> {
        raw::dassih(self, handle, access)?;
        Ok(())
    }

    /// DAS, handle to access method
    ///
    /// Return the allowed access method for a specified DAS file.
    ///
    /// Returns `access`.
    ///
    /// See [`dasham`](raw::dasham) for full documentation.
    pub fn dasham(&mut self, handle: i32) -> Result<String> {
        let mut access = blank(5);
        raw::dasham(self, handle, &mut access)?;
        Ok(trim(access))
    }

    /// arc sine of bracketed argument
    ///
    /// Compute arc sine of a bracketed argument.
    ///
    /// This routine produces a SPICE error if the |argument| exceeds
    /// 1.D0 by more than TOL. If ARG exceeds 1.D0, the argument is
    /// evaluated as if it equaled 1.D0, if ARG is less than -1.,
    /// the argument is evaluated as if it equaled -1.D0.
    ///
    /// See [`dasine`](raw::dasine) for full documentation.
    pub fn dasine(&mut self, arg: f64, tol: f64) -> Result<f64> {
        raw::dasine(self, arg, tol)
    }

    /// DAS, Fortran I/O, character
    ///
    /// Perform Fortran reads and writes of DAS character records.
    ///
    /// See [`dasioc`](raw::dasioc) for full documentation.
    pub fn dasioc(
        &mut self,
        action: &str,
        unit: i32,
        recno: i32,
        record: &mut [u8; 1024],
    ) -> Result<()> {
        raw::dasioc(self, action, unit, recno, record)?;
        Ok(())
    }

    /// DAS, Fortran I/O, double precision
    ///
    /// Perform Fortran reads and writes of DAS double precision records.
    /// This routine operates on DAS files having native binary format.
    ///
    /// See [`dasiod`](raw::dasiod) for full documentation.
    pub fn dasiod(
        &mut self,
        action: &str,
        unit: i32,
        recno: i32,
        record: &mut [f64; 128],
    ) -> Result<()> {
        raw::dasiod(self, action, unit, recno, record)?;
        Ok(())
    }

    /// DAS, Fortran I/O, integer
    ///
    /// Perform Fortran reads and writes of DAS integer records.
    /// This routine operates on DAS files having native binary
    /// format.
    ///
    /// See [`dasioi`](raw::dasioi) for full documentation.
    pub fn dasioi(
        &mut self,
        action: &str,
        unit: i32,
        recno: i32,
        record: &mut [i32; 256],
    ) -> Result<()> {
        raw::dasioi(self, action, unit, recno, record)?;
        Ok(())
    }

    /// DAS, last logical addresses
    ///
    /// Return last DAS logical addresses of character, double precision
    /// and integer type that are currently in use in a specified DAS
    /// file.
    ///
    /// Returns `(lastc, lastd, lasti)`.
    ///
    /// See [`daslla`](raw::daslla) for full documentation.
    pub fn daslla(&mut self, handle: i32) -> Result<(i32, i32, i32)> {
        let mut lastc: i32 = Default::default();
        let mut lastd: i32 = Default::default();
        let mut lasti: i32 = Default::default();
        raw::daslla(self, handle, &mut lastc, &mut lastd, &mut lasti)?;
        Ok((lastc, lastd, lasti))
    }

    /// DAS, remove comment records
    ///
    /// Decrease the size of the comment area in a DAS file to reclaim
    /// space freed by the removal of a specified number of comment
    /// records.
    ///
    /// See [`dasrcr`](raw::dasrcr) for full documentation.
    pub fn dasrcr(&mut self, handle: i32, n: i32) -> Result<()> {
        raw::dasrcr(self, handle, n)?;
        Ok(())
    }

    /// DAS, read data, character
    ///
    /// Read character data from a range of DAS logical addresses.
    ///
    /// See [`dasrdc`](raw::dasrdc) for full documentation.
    pub fn dasrdc(
        &mut self,
        handle: i32,
        first: i32,
        last: i32,
        bpos: i32,
        epos: i32,
        data: &mut CharVec,
    ) -> Result<()> {
        raw::dasrdc(self, handle, first, last, bpos, epos, data.as_arg_mut())?;
        Ok(())
    }

    /// DAS, read data, double precision
    ///
    /// Read double precision data from a range of DAS logical addresses.
    ///
    /// Returns `data`.
    ///
    /// See [`dasrdd`](raw::dasrdd) for full documentation.
    pub fn dasrdd(&mut self, handle: i32, first: i32, last: i32) -> Result<Vec<f64>> {
        let mut data: Vec<f64> = vec![Default::default(); (last + 1 - first).max(0) as usize];
        raw::dasrdd(self, handle, first, last, &mut data)?;
        Ok(data)
    }

    /// DAS, read data, integer
    ///
    /// Read integer data from a range of DAS logical addresses.
    ///
    /// Returns `data`.
    ///
    /// See [`dasrdi`](raw::dasrdi) for full documentation.
    pub fn dasrdi(&mut self, handle: i32, first: i32, last: i32) -> Result<Vec<i32>> {
        let mut data: Vec<i32> = vec![Default::default(); (last + 1 - first).max(0) as usize];
        raw::dasrdi(self, handle, first, last, &mut data)?;
        Ok(data)
    }

    /// DAS, read file record
    ///
    /// Return the contents of the file record of a specified DAS
    /// file.
    ///
    /// Returns `(idword, ifname, nresvr, nresvc, ncomr, ncomc)`.
    ///
    /// See [`dasrfr`](raw::dasrfr) for full documentation.
    pub fn dasrfr(&mut self, handle: i32) -> Result<(String, String, i32, i32, i32, i32)> {
        let mut idword = blank(8);
        let mut ifname = blank(60);
        let mut nresvr: i32 = Default::default();
        let mut nresvc: i32 = Default::default();
        let mut ncomr: i32 = Default::default();
        let mut ncomc: i32 = Default::default();
        raw::dasrfr(
            self,
            handle,
            &mut idword,
            &mut ifname,
            &mut nresvr,
            &mut nresvc,
            &mut ncomr,
            &mut ncomc,
        )?;
        Ok((trim(idword), trim(ifname), nresvr, nresvc, ncomr, ncomc))
    }

    /// DAS, read record, double precision
    ///
    /// Read DAS double precision physical records.
    ///
    /// Returns `datad`.
    ///
    /// See [`dasrrd`](raw::dasrrd) for full documentation.
    pub fn dasrrd(&mut self, handle: i32, recno: i32, first: i32, last: i32) -> Result<Vec<f64>> {
        let mut datad: Vec<f64> = vec![Default::default(); (last + 1 - first).max(0) as usize];
        raw::dasrrd(self, handle, recno, first, last, &mut datad)?;
        Ok(datad)
    }

    /// DAS, read record, integer
    ///
    /// Read DAS integer physical records.
    ///
    /// Returns `datai`.
    ///
    /// See [`dasrri`](raw::dasrri) for full documentation.
    pub fn dasrri(&mut self, handle: i32, recno: i32, first: i32, last: i32) -> Result<Vec<i32>> {
        let mut datai: Vec<i32> = vec![Default::default(); (last + 1 - first).max(0) as usize];
        raw::dasrri(self, handle, recno, first, last, &mut datai)?;
        Ok(datai)
    }

    /// DAS, read record, character
    ///
    /// Read DAS character physical records.
    ///
    /// Returns `datac`.
    ///
    /// See [`dasrrc`](raw::dasrrc) for full documentation.
    pub fn dasrrc(&mut self, handle: i32, recno: i32, first: i32, last: i32) -> Result<String> {
        let mut datac = blank((last + 1 - first).max(0));
        raw::dasrrc(self, handle, recno, first, last, &mut datac)?;
        Ok(trim(datac))
    }

    /// DAS, write record, double precision
    ///
    /// Write DAS double precision physical records.
    ///
    /// See [`daswrd`](raw::daswrd) for full documentation.
    pub fn daswrd(&mut self, handle: i32, recno: i32, recd: &[f64; 128]) -> Result<()> {
        raw::daswrd(self, handle, recno, recd)?;
        Ok(())
    }

    /// DAS, write record, integer
    ///
    /// Write DAS integer physical records.
    ///
    /// See [`daswri`](raw::daswri) for full documentation.
    pub fn daswri(&mut self, handle: i32, recno: i32, reci: &[i32; 256]) -> Result<()> {
        raw::daswri(self, handle, recno, reci)?;
        Ok(())
    }

    /// DAS, write record, character
    ///
    /// Write DAS character physical records.
    ///
    /// See [`daswrc`](raw::daswrc) for full documentation.
    pub fn daswrc(&mut self, handle: i32, recno: i32, recc: &str) -> Result<()> {
        raw::daswrc(self, handle, recno, recc)?;
        Ok(())
    }

    /// DAS, update record, double precision
    ///
    /// Update DAS double precision physical records.
    ///
    /// See [`dasurd`](raw::dasurd) for full documentation.
    pub fn dasurd(
        &mut self,
        handle: i32,
        recno: i32,
        first: i32,
        last: i32,
        datad: &[f64],
    ) -> Result<()> {
        raw::dasurd(self, handle, recno, first, last, datad)?;
        Ok(())
    }

    /// DAS, update record, integer
    ///
    /// Update DAS integer physical records.
    ///
    /// See [`dasuri`](raw::dasuri) for full documentation.
    pub fn dasuri(
        &mut self,
        handle: i32,
        recno: i32,
        first: i32,
        last: i32,
        datai: &[i32],
    ) -> Result<()> {
        raw::dasuri(self, handle, recno, first, last, datai)?;
        Ok(())
    }

    /// DAS, update record, character
    ///
    /// Update DAS character physical records.
    ///
    /// See [`dasurc`](raw::dasurc) for full documentation.
    pub fn dasurc(
        &mut self,
        handle: i32,
        recno: i32,
        first: i32,
        last: i32,
        datac: &str,
    ) -> Result<()> {
        raw::dasurc(self, handle, recno, first, last, datac)?;
        Ok(())
    }

    /// DAS, write buffered records
    ///
    /// Write out all buffered records of a specified DAS file.
    ///
    /// See [`daswbr`](raw::daswbr) for full documentation.
    pub fn daswbr(&mut self, handle: i32) -> Result<()> {
        raw::daswbr(self, handle)?;
        Ok(())
    }

    /// DAS, segregate data records
    ///
    /// Segregate the data records in a DAS file into clusters, using
    /// one cluster per data type present in the file.
    ///
    /// See [`dassdr`](raw::dassdr) for full documentation.
    pub fn dassdr(&mut self, handle: i32) -> Result<()> {
        raw::dassdr(self, handle)?;
        Ok(())
    }

    /// DAS, convert transfer file to binary file
    ///
    /// Convert the contents of a DAS transfer file into an equivalent
    /// binary DAS file.
    ///
    /// See [`dastb`](raw::dastb) for full documentation.
    pub fn dastb(&mut self, xfrlun: i32, binfil: &str) -> Result<()> {
        raw::dastb(self, xfrlun, binfil)?;
        Ok(())
    }

    /// DAS, update data, character
    ///
    /// Update character data in a specified range of DAS logical
    /// addresses with substrings of a character array.
    ///
    /// See [`dasudc`](raw::dasudc) for full documentation.
    pub fn dasudc(
        &mut self,
        handle: i32,
        first: i32,
        last: i32,
        bpos: i32,
        epos: i32,
        data: &CharVec,
    ) -> Result<()> {
        raw::dasudc(self, handle, first, last, bpos, epos, data.as_arg())?;
        Ok(())
    }

    /// DAS, update data, double precision
    ///
    /// Update data in a specified range of double precision addresses
    /// in a DAS file.
    ///
    /// See [`dasudd`](raw::dasudd) for full documentation.
    pub fn dasudd(&mut self, handle: i32, first: i32, last: i32, data: &[f64]) -> Result<()> {
        raw::dasudd(self, handle, first, last, data)?;
        Ok(())
    }

    /// DAS, update data, integer
    ///
    /// Update data in a specified range of integer addresses in a DAS
    /// file.
    ///
    /// See [`dasudi`](raw::dasudi) for full documentation.
    pub fn dasudi(&mut self, handle: i32, first: i32, last: i32, data: &[i32]) -> Result<()> {
        raw::dasudi(self, handle, first, last, data)?;
        Ok(())
    }

    /// DAS write file record
    ///
    /// Update the contents of the file record of a specified DAS file.
    ///
    /// See [`daswfr`](raw::daswfr) for full documentation.
    pub fn daswfr(
        &mut self,
        handle: i32,
        idword: &str,
        ifname: &str,
        nresvr: i32,
        nresvc: i32,
        ncomr: i32,
        ncomc: i32,
    ) -> Result<()> {
        raw::daswfr(self, handle, idword, ifname, nresvr, nresvc, ncomr, ncomc)?;
        Ok(())
    }

    /// Double precision arc hyperbolic tangent
    ///
    /// Return the inverse hyperbolic tangent of a double precision
    /// argument.
    ///
    /// See [`datanh`](raw::datanh) for full documentation.
    pub fn datanh(&mut self, x: f64) -> Result<f64> {
        raw::datanh(self, x)
    }

    /// Derivative of AZ/EL w.r.t. rectangular
    ///
    /// Compute the Jacobian matrix of the transformation from
    /// rectangular to azimuth/elevation coordinates.
    ///
    /// Returns `jacobi`.
    ///
    /// See [`dazldr`](raw::dazldr) for full documentation.
    pub fn dazldr(
        &mut self,
        x: f64,
        y: f64,
        z: f64,
        azccw: bool,
        elplsz: bool,
    ) -> Result<[[f64; 3]; 3]> {
        let mut jacobi: [[f64; 3]; 3] = Default::default();
        raw::dazldr(self, x, y, z, azccw, elplsz, &mut jacobi)?;
        Ok(jacobi)
    }

    /// Double precision cube root
    ///
    /// Return the cube root of a double precision number.
    ///
    /// See [`dcbrt`](raw::dcbrt) for full documentation.
    pub fn dcbrt(&self, x: f64) -> f64 {
        raw::dcbrt(x)
    }

    /// Derivative of cylindrical w.r.t. rectangular
    ///
    /// Compute the Jacobian matrix of the transformation from
    /// rectangular to cylindrical coordinates.
    ///
    /// Returns `jacobi`.
    ///
    /// See [`dcyldr`](raw::dcyldr) for full documentation.
    pub fn dcyldr(&mut self, x: f64, y: f64, z: f64) -> Result<[[f64; 3]; 3]> {
        let mut jacobi: [[f64; 3]; 3] = Default::default();
        raw::dcyldr(self, x, y, z, &mut jacobi)?;
        Ok(jacobi)
    }

    /// Delete a file
    ///
    /// Delete a file.
    ///
    /// See [`delfil`](raw::delfil) for full documentation.
    pub fn delfil(&mut self, filnam: &str) -> Result<()> {
        raw::delfil(self, filnam)?;
        Ok(())
    }

    /// Delta ET, ET - UTC
    ///
    /// Return the value of Delta ET (ET-UTC) for an input epoch.
    ///
    /// Returns `delta`.
    ///
    /// See [`deltet`](raw::deltet) for full documentation.
    pub fn deltet(&mut self, epoch: f64, eptype: &str) -> Result<f64> {
        let mut delta: f64 = Default::default();
        raw::deltet(self, epoch, eptype, &mut delta)?;
        Ok(delta)
    }

    /// Determinant of a double precision 3x3 matrix
    ///
    /// Compute the determinant of a double precision 3x3 matrix.
    ///
    /// See [`det`](raw::det) for full documentation.
    pub fn det(&self, m1: &[[f64; 3]; 3]) -> f64 {
        raw::det(m1)
    }

    /// Derivative of geodetic w.r.t. rectangular
    ///
    /// Compute the Jacobian matrix of the transformation from
    /// rectangular to geodetic coordinates.
    ///
    /// Returns `jacobi`.
    ///
    /// See [`dgeodr`](raw::dgeodr) for full documentation.
    pub fn dgeodr(&mut self, x: f64, y: f64, z: f64, re: f64, f: f64) -> Result<[[f64; 3]; 3]> {
        let mut jacobi: [[f64; 3]; 3] = Default::default();
        raw::dgeodr(self, x, y, z, re, f, &mut jacobi)?;
        Ok(jacobi)
    }

    /// Time derivative of half angle
    ///
    /// Calculate the value of the time derivative of the
    /// half angle of a spherical body given a state vector
    /// STATE and body radius BODYR.
    ///
    /// See [`dhfa`](raw::dhfa) for full documentation.
    pub fn dhfa(&mut self, state: &[f64; 6], bodyr: f64) -> Result<f64> {
        raw::dhfa(self, state, bodyr)
    }

    /// Diagonalize symmetric 2x2 matrix
    ///
    /// Diagonalize a symmetric 2x2 matrix.
    ///
    /// Returns `(diag, rotate)`.
    ///
    /// See [`diags2`](raw::diags2) for full documentation.
    pub fn diags2(&mut self, symmat: &[[f64; 2]; 2]) -> Result<([[f64; 2]; 2], [[f64; 2]; 2])> {
        let mut diag: [[f64; 2]; 2] = Default::default();
        let mut rotate: [[f64; 2]; 2] = Default::default();
        raw::diags2(self, symmat, &mut diag, &mut rotate)?;
        Ok((diag, rotate))
    }

    /// Difference of two character sets
    ///
    /// Take the difference of two character sets to form a third set.
    ///
    /// See [`diffc`](raw::diffc) for full documentation.
    pub fn diffc(&mut self, a: &CharCell, b: &CharCell, c: &mut CharCell) -> Result<()> {
        raw::diffc(self, a.as_arg(), b.as_arg(), c.as_arg_mut())?;
        Ok(())
    }

    /// Difference of two double precision sets
    ///
    /// Take the difference of two double precision sets to form
    /// a third set.
    ///
    /// See [`diffd`](raw::diffd) for full documentation.
    pub fn diffd(&mut self, a: &Cell<f64>, b: &Cell<f64>, c: &mut Cell<f64>) -> Result<()> {
        raw::diffd(
            self,
            a.as_raw_slice(),
            b.as_raw_slice(),
            c.as_raw_mut_slice(),
        )?;
        Ok(())
    }

    /// Difference of two integer sets
    ///
    /// Take the difference of two integer sets to form a third set.
    ///
    /// See [`diffi`](raw::diffi) for full documentation.
    pub fn diffi(&mut self, a: &Cell<i32>, b: &Cell<i32>, c: &mut Cell<i32>) -> Result<()> {
        raw::diffi(
            self,
            a.as_raw_slice(),
            b.as_raw_slice(),
            c.as_raw_mut_slice(),
        )?;
        Ok(())
    }

    /// DLA, begin backward search
    ///
    /// Begin a backward segment search in a DLA file.
    ///
    /// Returns `dladsc`.
    ///
    /// See [`dlabbs`](raw::dlabbs) for full documentation.
    pub fn dlabbs(&mut self, handle: i32) -> Result<Option<Vec<i32>>> {
        let mut dladsc: Vec<i32> = vec![Default::default(); inc::dla::DLADSZ as usize];
        let mut found: bool = Default::default();
        raw::dlabbs(self, handle, &mut dladsc, &mut found)?;
        Ok(if found { Some(dladsc) } else { None })
    }

    /// DLA, begin forward search
    ///
    /// Begin a forward segment search in a DLA file.
    ///
    /// Returns `dladsc`.
    ///
    /// See [`dlabfs`](raw::dlabfs) for full documentation.
    pub fn dlabfs(&mut self, handle: i32) -> Result<Option<Vec<i32>>> {
        let mut dladsc: Vec<i32> = vec![Default::default(); inc::dla::DLADSZ as usize];
        let mut found: bool = Default::default();
        raw::dlabfs(self, handle, &mut dladsc, &mut found)?;
        Ok(if found { Some(dladsc) } else { None })
    }

    /// DLA, begin new segment
    ///
    /// Begin a new segment in a DLA file.
    ///
    /// See [`dlabns`](raw::dlabns) for full documentation.
    pub fn dlabns(&mut self, handle: i32) -> Result<()> {
        raw::dlabns(self, handle)?;
        Ok(())
    }

    /// DLA, end new segment
    ///
    /// End a new segment in a DLA file.
    ///
    /// See [`dlaens`](raw::dlaens) for full documentation.
    pub fn dlaens(&mut self, handle: i32) -> Result<()> {
        raw::dlaens(self, handle)?;
        Ok(())
    }

    /// DLA, find next segment
    ///
    /// Find the segment following a specified segment in a DLA file.
    ///
    /// Returns `nxtdsc`.
    ///
    /// See [`dlafns`](raw::dlafns) for full documentation.
    pub fn dlafns(&mut self, handle: i32, dladsc: &[i32]) -> Result<Option<Vec<i32>>> {
        let mut nxtdsc: Vec<i32> = vec![Default::default(); inc::dla::DLADSZ as usize];
        let mut found: bool = Default::default();
        raw::dlafns(self, handle, dladsc, &mut nxtdsc, &mut found)?;
        Ok(if found { Some(nxtdsc) } else { None })
    }

    /// DLA, find previous segment
    ///
    /// Find the segment preceding a specified segment in a DLA file.
    ///
    /// Returns `prvdsc`.
    ///
    /// See [`dlafps`](raw::dlafps) for full documentation.
    pub fn dlafps(&mut self, handle: i32, dladsc: &[i32]) -> Result<Option<Vec<i32>>> {
        let mut prvdsc: Vec<i32> = vec![Default::default(); inc::dla::DLADSZ as usize];
        let mut found: bool = Default::default();
        raw::dlafps(self, handle, dladsc, &mut prvdsc, &mut found)?;
        Ok(if found { Some(prvdsc) } else { None })
    }

    /// DLA, open new file
    ///
    /// Open a new DLA file and set the file type.
    ///
    /// Returns `handle`.
    ///
    /// See [`dlaopn`](raw::dlaopn) for full documentation.
    pub fn dlaopn(&mut self, fname: &str, ftype: &str, ifname: &str, ncomch: i32) -> Result<i32> {
        let mut handle: i32 = Default::default();
        raw::dlaopn(self, fname, ftype, ifname, ncomch, &mut handle)?;
        Ok(handle)
    }

    /// DLA, same segment?
    ///
    /// Return a logical value indicating whether a two DLA
    /// segments, each identified by DAS handle and DLA descriptor,
    /// are in fact the same segment.
    ///
    /// See [`dlassg`](raw::dlassg) for full documentation.
    pub fn dlassg(&self, han1: i32, han2: i32, dsc1: &[i32; 8], dsc2: &[i32; 8]) -> bool {
        raw::dlassg(han1, han2, dsc1, dsc2)
    }

    /// Derivative of latitudinal w.r.t. rectangular
    ///
    /// Compute the Jacobian matrix of the transformation from
    /// rectangular to latitudinal coordinates.
    ///
    /// Returns `jacobi`.
    ///
    /// See [`dlatdr`](raw::dlatdr) for full documentation.
    pub fn dlatdr(&mut self, x: f64, y: f64, z: f64) -> Result<[[f64; 3]; 3]> {
        let mut jacobi: [[f64; 3]; 3] = Default::default();
        raw::dlatdr(self, x, y, z, &mut jacobi)?;
        Ok(jacobi)
    }

    /// Derivative of near point
    ///
    /// Compute the state (position and velocity) of an ellipsoid surface
    /// point nearest to the position component of a specified state.
    ///
    /// Returns `(dnear, dalt)`.
    ///
    /// See [`dnearp`](raw::dnearp) for full documentation.
    pub fn dnearp(
        &mut self,
        state: &[f64; 6],
        a: f64,
        b: f64,
        c: f64,
    ) -> Result<Option<([f64; 6], [f64; 2])>> {
        let mut dnear: [f64; 6] = Default::default();
        let mut dalt: [f64; 2] = Default::default();
        let mut found: bool = Default::default();
        raw::dnearp(self, state, a, b, c, &mut dnear, &mut dalt, &mut found)?;
        Ok(if found { Some((dnear, dalt)) } else { None })
    }

    /// D.p. number to hexadecimal string
    ///
    /// Convert a double precision number to an equivalent character
    /// string using a base 16 "scientific notation."
    ///
    /// Returns `(hxstr, hxssiz)`.
    ///
    /// See [`dp2hx`](raw::dp2hx) for full documentation.
    pub fn dp2hx(&mut self, number: f64) -> (String, i32) {
        let mut hxstr = blank(255);
        let mut hxssiz: i32 = Default::default();
        raw::dp2hx(self, number, &mut hxstr, &mut hxssiz);
        (trim(hxstr), hxssiz)
    }

    /// Format a double precision number
    ///
    /// Create a formatted string that represents a double precision
    /// number, using a format picture.
    ///
    /// Returns `str`.
    ///
    /// See [`dpfmt`](raw::dpfmt) for full documentation.
    pub fn dpfmt(&mut self, x: f64, pictur: &str) -> Result<String> {
        let mut str = blank((pictur.len() as i32));
        raw::dpfmt(self, x, pictur, &mut str)?;
        Ok(trim(str))
    }

    /// Derivative of planetographic w.r.t. rectangular
    ///
    /// Compute the Jacobian matrix of the transformation from
    /// rectangular to planetographic coordinates.
    ///
    /// Returns `jacobi`.
    ///
    /// See [`dpgrdr`](raw::dpgrdr) for full documentation.
    pub fn dpgrdr(
        &mut self,
        body: &str,
        x: f64,
        y: f64,
        z: f64,
        re: f64,
        f: f64,
    ) -> Result<[[f64; 3]; 3]> {
        let mut jacobi: [[f64; 3]; 3] = Default::default();
        raw::dpgrdr(self, body, x, y, z, re, f, &mut jacobi)?;
        Ok(jacobi)
    }

    /// Largest DP number
    ///
    /// Return the value of the largest (positive) number representable
    /// in a double precision variable.
    ///
    /// See [`dpmax`](raw::dpmax) for full documentation.
    pub fn dpmax(&self) -> f64 {
        raw::dpmax()
    }

    /// Smallest DP number
    ///
    /// Return the value of the smallest (negative) number representable
    /// in a double precision variable.
    ///
    /// See [`dpmin`](raw::dpmin) for full documentation.
    pub fn dpmin(&self) -> f64 {
        raw::dpmin()
    }

    /// Degrees per radian
    ///
    /// Return the number of degrees per radian.
    ///
    /// See [`dpr`](raw::dpr) for full documentation.
    pub fn dpr(&mut self) -> f64 {
        raw::dpr(self)
    }

    /// Double Precision Number to Character
    ///
    /// Take a double precision number and convert it to
    /// an equivalent character string representation (base 10).
    ///
    /// Returns `string`.
    ///
    /// See [`dpstr`](raw::dpstr) for full documentation.
    pub fn dpstr(&mut self, x: f64, sigdig: i32) -> String {
        let mut string = blank((sigdig + 6).max(0));
        raw::dpstr(self, x, sigdig, &mut string);
        trim(string)
    }

    /// Double Precision Number to Character
    ///
    /// Take a double precision number and convert it to an
    /// equivalent formatted character string representation (base 10).
    ///
    /// Returns `string`.
    ///
    /// See [`dpstrf`](raw::dpstrf) for full documentation.
    pub fn dpstrf(&mut self, x: f64, sigdig: i32, format: char) -> String {
        let mut string = blank((sigdig + 6).max(0));
        raw::dpstrf(self, x, sigdig, format, &mut string);
        trim(string)
    }

    /// Derivative of rectangular w.r.t. AZ/EL
    ///
    /// Compute the Jacobian matrix of the transformation from
    /// azimuth/elevation to rectangular coordinates.
    ///
    /// Returns `jacobi`.
    ///
    /// See [`drdazl`](raw::drdazl) for full documentation.
    pub fn drdazl(
        &mut self,
        range: f64,
        az: f64,
        el: f64,
        azccw: bool,
        elplsz: bool,
    ) -> Result<[[f64; 3]; 3]> {
        let mut jacobi: [[f64; 3]; 3] = Default::default();
        raw::drdazl(self, range, az, el, azccw, elplsz, &mut jacobi)?;
        Ok(jacobi)
    }

    /// Derivative of rectangular w.r.t. cylindrical
    ///
    /// Compute the Jacobian matrix of the transformation from
    /// cylindrical to rectangular coordinates.
    ///
    /// Returns `jacobi`.
    ///
    /// See [`drdcyl`](raw::drdcyl) for full documentation.
    pub fn drdcyl(&self, r: f64, clon: f64, z: f64) -> [[f64; 3]; 3] {
        let mut jacobi: [[f64; 3]; 3] = Default::default();
        raw::drdcyl(r, clon, z, &mut jacobi);
        jacobi
    }

    /// Derivative of rectangular w.r.t. geodetic
    ///
    /// Compute the Jacobian matrix of the transformation from geodetic
    /// to rectangular coordinates.
    ///
    /// Returns `jacobi`.
    ///
    /// See [`drdgeo`](raw::drdgeo) for full documentation.
    pub fn drdgeo(
        &mut self,
        lon: f64,
        lat: f64,
        alt: f64,
        re: f64,
        f: f64,
    ) -> Result<[[f64; 3]; 3]> {
        let mut jacobi: [[f64; 3]; 3] = Default::default();
        raw::drdgeo(self, lon, lat, alt, re, f, &mut jacobi)?;
        Ok(jacobi)
    }

    /// Derivative of rectangular w.r.t. latitudinal
    ///
    /// Compute the Jacobian matrix of the transformation from
    /// latitudinal to rectangular coordinates.
    ///
    /// Returns `jacobi`.
    ///
    /// See [`drdlat`](raw::drdlat) for full documentation.
    pub fn drdlat(&self, r: f64, lon: f64, lat: f64) -> [[f64; 3]; 3] {
        let mut jacobi: [[f64; 3]; 3] = Default::default();
        raw::drdlat(r, lon, lat, &mut jacobi);
        jacobi
    }

    /// Derivative of rectangular w.r.t. planetographic
    ///
    /// Compute the Jacobian matrix of the transformation from
    /// planetographic to rectangular coordinates.
    ///
    /// Returns `jacobi`.
    ///
    /// See [`drdpgr`](raw::drdpgr) for full documentation.
    pub fn drdpgr(
        &mut self,
        body: &str,
        lon: f64,
        lat: f64,
        alt: f64,
        re: f64,
        f: f64,
    ) -> Result<[[f64; 3]; 3]> {
        let mut jacobi: [[f64; 3]; 3] = Default::default();
        raw::drdpgr(self, body, lon, lat, alt, re, f, &mut jacobi)?;
        Ok(jacobi)
    }

    /// Derivative of rectangular w.r.t. spherical
    ///
    /// Compute the Jacobian matrix of the transformation from spherical
    /// to rectangular coordinates.
    ///
    /// Returns `jacobi`.
    ///
    /// See [`drdsph`](raw::drdsph) for full documentation.
    pub fn drdsph(&self, r: f64, colat: f64, slon: f64) -> [[f64; 3]; 3] {
        let mut jacobi: [[f64; 3]; 3] = Default::default();
        raw::drdsph(r, colat, slon, &mut jacobi);
        jacobi
    }

    /// Derivative of a rotation matrix
    ///
    /// Calculate the derivative with respect to the angle of rotation
    /// of a 3x3 coordinate system rotation matrix generated by a
    /// rotation of a specified angle about a specified axis.
    ///
    /// Returns `dmout`.
    ///
    /// See [`drotat`](raw::drotat) for full documentation.
    pub fn drotat(&mut self, angle: f64, iaxis: i32) -> Result<[[f64; 3]; 3]> {
        let mut dmout: [[f64; 3]; 3] = Default::default();
        raw::drotat(self, angle, iaxis, &mut dmout)?;
        Ok(dmout)
    }

    /// DSK, fetch type 2 bookkeeping data
    ///
    /// Return bookkeeping data from a DSK type 2 segment.
    ///
    /// Returns `(nv, np, nvxtot, vtxbds, voxsiz, voxori, vgrext, cgscal, vtxnpl, voxnpt, voxnpl)`.
    ///
    /// See [`dskb02`](raw::dskb02) for full documentation.
    pub fn dskb02(
        &mut self,
        handle: i32,
        dladsc: &[i32],
    ) -> Result<(
        i32,
        i32,
        i32,
        [[f64; 2]; 3],
        f64,
        [f64; 3],
        [i32; 3],
        i32,
        i32,
        i32,
        i32,
    )> {
        let mut nv: i32 = Default::default();
        let mut np: i32 = Default::default();
        let mut nvxtot: i32 = Default::default();
        let mut vtxbds: [[f64; 2]; 3] = Default::default();
        let mut voxsiz: f64 = Default::default();
        let mut voxori: [f64; 3] = Default::default();
        let mut vgrext: [i32; 3] = Default::default();
        let mut cgscal: i32 = Default::default();
        let mut vtxnpl: i32 = Default::default();
        let mut voxnpt: i32 = Default::default();
        let mut voxnpl: i32 = Default::default();
        raw::dskb02(
            self,
            handle,
            dladsc,
            &mut nv,
            &mut np,
            &mut nvxtot,
            &mut vtxbds,
            &mut voxsiz,
            &mut voxori,
            &mut vgrext,
            &mut cgscal,
            &mut vtxnpl,
            &mut voxnpt,
            &mut voxnpl,
        )?;
        Ok((
            nv, np, nvxtot, vtxbds, voxsiz, voxori, vgrext, cgscal, vtxnpl, voxnpt, voxnpl,
        ))
    }

    /// DSK, close file
    ///
    /// Close a DSK file.
    ///
    /// See [`dskcls`](raw::dskcls) for full documentation.
    pub fn dskcls(&mut self, handle: i32, optmiz: bool) -> Result<()> {
        raw::dskcls(self, handle, optmiz)?;
        Ok(())
    }

    /// DSK, fetch d.p. type 2 data
    ///
    /// Fetch double precision data from a type 2 DSK segment.
    ///
    /// Returns `(n, values)`.
    ///
    /// See [`dskd02`](raw::dskd02) for full documentation.
    pub fn dskd02(
        &mut self,
        handle: i32,
        dladsc: &[i32],
        item: i32,
        start: i32,
        room: i32,
    ) -> Result<(i32, Vec<f64>)> {
        let mut n: i32 = Default::default();
        let mut values: Vec<f64> = vec![Default::default(); room.max(0) as usize];
        raw::dskd02(self, handle, dladsc, item, start, room, &mut n, &mut values)?;
        Ok((n, values))
    }

    /// DSK, return DSK segment descriptor
    ///
    /// Return the DSK descriptor from a DSK segment identified
    /// by a DAS handle and DLA descriptor.
    ///
    /// Returns `dskdsc`.
    ///
    /// See [`dskgd`](raw::dskgd) for full documentation.
    pub fn dskgd(&mut self, handle: i32, dladsc: &[i32]) -> Result<Vec<f64>> {
        let mut dskdsc: Vec<f64> = vec![Default::default(); inc::dla::DLADSZ as usize];
        raw::dskgd(self, handle, dladsc, &mut dskdsc)?;
        Ok(dskdsc)
    }

    /// DSK, fetch integer type 2 data
    ///
    /// Fetch integer data from a type 2 DSK segment.
    ///
    /// Returns `(n, values)`.
    ///
    /// See [`dski02`](raw::dski02) for full documentation.
    pub fn dski02(
        &mut self,
        handle: i32,
        dladsc: &[i32],
        item: i32,
        start: i32,
        room: i32,
    ) -> Result<(i32, Vec<i32>)> {
        let mut n: i32 = Default::default();
        let mut values: Vec<i32> = vec![Default::default(); room.max(0) as usize];
        raw::dski02(self, handle, dladsc, item, start, room, &mut n, &mut values)?;
        Ok((n, values))
    }

    /// DSK, make spatial index for type 2 segment
    ///
    /// Make spatial index for a DSK type 2 segment. The index is
    /// returned as a pair of arrays, one of type INTEGER and one of type
    /// DOUBLE PRECISION. These arrays are suitable for use with the DSK
    /// type 2 writer DSKW02.
    ///
    /// Returns `(spaixd, spaixi)`.
    ///
    /// See [`dskmi2`](raw::dskmi2) for full documentation.
    pub fn dskmi2(
        &mut self,
        nv: i32,
        vrtces: &[[f64; 3]],
        np: i32,
        plates: &[[i32; 3]],
        finscl: f64,
        corscl: i32,
        worksz: i32,
        voxpsz: i32,
        voxlsz: i32,
        makvtl: bool,
        spxisz: i32,
    ) -> Result<([f64; 10], Vec<i32>)> {
        let mut work: Vec<[i32; 2]> = vec![Default::default(); worksz.max(0) as usize];
        let mut spaixd: [f64; 10] = Default::default();
        let mut spaixi: Vec<i32> = vec![Default::default(); spxisz.max(0) as usize];
        raw::dskmi2(
            self,
            nv,
            vrtces,
            np,
            plates,
            finscl,
            corscl,
            worksz,
            voxpsz,
            voxlsz,
            makvtl,
            spxisz,
            &mut work,
            &mut spaixd,
            &mut spaixi,
        )?;
        Ok((spaixd, spaixi))
    }

    /// DSK, type 2, compute normal vector for plate
    ///
    /// Compute the unit normal vector for a specified plate from a type
    /// 2 DSK segment.
    ///
    /// Returns `normal`.
    ///
    /// See [`dskn02`](raw::dskn02) for full documentation.
    pub fn dskn02(&mut self, handle: i32, dladsc: &[i32], plid: i32) -> Result<[f64; 3]> {
        let mut normal: [f64; 3] = Default::default();
        raw::dskn02(self, handle, dladsc, plid, &mut normal)?;
        Ok(normal)
    }

    /// DSK, get object IDs
    ///
    /// Find the set of body ID codes of all objects for which
    /// topographic data are provided in a specified DSK file.
    ///
    /// See [`dskobj`](raw::dskobj) for full documentation.
    pub fn dskobj(&mut self, dskfnm: &str, bodids: &mut Cell<i32>) -> Result<()> {
        raw::dskobj(self, dskfnm, bodids.as_raw_mut_slice())?;
        Ok(())
    }

    /// DSK, open new file
    ///
    /// Open a new DSK file for subsequent write operations.
    ///
    /// Returns `handle`.
    ///
    /// See [`dskopn`](raw::dskopn) for full documentation.
    pub fn dskopn(&mut self, fname: &str, ifname: &str, ncomch: i32) -> Result<i32> {
        let mut handle: i32 = Default::default();
        raw::dskopn(self, fname, ifname, ncomch, &mut handle)?;
        Ok(handle)
    }

    /// DSK, fetch type 2 plate data
    ///
    /// Fetch triangular plates from a type 2 DSK segment.
    ///
    /// Returns `(n, plates)`.
    ///
    /// See [`dskp02`](raw::dskp02) for full documentation.
    pub fn dskp02(
        &mut self,
        handle: i32,
        dladsc: &[i32],
        start: i32,
        room: i32,
    ) -> Result<(i32, Vec<[i32; 3]>)> {
        let mut n: i32 = Default::default();
        let mut plates: Vec<[i32; 3]> = vec![Default::default(); room.max(0) as usize];
        raw::dskp02(self, handle, dladsc, start, room, &mut n, &mut plates)?;
        Ok((n, plates))
    }

    /// DSK, determine range bounds for plate set
    ///
    /// Determine range bounds for a set of triangular plates to
    /// be stored in a type 2 DSK segment.
    ///
    /// Returns `(mncor3, mxcor3)`.
    ///
    /// See [`dskrb2`](raw::dskrb2) for full documentation.
    pub fn dskrb2(
        &mut self,
        nv: i32,
        vrtces: &[[f64; 3]],
        np: i32,
        plates: &[[i32; 3]],
        corsys: i32,
        corpar: &[f64],
    ) -> Result<(f64, f64)> {
        let mut mncor3: f64 = Default::default();
        let mut mxcor3: f64 = Default::default();
        raw::dskrb2(
            self,
            nv,
            vrtces,
            np,
            plates,
            corsys,
            corpar,
            &mut mncor3,
            &mut mxcor3,
        )?;
        Ok((mncor3, mxcor3))
    }

    /// DSK, get surface IDs for body
    ///
    /// Find the set of surface ID codes for all surfaces associated with
    /// a given body in a specified DSK file.
    ///
    /// See [`dsksrf`](raw::dsksrf) for full documentation.
    pub fn dsksrf(&mut self, dskfnm: &str, bodyid: i32, srfids: &mut Cell<i32>) -> Result<()> {
        raw::dsksrf(self, dskfnm, bodyid, srfids.as_raw_mut_slice())?;
        Ok(())
    }

    /// DSK, get tolerance
    ///
    /// Retrieve the value of a specified DSK tolerance or margin
    /// parameter.
    ///
    /// Returns `dpval`.
    ///
    /// See [`dskgtl`](raw::dskgtl) for full documentation.
    pub fn dskgtl(&mut self, keywrd: i32) -> Result<f64> {
        let mut dpval: f64 = Default::default();
        raw::dskgtl(self, keywrd, &mut dpval)?;
        Ok(dpval)
    }

    /// DSK, set tolerance
    ///
    /// Set the value of a specified DSK tolerance or margin parameter.
    ///
    /// See [`dskstl`](raw::dskstl) for full documentation.
    pub fn dskstl(&mut self, keywrd: i32, dpval: f64) -> Result<()> {
        raw::dskstl(self, keywrd, dpval)?;
        Ok(())
    }

    /// DSK, fetch type 2 vertex data
    ///
    /// Fetch vertices from a type 2 DSK segment.
    ///
    /// Returns `(n, vrtces)`.
    ///
    /// See [`dskv02`](raw::dskv02) for full documentation.
    pub fn dskv02(
        &mut self,
        handle: i32,
        dladsc: &[i32],
        start: i32,
        room: i32,
    ) -> Result<(i32, Vec<[f64; 3]>)> {
        let mut n: i32 = Default::default();
        let mut vrtces: Vec<[f64; 3]> = vec![Default::default(); room.max(0) as usize];
        raw::dskv02(self, handle, dladsc, start, room, &mut n, &mut vrtces)?;
        Ok((n, vrtces))
    }

    /// DSK, write type 2 segment
    ///
    /// Write a type 2 segment to a DSK file.
    ///
    /// See [`dskw02`](raw::dskw02) for full documentation.
    pub fn dskw02(
        &mut self,
        handle: i32,
        center: i32,
        surfid: i32,
        dclass: i32,
        frame: &str,
        corsys: i32,
        corpar: &[f64],
        mncor1: f64,
        mxcor1: f64,
        mncor2: f64,
        mxcor2: f64,
        mncor3: f64,
        mxcor3: f64,
        first: f64,
        last: f64,
        nv: i32,
        vrtces: &[[f64; 3]],
        np: i32,
        plates: &[[i32; 3]],
        spaixd: &[f64],
        spaixi: &[i32],
    ) -> Result<()> {
        raw::dskw02(
            self, handle, center, surfid, dclass, frame, corsys, corpar, mncor1, mxcor1, mncor2,
            mxcor2, mncor3, mxcor3, first, last, nv, vrtces, np, plates, spaixd, spaixi,
        )?;
        Ok(())
    }

    /// DSK, ray-surface intercept, type 2
    ///
    /// Determine the plate ID and body-fixed coordinates of the
    /// intersection of a specified ray with the surface defined by a
    /// type 2 DSK plate model.
    ///
    /// Returns `(plid, xpt)`.
    ///
    /// See [`dskx02`](raw::dskx02) for full documentation.
    pub fn dskx02(
        &mut self,
        handle: i32,
        dladsc: &[i32],
        vertex: &[f64; 3],
        raydir: &[f64; 3],
    ) -> Result<Option<(i32, [f64; 3])>> {
        let mut plid: i32 = Default::default();
        let mut xpt: [f64; 3] = Default::default();
        let mut found: bool = Default::default();
        raw::dskx02(
            self, handle, dladsc, vertex, raydir, &mut plid, &mut xpt, &mut found,
        )?;
        Ok(if found { Some((plid, xpt)) } else { None })
    }

    /// DSK, ray-surface intercept with source information
    ///
    /// Compute a ray-surface intercept using data provided by
    /// multiple loaded DSK segments. Return information about
    /// the source of the data defining the surface on which the
    /// intercept was found: DSK handle, DLA and DSK descriptors,
    /// and DSK data type-dependent parameters.
    ///
    /// Returns `(xpt, handle, dladsc, dskdsc, dc, ic)`.
    ///
    /// See [`dskxsi`](raw::dskxsi) for full documentation.
    pub fn dskxsi(
        &mut self,
        pri: bool,
        target: &str,
        nsurf: i32,
        srflst: &[i32],
        et: f64,
        fixref: &str,
        vertex: &[f64; 3],
        raydir: &[f64; 3],
        maxd: i32,
        maxi: i32,
    ) -> Result<Option<([f64; 3], i32, Vec<i32>, Vec<f64>, Vec<f64>, Vec<i32>)>> {
        let mut xpt: [f64; 3] = Default::default();
        let mut handle: i32 = Default::default();
        let mut dladsc: Vec<i32> = vec![Default::default(); inc::dla::DLADSZ as usize];
        let mut dskdsc: Vec<f64> = vec![Default::default(); inc::dla::DLADSZ as usize];
        let mut dc: Vec<f64> = vec![Default::default(); inc::dsksrc::DCSIZE as usize];
        let mut ic: Vec<i32> = vec![Default::default(); inc::dsksrc::ICSIZE as usize];
        let mut found: bool = Default::default();
        raw::dskxsi(
            self,
            pri,
            target,
            nsurf,
            srflst,
            et,
            fixref,
            vertex,
            raydir,
            maxd,
            maxi,
            &mut xpt,
            &mut handle,
            &mut dladsc,
            &mut dskdsc,
            &mut dc,
            &mut ic,
            &mut found,
        )?;
        Ok(if found {
            Some((xpt, handle, dladsc, dskdsc, dc, ic))
        } else {
            None
        })
    }

    /// DSK, ray-surface intercept, vectorized
    ///
    /// Compute ray-surface intercepts for a set of rays, using data
    /// provided by multiple loaded DSK segments.
    ///
    /// Returns `(xptarr, fndarr)`.
    ///
    /// See [`dskxv`](raw::dskxv) for full documentation.
    pub fn dskxv(
        &mut self,
        pri: bool,
        target: &str,
        nsurf: i32,
        srflst: &[i32],
        et: f64,
        fixref: &str,
        nrays: i32,
        vtxarr: &[[f64; 3]],
        dirarr: &[[f64; 3]],
    ) -> Result<(Vec<[f64; 3]>, Vec<bool>)> {
        let mut xptarr: Vec<[f64; 3]> = vec![Default::default(); nrays.max(0) as usize];
        let mut fndarr: Vec<bool> = vec![Default::default(); nrays.max(0) as usize];
        raw::dskxv(
            self,
            pri,
            target,
            nsurf,
            srflst,
            et,
            fixref,
            nrays,
            vtxarr,
            dirarr,
            &mut xptarr,
            &mut fndarr,
        )?;
        Ok((xptarr, fndarr))
    }

    /// DSK, fetch type 2 model size parameters
    ///
    /// Return plate model size parameters---plate count and
    /// vertex count---for a type 2 DSK segment.
    ///
    /// Returns `(nv, np)`.
    ///
    /// See [`dskz02`](raw::dskz02) for full documentation.
    pub fn dskz02(&mut self, handle: i32, dladsc: &[i32]) -> Result<(i32, i32)> {
        let mut nv: i32 = Default::default();
        let mut np: i32 = Default::default();
        raw::dskz02(self, handle, dladsc, &mut nv, &mut np)?;
        Ok((nv, np))
    }

    /// Derivative of spherical w.r.t. rectangular
    ///
    /// Compute the Jacobian matrix of the transformation from
    /// rectangular to spherical coordinates.
    ///
    /// Returns `jacobi`.
    ///
    /// See [`dsphdr`](raw::dsphdr) for full documentation.
    pub fn dsphdr(&mut self, x: f64, y: f64, z: f64) -> Result<[[f64; 3]; 3]> {
        let mut jacobi: [[f64; 3]; 3] = Default::default();
        raw::dsphdr(self, x, y, z, &mut jacobi)?;
        Ok(jacobi)
    }

    /// Unit Normalized Cross Product and Derivative
    ///
    /// Compute the unit vector parallel to the cross product of
    /// two 3-dimensional vectors and the derivative of this unit vector.
    ///
    /// Returns `sout`.
    ///
    /// See [`ducrss`](raw::ducrss) for full documentation.
    pub fn ducrss(&self, s1: &[f64; 6], s2: &[f64; 6]) -> [f64; 6] {
        let mut sout: [f64; 6] = Default::default();
        raw::ducrss(s1, s2, &mut sout);
        sout
    }

    /// Derivative of Vector cross product
    ///
    /// Compute the cross product of two 3-dimensional vectors
    /// and the derivative of this cross product.
    ///
    /// Returns `sout`.
    ///
    /// See [`dvcrss`](raw::dvcrss) for full documentation.
    pub fn dvcrss(&self, s1: &[f64; 6], s2: &[f64; 6]) -> [f64; 6] {
        let mut sout: [f64; 6] = Default::default();
        raw::dvcrss(s1, s2, &mut sout);
        sout
    }

    /// Derivative of Vector Dot Product, 3-D
    ///
    /// Compute the derivative of the dot product of two double
    /// precision position vectors.
    ///
    /// See [`dvdot`](raw::dvdot) for full documentation.
    pub fn dvdot(&self, s1: &[f64; 6], s2: &[f64; 6]) -> f64 {
        raw::dvdot(s1, s2)
    }

    /// Derivative and unit vector "V-hat" of a state
    ///
    /// Find the unit vector corresponding to a state vector and the
    /// derivative of the unit vector.
    ///
    /// Returns `sout`.
    ///
    /// See [`dvhat`](raw::dvhat) for full documentation.
    pub fn dvhat(&self, s1: &[f64; 6]) -> [f64; 6] {
        let mut sout: [f64; 6] = Default::default();
        raw::dvhat(s1, &mut sout);
        sout
    }

    /// Derivative of vector norm
    ///
    /// Calculate the derivative of the norm of a 3-vector.
    ///
    /// See [`dvnorm`](raw::dvnorm) for full documentation.
    pub fn dvnorm(&self, state: &[f64; 6]) -> f64 {
        raw::dvnorm(state)
    }

    /// Derivative of separation angle
    ///
    /// Calculate the time derivative of the separation angle between
    /// two input states, S1 and S2.
    ///
    /// See [`dvsep`](raw::dvsep) for full documentation.
    pub fn dvsep(&mut self, s1: &[f64; 6], s2: &[f64; 6]) -> Result<f64> {
        raw::dvsep(self, s1, s2)
    }

    /// Extract Double Precision Values From A String
    ///
    /// Locate a keyword and succeeding numeric words within a string.
    /// Parse and store the numeric words. Remove the keyword and
    /// numeric words from the input string.
    ///
    /// Returns `(nfound, parsed, values)`.
    ///
    /// See [`dxtrct`](raw::dxtrct) for full documentation.
    pub fn dxtrct(&mut self, keywd: &str, maxwds: i32, string: &mut str) -> (i32, i32, Vec<f64>) {
        let mut nfound: i32 = Default::default();
        let mut parsed: i32 = Default::default();
        let mut values: Vec<f64> = vec![Default::default(); maxwds.max(0) as usize];
        raw::dxtrct(
            self,
            keywd,
            maxwds,
            string,
            &mut nfound,
            &mut parsed,
            &mut values,
        );
        (nfound, parsed, values)
    }

    /// Ellipsoid Limb
    ///
    /// Find the limb of a triaxial ellipsoid, viewed from a specified
    /// point.
    ///
    /// Returns `limb`.
    ///
    /// See [`edlimb`](raw::edlimb) for full documentation.
    pub fn edlimb(&mut self, a: f64, b: f64, c: f64, viewpt: &[f64; 3]) -> Result<[f64; 9]> {
        let mut limb: [f64; 9] = Default::default();
        raw::edlimb(self, a, b, c, viewpt, &mut limb)?;
        Ok(limb)
    }

    /// Ellipsoid normal vector to surface point
    ///
    /// Return the unique point on an ellipsoid's surface where the
    /// outward normal direction is a given vector.
    ///
    /// Returns `point`.
    ///
    /// See [`ednmpt`](raw::ednmpt) for full documentation.
    pub fn ednmpt(&mut self, a: f64, b: f64, c: f64, normal: &[f64; 3]) -> Result<[f64; 3]> {
        let mut point: [f64; 3] = Default::default();
        raw::ednmpt(self, a, b, c, normal, &mut point)?;
        Ok(point)
    }

    /// Ellipsoid point
    ///
    /// Scale a point so that it lies on the surface of a specified
    /// triaxial ellipsoid that is centered at the origin and aligned
    /// with the Cartesian coordinate axes.
    ///
    /// Returns `ep`.
    ///
    /// See [`edpnt`](raw::edpnt) for full documentation.
    pub fn edpnt(&mut self, p: &[f64; 3], a: f64, b: f64, c: f64) -> Result<[f64; 3]> {
        let mut ep: [f64; 3] = Default::default();
        raw::edpnt(self, p, a, b, c, &mut ep)?;
        Ok(ep)
    }

    /// Ellipsoid terminator
    ///
    /// Compute a set of points on the umbral or penumbral terminator of
    /// a specified target body, where the target shape is modeled as an
    /// ellipsoid.
    ///
    /// Returns `(trgepc, obspos, trmpts)`.
    ///
    /// See [`edterm`](raw::edterm) for full documentation.
    pub fn edterm(
        &mut self,
        trmtyp: &str,
        source: &str,
        target: &str,
        et: f64,
        fixref: &str,
        abcorr: &str,
        obsrvr: &str,
        npts: i32,
    ) -> Result<(f64, [f64; 3], Vec<[f64; 3]>)> {
        let mut trgepc: f64 = Default::default();
        let mut obspos: [f64; 3] = Default::default();
        let mut trmpts: Vec<[f64; 3]> = vec![Default::default(); npts.max(0) as usize];
        raw::edterm(
            self,
            trmtyp,
            source,
            target,
            et,
            fixref,
            abcorr,
            obsrvr,
            npts,
            &mut trgepc,
            &mut obspos,
            &mut trmpts,
        )?;
        Ok((trgepc, obspos, trmpts))
    }

    /// EK, add character data to column
    ///
    /// Add data to a character column in a specified EK record.
    ///
    /// See [`ekacec`](raw::ekacec) for full documentation.
    pub fn ekacec(
        &mut self,
        handle: i32,
        segno: i32,
        recno: i32,
        column: &str,
        nvals: i32,
        cvals: &CharVec,
        isnull: bool,
    ) -> Result<()> {
        raw::ekacec(
            self,
            handle,
            segno,
            recno,
            column,
            nvals,
            cvals.as_arg(),
            isnull,
        )?;
        Ok(())
    }

    /// EK, add d.p. data to column
    ///
    /// Add data to an double precision column in a specified EK record.
    ///
    /// See [`ekaced`](raw::ekaced) for full documentation.
    pub fn ekaced(
        &mut self,
        handle: i32,
        segno: &mut i32,
        recno: i32,
        column: &str,
        nvals: i32,
        dvals: &[f64],
        isnull: bool,
    ) -> Result<()> {
        raw::ekaced(self, handle, segno, recno, column, nvals, dvals, isnull)?;
        Ok(())
    }

    /// EK, add integer data to column
    ///
    /// Add data to an integer column in a specified EK record.
    ///
    /// See [`ekacei`](raw::ekacei) for full documentation.
    pub fn ekacei(
        &mut self,
        handle: i32,
        segno: &mut i32,
        recno: i32,
        column: &str,
        nvals: i32,
        ivals: &[i32],
        isnull: bool,
    ) -> Result<()> {
        raw::ekacei(self, handle, segno, recno, column, nvals, ivals, isnull)?;
        Ok(())
    }

    /// EK, add character column to segment
    ///
    /// Add an entire character column to an EK segment.
    ///
    /// See [`ekaclc`](raw::ekaclc) for full documentation.
    pub fn ekaclc(
        &mut self,
        handle: i32,
        segno: i32,
        column: &str,
        cvals: &CharVec,
        entszs: &[i32],
        nlflgs: &[bool],
        rcptrs: &[i32],
        wkindx: &mut [i32],
    ) -> Result<()> {
        raw::ekaclc(
            self,
            handle,
            segno,
            column,
            cvals.as_arg(),
            entszs,
            nlflgs,
            rcptrs,
            wkindx,
        )?;
        Ok(())
    }

    /// EK, add d.p. column to segment
    ///
    /// Add an entire double precision column to an EK segment.
    ///
    /// See [`ekacld`](raw::ekacld) for full documentation.
    pub fn ekacld(
        &mut self,
        handle: i32,
        segno: i32,
        column: &str,
        dvals: &[f64],
        entszs: &[i32],
        nlflgs: &[bool],
        rcptrs: &[i32],
        wkindx: &mut [i32],
    ) -> Result<()> {
        raw::ekacld(
            self, handle, segno, column, dvals, entszs, nlflgs, rcptrs, wkindx,
        )?;
        Ok(())
    }

    /// EK, add integer column to segment
    ///
    /// Add an entire integer column to an EK segment.
    ///
    /// See [`ekacli`](raw::ekacli) for full documentation.
    pub fn ekacli(
        &mut self,
        handle: i32,
        segno: i32,
        column: &str,
        ivals: &[i32],
        entszs: &[i32],
        nlflgs: &[bool],
        rcptrs: &[i32],
        wkindx: &mut [i32],
    ) -> Result<()> {
        raw::ekacli(
            self, handle, segno, column, ivals, entszs, nlflgs, rcptrs, wkindx,
        )?;
        Ok(())
    }

    /// EK, append record onto segment
    ///
    /// Append a new, empty record at the end of a specified E-kernel
    /// segment.
    ///
    /// Returns `recno`.
    ///
    /// See [`ekappr`](raw::ekappr) for full documentation.
    pub fn ekappr(&mut self, handle: i32, segno: i32) -> Result<i32> {
        let mut recno: i32 = Default::default();
        raw::ekappr(self, handle, segno, &mut recno)?;
        Ok(recno)
    }

    /// EK, start new segment
    ///
    /// Start a new segment in an E-kernel.
    ///
    /// Returns `segno`.
    ///
    /// See [`ekbseg`](raw::ekbseg) for full documentation.
    pub fn ekbseg(
        &mut self,
        handle: i32,
        tabnam: &str,
        ncols: i32,
        cnames: &CharVec,
        decls: &CharVec,
    ) -> Result<i32> {
        let mut segno: i32 = Default::default();
        raw::ekbseg(
            self,
            handle,
            tabnam,
            ncols,
            cnames.as_arg(),
            decls.as_arg(),
            &mut segno,
        )?;
        Ok(segno)
    }

    /// EK, close file
    ///
    /// Close an E-kernel.
    ///
    /// See [`ekcls`](raw::ekcls) for full documentation.
    pub fn ekcls(&mut self, handle: i32) -> Result<()> {
        raw::ekcls(self, handle)?;
        Ok(())
    }

    /// EK, delete record from segment
    ///
    /// Delete a specified record from a specified E-kernel segment.
    ///
    /// See [`ekdelr`](raw::ekdelr) for full documentation.
    pub fn ekdelr(&mut self, handle: i32, segno: i32, recno: &mut i32) -> Result<()> {
        raw::ekdelr(self, handle, segno, recno)?;
        Ok(())
    }

    /// EK, finish fast write
    ///
    /// Complete a fast write operation on a new E-kernel segment.
    ///
    /// See [`ekffld`](raw::ekffld) for full documentation.
    pub fn ekffld(&mut self, handle: i32, segno: i32, rcptrs: &[i32]) -> Result<()> {
        raw::ekffld(self, handle, segno, rcptrs)?;
        Ok(())
    }

    /// EK, find data
    ///
    /// Find E-kernel data that satisfy a set of constraints.
    ///
    /// Returns `(nmrows, error, errmsg)`.
    ///
    /// See [`ekfind`](raw::ekfind) for full documentation.
    pub fn ekfind(&mut self, query: &str) -> Result<(i32, bool, String)> {
        let mut nmrows: i32 = Default::default();
        let mut error: bool = Default::default();
        let mut errmsg = blank(inc::errhnd::LMSGLN);
        raw::ekfind(self, query, &mut nmrows, &mut error, &mut errmsg)?;
        Ok((nmrows, error, trim(errmsg)))
    }

    /// EK, initialize segment for fast write
    ///
    /// Initialize a new E-kernel segment to allow fast writing.
    ///
    /// Returns `(segno, rcptrs)`.
    ///
    /// See [`ekifld`](raw::ekifld) for full documentation.
    pub fn ekifld(
        &mut self,
        handle: i32,
        tabnam: &str,
        ncols: i32,
        nrows: i32,
        cnames: &CharVec,
        decls: &CharVec,
    ) -> Result<(i32, Vec<i32>)> {
        let mut segno: i32 = Default::default();
        let mut rcptrs: Vec<i32> = vec![Default::default(); nrows.max(0) as usize];
        raw::ekifld(
            self,
            handle,
            tabnam,
            ncols,
            nrows,
            cnames.as_arg(),
            decls.as_arg(),
            &mut segno,
            &mut rcptrs,
        )?;
        Ok((segno, rcptrs))
    }

    /// EK, insert record into segment
    ///
    /// Add a new, empty record to a specified E-kernel segment at
    /// a specified index.
    ///
    /// See [`ekinsr`](raw::ekinsr) for full documentation.
    pub fn ekinsr(&mut self, handle: i32, segno: i32, recno: i32) -> Result<()> {
        raw::ekinsr(self, handle, segno, recno)?;
        Ok(())
    }

    /// EK, number of segments in file
    ///
    /// Return the number of segments in a specified EK.
    ///
    /// See [`eknseg`](raw::eknseg) for full documentation.
    pub fn eknseg(&mut self, handle: i32) -> Result<i32> {
        raw::eknseg(self, handle)
    }

    /// EK, open new file
    ///
    /// Open a new E-kernel file and prepare the file for writing.
    ///
    /// Returns `handle`.
    ///
    /// See [`ekopn`](raw::ekopn) for full documentation.
    pub fn ekopn(&mut self, fname: &str, ifname: &str, ncomch: i32) -> Result<i32> {
        let mut handle: i32 = Default::default();
        raw::ekopn(self, fname, ifname, ncomch, &mut handle)?;
        Ok(handle)
    }

    /// EK, open file for reading
    ///
    /// Open an existing E-kernel file for reading.
    ///
    /// Returns `handle`.
    ///
    /// See [`ekopr`](raw::ekopr) for full documentation.
    pub fn ekopr(&mut self, fname: &str) -> Result<i32> {
        let mut handle: i32 = Default::default();
        raw::ekopr(self, fname, &mut handle)?;
        Ok(handle)
    }

    /// EK, open scratch file
    ///
    /// Open a scratch (temporary) E-kernel file and prepare the file
    /// for writing.
    ///
    /// Returns `handle`.
    ///
    /// See [`ekops`](raw::ekops) for full documentation.
    pub fn ekops(&mut self) -> Result<i32> {
        let mut handle: i32 = Default::default();
        raw::ekops(self, &mut handle)?;
        Ok(handle)
    }

    /// EK, open file for writing
    ///
    /// Open an existing E-kernel file for writing.
    ///
    /// Returns `handle`.
    ///
    /// See [`ekopw`](raw::ekopw) for full documentation.
    pub fn ekopw(&mut self, fname: &str) -> Result<i32> {
        let mut handle: i32 = Default::default();
        raw::ekopw(self, fname, &mut handle)?;
        Ok(handle)
    }

    /// EK, parse SELECT clause
    ///
    /// Parse the SELECT clause of an EK query, returning full particulars
    /// concerning each selected item.
    ///
    /// Returns `(n, xbegs, xends, error, errmsg)`.
    ///
    /// See [`ekpsel`](raw::ekpsel) for full documentation.
    pub fn ekpsel(
        &mut self,
        query: &str,
        xtypes: &mut CharVec,
        xclass: &mut CharVec,
        tabs: &mut CharVec,
        cols: &mut CharVec,
    ) -> Result<(i32, Vec<i32>, Vec<i32>, bool, String)> {
        let mut n: i32 = Default::default();
        let mut xbegs: Vec<i32> = vec![Default::default(); inc::ekqlimit::MAXSEL as usize];
        let mut xends: Vec<i32> = vec![Default::default(); inc::ekqlimit::MAXSEL as usize];
        let mut error: bool = Default::default();
        let mut errmsg = blank(inc::errhnd::LMSGLN);
        raw::ekpsel(
            self,
            query,
            &mut n,
            &mut xbegs,
            &mut xends,
            xtypes.as_arg_mut(),
            xclass.as_arg_mut(),
            tabs.as_arg_mut(),
            cols.as_arg_mut(),
            &mut error,
            &mut errmsg,
        )?;
        Ok((n, xbegs, xends, error, trim(errmsg)))
    }

    /// EK, load event file
    ///
    /// Load an EK file, making it accessible to the EK readers.
    ///
    /// Returns `handle`.
    ///
    /// See [`eklef`](raw::eklef) for full documentation.
    pub fn eklef(&mut self, fname: &str) -> Result<i32> {
        let mut handle: i32 = Default::default();
        raw::eklef(self, fname, &mut handle)?;
        Ok(handle)
    }

    /// EK, unload event file
    ///
    /// Unload an EK file, making its contents inaccessible to the
    /// EK reader routines, and clearing space in order to allow other
    /// EK files to be loaded.
    ///
    /// See [`ekuef`](raw::ekuef) for full documentation.
    pub fn ekuef(&mut self, handle: i32) -> Result<()> {
        raw::ekuef(self, handle)?;
        Ok(())
    }

    /// EK, return number of loaded tables
    ///
    /// Return the number of loaded EK tables.
    ///
    /// Returns `n`.
    ///
    /// See [`ekntab`](raw::ekntab) for full documentation.
    pub fn ekntab(&mut self) -> Result<i32> {
        let mut n: i32 = Default::default();
        raw::ekntab(self, &mut n)?;
        Ok(n)
    }

    /// EK, return name of loaded table
    ///
    /// Return the name of a specified, loaded table.
    ///
    /// Returns `table`.
    ///
    /// See [`ektnam`](raw::ektnam) for full documentation.
    pub fn ektnam(&mut self, n: i32) -> Result<String> {
        let mut table = blank(inc::ektnamsz::TNAMSZ);
        raw::ektnam(self, n, &mut table)?;
        Ok(trim(table))
    }

    /// EK, column count
    ///
    /// Return the number of distinct columns in a specified, currently
    /// loaded table.
    ///
    /// Returns `ccount`.
    ///
    /// See [`ekccnt`](raw::ekccnt) for full documentation.
    pub fn ekccnt(&mut self, table: &str) -> Result<i32> {
        let mut ccount: i32 = Default::default();
        raw::ekccnt(self, table, &mut ccount)?;
        Ok(ccount)
    }

    /// EK, column info by index
    ///
    /// Return attribute information about a column belonging to a loaded
    /// EK table, specifying the column by table and index.
    ///
    /// Returns `(column, attdsc)`.
    ///
    /// See [`ekcii`](raw::ekcii) for full documentation.
    pub fn ekcii(&mut self, table: &str, cindex: i32) -> Result<(String, [i32; 6])> {
        let mut column = blank(inc::ekattdsc::ADSCSZ);
        let mut attdsc: [i32; 6] = Default::default();
        raw::ekcii(self, table, cindex, &mut column, &mut attdsc)?;
        Ok((trim(column), attdsc))
    }

    /// EK, search for events
    ///
    /// Search for EK events matching a specified set of constraints.
    ///
    /// Returns `(nmrows, semerr, errmsg)`.
    ///
    /// See [`eksrch`](raw::eksrch) for full documentation.
    pub fn eksrch(
        &mut self,
        eqryi: &Cell<i32>,
        eqryc: &str,
        eqryd: &[f64],
    ) -> Result<(i32, bool, String)> {
        let mut nmrows: i32 = Default::default();
        let mut semerr: bool = Default::default();
        let mut errmsg = blank(inc::errhnd::LMSGLN);
        raw::eksrch(
            self,
            eqryi.as_raw_slice(),
            eqryc,
            eqryd,
            &mut nmrows,
            &mut semerr,
            &mut errmsg,
        )?;
        Ok((nmrows, semerr, trim(errmsg)))
    }

    /// EK, get number of elements in column entry
    ///
    /// Return the number of elements in a specified column entry in
    /// the current row.
    ///
    /// Returns `nelt`.
    ///
    /// See [`eknelt`](raw::eknelt) for full documentation.
    pub fn eknelt(&mut self, selidx: i32, row: i32) -> Result<i32> {
        let mut nelt: i32 = Default::default();
        raw::eknelt(self, selidx, row, &mut nelt)?;
        Ok(nelt)
    }

    /// EK, get event data, character
    ///
    /// Return an element of an entry in a column of character
    /// type in a specified row.
    ///
    /// Returns `(cdata, null)`.
    ///
    /// See [`ekgc`](raw::ekgc) for full documentation.
    pub fn ekgc(&mut self, selidx: i32, row: i32, elment: i32) -> Result<Option<(String, bool)>> {
        let mut cdata = blank(inc::ekqlimit::MAXSTR);
        let mut null: bool = Default::default();
        let mut found: bool = Default::default();
        raw::ekgc(self, selidx, row, elment, &mut cdata, &mut null, &mut found)?;
        Ok(if found {
            Some((trim(cdata), null))
        } else {
            None
        })
    }

    /// EK, get event data, double precision
    ///
    /// Return an element of an entry in a column of double precision
    /// or `time' type in a specified row.
    ///
    /// Returns `(ddata, null)`.
    ///
    /// See [`ekgd`](raw::ekgd) for full documentation.
    pub fn ekgd(&mut self, selidx: i32, row: i32, elment: i32) -> Result<Option<(f64, bool)>> {
        let mut ddata: f64 = Default::default();
        let mut null: bool = Default::default();
        let mut found: bool = Default::default();
        raw::ekgd(self, selidx, row, elment, &mut ddata, &mut null, &mut found)?;
        Ok(if found { Some((ddata, null)) } else { None })
    }

    /// EK, get event data, integer
    ///
    /// Return an element of an entry in a column of integer
    /// type in a specified row.
    ///
    /// Returns `(idata, null)`.
    ///
    /// See [`ekgi`](raw::ekgi) for full documentation.
    pub fn ekgi(&mut self, selidx: i32, row: i32, elment: i32) -> Result<Option<(i32, bool)>> {
        let mut idata: i32 = Default::default();
        let mut null: bool = Default::default();
        let mut found: bool = Default::default();
        raw::ekgi(self, selidx, row, elment, &mut idata, &mut null, &mut found)?;
        Ok(if found { Some((idata, null)) } else { None })
    }

    /// EK, read column entry element, character
    ///
    /// Read data from a character column in a specified EK record.
    ///
    /// Returns `(nvals, isnull)`.
    ///
    /// See [`ekrcec`](raw::ekrcec) for full documentation.
    pub fn ekrcec(
        &mut self,
        handle: i32,
        segno: &mut i32,
        recno: i32,
        column: &str,
        cvals: &mut CharVec,
    ) -> Result<(i32, bool)> {
        let mut nvals: i32 = Default::default();
        let mut isnull: bool = Default::default();
        raw::ekrcec(
            self,
            handle,
            segno,
            recno,
            column,
            &mut nvals,
            cvals.as_arg_mut(),
            &mut isnull,
        )?;
        Ok((nvals, isnull))
    }

    /// EK, read column entry element, d.p.
    ///
    /// Read data from a double precision column in a specified EK
    /// record.
    ///
    /// Returns `(nvals, isnull)`.
    ///
    /// See [`ekrced`](raw::ekrced) for full documentation.
    pub fn ekrced(
        &mut self,
        handle: i32,
        segno: &mut i32,
        recno: i32,
        column: &str,
        dvals: &mut [f64],
    ) -> Result<(i32, bool)> {
        let mut nvals: i32 = Default::default();
        let mut isnull: bool = Default::default();
        raw::ekrced(
            self,
            handle,
            segno,
            recno,
            column,
            &mut nvals,
            dvals,
            &mut isnull,
        )?;
        Ok((nvals, isnull))
    }

    /// EK, read column entry element, integer
    ///
    /// Read data from an integer column in a specified EK record.
    ///
    /// Returns `(nvals, isnull)`.
    ///
    /// See [`ekrcei`](raw::ekrcei) for full documentation.
    pub fn ekrcei(
        &mut self,
        handle: i32,
        segno: &mut i32,
        recno: i32,
        column: &str,
        ivals: &mut [i32],
    ) -> Result<(i32, bool)> {
        let mut nvals: i32 = Default::default();
        let mut isnull: bool = Default::default();
        raw::ekrcei(
            self,
            handle,
            segno,
            recno,
            column,
            &mut nvals,
            ivals,
            &mut isnull,
        )?;
        Ok((nvals, isnull))
    }

    /// EK, return shadowing status \<STUB>
    ///
    /// Return shadowing status of a specified EK file. THIS IS A
    /// STUB ROUTINE.
    ///
    /// Returns `isshad`.
    ///
    /// See [`ekshdw`](raw::ekshdw) for full documentation.
    pub fn ekshdw(&self, handle: i32) -> bool {
        let mut isshad: bool = Default::default();
        raw::ekshdw(handle, &mut isshad);
        isshad
    }

    /// EK, return segment summary
    ///
    /// Return summary information for a specified segment in a
    /// specified EK.
    ///
    /// Returns `(tabnam, nrows, ncols, sizes, strlns, indexd, nullok)`.
    ///
    /// See [`ekssum`](raw::ekssum) for full documentation.
    pub fn ekssum(
        &mut self,
        handle: i32,
        segno: i32,
        cnames: &mut CharVec,
        dtypes: &mut CharVec,
    ) -> Result<(String, i32, i32, Vec<i32>, Vec<i32>, Vec<bool>, Vec<bool>)> {
        let mut tabnam = blank(inc::ektnamsz::TNAMSZ);
        let mut nrows: i32 = Default::default();
        let mut ncols: i32 = Default::default();
        let mut sizes: Vec<i32> = vec![Default::default(); inc::ekglimit::MXCLSG as usize];
        let mut strlns: Vec<i32> = vec![Default::default(); inc::ekglimit::MXCLSG as usize];
        let mut indexd: Vec<bool> = vec![Default::default(); inc::ekglimit::MXCLSG as usize];
        let mut nullok: Vec<bool> = vec![Default::default(); inc::ekglimit::MXCLSG as usize];
        raw::ekssum(
            self,
            handle,
            segno,
            &mut tabnam,
            &mut nrows,
            &mut ncols,
            cnames.as_arg_mut(),
            dtypes.as_arg_mut(),
            &mut sizes,
            &mut strlns,
            &mut indexd,
            &mut nullok,
        )?;
        Ok((trim(tabnam), nrows, ncols, sizes, strlns, indexd, nullok))
    }

    /// EK, update character column entry
    ///
    /// Update a character column entry in a specified EK record.
    ///
    /// See [`ekucec`](raw::ekucec) for full documentation.
    pub fn ekucec(
        &mut self,
        handle: i32,
        segno: &mut i32,
        recno: i32,
        column: &str,
        nvals: i32,
        cvals: &CharVec,
        isnull: bool,
    ) -> Result<()> {
        raw::ekucec(
            self,
            handle,
            segno,
            recno,
            column,
            nvals,
            cvals.as_arg(),
            isnull,
        )?;
        Ok(())
    }

    /// EK, update d.p. column entry
    ///
    /// Update a double precision column entry in a specified EK record.
    ///
    /// See [`ekuced`](raw::ekuced) for full documentation.
    pub fn ekuced(
        &mut self,
        handle: i32,
        segno: &mut i32,
        recno: i32,
        column: &str,
        nvals: i32,
        dvals: &[f64],
        isnull: bool,
    ) -> Result<()> {
        raw::ekuced(self, handle, segno, recno, column, nvals, dvals, isnull)?;
        Ok(())
    }

    /// EK, update integer column entry
    ///
    /// Update an integer column entry in a specified EK record.
    ///
    /// See [`ekucei`](raw::ekucei) for full documentation.
    pub fn ekucei(
        &mut self,
        handle: i32,
        segno: &mut i32,
        recno: i32,
        column: &str,
        nvals: i32,
        ivals: &[i32],
        isnull: bool,
    ) -> Result<()> {
        raw::ekucei(self, handle, segno, recno, column, nvals, ivals, isnull)?;
        Ok(())
    }

    /// Ellipse to center and generating vectors
    ///
    /// Convert a SPICE ellipse to a center vector and two generating
    /// vectors. The selected generating vectors are semi-axes of the
    /// ellipse.
    ///
    /// Returns `(center, smajor, sminor)`.
    ///
    /// See [`el2cgv`](raw::el2cgv) for full documentation.
    pub fn el2cgv(&self, ellips: &[f64; 9]) -> ([f64; 3], [f64; 3], [f64; 3]) {
        let mut center: [f64; 3] = Default::default();
        let mut smajor: [f64; 3] = Default::default();
        let mut sminor: [f64; 3] = Default::default();
        raw::el2cgv(ellips, &mut center, &mut smajor, &mut sminor);
        (center, smajor, sminor)
    }

    /// Element of a character set
    ///
    /// Determine whether an item is an element of a character set.
    ///
    /// See [`elemc`](raw::elemc) for full documentation.
    pub fn elemc(&mut self, item: &str, a: &CharCell) -> Result<bool> {
        raw::elemc(self, item, a.as_arg())
    }

    /// Element of a double precision set
    ///
    /// Determine whether an item is an element of a double
    /// precision set.
    ///
    /// See [`elemd`](raw::elemd) for full documentation.
    pub fn elemd(&mut self, item: f64, a: &Cell<f64>) -> Result<bool> {
        raw::elemd(self, item, a.as_raw_slice())
    }

    /// Element of an integer set
    ///
    /// Determine whether an item is an element of an integer set.
    ///
    /// See [`elemi`](raw::elemi) for full documentation.
    pub fn elemi(&mut self, item: i32, a: &Cell<i32>) -> Result<bool> {
        raw::elemi(self, item, a.as_raw_slice())
    }

    /// Elliptic time of flight
    ///
    /// Solve the time of flight equation MA = E - e sin(E) for the
    /// elliptic eccentric anomaly E, given mean anomaly the MA and
    /// the eccentricity ECC.
    ///
    /// Returns `e`.
    ///
    /// See [`elltof`](raw::elltof) for full documentation.
    pub fn elltof(&mut self, ma: f64, ecc: f64) -> Result<f64> {
        let mut e: f64 = Default::default();
        raw::elltof(self, ma, ecc, &mut e)?;
        Ok(e)
    }

    /// Encode a character string
    ///
    /// Encode a nonnegative integer number into a character string
    /// as the expansion of the number in base CHBASE (a function of
    /// the size of the available character set).
    ///
    /// Returns `string`.
    ///
    /// See [`enchar`](raw::enchar) for full documentation.
    pub fn enchar(&mut self, number: i32) -> Result<String> {
        let mut string = blank(5);
        raw::enchar(self, number, &mut string)?;
        Ok(trim(string))
    }

    /// Decode a character string
    ///
    /// Decode a character string encoded by ENCHAR.
    ///
    /// Returns `number`.
    ///
    /// See [`dechar`](raw::dechar) for full documentation.
    pub fn dechar(&mut self, string: &str) -> Result<i32> {
        let mut number: i32 = Default::default();
        raw::dechar(self, string, &mut number)?;
        Ok(number)
    }

    /// Equivalent characters
    ///
    /// Return .TRUE. if two given characters are equivalent when the
    /// case of the characters is ignored.
    ///
    /// See [`eqchr`](raw::eqchr) for full documentation.
    pub fn eqchr(&mut self, a: char, b: char) -> bool {
        raw::eqchr(self, a, b)
    }

    /// Not Equivalent characters
    ///
    /// Return .TRUE. if two given characters are not equivalent if the
    /// case of the characters is ignored.
    ///
    /// See [`nechr`](raw::nechr) for full documentation.
    pub fn nechr(&mut self, a: char, b: char) -> bool {
        raw::nechr(self, a, b)
    }

    /// Equinoctial Elements to position and velocity
    ///
    /// Compute the state (position and velocity) of an object whose
    /// trajectory is described via equinoctial elements relative to some
    /// fixed plane (usually the equatorial plane of some planet).
    ///
    /// Returns `state`.
    ///
    /// See [`eqncpv`](raw::eqncpv) for full documentation.
    pub fn eqncpv(
        &mut self,
        et: f64,
        epoch: f64,
        eqel: &[f64; 9],
        rapol: f64,
        decpol: f64,
    ) -> Result<[f64; 6]> {
        let mut state: [f64; 6] = Default::default();
        raw::eqncpv(self, et, epoch, eqel, rapol, decpol, &mut state)?;
        Ok(state)
    }

    /// Equivalent strings
    ///
    /// Determine whether two strings are equivalent.
    ///
    /// See [`eqstr`](raw::eqstr) for full documentation.
    pub fn eqstr(&self, a: &str, b: &str) -> bool {
        raw::eqstr(a, b)
    }

    /// Get/Set Default Error Action
    ///
    /// Retrieve or set the default error action.
    ///
    /// See [`erract`](raw::erract) for full documentation.
    pub fn erract(&mut self, op: &str, action: &mut str) -> Result<()> {
        raw::erract(self, op, action)?;
        Ok(())
    }

    /// Insert String into Error Message Text
    ///
    /// Substitute a character string for the first occurrence of
    /// a marker in the current long error message.
    ///
    /// See [`errch`](raw::errch) for full documentation.
    pub fn errch(&mut self, marker: &str, string: &str) {
        raw::errch(self, marker, string);
    }

    /// Get/Set Error Output Device Name
    ///
    /// Retrieve or set the name of the current output
    /// device for error messages.
    ///
    /// See [`errdev`](raw::errdev) for full documentation.
    pub fn errdev(&mut self, op: &str, device: &mut str) -> Result<()> {
        raw::errdev(self, op, device)?;
        Ok(())
    }

    /// Insert D.P. Number into Error Message Text
    ///
    /// Substitute a double precision number for the first occurrence of
    /// a marker found in the current long error message.
    ///
    /// See [`errdp`](raw::errdp) for full documentation.
    pub fn errdp(&mut self, marker: &str, dpnum: f64) {
        raw::errdp(self, marker, dpnum);
    }

    /// Insert filename into long error message text
    ///
    /// Substitute the first occurrence of a marker in the current long
    /// error message with the name of the file attached to the logical
    /// unit number.
    ///
    /// See [`errfnm`](raw::errfnm) for full documentation.
    pub fn errfnm(&mut self, marker: &str, unit: i32) -> Result<()> {
        raw::errfnm(self, marker, unit)?;
        Ok(())
    }

    /// Insert DAF/DAS file name into long error message
    ///
    /// Substitute the first occurrence of a marker in the current long
    /// error message with the file name associated with a given
    /// DAF/DAS handle.
    ///
    /// See [`errhan`](raw::errhan) for full documentation.
    pub fn errhan(&mut self, marker: &str, handle: i32) -> Result<()> {
        raw::errhan(self, marker, handle)?;
        Ok(())
    }

    /// Insert Integer into Error Message Text
    ///
    /// Substitute an integer for the first occurrence of a marker found
    /// in the current long error message.
    ///
    /// See [`errint`](raw::errint) for full documentation.
    pub fn errint(&mut self, marker: &str, intnum: i32) {
        raw::errint(self, marker, intnum);
    }

    /// Get/Set Error Output Items
    ///
    /// Retrieve or set the list of error message items
    /// to be output when an error is detected.
    ///
    /// See [`errprt`](raw::errprt) for full documentation.
    pub fn errprt(&mut self, op: &str, list: &mut str) -> Result<()> {
        raw::errprt(self, op, list)?;
        Ok(())
    }

    /// Equivalence search, character
    ///
    /// Search for a given value within a character string array.
    /// Return the index of the first equivalent array entry, or zero
    /// if no equivalent element is found.
    ///
    /// See [`esrchc`](raw::esrchc) for full documentation.
    pub fn esrchc(&self, value: &str, ndim: i32, array: &CharVec) -> i32 {
        raw::esrchc(value, ndim, array.as_arg())
    }

    /// ET to Local Solar Time
    ///
    /// Compute the local solar time for a given ephemeris epoch ET
    /// for an object on the surface of a body at a specified longitude.
    ///
    /// Returns `(hr, mn, sc, time, ampm)`.
    ///
    /// See [`et2lst`](raw::et2lst) for full documentation.
    pub fn et2lst(
        &mut self,
        et: f64,
        body: i32,
        lon: f64,
        type_: &str,
    ) -> Result<(i32, i32, i32, String, String)> {
        let mut hr: i32 = Default::default();
        let mut mn: i32 = Default::default();
        let mut sc: i32 = Default::default();
        let mut time = blank(50);
        let mut ampm = blank(50);
        raw::et2lst(
            self, et, body, lon, type_, &mut hr, &mut mn, &mut sc, &mut time, &mut ampm,
        )?;
        Ok((hr, mn, sc, trim(time), trim(ampm)))
    }

    /// Ephemeris Time to UTC
    ///
    /// Convert an input time from ephemeris seconds past J2000
    /// to Calendar, Day-of-Year, or Julian Date format, UTC.
    ///
    /// Returns `utcstr`.
    ///
    /// See [`et2utc`](raw::et2utc) for full documentation.
    pub fn et2utc(&mut self, et: f64, format: &str, prec: i32) -> Result<String> {
        let mut utcstr = blank((24 + prec).max(0));
        raw::et2utc(self, et, format, prec, &mut utcstr)?;
        Ok(trim(utcstr))
    }

    /// Convert ET to Calendar format
    ///
    /// Convert from an ephemeris epoch measured in seconds past
    /// the epoch of J2000 to a calendar string format using a
    /// formal calendar free of leapseconds.
    ///
    /// Returns `calstr`.
    ///
    /// See [`etcal`](raw::etcal) for full documentation.
    pub fn etcal(&mut self, et: f64) -> String {
        let mut calstr = blank(48);
        raw::etcal(self, et, &mut calstr);
        trim(calstr)
    }

    /// Euler angles to matrix
    ///
    /// Construct a rotation matrix from a set of Euler angles.
    ///
    /// Returns `r`.
    ///
    /// See [`eul2m`](raw::eul2m) for full documentation.
    pub fn eul2m(
        &mut self,
        angle3: f64,
        angle2: f64,
        angle1: f64,
        axis3: i32,
        axis2: i32,
        axis1: i32,
    ) -> Result<[[f64; 3]; 3]> {
        let mut r: [[f64; 3]; 3] = Default::default();
        raw::eul2m(self, angle3, angle2, angle1, axis3, axis2, axis1, &mut r)?;
        Ok(r)
    }

    /// Is an integer even?
    ///
    /// Determine whether an integer is even.
    ///
    /// See [`even`](raw::even) for full documentation.
    pub fn even(&self, ival: i32) -> bool {
        raw::even(ival)
    }

    /// Evaluate "two-line" element data
    ///
    /// Evaluate NORAD two-line element data for earth orbiting
    /// spacecraft. This evaluator uses algorithms as described
    /// in Vallado 2006 \[4].
    ///
    /// This routine supersedes SPICELIB routines EV2LIN and DPSPCE.
    ///
    /// Returns `state`.
    ///
    /// See [`evsgp4`](raw::evsgp4) for full documentation.
    pub fn evsgp4(&mut self, et: f64, geophs: &[f64; 8], elems: &[f64; 10]) -> Result<[f64; 6]> {
        let mut state: [f64; 6] = Default::default();
        raw::evsgp4(self, et, geophs, elems, &mut state)?;
        Ok(state)
    }

    /// Round to exact value
    ///
    /// Round an input double precision number to a specified exact value
    /// if the number and the value are equal to within some tolerance.
    ///
    /// See [`exact`](raw::exact) for full documentation.
    pub fn exact(&self, number: f64, value: f64, tol: f64) -> f64 {
        raw::exact(number, value, tol)
    }

    /// Report an excess of elements in a cell
    ///
    /// Set the long error message so as to indicate the number of excess
    /// elements encountered by a routine operating on cells or on data
    /// structures based on cells.
    ///
    /// See [`excess`](raw::excess) for full documentation.
    pub fn excess(&mut self, number: i32, struct_: &str) -> Result<()> {
        raw::excess(self, number, struct_)?;
        Ok(())
    }

    /// Does the file exist?
    ///
    /// Determine whether a file exists.
    ///
    /// See [`exists`](raw::exists) for full documentation.
    pub fn exists(&mut self, fname: &str) -> Result<bool> {
        raw::exists(self, fname)
    }

    /// Get Explanation for Short Error Message
    ///
    /// Return the explanation of a short error message.
    ///
    /// Returns `expl`.
    ///
    /// See [`expln`](raw::expln) for full documentation.
    pub fn expln(&self, msg: &str) -> String {
        let mut expl = blank(256);
        raw::expln(msg, &mut expl);
        trim(expl)
    }

    /// Fetch from a character set
    ///
    /// Return the location within the set array of the NTH element within
    /// the order imposed by the ASCII collating sequence.
    ///
    /// See [`fetchc`](raw::fetchc) for full documentation.
    pub fn fetchc(&mut self, nth: i32, set: &CharCell) -> Result<i32> {
        raw::fetchc(self, nth, set.as_arg())
    }

    /// Fetch from a DP set
    ///
    /// Return the location within the set array of the NTH element
    /// within the order imposed by the values of the elements.
    ///
    /// See [`fetchd`](raw::fetchd) for full documentation.
    pub fn fetchd(&mut self, nth: i32, set: &Cell<f64>) -> Result<i32> {
        raw::fetchd(self, nth, set.as_raw_slice())
    }

    /// Fetch from an integer set
    ///
    /// Return the location within the set array of the NTH element
    /// within the order imposed by the values of the elements.
    ///
    /// See [`fetchi`](raw::fetchi) for full documentation.
    pub fn fetchi(&mut self, nth: i32, set: &Cell<i32>) -> Result<i32> {
        raw::fetchi(self, nth, set.as_raw_slice())
    }

    /// Map name of open file to its logical unit.
    ///
    /// Map the name of an open file to its associated logical unit.
    ///
    /// Returns `lunit`.
    ///
    /// See [`fn2lun`](raw::fn2lun) for full documentation.
    pub fn fn2lun(&mut self, filnam: &str) -> Result<i32> {
        let mut lunit: i32 = Default::default();
        raw::fn2lun(self, filnam, &mut lunit)?;
        Ok(lunit)
    }

    /// Find a free logical unit
    ///
    /// Return the number of a free logical unit, if one is available.
    ///
    /// Returns `unit`.
    ///
    /// See [`fndlun`](raw::fndlun) for full documentation.
    pub fn fndlun(&mut self) -> Result<i32> {
        let mut unit: i32 = Default::default();
        raw::fndlun(self, &mut unit)?;
        Ok(unit)
    }

    /// Reserve a logical unit
    ///
    /// Reserve a logical unit number. Reserved units are never returned
    /// by FNDLUN or GETLUN.
    ///
    /// See [`reslun`](raw::reslun) for full documentation.
    pub fn reslun(&mut self, unit: i32) {
        raw::reslun(self, unit);
    }

    /// Free a reserved logical unit
    ///
    /// Free a logical unit number reserved by RESLUN.
    ///
    /// See [`frelun`](raw::frelun) for full documentation.
    pub fn frelun(&mut self, unit: i32) {
        raw::frelun(self, unit);
    }

    /// Find the next word after an index
    ///
    /// Find the beginning and end of the first word starting at
    /// or after a specified character.
    ///
    /// Returns `(b, e)`.
    ///
    /// See [`fndnwd`](raw::fndnwd) for full documentation.
    pub fn fndnwd(&self, string: &str, start: i32) -> (i32, i32) {
        let mut b: i32 = Default::default();
        let mut e: i32 = Default::default();
        raw::fndnwd(string, start, &mut b, &mut e);
        (b, e)
    }

    /// Is ray in FOV at time?
    ///
    /// Determine if a specified ray is within the field-of-view (FOV) of
    /// a specified instrument at a given time.
    ///
    /// Returns `visibl`.
    ///
    /// See [`fovray`](raw::fovray) for full documentation.
    pub fn fovray(
        &mut self,
        inst: &str,
        raydir: &[f64; 3],
        rframe: &str,
        abcorr: &str,
        obsrvr: &str,
        et: f64,
    ) -> Result<bool> {
        let mut visibl: bool = Default::default();
        raw::fovray(self, inst, raydir, rframe, abcorr, obsrvr, et, &mut visibl)?;
        Ok(visibl)
    }

    /// Is target in FOV at time?
    ///
    /// Determine if a specified ephemeris object is within the
    /// field-of-view (FOV) of a specified instrument at a given time.
    ///
    /// Returns `visibl`.
    ///
    /// See [`fovtrg`](raw::fovtrg) for full documentation.
    pub fn fovtrg(
        &mut self,
        inst: &str,
        target: &str,
        tshape: &str,
        tframe: &str,
        abcorr: &str,
        obsrvr: &str,
        et: f64,
    ) -> Result<bool> {
        let mut visibl: bool = Default::default();
        raw::fovtrg(
            self,
            inst,
            target,
            tshape,
            tframe,
            abcorr,
            obsrvr,
            et,
            &mut visibl,
        )?;
        Ok(visibl)
    }

    /// Build a right handed coordinate frame
    ///
    /// Build a right handed orthonormal frame (x,y,z) from a
    /// 3-dimensional input vector, where the X-axis of the resulting
    /// frame is parallel to the original input vector.
    ///
    /// Returns `(y, z)`.
    ///
    /// See [`frame`](raw::frame) for full documentation.
    pub fn frame(&self, x: &mut [f64; 3]) -> ([f64; 3], [f64; 3]) {
        let mut y: [f64; 3] = Default::default();
        let mut z: [f64; 3] = Default::default();
        raw::frame(x, &mut y, &mut z);
        (y, z)
    }

    /// frame NAMe to FRaMe id
    ///
    /// Look up the frame ID code associated with a string.
    ///
    /// Returns `frcode`.
    ///
    /// See [`namfrm`](raw::namfrm) for full documentation.
    pub fn namfrm(&mut self, frname: &str) -> Result<i32> {
        let mut frcode: i32 = Default::default();
        raw::namfrm(self, frname, &mut frcode)?;
        Ok(frcode)
    }

    /// FRaMe id to frame NAMe
    ///
    /// Retrieve the name of a reference frame associated with a SPICE ID
    /// code.
    ///
    /// Returns `frname`.
    ///
    /// See [`frmnam`](raw::frmnam) for full documentation.
    pub fn frmnam(&mut self, frcode: i32) -> Result<String> {
        let mut frname = blank(26);
        raw::frmnam(self, frcode, &mut frname)?;
        Ok(trim(frname))
    }

    /// FRame INFOrmation
    ///
    /// Retrieve the minimal attributes associated with a frame
    /// needed for converting transformations to and from it.
    ///
    /// Returns `(cent, frclss, clssid)`.
    ///
    /// See [`frinfo`](raw::frinfo) for full documentation.
    pub fn frinfo(&mut self, frcode: i32) -> Result<Option<(i32, i32, i32)>> {
        let mut cent: i32 = Default::default();
        let mut frclss: i32 = Default::default();
        let mut clssid: i32 = Default::default();
        let mut found: bool = Default::default();
        raw::frinfo(
            self,
            frcode,
            &mut cent,
            &mut frclss,
            &mut clssid,
            &mut found,
        )?;
        Ok(if found {
            Some((cent, frclss, clssid))
        } else {
            None
        })
    }

    /// Center ID to FRaMe id and name
    ///
    /// Retrieve frame ID code and name to associate with a frame center.
    ///
    /// Returns `(frcode, frname)`.
    ///
    /// See [`cidfrm`](raw::cidfrm) for full documentation.
    pub fn cidfrm(&mut self, cent: i32) -> Result<Option<(i32, String)>> {
        let mut frcode: i32 = Default::default();
        let mut frname = blank(26);
        let mut found: bool = Default::default();
        raw::cidfrm(self, cent, &mut frcode, &mut frname, &mut found)?;
        Ok(if found {
            Some((frcode, trim(frname)))
        } else {
            None
        })
    }

    /// Center NaMe to FRaMe id and name
    ///
    /// Retrieve frame ID code and name to associate with an object.
    ///
    /// Returns `(frcode, frname)`.
    ///
    /// See [`cnmfrm`](raw::cnmfrm) for full documentation.
    pub fn cnmfrm(&mut self, cname: &str) -> Result<Option<(i32, String)>> {
        let mut frcode: i32 = Default::default();
        let mut frname = blank(26);
        let mut found: bool = Default::default();
        raw::cnmfrm(self, cname, &mut frcode, &mut frname, &mut found)?;
        Ok(if found {
            Some((frcode, trim(frname)))
        } else {
            None
        })
    }

    /// frame Class and Class Id to FRaMe id and name
    ///
    /// Return the frame name, frame ID, and center associated with
    /// a given frame class and class ID.
    ///
    /// Returns `(frcode, frname, cent)`.
    ///
    /// See [`ccifrm`](raw::ccifrm) for full documentation.
    pub fn ccifrm(&mut self, frclss: i32, clssid: i32) -> Result<Option<(i32, String, i32)>> {
        let mut frcode: i32 = Default::default();
        let mut frname = blank(26);
        let mut cent: i32 = Default::default();
        let mut found: bool = Default::default();
        raw::ccifrm(
            self,
            frclss,
            clssid,
            &mut frcode,
            &mut frname,
            &mut cent,
            &mut found,
        )?;
        Ok(if found {
            Some((frcode, trim(frname), cent))
        } else {
            None
        })
    }

    /// Frame Change
    ///
    /// Return the state transformation matrix from one
    /// frame to another.
    ///
    /// Returns `xform`.
    ///
    /// See [`frmchg`](raw::frmchg) for full documentation.
    pub fn frmchg(&mut self, frame1: i32, frame2: i32, et: f64) -> Result<[[f64; 6]; 6]> {
        let mut xform: [[f64; 6]; 6] = Default::default();
        raw::frmchg(self, frame1, frame2, et, &mut xform)?;
        Ok(xform)
    }

    /// Frame get transformation
    ///
    /// Find the transformation from a user specified frame to
    /// another frame at a user specified epoch.
    ///
    /// Returns `(xform, outfrm)`.
    ///
    /// See [`frmget`](raw::frmget) for full documentation.
    pub fn frmget(&mut self, infrm: i32, et: f64) -> Result<Option<([[f64; 6]; 6], i32)>> {
        let mut xform: [[f64; 6]; 6] = Default::default();
        let mut outfrm: i32 = Default::default();
        let mut found: bool = Default::default();
        raw::frmget(self, infrm, et, &mut xform, &mut outfrm, &mut found)?;
        Ok(if found { Some((xform, outfrm)) } else { None })
    }

    /// First non-blank character
    ///
    /// Return the index of the first non-blank character in
    /// a character string.
    ///
    /// See [`frstnb`](raw::frstnb) for full documentation.
    pub fn frstnb(&self, string: &str) -> i32 {
        raw::frstnb(string)
    }

    /// First non-printable character
    ///
    /// Return the index of the first non-printable character in a
    /// character string. ASCII characters 32-126 are considered
    /// printable by this routine. (Blanks are considered printable.)
    ///
    /// See [`frstnp`](raw::frstnp) for full documentation.
    pub fn frstnp(&self, string: &str) -> i32 {
        raw::frstnp(string)
    }

    /// First printable character
    ///
    /// Return the index of the first printable character in a character
    /// string. ASCII characters 33-126 are printable. (Blanks are not
    /// considered printable.)
    ///
    /// See [`frstpc`](raw::frstpc) for full documentation.
    pub fn frstpc(&self, string: &str) -> i32 {
        raw::frstpc(string)
    }

    /// Greatest Common Divisor
    ///
    /// Return the greatest common divisor of two integers.
    ///
    /// See [`gcd`](raw::gcd) for full documentation.
    pub fn gcd(&self, a: i32, b: i32) -> i32 {
        raw::gcd(a, b)
    }

    /// Geodetic to rectangular coordinates
    ///
    /// Convert geodetic coordinates to rectangular coordinates.
    ///
    /// Returns `rectan`.
    ///
    /// See [`georec`](raw::georec) for full documentation.
    pub fn georec(&mut self, lon: f64, lat: f64, alt: f64, re: f64, f: f64) -> Result<[f64; 3]> {
        let mut rectan: [f64; 3] = Default::default();
        raw::georec(self, lon, lat, alt, re, f, &mut rectan)?;
        Ok(rectan)
    }

    /// Get the components from two-line elements
    ///
    /// Parse the "lines" of a two-line element set, returning the
    /// elements in units suitable for use in SPICE software.
    ///
    /// Returns `(epoch, elems)`.
    ///
    /// See [`getelm`](raw::getelm) for full documentation.
    pub fn getelm(&mut self, frstyr: i32, lines: &CharVec) -> Result<(f64, [f64; 10])> {
        let mut epoch: f64 = Default::default();
        let mut elems: [f64; 10] = Default::default();
        raw::getelm(self, frstyr, lines.as_arg(), &mut epoch, &mut elems)?;
        Ok((epoch, elems))
    }

    /// Get file architecture and type
    ///
    /// Determine the architecture and type of SPICE kernels.
    ///
    /// Returns `(arch, kertyp)`.
    ///
    /// See [`getfat`](raw::getfat) for full documentation.
    pub fn getfat(&mut self, file: &str) -> Result<(String, String)> {
        let mut arch = blank(3);
        let mut kertyp = blank(4);
        raw::getfat(self, file, &mut arch, &mut kertyp)?;
        Ok((trim(arch), trim(kertyp)))
    }

    /// Get instrument FOV parameters
    ///
    /// Return the field-of-view (FOV) parameters for a specified
    /// instrument. The instrument is specified by its NAIF ID code.
    ///
    /// Returns `(shape, frame, bsight, n, bounds)`.
    ///
    /// See [`getfov`](raw::getfov) for full documentation.
    pub fn getfov(
        &mut self,
        instid: i32,
        room: i32,
    ) -> Result<(String, String, [f64; 3], i32, Vec<[f64; 3]>)> {
        let mut shape = blank(9);
        let mut frame = blank(26);
        let mut bsight: [f64; 3] = Default::default();
        let mut n: i32 = Default::default();
        let mut bounds: Vec<[f64; 3]> = vec![Default::default(); room.max(0) as usize];
        raw::getfov(
            self,
            instid,
            room,
            &mut shape,
            &mut frame,
            &mut bsight,
            &mut n,
            &mut bounds,
        )?;
        Ok((trim(shape), trim(frame), bsight, n, bounds))
    }

    /// Get instrument FOV parameters, by instrument name
    ///
    /// Return the field-of-view (FOV) parameters for a specified
    /// instrument. The instrument is specified by name.
    ///
    /// Returns `(shape, frame, bsight, n, bounds)`.
    ///
    /// See [`getfvn`](raw::getfvn) for full documentation.
    pub fn getfvn(
        &mut self,
        inst: &str,
        room: i32,
    ) -> Result<(String, String, [f64; 3], i32, Vec<[f64; 3]>)> {
        let mut shape = blank(9);
        let mut frame = blank(26);
        let mut bsight: [f64; 3] = Default::default();
        let mut n: i32 = Default::default();
        let mut bounds: Vec<[f64; 3]> = vec![Default::default(); room.max(0) as usize];
        raw::getfvn(
            self,
            inst,
            room,
            &mut shape,
            &mut frame,
            &mut bsight,
            &mut n,
            &mut bounds,
        )?;
        Ok((trim(shape), trim(frame), bsight, n, bounds))
    }

    /// Get a free logical unit
    ///
    /// Return the number of a free logical unit.
    ///
    /// Returns `unit`.
    ///
    /// See [`getlun`](raw::getlun) for full documentation.
    pub fn getlun(&mut self) -> Result<i32> {
        let mut unit: i32 = Default::default();
        raw::getlun(self, &mut unit)?;
        Ok(unit)
    }

    /// Get Error Message
    ///
    /// Retrieve the current short error message, the explanation of the
    /// short error message, or the long error message.
    ///
    /// Returns `msg`.
    ///
    /// See [`getmsg`](raw::getmsg) for full documentation.
    pub fn getmsg(&mut self, option: &str) -> Result<String> {
        let mut msg = blank(inc::errhnd::LMSGLN);
        raw::getmsg(self, option, &mut msg)?;
        Ok(trim(msg))
    }

    /// GF, default bailout function
    ///
    /// Serve as a placeholder for an interrupt detection function.
    ///
    /// See [`gfbail`](raw::gfbail) for full documentation.
    pub fn gfbail(&self) -> bool {
        raw::gfbail()
    }

    /// GF, distance search
    ///
    /// Determine time intervals over which a specified constraint on
    /// observer-target distance is met.
    ///
    /// See [`gfdist`](raw::gfdist) for full documentation.
    pub fn gfdist(
        &mut self,
        target: &str,
        abcorr: &str,
        obsrvr: &str,
        relate: &str,
        refval: f64,
        adjust: f64,
        step: f64,
        cnfine: &Cell<f64>,
        mw: i32,
        result: &mut Cell<f64>,
    ) -> Result<()> {
        let nw = inc::gf::NWDIST;
        let mut work: Vec<f64> = vec![Default::default(); ((mw + 1 - LBCELL) * nw).max(0) as usize];
        raw::gfdist(
            self,
            target,
            abcorr,
            obsrvr,
            relate,
            refval,
            adjust,
            step,
            cnfine.as_raw_slice(),
            mw,
            nw,
            &mut work,
            result.as_raw_mut_slice(),
        )?;
        Ok(())
    }

    /// GF, Geometric event finder
    ///
    /// Determine time intervals when a specified geometric quantity
    /// satisfies a specified mathematical condition.
    ///
    /// See [`gfevnt`](raw::gfevnt) for full documentation.
    pub fn gfevnt(
        &mut self,
        udstep: fn(&mut f64, &mut f64, &mut Context) -> f2rust_std::Result<()>,
        udrefn: fn(f64, f64, bool, bool, &mut f64) -> (),
        gquant: &str,
        qnpars: i32,
        qpnams: &CharVec,
        qcpars: &CharVec,
        qdpars: &[f64],
        qipars: &[i32],
        qlpars: &[bool],
        op: &str,
        refval: f64,
        tol: f64,
        adjust: f64,
        cnfine: &Cell<f64>,
        rpt: bool,
        udrepi: fn(&[f64], &[u8], &[u8], &mut Context) -> f2rust_std::Result<()>,
        udrepu: fn(f64, f64, f64, &mut Context) -> f2rust_std::Result<()>,
        udrepf: fn(&mut Context) -> f2rust_std::Result<()>,
        mw: i32,
        bail: bool,
        udbail: fn() -> bool,
        result: &mut Cell<f64>,
    ) -> Result<()> {
        let nw = inc::gf::NWMAX;
        let mut work: Vec<f64> = vec![Default::default(); ((mw + 1 - LBCELL) * nw).max(0) as usize];
        raw::gfevnt(
            self,
            udstep,
            udrefn,
            gquant,
            qnpars,
            qpnams.as_arg(),
            qcpars.as_arg(),
            qdpars,
            qipars,
            qlpars,
            op,
            refval,
            tol,
            adjust,
            cnfine.as_raw_slice(),
            rpt,
            udrepi,
            udrepu,
            udrepf,
            mw,
            nw,
            &mut work,
            bail,
            udbail,
            result.as_raw_mut_slice(),
        )?;
        Ok(())
    }

    /// GF, is target in FOV?
    ///
    /// Determine time intervals when a specified target body or ray
    /// intersects the space bounded by the field-of-view (FOV) of a
    /// specified instrument. Report progress and handle interrupts if so
    /// commanded.
    ///
    /// See [`gffove`](raw::gffove) for full documentation.
    pub fn gffove(
        &mut self,
        inst: &str,
        tshape: &str,
        raydir: &[f64; 3],
        target: &str,
        tframe: &str,
        abcorr: &str,
        obsrvr: &str,
        tol: f64,
        udstep: fn(&mut f64, &mut f64, &mut Context) -> f2rust_std::Result<()>,
        udrefn: fn(f64, f64, bool, bool, &mut f64) -> (),
        rpt: bool,
        udrepi: fn(&[f64], &[u8], &[u8], &mut Context) -> f2rust_std::Result<()>,
        udrepu: fn(f64, f64, f64, &mut Context) -> f2rust_std::Result<()>,
        udrepf: fn(&mut Context) -> f2rust_std::Result<()>,
        bail: bool,
        udbail: fn() -> bool,
        cnfine: &Cell<f64>,
        result: &mut Cell<f64>,
    ) -> Result<()> {
        raw::gffove(
            self,
            inst,
            tshape,
            raydir,
            target,
            tframe,
            abcorr,
            obsrvr,
            tol,
            udstep,
            udrefn,
            rpt,
            udrepi,
            udrepu,
            udrepf,
            bail,
            udbail,
            cnfine.as_raw_slice(),
            result.as_raw_mut_slice(),
        )?;
        Ok(())
    }

    /// GF, illumination angle search
    ///
    /// Determine time intervals over which a specified constraint on
    /// the observed phase, solar incidence, or emission angle at
    /// a specified target body surface point is met.
    ///
    /// See [`gfilum`](raw::gfilum) for full documentation.
    pub fn gfilum(
        &mut self,
        method: &str,
        angtyp: &str,
        target: &str,
        illmn: &str,
        fixref: &str,
        abcorr: &str,
        obsrvr: &str,
        spoint: &[f64; 3],
        relate: &str,
        refval: f64,
        adjust: f64,
        step: f64,
        cnfine: &Cell<f64>,
        mw: i32,
        result: &mut Cell<f64>,
    ) -> Result<()> {
        let nw = inc::gf::NWILUM;
        let mut work: Vec<f64> = vec![Default::default(); ((mw + 1 - LBCELL) * nw).max(0) as usize];
        raw::gfilum(
            self,
            method,
            angtyp,
            target,
            illmn,
            fixref,
            abcorr,
            obsrvr,
            spoint,
            relate,
            refval,
            adjust,
            step,
            cnfine.as_raw_slice(),
            mw,
            nw,
            &mut work,
            result.as_raw_mut_slice(),
        )?;
        Ok(())
    }

    /// GF, occultation event
    ///
    /// Determine time intervals when an observer sees one target
    /// occulted by another. Report progress and handle interrupts
    /// if so commanded.
    ///
    /// See [`gfocce`](raw::gfocce) for full documentation.
    pub fn gfocce(
        &mut self,
        occtyp: &str,
        front: &str,
        fshape: &str,
        fframe: &str,
        back: &str,
        bshape: &str,
        bframe: &str,
        abcorr: &str,
        obsrvr: &str,
        tol: f64,
        udstep: fn(&mut f64, &mut f64, &mut Context) -> f2rust_std::Result<()>,
        udrefn: fn(f64, f64, bool, bool, &mut f64) -> (),
        rpt: bool,
        udrepi: fn(&[f64], &[u8], &[u8], &mut Context) -> f2rust_std::Result<()>,
        udrepu: fn(f64, f64, f64, &mut Context) -> f2rust_std::Result<()>,
        udrepf: fn(&mut Context) -> f2rust_std::Result<()>,
        bail: bool,
        udbail: fn() -> bool,
        cnfine: &Cell<f64>,
        result: &mut Cell<f64>,
    ) -> Result<()> {
        raw::gfocce(
            self,
            occtyp,
            front,
            fshape,
            fframe,
            back,
            bshape,
            bframe,
            abcorr,
            obsrvr,
            tol,
            udstep,
            udrefn,
            rpt,
            udrepi,
            udrepu,
            udrepf,
            bail,
            udbail,
            cnfine.as_raw_slice(),
            result.as_raw_mut_slice(),
        )?;
        Ok(())
    }

    /// GF, find occultation
    ///
    /// Determine time intervals when an observer sees one target body
    /// occulted by, or in transit across, another.
    ///
    /// The surfaces of the target bodies may be represented by triaxial
    /// ellipsoids or by topographic data provided by DSK files.
    ///
    /// See [`gfoclt`](raw::gfoclt) for full documentation.
    pub fn gfoclt(
        &mut self,
        occtyp: &str,
        front: &str,
        fshape: &str,
        fframe: &str,
        back: &str,
        bshape: &str,
        bframe: &str,
        abcorr: &str,
        obsrvr: &str,
        step: f64,
        cnfine: &Cell<f64>,
        result: &mut Cell<f64>,
    ) -> Result<()> {
        raw::gfoclt(
            self,
            occtyp,
            front,
            fshape,
            fframe,
            back,
            bshape,
            bframe,
            abcorr,
            obsrvr,
            step,
            cnfine.as_raw_slice(),
            result.as_raw_mut_slice(),
        )?;
        Ok(())
    }

    /// GF, phase angle search
    ///
    /// Determine time intervals for which a specified constraint
    /// on the phase angle between an illumination source, a target,
    /// and observer body centers is met.
    ///
    /// See [`gfpa`](raw::gfpa) for full documentation.
    pub fn gfpa(
        &mut self,
        target: &str,
        illmn: &str,
        abcorr: &str,
        obsrvr: &str,
        relate: &str,
        refval: f64,
        adjust: f64,
        step: f64,
        cnfine: &Cell<f64>,
        mw: i32,
        result: &mut Cell<f64>,
    ) -> Result<()> {
        let nw = inc::gf::NWPA;
        let mut work: Vec<f64> = vec![Default::default(); ((mw + 1 - LBCELL) * nw).max(0) as usize];
        raw::gfpa(
            self,
            target,
            illmn,
            abcorr,
            obsrvr,
            relate,
            refval,
            adjust,
            step,
            cnfine.as_raw_slice(),
            mw,
            nw,
            &mut work,
            result.as_raw_mut_slice(),
        )?;
        Ok(())
    }

    /// GF, observer-target vector coordinate search
    ///
    /// Determine time intervals for which a coordinate of an
    /// observer-target position vector satisfies a numerical constraint.
    ///
    /// See [`gfposc`](raw::gfposc) for full documentation.
    pub fn gfposc(
        &mut self,
        target: &str,
        frame: &str,
        abcorr: &str,
        obsrvr: &str,
        crdsys: &str,
        coord: &str,
        relate: &str,
        refval: f64,
        adjust: f64,
        step: f64,
        cnfine: &Cell<f64>,
        mw: i32,
        result: &mut Cell<f64>,
    ) -> Result<()> {
        let nw = inc::gf::NWMAX;
        let mut work: Vec<f64> = vec![Default::default(); ((mw + 1 - LBCELL) * nw).max(0) as usize];
        raw::gfposc(
            self,
            target,
            frame,
            abcorr,
            obsrvr,
            crdsys,
            coord,
            relate,
            refval,
            adjust,
            step,
            cnfine.as_raw_slice(),
            mw,
            nw,
            &mut work,
            result.as_raw_mut_slice(),
        )?;
        Ok(())
    }

    /// GF, default refinement estimator
    ///
    /// Estimate, using a bisection method, the next abscissa value at
    /// which a state change occurs. This is the default GF refinement
    /// method.
    ///
    /// Returns `t`.
    ///
    /// See [`gfrefn`](raw::gfrefn) for full documentation.
    pub fn gfrefn(&self, t1: f64, t2: f64, s1: bool, s2: bool) -> f64 {
        let mut t: f64 = Default::default();
        raw::gfrefn(t1, t2, s1, s2, &mut t);
        t
    }

    /// GF, is ray in FOV?
    ///
    /// Determine time intervals when a specified ray intersects the
    /// space bounded by the field-of-view (FOV) of a specified
    /// instrument.
    ///
    /// See [`gfrfov`](raw::gfrfov) for full documentation.
    pub fn gfrfov(
        &mut self,
        inst: &str,
        raydir: &[f64; 3],
        rframe: &str,
        abcorr: &str,
        obsrvr: &str,
        step: f64,
        cnfine: &Cell<f64>,
        result: &mut Cell<f64>,
    ) -> Result<()> {
        raw::gfrfov(
            self,
            inst,
            raydir,
            rframe,
            abcorr,
            obsrvr,
            step,
            cnfine.as_raw_slice(),
            result.as_raw_mut_slice(),
        )?;
        Ok(())
    }

    /// GF, progress report initialization
    ///
    /// Initialize a search progress report.
    ///
    /// See [`gfrepi`](raw::gfrepi) for full documentation.
    pub fn gfrepi(&mut self, window: &Cell<f64>, begmss: &str, endmss: &str) -> Result<()> {
        raw::gfrepi(self, window.as_raw_slice(), begmss, endmss)?;
        Ok(())
    }

    /// GF, progress report update
    ///
    /// Tell the progress reporting system how far a search has
    /// progressed.
    ///
    /// See [`gfrepu`](raw::gfrepu) for full documentation.
    pub fn gfrepu(&mut self, ivbeg: f64, ivend: f64, time: f64) -> Result<()> {
        raw::gfrepu(self, ivbeg, ivend, time)?;
        Ok(())
    }

    /// GF, progress report finalization
    ///
    /// Finish a progress report.
    ///
    /// See [`gfrepf`](raw::gfrepf) for full documentation.
    pub fn gfrepf(&mut self) -> Result<()> {
        raw::gfrepf(self)?;
        Ok(())
    }

    /// GF, range rate search
    ///
    /// Determine time intervals for which a specified constraint
    /// on the observer-target range rate is met.
    ///
    /// See [`gfrr`](raw::gfrr) for full documentation.
    pub fn gfrr(
        &mut self,
        target: &str,
        abcorr: &str,
        obsrvr: &str,
        relate: &str,
        refval: f64,
        adjust: f64,
        step: f64,
        cnfine: &Cell<f64>,
        mw: i32,
        result: &mut Cell<f64>,
    ) -> Result<()> {
        let nw = inc::gf::NWRR;
        let mut work: Vec<f64> = vec![Default::default(); ((mw + 1 - LBCELL) * nw).max(0) as usize];
        raw::gfrr(
            self,
            target,
            abcorr,
            obsrvr,
            relate,
            refval,
            adjust,
            step,
            cnfine.as_raw_slice(),
            mw,
            nw,
            &mut work,
            result.as_raw_mut_slice(),
        )?;
        Ok(())
    }

    /// GF, angular separation search
    ///
    /// Determine time intervals when the angular separation between
    /// the position vectors of two target bodies relative to an observer
    /// satisfies a numerical relationship.
    ///
    /// See [`gfsep`](raw::gfsep) for full documentation.
    pub fn gfsep(
        &mut self,
        targ1: &str,
        shape1: &str,
        frame1: &str,
        targ2: &str,
        shape2: &str,
        frame2: &str,
        abcorr: &str,
        obsrvr: &str,
        relate: &str,
        refval: f64,
        adjust: f64,
        step: f64,
        cnfine: &Cell<f64>,
        mw: i32,
        result: &mut Cell<f64>,
    ) -> Result<()> {
        let nw = inc::gf::NWSEP;
        let mut work: Vec<f64> = vec![Default::default(); ((mw + 1 - LBCELL) * nw).max(0) as usize];
        raw::gfsep(
            self,
            targ1,
            shape1,
            frame1,
            targ2,
            shape2,
            frame2,
            abcorr,
            obsrvr,
            relate,
            refval,
            adjust,
            step,
            cnfine.as_raw_slice(),
            mw,
            nw,
            &mut work,
            result.as_raw_mut_slice(),
        )?;
        Ok(())
    }

    /// GF, surface intercept vector coordinate search
    ///
    /// Determine time intervals for which a coordinate of a
    /// surface intercept position vector satisfies a numerical
    /// constraint.
    ///
    /// See [`gfsntc`](raw::gfsntc) for full documentation.
    pub fn gfsntc(
        &mut self,
        target: &str,
        fixref: &str,
        method: &str,
        abcorr: &str,
        obsrvr: &str,
        dref: &str,
        dvec: &[f64; 3],
        crdsys: &str,
        coord: &str,
        relate: &str,
        refval: f64,
        adjust: f64,
        step: f64,
        cnfine: &Cell<f64>,
        mw: i32,
        result: &mut Cell<f64>,
    ) -> Result<()> {
        let nw = inc::gf::NWMAX;
        let mut work: Vec<f64> = vec![Default::default(); ((mw + 1 - LBCELL) * nw).max(0) as usize];
        raw::gfsntc(
            self,
            target,
            fixref,
            method,
            abcorr,
            obsrvr,
            dref,
            dvec,
            crdsys,
            coord,
            relate,
            refval,
            adjust,
            step,
            cnfine.as_raw_slice(),
            mw,
            nw,
            &mut work,
            result.as_raw_mut_slice(),
        )?;
        Ok(())
    }

    /// GF, step size
    ///
    /// Return the time step set by the most recent call to GFSSTP.
    ///
    /// Returns `step`.
    ///
    /// See [`gfstep`](raw::gfstep) for full documentation.
    pub fn gfstep(&mut self, time: &mut f64) -> Result<f64> {
        let mut step: f64 = Default::default();
        raw::gfstep(self, time, &mut step)?;
        Ok(step)
    }

    /// Geometry finder set step size
    ///
    /// Set the step size to be returned by GFSTEP.
    ///
    /// See [`gfsstp`](raw::gfsstp) for full documentation.
    pub fn gfsstp(&mut self, step: f64) -> Result<()> {
        raw::gfsstp(self, step)?;
        Ok(())
    }

    /// GF, set a tolerance value for GF
    ///
    /// Override the default GF convergence value used in the high
    /// level GF routines.
    ///
    /// See [`gfstol`](raw::gfstol) for full documentation.
    pub fn gfstol(&mut self, value: f64) -> Result<()> {
        raw::gfstol(self, value)?;
        Ok(())
    }

    /// GF, subpoint vector coordinate search
    ///
    /// Determine time intervals for which a coordinate of an
    /// subpoint position vector satisfies a numerical constraint.
    ///
    /// See [`gfsubc`](raw::gfsubc) for full documentation.
    pub fn gfsubc(
        &mut self,
        target: &str,
        fixref: &str,
        method: &str,
        abcorr: &str,
        obsrvr: &str,
        crdsys: &str,
        coord: &str,
        relate: &str,
        refval: f64,
        adjust: f64,
        step: f64,
        cnfine: &Cell<f64>,
        mw: i32,
        result: &mut Cell<f64>,
    ) -> Result<()> {
        let nw = inc::gf::NWMAX;
        let mut work: Vec<f64> = vec![Default::default(); ((mw + 1 - LBCELL) * nw).max(0) as usize];
        raw::gfsubc(
            self,
            target,
            fixref,
            method,
            abcorr,
            obsrvr,
            crdsys,
            coord,
            relate,
            refval,
            adjust,
            step,
            cnfine.as_raw_slice(),
            mw,
            nw,
            &mut work,
            result.as_raw_mut_slice(),
        )?;
        Ok(())
    }

    /// GF, is target in FOV?
    ///
    /// Determine time intervals when a specified ephemeris object
    /// intersects the space bounded by the field-of-view (FOV) of a
    /// specified instrument.
    ///
    /// See [`gftfov`](raw::gftfov) for full documentation.
    pub fn gftfov(
        &mut self,
        inst: &str,
        target: &str,
        tshape: &str,
        tframe: &str,
        abcorr: &str,
        obsrvr: &str,
        step: f64,
        cnfine: &Cell<f64>,
        result: &mut Cell<f64>,
    ) -> Result<()> {
        raw::gftfov(
            self,
            inst,
            target,
            tshape,
            tframe,
            abcorr,
            obsrvr,
            step,
            cnfine.as_raw_slice(),
            result.as_raw_mut_slice(),
        )?;
        Ok(())
    }

    /// GF, user defined boolean
    ///
    /// Perform a GF search on a user defined boolean quantity.
    ///
    /// See [`gfudb`](raw::gfudb) for full documentation.
    pub fn gfudb(
        &mut self,
        udfuns: fn(&mut f64, &mut f64, &mut Context) -> f2rust_std::Result<()>,
        udfunb: fn(
            fn(&mut f64, &mut f64, &mut Context) -> f2rust_std::Result<()>,
            &mut f64,
            &mut bool,
            &mut Context,
        ) -> f2rust_std::Result<()>,
        step: f64,
        cnfine: &Cell<f64>,
        result: &mut Cell<f64>,
    ) -> Result<()> {
        raw::gfudb(
            self,
            udfuns,
            udfunb,
            step,
            cnfine.as_raw_slice(),
            result.as_raw_mut_slice(),
        )?;
        Ok(())
    }

    /// GF, user defined scalar
    ///
    /// Perform a GF search on a user defined scalar quantity.
    ///
    /// See [`gfuds`](raw::gfuds) for full documentation.
    pub fn gfuds(
        &mut self,
        udfuns: fn(&mut f64, &mut f64, &mut Context) -> f2rust_std::Result<()>,
        udqdec: fn(
            fn(&mut f64, &mut f64, &mut Context) -> f2rust_std::Result<()>,
            &mut f64,
            &mut bool,
            &mut Context,
        ) -> f2rust_std::Result<()>,
        relate: &str,
        refval: f64,
        adjust: f64,
        step: f64,
        cnfine: &Cell<f64>,
        mw: i32,
        result: &mut Cell<f64>,
    ) -> Result<()> {
        let nw = inc::gf::NWMAX;
        let mut work: Vec<f64> = vec![Default::default(); ((mw + 1 - LBCELL) * nw).max(0) as usize];
        raw::gfuds(
            self,
            udfuns,
            udqdec,
            relate,
            refval,
            adjust,
            step,
            cnfine.as_raw_slice(),
            mw,
            nw,
            &mut work,
            result.as_raw_mut_slice(),
        )?;
        Ok(())
    }

    /// Half the value of pi
    ///
    /// Return half the value of pi (the ratio of the circumference of
    /// a circle to its diameter).
    ///
    /// See [`halfpi`](raw::halfpi) for full documentation.
    pub fn halfpi(&mut self) -> f64 {
        raw::halfpi(self)
    }

    /// Hermite polynomial interpolation, equal spacing
    ///
    /// Evaluate, at a specified point, a Hermite interpolating polynomial
    /// for a specified set of equally spaced abscissa values and
    /// corresponding pairs of function and function derivative values.
    ///
    /// Returns `(f, df)`.
    ///
    /// See [`hrmesp`](raw::hrmesp) for full documentation.
    pub fn hrmesp(
        &mut self,
        n: i32,
        first: f64,
        step: f64,
        yvals: &[f64],
        x: f64,
        work: &mut [f64],
    ) -> Result<(f64, f64)> {
        let mut f: f64 = Default::default();
        let mut df: f64 = Default::default();
        raw::hrmesp(self, n, first, step, yvals, x, work, &mut f, &mut df)?;
        Ok((f, df))
    }

    /// Hermite polynomial interpolation
    ///
    /// Evaluate a Hermite interpolating polynomial at a specified
    /// abscissa value.
    ///
    /// Returns `(f, df)`.
    ///
    /// See [`hrmint`](raw::hrmint) for full documentation.
    pub fn hrmint(
        &mut self,
        n: i32,
        xvals: &[f64],
        yvals: &[f64],
        x: f64,
        work: &mut [f64],
    ) -> Result<(f64, f64)> {
        let mut f: f64 = Default::default();
        let mut df: f64 = Default::default();
        raw::hrmint(self, n, xvals, yvals, x, work, &mut f, &mut df)?;
        Ok((f, df))
    }

    /// Hexadecimal string to d.p. number
    ///
    /// Convert a string representing a double precision number in a
    /// base 16 "scientific notation" into its equivalent double
    /// precision number.
    ///
    /// Returns `(number, error, errmsg)`.
    ///
    /// See [`hx2dp`](raw::hx2dp) for full documentation.
    pub fn hx2dp(&mut self, string: &str) -> (f64, bool, String) {
        let mut number: f64 = Default::default();
        let mut error: bool = Default::default();
        let mut errmsg = blank(inc::errhnd::LMSGLN);
        raw::hx2dp(self, string, &mut number, &mut error, &mut errmsg);
        (number, error, trim(errmsg))
    }

    /// Signed hexadecimal string to integer
    ///
    /// Convert a signed hexadecimal string representation of an integer
    /// to its equivalent integer.
    ///
    /// Returns `(number, error, errmsg)`.
    ///
    /// See [`hx2int`](raw::hx2int) for full documentation.
    pub fn hx2int(&mut self, string: &str) -> (i32, bool, String) {
        let mut number: i32 = Default::default();
        let mut error: bool = Default::default();
        let mut errmsg = blank(inc::errhnd::LMSGLN);
        raw::hx2int(self, string, &mut number, &mut error, &mut errmsg);
        (number, error, trim(errmsg))
    }

    /// Hyperbolic time of flight
    ///
    /// Solve the time of flight equation MA = e sinh(F) - F for the
    /// hyperbolic eccentric anomaly F, given the mean anomaly, MA,
    /// and the eccentricity, e.
    ///
    /// Returns `f`.
    ///
    /// See [`hyptof`](raw::hyptof) for full documentation.
    pub fn hyptof(&mut self, ma: f64, ecc: f64) -> Result<f64> {
        let mut f: f64 = Default::default();
        raw::hyptof(self, ma, ecc, &mut f)?;
        Ok(f)
    }

    /// Return the 3x3 identity matrix
    ///
    /// Return the 3x3 identity matrix.
    ///
    /// Returns `matrix`.
    ///
    /// See [`ident`](raw::ident) for full documentation.
    pub fn ident(&self) -> [[f64; 3]; 3] {
        let mut matrix: [[f64; 3]; 3] = Default::default();
        raw::ident(&mut matrix);
        matrix
    }

    /// Get file architecture and type from ID word
    ///
    /// Extract the architecture and type of a SPICE binary kernel file
    /// from a file ID word.
    ///
    /// Returns `(arch, type_)`.
    ///
    /// See [`idw2at`](raw::idw2at) for full documentation.
    pub fn idw2at(&mut self, idword: &str) -> Result<(String, String)> {
        let mut arch = blank(3);
        let mut type_ = blank(4);
        raw::idw2at(self, idword, &mut arch, &mut type_)?;
        Ok((trim(arch), trim(type_)))
    }

    /// Illumination angles, general source, return flags
    ///
    /// Compute the illumination angles---phase, incidence, and
    /// emission---at a specified point on a target body. Return logical
    /// flags indicating whether the surface point is visible from
    /// the observer's position and whether the surface point is
    /// illuminated.
    ///
    /// The target body's surface is represented using topographic data
    /// provided by DSK files, or by a reference ellipsoid.
    ///
    /// The illumination source is a specified ephemeris object.
    ///
    /// Returns `(trgepc, srfvec, phase, incdnc, emissn, visibl, lit)`.
    ///
    /// See [`illumf`](raw::illumf) for full documentation.
    pub fn illumf(
        &mut self,
        method: &str,
        target: &str,
        ilusrc: &str,
        et: f64,
        fixref: &str,
        abcorr: &str,
        obsrvr: &str,
        spoint: &[f64; 3],
    ) -> Result<(f64, [f64; 3], f64, f64, f64, bool, bool)> {
        let mut trgepc: f64 = Default::default();
        let mut srfvec: [f64; 3] = Default::default();
        let mut phase: f64 = Default::default();
        let mut incdnc: f64 = Default::default();
        let mut emissn: f64 = Default::default();
        let mut visibl: bool = Default::default();
        let mut lit: bool = Default::default();
        raw::illumf(
            self,
            method,
            target,
            ilusrc,
            et,
            fixref,
            abcorr,
            obsrvr,
            spoint,
            &mut trgepc,
            &mut srfvec,
            &mut phase,
            &mut incdnc,
            &mut emissn,
            &mut visibl,
            &mut lit,
        )?;
        Ok((trgepc, srfvec, phase, incdnc, emissn, visibl, lit))
    }

    /// Illumination angles, general source
    ///
    /// Find the illumination angles (phase, incidence, and
    /// emission) at a specified surface point of a target body.
    ///
    /// The surface of the target body may be represented by a triaxial
    /// ellipsoid or by topographic data provided by DSK files.
    ///
    /// The illumination source is a specified ephemeris object.
    ///
    /// Returns `(trgepc, srfvec, phase, incdnc, emissn)`.
    ///
    /// See [`illumg`](raw::illumg) for full documentation.
    pub fn illumg(
        &mut self,
        method: &str,
        target: &str,
        ilusrc: &str,
        et: f64,
        fixref: &str,
        abcorr: &str,
        obsrvr: &str,
        spoint: &[f64; 3],
    ) -> Result<(f64, [f64; 3], f64, f64, f64)> {
        let mut trgepc: f64 = Default::default();
        let mut srfvec: [f64; 3] = Default::default();
        let mut phase: f64 = Default::default();
        let mut incdnc: f64 = Default::default();
        let mut emissn: f64 = Default::default();
        raw::illumg(
            self,
            method,
            target,
            ilusrc,
            et,
            fixref,
            abcorr,
            obsrvr,
            spoint,
            &mut trgepc,
            &mut srfvec,
            &mut phase,
            &mut incdnc,
            &mut emissn,
        )?;
        Ok((trgepc, srfvec, phase, incdnc, emissn))
    }

    /// Illumination angles
    ///
    /// Find the illumination angles (phase, solar incidence, and
    /// emission) at a specified surface point of a target body.
    ///
    /// This routine supersedes ILLUM.
    ///
    /// Returns `(trgepc, srfvec, phase, incdnc, emissn)`.
    ///
    /// See [`ilumin`](raw::ilumin) for full documentation.
    pub fn ilumin(
        &mut self,
        method: &str,
        target: &str,
        et: f64,
        fixref: &str,
        abcorr: &str,
        obsrvr: &str,
        spoint: &[f64; 3],
    ) -> Result<(f64, [f64; 3], f64, f64, f64)> {
        let mut trgepc: f64 = Default::default();
        let mut srfvec: [f64; 3] = Default::default();
        let mut phase: f64 = Default::default();
        let mut incdnc: f64 = Default::default();
        let mut emissn: f64 = Default::default();
        raw::ilumin(
            self,
            method,
            target,
            et,
            fixref,
            abcorr,
            obsrvr,
            spoint,
            &mut trgepc,
            &mut srfvec,
            &mut phase,
            &mut incdnc,
            &mut emissn,
        )?;
        Ok((trgepc, srfvec, phase, incdnc, emissn))
    }

    /// Intersection of cone and line segment
    ///
    /// Compute the points of intersection of a specified nappe of a cone
    /// and a line segment.
    ///
    /// Returns `(nxpts, xpt1, xpt2)`.
    ///
    /// See [`incnsg`](raw::incnsg) for full documentation.
    pub fn incnsg(
        &mut self,
        apex: &[f64; 3],
        axis: &[f64; 3],
        angle: f64,
        endpt1: &[f64; 3],
        endpt2: &[f64; 3],
    ) -> Result<(i32, [f64; 3], [f64; 3])> {
        let mut nxpts: i32 = Default::default();
        let mut xpt1: [f64; 3] = Default::default();
        let mut xpt2: [f64; 3] = Default::default();
        raw::incnsg(
            self, apex, axis, angle, endpt1, endpt2, &mut nxpts, &mut xpt1, &mut xpt2,
        )?;
        Ok((nxpts, xpt1, xpt2))
    }

    /// Intersection of ellipsoid and plane
    ///
    /// Find the intersection of a triaxial ellipsoid and a plane.
    ///
    /// Returns `ellips`.
    ///
    /// See [`inedpl`](raw::inedpl) for full documentation.
    pub fn inedpl(&mut self, a: f64, b: f64, c: f64, plane: &[f64; 4]) -> Result<Option<[f64; 9]>> {
        let mut ellips: [f64; 9] = Default::default();
        let mut found: bool = Default::default();
        raw::inedpl(self, a, b, c, plane, &mut ellips, &mut found)?;
        Ok(if found { Some(ellips) } else { None })
    }

    /// Intersection of ellipse and plane
    ///
    /// Find the intersection of an ellipse and a plane.
    ///
    /// Returns `(nxpts, xpt1, xpt2)`.
    ///
    /// See [`inelpl`](raw::inelpl) for full documentation.
    pub fn inelpl(
        &mut self,
        ellips: &[f64; 9],
        plane: &[f64; 4],
    ) -> Result<(i32, [f64; 3], [f64; 3])> {
        let mut nxpts: i32 = Default::default();
        let mut xpt1: [f64; 3] = Default::default();
        let mut xpt2: [f64; 3] = Default::default();
        raw::inelpl(self, ellips, plane, &mut nxpts, &mut xpt1, &mut xpt2)?;
        Ok((nxpts, xpt1, xpt2))
    }

    /// Intersection of ray and plane
    ///
    /// Find the intersection of a ray and a plane.
    ///
    /// Returns `(nxpts, xpt)`.
    ///
    /// See [`inrypl`](raw::inrypl) for full documentation.
    pub fn inrypl(
        &mut self,
        vertex: &[f64; 3],
        dir: &[f64; 3],
        plane: &[f64; 4],
    ) -> Result<(i32, [f64; 3])> {
        let mut nxpts: i32 = Default::default();
        let mut xpt: [f64; 3] = Default::default();
        raw::inrypl(self, vertex, dir, plane, &mut nxpts, &mut xpt)?;
        Ok((nxpts, xpt))
    }

    /// Inside Tetrahedral Angle
    ///
    /// Determine if a given vector lies inside the solid tetrahedral
    /// angle determined by 3 vectors. If it does, return the
    /// point where the scale factor such that SCALE*V lies in the
    /// plane spanned by E1, E2, and E3.
    ///
    /// Returns `scale`.
    ///
    /// See [`insang`](raw::insang) for full documentation.
    pub fn insang(&self, v: &[f64; 3], e1: &[f64; 3], e2: &[f64; 3], e3: &[f64; 3]) -> Option<f64> {
        let mut found: bool = Default::default();
        let mut scale: f64 = Default::default();
        raw::insang(v, e1, e2, e3, &mut found, &mut scale);
        if found { Some(scale) } else { None }
    }

    /// Insert at location in a character array
    ///
    /// Insert one or more elements into a character array at the
    /// indicated location.
    ///
    /// See [`inslac`](raw::inslac) for full documentation.
    pub fn inslac(
        &mut self,
        elts: &CharVec,
        ne: i32,
        loc: i32,
        array: &mut CharVec,
        na: &mut i32,
    ) -> Result<()> {
        raw::inslac(self, elts.as_arg(), ne, loc, array.as_arg_mut(), na)?;
        Ok(())
    }

    /// Insert at location in double precision array
    ///
    /// Insert one or more elements into a double precision array at
    /// the indicated location.
    ///
    /// See [`inslad`](raw::inslad) for full documentation.
    pub fn inslad(
        &mut self,
        elts: &[f64],
        ne: i32,
        loc: i32,
        array: &mut [f64],
        na: &mut i32,
    ) -> Result<()> {
        raw::inslad(self, elts, ne, loc, array, na)?;
        Ok(())
    }

    /// Insert at location in an integer array
    ///
    /// Insert one or more elements into an integer array at
    /// the indicated location.
    ///
    /// See [`inslai`](raw::inslai) for full documentation.
    pub fn inslai(
        &mut self,
        elts: &[i32],
        ne: i32,
        loc: i32,
        array: &mut [i32],
        na: &mut i32,
    ) -> Result<()> {
        raw::inslai(self, elts, ne, loc, array, na)?;
        Ok(())
    }

    /// Insert an item into a character set
    ///
    /// Insert an item into a character set.
    ///
    /// See [`insrtc`](raw::insrtc) for full documentation.
    pub fn insrtc(&mut self, item: &str, a: &mut CharCell) -> Result<()> {
        raw::insrtc(self, item, a.as_arg_mut())?;
        Ok(())
    }

    /// Insert an item into a double precision set
    ///
    /// Insert an item into a double precision set.
    ///
    /// See [`insrtd`](raw::insrtd) for full documentation.
    pub fn insrtd(&mut self, item: f64, a: &mut Cell<f64>) -> Result<()> {
        raw::insrtd(self, item, a.as_raw_mut_slice())?;
        Ok(())
    }

    /// Insert an item into an integer set
    ///
    /// Insert an item into an integer set.
    ///
    /// See [`insrti`](raw::insrti) for full documentation.
    pub fn insrti(&mut self, item: i32, a: &mut Cell<i32>) -> Result<()> {
        raw::insrti(self, item, a.as_raw_mut_slice())?;
        Ok(())
    }

    /// Integer to signed hexadecimal string
    ///
    /// Convert an integer to an equivalent signed hexadecimal string.
    ///
    /// Returns `(string, length)`.
    ///
    /// See [`int2hx`](raw::int2hx) for full documentation.
    pub fn int2hx(&mut self, number: i32) -> (String, i32) {
        let mut string = blank(9);
        let mut length: i32 = Default::default();
        raw::int2hx(self, number, &mut string, &mut length);
        (trim(string), length)
    }

    /// Intersect two character sets
    ///
    /// Intersect two character sets to form a third set.
    ///
    /// See [`interc`](raw::interc) for full documentation.
    pub fn interc(&mut self, a: &CharCell, b: &CharCell, c: &mut CharCell) -> Result<()> {
        raw::interc(self, a.as_arg(), b.as_arg(), c.as_arg_mut())?;
        Ok(())
    }

    /// Intersect two double precision sets
    ///
    /// Intersect two double precision sets to form a third set.
    ///
    /// See [`interd`](raw::interd) for full documentation.
    pub fn interd(&mut self, a: &Cell<f64>, b: &Cell<f64>, c: &mut Cell<f64>) -> Result<()> {
        raw::interd(
            self,
            a.as_raw_slice(),
            b.as_raw_slice(),
            c.as_raw_mut_slice(),
        )?;
        Ok(())
    }

    /// Intersect two integer sets
    ///
    /// Intersect two integer sets to form a third set.
    ///
    /// See [`interi`](raw::interi) for full documentation.
    pub fn interi(&mut self, a: &Cell<i32>, b: &Cell<i32>, c: &mut Cell<i32>) -> Result<()> {
        raw::interi(
            self,
            a.as_raw_slice(),
            b.as_raw_slice(),
            c.as_raw_mut_slice(),
        )?;
        Ok(())
    }

    /// Largest integer number
    ///
    /// Return the value of the largest (positive) number representable
    /// in an integer variable.
    ///
    /// See [`intmax`](raw::intmax) for full documentation.
    pub fn intmax(&self) -> i32 {
        raw::intmax()
    }

    /// Smallest integer number
    ///
    /// Return the value of the smallest (negative) number representable
    /// in an integer variable.
    ///
    /// See [`intmin`](raw::intmin) for full documentation.
    pub fn intmin(&self) -> i32 {
        raw::intmin()
    }

    /// Convert an integer to ordinal text
    ///
    /// Convert an integer to an equivalent written ordinal phrase.
    /// For example, convert 121 to 'ONE HUNDRED TWENTY-FIRST'.
    ///
    /// Returns `string`.
    ///
    /// See [`intord`](raw::intord) for full documentation.
    pub fn intord(&mut self, n: i32) -> String {
        let mut string = blank(148);
        raw::intord(self, n, &mut string);
        trim(string)
    }

    /// Integer to character string
    ///
    /// Convert an integer to an equivalent character string.
    ///
    /// Returns `string`.
    ///
    /// See [`intstr`](raw::intstr) for full documentation.
    pub fn intstr(&mut self, number: i32) -> String {
        let mut string = blank(80);
        raw::intstr(self, number, &mut string);
        trim(string)
    }

    /// Convert an integer to text
    ///
    /// Convert an integer to an equivalent written phrase.
    /// For example, convert 121 to 'ONE HUNDRED TWENTY-ONE'.
    ///
    /// Returns `string`.
    ///
    /// See [`inttxt`](raw::inttxt) for full documentation.
    pub fn inttxt(&mut self, n: i32) -> String {
        let mut string = blank(146);
        raw::inttxt(self, n, &mut string);
        trim(string)
    }

    /// Invert a 3x3 matrix
    ///
    /// Generate the inverse of a 3x3 matrix.
    ///
    /// Returns `mout`.
    ///
    /// See [`invert`](raw::invert) for full documentation.
    pub fn invert(&self, m: &[[f64; 3]; 3]) -> [[f64; 3]; 3] {
        let mut mout: [[f64; 3]; 3] = Default::default();
        raw::invert(m, &mut mout);
        mout
    }

    /// Invert nearly orthogonal matrices
    ///
    /// Construct the inverse of a 3x3 matrix with orthogonal columns and
    /// non-zero column norms using a numerically stable algorithm. The
    /// rows of the output matrix are the columns of the input matrix
    /// divided by the length squared of the corresponding columns.
    ///
    /// Returns `mit`.
    ///
    /// See [`invort`](raw::invort) for full documentation.
    pub fn invort(&mut self, m: &[[f64; 3]; 3]) -> Result<[[f64; 3]; 3]> {
        let mut mit: [[f64; 3]; 3] = Default::default();
        raw::invort(self, m, &mut mit)?;
        Ok(mit)
    }

    /// Inverse of state transformation matrix
    ///
    /// Return the inverse of a state transformation matrix.
    ///
    /// Returns `invmat`.
    ///
    /// See [`invstm`](raw::invstm) for full documentation.
    pub fn invstm(&mut self, mat: &[[f64; 6]; 6]) -> Result<[[f64; 6]; 6]> {
        let mut invmat: [[f64; 6]; 6] = Default::default();
        raw::invstm(self, mat, &mut invmat)?;
        Ok(invmat)
    }

    /// I/O error message writer
    ///
    /// Set the long error message equal to a standard I/O error message
    /// composed from an action, the name of a file, and a value of
    /// IOSTAT.
    ///
    /// See [`ioerr`](raw::ioerr) for full documentation.
    pub fn ioerr(&mut self, action: &str, file: &str, iostat: i32) {
        raw::ioerr(self, action, file, iostat);
    }

    /// Inertial reference frame transformation
    ///
    /// Return the matrix that transforms vectors from one specified
    /// inertial reference frame to another.
    ///
    /// Returns `rotab`.
    ///
    /// See [`irftrn`](raw::irftrn) for full documentation.
    pub fn irftrn(&mut self, refa: &str, refb: &str) -> Result<[[f64; 3]; 3]> {
        let mut rotab: [[f64; 3]; 3] = Default::default();
        raw::irftrn(self, refa, refb, &mut rotab)?;
        Ok(rotab)
    }

    /// Is a file currently open?
    ///
    /// Determine whether a named file is currently open.
    ///
    /// See [`isopen`](raw::isopen) for full documentation.
    pub fn isopen(&mut self, file: &str) -> Result<bool> {
        raw::isopen(self, file)
    }

    /// Is it an order vector
    ///
    /// Determine whether an array of N items contains the integers
    /// 1 through N.
    ///
    /// See [`isordv`](raw::isordv) for full documentation.
    pub fn isordv(&self, array: &mut [i32], n: i32) -> bool {
        raw::isordv(array, n)
    }

    /// Search in a character array
    ///
    /// Search for a given value within a character string array. Return
    /// the index of the first matching array entry, or zero if the key
    /// value was not found.
    ///
    /// See [`isrchc`](raw::isrchc) for full documentation.
    pub fn isrchc(&self, value: &str, ndim: i32, array: &CharVec) -> i32 {
        raw::isrchc(value, ndim, array.as_arg())
    }

    /// Search in a double precision array
    ///
    /// Search for a given value within a double precision array. Return
    /// the index of the first matching array entry, or zero if the key
    /// value was not found.
    ///
    /// See [`isrchd`](raw::isrchd) for full documentation.
    pub fn isrchd(&self, value: f64, ndim: i32, array: &[f64]) -> i32 {
        raw::isrchd(value, ndim, array)
    }

    /// Search in an integer array
    ///
    /// Search for a given value within a integer array. Return
    /// the index of the first matching array entry, or zero if
    /// the key value was not found.
    ///
    /// See [`isrchi`](raw::isrchi) for full documentation.
    pub fn isrchi(&self, value: i32, ndim: i32, array: &[i32]) -> i32 {
        raw::isrchi(value, ndim, array)
    }

    /// Indicate whether a matrix is a rotation matrix
    ///
    /// Indicate whether a 3x3 matrix is a rotation matrix.
    ///
    /// See [`isrot`](raw::isrot) for full documentation.
    pub fn isrot(&mut self, m: &[[f64; 3]; 3], ntol: f64, dtol: f64) -> Result<bool> {
        raw::isrot(self, m, ntol, dtol)
    }

    /// Julian Date of 1900.0 JAN 0.5
    ///
    /// Return the Julian Date of 1899 DEC 31 12:00:00 (1900 JAN 0.5).
    ///
    /// See [`j1900`](raw::j1900) for full documentation.
    pub fn j1900(&self) -> f64 {
        raw::j1900()
    }

    /// Julian Date of 1950.0 JAN 1.0
    ///
    /// Return the Julian Date of 1950 JAN 01 00:00:00 (1950 JAN 1.0).
    ///
    /// See [`j1950`](raw::j1950) for full documentation.
    pub fn j1950(&self) -> f64 {
        raw::j1950()
    }

    /// Julian Date of 2000 JAN 1.5
    ///
    /// Return the Julian Date of 2000 JAN 01 12:00:00 (2000 JAN 1.5).
    ///
    /// See [`j2000`](raw::j2000) for full documentation.
    pub fn j2000(&self) -> f64 {
        raw::j2000()
    }

    /// Julian Date of 2100 JAN 1.5
    ///
    /// Return the Julian Date of 2100 JAN 01 12:00:00 (2100 JAN 1.5).
    ///
    /// See [`j2100`](raw::j2100) for full documentation.
    pub fn j2100(&self) -> f64 {
        raw::j2100()
    }

    /// Julian to Gregorian Calendar
    ///
    /// Convert Year Month and Day on the Julian Calendar
    /// to the Gregorian Calendar
    ///
    /// Returns `doy`.
    ///
    /// See [`jul2gr`](raw::jul2gr) for full documentation.
    pub fn jul2gr(&mut self, year: &mut i32, month: &mut i32, day: &mut i32) -> Result<i32> {
        let mut doy: i32 = Default::default();
        raw::jul2gr(self, year, month, day, &mut doy)?;
        Ok(doy)
    }

    /// Gregorian to Julian Calendar
    ///
    /// Convert Year Month and Day on the  Gregorian Calendar
    /// to the Julian Calendar
    ///
    /// Returns `doy`.
    ///
    /// See [`gr2jul`](raw::gr2jul) for full documentation.
    pub fn gr2jul(&mut self, year: &mut i32, month: &mut i32, day: &mut i32) -> Result<i32> {
        let mut doy: i32 = Default::default();
        raw::gr2jul(self, year, month, day, &mut doy)?;
        Ok(doy)
    }

    /// Seconds per julian year
    ///
    /// Return the number of seconds in a julian year.
    ///
    /// See [`jyear`](raw::jyear) for full documentation.
    pub fn jyear(&self) -> f64 {
        raw::jyear()
    }

    /// Furnish a program with SPICE kernels
    ///
    /// Load one or more SPICE kernels into a program.
    ///
    /// See [`furnsh`](raw::furnsh) for full documentation.
    pub fn furnsh(&mut self, file: &str) -> Result<()> {
        raw::furnsh(self, file)?;
        Ok(())
    }

    /// Kernel Totals
    ///
    /// Return the number of kernels of a specified type that are
    /// currently loaded via the FURNSH interface.
    ///
    /// Returns `count`.
    ///
    /// See [`ktotal`](raw::ktotal) for full documentation.
    pub fn ktotal(&mut self, kind: &str) -> Result<i32> {
        let mut count: i32 = Default::default();
        raw::ktotal(self, kind, &mut count)?;
        Ok(count)
    }

    /// Kernel Data
    ///
    /// Return data for the nth kernel that is among a list of specified
    /// kernel types.
    ///
    /// Returns `(file, filtyp, srcfil, handle)`.
    ///
    /// See [`kdata`](raw::kdata) for full documentation.
    pub fn kdata(&mut self, which: i32, kind: &str) -> Option<(String, String, String, i32)> {
        let mut file = blank(255);
        let mut filtyp = blank(8);
        let mut srcfil = blank(255);
        let mut handle: i32 = Default::default();
        let mut found: bool = Default::default();
        raw::kdata(
            self,
            which,
            kind,
            &mut file,
            &mut filtyp,
            &mut srcfil,
            &mut handle,
            &mut found,
        );
        if found {
            Some((trim(file), trim(filtyp), trim(srcfil), handle))
        } else {
            None
        }
    }

    /// Kernel Information
    ///
    /// Return information about a specific kernel
    ///
    /// Returns `(filtyp, srcfil, handle)`.
    ///
    /// See [`kinfo`](raw::kinfo) for full documentation.
    pub fn kinfo(&mut self, file: &str) -> Option<(String, String, i32)> {
        let mut filtyp = blank(8);
        let mut srcfil = blank(255);
        let mut handle: i32 = Default::default();
        let mut found: bool = Default::default();
        raw::kinfo(
            self,
            file,
            &mut filtyp,
            &mut srcfil,
            &mut handle,
            &mut found,
        );
        if found {
            Some((trim(filtyp), trim(srcfil), handle))
        } else {
            None
        }
    }

    /// Keeper clear
    ///
    /// Clear the KEEPER subsystem: unload all kernels, clear the kernel
    /// pool, and re-initialize the subsystem. Existing watches on kernel
    /// variables are retained.
    ///
    /// See [`kclear`](raw::kclear) for full documentation.
    pub fn kclear(&mut self) -> Result<()> {
        raw::kclear(self)?;
        Ok(())
    }

    /// Unload a kernel
    ///
    /// Unload a SPICE kernel.
    ///
    /// See [`unload`](raw::unload) for full documentation.
    pub fn unload(&mut self, file: &str) -> Result<()> {
        raw::unload(self, file)?;
        Ok(())
    }

    /// Solve Kepler's Equation --- Equinoctial Form
    ///
    /// Solve the equinoctial version of Kepler's equation.
    ///
    /// See [`kepleq`](raw::kepleq) for full documentation.
    pub fn kepleq(&mut self, ml: f64, h: f64, k: f64) -> Result<f64> {
        raw::kepleq(self, ml, h, k)
    }

    /// Kernel pool frame IDs
    ///
    /// Return a SPICE set containing the frame IDs of all reference
    /// frames of a given class having specifications in the kernel pool.
    ///
    /// See [`kplfrm`](raw::kplfrm) for full documentation.
    pub fn kplfrm(&mut self, frmcls: i32, idset: &mut Cell<i32>) -> Result<()> {
        raw::kplfrm(self, frmcls, idset.as_raw_mut_slice())?;
        Ok(())
    }

    /// Solve Kepler's Equation --- Vector Form
    ///
    /// Solve the equation X = \< EVEC, U(X) > where U(X) is the unit
    /// vector \[ COS(X), SIN(X) ] and  \< , > denotes the two-dimensional
    /// dot product.
    ///
    /// See [`kpsolv`](raw::kpsolv) for full documentation.
    pub fn kpsolv(&mut self, evec: &[f64; 2]) -> Result<f64> {
        raw::kpsolv(self, evec)
    }

    /// Extract a substring starting with a keyword
    ///
    /// Locate a keyword in a string and extract the substring from
    /// the beginning of the first word following the keyword to the
    /// beginning of the first subsequent recognized terminator of a list.
    ///
    /// Returns `substr`.
    ///
    /// See [`kxtrct`](raw::kxtrct) for full documentation.
    pub fn kxtrct(
        &self,
        keywd: &str,
        terms: &CharVec,
        nterms: i32,
        wordsq: &mut str,
    ) -> Option<String> {
        let mut found: bool = Default::default();
        let mut substr = blank((terms.element_length() as i32));
        raw::kxtrct(
            keywd,
            terms.as_arg(),
            nterms,
            wordsq,
            &mut found,
            &mut substr,
        );
        if found { Some(trim(substr)) } else { None }
    }

    /// Last non-blank character
    ///
    /// Return the index of the last non-blank character in
    /// a character string.
    ///
    /// See [`lastnb`](raw::lastnb) for full documentation.
    pub fn lastnb(&self, string: &str) -> i32 {
        raw::lastnb(string)
    }

    /// Last printable character
    ///
    /// Return the index of the last printable character in a character
    /// string. ASCII characters 33-126 are printable. (Blanks are not
    /// considered printable.)
    ///
    /// See [`lastpc`](raw::lastpc) for full documentation.
    pub fn lastpc(&self, string: &str) -> i32 {
        raw::lastpc(string)
    }

    /// Latitudinal to cylindrical coordinates
    ///
    /// Convert from latitudinal coordinates to cylindrical coordinates.
    ///
    /// Returns `(r, clon, z)`.
    ///
    /// See [`latcyl`](raw::latcyl) for full documentation.
    pub fn latcyl(&self, radius: f64, lon: f64, lat: f64) -> (f64, f64, f64) {
        let mut r: f64 = Default::default();
        let mut clon: f64 = Default::default();
        let mut z: f64 = Default::default();
        raw::latcyl(radius, lon, lat, &mut r, &mut clon, &mut z);
        (r, clon, z)
    }

    /// Latitudinal to rectangular coordinates
    ///
    /// Convert from latitudinal coordinates to rectangular coordinates.
    ///
    /// Returns `rectan`.
    ///
    /// See [`latrec`](raw::latrec) for full documentation.
    pub fn latrec(&self, radius: f64, lon: f64, lat: f64) -> [f64; 3] {
        let mut rectan: [f64; 3] = Default::default();
        raw::latrec(radius, lon, lat, &mut rectan);
        rectan
    }

    /// Latitudinal to spherical coordinates
    ///
    /// Convert from latitudinal coordinates to spherical coordinates.
    ///
    /// Returns `(rho, colat, slon)`.
    ///
    /// See [`latsph`](raw::latsph) for full documentation.
    pub fn latsph(&mut self, radius: f64, lon: f64, lat: f64) -> (f64, f64, f64) {
        let mut rho: f64 = Default::default();
        let mut colat: f64 = Default::default();
        let mut slon: f64 = Default::default();
        raw::latsph(self, radius, lon, lat, &mut rho, &mut colat, &mut slon);
        (rho, colat, slon)
    }

    /// Latitudinal grid to surface points
    ///
    /// Map array of planetocentric longitude/latitude coordinate pairs
    /// to surface points on a specified target body.
    ///
    /// The surface of the target body may be represented by a triaxial
    /// ellipsoid or by topographic data provided by DSK files.
    ///
    /// Returns `srfpts`.
    ///
    /// See [`latsrf`](raw::latsrf) for full documentation.
    pub fn latsrf(
        &mut self,
        method: &str,
        target: &str,
        et: f64,
        fixref: &str,
        npts: i32,
        lonlat: &[[f64; 2]],
    ) -> Result<Vec<[f64; 3]>> {
        let mut srfpts: Vec<[f64; 3]> = vec![Default::default(); npts.max(0) as usize];
        raw::latsrf(self, method, target, et, fixref, npts, lonlat, &mut srfpts)?;
        Ok(srfpts)
    }

    /// Lagrange interpolation on equally spaced points
    ///
    /// Evaluate a Lagrange interpolating polynomial for a specified
    /// set of coordinate pairs whose first components are equally
    /// spaced, at a specified abscissa value.
    ///
    /// See [`lgresp`](raw::lgresp) for full documentation.
    pub fn lgresp(
        &mut self,
        n: i32,
        first: f64,
        step: f64,
        yvals: &[f64],
        work: &mut [f64],
        x: f64,
    ) -> Result<f64> {
        raw::lgresp(self, n, first, step, yvals, work, x)
    }

    /// Lagrange polynomial interpolation with derivative
    ///
    /// Evaluate a Lagrange interpolating polynomial, for a specified
    /// set of coordinate pairs, at a specified abscissa value. Return
    /// both the value of the polynomial and its derivative.
    ///
    /// Returns `(p, dp)`.
    ///
    /// See [`lgrind`](raw::lgrind) for full documentation.
    pub fn lgrind(
        &mut self,
        n: i32,
        xvals: &[f64],
        yvals: &[f64],
        work: &mut [f64],
        x: f64,
    ) -> Result<(f64, f64)> {
        let mut p: f64 = Default::default();
        let mut dp: f64 = Default::default();
        raw::lgrind(self, n, xvals, yvals, work, x, &mut p, &mut dp)?;
        Ok((p, dp))
    }

    /// Lagrange polynomial interpolation
    ///
    /// Evaluate a Lagrange interpolating polynomial for a specified
    /// set of coordinate pairs, at a specified abscissa value.
    ///
    /// See [`lgrint`](raw::lgrint) for full documentation.
    pub fn lgrint(
        &mut self,
        n: i32,
        xvals: &[f64],
        yvals: &[f64],
        work: &mut [f64],
        x: f64,
    ) -> Result<f64> {
        raw::lgrint(self, n, xvals, yvals, work, x)
    }

    /// Limb points on an extended object
    ///
    /// Find limb points on a target body. The limb is the set of points
    /// of tangency on the target of rays emanating from the observer.
    /// The caller specifies half-planes bounded by the observer-target
    /// center vector in which to search for limb points.
    ///
    /// The surface of the target body may be represented either by a
    /// triaxial ellipsoid or by topographic data.
    ///
    /// Returns `(npts, points, epochs, tangts)`.
    ///
    /// See [`limbpt`](raw::limbpt) for full documentation.
    pub fn limbpt(
        &mut self,
        method: &str,
        target: &str,
        et: f64,
        fixref: &str,
        abcorr: &str,
        corloc: &str,
        obsrvr: &str,
        refvec: &[f64; 3],
        rolstp: f64,
        ncuts: i32,
        schstp: f64,
        soltol: f64,
        maxn: i32,
    ) -> Result<(Vec<i32>, Vec<[f64; 3]>, Vec<f64>, Vec<[f64; 3]>)> {
        let mut npts: Vec<i32> = vec![Default::default(); ncuts.max(0) as usize];
        let mut points: Vec<[f64; 3]> = vec![Default::default(); maxn.max(0) as usize];
        let mut epochs: Vec<f64> = vec![Default::default(); maxn.max(0) as usize];
        let mut tangts: Vec<[f64; 3]> = vec![Default::default(); maxn.max(0) as usize];
        raw::limbpt(
            self,
            method,
            target,
            et,
            fixref,
            abcorr,
            corloc,
            obsrvr,
            refvec,
            rolstp,
            ncuts,
            schstp,
            soltol,
            maxn,
            &mut npts,
            &mut points,
            &mut epochs,
            &mut tangts,
        )?;
        Ok((npts, points, epochs, tangts))
    }

    /// Left-justify, Uppercase, Compress
    ///
    /// Left-justify, uppercase, and space-compress a character string.
    ///
    /// Returns `output`.
    ///
    /// See [`ljucrs`](raw::ljucrs) for full documentation.
    pub fn ljucrs(&mut self, n: i32, input: &str) -> String {
        let mut output = blank((input.len() as i32));
        raw::ljucrs(self, n, input, &mut output);
        trim(output)
    }

    /// LNK, allocate node
    ///
    /// Allocate a node in a doubly linked list pool.
    ///
    /// Returns `new`.
    ///
    /// See [`lnkan`](raw::lnkan) for full documentation.
    pub fn lnkan(&mut self, pool: &mut [[i32; 2]]) -> Result<i32> {
        let mut new: i32 = Default::default();
        raw::lnkan(self, pool, &mut new)?;
        Ok(new)
    }

    /// LNK, free sublist of a list
    ///
    /// Free a specified sublist in a list.
    ///
    /// See [`lnkfsl`](raw::lnkfsl) for full documentation.
    pub fn lnkfsl(&mut self, head: i32, tail: i32, pool: &mut [[i32; 2]]) -> Result<()> {
        raw::lnkfsl(self, head, tail, pool)?;
        Ok(())
    }

    /// LNK, head of list
    ///
    /// Return the head node of the list containing a specified node.
    ///
    /// See [`lnkhl`](raw::lnkhl) for full documentation.
    pub fn lnkhl(&mut self, node: i32, pool: &[[i32; 2]]) -> Result<i32> {
        raw::lnkhl(self, node, pool)
    }

    /// LNK, insert list after node
    ///
    /// Insert the list containing a specified node into a another list,
    /// following a specified node.
    ///
    /// See [`lnkila`](raw::lnkila) for full documentation.
    pub fn lnkila(&mut self, prev: i32, list: i32, pool: &mut [[i32; 2]]) -> Result<()> {
        raw::lnkila(self, prev, list, pool)?;
        Ok(())
    }

    /// LNK, insert list before node
    ///
    /// Insert the list containing a specified node into a another list,
    /// preceding a specified node.
    ///
    /// See [`lnkilb`](raw::lnkilb) for full documentation.
    pub fn lnkilb(&mut self, list: i32, next: i32, pool: &mut [[i32; 2]]) -> Result<()> {
        raw::lnkilb(self, list, next, pool)?;
        Ok(())
    }

    /// LNK, initialize
    ///
    /// Initialize a doubly linked list pool.
    ///
    /// See [`lnkini`](raw::lnkini) for full documentation.
    pub fn lnkini(&mut self, size: i32, pool: &mut [[i32; 2]]) -> Result<()> {
        raw::lnkini(self, size, pool)?;
        Ok(())
    }

    /// LNK, number of free nodes
    ///
    /// Return the number of free nodes in a doubly linked list pool.
    ///
    /// See [`lnknfn`](raw::lnknfn) for full documentation.
    pub fn lnknfn(&self, pool: &[[i32; 2]]) -> i32 {
        raw::lnknfn(pool)
    }

    /// LNK, next node
    ///
    /// Find the node following a specified node in a doubly linked list
    /// pool.
    ///
    /// See [`lnknxt`](raw::lnknxt) for full documentation.
    pub fn lnknxt(&mut self, node: i32, pool: &[[i32; 2]]) -> Result<i32> {
        raw::lnknxt(self, node, pool)
    }

    /// LNK, previous node
    ///
    /// Find the node preceding a specified node in a doubly linked list
    /// pool.
    ///
    /// See [`lnkprv`](raw::lnkprv) for full documentation.
    pub fn lnkprv(&mut self, node: i32, pool: &[[i32; 2]]) -> Result<i32> {
        raw::lnkprv(self, node, pool)
    }

    /// LNK, size
    ///
    /// Return the size of a doubly linked list pool.
    ///
    /// See [`lnksiz`](raw::lnksiz) for full documentation.
    pub fn lnksiz(&self, pool: &[[i32; 2]]) -> i32 {
        raw::lnksiz(pool)
    }

    /// LNK, tail of list
    ///
    /// Return the tail node of the list containing a specified node.
    ///
    /// See [`lnktl`](raw::lnktl) for full documentation.
    pub fn lnktl(&mut self, node: i32, pool: &[[i32; 2]]) -> Result<i32> {
        raw::lnktl(self, node, pool)
    }

    /// LNK, extract sublist from list
    ///
    /// Extract a specified sublist from a list.
    ///
    /// See [`lnkxsl`](raw::lnkxsl) for full documentation.
    pub fn lnkxsl(&mut self, head: i32, tail: i32, pool: &mut [[i32; 2]]) -> Result<()> {
        raw::lnkxsl(self, head, tail, pool)?;
        Ok(())
    }

    /// Locate an identifier in a list
    ///
    /// Find a given identifier, which consists of an integer array,
    /// within a list of such identifiers, or insert the identifier
    /// into the list. Return the location of the identifier and a flag
    /// indicating whether or not the identifier was already present.
    ///
    /// Returns `presnt`.
    ///
    /// See [`locati`](raw::locati) for full documentation.
    pub fn locati(
        &mut self,
        id: &[i32],
        idsz: i32,
        list: &mut [i32],
        pool: &mut [[i32; 2]],
        at: &mut i32,
    ) -> Result<bool> {
        let mut presnt: bool = Default::default();
        raw::locati(self, id, idsz, list, pool, at, &mut presnt)?;
        Ok(presnt)
    }

    /// Locate lines in a text file
    ///
    /// Locate a group of lines in a text file delimited by markers.
    ///
    /// Returns `(bline, eline)`.
    ///
    /// See [`locln`](raw::locln) for full documentation.
    pub fn locln(
        &mut self,
        unit: i32,
        bmark: &str,
        emark: &str,
        line: &mut str,
    ) -> Result<Option<(i32, i32)>> {
        let mut bline: i32 = Default::default();
        let mut eline: i32 = Default::default();
        let mut found: bool = Default::default();
        raw::locln(
            self, unit, bmark, emark, line, &mut bline, &mut eline, &mut found,
        )?;
        Ok(if found { Some((bline, eline)) } else { None })
    }

    /// Parse items from a list
    ///
    /// Parse a list of items delimited by a single character.
    ///
    /// Returns `n`.
    ///
    /// See [`lparse`](raw::lparse) for full documentation.
    pub fn lparse(&self, list: &str, delim: char, nmax: i32, items: &mut CharVec) -> i32 {
        let mut n: i32 = Default::default();
        raw::lparse(list, delim, nmax, &mut n, items.as_arg_mut());
        n
    }

    /// Parse a list of items
    ///
    /// Parse a list of items separated by multiple delimiters.
    ///
    /// Returns `n`.
    ///
    /// See [`lparsm`](raw::lparsm) for full documentation.
    pub fn lparsm(&self, list: &str, delims: &str, nmax: i32, items: &mut CharVec) -> i32 {
        let mut n: i32 = Default::default();
        raw::lparsm(list, delims, nmax, &mut n, items.as_arg_mut());
        n
    }

    /// Parse a list of items; return a set.
    ///
    /// Parse a list of items delimited by multiple delimiters,
    /// placing the resulting items into a set.
    ///
    /// See [`lparss`](raw::lparss) for full documentation.
    pub fn lparss(&mut self, list: &str, delims: &str, set: &mut CharCell) -> Result<()> {
        raw::lparss(self, list, delims, set.as_arg_mut())?;
        Ok(())
    }

    /// Longitude of the sun, planetocentric
    ///
    /// Compute L_s, the planetocentric longitude of the sun, as seen
    /// from a specified body.
    ///
    /// See [`lspcn`](raw::lspcn) for full documentation.
    pub fn lspcn(&mut self, body: &str, et: f64, abcorr: &str) -> Result<f64> {
        raw::lspcn(self, body, et, abcorr)
    }

    /// Last closest double precision array element
    ///
    /// Find the index of the array element closest to a given number X
    /// in an array of non-decreasing numbers.
    ///
    /// See [`lstcld`](raw::lstcld) for full documentation.
    pub fn lstcld(&self, x: f64, n: i32, array: &[f64]) -> i32 {
        raw::lstcld(x, n, array)
    }

    /// Closest integer array element
    ///
    /// Find the index of the array element closest to a given integer X
    /// in an array of non-decreasing integers.
    ///
    /// See [`lstcli`](raw::lstcli) for full documentation.
    pub fn lstcli(&self, x: i32, n: i32, array: &[i32]) -> i32 {
        raw::lstcli(x, n, array)
    }

    /// Last character element less than or equal to.
    ///
    /// Find the index of the largest array element less than or equal to
    /// a given character string in an ordered array of character strings.
    ///
    /// See [`lstlec`](raw::lstlec) for full documentation.
    pub fn lstlec(&self, string: &str, n: i32, array: &CharVec) -> i32 {
        raw::lstlec(string, n, array.as_arg())
    }

    /// Last double precision element less than or equal
    ///
    /// Find the index of the largest array element less than or equal
    /// to a given number X in an array of non-decreasing numbers.
    ///
    /// See [`lstled`](raw::lstled) for full documentation.
    pub fn lstled(&self, x: f64, n: i32, array: &[f64]) -> i32 {
        raw::lstled(x, n, array)
    }

    /// Last integer element less than or equal to
    ///
    /// Find the index of the largest array element less than or equal
    /// to a given integer X in an array of non-decreasing integers.
    ///
    /// See [`lstlei`](raw::lstlei) for full documentation.
    pub fn lstlei(&self, x: i32, n: i32, array: &[i32]) -> i32 {
        raw::lstlei(x, n, array)
    }

    /// Last character element less than
    ///
    /// Find the index of the largest array element less than a given
    /// character string in an ordered array of character strings.
    ///
    /// See [`lstltc`](raw::lstltc) for full documentation.
    pub fn lstltc(&self, string: &str, n: i32, array: &CharVec) -> i32 {
        raw::lstltc(string, n, array.as_arg())
    }

    /// Last double precision element less than
    ///
    /// Find the index of the largest array element less than
    /// a given number X in an array of non-decreasing numbers.
    ///
    /// See [`lstltd`](raw::lstltd) for full documentation.
    pub fn lstltd(&self, x: f64, n: i32, array: &[f64]) -> i32 {
        raw::lstltd(x, n, array)
    }

    /// Last integer element less than
    ///
    /// Find the index of the largest array element less than
    /// a given integer X in an array of non-decreasing integers.
    ///
    /// See [`lstlti`](raw::lstlti) for full documentation.
    pub fn lstlti(&self, x: i32, n: i32, array: &[i32]) -> i32 {
        raw::lstlti(x, n, array)
    }

    /// Light Time
    ///
    /// Compute the transmission (or reception) time of a signal at a
    /// specified target, given the reception (or transmission) time at a
    /// specified observer. Also return the elapsed time between
    /// transmission and reception.
    ///
    /// Returns `(ettarg, elapsd)`.
    ///
    /// See [`ltime`](raw::ltime) for full documentation.
    pub fn ltime(&mut self, etobs: f64, obs: i32, dir: &str, targ: i32) -> Result<(f64, f64)> {
        let mut ettarg: f64 = Default::default();
        let mut elapsd: f64 = Default::default();
        raw::ltime(self, etobs, obs, dir, targ, &mut ettarg, &mut elapsd)?;
        Ok((ettarg, elapsd))
    }

    /// Left trim
    ///
    /// Return the maximum of 1 and the location of the first non-blank
    /// character in the string.
    ///
    /// See [`ltrim`](raw::ltrim) for full documentation.
    pub fn ltrim(&self, string: &str) -> i32 {
        raw::ltrim(string)
    }

    /// Map logical unit of open file to its name.
    ///
    /// Map the logical unit of an open file to its associated filename.
    ///
    /// Returns `filnam`.
    ///
    /// See [`lun2fn`](raw::lun2fn) for full documentation.
    pub fn lun2fn(&mut self, lunit: i32) -> Result<String> {
        let mut filnam = blank(255);
        raw::lun2fn(self, lunit, &mut filnam)?;
        Ok(trim(filnam))
    }

    /// Scan for signed integer
    ///
    /// Scan a string from a specified starting position for the
    /// end of a decimal number.
    ///
    /// Returns `(last, nchar)`.
    ///
    /// See [`lx4dec`](raw::lx4dec) for full documentation.
    pub fn lx4dec(&mut self, string: &str, first: i32) -> (i32, i32) {
        let mut last: i32 = Default::default();
        let mut nchar: i32 = Default::default();
        raw::lx4dec(self, string, first, &mut last, &mut nchar);
        (last, nchar)
    }

    /// Scan for a number
    ///
    /// Scan a string from a specified starting position for the
    /// end of a number.
    ///
    /// Returns `(last, nchar)`.
    ///
    /// See [`lx4num`](raw::lx4num) for full documentation.
    pub fn lx4num(&mut self, string: &str, first: i32) -> (i32, i32) {
        let mut last: i32 = Default::default();
        let mut nchar: i32 = Default::default();
        raw::lx4num(self, string, first, &mut last, &mut nchar);
        (last, nchar)
    }

    /// Scan for signed integer
    ///
    /// Scan a string from a specified starting position for the
    /// end of a signed integer.
    ///
    /// Returns `(last, nchar)`.
    ///
    /// See [`lx4sgn`](raw::lx4sgn) for full documentation.
    pub fn lx4sgn(&mut self, string: &str, first: i32) -> (i32, i32) {
        let mut last: i32 = Default::default();
        let mut nchar: i32 = Default::default();
        raw::lx4sgn(self, string, first, &mut last, &mut nchar);
        (last, nchar)
    }

    /// Scan for unsigned integer
    ///
    /// Scan a string from a specified starting position for the
    /// end of an unsigned integer.
    ///
    /// Returns `(last, nchar)`.
    ///
    /// See [`lx4uns`](raw::lx4uns) for full documentation.
    pub fn lx4uns(&mut self, string: &str, first: i32) -> (i32, i32) {
        let mut last: i32 = Default::default();
        let mut nchar: i32 = Default::default();
        raw::lx4uns(self, string, first, &mut last, &mut nchar);
        (last, nchar)
    }

    /// Lex identifier
    ///
    /// Scan an identifier, starting from a specified character
    /// position.
    ///
    /// Returns `(last, nchar)`.
    ///
    /// See [`lxidnt`](raw::lxidnt) for full documentation.
    pub fn lxidnt(&self, idspec: &Cell<i32>, string: &str, first: i32) -> (i32, i32) {
        let mut last: i32 = Default::default();
        let mut nchar: i32 = Default::default();
        raw::lxidnt(idspec.as_raw_slice(), string, first, &mut last, &mut nchar);
        (last, nchar)
    }

    /// Lex, default identifier characters
    ///
    /// Return the default specification for the characters that may
    /// appear in an identifier.
    ///
    /// See [`lxdfid`](raw::lxdfid) for full documentation.
    pub fn lxdfid(&mut self, idspec: &mut Cell<i32>) -> Result<()> {
        raw::lxdfid(self, idspec.as_raw_mut_slice())?;
        Ok(())
    }

    /// Lex, custom identifier characters
    ///
    /// Set the acceptable characters that may appear in an identifier
    /// token.
    ///
    /// See [`lxcsid`](raw::lxcsid) for full documentation.
    pub fn lxcsid(&mut self, hdchrs: &str, tlchrs: &str, idspec: &mut Cell<i32>) -> Result<()> {
        raw::lxcsid(self, hdchrs, tlchrs, idspec.as_raw_mut_slice())?;
        Ok(())
    }

    /// Lex quoted string
    ///
    /// Scan (lex) a quoted string.
    ///
    /// Returns `(last, nchar)`.
    ///
    /// See [`lxqstr`](raw::lxqstr) for full documentation.
    pub fn lxqstr(&self, string: &str, qchar: char, first: i32) -> (i32, i32) {
        let mut last: i32 = Default::default();
        let mut nchar: i32 = Default::default();
        raw::lxqstr(string, qchar, first, &mut last, &mut nchar);
        (last, nchar)
    }

    /// Matrix to Euler angles
    ///
    /// Factor a rotation matrix as a product of three rotations about
    /// specified coordinate axes.
    ///
    /// Returns `(angle3, angle2, angle1)`.
    ///
    /// See [`m2eul`](raw::m2eul) for full documentation.
    pub fn m2eul(
        &mut self,
        r: &[[f64; 3]; 3],
        axis3: i32,
        axis2: i32,
        axis1: i32,
    ) -> Result<(f64, f64, f64)> {
        let mut angle3: f64 = Default::default();
        let mut angle2: f64 = Default::default();
        let mut angle1: f64 = Default::default();
        raw::m2eul(
            self,
            r,
            axis3,
            axis2,
            axis1,
            &mut angle3,
            &mut angle2,
            &mut angle1,
        )?;
        Ok((angle3, angle2, angle1))
    }

    /// Matrix to quaternion
    ///
    /// Find a unit quaternion corresponding to a specified rotation
    /// matrix.
    ///
    /// Returns `q`.
    ///
    /// See [`m2q`](raw::m2q) for full documentation.
    pub fn m2q(&mut self, r: &[[f64; 3]; 3]) -> Result<[f64; 4]> {
        let mut q: [f64; 4] = Default::default();
        raw::m2q(self, r, &mut q)?;
        Ok(q)
    }

    /// Match string against wildcard template
    ///
    /// Determine whether a string is matched by a template containing
    /// wild cards. This routine is case-insensitive.
    ///
    /// See [`matchi`](raw::matchi) for full documentation.
    pub fn matchi(&mut self, string: &str, templ: &str, wstr: char, wchr: char) -> bool {
        raw::matchi(self, string, templ, wstr, wchr)
    }

    /// Match string against wildcard template
    ///
    /// Determine whether a string is matched by a template containing
    /// wild cards.
    ///
    /// See [`matchw`](raw::matchw) for full documentation.
    pub fn matchw(&self, string: &str, templ: &str, wstr: char, wchr: char) -> bool {
        raw::matchw(string, templ, wstr, wchr)
    }

    /// Maximum element of array, DP
    ///
    /// Locate the maximum element of a DP array.
    ///
    /// Returns `(maxval, loc)`.
    ///
    /// See [`maxad`](raw::maxad) for full documentation.
    pub fn maxad(&self, array: &[f64], ndim: i32) -> (f64, i32) {
        let mut maxval: f64 = Default::default();
        let mut loc: i32 = Default::default();
        raw::maxad(array, ndim, &mut maxval, &mut loc);
        (maxval, loc)
    }

    /// Maximum element of array, integer
    ///
    /// Locate the maximum element of an integer array.
    ///
    /// Returns `(maxval, loc)`.
    ///
    /// See [`maxai`](raw::maxai) for full documentation.
    pub fn maxai(&self, array: &[i32], ndim: i32) -> (i32, i32) {
        let mut maxval: i32 = Default::default();
        let mut loc: i32 = Default::default();
        raw::maxai(array, ndim, &mut maxval, &mut loc);
        (maxval, loc)
    }

    /// Matrix equal to another, 3x3
    ///
    /// Set one double precision 3x3 matrix equal to another.
    ///
    /// Returns `mout`.
    ///
    /// See [`mequ`](raw::mequ) for full documentation.
    pub fn mequ(&self, m1: &[[f64; 3]; 3]) -> [[f64; 3]; 3] {
        let mut mout: [[f64; 3]; 3] = Default::default();
        raw::mequ(m1, &mut mout);
        mout
    }

    /// Minimum element of array, DP
    ///
    /// Locate the minimum element of a DP array.
    ///
    /// Returns `(minval, loc)`.
    ///
    /// See [`minad`](raw::minad) for full documentation.
    pub fn minad(&self, array: &[f64], ndim: i32) -> (f64, i32) {
        let mut minval: f64 = Default::default();
        let mut loc: i32 = Default::default();
        raw::minad(array, ndim, &mut minval, &mut loc);
        (minval, loc)
    }

    /// Minimum element of array, integer
    ///
    /// Locate the minimum element of an integer array.
    ///
    /// Returns `(minval, loc)`.
    ///
    /// See [`minai`](raw::minai) for full documentation.
    pub fn minai(&self, array: &[i32], ndim: i32) -> (i32, i32) {
        let mut minval: i32 = Default::default();
        let mut loc: i32 = Default::default();
        raw::minai(array, ndim, &mut minval, &mut loc);
        (minval, loc)
    }

    /// Matrix transpose times matrix, 3x3
    ///
    /// Multiply the transpose of a 3x3 matrix and a 3x3 matrix.
    ///
    /// Returns `mout`.
    ///
    /// See [`mtxm`](raw::mtxm) for full documentation.
    pub fn mtxm(&self, m1: &[[f64; 3]; 3], m2: &[[f64; 3]; 3]) -> [[f64; 3]; 3] {
        let mut mout: [[f64; 3]; 3] = Default::default();
        raw::mtxm(m1, m2, &mut mout);
        mout
    }

    /// Matrix transpose times matrix, general dimension
    ///
    /// Multiply the transpose of a matrix with another matrix,
    /// both of arbitrary size. (The dimensions of the matrices must be
    /// compatible with this multiplication.)
    ///
    /// Returns `mout`.
    ///
    /// See [`mtxmg`](raw::mtxmg) for full documentation.
    pub fn mtxmg(&self, m1: &[f64], m2: &[f64], nc1: i32, nr1r2: i32, nc2: i32) -> Vec<f64> {
        let mut mout: Vec<f64> = vec![Default::default(); (nc1 * nc2).max(0) as usize];
        raw::mtxmg(m1, m2, nc1, nr1r2, nc2, &mut mout);
        mout
    }

    /// Matrix transpose times vector, 3x3
    ///
    /// Multiply the transpose of a 3x3 matrix on the left with a vector
    /// on the right.
    ///
    /// Returns `vout`.
    ///
    /// See [`mtxv`](raw::mtxv) for full documentation.
    pub fn mtxv(&self, m: &[[f64; 3]; 3], vin: &[f64; 3]) -> [f64; 3] {
        let mut vout: [f64; 3] = Default::default();
        raw::mtxv(m, vin, &mut vout);
        vout
    }

    /// Matrix transpose times vector, general dimension
    ///
    /// Multiply the transpose of a matrix and a vector of
    /// arbitrary size.
    ///
    /// Returns `vout`.
    ///
    /// See [`mtxvg`](raw::mtxvg) for full documentation.
    pub fn mtxvg(&self, m1: &[f64], v2: &[f64], nc1: i32, nr1r2: i32) -> Vec<f64> {
        let mut vout: Vec<f64> = vec![Default::default(); nc1.max(0) as usize];
        raw::mtxvg(m1, v2, nc1, nr1r2, &mut vout);
        vout
    }

    /// Matrix times matrix, 3x3
    ///
    /// Multiply two 3x3 matrices.
    ///
    /// Returns `mout`.
    ///
    /// See [`mxm`](raw::mxm) for full documentation.
    pub fn mxm(&self, m1: &[[f64; 3]; 3], m2: &[[f64; 3]; 3]) -> [[f64; 3]; 3] {
        let mut mout: [[f64; 3]; 3] = Default::default();
        raw::mxm(m1, m2, &mut mout);
        mout
    }

    /// Matrix times matrix, general dimension
    ///
    /// Multiply two double precision matrices of arbitrary size.
    ///
    /// Returns `mout`.
    ///
    /// See [`mxmg`](raw::mxmg) for full documentation.
    pub fn mxmg(&self, m1: &[f64], m2: &[f64], nr1: i32, nc1r2: i32, nc2: i32) -> Vec<f64> {
        let mut mout: Vec<f64> = vec![Default::default(); (nr1 * nc2).max(0) as usize];
        raw::mxmg(m1, m2, nr1, nc1r2, nc2, &mut mout);
        mout
    }

    /// Matrix times matrix transpose, 3x3
    ///
    /// Multiply a 3x3 matrix and the transpose of another 3x3 matrix.
    ///
    /// Returns `mout`.
    ///
    /// See [`mxmt`](raw::mxmt) for full documentation.
    pub fn mxmt(&self, m1: &[[f64; 3]; 3], m2: &[[f64; 3]; 3]) -> [[f64; 3]; 3] {
        let mut mout: [[f64; 3]; 3] = Default::default();
        raw::mxmt(m1, m2, &mut mout);
        mout
    }

    /// Matrix times matrix transpose, general dimension
    ///
    /// Multiply a matrix and the transpose of a matrix, both of
    /// arbitrary size.
    ///
    /// Returns `mout`.
    ///
    /// See [`mxmtg`](raw::mxmtg) for full documentation.
    pub fn mxmtg(&self, m1: &[f64], m2: &[f64], nr1: i32, nc1c2: i32, nr2: i32) -> Vec<f64> {
        let mut mout: Vec<f64> = vec![Default::default(); (nr1 * nr2).max(0) as usize];
        raw::mxmtg(m1, m2, nr1, nc1c2, nr2, &mut mout);
        mout
    }

    /// Matrix times vector, 3x3
    ///
    /// Multiply a 3x3 double precision matrix with a 3-dimensional
    /// double precision vector.
    ///
    /// Returns `vout`.
    ///
    /// See [`mxv`](raw::mxv) for full documentation.
    pub fn mxv(&self, m: &[[f64; 3]; 3], vin: &[f64; 3]) -> [f64; 3] {
        let mut vout: [f64; 3] = Default::default();
        raw::mxv(m, vin, &mut vout);
        vout
    }

    /// Matrix time vector, general dimension
    ///
    /// Multiply a matrix and a vector of arbitrary size.
    ///
    /// Returns `vout`.
    ///
    /// See [`mxvg`](raw::mxvg) for full documentation.
    pub fn mxvg(&self, m1: &[f64], v2: &[f64], nr1: i32, nc1r2: i32) -> Vec<f64> {
        let mut vout: Vec<f64> = vec![Default::default(); nr1.max(0) as usize];
        raw::mxvg(m1, v2, nr1, nc1r2, &mut vout);
        vout
    }

    /// Non blank length of a string
    ///
    /// Return the non-blank length of a character string. (That is,
    /// the index of the last non-blank character when the string is
    /// left-justified.)
    ///
    /// See [`nblen`](raw::nblen) for full documentation.
    pub fn nblen(&self, string: &str) -> i32 {
        raw::nblen(string)
    }

    /// Non-blank width of a character array
    ///
    /// Determine the non-blank width of a character array---that is,
    /// the largest value of LASTNB for any element in the array.
    ///
    /// See [`nbwid`](raw::nbwid) for full documentation.
    pub fn nbwid(&self, array: &CharVec, nelt: i32) -> i32 {
        raw::nbwid(array.as_arg(), nelt)
    }

    /// NOT character position
    ///
    /// Find the first occurrence in a string of a character NOT belonging
    /// to a collection of characters, starting at a specified location,
    /// searching forwards.
    ///
    /// See [`ncpos`](raw::ncpos) for full documentation.
    pub fn ncpos(&self, str: &str, chars: &str, start: i32) -> i32 {
        raw::ncpos(str, chars, start)
    }

    /// NOT character position, reverse
    ///
    /// Find the first occurrence in a string of a character NOT
    /// belonging to a collection of characters, starting at a
    /// specified location, searching in reverse.
    ///
    /// See [`ncposr`](raw::ncposr) for full documentation.
    pub fn ncposr(&self, str: &str, chars: &str, start: i32) -> i32 {
        raw::ncposr(str, chars, start)
    }

    /// Nearest point on an ellipsoid
    ///
    /// Locate the point on the surface of an ellipsoid that is nearest
    /// to a specified position. Also return the altitude of the position
    /// above the ellipsoid.
    ///
    /// Returns `(npoint, alt)`.
    ///
    /// See [`nearpt`](raw::nearpt) for full documentation.
    pub fn nearpt(&mut self, positn: &[f64; 3], a: f64, b: f64, c: f64) -> Result<([f64; 3], f64)> {
        let mut npoint: [f64; 3] = Default::default();
        let mut alt: f64 = Default::default();
        raw::nearpt(self, positn, a, b, c, &mut npoint, &mut alt)?;
        Ok((npoint, alt))
    }

    /// No true entries?
    ///
    /// Determine if none the entries in an array of logicals are .TRUE.
    ///
    /// See [`notru`](raw::notru) for full documentation.
    pub fn notru(&self, logcls: &[bool], n: i32) -> bool {
        raw::notru(logcls, n)
    }

    /// Double Precision parsing of a string
    ///
    /// Parse a character string that represents a number and return
    /// a double precision value.
    ///
    /// Returns `(x, error, ptr)`.
    ///
    /// See [`nparsd`](raw::nparsd) for full documentation.
    pub fn nparsd(&mut self, string: &str) -> (f64, String, i32) {
        let mut x: f64 = Default::default();
        let mut error = blank(inc::errhnd::LMSGLN);
        let mut ptr: i32 = Default::default();
        raw::nparsd(self, string, &mut x, &mut error, &mut ptr);
        (x, trim(error), ptr)
    }

    /// Integer parsing of a character string
    ///
    /// Parse a character string that represents a number and return
    /// the FORTRAN-truncated integer value.
    ///
    /// Returns `(n, error, pnter)`.
    ///
    /// See [`nparsi`](raw::nparsi) for full documentation.
    pub fn nparsi(&mut self, string: &str) -> (i32, String, i32) {
        let mut n: i32 = Default::default();
        let mut error = blank(inc::errhnd::LMSGLN);
        let mut pnter: i32 = Default::default();
        raw::nparsi(self, string, &mut n, &mut error, &mut pnter);
        (n, trim(error), pnter)
    }

    /// Nearest point on ellipsoid to line
    ///
    /// Find nearest point on a triaxial ellipsoid to a specified line,
    /// and the distance from the ellipsoid to the line.
    ///
    /// Returns `(pnear, dist)`.
    ///
    /// See [`npedln`](raw::npedln) for full documentation.
    pub fn npedln(
        &mut self,
        a: f64,
        b: f64,
        c: f64,
        linept: &[f64; 3],
        linedr: &[f64; 3],
    ) -> Result<([f64; 3], f64)> {
        let mut pnear: [f64; 3] = Default::default();
        let mut dist: f64 = Default::default();
        raw::npedln(self, a, b, c, linept, linedr, &mut pnear, &mut dist)?;
        Ok((pnear, dist))
    }

    /// Nearest point on ellipse to point
    ///
    /// Find the nearest point on an ellipse to a specified point, both
    /// in three-dimensional space, and find the distance between the
    /// ellipse and the point.
    ///
    /// Returns `(pnear, dist)`.
    ///
    /// See [`npelpt`](raw::npelpt) for full documentation.
    pub fn npelpt(&mut self, point: &[f64; 3], ellips: &[f64; 9]) -> Result<([f64; 3], f64)> {
        let mut pnear: [f64; 3] = Default::default();
        let mut dist: f64 = Default::default();
        raw::npelpt(self, point, ellips, &mut pnear, &mut dist)?;
        Ok((pnear, dist))
    }

    /// Nearest point on line to point
    ///
    /// Find the nearest point on a line to a specified point, and find
    /// the distance between the two points.
    ///
    /// Returns `(pnear, dist)`.
    ///
    /// See [`nplnpt`](raw::nplnpt) for full documentation.
    pub fn nplnpt(
        &mut self,
        linpt: &[f64; 3],
        lindir: &[f64; 3],
        point: &[f64; 3],
    ) -> Result<([f64; 3], f64)> {
        let mut pnear: [f64; 3] = Default::default();
        let mut dist: f64 = Default::default();
        raw::nplnpt(self, linpt, lindir, point, &mut pnear, &mut dist)?;
        Ok((pnear, dist))
    }

    /// Nearest point on line segment
    ///
    /// Find the nearest point on a line segment to a given point.
    ///
    /// Returns `(pnear, dist)`.
    ///
    /// See [`npsgpt`](raw::npsgpt) for full documentation.
    pub fn npsgpt(
        &mut self,
        ep1: &[f64; 3],
        ep2: &[f64; 3],
        point: &[f64; 3],
    ) -> Result<([f64; 3], f64)> {
        let mut pnear: [f64; 3] = Default::default();
        let mut dist: f64 = Default::default();
        raw::npsgpt(self, ep1, ep2, point, &mut pnear, &mut dist)?;
        Ok((pnear, dist))
    }

    /// Normal vector and constant to plane
    ///
    /// Make a SPICE plane from a normal vector and a constant.
    ///
    /// Returns `plane`.
    ///
    /// See [`nvc2pl`](raw::nvc2pl) for full documentation.
    pub fn nvc2pl(&mut self, normal: &[f64; 3], konst: f64) -> Result<[f64; 4]> {
        let mut plane: [f64; 4] = Default::default();
        raw::nvc2pl(self, normal, konst, &mut plane)?;
        Ok(plane)
    }

    /// Normal vector and point to plane
    ///
    /// Make a SPICE plane from a normal vector and a point.
    ///
    /// Returns `plane`.
    ///
    /// See [`nvp2pl`](raw::nvp2pl) for full documentation.
    pub fn nvp2pl(&mut self, normal: &[f64; 3], point: &[f64; 3]) -> Result<[f64; 4]> {
        let mut plane: [f64; 4] = Default::default();
        raw::nvp2pl(self, normal, point, &mut plane)?;
        Ok(plane)
    }

    /// find occultation type at time
    ///
    /// Determine the occultation condition (not occulted, partially
    /// occulted, etc.) of one target relative to another target as seen
    /// by an observer at a given time.
    ///
    /// The surfaces of the target bodies may be represented by triaxial
    /// ellipsoids, points, or by topographic data provided by DSK files.
    ///
    /// Returns `ocltid`.
    ///
    /// See [`occult`](raw::occult) for full documentation.
    pub fn occult(
        &mut self,
        targ1: &str,
        shape1: &str,
        frame1: &str,
        targ2: &str,
        shape2: &str,
        frame2: &str,
        abcorr: &str,
        obsrvr: &str,
        et: f64,
    ) -> Result<i32> {
        let mut ocltid: i32 = Default::default();
        raw::occult(
            self,
            targ1,
            shape1,
            frame1,
            targ2,
            shape2,
            frame2,
            abcorr,
            obsrvr,
            et,
            &mut ocltid,
        )?;
        Ok(ocltid)
    }

    /// Is a number odd?
    ///
    /// Determine whether an integer is odd.
    ///
    /// See [`odd`](raw::odd) for full documentation.
    pub fn odd(&self, ival: i32) -> bool {
        raw::odd(ival)
    }

    /// Opposite Sign Double Precision Numbers
    ///
    /// Return .TRUE. if two given double precision numbers have opposite
    /// signs.
    ///
    /// See [`opsgnd`](raw::opsgnd) for full documentation.
    pub fn opsgnd(&self, x: f64, y: f64) -> bool {
        raw::opsgnd(x, y)
    }

    /// Opposite Sign Integers
    ///
    /// Return .TRUE. if two given integer numbers have opposite signs.
    ///
    /// See [`opsgni`](raw::opsgni) for full documentation.
    pub fn opsgni(&self, x: i32, y: i32) -> bool {
        raw::opsgni(x, y)
    }

    /// The ordinal position of an element in a set
    ///
    /// Return the ordinal position of a given item in a set. If the
    /// item does not appear in the set, return zero.
    ///
    /// See [`ordc`](raw::ordc) for full documentation.
    pub fn ordc(&mut self, item: &str, set: &CharCell) -> Result<i32> {
        raw::ordc(self, item, set.as_arg())
    }

    /// The ordinal position of an element in a set
    ///
    /// Return the ordinal position of a given item in a set. If the
    /// item does not appear in the set, return zero.
    ///
    /// See [`ordd`](raw::ordd) for full documentation.
    pub fn ordd(&mut self, item: f64, set: &Cell<f64>) -> Result<i32> {
        raw::ordd(self, item, set.as_raw_slice())
    }

    /// The ordinal position of an element in a set
    ///
    /// Return the ordinal position of a given item in a set. If the
    /// item does not appear in the set, return zero.
    ///
    /// See [`ordi`](raw::ordi) for full documentation.
    pub fn ordi(&mut self, item: i32, set: &Cell<i32>) -> Result<i32> {
        raw::ordi(self, item, set.as_raw_slice())
    }

    /// Determine conic elements from state
    ///
    /// Determine the set of osculating conic orbital elements that
    /// corresponds to the state (position, velocity) of a body at
    /// some epoch.
    ///
    /// Returns `elts`.
    ///
    /// See [`oscelt`](raw::oscelt) for full documentation.
    pub fn oscelt(&mut self, state: &[f64; 6], et: f64, mu: f64) -> Result<[f64; 8]> {
        let mut elts: [f64; 8] = Default::default();
        raw::oscelt(self, state, et, mu, &mut elts)?;
        Ok(elts)
    }

    /// Extended osculating elements from state
    ///
    /// Determine the set of osculating conic orbital elements that
    /// corresponds to the state (position, velocity) of a body at some
    /// epoch. In additional to the classical elements, return the true
    /// anomaly, semi-major axis, and period, if applicable.
    ///
    /// Returns `elts`.
    ///
    /// See [`oscltx`](raw::oscltx) for full documentation.
    pub fn oscltx(&mut self, state: &[f64; 6], et: f64, mu: f64) -> Result<Vec<f64>> {
        let mut elts: Vec<f64> = vec![Default::default(); inc::oscltx::OSCXSZ as usize];
        raw::oscltx(self, state, et, mu, &mut elts)?;
        Ok(elts)
    }

    /// Output Error Messages
    ///
    /// Output error messages.
    ///
    /// See [`outmsg`](raw::outmsg) for full documentation.
    pub fn outmsg(&mut self, list: &str) -> Result<()> {
        raw::outmsg(self, list)?;
        Ok(())
    }

    /// Pack a character array
    ///
    /// Pack the contents of a CHARACTER array. That is, take
    /// a set of arbitrarily spaced elements from an input array,
    /// and make them adjacent elements in an output array.
    ///
    /// Returns `nout`.
    ///
    /// See [`packac`](raw::packac) for full documentation.
    pub fn packac(
        &mut self,
        in_: &CharVec,
        pack: &[i32],
        npack: i32,
        maxout: i32,
        out: &mut CharVec,
    ) -> Result<i32> {
        let mut nout: i32 = Default::default();
        raw::packac(
            self,
            in_.as_arg(),
            pack,
            npack,
            maxout,
            &mut nout,
            out.as_arg_mut(),
        )?;
        Ok(nout)
    }

    /// Pack a double precision array
    ///
    /// Pack the contents of a double precision array. That is,
    /// take a set of arbitrarily spaced elements from an input
    /// array, and make them adjacent elements in an output array.
    ///
    /// Returns `(nout, out)`.
    ///
    /// See [`packad`](raw::packad) for full documentation.
    pub fn packad(
        &mut self,
        in_: &[f64],
        pack: &[i32],
        npack: i32,
        maxout: i32,
    ) -> Result<(i32, Vec<f64>)> {
        let mut nout: i32 = Default::default();
        let mut out: Vec<f64> = vec![Default::default(); maxout.max(0) as usize];
        raw::packad(self, in_, pack, npack, maxout, &mut nout, &mut out)?;
        Ok((nout, out))
    }

    /// Pack an integer array
    ///
    /// Pack the contents of an integer array. That is,
    /// take a set of arbitrarily spaced elements from an input
    /// array, and make them adjacent elements in an output array.
    ///
    /// Returns `(nout, out)`.
    ///
    /// See [`packai`](raw::packai) for full documentation.
    pub fn packai(
        &mut self,
        in_: &[i32],
        pack: &[i32],
        npack: i32,
        maxout: i32,
    ) -> Result<(i32, Vec<i32>)> {
        let mut nout: i32 = Default::default();
        let mut out: Vec<i32> = vec![Default::default(); maxout.max(0) as usize];
        raw::packai(self, in_, pack, npack, maxout, &mut nout, &mut out)?;
        Ok((nout, out))
    }

    /// Parse quoted string token
    ///
    /// Parse a quoted string token.
    ///
    /// Returns `(value, length, error, errmsg, ptr)`.
    ///
    /// See [`parsqs`](raw::parsqs) for full documentation.
    pub fn parsqs(&self, string: &str, qchar: char) -> (String, i32, bool, String, i32) {
        let mut value = blank((string.len() as i32));
        let mut length: i32 = Default::default();
        let mut error: bool = Default::default();
        let mut errmsg = blank(inc::errhnd::LMSGLN);
        let mut ptr: i32 = Default::default();
        raw::parsqs(
            string,
            qchar,
            &mut value,
            &mut length,
            &mut error,
            &mut errmsg,
            &mut ptr,
        );
        (trim(value), length, error, trim(errmsg), ptr)
    }

    /// Parabolic time of flight
    ///
    /// Solve the time of flight equation MA = D + (D**3) / 3
    /// for the parabolic eccentric anomaly D, given mean anomaly.
    ///
    /// Returns `d`.
    ///
    /// See [`partof`](raw::partof) for full documentation.
    pub fn partof(&mut self, ma: f64) -> Result<f64> {
        let mut d: f64 = Default::default();
        raw::partof(self, ma, &mut d)?;
        Ok(d)
    }

    /// PCK, add data to a type 3 segment
    ///
    /// Add data to a type 03 PCK segment in the binary PCK file
    /// associated with HANDLE. See also PCK03B and PCK03E.
    ///
    /// See [`pck03a`](raw::pck03a) for full documentation.
    pub fn pck03a(
        &mut self,
        handle: i32,
        ncsets: i32,
        coeffs: &[f64],
        epochs: &[f64],
    ) -> Result<()> {
        raw::pck03a(self, handle, ncsets, coeffs, epochs)?;
        Ok(())
    }

    /// PCK, begin a type 3 segment
    ///
    /// Begin a type 03 PCK segment in the binary PCK file associated with
    /// HANDLE. See also PCK03A and PCK03E.
    ///
    /// See [`pck03b`](raw::pck03b) for full documentation.
    pub fn pck03b(
        &mut self,
        handle: i32,
        segid: &str,
        body: i32,
        frame: &str,
        first: f64,
        last: f64,
        chbdeg: i32,
    ) -> Result<()> {
        raw::pck03b(self, handle, segid, body, frame, first, last, chbdeg)?;
        Ok(())
    }

    /// PCK, end a type 3 segment
    ///
    /// End the type 03 PCK segment currently being written to the binary
    /// PCK file associated with HANDLE. See also PCK03B and PCK03A.
    ///
    /// See [`pck03e`](raw::pck03e) for full documentation.
    pub fn pck03e(&mut self, handle: i32) -> Result<()> {
        raw::pck03e(self, handle)?;
        Ok(())
    }

    /// PCK, load binary file
    ///
    /// Load a binary PCK file for use by the readers. Return the
    /// handle of the loaded file which is used by other PCK routines to
    /// refer to the file.
    ///
    /// Returns `handle`.
    ///
    /// See [`pcklof`](raw::pcklof) for full documentation.
    pub fn pcklof(&mut self, fname: &str) -> Result<i32> {
        let mut handle: i32 = Default::default();
        raw::pcklof(self, fname, &mut handle)?;
        Ok(handle)
    }

    /// PCK, unload binary file
    ///
    /// Unload a binary PCK file so that it will no longer be searched by
    /// the readers.
    ///
    /// See [`pckuof`](raw::pckuof) for full documentation.
    pub fn pckuof(&mut self, handle: i32) -> Result<()> {
        raw::pckuof(self, handle)?;
        Ok(())
    }

    /// PCK, select file and segment
    ///
    /// Search through loaded files to find the first segment applicable
    /// to the body and time specified. Buffer searched segments in the
    /// process, to attempt to avoid re-reading files.
    ///
    /// Returns `(handle, descr, ident)`.
    ///
    /// See [`pcksfs`](raw::pcksfs) for full documentation.
    pub fn pcksfs(&mut self, body: i32, et: f64) -> Result<Option<(i32, Vec<f64>, String)>> {
        let mut handle: i32 = Default::default();
        let mut descr: Vec<f64> = vec![Default::default(); 5 as usize];
        let mut ident = blank(40);
        let mut found: bool = Default::default();
        raw::pcksfs(
            self,
            body,
            et,
            &mut handle,
            &mut descr,
            &mut ident,
            &mut found,
        )?;
        Ok(if found {
            Some((handle, descr, trim(ident)))
        } else {
            None
        })
    }

    /// PCK, close file
    ///
    /// Close an open PCK file.
    ///
    /// See [`pckcls`](raw::pckcls) for full documentation.
    pub fn pckcls(&mut self, handle: i32) -> Result<()> {
        raw::pckcls(self, handle)?;
        Ok(())
    }

    /// PCK, coverage
    ///
    /// Find the coverage window for a specified reference frame in a
    /// specified binary PCK file.
    ///
    /// See [`pckcov`](raw::pckcov) for full documentation.
    pub fn pckcov(&mut self, pckfnm: &str, idcode: i32, cover: &mut Cell<f64>) -> Result<()> {
        raw::pckcov(self, pckfnm, idcode, cover.as_raw_mut_slice())?;
        Ok(())
    }

    /// PCK, evaluate data record from type 2 segment
    ///
    /// Evaluate a single PCK data record from a segment of type 2
    /// (Chebyshev Polynomials, position only).
    ///
    /// Returns `eulang`.
    ///
    /// See [`pcke02`](raw::pcke02) for full documentation.
    pub fn pcke02(&mut self, et: f64, record: &[f64]) -> Result<[f64; 6]> {
        let mut eulang: [f64; 6] = Default::default();
        raw::pcke02(self, et, record, &mut eulang)?;
        Ok(eulang)
    }

    /// PCK, evaluate data record from type 3 segment
    ///
    /// Evaluate a single PCK data record from a segment of type 03
    /// (Variable width Chebyshev Polynomials for RA, DEC, and W) to
    /// obtain a state transformation matrix.
    ///
    /// Returns `rotmat`.
    ///
    /// See [`pcke03`](raw::pcke03) for full documentation.
    pub fn pcke03(&mut self, et: f64, record: &[f64]) -> Result<[[f64; 6]; 6]> {
        let mut rotmat: [[f64; 6]; 6] = Default::default();
        raw::pcke03(self, et, record, &mut rotmat)?;
        Ok(rotmat)
    }

    /// PCK, evaluate record, type 20
    ///
    /// Evaluate a single PCK data record from a segment of type 20
    /// (Chebyshev Polynomials, rotation derivative only).
    ///
    /// Returns `eulang`.
    ///
    /// See [`pcke20`](raw::pcke20) for full documentation.
    pub fn pcke20(&mut self, et: f64, record: &[f64]) -> Result<[f64; 6]> {
        let mut eulang: [f64; 6] = Default::default();
        raw::pcke20(self, et, record, &mut eulang)?;
        Ok(eulang)
    }

    /// PCK, get reference frame class ID set
    ///
    /// Find the set of reference frame class ID codes of all frames
    /// in a specified binary PCK file.
    ///
    /// See [`pckfrm`](raw::pckfrm) for full documentation.
    pub fn pckfrm(&mut self, pckfnm: &str, ids: &mut Cell<i32>) -> Result<()> {
        raw::pckfrm(self, pckfnm, ids.as_raw_mut_slice())?;
        Ok(())
    }

    /// PCK, get transformation matrix at time
    ///
    /// Return the name of an inertial reference frame and the 6 x 6
    /// state transformation matrix from that frame to the body fixed
    /// frame of a given body at a specified epoch.
    ///
    /// Returns `(ref_, tsipm)`.
    ///
    /// See [`pckmat`](raw::pckmat) for full documentation.
    pub fn pckmat(&mut self, body: i32, et: f64) -> Result<Option<(i32, [[f64; 6]; 6])>> {
        let mut ref_: i32 = Default::default();
        let mut tsipm: [[f64; 6]; 6] = Default::default();
        let mut found: bool = Default::default();
        raw::pckmat(self, body, et, &mut ref_, &mut tsipm, &mut found)?;
        Ok(if found { Some((ref_, tsipm)) } else { None })
    }

    /// PCK, open new file
    ///
    /// Create a new PCK file, returning the handle of the opened file.
    ///
    /// Returns `handle`.
    ///
    /// See [`pckopn`](raw::pckopn) for full documentation.
    pub fn pckopn(&mut self, name: &str, ifname: &str, ncomch: i32) -> Result<i32> {
        let mut handle: i32 = Default::default();
        raw::pckopn(self, name, ifname, ncomch, &mut handle)?;
        Ok(handle)
    }

    /// PCK, pack descriptor
    ///
    /// Perform routine error checks and if all checks pass, pack the
    /// descriptor for a PCK segment
    ///
    /// Returns `descr`.
    ///
    /// See [`pckpds`](raw::pckpds) for full documentation.
    pub fn pckpds(
        &mut self,
        body: i32,
        frame: &str,
        type_: i32,
        first: f64,
        last: f64,
    ) -> Result<Vec<f64>> {
        let mut descr: Vec<f64> = vec![Default::default(); 5 as usize];
        raw::pckpds(self, body, frame, type_, first, last, &mut descr)?;
        Ok(descr)
    }

    /// PCK, read record from type 2 segment
    ///
    /// Read a single PCK data record from a segment of type 2
    /// (Chebyshev, 3-vector only).
    ///
    /// Returns `record`.
    ///
    /// See [`pckr02`](raw::pckr02) for full documentation.
    pub fn pckr02(&mut self, handle: i32, descr: &[f64; 5], et: f64) -> Result<Vec<f64>> {
        let mut record: Vec<f64> = vec![Default::default(); 128 as usize];
        raw::pckr02(self, handle, descr, et, &mut record)?;
        Ok(record)
    }

    /// PCK, read record from type 3 segment
    ///
    /// Read a single PCK data record from a segment of type 03.
    ///
    /// Returns `record`.
    ///
    /// See [`pckr03`](raw::pckr03) for full documentation.
    pub fn pckr03(&mut self, handle: i32, descr: &[f64; 5], et: f64) -> Result<Vec<f64>> {
        let mut record: Vec<f64> = vec![Default::default(); 128 as usize];
        raw::pckr03(self, handle, descr, et, &mut record)?;
        Ok(record)
    }

    /// PCK, read record from segment, type 20
    ///
    /// Read a single PCK data record from a segment of type 20
    /// (Chebyshev, derivative coefficients only).
    ///
    /// Returns `record`.
    ///
    /// See [`pckr20`](raw::pckr20) for full documentation.
    pub fn pckr20(&mut self, handle: i32, descr: &[f64; 5], et: f64) -> Result<Vec<f64>> {
        let mut record: Vec<f64> = vec![Default::default(); 128 as usize];
        raw::pckr20(self, handle, descr, et, &mut record)?;
        Ok(record)
    }

    /// PCK, unpack segment descriptor
    ///
    /// Unpack the contents of a PCK segment descriptor
    ///
    /// Returns `(body, frame, type_, first, last, begin, end)`.
    ///
    /// See [`pckuds`](raw::pckuds) for full documentation.
    pub fn pckuds(&mut self, descr: &[f64]) -> Result<(i32, i32, i32, f64, f64, i32, i32)> {
        let mut body: i32 = Default::default();
        let mut frame: i32 = Default::default();
        let mut type_: i32 = Default::default();
        let mut first: f64 = Default::default();
        let mut last: f64 = Default::default();
        let mut begin: i32 = Default::default();
        let mut end: i32 = Default::default();
        raw::pckuds(
            self, descr, &mut body, &mut frame, &mut type_, &mut first, &mut last, &mut begin,
            &mut end,
        )?;
        Ok((body, frame, type_, first, last, begin, end))
    }

    /// PCK, write type 2 segment
    ///
    /// Write a type 2 segment to a PCK binary file given the file
    /// handle, body-fixed frame class ID, frame, time range covered by
    /// the segment, and the Chebyshev polynomial coefficients.
    ///
    /// See [`pckw02`](raw::pckw02) for full documentation.
    pub fn pckw02(
        &mut self,
        handle: i32,
        clssid: i32,
        frame: &str,
        first: f64,
        last: f64,
        segid: &str,
        intlen: f64,
        n: i32,
        polydg: i32,
        cdata: &[f64],
        btime: f64,
    ) -> Result<()> {
        raw::pckw02(
            self, handle, clssid, frame, first, last, segid, intlen, n, polydg, cdata, btime,
        )?;
        Ok(())
    }

    /// PCK, write segment, type 20
    ///
    /// Write a type 20 segment to a PCK file.
    ///
    /// See [`pckw20`](raw::pckw20) for full documentation.
    pub fn pckw20(
        &mut self,
        handle: i32,
        clssid: i32,
        frame: &str,
        first: f64,
        last: f64,
        segid: &str,
        intlen: f64,
        n: i32,
        polydg: i32,
        cdata: &[f64],
        ascale: f64,
        tscale: f64,
        initjd: f64,
        initfr: f64,
    ) -> Result<()> {
        raw::pckw20(
            self, handle, clssid, frame, first, last, segid, intlen, n, polydg, cdata, ascale,
            tscale, initjd, initfr,
        )?;
        Ok(())
    }

    /// Printable width of a character array
    ///
    /// Determine the printable width of a character array.
    ///
    /// See [`pcwid`](raw::pcwid) for full documentation.
    pub fn pcwid(&self, array: &CharVec, nelt: i32) -> i32 {
        raw::pcwid(array.as_arg(), nelt)
    }

    /// Planetographic to rectangular
    ///
    /// Convert planetographic coordinates to rectangular coordinates.
    ///
    /// Returns `rectan`.
    ///
    /// See [`pgrrec`](raw::pgrrec) for full documentation.
    pub fn pgrrec(
        &mut self,
        body: &str,
        lon: f64,
        lat: f64,
        alt: f64,
        re: f64,
        f: f64,
    ) -> Result<[f64; 3]> {
        let mut rectan: [f64; 3] = Default::default();
        raw::pgrrec(self, body, lon, lat, alt, re, f, &mut rectan)?;
        Ok(rectan)
    }

    /// Phase angle quantity between bodies centers
    ///
    /// Compute the apparent phase angle for a target, observer,
    /// illuminator set of ephemeris objects.
    ///
    /// See [`phaseq`](raw::phaseq) for full documentation.
    pub fn phaseq(
        &mut self,
        et: f64,
        target: &str,
        illmn: &str,
        obsrvr: &str,
        abcorr: &str,
    ) -> Result<f64> {
        raw::phaseq(self, et, target, illmn, obsrvr, abcorr)
    }

    /// Value of pi
    ///
    /// Return the value of pi (the ratio of the circumference of
    /// a circle to its diameter).
    ///
    /// See [`pi`](raw::pi) for full documentation.
    pub fn pi(&mut self) -> f64 {
        raw::pi(self)
    }

    /// Project ellipse onto plane
    ///
    /// Project an ellipse onto a plane, orthogonally.
    ///
    /// Returns `elout`.
    ///
    /// See [`pjelpl`](raw::pjelpl) for full documentation.
    pub fn pjelpl(&mut self, elin: &[f64; 9], plane: &[f64; 4]) -> Result<[f64; 9]> {
        let mut elout: [f64; 9] = Default::default();
        raw::pjelpl(self, elin, plane, &mut elout)?;
        Ok(elout)
    }

    /// Plane to normal vector and constant
    ///
    /// Return a unit normal vector and constant that define a specified
    /// plane.
    ///
    /// Returns `(normal, konst)`.
    ///
    /// See [`pl2nvc`](raw::pl2nvc) for full documentation.
    pub fn pl2nvc(&self, plane: &[f64; 4]) -> ([f64; 3], f64) {
        let mut normal: [f64; 3] = Default::default();
        let mut konst: f64 = Default::default();
        raw::pl2nvc(plane, &mut normal, &mut konst);
        (normal, konst)
    }

    /// Plane to normal vector and point
    ///
    /// Return a unit normal vector and point that define a specified
    /// plane.
    ///
    /// Returns `(normal, point)`.
    ///
    /// See [`pl2nvp`](raw::pl2nvp) for full documentation.
    pub fn pl2nvp(&self, plane: &[f64; 4]) -> ([f64; 3], [f64; 3]) {
        let mut normal: [f64; 3] = Default::default();
        let mut point: [f64; 3] = Default::default();
        raw::pl2nvp(plane, &mut normal, &mut point);
        (normal, point)
    }

    /// Plane to point and spanning vectors
    ///
    /// Return a point and two orthogonal spanning vectors that generate
    /// a specified plane.
    ///
    /// Returns `(point, span1, span2)`.
    ///
    /// See [`pl2psv`](raw::pl2psv) for full documentation.
    pub fn pl2psv(&self, plane: &[f64; 4]) -> ([f64; 3], [f64; 3], [f64; 3]) {
        let mut point: [f64; 3] = Default::default();
        let mut span1: [f64; 3] = Default::default();
        let mut span2: [f64; 3] = Default::default();
        raw::pl2psv(plane, &mut point, &mut span1, &mut span2);
        (point, span1, span2)
    }

    /// Planetographic Longitude Sense
    ///
    /// Indicate for a specified body whether planetographic and
    /// planetocentric longitude increase in the same sense.
    ///
    /// See [`plnsns`](raw::plnsns) for full documentation.
    pub fn plnsns(&mut self, bodid: i32) -> Result<i32> {
        raw::plnsns(self, bodid)
    }

    /// Compute area of plate set
    ///
    /// Compute the total area of a collection of triangular plates.
    ///
    /// See [`pltar`](raw::pltar) for full documentation.
    pub fn pltar(
        &mut self,
        nv: i32,
        vrtces: &[[f64; 3]],
        np: i32,
        plates: &[[i32; 3]],
    ) -> Result<f64> {
        raw::pltar(self, nv, vrtces, np, plates)
    }

    /// Plate expander
    ///
    /// Expand a triangular plate by a specified amount. The expanded
    /// plate is co-planar with, and has the same orientation as, the
    /// original. The centroids of the two plates coincide.
    ///
    /// Returns `overts`.
    ///
    /// See [`pltexp`](raw::pltexp) for full documentation.
    pub fn pltexp(&self, iverts: &[[f64; 3]; 3], delta: f64) -> [[f64; 3]; 3] {
        let mut overts: [[f64; 3]; 3] = Default::default();
        raw::pltexp(iverts, delta, &mut overts);
        overts
    }

    /// Nearest point on triangular plate
    ///
    /// Find the nearest point on a triangular plate to a given point.
    ///
    /// Returns `(pnear, dist)`.
    ///
    /// See [`pltnp`](raw::pltnp) for full documentation.
    pub fn pltnp(
        &mut self,
        point: &[f64; 3],
        v1: &[f64; 3],
        v2: &[f64; 3],
        v3: &[f64; 3],
    ) -> Result<([f64; 3], f64)> {
        let mut pnear: [f64; 3] = Default::default();
        let mut dist: f64 = Default::default();
        raw::pltnp(self, point, v1, v2, v3, &mut pnear, &mut dist)?;
        Ok((pnear, dist))
    }

    /// DSK, compute outward normal of plate
    ///
    /// Compute an outward normal vector of a triangular plate.
    /// The vector does not necessarily have unit length.
    ///
    /// Returns `normal`.
    ///
    /// See [`pltnrm`](raw::pltnrm) for full documentation.
    pub fn pltnrm(&self, v1: &[f64; 3], v2: &[f64; 3], v3: &[f64; 3]) -> [f64; 3] {
        let mut normal: [f64; 3] = Default::default();
        raw::pltnrm(v1, v2, v3, &mut normal);
        normal
    }

    /// Compute volume of plate model
    ///
    /// Compute the volume of a three-dimensional region bounded by a
    /// collection of triangular plates.
    ///
    /// See [`pltvol`](raw::pltvol) for full documentation.
    pub fn pltvol(
        &mut self,
        nv: i32,
        vrtces: &[[f64; 3]],
        np: i32,
        plates: &[[i32; 3]],
    ) -> Result<f64> {
        raw::pltvol(self, nv, vrtces, np, plates)
    }

    /// Compute a Polynomial and its Derivatives
    ///
    /// Compute the value of a polynomial and its first
    /// NDERIV derivatives at the value T.
    ///
    /// Returns `p`.
    ///
    /// See [`polyds`](raw::polyds) for full documentation.
    pub fn polyds(&self, coeffs: &[f64], deg: i32, nderiv: i32, t: f64) -> Vec<f64> {
        let mut p: Vec<f64> = vec![Default::default(); (nderiv + 1).max(0) as usize];
        raw::polyds(coeffs, deg, nderiv, t, &mut p);
        p
    }

    /// Clear the pool of kernel variables
    ///
    /// Remove all kernel variables from the kernel pool. Watches
    /// on kernel variables are retained.
    ///
    /// See [`clpool`](raw::clpool) for full documentation.
    pub fn clpool(&mut self) -> Result<()> {
        raw::clpool(self)?;
        Ok(())
    }

    /// Load variables from a kernel file into the pool
    ///
    /// Load the variables contained in a NAIF ASCII kernel file into the
    /// kernel pool.
    ///
    /// See [`ldpool`](raw::ldpool) for full documentation.
    pub fn ldpool(&mut self, fname: &str) -> Result<()> {
        raw::ldpool(self, fname)?;
        Ok(())
    }

    /// Confirm the existence of a pooled kernel variable
    ///
    /// Confirm the existence of a numeric kernel variable in the kernel
    /// pool.
    ///
    /// See [`expool`](raw::expool) for full documentation.
    pub fn expool(&mut self, name: &str) -> Result<Option<()>> {
        let mut found: bool = Default::default();
        raw::expool(self, name, &mut found)?;
        Ok(if found { Some(()) } else { None })
    }

    /// Write the variables in pool to a specified unit
    ///
    /// Write to a specified unit a set of "keyword = value" assignments
    /// for all currently defined kernel variables. The assignments
    /// constitute a text kernel from which the current state of the
    /// kernel pool can be restored.
    ///
    /// See [`wrpool`](raw::wrpool) for full documentation.
    pub fn wrpool(&mut self, unit: i32) -> Result<()> {
        raw::wrpool(self, unit)?;
        Ok(())
    }

    /// Set watch on a pool variable
    ///
    /// Add a name to the list of agents to notify whenever a member of
    /// a list of kernel variables is updated.
    ///
    /// See [`swpool`](raw::swpool) for full documentation.
    pub fn swpool(&mut self, agent: &str, nnames: i32, names: &CharVec) -> Result<()> {
        raw::swpool(self, agent, nnames, names.as_arg())?;
        Ok(())
    }

    /// Check variable in the pool for update
    ///
    /// Indicate whether or not any watched kernel variables that have a
    /// specified agent on their notification list have been updated.
    ///
    /// Returns `update`.
    ///
    /// See [`cvpool`](raw::cvpool) for full documentation.
    pub fn cvpool(&mut self, agent: &str) -> Result<bool> {
        let mut update: bool = Default::default();
        raw::cvpool(self, agent, &mut update)?;
        Ok(update)
    }

    /// Get character data from the kernel pool
    ///
    /// Return the character value of a kernel variable from the
    /// kernel pool.
    ///
    /// Returns `n`.
    ///
    /// See [`gcpool`](raw::gcpool) for full documentation.
    pub fn gcpool(
        &mut self,
        name: &str,
        start: i32,
        room: i32,
        cvals: &mut CharVec,
    ) -> Result<Option<i32>> {
        let mut n: i32 = Default::default();
        let mut found: bool = Default::default();
        raw::gcpool(
            self,
            name,
            start,
            room,
            &mut n,
            cvals.as_arg_mut(),
            &mut found,
        )?;
        Ok(if found { Some(n) } else { None })
    }

    /// Get d.p. values from the kernel pool
    ///
    /// Return the d.p. value of a kernel variable from the kernel pool.
    ///
    /// Returns `(n, values)`.
    ///
    /// See [`gdpool`](raw::gdpool) for full documentation.
    pub fn gdpool(&mut self, name: &str, start: i32, room: i32) -> Result<Option<(i32, Vec<f64>)>> {
        let mut n: i32 = Default::default();
        let mut values: Vec<f64> = vec![Default::default(); room.max(0) as usize];
        let mut found: bool = Default::default();
        raw::gdpool(self, name, start, room, &mut n, &mut values, &mut found)?;
        Ok(if found { Some((n, values)) } else { None })
    }

    /// Get integers from the kernel pool
    ///
    /// Return the integer value of a kernel variable from the
    /// kernel pool.
    ///
    /// Returns `(n, ivals)`.
    ///
    /// See [`gipool`](raw::gipool) for full documentation.
    pub fn gipool(&mut self, name: &str, start: i32, room: i32) -> Result<Option<(i32, Vec<i32>)>> {
        let mut n: i32 = Default::default();
        let mut ivals: Vec<i32> = vec![Default::default(); room.max(0) as usize];
        let mut found: bool = Default::default();
        raw::gipool(self, name, start, room, &mut n, &mut ivals, &mut found)?;
        Ok(if found { Some((n, ivals)) } else { None })
    }

    /// Data for a kernel pool variable
    ///
    /// Return the data about a kernel pool variable.
    ///
    /// Returns `(n, type_)`.
    ///
    /// See [`dtpool`](raw::dtpool) for full documentation.
    pub fn dtpool(&mut self, name: &str) -> Result<Option<(i32, String)>> {
        let mut found: bool = Default::default();
        let mut n: i32 = Default::default();
        let mut type_ = blank(1);
        raw::dtpool(self, name, &mut found, &mut n, &mut type_)?;
        Ok(if found { Some((n, trim(type_))) } else { None })
    }

    /// Put character strings into the kernel pool
    ///
    /// Provide toolkit programmers a method for programmatically
    /// inserting character data into the kernel pool.
    ///
    /// See [`pcpool`](raw::pcpool) for full documentation.
    pub fn pcpool(&mut self, name: &str, n: i32, cvals: &CharVec) -> Result<()> {
        raw::pcpool(self, name, n, cvals.as_arg())?;
        Ok(())
    }

    /// Put d.p.'s into the kernel pool
    ///
    /// Provide toolkit programmers a method for programmatically
    /// inserting double precision data into the kernel pool.
    ///
    /// See [`pdpool`](raw::pdpool) for full documentation.
    pub fn pdpool(&mut self, name: &str, n: i32, values: &[f64]) -> Result<()> {
        raw::pdpool(self, name, n, values)?;
        Ok(())
    }

    /// Put integers into the kernel pool
    ///
    /// Provide toolkit programmers a method for programmatically
    /// inserting integer data into the kernel pool.
    ///
    /// See [`pipool`](raw::pipool) for full documentation.
    pub fn pipool(&mut self, name: &str, n: i32, ivals: &[i32]) -> Result<()> {
        raw::pipool(self, name, n, ivals)?;
        Ok(())
    }

    /// Load variables from memory into the pool
    ///
    /// Load the variables contained in an internal buffer into the
    /// kernel pool.
    ///
    /// See [`lmpool`](raw::lmpool) for full documentation.
    pub fn lmpool(&mut self, cvals: &CharVec, n: i32) -> Result<()> {
        raw::lmpool(self, cvals.as_arg(), n)?;
        Ok(())
    }

    /// Get size limitations of the kernel pool
    ///
    /// Return the kernel pool size limitations.
    ///
    /// Returns `n`.
    ///
    /// See [`szpool`](raw::szpool) for full documentation.
    pub fn szpool(&mut self, name: &str) -> Result<Option<i32>> {
        let mut n: i32 = Default::default();
        let mut found: bool = Default::default();
        raw::szpool(self, name, &mut n, &mut found)?;
        Ok(if found { Some(n) } else { None })
    }

    /// Delete a variable from the kernel pool
    ///
    /// Delete a variable from the kernel pool.
    ///
    /// See [`dvpool`](raw::dvpool) for full documentation.
    pub fn dvpool(&mut self, name: &str) -> Result<()> {
        raw::dvpool(self, name)?;
        Ok(())
    }

    /// Get names of kernel pool variables
    ///
    /// Return names of kernel variables matching a specified template.
    ///
    /// Returns `n`.
    ///
    /// See [`gnpool`](raw::gnpool) for full documentation.
    pub fn gnpool(
        &mut self,
        name: &str,
        start: i32,
        room: i32,
        cvals: &mut CharVec,
    ) -> Result<Option<i32>> {
        let mut n: i32 = Default::default();
        let mut found: bool = Default::default();
        raw::gnpool(
            self,
            name,
            start,
            room,
            &mut n,
            cvals.as_arg_mut(),
            &mut found,
        )?;
        Ok(if found { Some(n) } else { None })
    }

    /// Delete watch from kernel pool
    ///
    /// Delete a name from the list of agents to notify whenever a member
    /// of a list of kernel variables is updated.
    ///
    /// See [`dwpool`](raw::dwpool) for full documentation.
    pub fn dwpool(&mut self, agent: &str) -> Result<()> {
        raw::dwpool(self, agent)?;
        Ok(())
    }

    /// Position of substring
    ///
    /// Find the first occurrence in a string of a substring, starting at
    /// a specified location, searching forward.
    ///
    /// See [`pos`](raw::pos) for full documentation.
    pub fn pos(&self, str: &str, substr: &str, start: i32) -> i32 {
        raw::pos(str, substr, start)
    }

    /// Position of substring, reverse search
    ///
    /// Find the first occurrence in a string of a substring, starting at
    /// a specified location, searching in reverse.
    ///
    /// See [`posr`](raw::posr) for full documentation.
    pub fn posr(&self, str: &str, substr: &str, start: i32) -> i32 {
        raw::posr(str, substr, start)
    }

    /// Product of a double precision array
    ///
    /// Return the product of the elements of a double precision array.
    ///
    /// See [`prodad`](raw::prodad) for full documentation.
    pub fn prodad(&self, array: &[f64], n: i32) -> f64 {
        raw::prodad(array, n)
    }

    /// Product of an integer array
    ///
    /// Return the product of the elements of an integer array.
    ///
    /// See [`prodai`](raw::prodai) for full documentation.
    pub fn prodai(&self, array: &[i32], n: i32) -> i32 {
        raw::prodai(array, n)
    }

    /// Propagate a two-body solution
    ///
    /// Compute the state of a massless body at time t_0 + DT by applying
    /// the two-body force model to a given central mass and a given body
    /// state at time t_0.
    ///
    /// Returns `pvprop`.
    ///
    /// See [`prop2b`](raw::prop2b) for full documentation.
    pub fn prop2b(&mut self, gm: f64, pvinit: &[f64; 6], dt: f64) -> Result<[f64; 6]> {
        let mut pvprop: [f64; 6] = Default::default();
        raw::prop2b(self, gm, pvinit, dt, &mut pvprop)?;
        Ok(pvprop)
    }

    /// Parse d.p. number with error checking
    ///
    /// Parse a string as a double precision number, encapsulating error
    /// handling.
    ///
    /// Returns `dpval`.
    ///
    /// See [`prsdp`](raw::prsdp) for full documentation.
    pub fn prsdp(&mut self, string: &str) -> Result<f64> {
        let mut dpval: f64 = Default::default();
        raw::prsdp(self, string, &mut dpval)?;
        Ok(dpval)
    }

    /// Parse integer with error checking
    ///
    /// Parse a string as an integer, encapsulating error handling.
    ///
    /// Returns `intval`.
    ///
    /// See [`prsint`](raw::prsint) for full documentation.
    pub fn prsint(&mut self, string: &str) -> Result<i32> {
        let mut intval: i32 = Default::default();
        raw::prsint(self, string, &mut intval)?;
        Ok(intval)
    }

    /// Encode a character string, portably
    ///
    /// Encode a nonnegative integer number into a character string,
    /// portably, using 128 as the base for encoding.
    ///
    /// Returns `string`.
    ///
    /// See [`prtenc`](raw::prtenc) for full documentation.
    pub fn prtenc(&mut self, number: i32) -> Result<String> {
        let mut string = blank(5);
        raw::prtenc(self, number, &mut string)?;
        Ok(trim(string))
    }

    /// Decode a character string
    ///
    /// Decode a character string encoded by PRTENC.
    ///
    /// Returns `number`.
    ///
    /// See [`prtdec`](raw::prtdec) for full documentation.
    pub fn prtdec(&mut self, string: &str) -> Result<i32> {
        let mut number: i32 = Default::default();
        raw::prtdec(self, string, &mut number)?;
        Ok(number)
    }

    /// Is This Message Type Selected for Output?
    ///
    /// Indicate whether the specified message type has been selected
    /// for output.
    ///
    /// See [`msgsel`](raw::msgsel) for full documentation.
    pub fn msgsel(&mut self, type_: &str) -> Result<bool> {
        raw::msgsel(self, type_)
    }

    /// Point and spanning vectors to plane
    ///
    /// Make a SPICE plane from a point and two spanning vectors.
    ///
    /// Returns `plane`.
    ///
    /// See [`psv2pl`](raw::psv2pl) for full documentation.
    pub fn psv2pl(
        &mut self,
        point: &[f64; 3],
        span1: &[f64; 3],
        span2: &[f64; 3],
    ) -> Result<[f64; 4]> {
        let mut plane: [f64; 4] = Default::default();
        raw::psv2pl(self, point, span1, span2, &mut plane)?;
        Ok(plane)
    }

    /// Get Error Response Action
    ///
    /// Return the value of the current error response action.
    ///
    /// Returns `action`.
    ///
    /// See [`getact`](raw::getact) for full documentation.
    pub fn getact(&mut self) -> i32 {
        let mut action: i32 = Default::default();
        raw::getact(self, &mut action);
        action
    }

    /// Get Error Output Device Specification
    ///
    /// Return the value of the current error output device
    /// specification.
    ///
    /// Returns `device`.
    ///
    /// See [`getdev`](raw::getdev) for full documentation.
    pub fn getdev(&mut self) -> String {
        let mut device = blank(255);
        raw::getdev(self, &mut device);
        trim(device)
    }

    /// Get Long Error Message
    ///
    /// Return the value of the current long error message.
    ///
    /// Returns `msg`.
    ///
    /// See [`getlms`](raw::getlms) for full documentation.
    pub fn getlms(&mut self) -> String {
        let mut msg = blank(inc::errhnd::LMSGLN);
        raw::getlms(self, &mut msg);
        trim(msg)
    }

    /// Get Short Error Message
    ///
    /// Return the value of the current short error message.
    ///
    /// Returns `msg`.
    ///
    /// See [`getsms`](raw::getsms) for full documentation.
    pub fn getsms(&mut self) -> String {
        let mut msg = blank(inc::errhnd::SMSGLN);
        raw::getsms(self, &mut msg);
        trim(msg)
    }

    /// Position Transformation Matrix
    ///
    /// Return the matrix that transforms position vectors from one
    /// specified frame to another at a specified epoch.
    ///
    /// Returns `rotate`.
    ///
    /// See [`pxform`](raw::pxform) for full documentation.
    pub fn pxform(&mut self, from: &str, to: &str, et: f64) -> Result<[[f64; 3]; 3]> {
        let mut rotate: [[f64; 3]; 3] = Default::default();
        raw::pxform(self, from, to, et, &mut rotate)?;
        Ok(rotate)
    }

    /// Position Transform Matrix, Different Epochs
    ///
    /// Return the 3x3 matrix that transforms position vectors from one
    /// specified frame at a specified epoch to another specified
    /// frame at another specified epoch.
    ///
    /// Returns `rotate`.
    ///
    /// See [`pxfrm2`](raw::pxfrm2) for full documentation.
    pub fn pxfrm2(
        &mut self,
        from: &str,
        to: &str,
        etfrom: f64,
        etto: f64,
    ) -> Result<[[f64; 3]; 3]> {
        let mut rotate: [[f64; 3]; 3] = Default::default();
        raw::pxfrm2(self, from, to, etfrom, etto, &mut rotate)?;
        Ok(rotate)
    }

    /// Quaternion to matrix
    ///
    /// Find the rotation matrix corresponding to a specified unit
    /// quaternion.
    ///
    /// Returns `r`.
    ///
    /// See [`q2m`](raw::q2m) for full documentation.
    pub fn q2m(&self, q: &[f64; 4]) -> [[f64; 3]; 3] {
        let mut r: [[f64; 3]; 3] = Default::default();
        raw::q2m(q, &mut r);
        r
    }

    /// Quadratic derivative
    ///
    /// Estimate the derivative of a function by finding the derivative
    /// of a quadratic approximating function. This derivative estimate
    /// is equivalent to that found by computing the average of forward
    /// and backward differences.
    ///
    /// Returns `dfdt`.
    ///
    /// See [`qderiv`](raw::qderiv) for full documentation.
    pub fn qderiv(&mut self, ndim: i32, f0: &[f64], f2: &[f64], delta: f64) -> Result<Vec<f64>> {
        let mut dfdt: Vec<f64> = vec![Default::default(); ndim.max(0) as usize];
        raw::qderiv(self, ndim, f0, f2, delta, &mut dfdt)?;
        Ok(dfdt)
    }

    /// Quaternion and quaternion derivative to a.v.
    ///
    /// Derive angular velocity from a unit quaternion and its derivative
    /// with respect to time.
    ///
    /// Returns `av`.
    ///
    /// See [`qdq2av`](raw::qdq2av) for full documentation.
    pub fn qdq2av(&self, q: &[f64; 4], dq: &[f64; 4]) -> [f64; 3] {
        let mut av: [f64; 3] = Default::default();
        raw::qdq2av(q, dq, &mut av);
        av
    }

    /// Quaternion times quaternion
    ///
    /// Multiply two quaternions.
    ///
    /// Returns `qout`.
    ///
    /// See [`qxq`](raw::qxq) for full documentation.
    pub fn qxq(&self, q1: &[f64; 4], q2: &[f64; 4]) -> [f64; 4] {
        let mut qout: [f64; 4] = Default::default();
        raw::qxq(q1, q2, &mut qout);
        qout
    }

    /// Range, RA and DEC to rectangular coordinates
    ///
    /// Convert from range, right ascension, and declination to
    /// rectangular coordinates.
    ///
    /// Returns `rectan`.
    ///
    /// See [`radrec`](raw::radrec) for full documentation.
    pub fn radrec(&self, range: f64, ra: f64, dec: f64) -> [f64; 3] {
        let mut rectan: [f64; 3] = Default::default();
        raw::radrec(range, ra, dec, &mut rectan);
        rectan
    }

    /// Rotation and angular velocity to transform
    ///
    /// Determine a state transformation matrix from a rotation matrix
    /// and the angular velocity of the rotation.
    ///
    /// Returns `xform`.
    ///
    /// See [`rav2xf`](raw::rav2xf) for full documentation.
    pub fn rav2xf(&self, rot: &[[f64; 3]; 3], av: &[f64; 3]) -> [[f64; 6]; 6] {
        let mut xform: [[f64; 6]; 6] = Default::default();
        raw::rav2xf(rot, av, &mut xform);
        xform
    }

    /// Rotation axis of a matrix
    ///
    /// Compute the axis of the rotation given by an input matrix
    /// and the angle of the rotation about that axis.
    ///
    /// Returns `(axis, angle)`.
    ///
    /// See [`raxisa`](raw::raxisa) for full documentation.
    pub fn raxisa(&mut self, matrix: &[[f64; 3]; 3]) -> Result<([f64; 3], f64)> {
        let mut axis: [f64; 3] = Default::default();
        let mut angle: f64 = Default::default();
        raw::raxisa(self, matrix, &mut axis, &mut angle)?;
        Ok((axis, angle))
    }

    /// Read encoded characters from a text file
    ///
    /// Read and decode encoded characters from a text file.
    ///
    /// See [`rdencc`](raw::rdencc) for full documentation.
    pub fn rdencc(&mut self, unit: i32, n: i32, data: &mut CharVec) -> Result<()> {
        raw::rdencc(self, unit, n, data.as_arg_mut())?;
        Ok(())
    }

    /// Read encoded d.p. numbers from file
    ///
    /// Read N encoded d.p. numbers from a text file, decoding them
    /// into their equivalent d.p. numbers.
    ///
    /// Returns `data`.
    ///
    /// See [`rdencd`](raw::rdencd) for full documentation.
    pub fn rdencd(&mut self, unit: i32, n: i32) -> Result<Vec<f64>> {
        let mut data: Vec<f64> = vec![Default::default(); n.max(0) as usize];
        raw::rdencd(self, unit, n, &mut data)?;
        Ok(data)
    }

    /// Read encoded integers from text file
    ///
    /// Read N encoded integers from a text file, decoding them into
    /// their equivalent integers.
    ///
    /// Returns `data`.
    ///
    /// See [`rdenci`](raw::rdenci) for full documentation.
    pub fn rdenci(&mut self, unit: i32, n: i32) -> Result<Vec<i32>> {
        let mut data: Vec<i32> = vec![Default::default(); n.max(0) as usize];
        raw::rdenci(self, unit, n, &mut data)?;
        Ok(data)
    }

    /// Open and initialize a new kernel file
    ///
    /// Open and initialize a SPICE ASCII kernel file.
    ///
    /// See [`rdknew`](raw::rdknew) for full documentation.
    pub fn rdknew(&mut self, kernel: &str) -> Result<()> {
        raw::rdknew(self, kernel)?;
        Ok(())
    }

    /// Read the next data line from a kernel file
    ///
    /// Read the next line of data from a SPICE ASCII kernel file.
    ///
    /// Returns `(line, eof)`.
    ///
    /// See [`rdkdat`](raw::rdkdat) for full documentation.
    pub fn rdkdat(&mut self) -> Result<(String, bool)> {
        let mut line = blank(80);
        let mut eof: bool = Default::default();
        raw::rdkdat(self, &mut line, &mut eof)?;
        Ok((trim(line), eof))
    }

    /// Reading kernel at line number
    ///
    /// Return the name of file and line number of the last line read by
    /// the entry point RDKDAT.
    ///
    /// Returns `(kernel, number)`.
    ///
    /// See [`rdklin`](raw::rdklin) for full documentation.
    pub fn rdklin(&mut self) -> (String, i32) {
        let mut kernel = blank(255);
        let mut number: i32 = Default::default();
        raw::rdklin(self, &mut kernel, &mut number);
        (trim(kernel), number)
    }

    /// Read the next variable from a kernel file
    ///
    /// Read the next variable from a SPICE ASCII kernel file into a
    /// double precision symbol table.
    ///
    /// Returns `(name, eof)`.
    ///
    /// See [`rdkvar`](raw::rdkvar) for full documentation.
    pub fn rdkvar(
        &mut self,
        tabsym: &mut CharCell,
        tabptr: &mut Cell<i32>,
        tabval: &mut Cell<f64>,
    ) -> Result<(String, bool)> {
        let mut name = blank(80);
        let mut eof: bool = Default::default();
        raw::rdkvar(
            self,
            tabsym.as_arg_mut(),
            tabptr.as_raw_mut_slice(),
            tabval.as_raw_mut_slice(),
            &mut name,
            &mut eof,
        )?;
        Ok((trim(name), eof))
    }

    /// Read non-blank line
    ///
    /// Read the next non-blank line of text from a text file.
    ///
    /// Returns `eof`.
    ///
    /// See [`rdnbl`](raw::rdnbl) for full documentation.
    pub fn rdnbl(&mut self, file: &str, line: &mut str) -> Result<bool> {
        let mut eof: bool = Default::default();
        raw::rdnbl(self, file, line, &mut eof)?;
        Ok(eof)
    }

    /// Read a line from a text file
    ///
    /// Read the next line of text from a text file.
    ///
    /// Returns `eof`.
    ///
    /// See [`rdtext`](raw::rdtext) for full documentation.
    pub fn rdtext(&mut self, file: &str, line: &mut str) -> Result<bool> {
        let mut eof: bool = Default::default();
        raw::rdtext(self, file, line, &mut eof)?;
        Ok(eof)
    }

    /// Close a text file opened by RDTEXT
    ///
    /// Close a text file currently opened by RDTEXT.
    ///
    /// See [`cltext`](raw::cltext) for full documentation.
    pub fn cltext(&mut self, file: &str) -> Result<()> {
        raw::cltext(self, file)?;
        Ok(())
    }

    /// Read array of lines from a logical unit
    ///
    /// Read lines from a Fortran logical unit and place them in a
    /// character string array.
    ///
    /// Returns `(numlin, eof)`.
    ///
    /// See [`readla`](raw::readla) for full documentation.
    pub fn readla(&mut self, unit: i32, maxlin: i32, array: &mut CharVec) -> Result<(i32, bool)> {
        let mut numlin: i32 = Default::default();
        let mut eof: bool = Default::default();
        raw::readla(
            self,
            unit,
            maxlin,
            &mut numlin,
            array.as_arg_mut(),
            &mut eof,
        )?;
        Ok((numlin, eof))
    }

    /// Read a text line from a logical unit
    ///
    /// Read a single line of text from the Fortran logical unit UNIT,
    /// reporting the end of file if it occurs.
    ///
    /// Returns `eof`.
    ///
    /// See [`readln`](raw::readln) for full documentation.
    pub fn readln(&mut self, unit: i32, line: &mut str) -> Result<bool> {
        let mut eof: bool = Default::default();
        raw::readln(self, unit, line, &mut eof)?;
        Ok(eof)
    }

    /// Rectangular coordinates to AZ/EL
    ///
    /// Convert rectangular coordinates of a point to range, azimuth and
    /// elevation.
    ///
    /// Returns `(range, az, el)`.
    ///
    /// See [`recazl`](raw::recazl) for full documentation.
    pub fn recazl(&mut self, rectan: &[f64; 3], azccw: bool, elplsz: bool) -> (f64, f64, f64) {
        let mut range: f64 = Default::default();
        let mut az: f64 = Default::default();
        let mut el: f64 = Default::default();
        raw::recazl(self, rectan, azccw, elplsz, &mut range, &mut az, &mut el);
        (range, az, el)
    }

    /// Rectangular to cylindrical coordinates
    ///
    /// Convert from rectangular to cylindrical coordinates.
    ///
    /// Returns `(r, clon, z)`.
    ///
    /// See [`reccyl`](raw::reccyl) for full documentation.
    pub fn reccyl(&mut self, rectan: &[f64; 3]) -> (f64, f64, f64) {
        let mut r: f64 = Default::default();
        let mut clon: f64 = Default::default();
        let mut z: f64 = Default::default();
        raw::reccyl(self, rectan, &mut r, &mut clon, &mut z);
        (r, clon, z)
    }

    /// Rectangular to geodetic
    ///
    /// Convert from rectangular coordinates to geodetic coordinates.
    ///
    /// Returns `(lon, lat, alt)`.
    ///
    /// See [`recgeo`](raw::recgeo) for full documentation.
    pub fn recgeo(&mut self, rectan: &[f64; 3], re: f64, f: f64) -> Result<(f64, f64, f64)> {
        let mut lon: f64 = Default::default();
        let mut lat: f64 = Default::default();
        let mut alt: f64 = Default::default();
        raw::recgeo(self, rectan, re, f, &mut lon, &mut lat, &mut alt)?;
        Ok((lon, lat, alt))
    }

    /// Rectangular to latitudinal coordinates
    ///
    /// Convert from rectangular coordinates to latitudinal coordinates.
    ///
    /// Returns `(radius, lon, lat)`.
    ///
    /// See [`reclat`](raw::reclat) for full documentation.
    pub fn reclat(&self, rectan: &[f64; 3]) -> (f64, f64, f64) {
        let mut radius: f64 = Default::default();
        let mut lon: f64 = Default::default();
        let mut lat: f64 = Default::default();
        raw::reclat(rectan, &mut radius, &mut lon, &mut lat);
        (radius, lon, lat)
    }

    /// Rectangular to planetographic
    ///
    /// Convert rectangular coordinates to planetographic coordinates.
    ///
    /// Returns `(lon, lat, alt)`.
    ///
    /// See [`recpgr`](raw::recpgr) for full documentation.
    pub fn recpgr(
        &mut self,
        body: &str,
        rectan: &[f64; 3],
        re: f64,
        f: f64,
    ) -> Result<(f64, f64, f64)> {
        let mut lon: f64 = Default::default();
        let mut lat: f64 = Default::default();
        let mut alt: f64 = Default::default();
        raw::recpgr(self, body, rectan, re, f, &mut lon, &mut lat, &mut alt)?;
        Ok((lon, lat, alt))
    }

    /// Rectangular coordinates to RA and DEC
    ///
    /// Convert rectangular coordinates to range, right ascension,
    /// and declination.
    ///
    /// Returns `(range, ra, dec)`.
    ///
    /// See [`recrad`](raw::recrad) for full documentation.
    pub fn recrad(&mut self, rectan: &[f64; 3]) -> (f64, f64, f64) {
        let mut range: f64 = Default::default();
        let mut ra: f64 = Default::default();
        let mut dec: f64 = Default::default();
        raw::recrad(self, rectan, &mut range, &mut ra, &mut dec);
        (range, ra, dec)
    }

    /// Rectangular to spherical coordinates
    ///
    /// Convert from rectangular coordinates to spherical coordinates.
    ///
    /// Returns `(r, colat, slon)`.
    ///
    /// See [`recsph`](raw::recsph) for full documentation.
    pub fn recsph(&self, rectan: &[f64; 3]) -> (f64, f64, f64) {
        let mut r: f64 = Default::default();
        let mut colat: f64 = Default::default();
        let mut slon: f64 = Default::default();
        raw::recsph(rectan, &mut r, &mut colat, &mut slon);
        (r, colat, slon)
    }

    /// Reference frame Change
    ///
    /// Return the transformation matrix from one
    /// frame to another.
    ///
    /// Returns `rotate`.
    ///
    /// See [`refchg`](raw::refchg) for full documentation.
    pub fn refchg(&mut self, frame1: i32, frame2: i32, et: f64) -> Result<[[f64; 3]; 3]> {
        let mut rotate: [[f64; 3]; 3] = Default::default();
        raw::refchg(self, frame1, frame2, et, &mut rotate)?;
        Ok(rotate)
    }

    /// Remove elements from a character array
    ///
    /// Remove one or more elements from a character array at the
    /// indicated location.
    ///
    /// See [`remlac`](raw::remlac) for full documentation.
    pub fn remlac(&mut self, ne: i32, loc: i32, array: &mut CharVec, na: &mut i32) -> Result<()> {
        raw::remlac(self, ne, loc, array.as_arg_mut(), na)?;
        Ok(())
    }

    /// Remove elements from a double precision array
    ///
    /// Remove one or more elements from a double precision array at the
    /// indicated location.
    ///
    /// See [`remlad`](raw::remlad) for full documentation.
    pub fn remlad(&mut self, ne: i32, loc: i32, array: &mut [f64], na: &mut i32) -> Result<()> {
        raw::remlad(self, ne, loc, array, na)?;
        Ok(())
    }

    /// Remove elements from an integer array
    ///
    /// Remove one or more elements from an integer array at the
    /// indicated location.
    ///
    /// See [`remlai`](raw::remlai) for full documentation.
    pub fn remlai(&mut self, ne: i32, loc: i32, array: &mut [i32], na: &mut i32) -> Result<()> {
        raw::remlai(self, ne, loc, array, na)?;
        Ok(())
    }

    /// Remove an item from a character set
    ///
    /// Remove an item from a character set.
    ///
    /// See [`removc`](raw::removc) for full documentation.
    pub fn removc(&mut self, item: &str, a: &mut CharCell) -> Result<()> {
        raw::removc(self, item, a.as_arg_mut())?;
        Ok(())
    }

    /// Remove an item from a double precision set
    ///
    /// Remove an item from a double precision set.
    ///
    /// See [`removd`](raw::removd) for full documentation.
    pub fn removd(&mut self, item: f64, a: &mut Cell<f64>) -> Result<()> {
        raw::removd(self, item, a.as_raw_mut_slice())?;
        Ok(())
    }

    /// Remove an item from an integer set
    ///
    /// Remove an item from an integer set.
    ///
    /// See [`removi`](raw::removi) for full documentation.
    pub fn removi(&mut self, item: i32, a: &mut Cell<i32>) -> Result<()> {
        raw::removi(self, item, a.as_raw_mut_slice())?;
        Ok(())
    }

    /// Reorder a character array
    ///
    /// Reorder the elements of an array of character strings according to
    /// a given order vector.
    ///
    /// See [`reordc`](raw::reordc) for full documentation.
    pub fn reordc(&self, iorder: &mut [i32], ndim: i32, array: &mut CharVec) {
        raw::reordc(iorder, ndim, array.as_arg_mut());
    }

    /// Reorder a double precision array
    ///
    /// Reorder the elements of a double precision array according to a
    /// given order vector.
    ///
    /// See [`reordd`](raw::reordd) for full documentation.
    pub fn reordd(&self, iorder: &mut [i32], ndim: i32, array: &mut [f64]) {
        raw::reordd(iorder, ndim, array);
    }

    /// Reorder an integer array
    ///
    /// Reorder the elements of an integer array according to a given
    /// order vector.
    ///
    /// See [`reordi`](raw::reordi) for full documentation.
    pub fn reordi(&self, iorder: &mut [i32], ndim: i32, array: &mut [i32]) {
        raw::reordi(iorder, ndim, array);
    }

    /// Reorder a logical array
    ///
    /// Reorder the elements of a logical array according to a given order
    /// vector.
    ///
    /// See [`reordl`](raw::reordl) for full documentation.
    pub fn reordl(&self, iorder: &mut [i32], ndim: i32, array: &mut [bool]) {
        raw::reordl(iorder, ndim, array);
    }

    /// Replace a word
    ///
    /// Replace the Nth word in a string with a new word.
    ///
    /// See [`replwd`](raw::replwd) for full documentation.
    pub fn replwd(&self, instr: &str, nth: i32, new: &str, outstr: &mut str) {
        raw::replwd(instr, nth, new, outstr);
    }

    /// Reset Error Status
    ///
    /// Reset the SPICELIB error status to a value of "no error."
    /// As a result, the status routine, FAILED, will return a value
    /// of .FALSE.
    ///
    /// See [`reset`](raw::reset) for full documentation.
    pub fn reset(&mut self) {
        raw::reset(self);
    }

    /// Immediate Return Indicator
    ///
    /// Return .TRUE. if SPICELIB routines should return immediately upon
    /// entry.
    ///
    /// See [`return_`](raw::return_) for full documentation.
    pub fn return_(&mut self) -> bool {
        raw::return_(self)
    }

    /// Remainder --- double precision
    ///
    /// Compute the integer quotient and non-negative remainder
    /// of NUM and DENOM.
    ///
    /// Returns `(q, rem)`.
    ///
    /// See [`rmaind`](raw::rmaind) for full documentation.
    pub fn rmaind(&mut self, num: f64, denom: f64) -> Result<(f64, f64)> {
        let mut q: f64 = Default::default();
        let mut rem: f64 = Default::default();
        raw::rmaind(self, num, denom, &mut q, &mut rem)?;
        Ok((q, rem))
    }

    /// Remainder --- integer
    ///
    /// Compute the integer quotient and non-negative remainder
    /// of NUM and DENOM.
    ///
    /// Returns `(q, rem)`.
    ///
    /// See [`rmaini`](raw::rmaini) for full documentation.
    pub fn rmaini(&mut self, num: i32, denom: i32) -> Result<(i32, i32)> {
        let mut q: i32 = Default::default();
        let mut rem: i32 = Default::default();
        raw::rmaini(self, num, denom, &mut q, &mut rem)?;
        Ok((q, rem))
    }

    /// Remove duplicates from a character array
    ///
    /// Remove duplicate elements from a character array.
    ///
    /// See [`rmdupc`](raw::rmdupc) for full documentation.
    pub fn rmdupc(&self, nelt: &mut i32, array: &mut CharVec) {
        raw::rmdupc(nelt, array.as_arg_mut());
    }

    /// Remove duplicates from a double precision array
    ///
    /// Remove duplicate elements from a double precision array.
    ///
    /// See [`rmdupd`](raw::rmdupd) for full documentation.
    pub fn rmdupd(&self, nelt: &mut i32, array: &mut [f64]) {
        raw::rmdupd(nelt, array);
    }

    /// Remove duplicates from an integer array
    ///
    /// Remove duplicate elements from an integer array.
    ///
    /// See [`rmdupi`](raw::rmdupi) for full documentation.
    pub fn rmdupi(&self, nelt: &mut i32, array: &mut [i32]) {
        raw::rmdupi(nelt, array);
    }

    /// Generate a rotation matrix
    ///
    /// Calculate the 3x3 rotation matrix generated by a rotation
    /// of a specified angle about a specified axis. This rotation
    /// is thought of as rotating the coordinate system.
    ///
    /// Returns `mout`.
    ///
    /// See [`rotate`](raw::rotate) for full documentation.
    pub fn rotate(&mut self, angle: f64, iaxis: i32) -> [[f64; 3]; 3] {
        let mut mout: [[f64; 3]; 3] = Default::default();
        raw::rotate(self, angle, iaxis, &mut mout);
        mout
    }

    /// Frame get rotation
    ///
    /// Find the rotation from a user specified frame to another frame at
    /// a user specified epoch.
    ///
    /// Returns `(rotate, outfrm)`.
    ///
    /// See [`rotget`](raw::rotget) for full documentation.
    pub fn rotget(&mut self, infrm: i32, et: f64) -> Result<Option<([[f64; 3]; 3], i32)>> {
        let mut rotate: [[f64; 3]; 3] = Default::default();
        let mut outfrm: i32 = Default::default();
        let mut found: bool = Default::default();
        raw::rotget(self, infrm, et, &mut rotate, &mut outfrm, &mut found)?;
        Ok(if found { Some((rotate, outfrm)) } else { None })
    }

    /// Rotate a matrix
    ///
    /// Apply a rotation of ANGLE radians about axis IAXIS to a matrix.
    /// This rotation is thought of as rotating the coordinate system.
    ///
    /// Returns `mout`.
    ///
    /// See [`rotmat`](raw::rotmat) for full documentation.
    pub fn rotmat(&mut self, m1: &[[f64; 3]; 3], angle: f64, iaxis: i32) -> [[f64; 3]; 3] {
        let mut mout: [[f64; 3]; 3] = Default::default();
        raw::rotmat(self, m1, angle, iaxis, &mut mout);
        mout
    }

    /// Transform a vector via a rotation
    ///
    /// Transform a vector to a new reference frame rotated by ANGLE
    /// radians about axis IAXIS. This transformation rotates V1 by
    /// -ANGLE radians about the specified axis.
    ///
    /// Returns `vout`.
    ///
    /// See [`rotvec`](raw::rotvec) for full documentation.
    pub fn rotvec(&mut self, v1: &[f64; 3], angle: f64, iaxis: i32) -> [f64; 3] {
        let mut vout: [f64; 3] = Default::default();
        raw::rotvec(self, v1, angle, iaxis, &mut vout);
        vout
    }

    /// Radians per degree
    ///
    /// Return the number of radians per degree.
    ///
    /// See [`rpd`](raw::rpd) for full documentation.
    pub fn rpd(&mut self) -> f64 {
        raw::rpd(self)
    }

    /// Roots of a quadratic equation
    ///
    /// Find the roots of a quadratic equation.
    ///
    /// Returns `(root1, root2)`.
    ///
    /// See [`rquad`](raw::rquad) for full documentation.
    pub fn rquad(&mut self, a: f64, b: f64, c: f64) -> Result<([f64; 2], [f64; 2])> {
        let mut root1: [f64; 2] = Default::default();
        let mut root2: [f64; 2] = Default::default();
        raw::rquad(self, a, b, c, &mut root1, &mut root2)?;
        Ok((root1, root2))
    }

    /// Right trim
    ///
    /// Return the maximum of 1 and the location of the last non-blank
    /// character in the string.
    ///
    /// See [`rtrim`](raw::rtrim) for full documentation.
    pub fn rtrim(&self, string: &str) -> i32 {
        raw::rtrim(string)
    }

    /// Semi-axes of ellipse from generating vectors
    ///
    /// Find semi-axis vectors of an ellipse generated by two arbitrary
    /// three-dimensional vectors.
    ///
    /// Returns `(smajor, sminor)`.
    ///
    /// See [`saelgv`](raw::saelgv) for full documentation.
    pub fn saelgv(&mut self, vec1: &[f64; 3], vec2: &[f64; 3]) -> Result<([f64; 3], [f64; 3])> {
        let mut smajor: [f64; 3] = Default::default();
        let mut sminor: [f64; 3] = Default::default();
        raw::saelgv(self, vec1, vec2, &mut smajor, &mut sminor)?;
        Ok((smajor, sminor))
    }

    /// Same character
    ///
    /// Determine if two characters from different strings are the
    /// same.
    ///
    /// See [`samch`](raw::samch) for full documentation.
    pub fn samch(&self, str1: &str, l1: i32, str2: &str, l2: i32) -> bool {
        raw::samch(str1, l1, str2, l2)
    }

    /// Same character --- case insensitive
    ///
    /// Determine if two characters from different strings are the
    /// same when the case of the characters is ignored.
    ///
    /// See [`samchi`](raw::samchi) for full documentation.
    pub fn samchi(&mut self, str1: &str, l1: i32, str2: &str, l2: i32) -> bool {
        raw::samchi(self, str1, l1, str2, l2)
    }

    /// Are two integer arrays the same?
    ///
    /// Indicate whether two integer arrays are equal.
    ///
    /// See [`sameai`](raw::sameai) for full documentation.
    pub fn sameai(&self, a1: &[i32], a2: &[i32], ndim: i32) -> bool {
        raw::sameai(a1, a2, ndim)
    }

    /// Same substrings, case insensitive
    ///
    /// Determine whether or not two substrings are the same up to
    /// case.
    ///
    /// See [`samsbi`](raw::samsbi) for full documentation.
    pub fn samsbi(&mut self, str1: &str, b1: i32, e1: i32, str2: &str, b2: i32, e2: i32) -> bool {
        raw::samsbi(self, str1, b1, e1, str2, b2, e2)
    }

    /// Same substrings
    ///
    /// Determine whether or not two substrings are the same
    ///
    /// See [`samsub`](raw::samsub) for full documentation.
    pub fn samsub(&self, str1: &str, b1: i32, e1: i32, str2: &str, b2: i32, e2: i32) -> bool {
        raw::samsub(str1, b1, e1, str2, b2, e2)
    }

    /// Convert type 1 SCLK string to ticks
    ///
    /// Convert a character representation of a type 1 spacecraft clock
    /// count to ticks.
    ///
    /// Returns `ticks`.
    ///
    /// See [`sctk01`](raw::sctk01) for full documentation.
    pub fn sctk01(&mut self, sc: i32, clkstr: &str) -> Result<f64> {
        let mut ticks: f64 = Default::default();
        raw::sctk01(self, sc, clkstr, &mut ticks)?;
        Ok(ticks)
    }

    /// Convert ticks to a type 1 SCLK string.
    ///
    /// Convert a number of ticks to an equivalent type 1 spacecraft clock
    /// string.
    ///
    /// Returns `clkstr`.
    ///
    /// See [`scfm01`](raw::scfm01) for full documentation.
    pub fn scfm01(&mut self, sc: i32, ticks: f64) -> Result<String> {
        let mut clkstr = blank(256);
        raw::scfm01(self, sc, ticks, &mut clkstr)?;
        Ok(trim(clkstr))
    }

    /// Ticks to ET, type 01
    ///
    /// Convert encoded type 1 spacecraft clock (`ticks') to ephemeris
    /// seconds past J2000 (ET).
    ///
    /// See [`scte01`](raw::scte01) for full documentation.
    pub fn scte01(&mut self, sc: i32, sclkdp: f64, et: &mut f64) -> Result<()> {
        raw::scte01(self, sc, sclkdp, et)?;
        Ok(())
    }

    /// ET to discrete ticks, type 1
    ///
    /// Convert ephemeris seconds past J2000 (ET) to discrete encoded
    /// type 1 spacecraft clock (`ticks').
    ///
    /// Returns `sclkdp`.
    ///
    /// See [`scet01`](raw::scet01) for full documentation.
    pub fn scet01(&mut self, sc: i32, et: f64) -> Result<f64> {
        let mut sclkdp: f64 = Default::default();
        raw::scet01(self, sc, et, &mut sclkdp)?;
        Ok(sclkdp)
    }

    /// ET to continuous ticks, type 1
    ///
    /// Convert ephemeris seconds past J2000 (ET) to continuous encoded
    /// type 1 spacecraft clock (`ticks').  The output value need not be
    /// integral.
    ///
    /// Returns `sclkdp`.
    ///
    /// See [`scec01`](raw::scec01) for full documentation.
    pub fn scec01(&mut self, sc: i32, et: f64) -> Result<f64> {
        let mut sclkdp: f64 = Default::default();
        raw::scec01(self, sc, et, &mut sclkdp)?;
        Ok(sclkdp)
    }

    /// SCLK type, using type 1 SCLK database
    ///
    /// Return the type of a specified clock. The clock need not be
    /// type 1.
    ///
    /// Returns `clktyp`.
    ///
    /// See [`scty01`](raw::scty01) for full documentation.
    pub fn scty01(&mut self, sc: i32) -> Result<i32> {
        let mut clktyp: i32 = Default::default();
        raw::scty01(self, sc, &mut clktyp)?;
        Ok(clktyp)
    }

    /// Scanning preparation
    ///
    /// Prepare recognized markers and auxiliary arrays for the
    /// routine SCAN.
    ///
    /// Returns `(mrklen, pnters)`.
    ///
    /// See [`scanpr`](raw::scanpr) for full documentation.
    pub fn scanpr(&self, nmarks: &mut i32, marks: &mut CharVec) -> (Vec<i32>, Vec<i32>) {
        let mut mrklen: Vec<i32> = vec![Default::default(); (*nmarks).max(0) as usize];
        let mut pnters: Vec<i32> = vec![Default::default(); 132 as usize];
        raw::scanpr(nmarks, marks.as_arg_mut(), &mut mrklen, &mut pnters);
        (mrklen, pnters)
    }

    /// Scan a string for tokens
    ///
    /// Scan a string and return the beginnings and ends of recognized
    /// and unrecognized substrings. The full collection of these
    /// substrings partitions the string.
    ///
    /// Returns `(ntokns, ident, beg, end)`.
    ///
    /// See [`scan`](raw::scan) for full documentation.
    pub fn scan(
        &self,
        string: &str,
        marks: &CharVec,
        mrklen: &[i32],
        pnters: &[i32],
        room: i32,
        start: &mut i32,
    ) -> (i32, Vec<i32>, Vec<i32>, Vec<i32>) {
        let mut ntokns: i32 = Default::default();
        let mut ident: Vec<i32> = vec![Default::default(); room.max(0) as usize];
        let mut beg: Vec<i32> = vec![Default::default(); room.max(0) as usize];
        let mut end: Vec<i32> = vec![Default::default(); room.max(0) as usize];
        raw::scan(
            string,
            marks.as_arg(),
            mrklen,
            pnters,
            room,
            start,
            &mut ntokns,
            &mut ident,
            &mut beg,
            &mut end,
        );
        (ntokns, ident, beg, end)
    }

    /// Scan --- reject tokens
    ///
    /// Reject those tokens descriptors whose identities are among those
    /// of a specific collection.
    ///
    /// See [`scanrj`](raw::scanrj) for full documentation.
    pub fn scanrj(
        &self,
        ids: &[i32],
        n: i32,
        ntokns: &mut i32,
        ident: &mut [i32],
        beg: &mut [i32],
        end: &mut [i32],
    ) {
        raw::scanrj(ids, n, ntokns, ident, beg, end);
    }

    /// Set the cardinality of a character cell
    ///
    /// Set the cardinality of a character cell.
    ///
    /// See [`scardc`](raw::scardc) for full documentation.
    pub fn scardc(&mut self, card: i32, cell: &mut CharCell) -> Result<()> {
        raw::scardc(self, card, cell.as_arg_mut())?;
        Ok(())
    }

    /// Set the cardinality of a double precision cell
    ///
    /// Set the cardinality of a double precision cell.
    ///
    /// See [`scardd`](raw::scardd) for full documentation.
    pub fn scardd(&mut self, card: i32, cell: &mut Cell<f64>) -> Result<()> {
        raw::scardd(self, card, cell.as_raw_mut_slice())?;
        Ok(())
    }

    /// Set the cardinality of an integer cell
    ///
    /// Set the cardinality of an integer cell.
    ///
    /// See [`scardi`](raw::scardi) for full documentation.
    pub fn scardi(&mut self, card: i32, cell: &mut Cell<i32>) -> Result<()> {
        raw::scardi(self, card, cell.as_raw_mut_slice())?;
        Ok(())
    }

    /// Decode spacecraft clock
    ///
    /// Convert a double precision encoding of spacecraft clock time into
    /// a character representation.
    ///
    /// Returns `sclkch`.
    ///
    /// See [`scdecd`](raw::scdecd) for full documentation.
    pub fn scdecd(&mut self, sc: i32, sclkdp: f64) -> Result<String> {
        let mut sclkch = blank(256);
        raw::scdecd(self, sc, sclkdp, &mut sclkch)?;
        Ok(trim(sclkch))
    }

    /// ET to continuous SCLK ticks
    ///
    /// Convert ephemeris seconds past J2000 (ET) to continuous encoded
    /// spacecraft clock (`ticks').  Non-integral tick values may be
    /// returned.
    ///
    /// Returns `sclkdp`.
    ///
    /// See [`sce2c`](raw::sce2c) for full documentation.
    pub fn sce2c(&mut self, sc: i32, et: f64) -> Result<f64> {
        let mut sclkdp: f64 = Default::default();
        raw::sce2c(self, sc, et, &mut sclkdp)?;
        Ok(sclkdp)
    }

    /// ET to SCLK string
    ///
    /// Convert an epoch specified as ephemeris seconds past J2000 (ET) to
    /// a character string representation of a spacecraft clock value
    /// (SCLK).
    ///
    /// Returns `sclkch`.
    ///
    /// See [`sce2s`](raw::sce2s) for full documentation.
    pub fn sce2s(&mut self, sc: i32, et: f64) -> Result<String> {
        let mut sclkch = blank(256);
        raw::sce2s(self, sc, et, &mut sclkch)?;
        Ok(trim(sclkch))
    }

    /// ET to discrete SCLK ticks
    ///
    /// Convert ephemeris seconds past J2000 (ET) to integral
    /// encoded spacecraft clock (`ticks'). For conversion to
    /// fractional ticks, (required for C-kernel production), see
    /// the routine SCE2C.
    ///
    /// Returns `sclkdp`.
    ///
    /// See [`sce2t`](raw::sce2t) for full documentation.
    pub fn sce2t(&mut self, sc: i32, et: f64) -> Result<f64> {
        let mut sclkdp: f64 = Default::default();
        raw::sce2t(self, sc, et, &mut sclkdp)?;
        Ok(sclkdp)
    }

    /// Encode spacecraft clock
    ///
    /// Encode a character representation of spacecraft clock time into a
    /// double precision number.
    ///
    /// Returns `sclkdp`.
    ///
    /// See [`scencd`](raw::scencd) for full documentation.
    pub fn scencd(&mut self, sc: i32, sclkch: &str) -> Result<f64> {
        let mut sclkdp: f64 = Default::default();
        raw::scencd(self, sc, sclkch, &mut sclkdp)?;
        Ok(sclkdp)
    }

    /// Convert SCLK "ticks" to character clock format
    ///
    /// Convert encoded spacecraft clock ticks to character clock format.
    ///
    /// Returns `clkstr`.
    ///
    /// See [`scfmt`](raw::scfmt) for full documentation.
    pub fn scfmt(&mut self, sc: i32, ticks: f64) -> Result<String> {
        let mut clkstr = blank(256);
        raw::scfmt(self, sc, ticks, &mut clkstr)?;
        Ok(trim(clkstr))
    }

    /// SCLK lookup of integer data, type 1
    ///
    /// Look up integer type 1 SCLK data from the kernel pool.
    ///
    /// Returns `(n, ival)`.
    ///
    /// See [`scli01`](raw::scli01) for full documentation.
    pub fn scli01(&mut self, name: &str, sc: i32, maxnv: i32) -> Result<(i32, Vec<i32>)> {
        let mut n: i32 = Default::default();
        let mut ival: Vec<i32> = vec![Default::default(); maxnv.max(0) as usize];
        raw::scli01(self, name, sc, maxnv, &mut n, &mut ival)?;
        Ok((n, ival))
    }

    /// SCLK lookup of double precision data, type 1
    ///
    /// Look up double precision type 1 SCLK data from the kernel pool.
    ///
    /// Returns `(n, dval)`.
    ///
    /// See [`scld01`](raw::scld01) for full documentation.
    pub fn scld01(&mut self, name: &str, sc: i32, maxnv: i32) -> Result<(i32, Vec<f64>)> {
        let mut n: i32 = Default::default();
        let mut dval: Vec<f64> = vec![Default::default(); maxnv.max(0) as usize];
        raw::scld01(self, name, sc, maxnv, &mut n, &mut dval)?;
        Ok((n, dval))
    }

    /// Parse a spacecraft clock string
    ///
    /// Parse a character representation of spacecraft clock time and
    /// encode it as a double precision number.
    ///
    /// Returns `(error, msg, sclkdp)`.
    ///
    /// See [`scpars`](raw::scpars) for full documentation.
    pub fn scpars(&mut self, sc: i32, sclkch: &str) -> Result<(bool, String, f64)> {
        let mut error: bool = Default::default();
        let mut msg = blank(inc::errhnd::LMSGLN);
        let mut sclkdp: f64 = Default::default();
        raw::scpars(self, sc, sclkch, &mut error, &mut msg, &mut sclkdp)?;
        Ok((error, trim(msg), sclkdp))
    }

    /// Spacecraft Clock Partition Information
    ///
    /// Get spacecraft clock partition information from a spacecraft
    /// clock kernel file.
    ///
    /// Returns `(nparts, pstart, pstop)`.
    ///
    /// See [`scpart`](raw::scpart) for full documentation.
    pub fn scpart(&mut self, sc: i32) -> Result<(i32, Vec<f64>, Vec<f64>)> {
        let mut nparts: i32 = Default::default();
        let mut pstart: Vec<f64> = vec![Default::default(); inc::sclk::MXPART as usize];
        let mut pstop: Vec<f64> = vec![Default::default(); inc::sclk::MXPART as usize];
        raw::scpart(self, sc, &mut nparts, &mut pstart, &mut pstop)?;
        Ok((nparts, pstart, pstop))
    }

    /// Convert type 1 SCLK string to ticks
    ///
    /// Convert a character representation of a type 1 spacecraft clock
    /// count to ticks.
    ///
    /// Returns `(error, msg, ticks)`.
    ///
    /// See [`scps01`](raw::scps01) for full documentation.
    pub fn scps01(&mut self, sc: i32, clkstr: &str) -> Result<(bool, String, f64)> {
        let mut error: bool = Default::default();
        let mut msg = blank(inc::errhnd::LMSGLN);
        let mut ticks: f64 = Default::default();
        raw::scps01(self, sc, clkstr, &mut error, &mut msg, &mut ticks)?;
        Ok((error, trim(msg), ticks))
    }

    /// SCLK string to ET
    ///
    /// Convert a spacecraft clock string to ephemeris seconds past
    /// J2000 (ET).
    ///
    /// Returns `et`.
    ///
    /// See [`scs2e`](raw::scs2e) for full documentation.
    pub fn scs2e(&mut self, sc: i32, sclkch: &str) -> Result<f64> {
        let mut et: f64 = Default::default();
        raw::scs2e(self, sc, sclkch, &mut et)?;
        Ok(et)
    }

    /// SCLK ticks to ET
    ///
    /// Convert encoded spacecraft clock (`ticks') to ephemeris
    /// seconds past J2000 (ET).
    ///
    /// Returns `et`.
    ///
    /// See [`sct2e`](raw::sct2e) for full documentation.
    pub fn sct2e(&mut self, sc: i32, sclkdp: f64) -> Result<f64> {
        let mut et: f64 = Default::default();
        raw::sct2e(self, sc, sclkdp, &mut et)?;
        Ok(et)
    }

    /// Convert spacecraft clock string to ticks.
    ///
    /// Convert a spacecraft clock format string to number of "ticks".
    ///
    /// Returns `ticks`.
    ///
    /// See [`sctiks`](raw::sctiks) for full documentation.
    pub fn sctiks(&mut self, sc: i32, clkstr: &str) -> Result<f64> {
        let mut ticks: f64 = Default::default();
        raw::sctiks(self, sc, clkstr, &mut ticks)?;
        Ok(ticks)
    }

    /// SCLK name to ID code
    ///
    /// Convert an SCLK name string to a NAIF integer code.
    ///
    /// Returns `clkid`.
    ///
    /// See [`scn2id`](raw::scn2id) for full documentation.
    pub fn scn2id(&mut self, clknam: &str) -> Result<Option<i32>> {
        let mut clkid: i32 = Default::default();
        let mut found: bool = Default::default();
        raw::scn2id(self, clknam, &mut clkid, &mut found)?;
        Ok(if found { Some(clkid) } else { None })
    }

    /// SCLK ID code to name
    ///
    /// Convert a NAIF integer code for a spacecraft clock to an SCLK name
    /// string.
    ///
    /// Returns `clknam`.
    ///
    /// See [`scid2n`](raw::scid2n) for full documentation.
    pub fn scid2n(&mut self, clkid: i32) -> Result<Option<String>> {
        let mut clknam = blank(256);
        let mut found: bool = Default::default();
        raw::scid2n(self, clkid, &mut clknam, &mut found)?;
        Ok(if found { Some(trim(clknam)) } else { None })
    }

    /// SCLK type
    ///
    /// Return the spacecraft clock type for a specified spacecraft.
    ///
    /// See [`sctype`](raw::sctype) for full documentation.
    pub fn sctype(&mut self, sc: i32) -> Result<i32> {
        raw::sctype(self, sc)
    }

    /// Symmetric difference of two character sets
    ///
    /// Take the symmetric difference of two character sets to form
    /// a third set.
    ///
    /// See [`sdiffc`](raw::sdiffc) for full documentation.
    pub fn sdiffc(&mut self, a: &CharCell, b: &CharCell, c: &mut CharCell) -> Result<()> {
        raw::sdiffc(self, a.as_arg(), b.as_arg(), c.as_arg_mut())?;
        Ok(())
    }

    /// Symmetric difference of two double precision sets
    ///
    /// Take the symmetric difference of two double precision sets
    /// to form a third set.
    ///
    /// See [`sdiffd`](raw::sdiffd) for full documentation.
    pub fn sdiffd(&mut self, a: &Cell<f64>, b: &Cell<f64>, c: &mut Cell<f64>) -> Result<()> {
        raw::sdiffd(
            self,
            a.as_raw_slice(),
            b.as_raw_slice(),
            c.as_raw_mut_slice(),
        )?;
        Ok(())
    }

    /// Symmetric difference of two integer sets
    ///
    /// Take the symmetric difference of two integer sets to form
    /// a third set.
    ///
    /// See [`sdiffi`](raw::sdiffi) for full documentation.
    pub fn sdiffi(&mut self, a: &Cell<i32>, b: &Cell<i32>, c: &mut Cell<i32>) -> Result<()> {
        raw::sdiffi(
            self,
            a.as_raw_slice(),
            b.as_raw_slice(),
            c.as_raw_mut_slice(),
        )?;
        Ok(())
    }

    /// String from pool
    ///
    /// Retrieve the string starting at the FIDX element of the kernel
    /// pool variable, where the string may be continued across several
    /// components of the kernel pool variable.
    ///
    /// Returns `(size, lidx)`.
    ///
    /// See [`sepool`](raw::sepool) for full documentation.
    pub fn sepool(
        &mut self,
        item: &str,
        fidx: i32,
        contin: &str,
        string: &mut str,
    ) -> Result<Option<(i32, i32)>> {
        let mut size: i32 = Default::default();
        let mut lidx: i32 = Default::default();
        let mut found: bool = Default::default();
        raw::sepool(
            self, item, fidx, contin, string, &mut size, &mut lidx, &mut found,
        )?;
        Ok(if found { Some((size, lidx)) } else { None })
    }

    /// Compare character sets
    ///
    /// Compare two character sets, as indicated by a relational operator.
    ///
    /// See [`setc`](raw::setc) for full documentation.
    pub fn setc(&mut self, a: &CharCell, op: &str, b: &CharCell) -> Result<bool> {
        raw::setc(self, a.as_arg(), op, b.as_arg())
    }

    /// Compare double precision sets
    ///
    /// Compare two double precision sets, as indicated by a relational
    /// operator.
    ///
    /// See [`setd`](raw::setd) for full documentation.
    pub fn setd(&mut self, a: &Cell<f64>, op: &str, b: &Cell<f64>) -> Result<bool> {
        raw::setd(self, a.as_raw_slice(), op, b.as_raw_slice())
    }

    /// Error Status Indicator
    ///
    /// Return .TRUE. if an error condition has been signaled via SIGERR.
    /// FAILED is the SPICELIB status indicator.
    ///
    /// See [`failed`](raw::failed) for full documentation.
    pub fn failed(&mut self) -> bool {
        raw::failed(self)
    }

    /// Compare integer sets
    ///
    /// Compare two integer sets, as indicated by a relational operator.
    ///
    /// See [`seti`](raw::seti) for full documentation.
    pub fn seti(&mut self, a: &Cell<i32>, op: &str, b: &Cell<i32>) -> Result<bool> {
        raw::seti(self, a.as_raw_slice(), op, b.as_raw_slice())
    }

    /// Set Long Error Message
    ///
    /// Set the value of the current long error message.
    ///
    /// See [`setmsg`](raw::setmsg) for full documentation.
    pub fn setmsg(&mut self, msg: &str) {
        raw::setmsg(self, msg);
    }

    /// Generic Segments: Fetch constants
    ///
    /// Fetch from the constants partition of a generic segment the
    /// double precision numbers from FIRST to LAST. The segment is
    /// identified by a DAF file handle and segment descriptor.
    ///
    /// Returns `values`.
    ///
    /// See [`sgfcon`](raw::sgfcon) for full documentation.
    pub fn sgfcon(
        &mut self,
        handle: i32,
        descr: &[f64],
        first: i32,
        last: i32,
    ) -> Result<Vec<f64>> {
        let mut values: Vec<f64> = vec![Default::default(); (last + 1 - first).max(0) as usize];
        raw::sgfcon(self, handle, descr, first, last, &mut values)?;
        Ok(values)
    }

    /// Generic Segment: Fetch data packets
    ///
    /// Fetch the data packets indexed from FIRST to LAST from the
    /// packet partition of a generic segment. The segment is
    /// identified by a DAF file handle and segment descriptor.
    ///
    /// Returns `(values, ends)`.
    ///
    /// See [`sgfpkt`](raw::sgfpkt) for full documentation.
    pub fn sgfpkt(
        &mut self,
        handle: i32,
        descr: &[f64],
        first: i32,
        last: i32,
    ) -> Result<(Vec<f64>, Vec<i32>)> {
        let mut values: Vec<f64> = vec![Default::default(); (last + 1 - first).max(0) as usize];
        let mut ends: Vec<i32> = vec![Default::default(); (last + 1 - first).max(0) as usize];
        raw::sgfpkt(self, handle, descr, first, last, &mut values, &mut ends)?;
        Ok((values, ends))
    }

    /// Generic Segments: Fetch references
    ///
    /// Fetch from the references partition of a generic segment
    /// the double precision numbers from FIRST to LAST. The
    /// segment is identified by a DAF file handle and segment
    /// descriptor.
    ///
    /// Returns `values`.
    ///
    /// See [`sgfref`](raw::sgfref) for full documentation.
    pub fn sgfref(
        &mut self,
        handle: i32,
        descr: &[f64],
        first: i32,
        last: i32,
    ) -> Result<Vec<f64>> {
        let mut values: Vec<f64> = vec![Default::default(); (last + 1 - first).max(0) as usize];
        raw::sgfref(self, handle, descr, first, last, &mut values)?;
        Ok(values)
    }

    /// Generic Segments: Fetch ref. value and index
    ///
    /// Find the reference value associated with the value X and its
    /// index in a generic segment. The segment is identified by a DAF
    /// file handle and segment descriptor.
    ///
    /// Returns `(value, indx)`.
    ///
    /// See [`sgfrvi`](raw::sgfrvi) for full documentation.
    pub fn sgfrvi(&mut self, handle: i32, descr: &[f64], x: f64) -> Result<Option<(f64, i32)>> {
        let mut value: f64 = Default::default();
        let mut indx: i32 = Default::default();
        let mut found: bool = Default::default();
        raw::sgfrvi(self, handle, descr, x, &mut value, &mut indx, &mut found)?;
        Ok(if found { Some((value, indx)) } else { None })
    }

    /// Generic segments: Fetch meta data value
    ///
    /// Obtain the value of a specified generic segment meta data item.
    ///
    /// Returns `value`.
    ///
    /// See [`sgmeta`](raw::sgmeta) for full documentation.
    pub fn sgmeta(&mut self, handle: i32, descr: &[f64], mnemon: i32) -> Result<i32> {
        let mut value: i32 = Default::default();
        raw::sgmeta(self, handle, descr, mnemon, &mut value)?;
        Ok(value)
    }

    /// Generic segments: Begin a fixed size segment.
    ///
    /// Begin writing a generic segment that will contain fixed size data
    /// packets.
    ///
    /// See [`sgbwfs`](raw::sgbwfs) for full documentation.
    pub fn sgbwfs(
        &mut self,
        handle: i32,
        descr: &[f64],
        segid: &str,
        nconst: i32,
        const_: &[f64],
        pktsiz: &[i32],
        idxtyp: i32,
    ) -> Result<()> {
        raw::sgbwfs(self, handle, descr, segid, nconst, const_, pktsiz, idxtyp)?;
        Ok(())
    }

    /// Generic segments: Begin a variable size segment.
    ///
    /// Begin writing a generic segment that will contain variable size
    /// data packets.
    ///
    /// See [`sgbwvs`](raw::sgbwvs) for full documentation.
    pub fn sgbwvs(
        &mut self,
        handle: i32,
        descr: &[f64],
        segid: &str,
        nconst: i32,
        const_: &[f64],
        idxtyp: i32,
    ) -> Result<()> {
        raw::sgbwvs(self, handle, descr, segid, nconst, const_, idxtyp)?;
        Ok(())
    }

    /// Generic segments: Write fixed size packets.
    ///
    /// Write one or more fixed size data packets to the generic segment
    /// currently being written to the DAF file associated with HANDLE.
    ///
    /// See [`sgwfpk`](raw::sgwfpk) for full documentation.
    pub fn sgwfpk(
        &mut self,
        handle: i32,
        npkts: i32,
        pktdat: &[f64],
        nrefs: i32,
        refdat: &[f64],
    ) -> Result<()> {
        raw::sgwfpk(self, handle, npkts, pktdat, nrefs, refdat)?;
        Ok(())
    }

    /// Generic segment: Write variable size packets.
    ///
    /// Write one or more variable size data packets to the generic
    /// segment currently being written to the DAF file associated with
    /// HANDLE.
    ///
    /// See [`sgwvpk`](raw::sgwvpk) for full documentation.
    pub fn sgwvpk(
        &mut self,
        handle: i32,
        npkts: i32,
        pktsiz: &[i32],
        pktdat: &[f64],
        nrefs: i32,
        refdat: &[f64],
    ) -> Result<()> {
        raw::sgwvpk(self, handle, npkts, pktsiz, pktdat, nrefs, refdat)?;
        Ok(())
    }

    /// Generic segments: End a segment.
    ///
    /// End the generic segment in the DAF file associated with HANDLE.
    ///
    /// See [`sgwes`](raw::sgwes) for full documentation.
    pub fn sgwes(&mut self, handle: i32) -> Result<()> {
        raw::sgwes(self, handle)?;
        Ok(())
    }

    /// Sharpen a rotation
    ///
    /// Adjust the columns of a matrix that is "nearly" a rotation
    /// so that they are numerically unit length and orthogonal,
    /// going from left to right in the usual printed presentation
    /// of a matrix.
    ///
    /// See [`sharpr`](raw::sharpr) for full documentation.
    pub fn sharpr(&self, rot: &mut [[f64; 3]; 3]) {
        raw::sharpr(rot);
    }

    /// Shell sort a character array
    ///
    /// Sort an array of character strings according to the ASCII
    /// collating sequence using the Shell Sort algorithm.
    ///
    /// See [`shellc`](raw::shellc) for full documentation.
    pub fn shellc(&self, ndim: i32, array: &mut CharVec) {
        raw::shellc(ndim, array.as_arg_mut());
    }

    /// Shell sort a double precision array
    ///
    /// Sort a double precision array using the Shell Sort algorithm.
    ///
    /// See [`shelld`](raw::shelld) for full documentation.
    pub fn shelld(&self, ndim: i32, array: &mut [f64]) {
        raw::shelld(ndim, array);
    }

    /// Shell sort an integer array
    ///
    /// Sort an integer array using the Shell Sort algorithm.
    ///
    /// See [`shelli`](raw::shelli) for full documentation.
    pub fn shelli(&self, ndim: i32, array: &mut [i32]) {
        raw::shelli(ndim, array);
    }

    /// Retain significant digits
    ///
    /// Retain only the significant digits in a numeric string.
    ///
    /// Returns `out`.
    ///
    /// See [`sigdgt`](raw::sigdgt) for full documentation.
    pub fn sigdgt(&self, in_: &str) -> String {
        let mut out = blank((in_.len() as i32));
        raw::sigdgt(in_, &mut out);
        trim(out)
    }

    /// Signal Error Condition
    ///
    /// Inform the SPICELIB error processing mechanism that an error has
    /// occurred, and specify the type of error.
    ///
    /// See [`sigerr`](raw::sigerr) for full documentation.
    pub fn sigerr(&mut self, msg: &str) -> Result<()> {
        raw::sigerr(self, msg)?;
        Ok(())
    }

    /// Surface intercept
    ///
    /// Compute, for a given observer and a ray emanating from the
    /// observer, the surface intercept of the ray on a target body at
    /// a specified epoch, optionally corrected for light time and
    /// stellar aberration.
    ///
    /// The surface of the target body may be represented by a triaxial
    /// ellipsoid or by topographic data provided by DSK files.
    ///
    /// This routine supersedes SRFXPT.
    ///
    /// Returns `(spoint, trgepc, srfvec)`.
    ///
    /// See [`sincpt`](raw::sincpt) for full documentation.
    pub fn sincpt(
        &mut self,
        method: &str,
        target: &str,
        et: f64,
        fixref: &str,
        abcorr: &str,
        obsrvr: &str,
        dref: &str,
        dvec: &[f64; 3],
    ) -> Result<Option<([f64; 3], f64, [f64; 3])>> {
        let mut spoint: [f64; 3] = Default::default();
        let mut trgepc: f64 = Default::default();
        let mut srfvec: [f64; 3] = Default::default();
        let mut found: bool = Default::default();
        raw::sincpt(
            self,
            method,
            target,
            et,
            fixref,
            abcorr,
            obsrvr,
            dref,
            dvec,
            &mut spoint,
            &mut trgepc,
            &mut srfvec,
            &mut found,
        )?;
        Ok(if found {
            Some((spoint, trgepc, srfvec))
        } else {
            None
        })
    }

    /// Size of a character cell
    ///
    /// Return the size (maximum cardinality) of a character cell.
    ///
    /// See [`sizec`](raw::sizec) for full documentation.
    pub fn sizec(&mut self, cell: &CharCell) -> Result<i32> {
        raw::sizec(self, cell.as_arg())
    }

    /// Size of a double precision cell
    ///
    /// Return the size (maximum cardinality) of a double precision
    /// cell.
    ///
    /// See [`sized`](raw::sized) for full documentation.
    pub fn sized(&mut self, cell: &Cell<f64>) -> Result<i32> {
        raw::sized(self, cell.as_raw_slice())
    }

    /// Size of an integer cell
    ///
    /// Return the size (maximum cardinality) of an integer cell.
    ///
    /// See [`sizei`](raw::sizei) for full documentation.
    pub fn sizei(&mut self, cell: &Cell<i32>) -> Result<i32> {
        raw::sizei(self, cell.as_raw_slice())
    }

    /// Same Sign Double Precision Numbers
    ///
    /// Return .TRUE. if two given double precision numbers have the same
    /// sign.
    ///
    /// See [`smsgnd`](raw::smsgnd) for full documentation.
    pub fn smsgnd(&self, x: f64, y: f64) -> bool {
        raw::smsgnd(x, y)
    }

    /// Same Sign Integer Numbers
    ///
    /// Return .TRUE. if two given integer numbers have the same sign.
    ///
    /// See [`smsgni`](raw::smsgni) for full documentation.
    pub fn smsgni(&self, x: i32, y: i32) -> bool {
        raw::smsgni(x, y)
    }

    /// Some entries false?
    ///
    /// Determine if some of the entries in an array of logicals are
    /// .FALSE.
    ///
    /// See [`somfls`](raw::somfls) for full documentation.
    pub fn somfls(&self, logcls: &[bool], n: i32) -> bool {
        raw::somfls(logcls, n)
    }

    /// Some entries true?
    ///
    /// Determine if some of the entries in an array of logicals are
    /// .TRUE.
    ///
    /// See [`somtru`](raw::somtru) for full documentation.
    pub fn somtru(&self, logcls: &[bool], n: i32) -> bool {
        raw::somtru(logcls, n)
    }

    /// SPK and CK, ASCII to binary
    ///
    /// Convert a text (ASCII) format SPK or CK file to an equivalent
    /// binary file, including comments.
    ///
    /// See [`spca2b`](raw::spca2b) for full documentation.
    pub fn spca2b(&mut self, text: &str, binary: &str) -> Result<()> {
        raw::spca2b(self, text, binary)?;
        Ok(())
    }

    /// SPK and CK, add comments
    ///
    /// Store text from a text file in the comment area of a binary SPK
    /// or CK file, appending it to whatever text may already have
    /// been stored there.
    ///
    /// See [`spcac`](raw::spcac) for full documentation.
    pub fn spcac(&mut self, handle: i32, unit: i32, bmark: &str, emark: &str) -> Result<()> {
        raw::spcac(self, handle, unit, bmark, emark)?;
        Ok(())
    }

    /// SPK and CK, binary to ASCII
    ///
    /// Convert a binary SPK or CK file to an equivalent text (ASCII)
    /// file, including the comment area.
    ///
    /// See [`spcb2a`](raw::spcb2a) for full documentation.
    pub fn spcb2a(&mut self, binary: &str, text: &str) -> Result<()> {
        raw::spcb2a(self, binary, text)?;
        Ok(())
    }

    /// SPK and CK, binary to text
    ///
    /// Convert the contents of a binary SPK or CK file to text,
    /// including comments if present, and write them to a text file
    /// opened by the calling program.
    ///
    /// See [`spcb2t`](raw::spcb2t) for full documentation.
    pub fn spcb2t(&mut self, binary: &str, unit: i32) -> Result<()> {
        raw::spcb2t(self, binary, unit)?;
        Ok(())
    }

    /// SPK and CK, delete comments
    ///
    /// Empty the comment area of a binary SPK or CK file.
    ///
    /// See [`spcdc`](raw::spcdc) for full documentation.
    pub fn spcdc(&mut self, handle: i32) -> Result<()> {
        raw::spcdc(self, handle)?;
        Ok(())
    }

    /// SPK and CK, extract comments
    ///
    /// Extract the text from the comment area of a binary SPK or CK file
    /// and write it to a text file.
    ///
    /// See [`spcec`](raw::spcec) for full documentation.
    pub fn spcec(&mut self, handle: i32, unit: i32) -> Result<()> {
        raw::spcec(self, handle, unit)?;
        Ok(())
    }

    /// SPK or CK, open new file
    ///
    /// Open a new SPK or CK file for subsequent write requests.
    ///
    /// Returns `handle`.
    ///
    /// See [`spcopn`](raw::spcopn) for full documentation.
    pub fn spcopn(&mut self, fname: &str, ifname: &str) -> Result<i32> {
        let mut handle: i32 = Default::default();
        raw::spcopn(self, fname, ifname, &mut handle)?;
        Ok(handle)
    }

    /// SPK and CK, read first line of comments
    ///
    /// Read the first line of text from the comment area
    /// of a binary SPK or CK file.
    ///
    /// Returns `(line, eoc)`.
    ///
    /// See [`spcrfl`](raw::spcrfl) for full documentation.
    pub fn spcrfl(&mut self, handle: i32) -> Result<(String, bool)> {
        let mut line = blank(80);
        let mut eoc: bool = Default::default();
        raw::spcrfl(self, handle, &mut line, &mut eoc)?;
        Ok((trim(line), eoc))
    }

    /// SPK and CK, read next line of comments
    ///
    /// Continue reading lines from the comment area of a binary
    /// SPK or CK file specified by the most recent call to
    /// the routine SPCRFL.
    ///
    /// Returns `(line, eoc)`.
    ///
    /// See [`spcrnl`](raw::spcrnl) for full documentation.
    pub fn spcrnl(&mut self) -> Result<(String, bool)> {
        let mut line = blank(80);
        let mut eoc: bool = Default::default();
        raw::spcrnl(self, &mut line, &mut eoc)?;
        Ok((trim(line), eoc))
    }

    /// SPK and CK, text to binary
    ///
    /// Reconstruct a binary SPK or CK file including comments
    /// from a text file opened by the calling program.
    ///
    /// See [`spct2b`](raw::spct2b) for full documentation.
    pub fn spct2b(&mut self, unit: i32, binary: &str) -> Result<()> {
        raw::spct2b(self, unit, binary)?;
        Ok(())
    }

    /// Seconds per day
    ///
    /// Return the number of seconds in a day.
    ///
    /// See [`spd`](raw::spd) for full documentation.
    pub fn spd(&self) -> f64 {
        raw::spd()
    }

    /// Spherical to cylindrical coordinates
    ///
    /// Convert from spherical coordinates to cylindrical coordinates.
    ///
    /// Returns `(r, clon, z)`.
    ///
    /// See [`sphcyl`](raw::sphcyl) for full documentation.
    pub fn sphcyl(&self, radius: f64, colat: f64, slon: f64) -> (f64, f64, f64) {
        let mut r: f64 = Default::default();
        let mut clon: f64 = Default::default();
        let mut z: f64 = Default::default();
        raw::sphcyl(radius, colat, slon, &mut r, &mut clon, &mut z);
        (r, clon, z)
    }

    /// Spherical to latitudinal coordinates
    ///
    /// Convert from spherical coordinates to latitudinal coordinates.
    ///
    /// Returns `(radius, lon, lat)`.
    ///
    /// See [`sphlat`](raw::sphlat) for full documentation.
    pub fn sphlat(&mut self, r: f64, colat: f64, slon: f64) -> (f64, f64, f64) {
        let mut radius: f64 = Default::default();
        let mut lon: f64 = Default::default();
        let mut lat: f64 = Default::default();
        raw::sphlat(self, r, colat, slon, &mut radius, &mut lon, &mut lat);
        (radius, lon, lat)
    }

    /// Spherical to rectangular coordinates
    ///
    /// Convert from spherical coordinates to rectangular coordinates.
    ///
    /// Returns `rectan`.
    ///
    /// See [`sphrec`](raw::sphrec) for full documentation.
    pub fn sphrec(&self, r: f64, colat: f64, slon: f64) -> [f64; 3] {
        let mut rectan: [f64; 3] = Default::default();
        raw::sphrec(r, colat, slon, &mut rectan);
        rectan
    }

    /// Spherical surface distance
    ///
    /// Return the distance between two points on a sphere, measured
    /// along the shortest great circle arc connecting them.
    ///
    /// See [`sphsd`](raw::sphsd) for full documentation.
    pub fn sphsd(
        &mut self,
        radius: f64,
        lon1: f64,
        lat1: f64,
        lon2: f64,
        lat2: f64,
    ) -> Result<f64> {
        raw::sphsd(self, radius, lon1, lat1, lon2, lat2)
    }

    /// SPK type 14: Add data to a segment
    ///
    /// Add data to a type 14 SPK segment associated with HANDLE.
    ///
    /// See [`spk14a`](raw::spk14a) for full documentation.
    pub fn spk14a(
        &mut self,
        handle: i32,
        ncsets: i32,
        coeffs: &[f64],
        epochs: &[f64],
    ) -> Result<()> {
        raw::spk14a(self, handle, ncsets, coeffs, epochs)?;
        Ok(())
    }

    /// SPK type 14: Begin a segment.
    ///
    /// Begin a type 14 SPK segment in the SPK file associated with
    /// HANDLE.
    ///
    /// See [`spk14b`](raw::spk14b) for full documentation.
    pub fn spk14b(
        &mut self,
        handle: i32,
        segid: &str,
        body: i32,
        center: i32,
        frame: &str,
        first: f64,
        last: f64,
        chbdeg: i32,
    ) -> Result<()> {
        raw::spk14b(
            self, handle, segid, body, center, frame, first, last, chbdeg,
        )?;
        Ok(())
    }

    /// SPK type 14: End a segment.
    ///
    /// End the type 14 SPK segment currently being written to the SPK
    /// file associated with HANDLE.
    ///
    /// See [`spk14e`](raw::spk14e) for full documentation.
    pub fn spk14e(&mut self, handle: i32) -> Result<()> {
        raw::spk14e(self, handle)?;
        Ok(())
    }

    /// SPK, aberration corrected state
    ///
    /// Return the state (position and velocity) of a target body
    /// relative to an observer, optionally corrected for light time
    /// and stellar aberration, expressed relative to an inertial
    /// reference frame.
    ///
    /// Returns `(starg, lt, dlt)`.
    ///
    /// See [`spkacs`](raw::spkacs) for full documentation.
    pub fn spkacs(
        &mut self,
        targ: i32,
        et: f64,
        ref_: &str,
        abcorr: &str,
        obs: i32,
    ) -> Result<([f64; 6], f64, f64)> {
        let mut starg: [f64; 6] = Default::default();
        let mut lt: f64 = Default::default();
        let mut dlt: f64 = Default::default();
        raw::spkacs(
            self, targ, et, ref_, abcorr, obs, &mut starg, &mut lt, &mut dlt,
        )?;
        Ok((starg, lt, dlt))
    }

    /// S/P Kernel, apparent position only
    ///
    /// Return the position of a target body relative to an observer,
    /// optionally corrected for light time and stellar aberration.
    ///
    /// Returns `(ptarg, lt)`.
    ///
    /// See [`spkapo`](raw::spkapo) for full documentation.
    pub fn spkapo(
        &mut self,
        targ: i32,
        et: f64,
        ref_: &str,
        sobs: &[f64; 6],
        abcorr: &str,
    ) -> Result<([f64; 3], f64)> {
        let mut ptarg: [f64; 3] = Default::default();
        let mut lt: f64 = Default::default();
        raw::spkapo(self, targ, et, ref_, sobs, abcorr, &mut ptarg, &mut lt)?;
        Ok((ptarg, lt))
    }

    /// SPK, apparent state
    ///
    /// Return the state (position and velocity) of a target body
    /// relative to an observer specified by its state and
    /// acceleration relative to the solar system barycenter. The
    /// returned state may be optionally corrected for light time
    /// and stellar aberration. All input and output vectors are
    /// expressed relative to an inertial reference frame.
    ///
    /// This routine supersedes SPKAPP.
    ///
    /// SPICE users normally should call the high-level API routines
    /// SPKEZR or SPKEZ rather than this routine.
    ///
    /// Returns `(starg, lt, dlt)`.
    ///
    /// See [`spkaps`](raw::spkaps) for full documentation.
    pub fn spkaps(
        &mut self,
        targ: i32,
        et: f64,
        ref_: &str,
        abcorr: &str,
        stobs: &[f64; 6],
        accobs: &[f64; 3],
    ) -> Result<([f64; 6], f64, f64)> {
        let mut starg: [f64; 6] = Default::default();
        let mut lt: f64 = Default::default();
        let mut dlt: f64 = Default::default();
        raw::spkaps(
            self, targ, et, ref_, abcorr, stobs, accobs, &mut starg, &mut lt, &mut dlt,
        )?;
        Ok((starg, lt, dlt))
    }

    /// S/P Kernel, Load ephemeris file
    ///
    /// Load an ephemeris file for use by the readers. Return that file's
    /// handle, to be used by other SPK routines to refer to the file.
    ///
    /// Returns `handle`.
    ///
    /// See [`spklef`](raw::spklef) for full documentation.
    pub fn spklef(&mut self, fname: &str) -> Result<i32> {
        let mut handle: i32 = Default::default();
        raw::spklef(self, fname, &mut handle)?;
        Ok(handle)
    }

    /// SPK Kernel, Unload ephemeris file
    ///
    /// Unload an ephemeris file so that it will no longer be searched by
    /// the readers.
    ///
    /// See [`spkuef`](raw::spkuef) for full documentation.
    pub fn spkuef(&mut self, handle: i32) -> Result<()> {
        raw::spkuef(self, handle)?;
        Ok(())
    }

    /// S/P Kernel, Select file and segment
    ///
    /// Search through loaded SPK files to find the highest-priority
    /// segment applicable to the body and time specified and buffer
    /// searched segments in the process, to attempt to avoid re-reading
    /// files.
    ///
    /// Returns `(handle, descr, ident)`.
    ///
    /// See [`spksfs`](raw::spksfs) for full documentation.
    pub fn spksfs(&mut self, body: i32, et: f64) -> Result<Option<(i32, Vec<f64>, String)>> {
        let mut handle: i32 = Default::default();
        let mut descr: Vec<f64> = vec![Default::default(); 5 as usize];
        let mut ident = blank(40);
        let mut found: bool = Default::default();
        raw::spksfs(
            self,
            body,
            et,
            &mut handle,
            &mut descr,
            &mut ident,
            &mut found,
        )?;
        Ok(if found {
            Some((handle, descr, trim(ident)))
        } else {
            None
        })
    }

    /// SPK, Close file
    ///
    /// Close an open SPK file.
    ///
    /// See [`spkcls`](raw::spkcls) for full documentation.
    pub fn spkcls(&mut self, handle: i32) -> Result<()> {
        raw::spkcls(self, handle)?;
        Ok(())
    }

    /// SPK coverage
    ///
    /// Find the coverage window for a specified ephemeris object in a
    /// specified SPK file.
    ///
    /// See [`spkcov`](raw::spkcov) for full documentation.
    pub fn spkcov(&mut self, spkfnm: &str, idcode: i32, cover: &mut Cell<f64>) -> Result<()> {
        raw::spkcov(self, spkfnm, idcode, cover.as_raw_mut_slice())?;
        Ok(())
    }

    /// SPK, constant position observer state
    ///
    /// Return the state of a specified target relative to an "observer,"
    /// where the observer has constant position in a specified reference
    /// frame. The observer's position is provided by the calling program
    /// rather than by loaded SPK files.
    ///
    /// Returns `(state, lt)`.
    ///
    /// See [`spkcpo`](raw::spkcpo) for full documentation.
    pub fn spkcpo(
        &mut self,
        target: &str,
        et: f64,
        outref: &str,
        refloc: &str,
        abcorr: &str,
        obspos: &[f64; 3],
        obsctr: &str,
        obsref: &str,
    ) -> Result<([f64; 6], f64)> {
        let mut state: [f64; 6] = Default::default();
        let mut lt: f64 = Default::default();
        raw::spkcpo(
            self, target, et, outref, refloc, abcorr, obspos, obsctr, obsref, &mut state, &mut lt,
        )?;
        Ok((state, lt))
    }

    /// SPK, constant position target state
    ///
    /// Return the state, relative to a specified observer, of a target
    /// having constant position in a specified reference frame. The
    /// target's position is provided by the calling program rather than
    /// by loaded SPK files.
    ///
    /// Returns `(state, lt)`.
    ///
    /// See [`spkcpt`](raw::spkcpt) for full documentation.
    pub fn spkcpt(
        &mut self,
        trgpos: &[f64; 3],
        trgctr: &str,
        trgref: &str,
        et: f64,
        outref: &str,
        refloc: &str,
        abcorr: &str,
        obsrvr: &str,
    ) -> Result<([f64; 6], f64)> {
        let mut state: [f64; 6] = Default::default();
        let mut lt: f64 = Default::default();
        raw::spkcpt(
            self, trgpos, trgctr, trgref, et, outref, refloc, abcorr, obsrvr, &mut state, &mut lt,
        )?;
        Ok((state, lt))
    }

    /// SPK, constant velocity observer state
    ///
    /// Return the state of a specified target relative to an "observer,"
    /// where the observer has constant velocity in a specified reference
    /// frame. The observer's state is provided by the calling program
    /// rather than by loaded SPK files.
    ///
    /// Returns `(state, lt)`.
    ///
    /// See [`spkcvo`](raw::spkcvo) for full documentation.
    pub fn spkcvo(
        &mut self,
        target: &str,
        et: f64,
        outref: &str,
        refloc: &str,
        abcorr: &str,
        obssta: &[f64; 6],
        obsepc: f64,
        obsctr: &str,
        obsref: &str,
    ) -> Result<([f64; 6], f64)> {
        let mut state: [f64; 6] = Default::default();
        let mut lt: f64 = Default::default();
        raw::spkcvo(
            self, target, et, outref, refloc, abcorr, obssta, obsepc, obsctr, obsref, &mut state,
            &mut lt,
        )?;
        Ok((state, lt))
    }

    /// SPK, constant velocity target state
    ///
    /// Return the state, relative to a specified observer, of a target
    /// having constant velocity in a specified reference frame. The
    /// target's state is provided by the calling program rather than by
    /// loaded SPK files.
    ///
    /// Returns `(state, lt)`.
    ///
    /// See [`spkcvt`](raw::spkcvt) for full documentation.
    pub fn spkcvt(
        &mut self,
        trgsta: &[f64; 6],
        trgepc: f64,
        trgctr: &str,
        trgref: &str,
        et: f64,
        outref: &str,
        refloc: &str,
        abcorr: &str,
        obsrvr: &str,
    ) -> Result<([f64; 6], f64)> {
        let mut state: [f64; 6] = Default::default();
        let mut lt: f64 = Default::default();
        raw::spkcvt(
            self, trgsta, trgepc, trgctr, trgref, et, outref, refloc, abcorr, obsrvr, &mut state,
            &mut lt,
        )?;
        Ok((state, lt))
    }

    /// S/P Kernel, evaluate, type 1
    ///
    /// Evaluate a single SPK data record from a segment of type 1
    /// (Difference Lines).
    ///
    /// Returns `state`.
    ///
    /// See [`spke01`](raw::spke01) for full documentation.
    pub fn spke01(&mut self, et: f64, record: &[f64]) -> [f64; 6] {
        let mut state: [f64; 6] = Default::default();
        raw::spke01(self, et, record, &mut state);
        state
    }

    /// SPK, evaluate record, type 2
    ///
    /// Evaluate a single data record from an PCK or SPK segment of type
    /// 2 (Chebyshev Polynomials, 3 components).
    ///
    /// Returns `xyzdot`.
    ///
    /// See [`spke02`](raw::spke02) for full documentation.
    pub fn spke02(&mut self, et: f64, record: &[f64]) -> Result<[f64; 6]> {
        let mut xyzdot: [f64; 6] = Default::default();
        raw::spke02(self, et, record, &mut xyzdot)?;
        Ok(xyzdot)
    }

    /// S/P Kernel, evaluate, type 3
    ///
    /// Evaluate a single SPK data record from a segment of type 3
    /// (Chebyshev Polynomials, position and velocity).
    ///
    /// Returns `state`.
    ///
    /// See [`spke03`](raw::spke03) for full documentation.
    pub fn spke03(&mut self, et: f64, record: &[f64]) -> Result<[f64; 6]> {
        let mut state: [f64; 6] = Default::default();
        raw::spke03(self, et, record, &mut state)?;
        Ok(state)
    }

    /// Evaluate SPK record, type 5
    ///
    /// Evaluate a single SPK data record from a segment of type 5
    /// (two body propagation between discrete state vectors).
    ///
    /// Returns `state`.
    ///
    /// See [`spke05`](raw::spke05) for full documentation.
    pub fn spke05(&mut self, et: f64, record: &[f64]) -> Result<[f64; 6]> {
        let mut state: [f64; 6] = Default::default();
        raw::spke05(self, et, record, &mut state)?;
        Ok(state)
    }

    /// S/P Kernel, evaluate, type 8
    ///
    /// Evaluate a single SPK data record from a segment of type 8
    /// (equally spaced discrete states, interpolated by Lagrange
    /// polynomials).
    ///
    /// Returns `state`.
    ///
    /// See [`spke08`](raw::spke08) for full documentation.
    pub fn spke08(&mut self, et: f64, record: &[f64]) -> Result<[f64; 6]> {
        let mut state: [f64; 6] = Default::default();
        raw::spke08(self, et, record, &mut state)?;
        Ok(state)
    }

    /// S/P Kernel, evaluate, type 9
    ///
    /// Evaluate a single SPK data record from a segment of type 9
    /// (discrete states, evaluated by Lagrange interpolation).
    ///
    /// Returns `state`.
    ///
    /// See [`spke09`](raw::spke09) for full documentation.
    pub fn spke09(&mut self, et: f64, record: &mut [f64]) -> Result<[f64; 6]> {
        let mut state: [f64; 6] = Default::default();
        raw::spke09(self, et, record, &mut state)?;
        Ok(state)
    }

    /// Evaluate SPK record, type 10
    ///
    /// Evaluate a single SPK data record from a segment of type 10
    /// (NORAD two-line element sets.). This evaluator uses algorithms
    /// as described in Vallado 2006 \[4].
    ///
    /// Returns `state`.
    ///
    /// See [`spke10`](raw::spke10) for full documentation.
    pub fn spke10(&mut self, et: f64, record: &[f64]) -> Result<[f64; 6]> {
        let mut state: [f64; 6] = Default::default();
        raw::spke10(self, et, record, &mut state)?;
        Ok(state)
    }

    /// S/P Kernel, evaluate, type 12
    ///
    /// Evaluate a single data record from a type 12 SPK segment.
    ///
    /// Returns `state`.
    ///
    /// See [`spke12`](raw::spke12) for full documentation.
    pub fn spke12(&mut self, et: f64, record: &[f64]) -> Result<[f64; 6]> {
        let mut state: [f64; 6] = Default::default();
        raw::spke12(self, et, record, &mut state)?;
        Ok(state)
    }

    /// S/P Kernel, evaluate, type 13
    ///
    /// Evaluate a single data record from a type 13 SPK segment.
    ///
    /// Returns `state`.
    ///
    /// See [`spke13`](raw::spke13) for full documentation.
    pub fn spke13(&mut self, et: f64, record: &[f64]) -> Result<[f64; 6]> {
        let mut state: [f64; 6] = Default::default();
        raw::spke13(self, et, record, &mut state)?;
        Ok(state)
    }

    /// S/P Kernel, evaluate, type 14
    ///
    /// Evaluate a single data record from a type 14 SPK segment.
    ///
    /// Returns `state`.
    ///
    /// See [`spke14`](raw::spke14) for full documentation.
    pub fn spke14(&mut self, et: f64, record: &[f64]) -> Result<[f64; 6]> {
        let mut state: [f64; 6] = Default::default();
        raw::spke14(self, et, record, &mut state)?;
        Ok(state)
    }

    /// Evaluate a type 15 SPK data record
    ///
    /// Evaluate a single SPK data record from a segment of type 15
    /// (Precessing Conic Propagation).
    ///
    /// Returns `state`.
    ///
    /// See [`spke15`](raw::spke15) for full documentation.
    pub fn spke15(&mut self, et: f64, recin: &[f64]) -> Result<[f64; 6]> {
        let mut state: [f64; 6] = Default::default();
        raw::spke15(self, et, recin, &mut state)?;
        Ok(state)
    }

    /// Evaluate a type 17 SPK data record
    ///
    /// Evaluate a single SPK data record from a segment of type 17
    /// (Equinoctial Elements).
    ///
    /// Returns `state`.
    ///
    /// See [`spke17`](raw::spke17) for full documentation.
    pub fn spke17(&mut self, et: f64, recin: &[f64]) -> Result<[f64; 6]> {
        let mut state: [f64; 6] = Default::default();
        raw::spke17(self, et, recin, &mut state)?;
        Ok(state)
    }

    /// S/P Kernel, evaluate, type 18
    ///
    /// Evaluate a single data record from a type 18 SPK segment.
    ///
    /// Returns `state`.
    ///
    /// See [`spke18`](raw::spke18) for full documentation.
    pub fn spke18(&mut self, et: f64, record: &mut [f64]) -> Result<[f64; 6]> {
        let mut state: [f64; 6] = Default::default();
        raw::spke18(self, et, record, &mut state)?;
        Ok(state)
    }

    /// SPK, evaluate record, type 19
    ///
    /// Evaluate a single data record from a type 19 SPK segment.
    ///
    /// Returns `state`.
    ///
    /// See [`spke19`](raw::spke19) for full documentation.
    pub fn spke19(&mut self, et: f64, record: &mut [f64]) -> Result<[f64; 6]> {
        let mut state: [f64; 6] = Default::default();
        raw::spke19(self, et, record, &mut state)?;
        Ok(state)
    }

    /// SPK, evaluate Chebyshev polynomials, type 20
    ///
    /// Evaluate a single data record from an SPK or PCK segment of type
    /// 20 (Chebyshev polynomials, velocity only).
    ///
    /// Returns `xyzdot`.
    ///
    /// See [`spke20`](raw::spke20) for full documentation.
    pub fn spke20(&mut self, et: f64, record: &[f64]) -> Result<[f64; 6]> {
        let mut xyzdot: [f64; 6] = Default::default();
        raw::spke20(self, et, record, &mut xyzdot)?;
        Ok(xyzdot)
    }

    /// S/P Kernel, evaluate, type 21
    ///
    /// Evaluate a single SPK data record from a segment of type 21
    /// (Extended Difference Lines).
    ///
    /// Returns `state`.
    ///
    /// See [`spke21`](raw::spke21) for full documentation.
    pub fn spke21(&mut self, et: f64, record: &[f64]) -> Result<[f64; 6]> {
        let mut state: [f64; 6] = Default::default();
        raw::spke21(self, et, record, &mut state)?;
        Ok(state)
    }

    /// S/P Kernel, easy reader
    ///
    /// Return the state (position and velocity) of a target body
    /// relative to an observing body, optionally corrected for light
    /// time (planetary aberration) and stellar aberration.
    ///
    /// Returns `(starg, lt)`.
    ///
    /// See [`spkez`](raw::spkez) for full documentation.
    pub fn spkez(
        &mut self,
        targ: i32,
        et: f64,
        ref_: &str,
        abcorr: &str,
        obs: i32,
    ) -> Result<([f64; 6], f64)> {
        let mut starg: [f64; 6] = Default::default();
        let mut lt: f64 = Default::default();
        raw::spkez(self, targ, et, ref_, abcorr, obs, &mut starg, &mut lt)?;
        Ok((starg, lt))
    }

    /// S/P Kernel, easy position
    ///
    /// Return the position of a target body relative to an observing
    /// body, optionally corrected for light time (planetary aberration)
    /// and stellar aberration.
    ///
    /// Returns `(ptarg, lt)`.
    ///
    /// See [`spkezp`](raw::spkezp) for full documentation.
    pub fn spkezp(
        &mut self,
        targ: i32,
        et: f64,
        ref_: &str,
        abcorr: &str,
        obs: i32,
    ) -> Result<([f64; 3], f64)> {
        let mut ptarg: [f64; 3] = Default::default();
        let mut lt: f64 = Default::default();
        raw::spkezp(self, targ, et, ref_, abcorr, obs, &mut ptarg, &mut lt)?;
        Ok((ptarg, lt))
    }

    /// S/P Kernel, easier reader
    ///
    /// Return the state (position and velocity) of a target body
    /// relative to an observing body, optionally corrected for light
    /// time (planetary aberration) and stellar aberration.
    ///
    /// Returns `(starg, lt)`.
    ///
    /// See [`spkezr`](raw::spkezr) for full documentation.
    pub fn spkezr(
        &mut self,
        targ: &str,
        et: f64,
        ref_: &str,
        abcorr: &str,
        obs: &str,
    ) -> Result<([f64; 6], f64)> {
        let mut starg: [f64; 6] = Default::default();
        let mut lt: f64 = Default::default();
        raw::spkezr(self, targ, et, ref_, abcorr, obs, &mut starg, &mut lt)?;
        Ok((starg, lt))
    }

    /// S/P Kernel, geometric state
    ///
    /// Compute the geometric state (position and velocity) of a target
    /// body relative to an observing body.
    ///
    /// Returns `(state, lt)`.
    ///
    /// See [`spkgeo`](raw::spkgeo) for full documentation.
    pub fn spkgeo(&mut self, targ: i32, et: f64, ref_: &str, obs: i32) -> Result<([f64; 6], f64)> {
        let mut state: [f64; 6] = Default::default();
        let mut lt: f64 = Default::default();
        raw::spkgeo(self, targ, et, ref_, obs, &mut state, &mut lt)?;
        Ok((state, lt))
    }

    /// S/P Kernel, geometric position
    ///
    /// Compute the geometric position of a target body relative to an
    /// observing body.
    ///
    /// Returns `(pos, lt)`.
    ///
    /// See [`spkgps`](raw::spkgps) for full documentation.
    pub fn spkgps(&mut self, targ: i32, et: f64, ref_: &str, obs: i32) -> Result<([f64; 3], f64)> {
        let mut pos: [f64; 3] = Default::default();
        let mut lt: f64 = Default::default();
        raw::spkgps(self, targ, et, ref_, obs, &mut pos, &mut lt)?;
        Ok((pos, lt))
    }

    /// S/P Kernel, light time corrected state
    ///
    /// Return the state (position and velocity) of a target body
    /// relative to an observer, optionally corrected for light time,
    /// expressed relative to an inertial reference frame.
    ///
    /// Returns `(starg, lt, dlt)`.
    ///
    /// See [`spkltc`](raw::spkltc) for full documentation.
    pub fn spkltc(
        &mut self,
        targ: i32,
        et: f64,
        ref_: &str,
        abcorr: &str,
        stobs: &[f64; 6],
    ) -> Result<([f64; 6], f64, f64)> {
        let mut starg: [f64; 6] = Default::default();
        let mut lt: f64 = Default::default();
        let mut dlt: f64 = Default::default();
        raw::spkltc(
            self, targ, et, ref_, abcorr, stobs, &mut starg, &mut lt, &mut dlt,
        )?;
        Ok((starg, lt, dlt))
    }

    /// SPK objects
    ///
    /// Find the set of ID codes of all objects in a specified SPK file.
    ///
    /// See [`spkobj`](raw::spkobj) for full documentation.
    pub fn spkobj(&mut self, spkfnm: &str, ids: &mut Cell<i32>) -> Result<()> {
        raw::spkobj(self, spkfnm, ids.as_raw_mut_slice())?;
        Ok(())
    }

    /// SPK open for addition
    ///
    /// Open an existing SPK file for subsequent write.
    ///
    /// Returns `handle`.
    ///
    /// See [`spkopa`](raw::spkopa) for full documentation.
    pub fn spkopa(&mut self, file: &str) -> Result<i32> {
        let mut handle: i32 = Default::default();
        raw::spkopa(self, file, &mut handle)?;
        Ok(handle)
    }

    /// SPK, open new file.
    ///
    /// Create a new SPK file, returning the handle of the opened file.
    ///
    /// Returns `handle`.
    ///
    /// See [`spkopn`](raw::spkopn) for full documentation.
    pub fn spkopn(&mut self, fname: &str, ifname: &str, ncomch: i32) -> Result<i32> {
        let mut handle: i32 = Default::default();
        raw::spkopn(self, fname, ifname, ncomch, &mut handle)?;
        Ok(handle)
    }

    /// SPK pack descriptor
    ///
    /// Perform routine error checks and if all check pass, pack the
    /// descriptor for an SPK segment
    ///
    /// Returns `descr`.
    ///
    /// See [`spkpds`](raw::spkpds) for full documentation.
    pub fn spkpds(
        &mut self,
        body: i32,
        center: i32,
        frame: &str,
        type_: i32,
        first: f64,
        last: f64,
    ) -> Result<Vec<f64>> {
        let mut descr: Vec<f64> = vec![Default::default(); 5 as usize];
        raw::spkpds(self, body, center, frame, type_, first, last, &mut descr)?;
        Ok(descr)
    }

    /// S/P Kernel, position
    ///
    /// Return the position of a target body relative to an observing
    /// body, optionally corrected for light time (planetary aberration)
    /// and stellar aberration.
    ///
    /// Returns `(ptarg, lt)`.
    ///
    /// See [`spkpos`](raw::spkpos) for full documentation.
    pub fn spkpos(
        &mut self,
        targ: &str,
        et: f64,
        ref_: &str,
        abcorr: &str,
        obs: &str,
    ) -> Result<([f64; 3], f64)> {
        let mut ptarg: [f64; 3] = Default::default();
        let mut lt: f64 = Default::default();
        raw::spkpos(self, targ, et, ref_, abcorr, obs, &mut ptarg, &mut lt)?;
        Ok((ptarg, lt))
    }

    /// S/P Kernel, position and velocity
    ///
    /// Return the state (position and velocity) of a target body
    /// relative to some center of motion in a specified frame.
    ///
    /// Returns `(state, center)`.
    ///
    /// See [`spkpv`](raw::spkpv) for full documentation.
    pub fn spkpv(
        &mut self,
        handle: i32,
        descr: &[f64; 5],
        et: f64,
        ref_: &str,
    ) -> Result<([f64; 6], i32)> {
        let mut state: [f64; 6] = Default::default();
        let mut center: i32 = Default::default();
        raw::spkpv(self, handle, descr, et, ref_, &mut state, &mut center)?;
        Ok((state, center))
    }

    /// S/P Kernel, position and velocity in native frame
    ///
    /// Return, for a specified SPK segment and time, the state (position
    /// and velocity) of the segment's target body relative to its center
    /// of motion.
    ///
    /// Returns `(ref_, state, center)`.
    ///
    /// See [`spkpvn`](raw::spkpvn) for full documentation.
    pub fn spkpvn(
        &mut self,
        handle: i32,
        descr: &[f64; 5],
        et: f64,
    ) -> Result<(i32, [f64; 6], i32)> {
        let mut ref_: i32 = Default::default();
        let mut state: [f64; 6] = Default::default();
        let mut center: i32 = Default::default();
        raw::spkpvn(self, handle, descr, et, &mut ref_, &mut state, &mut center)?;
        Ok((ref_, state, center))
    }

    /// Read SPK record from segment, type 1
    ///
    /// Read a single SPK data record from a segment of type 1
    /// (Difference Lines).
    ///
    /// Returns `record`.
    ///
    /// See [`spkr01`](raw::spkr01) for full documentation.
    pub fn spkr01(&mut self, handle: i32, descr: &[f64; 5], et: f64) -> Result<Vec<f64>> {
        let mut record: Vec<f64> = vec![Default::default(); 128 as usize];
        raw::spkr01(self, handle, descr, et, &mut record)?;
        Ok(record)
    }

    /// SPK, read record from segment, type 2
    ///
    /// Read a single SPK data record from a segment of type 2
    /// (Chebyshev, position only).
    ///
    /// Returns `record`.
    ///
    /// See [`spkr02`](raw::spkr02) for full documentation.
    pub fn spkr02(&mut self, handle: i32, descr: &[f64; 5], et: f64) -> Result<Vec<f64>> {
        let mut record: Vec<f64> = vec![Default::default(); 128 as usize];
        raw::spkr02(self, handle, descr, et, &mut record)?;
        Ok(record)
    }

    /// SPK, read record from segment, type 3
    ///
    /// Read a single SPK data record from a segment of type 3
    /// (Chebyshev coefficients, position and velocity).
    ///
    /// Returns `record`.
    ///
    /// See [`spkr03`](raw::spkr03) for full documentation.
    pub fn spkr03(&mut self, handle: i32, descr: &[f64; 5], et: f64) -> Result<Vec<f64>> {
        let mut record: Vec<f64> = vec![Default::default(); 128 as usize];
        raw::spkr03(self, handle, descr, et, &mut record)?;
        Ok(record)
    }

    /// Read SPK record from segment, type 5
    ///
    /// Read a single SPK data record from a segment of type 5
    /// ( two body propagation between discrete state vectors ).
    ///
    /// Returns `record`.
    ///
    /// See [`spkr05`](raw::spkr05) for full documentation.
    pub fn spkr05(&mut self, handle: i32, descr: &[f64; 5], et: f64) -> Result<Vec<f64>> {
        let mut record: Vec<f64> = vec![Default::default(); 128 as usize];
        raw::spkr05(self, handle, descr, et, &mut record)?;
        Ok(record)
    }

    /// Read SPK record from segment, type 8
    ///
    /// Read a single SPK data record from a segment of type 8
    /// (equally spaced discrete states, interpolated by Lagrange
    /// polynomials).
    ///
    /// Returns `record`.
    ///
    /// See [`spkr08`](raw::spkr08) for full documentation.
    pub fn spkr08(&mut self, handle: i32, descr: &[f64; 5], et: f64) -> Result<Vec<f64>> {
        let mut record: Vec<f64> = vec![Default::default(); 128 as usize];
        raw::spkr08(self, handle, descr, et, &mut record)?;
        Ok(record)
    }

    /// Read SPK record from segment, type 9
    ///
    /// Read a single SPK data record from a segment of type 9
    /// (Unequally spaced discrete states, interpolated by Lagrange
    /// polynomials).
    ///
    /// Returns `record`.
    ///
    /// See [`spkr09`](raw::spkr09) for full documentation.
    pub fn spkr09(&mut self, handle: i32, descr: &[f64; 5], et: f64) -> Result<Vec<f64>> {
        let mut record: Vec<f64> = vec![Default::default(); 128 as usize];
        raw::spkr09(self, handle, descr, et, &mut record)?;
        Ok(record)
    }

    /// SPK, read record from SPK type 10 segment
    ///
    /// Read a single SPK data record from a segment of type 10
    /// (NORAD two line element sets).
    ///
    /// Returns `record`.
    ///
    /// See [`spkr10`](raw::spkr10) for full documentation.
    pub fn spkr10(&mut self, handle: i32, descr: &[f64; 5], et: f64) -> Result<Vec<f64>> {
        let mut record: Vec<f64> = vec![Default::default(); 128 as usize];
        raw::spkr10(self, handle, descr, et, &mut record)?;
        Ok(record)
    }

    /// Read SPK record from segment, type 12
    ///
    /// Read a single data record from a type 12 SPK segment.
    ///
    /// Returns `record`.
    ///
    /// See [`spkr12`](raw::spkr12) for full documentation.
    pub fn spkr12(&mut self, handle: i32, descr: &[f64], et: f64) -> Result<Vec<f64>> {
        let mut record: Vec<f64> = vec![Default::default(); 128 as usize];
        raw::spkr12(self, handle, descr, et, &mut record)?;
        Ok(record)
    }

    /// Read SPK record from segment, type 13
    ///
    /// Read a single data record from a type 13 SPK segment.
    ///
    /// Returns `record`.
    ///
    /// See [`spkr13`](raw::spkr13) for full documentation.
    pub fn spkr13(&mut self, handle: i32, descr: &[f64], et: f64) -> Result<Vec<f64>> {
        let mut record: Vec<f64> = vec![Default::default(); 128 as usize];
        raw::spkr13(self, handle, descr, et, &mut record)?;
        Ok(record)
    }

    /// Read SPK record from segment, type 14
    ///
    /// Read a single data record from a type 14 SPK segment.
    ///
    /// Returns `record`.
    ///
    /// See [`spkr14`](raw::spkr14) for full documentation.
    pub fn spkr14(&mut self, handle: i32, descr: &[f64], et: f64) -> Result<Vec<f64>> {
        let mut record: Vec<f64> = vec![Default::default(); 128 as usize];
        raw::spkr14(self, handle, descr, et, &mut record)?;
        Ok(record)
    }

    /// Read SPK record from segment, type 15
    ///
    /// Read a single SPK data record from a segment of type 15
    /// (Precessing Conic Propagation).
    ///
    /// Returns `record`.
    ///
    /// See [`spkr15`](raw::spkr15) for full documentation.
    pub fn spkr15(&mut self, handle: i32, descr: &[f64; 5], et: f64) -> Result<Vec<f64>> {
        let mut record: Vec<f64> = vec![Default::default(); 128 as usize];
        raw::spkr15(self, handle, descr, et, &mut record)?;
        Ok(record)
    }

    /// Read SPK record from segment, type 17
    ///
    /// Read a single SPK data record from a segment of type 17
    /// (Equinoctial Elements).
    ///
    /// Returns `record`.
    ///
    /// See [`spkr17`](raw::spkr17) for full documentation.
    pub fn spkr17(&mut self, handle: i32, descr: &[f64; 5], et: f64) -> Result<Vec<f64>> {
        let mut record: Vec<f64> = vec![Default::default(); 128 as usize];
        raw::spkr17(self, handle, descr, et, &mut record)?;
        Ok(record)
    }

    /// Read SPK record from segment, type 18
    ///
    /// Read a single SPK data record from a segment of type 18
    /// (MEX/Rosetta Orbit file interpolation).
    ///
    /// Returns `record`.
    ///
    /// See [`spkr18`](raw::spkr18) for full documentation.
    pub fn spkr18(&mut self, handle: i32, descr: &[f64; 5], et: f64) -> Result<Vec<f64>> {
        let mut record: Vec<f64> = vec![Default::default(); 128 as usize];
        raw::spkr18(self, handle, descr, et, &mut record)?;
        Ok(record)
    }

    /// SPK, read record from segment, type 19
    ///
    /// Read a single SPK data record from a segment of type 19
    /// (ESOC/DDID Piecewise Interpolation).
    ///
    /// Returns `record`.
    ///
    /// See [`spkr19`](raw::spkr19) for full documentation.
    pub fn spkr19(&mut self, handle: i32, descr: &[f64; 5], et: f64) -> Result<Vec<f64>> {
        let mut record: Vec<f64> = vec![Default::default(); 128 as usize];
        raw::spkr19(self, handle, descr, et, &mut record)?;
        Ok(record)
    }

    /// SPK, read record from segment, type 20
    ///
    /// Read a single SPK data record from a segment of type 20
    /// (Chebyshev, velocity coefficients only).
    ///
    /// Returns `record`.
    ///
    /// See [`spkr20`](raw::spkr20) for full documentation.
    pub fn spkr20(&mut self, handle: i32, descr: &[f64; 5], et: f64) -> Result<Vec<f64>> {
        let mut record: Vec<f64> = vec![Default::default(); 128 as usize];
        raw::spkr20(self, handle, descr, et, &mut record)?;
        Ok(record)
    }

    /// Read SPK record from segment, type 21
    ///
    /// Read a single SPK data record from a segment of type 21
    /// (Extended Difference Lines).
    ///
    /// Returns `record`.
    ///
    /// See [`spkr21`](raw::spkr21) for full documentation.
    pub fn spkr21(&mut self, handle: i32, descr: &[f64; 5], et: f64) -> Result<Vec<f64>> {
        let mut record: Vec<f64> = vec![Default::default(); 128 as usize];
        raw::spkr21(self, handle, descr, et, &mut record)?;
        Ok(record)
    }

    /// S/P Kernel, subset, type 1
    ///
    /// Extract a subset of the data in a SPK segment of type 1
    /// into a new segment.
    ///
    /// See [`spks01`](raw::spks01) for full documentation.
    pub fn spks01(
        &mut self,
        handle: i32,
        baddr: i32,
        eaddr: i32,
        begin: f64,
        end: f64,
    ) -> Result<()> {
        raw::spks01(self, handle, baddr, eaddr, begin, end)?;
        Ok(())
    }

    /// S/P Kernel, subset, type 2
    ///
    /// Extract a subset of the data in a SPK segment of type 2
    /// into a new segment.
    ///
    /// See [`spks02`](raw::spks02) for full documentation.
    pub fn spks02(
        &mut self,
        handle: i32,
        baddr: i32,
        eaddr: i32,
        begin: f64,
        end: f64,
    ) -> Result<()> {
        raw::spks02(self, handle, baddr, eaddr, begin, end)?;
        Ok(())
    }

    /// S/P Kernel, subset, type 3
    ///
    /// Extract a subset of the data in a SPK segment of type 3 (Chebyshev
    /// polynomials, position and velocity) into a new segment.
    ///
    /// See [`spks03`](raw::spks03) for full documentation.
    pub fn spks03(
        &mut self,
        handle: i32,
        baddr: i32,
        eaddr: i32,
        begin: f64,
        end: f64,
    ) -> Result<()> {
        raw::spks03(self, handle, baddr, eaddr, begin, end)?;
        Ok(())
    }

    /// S/P Kernel, subset, type 5
    ///
    /// Extract a subset of the data in an SPK segment of type 5
    /// into a new segment.
    ///
    /// See [`spks05`](raw::spks05) for full documentation.
    pub fn spks05(
        &mut self,
        handle: i32,
        baddr: i32,
        eaddr: i32,
        begin: f64,
        end: f64,
    ) -> Result<()> {
        raw::spks05(self, handle, baddr, eaddr, begin, end)?;
        Ok(())
    }

    /// S/P Kernel, subset, type 8
    ///
    /// Extract a subset of the data in an SPK segment of type 8
    /// into a new segment.
    ///
    /// See [`spks08`](raw::spks08) for full documentation.
    pub fn spks08(
        &mut self,
        handle: i32,
        baddr: i32,
        eaddr: i32,
        begin: f64,
        end: f64,
    ) -> Result<()> {
        raw::spks08(self, handle, baddr, eaddr, begin, end)?;
        Ok(())
    }

    /// S/P Kernel, subset, type 9
    ///
    /// Extract a subset of the data in an SPK segment of type 9
    /// into a new segment.
    ///
    /// See [`spks09`](raw::spks09) for full documentation.
    pub fn spks09(
        &mut self,
        handle: i32,
        baddr: i32,
        eaddr: i32,
        begin: f64,
        end: f64,
    ) -> Result<()> {
        raw::spks09(self, handle, baddr, eaddr, begin, end)?;
        Ok(())
    }

    /// S/P Kernel, subset, type 10
    ///
    /// Extract a subset of the data in a type 10 SPK segment into a new
    /// type 10 segment.
    ///
    /// See [`spks10`](raw::spks10) for full documentation.
    pub fn spks10(
        &mut self,
        srchan: i32,
        srcdsc: &[f64],
        dsthan: i32,
        dstdsc: &[f64],
        dstsid: &str,
    ) -> Result<()> {
        raw::spks10(self, srchan, srcdsc, dsthan, dstdsc, dstsid)?;
        Ok(())
    }

    /// S/P Kernel, subset, type 12
    ///
    /// Extract a subset of the data in an SPK segment of type 12
    /// into a new segment.
    ///
    /// See [`spks12`](raw::spks12) for full documentation.
    pub fn spks12(
        &mut self,
        handle: i32,
        baddr: i32,
        eaddr: i32,
        begin: f64,
        end: f64,
    ) -> Result<()> {
        raw::spks12(self, handle, baddr, eaddr, begin, end)?;
        Ok(())
    }

    /// S/P Kernel, subset, type 13
    ///
    /// Extract a subset of the data in an SPK segment of type 13
    /// into a new segment.
    ///
    /// See [`spks13`](raw::spks13) for full documentation.
    pub fn spks13(
        &mut self,
        handle: i32,
        baddr: i32,
        eaddr: i32,
        begin: f64,
        end: f64,
    ) -> Result<()> {
        raw::spks13(self, handle, baddr, eaddr, begin, end)?;
        Ok(())
    }

    /// S/P Kernel, subset, type 14
    ///
    /// Extract a subset of the data in a type 14 SPK segment into a new
    /// type 14 segment.
    ///
    /// See [`spks14`](raw::spks14) for full documentation.
    pub fn spks14(
        &mut self,
        srchan: i32,
        srcdsc: &[f64],
        dsthan: i32,
        dstdsc: &[f64],
        dstsid: &str,
    ) -> Result<()> {
        raw::spks14(self, srchan, srcdsc, dsthan, dstdsc, dstsid)?;
        Ok(())
    }

    /// S/P Kernel, subset, type 15
    ///
    /// Extract a subset of the data in an SPK segment of type 15
    /// into a new segment.
    ///
    /// See [`spks15`](raw::spks15) for full documentation.
    pub fn spks15(
        &mut self,
        handle: i32,
        baddr: i32,
        eaddr: i32,
        begin: f64,
        end: f64,
    ) -> Result<()> {
        raw::spks15(self, handle, baddr, eaddr, begin, end)?;
        Ok(())
    }

    /// S/P Kernel, subset, type 17
    ///
    /// Extract a subset of the data in an SPK segment of type 17
    /// into a new segment.
    ///
    /// See [`spks17`](raw::spks17) for full documentation.
    pub fn spks17(
        &mut self,
        handle: i32,
        baddr: i32,
        eaddr: i32,
        begin: f64,
        end: f64,
    ) -> Result<()> {
        raw::spks17(self, handle, baddr, eaddr, begin, end)?;
        Ok(())
    }

    /// S/P Kernel, subset, type 18
    ///
    /// Extract a subset of the data in an SPK segment of type 18
    /// into a new segment.
    ///
    /// See [`spks18`](raw::spks18) for full documentation.
    pub fn spks18(
        &mut self,
        handle: i32,
        baddr: i32,
        eaddr: i32,
        begin: f64,
        end: f64,
    ) -> Result<()> {
        raw::spks18(self, handle, baddr, eaddr, begin, end)?;
        Ok(())
    }

    /// S/P Kernel, subset, type 19
    ///
    /// Extract a subset of the data in an SPK segment of type 19
    /// into a new segment.
    ///
    /// See [`spks19`](raw::spks19) for full documentation.
    pub fn spks19(
        &mut self,
        handle: i32,
        baddr: i32,
        eaddr: i32,
        begin: f64,
        end: f64,
    ) -> Result<()> {
        raw::spks19(self, handle, baddr, eaddr, begin, end)?;
        Ok(())
    }

    /// S/P Kernel, subset, type 20
    ///
    /// Extract a subset of the data in a SPK segment of type 20
    /// into a new segment.
    ///
    /// See [`spks20`](raw::spks20) for full documentation.
    pub fn spks20(
        &mut self,
        handle: i32,
        baddr: i32,
        eaddr: i32,
        begin: f64,
        end: f64,
    ) -> Result<()> {
        raw::spks20(self, handle, baddr, eaddr, begin, end)?;
        Ok(())
    }

    /// S/P Kernel, subset, type 21
    ///
    /// Extract a subset of the data in a SPK segment of type 21
    /// into a new segment.
    ///
    /// See [`spks21`](raw::spks21) for full documentation.
    pub fn spks21(
        &mut self,
        handle: i32,
        baddr: i32,
        eaddr: i32,
        begin: f64,
        end: f64,
    ) -> Result<()> {
        raw::spks21(self, handle, baddr, eaddr, begin, end)?;
        Ok(())
    }

    /// S/P Kernel, solar system barycenter
    ///
    /// Return the state (position and velocity) of a target body
    /// relative to the solar system barycenter.
    ///
    /// Returns `starg`.
    ///
    /// See [`spkssb`](raw::spkssb) for full documentation.
    pub fn spkssb(&mut self, targ: i32, et: f64, ref_: &str) -> Result<[f64; 6]> {
        let mut starg: [f64; 6] = Default::default();
        raw::spkssb(self, targ, et, ref_, &mut starg)?;
        Ok(starg)
    }

    /// S/P Kernel, subset
    ///
    /// Extract a subset of the data in an SPK segment into a
    /// separate segment.
    ///
    /// See [`spksub`](raw::spksub) for full documentation.
    pub fn spksub(
        &mut self,
        handle: i32,
        descr: &[f64; 5],
        ident: &str,
        begin: f64,
        end: f64,
        newh: i32,
    ) -> Result<()> {
        raw::spksub(self, handle, descr, ident, begin, end, newh)?;
        Ok(())
    }

    /// SPK - unpack segment descriptor
    ///
    /// Unpack the contents of an SPK segment descriptor.
    ///
    /// Returns `(body, center, frame, type_, first, last, baddrs, eaddrs)`.
    ///
    /// See [`spkuds`](raw::spkuds) for full documentation.
    pub fn spkuds(&mut self, descr: &[f64]) -> Result<(i32, i32, i32, i32, f64, f64, i32, i32)> {
        let mut body: i32 = Default::default();
        let mut center: i32 = Default::default();
        let mut frame: i32 = Default::default();
        let mut type_: i32 = Default::default();
        let mut first: f64 = Default::default();
        let mut last: f64 = Default::default();
        let mut baddrs: i32 = Default::default();
        let mut eaddrs: i32 = Default::default();
        raw::spkuds(
            self,
            descr,
            &mut body,
            &mut center,
            &mut frame,
            &mut type_,
            &mut first,
            &mut last,
            &mut baddrs,
            &mut eaddrs,
        )?;
        Ok((body, center, frame, type_, first, last, baddrs, eaddrs))
    }

    /// Write SPK segment, type 1
    ///
    /// Write a type 1 segment to an SPK file.
    ///
    /// See [`spkw01`](raw::spkw01) for full documentation.
    pub fn spkw01(
        &mut self,
        handle: i32,
        body: i32,
        center: i32,
        frame: &str,
        first: f64,
        last: f64,
        segid: &str,
        n: i32,
        dlines: &[[f64; 71]],
        epochs: &[f64],
    ) -> Result<()> {
        raw::spkw01(
            self, handle, body, center, frame, first, last, segid, n, dlines, epochs,
        )?;
        Ok(())
    }

    /// SPK, write segment, type 2
    ///
    /// Write a type 2 segment to an SPK file.
    ///
    /// See [`spkw02`](raw::spkw02) for full documentation.
    pub fn spkw02(
        &mut self,
        handle: i32,
        body: i32,
        center: i32,
        frame: &str,
        first: f64,
        last: f64,
        segid: &str,
        intlen: f64,
        n: i32,
        polydg: i32,
        cdata: &[f64],
        btime: f64,
    ) -> Result<()> {
        raw::spkw02(
            self, handle, body, center, frame, first, last, segid, intlen, n, polydg, cdata, btime,
        )?;
        Ok(())
    }

    /// SPK, write segment, type 3
    ///
    /// Write a type 3 segment to an SPK file.
    ///
    /// See [`spkw03`](raw::spkw03) for full documentation.
    pub fn spkw03(
        &mut self,
        handle: i32,
        body: i32,
        center: i32,
        frame: &str,
        first: f64,
        last: f64,
        segid: &str,
        intlen: f64,
        n: i32,
        polydg: i32,
        cdata: &[f64],
        btime: f64,
    ) -> Result<()> {
        raw::spkw03(
            self, handle, body, center, frame, first, last, segid, intlen, n, polydg, cdata, btime,
        )?;
        Ok(())
    }

    /// Write SPK segment, type 5
    ///
    /// Write an SPK segment of type 5 given a time-ordered set of
    /// discrete states and epochs, and the gravitational parameter
    /// of a central body.
    ///
    /// See [`spkw05`](raw::spkw05) for full documentation.
    pub fn spkw05(
        &mut self,
        handle: i32,
        body: i32,
        center: i32,
        frame: &str,
        first: f64,
        last: f64,
        segid: &str,
        gm: f64,
        n: i32,
        states: &[[f64; 6]],
        epochs: &[f64],
    ) -> Result<()> {
        raw::spkw05(
            self, handle, body, center, frame, first, last, segid, gm, n, states, epochs,
        )?;
        Ok(())
    }

    /// Write SPK segment, type 8
    ///
    /// Write a type 8 segment to an SPK file.
    ///
    /// See [`spkw08`](raw::spkw08) for full documentation.
    pub fn spkw08(
        &mut self,
        handle: i32,
        body: i32,
        center: i32,
        frame: &str,
        first: f64,
        last: f64,
        segid: &str,
        degree: i32,
        n: i32,
        states: &[[f64; 6]],
        begtim: f64,
        step: f64,
    ) -> Result<()> {
        raw::spkw08(
            self, handle, body, center, frame, first, last, segid, degree, n, states, begtim, step,
        )?;
        Ok(())
    }

    /// Write SPK segment, type 9
    ///
    /// Write a type 9 segment to an SPK file.
    ///
    /// See [`spkw09`](raw::spkw09) for full documentation.
    pub fn spkw09(
        &mut self,
        handle: i32,
        body: i32,
        center: i32,
        frame: &str,
        first: f64,
        last: f64,
        segid: &str,
        degree: i32,
        n: i32,
        states: &[[f64; 6]],
        epochs: &[f64],
    ) -> Result<()> {
        raw::spkw09(
            self, handle, body, center, frame, first, last, segid, degree, n, states, epochs,
        )?;
        Ok(())
    }

    /// SPK - write a type 10 segment
    ///
    /// Write an SPK type 10 segment to the file specified by
    /// the input HANDLE.
    ///
    /// See [`spkw10`](raw::spkw10) for full documentation.
    pub fn spkw10(
        &mut self,
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
    ) -> Result<()> {
        raw::spkw10(
            self, handle, body, center, frame, first, last, segid, consts, n, elems, epochs,
        )?;
        Ok(())
    }

    /// SPK, write segment, type 12
    ///
    /// Write a type 12 segment to an SPK file.
    ///
    /// See [`spkw12`](raw::spkw12) for full documentation.
    pub fn spkw12(
        &mut self,
        handle: i32,
        body: i32,
        center: i32,
        frame: &str,
        first: f64,
        last: f64,
        segid: &str,
        degree: i32,
        n: i32,
        states: &[[f64; 6]],
        begtim: f64,
        step: f64,
    ) -> Result<()> {
        raw::spkw12(
            self, handle, body, center, frame, first, last, segid, degree, n, states, begtim, step,
        )?;
        Ok(())
    }

    /// Write SPK segment, type 13
    ///
    /// Write a type 13 segment to an SPK file.
    ///
    /// See [`spkw13`](raw::spkw13) for full documentation.
    pub fn spkw13(
        &mut self,
        handle: i32,
        body: i32,
        center: i32,
        frame: &str,
        first: f64,
        last: f64,
        segid: &str,
        degree: i32,
        n: i32,
        states: &[[f64; 6]],
        epochs: &[f64],
    ) -> Result<()> {
        raw::spkw13(
            self, handle, body, center, frame, first, last, segid, degree, n, states, epochs,
        )?;
        Ok(())
    }

    /// SPK, write a type 15 segment
    ///
    /// Write an SPK segment of type 15 given a type 15 data record.
    ///
    /// See [`spkw15`](raw::spkw15) for full documentation.
    pub fn spkw15(
        &mut self,
        handle: i32,
        body: i32,
        center: i32,
        frame: &str,
        first: f64,
        last: f64,
        segid: &str,
        epoch: f64,
        tp: &[f64; 3],
        pa: &[f64; 3],
        p: f64,
        ecc: f64,
        j2flg: f64,
        pv: &[f64; 3],
        gm: f64,
        j2: f64,
        radius: f64,
    ) -> Result<()> {
        raw::spkw15(
            self, handle, body, center, frame, first, last, segid, epoch, tp, pa, p, ecc, j2flg,
            pv, gm, j2, radius,
        )?;
        Ok(())
    }

    /// SPK, write a type 17 segment
    ///
    /// Write an SPK segment of type 17 given a type 17 data record.
    ///
    /// See [`spkw17`](raw::spkw17) for full documentation.
    pub fn spkw17(
        &mut self,
        handle: i32,
        body: i32,
        center: i32,
        frame: &str,
        first: f64,
        last: f64,
        segid: &str,
        epoch: f64,
        eqel: &[f64; 9],
        rapol: f64,
        decpol: f64,
    ) -> Result<()> {
        raw::spkw17(
            self, handle, body, center, frame, first, last, segid, epoch, eqel, rapol, decpol,
        )?;
        Ok(())
    }

    /// Write SPK segment, type 18
    ///
    /// Write a type 18 segment to an SPK file.
    ///
    /// See [`spkw18`](raw::spkw18) for full documentation.
    pub fn spkw18(
        &mut self,
        handle: i32,
        subtyp: i32,
        body: i32,
        center: i32,
        frame: &str,
        first: f64,
        last: f64,
        segid: &str,
        degree: i32,
        n: i32,
        packts: &[f64],
        epochs: &[f64],
    ) -> Result<()> {
        raw::spkw18(
            self, handle, subtyp, body, center, frame, first, last, segid, degree, n, packts,
            epochs,
        )?;
        Ok(())
    }

    /// Write SPK segment, type 19
    ///
    /// Write a type 19 segment to an SPK file.
    ///
    /// See [`spkw19`](raw::spkw19) for full documentation.
    pub fn spkw19(
        &mut self,
        handle: i32,
        body: i32,
        center: i32,
        frame: &str,
        first: f64,
        last: f64,
        segid: &str,
        nintvl: i32,
        npkts: &[i32],
        subtps: &[i32],
        degres: &[i32],
        packts: &[f64],
        epochs: &[f64],
        ivlbds: &[f64],
        sellst: bool,
    ) -> Result<()> {
        raw::spkw19(
            self, handle, body, center, frame, first, last, segid, nintvl, npkts, subtps, degres,
            packts, epochs, ivlbds, sellst,
        )?;
        Ok(())
    }

    /// SPK, write segment, type 20
    ///
    /// Write a type 20 segment to an SPK file.
    ///
    /// See [`spkw20`](raw::spkw20) for full documentation.
    pub fn spkw20(
        &mut self,
        handle: i32,
        body: i32,
        center: i32,
        frame: &str,
        first: f64,
        last: f64,
        segid: &str,
        intlen: f64,
        n: i32,
        polydg: i32,
        cdata: &[f64],
        dscale: f64,
        tscale: f64,
        initjd: f64,
        initfr: f64,
    ) -> Result<()> {
        raw::spkw20(
            self, handle, body, center, frame, first, last, segid, intlen, n, polydg, cdata,
            dscale, tscale, initjd, initfr,
        )?;
        Ok(())
    }

    /// Write SPK segment, type 21
    ///
    /// Write a type 21 segment to an SPK file.
    ///
    /// See [`spkw21`](raw::spkw21) for full documentation.
    pub fn spkw21(
        &mut self,
        handle: i32,
        body: i32,
        center: i32,
        frame: &str,
        first: f64,
        last: f64,
        segid: &str,
        n: i32,
        dlsize: i32,
        dlines: &[f64],
        epochs: &[f64],
    ) -> Result<()> {
        raw::spkw21(
            self, handle, body, center, frame, first, last, segid, n, dlsize, dlines, epochs,
        )?;
        Ok(())
    }

    /// Surface and body ID codes to surface string
    ///
    /// Translate a surface ID code, together with a body ID code, to the
    /// corresponding surface name. If no such name exists, return a
    /// string representation of the surface ID code.
    ///
    /// Returns `(srfstr, isname)`.
    ///
    /// See [`srfc2s`](raw::srfc2s) for full documentation.
    pub fn srfc2s(&mut self, code: i32, bodyid: i32) -> Result<(String, bool)> {
        let mut srfstr = blank(inc::srftrn::SFNMLN);
        let mut isname: bool = Default::default();
        raw::srfc2s(self, code, bodyid, &mut srfstr, &mut isname)?;
        Ok((trim(srfstr), isname))
    }

    /// Surface ID and body string to surface string
    ///
    /// Translate a surface ID code, together with a body string, to the
    /// corresponding surface name. If no such surface name exists,
    /// return a string representation of the surface ID code.
    ///
    /// Returns `(srfstr, isname)`.
    ///
    /// See [`srfcss`](raw::srfcss) for full documentation.
    pub fn srfcss(&mut self, code: i32, bodstr: &str) -> Result<(String, bool)> {
        let mut srfstr = blank(inc::srftrn::SFNMLN);
        let mut isname: bool = Default::default();
        raw::srfcss(self, code, bodstr, &mut srfstr, &mut isname)?;
        Ok((trim(srfstr), isname))
    }

    /// Map surface points to outward normal vectors
    ///
    /// Map array of surface points on a specified target body to
    /// the corresponding unit length outward surface normal vectors.
    ///
    /// The surface of the target body may be represented by a triaxial
    /// ellipsoid or by topographic data provided by DSK files.
    ///
    /// Returns `normls`.
    ///
    /// See [`srfnrm`](raw::srfnrm) for full documentation.
    pub fn srfnrm(
        &mut self,
        method: &str,
        target: &str,
        et: f64,
        fixref: &str,
        npts: i32,
        srfpts: &[[f64; 3]],
    ) -> Result<Vec<[f64; 3]>> {
        let mut normls: Vec<[f64; 3]> = vec![Default::default(); npts.max(0) as usize];
        raw::srfnrm(self, method, target, et, fixref, npts, srfpts, &mut normls)?;
        Ok(normls)
    }

    /// Surface to rectangular coordinates
    ///
    /// Convert planetocentric latitude and longitude of a surface
    /// point on a specified body to rectangular coordinates.
    ///
    /// Returns `rectan`.
    ///
    /// See [`srfrec`](raw::srfrec) for full documentation.
    pub fn srfrec(&mut self, body: i32, lon: f64, lat: f64) -> Result<[f64; 3]> {
        let mut rectan: [f64; 3] = Default::default();
        raw::srfrec(self, body, lon, lat, &mut rectan)?;
        Ok(rectan)
    }

    /// Surface and body strings to surface ID code
    ///
    /// Translate a surface string, together with a body string, to the
    /// corresponding surface ID code. The input strings may contain
    /// names or integer ID codes.
    ///
    /// Returns `code`.
    ///
    /// See [`srfs2c`](raw::srfs2c) for full documentation.
    pub fn srfs2c(&mut self, srfstr: &str, bodstr: &str) -> Result<Option<i32>> {
        let mut code: i32 = Default::default();
        let mut found: bool = Default::default();
        raw::srfs2c(self, srfstr, bodstr, &mut code, &mut found)?;
        Ok(if found { Some(code) } else { None })
    }

    /// Surface string and body ID code to surface ID code
    ///
    /// Translate a surface string, together with a body ID code, to the
    /// corresponding surface ID code. The input surface string may
    /// contain a name or an integer ID code.
    ///
    /// Returns `code`.
    ///
    /// See [`srfscc`](raw::srfscc) for full documentation.
    pub fn srfscc(&mut self, srfstr: &str, bodyid: i32) -> Result<Option<i32>> {
        let mut code: i32 = Default::default();
        let mut found: bool = Default::default();
        raw::srfscc(self, srfstr, bodyid, &mut code, &mut found)?;
        Ok(if found { Some(code) } else { None })
    }

    /// Set the size of a character cell
    ///
    /// Set the size (maximum cardinality) of a character cell.
    ///
    /// See [`ssizec`](raw::ssizec) for full documentation.
    pub fn ssizec(&mut self, size: i32, cell: &mut CharCell) -> Result<()> {
        raw::ssizec(self, size, cell.as_arg_mut())?;
        Ok(())
    }

    /// Set the size of a double precision cell
    ///
    /// Set the size (maximum cardinality) of a double precision cell.
    ///
    /// See [`ssized`](raw::ssized) for full documentation.
    pub fn ssized(&mut self, size: i32, cell: &mut Cell<f64>) -> Result<()> {
        raw::ssized(self, size, cell.as_raw_mut_slice())?;
        Ok(())
    }

    /// Set the size of an integer cell
    ///
    /// Set the size (maximum cardinality) of an integer cell.
    ///
    /// See [`ssizei`](raw::ssizei) for full documentation.
    pub fn ssizei(&mut self, size: i32, cell: &mut Cell<i32>) -> Result<()> {
        raw::ssizei(self, size, cell.as_raw_mut_slice())?;
        Ok(())
    }

    /// STAR catalog type 1, find stars in RA-DEC box
    ///
    /// Search through a type 1 star catalog and return the number of
    /// stars within a specified RA - DEC rectangle.
    ///
    /// Returns `nstars`.
    ///
    /// See [`stcf01`](raw::stcf01) for full documentation.
    pub fn stcf01(
        &mut self,
        catnam: &str,
        westra: f64,
        eastra: f64,
        sthdec: f64,
        nthdec: f64,
    ) -> Result<i32> {
        let mut nstars: i32 = Default::default();
        raw::stcf01(self, catnam, westra, eastra, sthdec, nthdec, &mut nstars)?;
        Ok(nstars)
    }

    /// STAR catalog type 1, get star data
    ///
    /// Get data for a single star from a SPICE type 1 star catalog.
    ///
    /// Returns `(ra, dec, rasig, decsig, catnum, sptype, vmag)`.
    ///
    /// See [`stcg01`](raw::stcg01) for full documentation.
    pub fn stcg01(&mut self, index: i32) -> Result<(f64, f64, f64, f64, i32, String, f64)> {
        let mut ra: f64 = Default::default();
        let mut dec: f64 = Default::default();
        let mut rasig: f64 = Default::default();
        let mut decsig: f64 = Default::default();
        let mut catnum: i32 = Default::default();
        let mut sptype = blank(4);
        let mut vmag: f64 = Default::default();
        raw::stcg01(
            self,
            index,
            &mut ra,
            &mut dec,
            &mut rasig,
            &mut decsig,
            &mut catnum,
            &mut sptype,
            &mut vmag,
        )?;
        Ok((ra, dec, rasig, decsig, catnum, trim(sptype), vmag))
    }

    /// STAR catalog type 1, load catalog file
    ///
    /// Load SPICE type 1 star catalog and return the catalog's
    /// table name.
    ///
    /// Returns `(tabnam, handle)`.
    ///
    /// See [`stcl01`](raw::stcl01) for full documentation.
    pub fn stcl01(&mut self, catfnm: &str) -> Result<(String, i32)> {
        let mut tabnam = blank(inc::ektnamsz::TNAMSZ);
        let mut handle: i32 = Default::default();
        raw::stcl01(self, catfnm, &mut tabnam, &mut handle)?;
        Ok((trim(tabnam), handle))
    }

    /// Standard IO
    ///
    /// Return the logical unit associated with some standard input or
    /// standard output.
    ///
    /// Returns `unit`.
    ///
    /// See [`stdio`](raw::stdio) for full documentation.
    pub fn stdio(&mut self, name: &str) -> Result<i32> {
        let mut unit: i32 = Default::default();
        raw::stdio(self, name, &mut unit)?;
        Ok(unit)
    }

    /// Stellar Aberration
    ///
    /// Correct the apparent position of an object for stellar
    /// aberration.
    ///
    /// Returns `appobj`.
    ///
    /// See [`stelab`](raw::stelab) for full documentation.
    pub fn stelab(&mut self, pobj: &[f64; 3], vobs: &[f64; 3]) -> Result<[f64; 3]> {
        let mut appobj: [f64; 3] = Default::default();
        raw::stelab(self, pobj, vobs, &mut appobj)?;
        Ok(appobj)
    }

    /// Stellar aberration, transmission case
    ///
    /// Correct the position of a target for the stellar aberration
    /// effect on radiation transmitted from a specified observer to
    /// the target.
    ///
    /// Returns `corpos`.
    ///
    /// See [`stlabx`](raw::stlabx) for full documentation.
    pub fn stlabx(&mut self, pobj: &[f64; 3], vobs: &[f64; 3]) -> Result<[f64; 3]> {
        let mut corpos: [f64; 3] = Default::default();
        raw::stlabx(self, pobj, vobs, &mut corpos)?;
        Ok(corpos)
    }

    /// Stumpff functions 0 through 3
    ///
    /// Compute the values of the Stumpff functions C_0 through C_3 at
    /// a specified point.
    ///
    /// Returns `(c0, c1, c2, c3)`.
    ///
    /// See [`stmp03`](raw::stmp03) for full documentation.
    pub fn stmp03(&mut self, x: f64) -> Result<(f64, f64, f64, f64)> {
        let mut c0: f64 = Default::default();
        let mut c1: f64 = Default::default();
        let mut c2: f64 = Default::default();
        let mut c3: f64 = Default::default();
        raw::stmp03(self, x, &mut c0, &mut c1, &mut c2, &mut c3)?;
        Ok((c0, c1, c2, c3))
    }

    /// String from pool
    ///
    /// Retrieve the Nth string from a kernel pool variable, where the
    /// string may be continued across several components of the kernel
    /// pool variable.
    ///
    /// Returns `size`.
    ///
    /// See [`stpool`](raw::stpool) for full documentation.
    pub fn stpool(
        &mut self,
        item: &str,
        nth: i32,
        contin: &str,
        nthstr: &mut str,
    ) -> Result<Option<i32>> {
        let mut size: i32 = Default::default();
        let mut found: bool = Default::default();
        raw::stpool(self, item, nth, contin, nthstr, &mut size, &mut found)?;
        Ok(if found { Some(size) } else { None })
    }

    /// String to ET
    ///
    /// Convert a string representing an epoch to a double precision
    /// value representing the number of TDB seconds past the J2000
    /// epoch corresponding to the input epoch.
    ///
    /// Returns `et`.
    ///
    /// See [`str2et`](raw::str2et) for full documentation.
    pub fn str2et(&mut self, timstr: &str) -> Result<f64> {
        let mut et: f64 = Default::default();
        raw::str2et(self, timstr, &mut et)?;
        Ok(et)
    }

    /// Sub-observer point
    ///
    /// Compute the rectangular coordinates of the sub-observer point on
    /// a target body at a specified epoch, optionally corrected for
    /// light time and stellar aberration.
    ///
    /// The surface of the target body may be represented by a triaxial
    /// ellipsoid or by topographic data provided by DSK files.
    ///
    /// This routine supersedes SUBPT.
    ///
    /// Returns `(spoint, trgepc, srfvec)`.
    ///
    /// See [`subpnt`](raw::subpnt) for full documentation.
    pub fn subpnt(
        &mut self,
        method: &str,
        target: &str,
        et: f64,
        fixref: &str,
        abcorr: &str,
        obsrvr: &str,
    ) -> Result<([f64; 3], f64, [f64; 3])> {
        let mut spoint: [f64; 3] = Default::default();
        let mut trgepc: f64 = Default::default();
        let mut srfvec: [f64; 3] = Default::default();
        raw::subpnt(
            self,
            method,
            target,
            et,
            fixref,
            abcorr,
            obsrvr,
            &mut spoint,
            &mut trgepc,
            &mut srfvec,
        )?;
        Ok((spoint, trgepc, srfvec))
    }

    /// Sub-solar point
    ///
    /// Compute the rectangular coordinates of the sub-solar point on
    /// a target body at a specified epoch, optionally corrected for
    /// light time and stellar aberration.
    ///
    /// The surface of the target body may be represented by a triaxial
    /// ellipsoid or by topographic data provided by DSK files.
    ///
    /// This routine supersedes SUBSOL.
    ///
    /// Returns `(spoint, trgepc, srfvec)`.
    ///
    /// See [`subslr`](raw::subslr) for full documentation.
    pub fn subslr(
        &mut self,
        method: &str,
        target: &str,
        et: f64,
        fixref: &str,
        abcorr: &str,
        obsrvr: &str,
    ) -> Result<([f64; 3], f64, [f64; 3])> {
        let mut spoint: [f64; 3] = Default::default();
        let mut trgepc: f64 = Default::default();
        let mut srfvec: [f64; 3] = Default::default();
        raw::subslr(
            self,
            method,
            target,
            et,
            fixref,
            abcorr,
            obsrvr,
            &mut spoint,
            &mut trgepc,
            &mut srfvec,
        )?;
        Ok((spoint, trgepc, srfvec))
    }

    /// Sum of a double precision array
    ///
    /// Return the sum of the elements of a double precision array.
    ///
    /// See [`sumad`](raw::sumad) for full documentation.
    pub fn sumad(&self, array: &[f64], n: i32) -> f64 {
        raw::sumad(array, n)
    }

    /// Sum of an integer array
    ///
    /// Return the sum of the elements of an integer array.
    ///
    /// See [`sumai`](raw::sumai) for full documentation.
    pub fn sumai(&self, array: &[i32], n: i32) -> i32 {
        raw::sumai(array, n)
    }

    /// Surface normal vector on an ellipsoid
    ///
    /// Compute the outward-pointing, unit normal vector at a point on
    /// the surface of an ellipsoid.
    ///
    /// Returns `normal`.
    ///
    /// See [`surfnm`](raw::surfnm) for full documentation.
    pub fn surfnm(&mut self, a: f64, b: f64, c: f64, point: &[f64; 3]) -> Result<[f64; 3]> {
        let mut normal: [f64; 3] = Default::default();
        raw::surfnm(self, a, b, c, point, &mut normal)?;
        Ok(normal)
    }

    /// Surface point on an ellipsoid
    ///
    /// Determine the intersection of a line-of-sight vector with the
    /// surface of an ellipsoid.
    ///
    /// Returns `point`.
    ///
    /// See [`surfpt`](raw::surfpt) for full documentation.
    pub fn surfpt(
        &mut self,
        positn: &[f64; 3],
        u: &[f64; 3],
        a: f64,
        b: f64,
        c: f64,
    ) -> Result<Option<[f64; 3]>> {
        let mut point: [f64; 3] = Default::default();
        let mut found: bool = Default::default();
        raw::surfpt(self, positn, u, a, b, c, &mut point, &mut found)?;
        Ok(if found { Some(point) } else { None })
    }

    /// Surface point and velocity
    ///
    /// Find the state (position and velocity) of the surface intercept
    /// defined by a specified ray, ray velocity, and ellipsoid.
    ///
    /// Returns `stx`.
    ///
    /// See [`surfpv`](raw::surfpv) for full documentation.
    pub fn surfpv(
        &mut self,
        stvrtx: &[f64; 6],
        stdir: &[f64; 6],
        a: f64,
        b: f64,
        c: f64,
    ) -> Result<Option<[f64; 6]>> {
        let mut stx: [f64; 6] = Default::default();
        let mut found: bool = Default::default();
        raw::surfpv(self, stvrtx, stdir, a, b, c, &mut stx, &mut found)?;
        Ok(if found { Some(stx) } else { None })
    }

    /// Swap array, character
    ///
    /// Swap (exchange) two non-intersecting groups of contiguous
    /// elements of a character array.
    ///
    /// See [`swapac`](raw::swapac) for full documentation.
    pub fn swapac(
        &mut self,
        n: i32,
        locn: i32,
        m: i32,
        locm: i32,
        array: &mut CharVec,
    ) -> Result<()> {
        raw::swapac(self, n, locn, m, locm, array.as_arg_mut())?;
        Ok(())
    }

    /// Swap elements within a DP array
    ///
    /// Swap (exchange) two non-intersecting groups of contiguous
    /// elements of a double precision array.
    ///
    /// See [`swapad`](raw::swapad) for full documentation.
    pub fn swapad(
        &mut self,
        n: i32,
        locn: i32,
        m: i32,
        locm: i32,
        array: &mut [f64],
    ) -> Result<()> {
        raw::swapad(self, n, locn, m, locm, array)?;
        Ok(())
    }

    /// Swap elements within an integer array
    ///
    /// Swap (exchange) two non-intersecting groups of contiguous
    /// elements of an integer array.
    ///
    /// See [`swapai`](raw::swapai) for full documentation.
    pub fn swapai(
        &mut self,
        n: i32,
        locn: i32,
        m: i32,
        locm: i32,
        array: &mut [i32],
    ) -> Result<()> {
        raw::swapai(self, n, locn, m, locm, array)?;
        Ok(())
    }

    /// State Transformation Matrix
    ///
    /// Return the state transformation matrix from one frame to
    /// another at a specified epoch.
    ///
    /// Returns `xform`.
    ///
    /// See [`sxform`](raw::sxform) for full documentation.
    pub fn sxform(&mut self, from: &str, to: &str, et: f64) -> Result<[[f64; 6]; 6]> {
        let mut xform: [[f64; 6]; 6] = Default::default();
        raw::sxform(self, from, to, et, &mut xform)?;
        Ok(xform)
    }

    /// Delete a symbol from the symbol table
    ///
    /// Delete a symbol from a character symbol table. The symbol and its
    /// associated values are deleted.
    ///
    /// See [`sydelc`](raw::sydelc) for full documentation.
    pub fn sydelc(
        &mut self,
        name: &str,
        tabsym: &mut CharCell,
        tabptr: &mut Cell<i32>,
        tabval: &mut CharCell,
    ) -> Result<()> {
        raw::sydelc(
            self,
            name,
            tabsym.as_arg_mut(),
            tabptr.as_raw_mut_slice(),
            tabval.as_arg_mut(),
        )?;
        Ok(())
    }

    /// Delete a symbol from a symbol table
    ///
    /// Delete a symbol from a double precision symbol table. The symbol
    /// and its associated values are deleted.
    ///
    /// See [`sydeld`](raw::sydeld) for full documentation.
    pub fn sydeld(
        &mut self,
        name: &str,
        tabsym: &mut CharCell,
        tabptr: &mut Cell<i32>,
        tabval: &mut Cell<f64>,
    ) -> Result<()> {
        raw::sydeld(
            self,
            name,
            tabsym.as_arg_mut(),
            tabptr.as_raw_mut_slice(),
            tabval.as_raw_mut_slice(),
        )?;
        Ok(())
    }

    /// Delete a symbol from a symbol table
    ///
    /// Delete a symbol from an integer symbol table. The symbol
    /// and its associated values are deleted.
    ///
    /// See [`sydeli`](raw::sydeli) for full documentation.
    pub fn sydeli(
        &mut self,
        name: &str,
        tabsym: &mut CharCell,
        tabptr: &mut Cell<i32>,
        tabval: &mut Cell<i32>,
    ) -> Result<()> {
        raw::sydeli(
            self,
            name,
            tabsym.as_arg_mut(),
            tabptr.as_raw_mut_slice(),
            tabval.as_raw_mut_slice(),
        )?;
        Ok(())
    }

    /// Return the dimension of a symbol
    ///
    /// Return the dimension of a particular symbol in a character symbol
    /// table. If the symbol is not found, the function returns the value
    /// zero.
    ///
    /// See [`sydimc`](raw::sydimc) for full documentation.
    pub fn sydimc(
        &mut self,
        name: &str,
        tabsym: &CharCell,
        tabptr: &Cell<i32>,
        tabval: &CharCell,
    ) -> Result<i32> {
        raw::sydimc(
            self,
            name,
            tabsym.as_arg(),
            tabptr.as_raw_slice(),
            tabval.as_arg(),
        )
    }

    /// Return the dimension of a symbol
    ///
    /// Return the dimension of a particular symbol in a double precision
    /// symbol table. If the symbol is not found, the function returns the
    /// value zero.
    ///
    /// See [`sydimd`](raw::sydimd) for full documentation.
    pub fn sydimd(
        &mut self,
        name: &str,
        tabsym: &CharCell,
        tabptr: &Cell<i32>,
        tabval: &Cell<f64>,
    ) -> Result<i32> {
        raw::sydimd(
            self,
            name,
            tabsym.as_arg(),
            tabptr.as_raw_slice(),
            tabval.as_raw_slice(),
        )
    }

    /// Return the dimension of a symbol
    ///
    /// Return the dimension of a particular symbol in an integer symbol
    /// table. If the symbol is not found, the function returns the
    /// value zero.
    ///
    /// See [`sydimi`](raw::sydimi) for full documentation.
    pub fn sydimi(
        &mut self,
        name: &str,
        tabsym: &CharCell,
        tabptr: &Cell<i32>,
        tabval: &Cell<i32>,
    ) -> Result<i32> {
        raw::sydimi(
            self,
            name,
            tabsym.as_arg(),
            tabptr.as_raw_slice(),
            tabval.as_raw_slice(),
        )
    }

    /// Create a duplicate of a symbol
    ///
    /// Create a duplicate of a symbol within a character symbol table.
    /// If a symbol with the new name already exists, its components
    /// are replaced.
    ///
    /// See [`sydupc`](raw::sydupc) for full documentation.
    pub fn sydupc(
        &mut self,
        name: &str,
        copy: &str,
        tabsym: &mut CharCell,
        tabptr: &mut Cell<i32>,
        tabval: &mut CharCell,
    ) -> Result<()> {
        raw::sydupc(
            self,
            name,
            copy,
            tabsym.as_arg_mut(),
            tabptr.as_raw_mut_slice(),
            tabval.as_arg_mut(),
        )?;
        Ok(())
    }

    /// Create a duplicate of a symbol
    ///
    /// Create a duplicate of a symbol within a double precision symbol
    /// table. If a symbol with the new name already exists, its
    /// components are replaced.
    ///
    /// See [`sydupd`](raw::sydupd) for full documentation.
    pub fn sydupd(
        &mut self,
        name: &str,
        copy: &str,
        tabsym: &mut CharCell,
        tabptr: &mut Cell<i32>,
        tabval: &mut Cell<f64>,
    ) -> Result<()> {
        raw::sydupd(
            self,
            name,
            copy,
            tabsym.as_arg_mut(),
            tabptr.as_raw_mut_slice(),
            tabval.as_raw_mut_slice(),
        )?;
        Ok(())
    }

    /// Create a duplicate of a symbol
    ///
    /// Create a duplicate of a symbol within an integer symbol table.
    /// If a symbol with the new name already exists, its components
    /// are replaced.
    ///
    /// See [`sydupi`](raw::sydupi) for full documentation.
    pub fn sydupi(
        &mut self,
        name: &str,
        copy: &str,
        tabsym: &mut CharCell,
        tabptr: &mut Cell<i32>,
        tabval: &mut Cell<i32>,
    ) -> Result<()> {
        raw::sydupi(
            self,
            name,
            copy,
            tabsym.as_arg_mut(),
            tabptr.as_raw_mut_slice(),
            tabval.as_raw_mut_slice(),
        )?;
        Ok(())
    }

    /// Enqueue a value onto a symbol
    ///
    /// Enqueue a value onto a particular symbol in a character
    /// symbol table. If the symbol is not in the table, a new one
    /// is created.
    ///
    /// See [`syenqc`](raw::syenqc) for full documentation.
    pub fn syenqc(
        &mut self,
        name: &str,
        value: &str,
        tabsym: &mut CharCell,
        tabptr: &mut Cell<i32>,
        tabval: &mut CharCell,
    ) -> Result<()> {
        raw::syenqc(
            self,
            name,
            value,
            tabsym.as_arg_mut(),
            tabptr.as_raw_mut_slice(),
            tabval.as_arg_mut(),
        )?;
        Ok(())
    }

    /// Enqueue a value onto a symbol
    ///
    /// Enqueue a value onto a particular symbol in a double precision
    /// symbol table. If the symbol is not in the table, a new one
    /// is created.
    ///
    /// See [`syenqd`](raw::syenqd) for full documentation.
    pub fn syenqd(
        &mut self,
        name: &str,
        value: f64,
        tabsym: &mut CharCell,
        tabptr: &mut Cell<i32>,
        tabval: &mut Cell<f64>,
    ) -> Result<()> {
        raw::syenqd(
            self,
            name,
            value,
            tabsym.as_arg_mut(),
            tabptr.as_raw_mut_slice(),
            tabval.as_raw_mut_slice(),
        )?;
        Ok(())
    }

    /// Enqueue a value onto a symbol
    ///
    /// Enqueue a value onto a particular symbol in an integer
    /// symbol table. If the symbol is not in the table, a new one
    /// is created.
    ///
    /// See [`syenqi`](raw::syenqi) for full documentation.
    pub fn syenqi(
        &mut self,
        name: &str,
        value: i32,
        tabsym: &mut CharCell,
        tabptr: &mut Cell<i32>,
        tabval: &mut Cell<i32>,
    ) -> Result<()> {
        raw::syenqi(
            self,
            name,
            value,
            tabsym.as_arg_mut(),
            tabptr.as_raw_mut_slice(),
            tabval.as_raw_mut_slice(),
        )?;
        Ok(())
    }

    /// Fetch the Nth symbol in the table
    ///
    /// Fetch the Nth symbol in a character symbol table.
    ///
    /// Returns `name`.
    ///
    /// See [`syfetc`](raw::syfetc) for full documentation.
    pub fn syfetc(
        &mut self,
        nth: i32,
        tabsym: &CharCell,
        tabptr: &Cell<i32>,
        tabval: &CharCell,
    ) -> Result<Option<String>> {
        let mut name = blank((tabsym.element_length() as i32));
        let mut found: bool = Default::default();
        raw::syfetc(
            self,
            nth,
            tabsym.as_arg(),
            tabptr.as_raw_slice(),
            tabval.as_arg(),
            &mut name,
            &mut found,
        )?;
        Ok(if found { Some(trim(name)) } else { None })
    }

    /// Fetch the Nth symbol in the table
    ///
    /// Fetch the Nth symbol in a double precision symbol table.
    ///
    /// Returns `name`.
    ///
    /// See [`syfetd`](raw::syfetd) for full documentation.
    pub fn syfetd(
        &mut self,
        nth: i32,
        tabsym: &CharCell,
        tabptr: &Cell<i32>,
        tabval: &Cell<f64>,
    ) -> Result<Option<String>> {
        let mut name = blank((tabsym.element_length() as i32));
        let mut found: bool = Default::default();
        raw::syfetd(
            self,
            nth,
            tabsym.as_arg(),
            tabptr.as_raw_slice(),
            tabval.as_raw_slice(),
            &mut name,
            &mut found,
        )?;
        Ok(if found { Some(trim(name)) } else { None })
    }

    /// Fetch the Nth symbol in the table
    ///
    /// Fetch the Nth symbol in an integer symbol table.
    ///
    /// Returns `name`.
    ///
    /// See [`syfeti`](raw::syfeti) for full documentation.
    pub fn syfeti(
        &mut self,
        nth: i32,
        tabsym: &CharCell,
        tabptr: &Cell<i32>,
        tabval: &Cell<i32>,
    ) -> Result<Option<String>> {
        let mut name = blank((tabsym.element_length() as i32));
        let mut found: bool = Default::default();
        raw::syfeti(
            self,
            nth,
            tabsym.as_arg(),
            tabptr.as_raw_slice(),
            tabval.as_raw_slice(),
            &mut name,
            &mut found,
        )?;
        Ok(if found { Some(trim(name)) } else { None })
    }

    /// Return all components for a symbol
    ///
    /// Return the dimension and associated values for a particular
    /// symbol.
    ///
    /// Returns `n`.
    ///
    /// See [`sygetc`](raw::sygetc) for full documentation.
    pub fn sygetc(
        &mut self,
        name: &str,
        tabsym: &CharCell,
        tabptr: &Cell<i32>,
        tabval: &CharCell,
        values: &mut CharVec,
    ) -> Result<Option<i32>> {
        let mut n: i32 = Default::default();
        let mut found: bool = Default::default();
        raw::sygetc(
            self,
            name,
            tabsym.as_arg(),
            tabptr.as_raw_slice(),
            tabval.as_arg(),
            &mut n,
            values.as_arg_mut(),
            &mut found,
        )?;
        Ok(if found { Some(n) } else { None })
    }

    /// Return all components for a symbol
    ///
    /// Return the dimension and associated values for a particular
    /// symbol.
    ///
    /// Returns `(n, values)`.
    ///
    /// See [`sygetd`](raw::sygetd) for full documentation.
    pub fn sygetd(
        &mut self,
        name: &str,
        tabsym: &CharCell,
        tabptr: &Cell<i32>,
        tabval: &Cell<f64>,
    ) -> Result<Option<(i32, Vec<f64>)>> {
        let mut n: i32 = Default::default();
        let mut values: Vec<f64> = vec![Default::default(); (tabval.len() as i32) as usize];
        let mut found: bool = Default::default();
        raw::sygetd(
            self,
            name,
            tabsym.as_arg(),
            tabptr.as_raw_slice(),
            tabval.as_raw_slice(),
            &mut n,
            &mut values,
            &mut found,
        )?;
        Ok(if found { Some((n, values)) } else { None })
    }

    /// Return all components for a symbol
    ///
    /// Return the dimension and associated values for a particular
    /// symbol.
    ///
    /// Returns `(n, values)`.
    ///
    /// See [`sygeti`](raw::sygeti) for full documentation.
    pub fn sygeti(
        &mut self,
        name: &str,
        tabsym: &CharCell,
        tabptr: &Cell<i32>,
        tabval: &Cell<i32>,
    ) -> Result<Option<(i32, Vec<i32>)>> {
        let mut n: i32 = Default::default();
        let mut values: Vec<i32> = vec![Default::default(); (tabval.len() as i32) as usize];
        let mut found: bool = Default::default();
        raw::sygeti(
            self,
            name,
            tabsym.as_arg(),
            tabptr.as_raw_slice(),
            tabval.as_raw_slice(),
            &mut n,
            &mut values,
            &mut found,
        )?;
        Ok(if found { Some((n, values)) } else { None })
    }

    /// Return Nth value associated with the symbol
    ///
    /// Return the Nth value associated with a particular symbol in a
    /// character symbol table.
    ///
    /// Returns `value`.
    ///
    /// See [`synthc`](raw::synthc) for full documentation.
    pub fn synthc(
        &mut self,
        name: &str,
        nth: i32,
        tabsym: &CharCell,
        tabptr: &Cell<i32>,
        tabval: &CharCell,
    ) -> Result<Option<String>> {
        let mut value = blank((tabval.element_length() as i32));
        let mut found: bool = Default::default();
        raw::synthc(
            self,
            name,
            nth,
            tabsym.as_arg(),
            tabptr.as_raw_slice(),
            tabval.as_arg(),
            &mut value,
            &mut found,
        )?;
        Ok(if found { Some(trim(value)) } else { None })
    }

    /// Return the Nth component of a symbol
    ///
    /// Return the Nth component of a particular symbol in a double
    /// precision symbol table.
    ///
    /// Returns `value`.
    ///
    /// See [`synthd`](raw::synthd) for full documentation.
    pub fn synthd(
        &mut self,
        name: &str,
        nth: i32,
        tabsym: &CharCell,
        tabptr: &Cell<i32>,
        tabval: &Cell<f64>,
    ) -> Result<Option<f64>> {
        let mut value: f64 = Default::default();
        let mut found: bool = Default::default();
        raw::synthd(
            self,
            name,
            nth,
            tabsym.as_arg(),
            tabptr.as_raw_slice(),
            tabval.as_raw_slice(),
            &mut value,
            &mut found,
        )?;
        Ok(if found { Some(value) } else { None })
    }

    /// Return the Nth component of a symbol
    ///
    /// Return the Nth component of a particular symbol in an integer
    /// symbol table.
    ///
    /// Returns `value`.
    ///
    /// See [`synthi`](raw::synthi) for full documentation.
    pub fn synthi(
        &mut self,
        name: &str,
        nth: i32,
        tabsym: &CharCell,
        tabptr: &Cell<i32>,
        tabval: &Cell<i32>,
    ) -> Result<Option<i32>> {
        let mut value: i32 = Default::default();
        let mut found: bool = Default::default();
        raw::synthi(
            self,
            name,
            nth,
            tabsym.as_arg(),
            tabptr.as_raw_slice(),
            tabval.as_raw_slice(),
            &mut value,
            &mut found,
        )?;
        Ok(if found { Some(value) } else { None })
    }

    /// Order the components of a single symbol
    ///
    /// Order the components of a single symbol in a character symbol
    /// table. The components are ordered according to the ASCII collating
    /// sequence.
    ///
    /// See [`syordc`](raw::syordc) for full documentation.
    pub fn syordc(
        &mut self,
        name: &str,
        tabsym: &CharCell,
        tabptr: &Cell<i32>,
        tabval: &mut CharCell,
    ) -> Result<()> {
        raw::syordc(
            self,
            name,
            tabsym.as_arg(),
            tabptr.as_raw_slice(),
            tabval.as_arg_mut(),
        )?;
        Ok(())
    }

    /// Order the components of a single symbol
    ///
    /// Order the components of a single symbol in a double precision
    /// symbol table. The components are sorted in increasing order.
    ///
    /// See [`syordd`](raw::syordd) for full documentation.
    pub fn syordd(
        &mut self,
        name: &str,
        tabsym: &CharCell,
        tabptr: &Cell<i32>,
        tabval: &mut Cell<f64>,
    ) -> Result<()> {
        raw::syordd(
            self,
            name,
            tabsym.as_arg(),
            tabptr.as_raw_slice(),
            tabval.as_raw_mut_slice(),
        )?;
        Ok(())
    }

    /// Order the components of a single symbol
    ///
    /// Order the components of a single symbol in an integer symbol
    /// table. The components are sorted in increasing order.
    ///
    /// See [`syordi`](raw::syordi) for full documentation.
    pub fn syordi(
        &mut self,
        name: &str,
        tabsym: &CharCell,
        tabptr: &Cell<i32>,
        tabval: &mut Cell<i32>,
    ) -> Result<()> {
        raw::syordi(
            self,
            name,
            tabsym.as_arg(),
            tabptr.as_raw_slice(),
            tabval.as_raw_mut_slice(),
        )?;
        Ok(())
    }

    /// Pop a value from a particular symbol
    ///
    /// Pop a value associated with a particular symbol in a character
    /// symbol table. The first value associated with the symbol is
    /// removed, and subsequent values are moved forward.
    ///
    /// Returns `value`.
    ///
    /// See [`sypopc`](raw::sypopc) for full documentation.
    pub fn sypopc(
        &mut self,
        name: &str,
        tabsym: &mut CharCell,
        tabptr: &mut Cell<i32>,
        tabval: &mut CharCell,
    ) -> Result<Option<String>> {
        let mut value = blank((tabval.element_length() as i32));
        let mut found: bool = Default::default();
        raw::sypopc(
            self,
            name,
            tabsym.as_arg_mut(),
            tabptr.as_raw_mut_slice(),
            tabval.as_arg_mut(),
            &mut value,
            &mut found,
        )?;
        Ok(if found { Some(trim(value)) } else { None })
    }

    /// Pop a value from a particular symbol
    ///
    /// Pop a value associated with a particular symbol in a double
    /// precision symbol table. The first value associated with the
    /// symbol is removed, and subsequent values are moved forward.
    ///
    /// Returns `value`.
    ///
    /// See [`sypopd`](raw::sypopd) for full documentation.
    pub fn sypopd(
        &mut self,
        name: &str,
        tabsym: &mut CharCell,
        tabptr: &mut Cell<i32>,
        tabval: &mut Cell<f64>,
    ) -> Result<Option<f64>> {
        let mut value: f64 = Default::default();
        let mut found: bool = Default::default();
        raw::sypopd(
            self,
            name,
            tabsym.as_arg_mut(),
            tabptr.as_raw_mut_slice(),
            tabval.as_raw_mut_slice(),
            &mut value,
            &mut found,
        )?;
        Ok(if found { Some(value) } else { None })
    }

    /// Pop a value from a particular symbol
    ///
    /// Pop a value associated with a particular symbol in an integer
    /// symbol table. The first value associated with the symbol is
    /// removed, and subsequent values are moved forward.
    ///
    /// Returns `value`.
    ///
    /// See [`sypopi`](raw::sypopi) for full documentation.
    pub fn sypopi(
        &mut self,
        name: &str,
        tabsym: &mut CharCell,
        tabptr: &mut Cell<i32>,
        tabval: &mut Cell<i32>,
    ) -> Result<Option<i32>> {
        let mut value: i32 = Default::default();
        let mut found: bool = Default::default();
        raw::sypopi(
            self,
            name,
            tabsym.as_arg_mut(),
            tabptr.as_raw_mut_slice(),
            tabval.as_raw_mut_slice(),
            &mut value,
            &mut found,
        )?;
        Ok(if found { Some(value) } else { None })
    }

    /// Push a value onto a particular symbol
    ///
    /// Push a value onto a particular symbol in a character symbol table.
    /// The previous value(s) associated with the symbol is extended at
    /// the front. A new symbol is created if necessary.
    ///
    /// See [`sypshc`](raw::sypshc) for full documentation.
    pub fn sypshc(
        &mut self,
        name: &str,
        value: &str,
        tabsym: &mut CharCell,
        tabptr: &mut Cell<i32>,
        tabval: &mut CharCell,
    ) -> Result<()> {
        raw::sypshc(
            self,
            name,
            value,
            tabsym.as_arg_mut(),
            tabptr.as_raw_mut_slice(),
            tabval.as_arg_mut(),
        )?;
        Ok(())
    }

    /// Push a value onto a particular symbol
    ///
    /// Push a value onto a particular symbol in a double precision
    /// symbol table. The previous value(s) associated with the symbol
    /// is extended at the front. A new symbol is created if necessary.
    ///
    /// See [`sypshd`](raw::sypshd) for full documentation.
    pub fn sypshd(
        &mut self,
        name: &str,
        value: f64,
        tabsym: &mut CharCell,
        tabptr: &mut Cell<i32>,
        tabval: &mut Cell<f64>,
    ) -> Result<()> {
        raw::sypshd(
            self,
            name,
            value,
            tabsym.as_arg_mut(),
            tabptr.as_raw_mut_slice(),
            tabval.as_raw_mut_slice(),
        )?;
        Ok(())
    }

    /// Push a value onto a particular symbol
    ///
    /// Push a value onto a particular symbol in an integer symbol table.
    /// The previous value(s) associated with the symbol is extended at
    /// the front. A new symbol is created if necessary.
    ///
    /// See [`sypshi`](raw::sypshi) for full documentation.
    pub fn sypshi(
        &mut self,
        name: &str,
        value: i32,
        tabsym: &mut CharCell,
        tabptr: &mut Cell<i32>,
        tabval: &mut Cell<i32>,
    ) -> Result<()> {
        raw::sypshi(
            self,
            name,
            value,
            tabsym.as_arg_mut(),
            tabptr.as_raw_mut_slice(),
            tabval.as_raw_mut_slice(),
        )?;
        Ok(())
    }

    /// Set the values associated with a symbol
    ///
    /// Set the values of a particular symbol in a character symbol table.
    /// If the symbol already exists, the previous values associated with
    /// it are removed, otherwise a new symbol is created.
    ///
    /// See [`syputc`](raw::syputc) for full documentation.
    pub fn syputc(
        &mut self,
        name: &str,
        values: &CharVec,
        n: i32,
        tabsym: &mut CharCell,
        tabptr: &mut Cell<i32>,
        tabval: &mut CharCell,
    ) -> Result<()> {
        raw::syputc(
            self,
            name,
            values.as_arg(),
            n,
            tabsym.as_arg_mut(),
            tabptr.as_raw_mut_slice(),
            tabval.as_arg_mut(),
        )?;
        Ok(())
    }

    /// Set the values associated with a symbol
    ///
    /// Set the values of a particular symbol in a double precision
    /// symbol table. If the symbol already exists, the previous values
    /// associated with it are removed, otherwise a new symbol is created.
    ///
    /// See [`syputd`](raw::syputd) for full documentation.
    pub fn syputd(
        &mut self,
        name: &str,
        values: &[f64],
        n: i32,
        tabsym: &mut CharCell,
        tabptr: &mut Cell<i32>,
        tabval: &mut Cell<f64>,
    ) -> Result<()> {
        raw::syputd(
            self,
            name,
            values,
            n,
            tabsym.as_arg_mut(),
            tabptr.as_raw_mut_slice(),
            tabval.as_raw_mut_slice(),
        )?;
        Ok(())
    }

    /// Set the values associated with a symbol
    ///
    /// Set the values of a particular symbol in an integer symbol table.
    /// If the symbol already exists, the previous values associated with
    /// it are removed, otherwise a new symbol is created.
    ///
    /// See [`syputi`](raw::syputi) for full documentation.
    pub fn syputi(
        &mut self,
        name: &str,
        values: &[i32],
        n: i32,
        tabsym: &mut CharCell,
        tabptr: &mut Cell<i32>,
        tabval: &mut Cell<i32>,
    ) -> Result<()> {
        raw::syputi(
            self,
            name,
            values,
            n,
            tabsym.as_arg_mut(),
            tabptr.as_raw_mut_slice(),
            tabval.as_raw_mut_slice(),
        )?;
        Ok(())
    }

    /// Rename an existing symbol
    ///
    /// Rename an existing symbol in a character symbol table.
    ///
    /// See [`syrenc`](raw::syrenc) for full documentation.
    pub fn syrenc(
        &mut self,
        old: &str,
        new: &str,
        tabsym: &mut CharCell,
        tabptr: &mut Cell<i32>,
        tabval: &mut CharCell,
    ) -> Result<()> {
        raw::syrenc(
            self,
            old,
            new,
            tabsym.as_arg_mut(),
            tabptr.as_raw_mut_slice(),
            tabval.as_arg_mut(),
        )?;
        Ok(())
    }

    /// Rename an existing symbol
    ///
    /// Rename an existing symbol in a double precision symbol table.
    ///
    /// See [`syrend`](raw::syrend) for full documentation.
    pub fn syrend(
        &mut self,
        old: &str,
        new: &str,
        tabsym: &mut CharCell,
        tabptr: &mut Cell<i32>,
        tabval: &mut Cell<f64>,
    ) -> Result<()> {
        raw::syrend(
            self,
            old,
            new,
            tabsym.as_arg_mut(),
            tabptr.as_raw_mut_slice(),
            tabval.as_raw_mut_slice(),
        )?;
        Ok(())
    }

    /// Rename an existing symbol
    ///
    /// Rename an existing symbol in an integer symbol table.
    ///
    /// See [`syreni`](raw::syreni) for full documentation.
    pub fn syreni(
        &mut self,
        old: &str,
        new: &str,
        tabsym: &mut CharCell,
        tabptr: &mut Cell<i32>,
        tabval: &mut Cell<i32>,
    ) -> Result<()> {
        raw::syreni(
            self,
            old,
            new,
            tabsym.as_arg_mut(),
            tabptr.as_raw_mut_slice(),
            tabval.as_raw_mut_slice(),
        )?;
        Ok(())
    }

    /// Select a subset of the values of a symbol
    ///
    /// Select a subset of the values associated with a particular
    /// symbol in a character symbol table.
    ///
    /// See [`syselc`](raw::syselc) for full documentation.
    pub fn syselc(
        &mut self,
        name: &str,
        begin: i32,
        end: i32,
        tabsym: &CharCell,
        tabptr: &Cell<i32>,
        tabval: &CharCell,
        values: &mut CharVec,
    ) -> Result<Option<()>> {
        let mut found: bool = Default::default();
        raw::syselc(
            self,
            name,
            begin,
            end,
            tabsym.as_arg(),
            tabptr.as_raw_slice(),
            tabval.as_arg(),
            values.as_arg_mut(),
            &mut found,
        )?;
        Ok(if found { Some(()) } else { None })
    }

    /// Select a subset of the values of a symbol
    ///
    /// Select a subset of the values associated with a particular
    /// symbol in a double precision symbol table.
    ///
    /// Returns `values`.
    ///
    /// See [`syseld`](raw::syseld) for full documentation.
    pub fn syseld(
        &mut self,
        name: &str,
        begin: i32,
        end: i32,
        tabsym: &CharCell,
        tabptr: &Cell<i32>,
        tabval: &Cell<f64>,
    ) -> Result<Option<Vec<f64>>> {
        let mut values: Vec<f64> = vec![Default::default(); (end + 1 - begin).max(0) as usize];
        let mut found: bool = Default::default();
        raw::syseld(
            self,
            name,
            begin,
            end,
            tabsym.as_arg(),
            tabptr.as_raw_slice(),
            tabval.as_raw_slice(),
            &mut values,
            &mut found,
        )?;
        Ok(if found { Some(values) } else { None })
    }

    /// Select a subset of the values of a symbol
    ///
    /// Select a subset of the values associated with a particular
    /// symbol in an integer symbol table.
    ///
    /// Returns `values`.
    ///
    /// See [`syseli`](raw::syseli) for full documentation.
    pub fn syseli(
        &mut self,
        name: &str,
        begin: i32,
        end: i32,
        tabsym: &CharCell,
        tabptr: &Cell<i32>,
        tabval: &Cell<i32>,
    ) -> Result<Option<Vec<i32>>> {
        let mut values: Vec<i32> = vec![Default::default(); (end + 1 - begin).max(0) as usize];
        let mut found: bool = Default::default();
        raw::syseli(
            self,
            name,
            begin,
            end,
            tabsym.as_arg(),
            tabptr.as_raw_slice(),
            tabval.as_raw_slice(),
            &mut values,
            &mut found,
        )?;
        Ok(if found { Some(values) } else { None })
    }

    /// Set the value associated with a symbol
    ///
    /// Set the value of a particular symbol in a character symbol table.
    /// If the symbol already exists, the previous values associated with
    /// it are removed, otherwise a new symbol is created.
    ///
    /// See [`sysetc`](raw::sysetc) for full documentation.
    pub fn sysetc(
        &mut self,
        name: &str,
        value: &str,
        tabsym: &mut CharCell,
        tabptr: &mut Cell<i32>,
        tabval: &mut CharCell,
    ) -> Result<()> {
        raw::sysetc(
            self,
            name,
            value,
            tabsym.as_arg_mut(),
            tabptr.as_raw_mut_slice(),
            tabval.as_arg_mut(),
        )?;
        Ok(())
    }

    /// Set the value associated with a symbol
    ///
    /// Set the value of a particular symbol in a double precision symbol
    /// table. If the symbol already exists, the previous values
    /// associated with it are removed, otherwise a new symbol is created.
    ///
    /// See [`sysetd`](raw::sysetd) for full documentation.
    pub fn sysetd(
        &mut self,
        name: &str,
        value: f64,
        tabsym: &mut CharCell,
        tabptr: &mut Cell<i32>,
        tabval: &mut Cell<f64>,
    ) -> Result<()> {
        raw::sysetd(
            self,
            name,
            value,
            tabsym.as_arg_mut(),
            tabptr.as_raw_mut_slice(),
            tabval.as_raw_mut_slice(),
        )?;
        Ok(())
    }

    /// Set the value associated with a symbol
    ///
    /// Set the value of a particular symbol in an integer symbol table.
    /// If the symbol already exists, the previous values associated with
    /// it are removed, otherwise a new symbol is created.
    ///
    /// See [`syseti`](raw::syseti) for full documentation.
    pub fn syseti(
        &mut self,
        name: &str,
        value: i32,
        tabsym: &mut CharCell,
        tabptr: &mut Cell<i32>,
        tabval: &mut Cell<i32>,
    ) -> Result<()> {
        raw::syseti(
            self,
            name,
            value,
            tabsym.as_arg_mut(),
            tabptr.as_raw_mut_slice(),
            tabval.as_raw_mut_slice(),
        )?;
        Ok(())
    }

    /// Transpose two values associated with a symbol
    ///
    /// Transpose two values associated with a particular symbol in a
    /// character symbol table.
    ///
    /// See [`sytrnc`](raw::sytrnc) for full documentation.
    pub fn sytrnc(
        &mut self,
        name: &str,
        idx1: i32,
        idx2: i32,
        tabsym: &CharCell,
        tabptr: &Cell<i32>,
        tabval: &mut CharCell,
    ) -> Result<()> {
        raw::sytrnc(
            self,
            name,
            idx1,
            idx2,
            tabsym.as_arg(),
            tabptr.as_raw_slice(),
            tabval.as_arg_mut(),
        )?;
        Ok(())
    }

    /// Transpose two values associated with a symbol
    ///
    /// Transpose two values associated with a particular symbol in a
    /// double precision symbol table.
    ///
    /// See [`sytrnd`](raw::sytrnd) for full documentation.
    pub fn sytrnd(
        &mut self,
        name: &str,
        idx1: i32,
        idx2: i32,
        tabsym: &CharCell,
        tabptr: &Cell<i32>,
        tabval: &mut Cell<f64>,
    ) -> Result<()> {
        raw::sytrnd(
            self,
            name,
            idx1,
            idx2,
            tabsym.as_arg(),
            tabptr.as_raw_slice(),
            tabval.as_raw_mut_slice(),
        )?;
        Ok(())
    }

    /// Transpose two values associated with a symbol
    ///
    /// Transpose two values associated with a particular symbol in an
    /// integer symbol table.
    ///
    /// See [`sytrni`](raw::sytrni) for full documentation.
    pub fn sytrni(
        &mut self,
        name: &str,
        idx1: i32,
        idx2: i32,
        tabsym: &CharCell,
        tabptr: &Cell<i32>,
        tabval: &mut Cell<i32>,
    ) -> Result<()> {
        raw::sytrni(
            self,
            name,
            idx1,
            idx2,
            tabsym.as_arg(),
            tabptr.as_raw_slice(),
            tabval.as_raw_mut_slice(),
        )?;
        Ok(())
    }

    /// Ray-ellipsoid tangent point
    ///
    /// Compute, for a given observer, ray emanating from the observer,
    /// and target, the "tangent point": the point on the ray nearest
    /// to the target's surface. Also compute the point on the target's
    /// surface nearest to the tangent point.
    ///
    /// The locations of both points are optionally corrected for light
    /// time and stellar aberration.
    ///
    /// The surface shape is modeled as a triaxial ellipsoid.
    ///
    /// Returns `(tanpt, alt, range, srfpt, trgepc, srfvec)`.
    ///
    /// See [`tangpt`](raw::tangpt) for full documentation.
    pub fn tangpt(
        &mut self,
        method: &str,
        target: &str,
        et: f64,
        fixref: &str,
        abcorr: &str,
        corloc: &str,
        obsrvr: &str,
        dref: &str,
        dvec: &[f64; 3],
    ) -> Result<([f64; 3], f64, f64, [f64; 3], f64, [f64; 3])> {
        let mut tanpt: [f64; 3] = Default::default();
        let mut alt: f64 = Default::default();
        let mut range: f64 = Default::default();
        let mut srfpt: [f64; 3] = Default::default();
        let mut trgepc: f64 = Default::default();
        let mut srfvec: [f64; 3] = Default::default();
        raw::tangpt(
            self,
            method,
            target,
            et,
            fixref,
            abcorr,
            corloc,
            obsrvr,
            dref,
            dvec,
            &mut tanpt,
            &mut alt,
            &mut range,
            &mut srfpt,
            &mut trgepc,
            &mut srfvec,
        )?;
        Ok((tanpt, alt, range, srfpt, trgepc, srfvec))
    }

    /// Time Check
    ///
    /// Determine whether the components of a time vector are in the
    /// "usual" range for the components, if component checking is
    /// enabled.
    ///
    /// If component checking is not enabled, this routine simply
    /// returns after setting the outputs.
    ///
    /// Returns `(ok, error)`.
    ///
    /// See [`tcheck`](raw::tcheck) for full documentation.
    pub fn tcheck(
        &mut self,
        tvec: &[f64],
        type_: &str,
        mods: bool,
        modify: &CharVec,
    ) -> (bool, String) {
        let mut ok: bool = Default::default();
        let mut error = blank(inc::errhnd::LMSGLN);
        raw::tcheck(
            self,
            tvec,
            type_,
            mods,
            modify.as_arg(),
            &mut ok,
            &mut error,
        );
        (ok, trim(error))
    }

    /// Parse check---check format of strings
    ///
    /// Restrict the set of strings that are recognized by SPICE time
    /// parsing routines to those that have standard values for all time
    /// components.
    ///
    /// See [`tparch`](raw::tparch) for full documentation.
    pub fn tparch(&mut self, type_: &str) {
        raw::tparch(self, type_);
    }

    /// Time components are checked
    ///
    /// Determine whether component checking is enabled for time strings.
    ///
    /// Returns `type_`.
    ///
    /// See [`tchckd`](raw::tchckd) for full documentation.
    pub fn tchckd(&mut self) -> String {
        let mut type_ = blank(3);
        raw::tchckd(self, &mut type_);
        trim(type_)
    }

    /// Terminator points on an extended object
    ///
    /// Find terminator points on a target body. The caller specifies
    /// half-planes, bounded by the illumination source center-target
    /// center vector, in which to search for terminator points.
    ///
    /// The terminator can be either umbral or penumbral. The umbral
    /// terminator is the boundary of the region on the target surface
    /// where no light from the source is visible. The penumbral
    /// terminator is the boundary of the region on the target surface
    /// where none of the light from the source is blocked by the target
    /// itself.
    ///
    /// The surface of the target body may be represented either by a
    /// triaxial ellipsoid or by topographic data.
    ///
    /// Returns `(npts, points, epochs, trmvcs)`.
    ///
    /// See [`termpt`](raw::termpt) for full documentation.
    pub fn termpt(
        &mut self,
        method: &str,
        ilusrc: &str,
        target: &str,
        et: f64,
        fixref: &str,
        abcorr: &str,
        corloc: &str,
        obsrvr: &str,
        refvec: &[f64; 3],
        rolstp: f64,
        ncuts: i32,
        schstp: f64,
        soltol: f64,
        maxn: i32,
    ) -> Result<(Vec<i32>, Vec<[f64; 3]>, Vec<f64>, Vec<[f64; 3]>)> {
        let mut npts: Vec<i32> = vec![Default::default(); ncuts.max(0) as usize];
        let mut points: Vec<[f64; 3]> = vec![Default::default(); maxn.max(0) as usize];
        let mut epochs: Vec<f64> = vec![Default::default(); maxn.max(0) as usize];
        let mut trmvcs: Vec<[f64; 3]> = vec![Default::default(); maxn.max(0) as usize];
        raw::termpt(
            self,
            method,
            ilusrc,
            target,
            et,
            fixref,
            abcorr,
            corloc,
            obsrvr,
            refvec,
            rolstp,
            ncuts,
            schstp,
            soltol,
            maxn,
            &mut npts,
            &mut points,
            &mut epochs,
            &mut trmvcs,
        )?;
        Ok((npts, points, epochs, trmvcs))
    }

    /// Time --- Expand year
    ///
    /// Expand an abbreviated year to a full year specification.
    ///
    /// See [`texpyr`](raw::texpyr) for full documentation.
    pub fn texpyr(&mut self, year: &mut i32) {
        raw::texpyr(self, year);
    }

    /// Time --- set year expansion boundaries
    ///
    /// Set the lower bound on the 100 year range.
    ///
    /// See [`tsetyr`](raw::tsetyr) for full documentation.
    pub fn tsetyr(&mut self, year: i32) {
        raw::tsetyr(self, year);
    }

    /// Time Software Defaults
    ///
    /// Set and retrieve the defaults associated with calendar
    /// input strings.
    ///
    /// See [`timdef`](raw::timdef) for full documentation.
    pub fn timdef(&mut self, action: &str, item: &str, value: &mut str) -> Result<()> {
        raw::timdef(self, action, item, value)?;
        Ok(())
    }

    /// Time Output
    ///
    /// Convert an input epoch represented in TDB seconds past the TDB
    /// epoch of J2000 to a character string formatted to the
    /// specifications of a user's format picture.
    ///
    /// Returns `output`.
    ///
    /// See [`timout`](raw::timout) for full documentation.
    pub fn timout(&mut self, et: f64, pictur: &str) -> Result<String> {
        let mut output = blank((pictur.len() as i32 * 2));
        raw::timout(self, et, pictur, &mut output)?;
        Ok(trim(output))
    }

    /// Transformation, inertial position to bodyfixed
    ///
    /// Return a 3x3 matrix that transforms positions in inertial
    /// coordinates to positions in body-equator-and-prime-meridian
    /// coordinates.
    ///
    /// Returns `tipm`.
    ///
    /// See [`tipbod`](raw::tipbod) for full documentation.
    pub fn tipbod(&mut self, ref_: &str, body: i32, et: f64) -> Result<[[f64; 3]; 3]> {
        let mut tipm: [[f64; 3]; 3] = Default::default();
        raw::tipbod(self, ref_, body, et, &mut tipm)?;
        Ok(tipm)
    }

    /// Transformation, inertial state to bodyfixed
    ///
    /// Return a 6x6 matrix that transforms states in inertial
    /// coordinates to states in body-equator-and-prime-meridian
    /// coordinates.
    ///
    /// Returns `tsipm`.
    ///
    /// See [`tisbod`](raw::tisbod) for full documentation.
    pub fn tisbod(&mut self, ref_: &str, body: i32, et: f64) -> Result<[[f64; 6]; 6]> {
        let mut tsipm: [[f64; 6]; 6] = Default::default();
        raw::tisbod(self, ref_, body, et, &mut tsipm)?;
        Ok(tsipm)
    }

    /// TK frame, find position rotation
    ///
    /// Find the position rotation matrix from a Text Kernel (TK) frame
    /// with the specified frame class ID to its base frame.
    ///
    /// Returns `(rot, frame)`.
    ///
    /// See [`tkfram`](raw::tkfram) for full documentation.
    pub fn tkfram(&mut self, frcode: i32) -> Result<Option<([[f64; 3]; 3], i32)>> {
        let mut rot: [[f64; 3]; 3] = Default::default();
        let mut frame: i32 = Default::default();
        let mut found: bool = Default::default();
        raw::tkfram(self, frcode, &mut rot, &mut frame, &mut found)?;
        Ok(if found { Some((rot, frame)) } else { None })
    }

    /// Toolkit version strings
    ///
    /// Return the latest version string of a given item such as the
    /// Toolkit or a routine name.
    ///
    /// Returns `verstr`.
    ///
    /// See [`tkvrsn`](raw::tkvrsn) for full documentation.
    pub fn tkvrsn(&self, item: &str) -> String {
        let mut verstr = blank(80);
        raw::tkvrsn(item, &mut verstr);
        trim(verstr)
    }

    /// To Standard Output
    ///
    /// Write a line of text to standard output.
    ///
    /// See [`tostdo`](raw::tostdo) for full documentation.
    pub fn tostdo(&mut self, line: &str) -> Result<()> {
        raw::tostdo(self, line)?;
        Ok(())
    }

    /// Parse a UTC time string
    ///
    /// Parse a time string and return seconds past the J2000 epoch
    /// on a formal calendar.
    ///
    /// Returns `(sp2000, errmsg)`.
    ///
    /// See [`tparse`](raw::tparse) for full documentation.
    pub fn tparse(&mut self, string: &str) -> Result<(f64, String)> {
        let mut sp2000: f64 = Default::default();
        let mut errmsg = blank(inc::errhnd::LMSGLN);
        raw::tparse(self, string, &mut sp2000, &mut errmsg)?;
        Ok((sp2000, trim(errmsg)))
    }

    /// Time string ---parse to a time vector
    ///
    /// Parse the components of a time string and return a vector of the
    /// components of that string. Also return an array of any modifiers
    /// present in the input string; these may alter the interpretation
    /// of the components.
    ///
    /// Returns `(tvec, ntvec, type_, mods, yabbrv, succes, pictur, error)`.
    ///
    /// See [`tpartv`](raw::tpartv) for full documentation.
    pub fn tpartv(
        &mut self,
        string: &str,
        modify: &mut CharVec,
    ) -> (Vec<f64>, i32, String, bool, bool, bool, String, String) {
        let mut tvec: Vec<f64> = vec![Default::default(); 6 as usize];
        let mut ntvec: i32 = Default::default();
        let mut type_ = blank(3);
        let mut mods: bool = Default::default();
        let mut yabbrv: bool = Default::default();
        let mut succes: bool = Default::default();
        let mut pictur = blank((64 * 5));
        let mut error = blank(inc::errhnd::LMSGLN);
        raw::tpartv(
            self,
            string,
            &mut tvec,
            &mut ntvec,
            &mut type_,
            modify.as_arg_mut(),
            &mut mods,
            &mut yabbrv,
            &mut succes,
            &mut pictur,
            &mut error,
        );
        (
            tvec,
            ntvec,
            trim(type_),
            mods,
            yabbrv,
            succes,
            trim(pictur),
            trim(error),
        )
    }

    /// Create a Time Format Picture
    ///
    /// Create a time format picture suitable for use by the routine
    /// TIMOUT from a given sample time string.
    ///
    /// Returns `(pictur, ok, errmsg)`.
    ///
    /// See [`tpictr`](raw::tpictr) for full documentation.
    pub fn tpictr(&mut self, sample: &str) -> (String, bool, String) {
        let mut pictur = blank((64 * 5));
        let mut ok: bool = Default::default();
        let mut errmsg = blank(inc::errhnd::LMSGLN);
        raw::tpictr(self, sample, &mut pictur, &mut ok, &mut errmsg);
        (trim(pictur), ok, trim(errmsg))
    }

    /// Trace of a 3x3 matrix
    ///
    /// Return the trace of a 3x3 matrix.
    ///
    /// See [`trace`](raw::trace) for full documentation.
    pub fn trace(&self, matrix: &[[f64; 3]; 3]) -> f64 {
        raw::trace(matrix)
    }

    /// Trace of a matrix, general dimension
    ///
    /// Return the trace of a square matrix of arbitrary dimension.
    ///
    /// See [`traceg`](raw::traceg) for full documentation.
    pub fn traceg(&self, matrix: &[f64], ndim: i32) -> f64 {
        raw::traceg(matrix, ndim)
    }

    /// Module Check In
    ///
    /// Inform the SPICELIB error handling mechanism of entry into a
    /// routine.
    ///
    /// See [`chkin`](raw::chkin) for full documentation.
    pub fn chkin(&mut self, module: &str) -> Result<()> {
        raw::chkin(self, module)?;
        Ok(())
    }

    /// Module Check Out
    ///
    /// Inform the SPICELIB error handling mechanism of exit from a
    /// routine.
    ///
    /// See [`chkout`](raw::chkout) for full documentation.
    pub fn chkout(&mut self, module: &str) -> Result<()> {
        raw::chkout(self, module)?;
        Ok(())
    }

    /// Traceback depth
    ///
    /// Return the number of modules in the traceback representation.
    ///
    /// Returns `depth`.
    ///
    /// See [`trcdep`](raw::trcdep) for full documentation.
    pub fn trcdep(&mut self) -> i32 {
        let mut depth: i32 = Default::default();
        raw::trcdep(self, &mut depth);
        depth
    }

    /// Maximum traceback depth encountered.
    ///
    /// Return the maximum number of modules encountered in the
    /// traceback so far.
    ///
    /// Returns `depth`.
    ///
    /// See [`trcmxd`](raw::trcmxd) for full documentation.
    pub fn trcmxd(&mut self) -> i32 {
        let mut depth: i32 = Default::default();
        raw::trcmxd(self, &mut depth);
        depth
    }

    /// Get Module Name from Traceback
    ///
    /// Return the name of the module having the specified position in
    /// the trace representation. The first module to check in is at
    /// position 1.
    ///
    /// Returns `name`.
    ///
    /// See [`trcnam`](raw::trcnam) for full documentation.
    pub fn trcnam(&mut self, index: i32) -> Result<String> {
        let mut name = blank(255);
        raw::trcnam(self, index, &mut name)?;
        Ok(trim(name))
    }

    /// Get Quick Traceback
    ///
    /// Return a string containing a traceback.
    ///
    /// Returns `trace`.
    ///
    /// See [`qcktrc`](raw::qcktrc) for full documentation.
    pub fn qcktrc(&mut self) -> String {
        let mut trace = blank(1200);
        raw::qcktrc(self, &mut trace);
        trim(trace)
    }

    /// Turn tracing off
    ///
    /// Disable tracing.
    ///
    /// See [`trcoff`](raw::trcoff) for full documentation.
    pub fn trcoff(&mut self) {
        raw::trcoff(self);
    }

    /// Separation quantity from observer
    ///
    /// Compute the angular separation in radians between two spherical
    /// or point objects.
    ///
    /// See [`trgsep`](raw::trgsep) for full documentation.
    pub fn trgsep(
        &mut self,
        et: f64,
        targ1: &str,
        shape1: &str,
        frame1: &str,
        targ2: &str,
        shape2: &str,
        frame2: &str,
        obsrvr: &str,
        abcorr: &str,
    ) -> Result<f64> {
        raw::trgsep(
            self, et, targ1, shape1, frame1, targ2, shape2, frame2, obsrvr, abcorr,
        )
    }

    /// Time transformation
    ///
    /// Transform a time vector from one representation and system
    /// to another.
    ///
    /// See [`ttrans`](raw::ttrans) for full documentation.
    pub fn ttrans(&mut self, from: &str, to: &str, tvec: &mut [f64; 7]) -> Result<()> {
        raw::ttrans(self, from, to, tvec)?;
        Ok(())
    }

    /// Twice the value of pi
    ///
    /// Return twice the value of pi (the ratio of the circumference of
    /// a circle to its diameter).
    ///
    /// See [`twopi`](raw::twopi) for full documentation.
    pub fn twopi(&mut self) -> f64 {
        raw::twopi(self)
    }

    /// Two vectors defining an orthonormal frame
    ///
    /// Find the transformation to the right-handed frame having a
    /// given vector as a specified axis and having a second given
    /// vector lying in a specified coordinate plane.
    ///
    /// Returns `mout`.
    ///
    /// See [`twovec`](raw::twovec) for full documentation.
    pub fn twovec(
        &mut self,
        axdef: &[f64; 3],
        indexa: i32,
        plndef: &[f64; 3],
        indexp: i32,
    ) -> Result<[[f64; 3]; 3]> {
        let mut mout: [[f64; 3]; 3] = Default::default();
        raw::twovec(self, axdef, indexa, plndef, indexp, &mut mout)?;
        Ok(mout)
    }

    /// Two states defining a frame transformation
    ///
    /// Find the state transformation from a base frame to the
    /// right-handed frame defined by two state vectors: one state
    /// vector defining a specified axis and a second state vector
    /// defining a specified coordinate plane.
    ///
    /// Returns `xform`.
    ///
    /// See [`twovxf`](raw::twovxf) for full documentation.
    pub fn twovxf(
        &mut self,
        axdef: &[f64; 6],
        indexa: i32,
        plndef: &[f64; 6],
        indexp: i32,
    ) -> Result<[[f64; 6]; 6]> {
        let mut xform: [[f64; 6]; 6] = Default::default();
        raw::twovxf(self, axdef, indexa, plndef, indexp, &mut xform)?;
        Ok(xform)
    }

    /// Text file, open new
    ///
    /// Open a new text file for subsequent write access.
    ///
    /// Returns `unit`.
    ///
    /// See [`txtopn`](raw::txtopn) for full documentation.
    pub fn txtopn(&mut self, fname: &str) -> Result<i32> {
        let mut unit: i32 = Default::default();
        raw::txtopn(self, fname, &mut unit)?;
        Ok(unit)
    }

    /// Text file, open for read
    ///
    /// Open a text file for read access.
    ///
    /// Returns `unit`.
    ///
    /// See [`txtopr`](raw::txtopr) for full documentation.
    pub fn txtopr(&mut self, fname: &str) -> Result<i32> {
        let mut unit: i32 = Default::default();
        raw::txtopr(self, fname, &mut unit)?;
        Ok(unit)
    }

    /// Seconds per tropical year
    ///
    /// Return the number of seconds in a tropical year.
    ///
    /// See [`tyear`](raw::tyear) for full documentation.
    pub fn tyear(&self) -> f64 {
        raw::tyear()
    }

    /// Unitized cross product, 3x3
    ///
    /// Compute the normalized cross product of two 3-vectors.
    ///
    /// Returns `vout`.
    ///
    /// See [`ucrss`](raw::ucrss) for full documentation.
    pub fn ucrss(&self, v1: &[f64; 3], v2: &[f64; 3]) -> [f64; 3] {
        let mut vout: [f64; 3] = Default::default();
        raw::ucrss(v1, v2, &mut vout);
        vout
    }

    /// Derivative of function less than zero, df(x)/dx \< 0
    ///
    /// Return .TRUE. if the derivative of the callback function UDFUNC
    /// at a given abscissa value is negative.
    ///
    /// Returns `isdecr`.
    ///
    /// See [`uddc`](raw::uddc) for full documentation.
    pub fn uddc(
        &mut self,
        udfunc: fn(&mut f64, &mut f64, &mut Context) -> f2rust_std::Result<()>,
        x: f64,
        dx: f64,
    ) -> Result<bool> {
        let mut isdecr: bool = Default::default();
        raw::uddc(self, udfunc, x, dx, &mut isdecr)?;
        Ok(isdecr)
    }

    /// First derivative of a function, df(x)/dx
    ///
    /// Calculate the first derivative of a caller-specified scalar
    /// function using a three-point estimation.
    ///
    /// Returns `deriv`.
    ///
    /// See [`uddf`](raw::uddf) for full documentation.
    pub fn uddf(
        &mut self,
        udfunc: fn(&mut f64, &mut f64, &mut Context) -> f2rust_std::Result<()>,
        x: f64,
        dx: f64,
    ) -> Result<f64> {
        let mut deriv: f64 = Default::default();
        raw::uddf(self, udfunc, x, dx, &mut deriv)?;
        Ok(deriv)
    }

    /// Dummy function for UDFUNS
    ///
    /// Serve as a dummy function for GF routines expecting an UDFUNS
    /// argument. It is a no-op routine with an argument signature
    /// matching UDFUNS.
    ///
    /// See [`udf`](raw::udf) for full documentation.
    pub fn udf(&mut self, x: &mut f64, value: &mut f64) -> Result<()> {
        raw::udf(self, x, value)?;
        Ok(())
    }

    /// Union two character sets
    ///
    /// Compute the union of two character sets to form a third set.
    ///
    /// See [`unionc`](raw::unionc) for full documentation.
    pub fn unionc(&mut self, a: &CharCell, b: &CharCell, c: &mut CharCell) -> Result<()> {
        raw::unionc(self, a.as_arg(), b.as_arg(), c.as_arg_mut())?;
        Ok(())
    }

    /// Union two double precision sets
    ///
    /// Compute the union of two double precision sets to form a third
    /// set.
    ///
    /// See [`uniond`](raw::uniond) for full documentation.
    pub fn uniond(&mut self, a: &Cell<f64>, b: &Cell<f64>, c: &mut Cell<f64>) -> Result<()> {
        raw::uniond(
            self,
            a.as_raw_slice(),
            b.as_raw_slice(),
            c.as_raw_mut_slice(),
        )?;
        Ok(())
    }

    /// Union two integer sets
    ///
    /// Compute the union of two integer sets to form a third set.
    ///
    /// See [`unioni`](raw::unioni) for full documentation.
    pub fn unioni(&mut self, a: &Cell<i32>, b: &Cell<i32>, c: &mut Cell<i32>) -> Result<()> {
        raw::unioni(
            self,
            a.as_raw_slice(),
            b.as_raw_slice(),
            c.as_raw_mut_slice(),
        )?;
        Ok(())
    }

    /// Uniform time scale transformation
    ///
    /// Transform time from one uniform scale to another. The uniform
    /// time scales are TAI, GPS, TT, TDT, TDB, ET, JED, JDTDB, JDTDT.
    ///
    /// See [`unitim`](raw::unitim) for full documentation.
    pub fn unitim(&mut self, epoch: f64, insys: &str, outsys: &str) -> Result<f64> {
        raw::unitim(self, epoch, insys, outsys)
    }

    /// Unit vector and norm, 3 dimensional
    ///
    /// Normalize a double precision 3-vector and return its magnitude.
    ///
    /// Returns `(vout, vmag)`.
    ///
    /// See [`unorm`](raw::unorm) for full documentation.
    pub fn unorm(&self, v1: &[f64; 3]) -> ([f64; 3], f64) {
        let mut vout: [f64; 3] = Default::default();
        let mut vmag: f64 = Default::default();
        raw::unorm(v1, &mut vout, &mut vmag);
        (vout, vmag)
    }

    /// Unit vector and norm, general dimension
    ///
    /// Normalize a double precision vector of arbitrary dimension and
    /// return its magnitude.
    ///
    /// Returns `(vout, vmag)`.
    ///
    /// See [`unormg`](raw::unormg) for full documentation.
    pub fn unormg(&self, v1: &[f64], ndim: i32) -> (Vec<f64>, f64) {
        let mut vout: Vec<f64> = vec![Default::default(); ndim.max(0) as usize];
        let mut vmag: f64 = Default::default();
        raw::unormg(v1, ndim, &mut vout, &mut vmag);
        (vout, vmag)
    }

    /// UTC to Ephemeris Time
    ///
    /// Convert an input time from Calendar or Julian Date format, UTC,
    /// to ephemeris seconds past J2000.
    ///
    /// Returns `et`.
    ///
    /// See [`utc2et`](raw::utc2et) for full documentation.
    pub fn utc2et(&mut self, utcstr: &str) -> Result<f64> {
        let mut et: f64 = Default::default();
        raw::utc2et(self, utcstr, &mut et)?;
        Ok(et)
    }

    /// Vector addition, 3 dimensional
    ///
    /// Add two double precision 3-dimensional vectors.
    ///
    /// Returns `vout`.
    ///
    /// See [`vadd`](raw::vadd) for full documentation.
    pub fn vadd(&self, v1: &[f64; 3], v2: &[f64; 3]) -> [f64; 3] {
        let mut vout: [f64; 3] = Default::default();
        raw::vadd(v1, v2, &mut vout);
        vout
    }

    /// Vector addition, general dimension
    ///
    /// Add two vectors of arbitrary dimension.
    ///
    /// Returns `vout`.
    ///
    /// See [`vaddg`](raw::vaddg) for full documentation.
    pub fn vaddg(&self, v1: &[f64], v2: &[f64], ndim: i32) -> Vec<f64> {
        let mut vout: Vec<f64> = vec![Default::default(); ndim.max(0) as usize];
        raw::vaddg(v1, v2, ndim, &mut vout);
        vout
    }

    /// Validate a character set
    ///
    /// Create a valid set from a character set array.
    ///
    /// See [`validc`](raw::validc) for full documentation.
    pub fn validc(&mut self, size: i32, n: i32, a: &mut CharCell) -> Result<()> {
        raw::validc(self, size, n, a.as_arg_mut())?;
        Ok(())
    }

    /// Validate a double precision set
    ///
    /// Create a valid set from a double precision set array.
    ///
    /// See [`validd`](raw::validd) for full documentation.
    pub fn validd(&mut self, size: i32, n: i32, a: &mut Cell<f64>) -> Result<()> {
        raw::validd(self, size, n, a.as_raw_mut_slice())?;
        Ok(())
    }

    /// Validate an integer set
    ///
    /// Create a valid set from an integer set array.
    ///
    /// See [`validi`](raw::validi) for full documentation.
    pub fn validi(&mut self, size: i32, n: i32, a: &mut Cell<i32>) -> Result<()> {
        raw::validi(self, size, n, a.as_raw_mut_slice())?;
        Ok(())
    }

    /// Vector cross product, 3 dimensions
    ///
    /// Compute the cross product of two 3-dimensional vectors.
    ///
    /// Returns `vout`.
    ///
    /// See [`vcrss`](raw::vcrss) for full documentation.
    pub fn vcrss(&self, v1: &[f64; 3], v2: &[f64; 3]) -> [f64; 3] {
        let mut vout: [f64; 3] = Default::default();
        raw::vcrss(v1, v2, &mut vout);
        vout
    }

    /// Vector distance
    ///
    /// Return the distance between two three-dimensional vectors.
    ///
    /// See [`vdist`](raw::vdist) for full documentation.
    pub fn vdist(&self, v1: &[f64; 3], v2: &[f64; 3]) -> f64 {
        raw::vdist(v1, v2)
    }

    /// Vector distance, general dimension
    ///
    /// Return the distance between two vectors of arbitrary dimension.
    ///
    /// See [`vdistg`](raw::vdistg) for full documentation.
    pub fn vdistg(&self, v1: &[f64], v2: &[f64], ndim: i32) -> f64 {
        raw::vdistg(v1, v2, ndim)
    }

    /// Vector dot product, 3 dimensions
    ///
    /// Compute the dot product of two double precision, 3-dimensional
    /// vectors.
    ///
    /// See [`vdot`](raw::vdot) for full documentation.
    pub fn vdot(&self, v1: &[f64; 3], v2: &[f64; 3]) -> f64 {
        raw::vdot(v1, v2)
    }

    /// Vector dot product, general dimension
    ///
    /// Compute the dot product of two vectors of arbitrary dimension.
    ///
    /// See [`vdotg`](raw::vdotg) for full documentation.
    pub fn vdotg(&self, v1: &[f64], v2: &[f64], ndim: i32) -> f64 {
        raw::vdotg(v1, v2, ndim)
    }

    /// Vector equality, 3 dimensions
    ///
    /// Make one double precision 3-dimensional vector equal to
    /// another.
    ///
    /// Returns `vout`.
    ///
    /// See [`vequ`](raw::vequ) for full documentation.
    pub fn vequ(&self, vin: &[f64; 3]) -> [f64; 3] {
        let mut vout: [f64; 3] = Default::default();
        raw::vequ(vin, &mut vout);
        vout
    }

    /// Vector equality, general dimension
    ///
    /// Make one double precision vector of arbitrary dimension equal
    /// to another.
    ///
    /// Returns `vout`.
    ///
    /// See [`vequg`](raw::vequg) for full documentation.
    pub fn vequg(&self, vin: &[f64], ndim: i32) -> Vec<f64> {
        let mut vout: Vec<f64> = vec![Default::default(); ndim.max(0) as usize];
        raw::vequg(vin, ndim, &mut vout);
        vout
    }

    /// "V-Hat", unit vector along V, 3 dimensions
    ///
    /// Find the unit vector along a double precision 3-dimensional
    /// vector.
    ///
    /// Returns `vout`.
    ///
    /// See [`vhat`](raw::vhat) for full documentation.
    pub fn vhat(&self, v1: &[f64; 3]) -> [f64; 3] {
        let mut vout: [f64; 3] = Default::default();
        raw::vhat(v1, &mut vout);
        vout
    }

    /// "V-Hat", unit vector along V, general dimension
    ///
    /// Find the unit vector along a double precision vector of
    /// arbitrary dimension.
    ///
    /// Returns `vout`.
    ///
    /// See [`vhatg`](raw::vhatg) for full documentation.
    pub fn vhatg(&self, v1: &[f64], ndim: i32) -> Vec<f64> {
        let mut vout: Vec<f64> = vec![Default::default(); ndim.max(0) as usize];
        raw::vhatg(v1, ndim, &mut vout);
        vout
    }

    /// "V-Hat", 3-d unit vector along V, in place
    ///
    /// Scale a three-dimensional vector to unit length.
    ///
    /// See [`vhatip`](raw::vhatip) for full documentation.
    pub fn vhatip(&self, v: &mut [f64; 3]) {
        raw::vhatip(v);
    }

    /// Vector linear combination, 3 dimensions
    ///
    /// Compute a vector linear combination of two double precision,
    /// 3-dimensional vectors.
    ///
    /// Returns `sum`.
    ///
    /// See [`vlcom`](raw::vlcom) for full documentation.
    pub fn vlcom(&self, a: f64, v1: &[f64; 3], b: f64, v2: &[f64; 3]) -> [f64; 3] {
        let mut sum: [f64; 3] = Default::default();
        raw::vlcom(a, v1, b, v2, &mut sum);
        sum
    }

    /// Vector linear combination, 3 dimensions
    ///
    /// Compute the vector linear combination of three double precision
    /// 3-dimensional vectors.
    ///
    /// Returns `sum`.
    ///
    /// See [`vlcom3`](raw::vlcom3) for full documentation.
    pub fn vlcom3(
        &self,
        a: f64,
        v1: &[f64; 3],
        b: f64,
        v2: &[f64; 3],
        c: f64,
        v3: &[f64; 3],
    ) -> [f64; 3] {
        let mut sum: [f64; 3] = Default::default();
        raw::vlcom3(a, v1, b, v2, c, v3, &mut sum);
        sum
    }

    /// Vector linear combination, general dimension
    ///
    /// Compute a vector linear combination of two double precision
    /// vectors of arbitrary dimension.
    ///
    /// Returns `sum`.
    ///
    /// See [`vlcomg`](raw::vlcomg) for full documentation.
    pub fn vlcomg(&self, n: i32, a: f64, v1: &[f64], b: f64, v2: &[f64]) -> Vec<f64> {
        let mut sum: Vec<f64> = vec![Default::default(); n.max(0) as usize];
        raw::vlcomg(n, a, v1, b, v2, &mut sum);
        sum
    }

    /// Negate vector, "-V", general dimension
    ///
    /// Negate a double precision vector of arbitrary dimension.
    ///
    /// Returns `vout`.
    ///
    /// See [`vminug`](raw::vminug) for full documentation.
    pub fn vminug(&self, vin: &[f64], ndim: i32) -> Vec<f64> {
        let mut vout: Vec<f64> = vec![Default::default(); ndim.max(0) as usize];
        raw::vminug(vin, ndim, &mut vout);
        vout
    }

    /// Negate vector, "-V", 3 dimensions
    ///
    /// Negate a double precision 3-dimensional vector.
    ///
    /// Returns `vout`.
    ///
    /// See [`vminus`](raw::vminus) for full documentation.
    pub fn vminus(&self, v1: &[f64; 3]) -> [f64; 3] {
        let mut vout: [f64; 3] = Default::default();
        raw::vminus(v1, &mut vout);
        vout
    }

    /// Vector norm, 3 dimensions
    ///
    /// Compute the magnitude of a double precision 3-dimensional
    /// vector.
    ///
    /// See [`vnorm`](raw::vnorm) for full documentation.
    pub fn vnorm(&self, v1: &[f64; 3]) -> f64 {
        raw::vnorm(v1)
    }

    /// Vector norm, general dimension
    ///
    /// Compute the magnitude of a double precision vector of arbitrary
    /// dimension.
    ///
    /// See [`vnormg`](raw::vnormg) for full documentation.
    pub fn vnormg(&self, v1: &[f64], ndim: i32) -> f64 {
        raw::vnormg(v1, ndim)
    }

    /// Pack three scalar components into a vector
    ///
    /// Pack three scalar components into a vector.
    ///
    /// Returns `v`.
    ///
    /// See [`vpack`](raw::vpack) for full documentation.
    pub fn vpack(&self, x: f64, y: f64, z: f64) -> [f64; 3] {
        let mut v: [f64; 3] = Default::default();
        raw::vpack(x, y, z, &mut v);
        v
    }

    /// Perpendicular component of a 3-vector
    ///
    /// Find the component of a vector that is perpendicular to a second
    /// vector. All vectors are 3-dimensional.
    ///
    /// Returns `p`.
    ///
    /// See [`vperp`](raw::vperp) for full documentation.
    pub fn vperp(&self, a: &[f64; 3], b: &[f64; 3]) -> [f64; 3] {
        let mut p: [f64; 3] = Default::default();
        raw::vperp(a, b, &mut p);
        p
    }

    /// Vector projection onto plane
    ///
    /// Project a vector onto a specified plane, orthogonally.
    ///
    /// Returns `vout`.
    ///
    /// See [`vprjp`](raw::vprjp) for full documentation.
    pub fn vprjp(&mut self, vin: &[f64; 3], plane: &[f64; 4]) -> Result<[f64; 3]> {
        let mut vout: [f64; 3] = Default::default();
        raw::vprjp(self, vin, plane, &mut vout)?;
        Ok(vout)
    }

    /// Vector projection onto plane, inverted
    ///
    /// Find the vector in a specified plane that maps to a specified
    /// vector in another plane under orthogonal projection.
    ///
    /// Returns `vout`.
    ///
    /// See [`vprjpi`](raw::vprjpi) for full documentation.
    pub fn vprjpi(
        &mut self,
        vin: &[f64; 3],
        projpl: &[f64; 4],
        invpl: &[f64; 4],
    ) -> Result<Option<[f64; 3]>> {
        let mut vout: [f64; 3] = Default::default();
        let mut found: bool = Default::default();
        raw::vprjpi(self, vin, projpl, invpl, &mut vout, &mut found)?;
        Ok(if found { Some(vout) } else { None })
    }

    /// Vector projection, 3 dimensions
    ///
    /// Compute the projection of one 3-dimensional vector onto another
    /// 3-dimensional vector.
    ///
    /// Returns `p`.
    ///
    /// See [`vproj`](raw::vproj) for full documentation.
    pub fn vproj(&self, a: &[f64; 3], b: &[f64; 3]) -> [f64; 3] {
        let mut p: [f64; 3] = Default::default();
        raw::vproj(a, b, &mut p);
        p
    }

    /// Vector projection, general dimension
    ///
    /// Compute the projection of one vector onto another vector. All
    /// vectors are of arbitrary dimension.
    ///
    /// Returns `p`.
    ///
    /// See [`vprojg`](raw::vprojg) for full documentation.
    pub fn vprojg(&self, a: &[f64], b: &[f64], ndim: i32) -> Vec<f64> {
        let mut p: Vec<f64> = vec![Default::default(); ndim.max(0) as usize];
        raw::vprojg(a, b, ndim, &mut p);
        p
    }

    /// Vector relative difference, 3 dimensions
    ///
    /// Return the relative difference between two 3-dimensional vectors.
    ///
    /// See [`vrel`](raw::vrel) for full documentation.
    pub fn vrel(&self, v1: &[f64; 3], v2: &[f64; 3]) -> f64 {
        raw::vrel(v1, v2)
    }

    /// Vector relative difference, general dimension
    ///
    /// Return the relative difference between two vectors of general
    /// dimension.
    ///
    /// See [`vrelg`](raw::vrelg) for full documentation.
    pub fn vrelg(&self, v1: &[f64], v2: &[f64], ndim: i32) -> f64 {
        raw::vrelg(v1, v2, ndim)
    }

    /// Vector rotation about an axis
    ///
    /// Rotate a vector about a specified axis vector by a specified
    /// angle and return the rotated vector.
    ///
    /// Returns `r`.
    ///
    /// See [`vrotv`](raw::vrotv) for full documentation.
    pub fn vrotv(&self, v: &[f64; 3], axis: &[f64; 3], theta: f64) -> [f64; 3] {
        let mut r: [f64; 3] = Default::default();
        raw::vrotv(v, axis, theta, &mut r);
        r
    }

    /// Vector scaling, 3 dimensions
    ///
    /// Multiply a scalar and a double precision 3-dimensional vector.
    ///
    /// Returns `vout`.
    ///
    /// See [`vscl`](raw::vscl) for full documentation.
    pub fn vscl(&self, s: f64, v1: &[f64; 3]) -> [f64; 3] {
        let mut vout: [f64; 3] = Default::default();
        raw::vscl(s, v1, &mut vout);
        vout
    }

    /// Vector scaling, general dimension
    ///
    /// Multiply a scalar and a double precision vector of arbitrary
    /// dimension.
    ///
    /// Returns `vout`.
    ///
    /// See [`vsclg`](raw::vsclg) for full documentation.
    pub fn vsclg(&self, s: f64, v1: &[f64], ndim: i32) -> Vec<f64> {
        let mut vout: Vec<f64> = vec![Default::default(); ndim.max(0) as usize];
        raw::vsclg(s, v1, ndim, &mut vout);
        vout
    }

    /// Vector scaling, 3 dimensions, in place
    ///
    /// Multiply a scalar and a 3-dimensional double precision vector,
    /// replacing the input vector with the result.
    ///
    /// See [`vsclip`](raw::vsclip) for full documentation.
    pub fn vsclip(&self, s: f64, v: &mut [f64; 3]) {
        raw::vsclip(s, v);
    }

    /// Angular separation of vectors, 3 dimensions
    ///
    /// Find the separation angle in radians between two double
    /// precision, 3-dimensional vectors. This angle is defined as zero
    /// if either vector is zero.
    ///
    /// See [`vsep`](raw::vsep) for full documentation.
    pub fn vsep(&mut self, v1: &[f64; 3], v2: &[f64; 3]) -> f64 {
        raw::vsep(self, v1, v2)
    }

    /// Angular separation of vectors, general dimension
    ///
    /// Find the separation angle in radians between two double precision
    /// vectors of arbitrary dimension. This angle is defined as zero if
    /// either vector is zero.
    ///
    /// See [`vsepg`](raw::vsepg) for full documentation.
    pub fn vsepg(&mut self, v1: &[f64], v2: &[f64], ndim: i32) -> f64 {
        raw::vsepg(self, v1, v2, ndim)
    }

    /// Vector subtraction, 3 dimensions
    ///
    /// Compute the difference between two double precision 3-dimensional
    /// vectors.
    ///
    /// Returns `vout`.
    ///
    /// See [`vsub`](raw::vsub) for full documentation.
    pub fn vsub(&self, v1: &[f64; 3], v2: &[f64; 3]) -> [f64; 3] {
        let mut vout: [f64; 3] = Default::default();
        raw::vsub(v1, v2, &mut vout);
        vout
    }

    /// Vector subtraction, general dimension
    ///
    /// Compute the difference between two double precision vectors of
    /// arbitrary dimension.
    ///
    /// Returns `vout`.
    ///
    /// See [`vsubg`](raw::vsubg) for full documentation.
    pub fn vsubg(&self, v1: &[f64], v2: &[f64], ndim: i32) -> Vec<f64> {
        let mut vout: Vec<f64> = vec![Default::default(); ndim.max(0) as usize];
        raw::vsubg(v1, v2, ndim, &mut vout);
        vout
    }

    /// Vector transpose times matrix times vector, 3 dim
    ///
    /// Multiply the transpose of a 3-dimensional column vector,
    /// a 3x3 matrix, and a 3-dimensional column vector.
    ///
    /// See [`vtmv`](raw::vtmv) for full documentation.
    pub fn vtmv(&self, v1: &[f64; 3], matrix: &[[f64; 3]; 3], v2: &[f64; 3]) -> f64 {
        raw::vtmv(v1, matrix, v2)
    }

    /// Vector transpose times matrix times vector
    ///
    /// Multiply the transpose of a n-dimensional column vector,
    /// a nxm matrix, and a m-dimensional column vector.
    ///
    /// See [`vtmvg`](raw::vtmvg) for full documentation.
    pub fn vtmvg(&self, v1: &[f64], matrix: &[f64], v2: &[f64], nrow: i32, ncol: i32) -> f64 {
        raw::vtmvg(v1, matrix, v2, nrow, ncol)
    }

    /// Unpack three scalar components from a vector
    ///
    /// Unpack three scalar components from a vector.
    ///
    /// Returns `(x, y, z)`.
    ///
    /// See [`vupack`](raw::vupack) for full documentation.
    pub fn vupack(&self, v: &[f64; 3]) -> (f64, f64, f64) {
        let mut x: f64 = Default::default();
        let mut y: f64 = Default::default();
        let mut z: f64 = Default::default();
        raw::vupack(v, &mut x, &mut y, &mut z);
        (x, y, z)
    }

    /// Is a vector the zero vector?
    ///
    /// Indicate whether a 3-vector is the zero vector.
    ///
    /// See [`vzero`](raw::vzero) for full documentation.
    pub fn vzero(&self, v: &[f64; 3]) -> bool {
        raw::vzero(v)
    }

    /// Is a vector the zero vector? -- general dim.
    ///
    /// Indicate whether an n-dimensional vector is the zero vector.
    ///
    /// See [`vzerog`](raw::vzerog) for full documentation.
    pub fn vzerog(&self, v: &[f64], ndim: i32) -> bool {
        raw::vzerog(v, ndim)
    }

    /// Word Count
    ///
    /// Return the number of words in a string.
    ///
    /// See [`wdcnt`](raw::wdcnt) for full documentation.
    pub fn wdcnt(&self, string: &str) -> i32 {
        raw::wdcnt(string)
    }

    /// Index of a Word Within a String
    ///
    /// Find the index of a word within a string. If the word does not
    /// exist as a word within the string, the value zero is returned.
    ///
    /// See [`wdindx`](raw::wdindx) for full documentation.
    pub fn wdindx(&self, string: &str, word: &str) -> i32 {
        raw::wdindx(string, word)
    }

    /// Cardinality of a double precision window
    ///
    /// Return the cardinality (number of intervals) of a double
    /// precision window.
    ///
    /// See [`wncard`](raw::wncard) for full documentation.
    pub fn wncard(&mut self, window: &Cell<f64>) -> Result<i32> {
        raw::wncard(self, window.as_raw_slice())
    }

    /// Complement a DP window
    ///
    /// Determine the complement of a double precision window with
    /// respect to the interval \[LEFT,RIGHT].
    ///
    /// See [`wncomd`](raw::wncomd) for full documentation.
    pub fn wncomd(
        &mut self,
        left: f64,
        right: f64,
        window: &Cell<f64>,
        result: &mut Cell<f64>,
    ) -> Result<()> {
        raw::wncomd(
            self,
            left,
            right,
            window.as_raw_slice(),
            result.as_raw_mut_slice(),
        )?;
        Ok(())
    }

    /// Contract the intervals of a DP window
    ///
    /// Contract each of the intervals of a double precision window.
    ///
    /// See [`wncond`](raw::wncond) for full documentation.
    pub fn wncond(&mut self, left: f64, right: f64, window: &mut Cell<f64>) -> Result<()> {
        raw::wncond(self, left, right, window.as_raw_mut_slice())?;
        Ok(())
    }

    /// Difference two DP windows
    ///
    /// Place the difference of two double precision windows into
    /// a third window.
    ///
    /// See [`wndifd`](raw::wndifd) for full documentation.
    pub fn wndifd(&mut self, a: &Cell<f64>, b: &Cell<f64>, c: &mut Cell<f64>) -> Result<()> {
        raw::wndifd(
            self,
            a.as_raw_slice(),
            b.as_raw_slice(),
            c.as_raw_mut_slice(),
        )?;
        Ok(())
    }

    /// Element of a DP window
    ///
    /// Determine whether a point is an element of a double precision
    /// window.
    ///
    /// See [`wnelmd`](raw::wnelmd) for full documentation.
    pub fn wnelmd(&mut self, point: f64, window: &Cell<f64>) -> Result<bool> {
        raw::wnelmd(self, point, window.as_raw_slice())
    }

    /// Expand the intervals of a DP window
    ///
    /// Expand each of the intervals of a double precision window.
    ///
    /// See [`wnexpd`](raw::wnexpd) for full documentation.
    pub fn wnexpd(&mut self, left: f64, right: f64, window: &mut Cell<f64>) -> Result<()> {
        raw::wnexpd(self, left, right, window.as_raw_mut_slice())?;
        Ok(())
    }

    /// Extract the endpoints from a DP window
    ///
    /// Extract the left or right endpoints from a double precision
    /// window.
    ///
    /// See [`wnextd`](raw::wnextd) for full documentation.
    pub fn wnextd(&mut self, side: char, window: &mut Cell<f64>) -> Result<()> {
        raw::wnextd(self, side, window.as_raw_mut_slice())?;
        Ok(())
    }

    /// Fetch an interval from a DP window
    ///
    /// Fetch a particular interval from a double precision window.
    ///
    /// Returns `(left, right)`.
    ///
    /// See [`wnfetd`](raw::wnfetd) for full documentation.
    pub fn wnfetd(&mut self, window: &Cell<f64>, n: i32) -> Result<(f64, f64)> {
        let mut left: f64 = Default::default();
        let mut right: f64 = Default::default();
        raw::wnfetd(self, window.as_raw_slice(), n, &mut left, &mut right)?;
        Ok((left, right))
    }

    /// Fill small gaps in a DP window
    ///
    /// Fill small gaps between adjacent intervals of a double precision
    /// window.
    ///
    /// See [`wnfild`](raw::wnfild) for full documentation.
    pub fn wnfild(&mut self, smlgap: f64, window: &mut Cell<f64>) -> Result<()> {
        raw::wnfild(self, smlgap, window.as_raw_mut_slice())?;
        Ok(())
    }

    /// Filter small intervals from a DP window
    ///
    /// Filter (remove) small intervals from a double precision window.
    ///
    /// See [`wnfltd`](raw::wnfltd) for full documentation.
    pub fn wnfltd(&mut self, smlint: f64, window: &mut Cell<f64>) -> Result<()> {
        raw::wnfltd(self, smlint, window.as_raw_mut_slice())?;
        Ok(())
    }

    /// Included in a double precision window
    ///
    /// Determine whether an interval is included in a double precision
    /// window.
    ///
    /// See [`wnincd`](raw::wnincd) for full documentation.
    pub fn wnincd(&mut self, left: f64, right: f64, window: &Cell<f64>) -> Result<bool> {
        raw::wnincd(self, left, right, window.as_raw_slice())
    }

    /// Insert an interval into a DP window
    ///
    /// Insert an interval into a double precision window.
    ///
    /// See [`wninsd`](raw::wninsd) for full documentation.
    pub fn wninsd(&mut self, left: f64, right: f64, window: &mut Cell<f64>) -> Result<()> {
        raw::wninsd(self, left, right, window.as_raw_mut_slice())?;
        Ok(())
    }

    /// Intersect two DP windows
    ///
    /// Place the intersection of two double precision windows into
    /// a third window.
    ///
    /// See [`wnintd`](raw::wnintd) for full documentation.
    pub fn wnintd(&mut self, a: &Cell<f64>, b: &Cell<f64>, c: &mut Cell<f64>) -> Result<()> {
        raw::wnintd(
            self,
            a.as_raw_slice(),
            b.as_raw_slice(),
            c.as_raw_mut_slice(),
        )?;
        Ok(())
    }

    /// Compare two DP windows
    ///
    /// Compare two double precision windows.
    ///
    /// See [`wnreld`](raw::wnreld) for full documentation.
    pub fn wnreld(&mut self, a: &Cell<f64>, op: &str, b: &Cell<f64>) -> Result<bool> {
        raw::wnreld(self, a.as_raw_slice(), op, b.as_raw_slice())
    }

    /// Summary of a double precision window
    ///
    /// Summarize the contents of a double precision window.
    ///
    /// Returns `(meas, avg, stddev, idxsml, idxlon)`.
    ///
    /// See [`wnsumd`](raw::wnsumd) for full documentation.
    pub fn wnsumd(&mut self, window: &Cell<f64>) -> Result<(f64, f64, f64, i32, i32)> {
        let mut meas: f64 = Default::default();
        let mut avg: f64 = Default::default();
        let mut stddev: f64 = Default::default();
        let mut idxsml: i32 = Default::default();
        let mut idxlon: i32 = Default::default();
        raw::wnsumd(
            self,
            window.as_raw_slice(),
            &mut meas,
            &mut avg,
            &mut stddev,
            &mut idxsml,
            &mut idxlon,
        )?;
        Ok((meas, avg, stddev, idxsml, idxlon))
    }

    /// Union two DP windows
    ///
    /// Place the union of two double precision windows into a third
    /// window.
    ///
    /// See [`wnunid`](raw::wnunid) for full documentation.
    pub fn wnunid(&mut self, a: &Cell<f64>, b: &Cell<f64>, c: &mut Cell<f64>) -> Result<()> {
        raw::wnunid(
            self,
            a.as_raw_slice(),
            b.as_raw_slice(),
            c.as_raw_mut_slice(),
        )?;
        Ok(())
    }

    /// Validate a DP window
    ///
    /// Form a valid double precision window from the contents
    /// of a window array.
    ///
    /// See [`wnvald`](raw::wnvald) for full documentation.
    pub fn wnvald(&mut self, size: i32, n: i32, window: &mut Cell<f64>) -> Result<()> {
        raw::wnvald(self, size, n, window.as_raw_mut_slice())?;
        Ok(())
    }

    /// Write characters to text file encoded
    ///
    /// Encode and write characters to a text file.
    ///
    /// See [`wrencc`](raw::wrencc) for full documentation.
    pub fn wrencc(&mut self, unit: i32, n: i32, data: &CharVec) -> Result<()> {
        raw::wrencc(self, unit, n, data.as_arg())?;
        Ok(())
    }

    /// Write encoded d.p. numbers to text file
    ///
    /// Encode and write d.p. numbers to a text file.
    ///
    /// See [`wrencd`](raw::wrencd) for full documentation.
    pub fn wrencd(&mut self, unit: i32, n: i32, data: &[f64]) -> Result<()> {
        raw::wrencd(self, unit, n, data)?;
        Ok(())
    }

    /// Write encoded integers to text file
    ///
    /// Encode and write integers to a text file.
    ///
    /// See [`wrenci`](raw::wrenci) for full documentation.
    pub fn wrenci(&mut self, unit: i32, n: i32, data: &[i32]) -> Result<()> {
        raw::wrenci(self, unit, n, data)?;
        Ok(())
    }

    /// Write array of lines to a logical unit
    ///
    /// Write an array of text lines to a Fortran logical unit.
    ///
    /// See [`writla`](raw::writla) for full documentation.
    pub fn writla(&mut self, numlin: i32, array: &CharVec, unit: i32) -> Result<()> {
        raw::writla(self, numlin, array.as_arg(), unit)?;
        Ok(())
    }

    /// Write a text line to a logical unit
    ///
    /// Write a single line of text to the Fortran logical unit UNIT.
    ///
    /// See [`writln`](raw::writln) for full documentation.
    pub fn writln(&mut self, line: &str, unit: i32) -> Result<()> {
        raw::writln(self, line, unit)?;
        Ok(())
    }

    /// Write a variable to a kernel file
    ///
    /// Write the value of a variable in a double precision symbol
    /// table to a NAIF ASCII kernel file.
    ///
    /// See [`wrkvar`](raw::wrkvar) for full documentation.
    pub fn wrkvar(
        &mut self,
        unit: i32,
        name: &str,
        dirctv: &str,
        tabsym: &CharCell,
        tabptr: &Cell<i32>,
        tabval: &Cell<f64>,
    ) -> Result<()> {
        raw::wrkvar(
            self,
            unit,
            name,
            dirctv,
            tabsym.as_arg(),
            tabptr.as_raw_slice(),
            tabval.as_raw_slice(),
        )?;
        Ok(())
    }

    /// Write Output Line to a Device
    ///
    /// Write a character string to an output device.
    ///
    /// See [`wrline`](raw::wrline) for full documentation.
    pub fn wrline(&mut self, device: &str, line: &str) -> Result<()> {
        raw::wrline(self, device, line)?;
        Ok(())
    }

    /// Close a device
    ///
    /// Close a device.
    ///
    /// See [`clline`](raw::clline) for full documentation.
    pub fn clline(&mut self, device: &str) -> Result<()> {
        raw::clline(self, device)?;
        Ok(())
    }

    /// list voxels intersected by a ray
    ///
    /// Return a list of voxels that a given ray intersects in a given
    /// voxel grid.
    ///
    /// Returns `(nvx, voxlst)`.
    ///
    /// See [`xdda`](raw::xdda) for full documentation.
    pub fn xdda(
        &mut self,
        vertex: &[f64; 3],
        raydir: &[f64; 3],
        grdext: &[i32; 3],
        maxnvx: i32,
    ) -> Result<(i32, Vec<[i32; 3]>)> {
        let mut nvx: i32 = Default::default();
        let mut voxlst: Vec<[i32; 3]> = vec![Default::default(); maxnvx.max(0) as usize];
        raw::xdda(self, vertex, raydir, grdext, maxnvx, &mut nvx, &mut voxlst)?;
        Ok((nvx, voxlst))
    }

    /// State transformation to Euler angles
    ///
    /// Convert a state transformation matrix to Euler angles and their
    /// derivatives, given a specified set of axes.
    ///
    /// Returns `(eulang, unique)`.
    ///
    /// See [`xf2eul`](raw::xf2eul) for full documentation.
    pub fn xf2eul(
        &mut self,
        xform: &[[f64; 6]; 6],
        axisa: i32,
        axisb: i32,
        axisc: i32,
    ) -> Result<([f64; 6], bool)> {
        let mut eulang: [f64; 6] = Default::default();
        let mut unique: bool = Default::default();
        raw::xf2eul(self, xform, axisa, axisb, axisc, &mut eulang, &mut unique)?;
        Ok((eulang, unique))
    }

    /// Euler angles and derivative to transformation
    ///
    /// Compute a state transformation from an Euler angle factorization
    /// of a rotation and the derivatives of those Euler angles.
    ///
    /// Returns `xform`.
    ///
    /// See [`eul2xf`](raw::eul2xf) for full documentation.
    pub fn eul2xf(
        &mut self,
        eulang: &[f64; 6],
        axisa: i32,
        axisb: i32,
        axisc: i32,
    ) -> Result<[[f64; 6]; 6]> {
        let mut xform: [[f64; 6]; 6] = Default::default();
        raw::eul2xf(self, eulang, axisa, axisb, axisc, &mut xform)?;
        Ok(xform)
    }

    /// Transform to rotation and angular velocity
    ///
    /// Determine the rotation matrix and angular velocity of the
    /// rotation from a state transformation matrix.
    ///
    /// Returns `(rot, av)`.
    ///
    /// See [`xf2rav`](raw::xf2rav) for full documentation.
    pub fn xf2rav(&self, xform: &[[f64; 6]; 6]) -> ([[f64; 3]; 3], [f64; 3]) {
        let mut rot: [[f64; 3]; 3] = Default::default();
        let mut av: [f64; 3] = Default::default();
        raw::xf2rav(xform, &mut rot, &mut av);
        (rot, av)
    }

    /// Transform state between coordinate systems
    ///
    /// Transform a state between coordinate systems.
    ///
    /// Returns `ostate`.
    ///
    /// See [`xfmsta`](raw::xfmsta) for full documentation.
    pub fn xfmsta(
        &mut self,
        istate: &[f64; 6],
        icosys: &str,
        ocosys: &str,
        body: &str,
    ) -> Result<[f64; 6]> {
        let mut ostate: [f64; 6] = Default::default();
        raw::xfmsta(self, istate, icosys, ocosys, body, &mut ostate)?;
        Ok(ostate)
    }

    /// Transpose a matrix by blocks
    ///
    /// Transpose the square blocks within a matrix.
    ///
    /// Returns `btmat`.
    ///
    /// See [`xposbl`](raw::xposbl) for full documentation.
    pub fn xposbl(&mut self, bmat: &[f64], nrow: i32, ncol: i32, bsize: i32) -> Result<Vec<f64>> {
        let mut btmat: Vec<f64> = vec![Default::default(); (nrow * ncol).max(0) as usize];
        raw::xposbl(self, bmat, nrow, ncol, bsize, &mut btmat)?;
        Ok(btmat)
    }

    /// Transpose a matrix, 3x3
    ///
    /// Transpose a 3x3 matrix.
    ///
    /// Returns `mout`.
    ///
    /// See [`xpose`](raw::xpose) for full documentation.
    pub fn xpose(&self, m1: &[[f64; 3]; 3]) -> [[f64; 3]; 3] {
        let mut mout: [[f64; 3]; 3] = Default::default();
        raw::xpose(m1, &mut mout);
        mout
    }

    /// Transpose a matrix, general
    ///
    /// Transpose a matrix of arbitrary size (the matrix
    /// need not be square).
    ///
    /// Returns `xposem`.
    ///
    /// See [`xposeg`](raw::xposeg) for full documentation.
    pub fn xposeg(&self, matrix: &[f64], nrow: i32, ncol: i32) -> Vec<f64> {
        let mut xposem: Vec<f64> = vec![Default::default(); (nrow * ncol).max(0) as usize];
        raw::xposeg(matrix, nrow, ncol, &mut xposem);
        xposem
    }

    /// Transpose a matrix, general dimension, in place
    ///
    /// Transpose a matrix of arbitrary size and shape in place.
    ///
    /// See [`xpsgip`](raw::xpsgip) for full documentation.
    pub fn xpsgip(&self, nrow: i32, ncol: i32, matrix: &mut [f64]) {
        raw::xpsgip(nrow, ncol, matrix);
    }
}
