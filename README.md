# comix

Build and format Standard Terminal Arrival (STAR) and Departure Procedure (DP) [GeoJSON](https://geojson.org/) files from formatted FAA aeronautical data

## Rationale

Named for the `COMIX` arrival into San Diego International Airport, this CLI takes point (fixes and navaids) and line (routes) for arrival and departure procedures from bespoke FAA data files and converts them into GeoJSON format. This allows for use of the data with mapping and other visualization tools.

## Usage

The `comix` application is a Rust-based executable. This means it can run on Windows, MacOS and Linux machines without the need for installing runtimes or other prerequisites.

### Installation

Visit the [Releases page](https://github.com/brianknight10/comix/releases). Download and unzip the package that is appropriate for your operating system and architecture. Place the `comix` executable wherever you'd like on your file system.

### Running the application

The application can be called using `comix`. You can see the configuration options by running `comix --help`.

```
Format STARs and DPs from FAA data

Usage: comix --input <INPUT> --output <OUTPUT> <GEOMETRY>

Arguments:
  <GEOMETRY>
          The geometry collection to create

          Possible values:
          - point: Point geometry in latitude and longitude coordinates
          - line:  LineString geometry in latitude and longitude coordinates

Options:
  -i, --input <INPUT>
          The input file path

  -o, --output <OUTPUT>
          The output file path

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

To run the application, supply the source data file as the input, an output file path, and whether you are converting points or lines.

For example, to convert FAA route data into `MultiLineStrings` per procedure, run:

```
comix -i input.csv -o output.json lines
```

### Examples

The `examples` directory contains FAA data for San Diego International Airport (SAN) in the form of comma-separated variable (CSV) files. There are two files, one containing points for navaids and fixes, and one containing route segments for procedures and transitions, for SAN's STARs and DPs respectively. The corresponding `json` files are the transformed GeoJSON outputs from `comix`. 