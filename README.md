# Detect Range

## About

Detect Range employs the radar line-of-sight (LOS) range equation using the effective Earth
radius (k-factor) model. This model is widely used in radio-frequency (RF) engineering to
estimate beyond-the-geometric-horizon line-of-sight distance by approximating atmospheric
refraction through an effective Earth radius.

The Earth's atmosphere causes RF energy to refract, resulting in downward bending of
propagating waves and an extension of the apparent radio horizon.

The radar line-of-sight distance is computed in the program as:

$$
d_{\text{LOS}} =
\sqrt{2 k R_e h_r}
+
\sqrt{2 k R_e h_t}
$$

where:

- $d_{\text{LOS}}$ = radar line-of-sight distance
- $R_e$ = true Earth radius
- $k$ = effective Earth radius factor
- $h_r$ = radar antenna height above ground level
- $h_t$ = target height above ground level

Under standard atmospheric conditions, the following values are used:

$$
k = \frac{4}{3}
$$



$$
R_e = 6371008.7714
$$


## Usage

This program is primarily designed for the aviation community and as such utilises industry standard units for entry. the default units for entry are:

- Antenna (Radar) AGL = Meters
- Target (Aircraft) AGL = Feet
- LOS Range = Nautical Miles

Nautical miles is returned as an integer value rounded down for display, all maths is computed using 64-bit floating point values utilising SI units.

### Installation

This tool is written in Rust, to build it locally, ensure a Rust toolchain is installed then clone the repository and build using Cargo:

```
git clone https://github.com/rich-davs/detect-range
cd detect-range
cargo build --release
```

The compiled binary will be located at:

```
target/release/detect-range
```


### Commands

To initiate the program in interactive terminal mode:

```
detect-range
```

this will prompt users to input Hr and Ht in the terminal before providing an answer.

Utilising arguments the format is:
```
detect-range --hr {radar height} --ht {target height}
```
Both variables hr and ht must be provided to receive as a result.

## Future Improvements

A number of planned future improvements will hopefully implemented:

- Take variable input unit types (meters/feet)
- Compare solution against a Digital Terrain Elevation model in a fixed location to determine true LoS calculations, enabling overlays on maps or data based outputs

As my first ever public programme I very much appreciate any feedback on my Rust as I continue to learn and develop, please leave comments for improvements and I will explore further features as my knowledge builds.

Cheers,

_rich-davs_

### References

*Skolnik, M. I., Radar Handbook, 3rd ed., McGraw-Hill, 2008.*


