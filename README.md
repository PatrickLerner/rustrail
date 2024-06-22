[![Rust](https://github.com/PatrickLerner/rustrail/actions/workflows/rust.yml/badge.svg)](https://github.com/PatrickLerner/rustrail/actions/workflows/rust.yml) [![codecov](https://codecov.io/github/PatrickLerner/rustrail/graph/badge.svg?token=L96BAYS6N1)](https://codecov.io/github/PatrickLerner/rustrail)

# Rustrail

Experimental railroad / train physic simulation.

## Required assets

Assets must be downloaded and saved to `assets` folder.

###  `assets/dgm200_utm32s.tif`

Digitales Gittermodell (DGM, aka a height map) to get information about
Germany's topography.

[Download](https://daten.gdz.bkg.bund.de/produkte/dgm/dgm200/aktuell/dgm200.utm32s.geotiff.zip)

## Research / Sources

- [Fahrdynamik des Schienenverkehrs, Dietrich Wende](https://link.springer.com/book/10.1007/978-3-322-82961-0)

## Developer notes

- Code test coverage: `cargo llvm-cov --open`
