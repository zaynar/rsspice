These examples are ported from the lessons at https://naif.jpl.nasa.gov/naif/lessons.html

The programs expect to find a `kernels` directory under the current working directory.
Download https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/Lessons/spice_lessons_f_unix.zip
and copy `lessons/*/kernels/` into the corresponding directories here, then use
`cargo run --example`:

```
$ pwd
rsspice/lessons/binary_pck
$ cargo run --example erotat
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.98s
     Running `target/debug/examples/erotat`

Earth-Moon direction using low accuracy
PCK and IAU_EARTH frame:
Moon lon (deg):             -35.496272
Moon lat (deg):              26.416959

Earth-Moon direction using high accuracy
PCK and ITRF93 frame:
Moon lon (deg):             -35.554286
Moon lat (deg):              26.419156

[...]
```
