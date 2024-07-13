[![Rust](https://github.com/PatrickLerner/rustrail/actions/workflows/rust.yml/badge.svg)](https://github.com/PatrickLerner/rustrail/actions/workflows/rust.yml) [![codecov](https://codecov.io/github/PatrickLerner/rustrail/graph/badge.svg?token=L96BAYS6N1)](https://codecov.io/github/PatrickLerner/rustrail) ![GitHub License](https://img.shields.io/github/license/PatrickLerner/rustrail?style=flat&color=%235E81AC)

# Rustrail

Experimental railroad / train physic simulation.

## Required assets

Assets must be downloaded and saved to `assets` folder.

###  `assets/dgm200_utm32s.tif`

Digitales Geländemodell (digital landscape model, aka a height map) to get information about
Germany's topography.

[Download](https://daten.gdz.bkg.bund.de/produkte/dgm/dgm200/aktuell/dgm200.utm32s.geotiff.zip)

### OpenStreetMap

Requires a download of OpenStreetMap Data (e.g. [geofabrik][]) that must be placed in `assets/` folder.

[geofabrik]: https://download.geofabrik.de/europe/germany/hessen.html

## Research / Sources

- [Fahrdynamik des Schienenverkehrs, Dietrich Wende](https://link.springer.com/book/10.1007/978-3-322-82961-0)
- [Das System Bahn: Der ICE](https://www.db-systemtechnik.de/resource/blob/1665152/b1e975afc4621103696b63e8247d37ce/Aktuell_D_Schulbroschuere-Regensburg_Das-System-Bahn-der-ICE-data.pdf)

## Developer notes

- Code test coverage: `cargo llvm-cov --open`
