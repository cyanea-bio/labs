# cyanea-proteomics

> Mass-spectrometry proteomics for the Cyanea bioinformatics ecosystem — spectrum I/O, in-silico digestion, database search, protein inference, quantification, and FDR control.

## What's Inside

- **Spectrum I/O** — MGF and mzML parsing into a common spectrum model (`MassSpectrum`, `MsLevel`, `Precursor`, `Peak`, `mgf`, `mzml`)
- **Peptides** — in-silico digestion (trypsin, LysC, chymotrypsin, AspN, GluC, …), fragment-ion generation, and modifications (`digest`, `Peptide`, `Protease`, `DigestConfig`, `Modification`)
- **Database search** — PSM scoring against a peptide database (`search` module, `Psm`)
- **Protein inference** — parsimony-based grouping of peptides into proteins (`ProteinEntry`, `ProteinGroup`)
- **Quantification** — spectral counting, label-free intensity, and TMT/iTRAQ reporter quantification (`ProteinQuant`, `QuantMethod`, `TmtPlex`)
- **FDR** — target–decoy false-discovery-rate estimation (`FdrConfig`, `FdrResult`)
- **Output** — mzTab result export (`mztab` module)

## Quick Start

```toml
[dependencies]
cyanea-proteomics = "0.1"
```

```rust
use cyanea_proteomics::{digest, DigestConfig, Protease};

let protein = b"MKWVTFISLLFLFSSAYSRGVFRR";
let config = DigestConfig {
    protease: Protease::Trypsin,
    ..Default::default()
};

let peptides = digest(protein, &config).unwrap();
println!("{} tryptic peptides", peptides.len());
```

## See Also

- [API Reference](docs/API.md)
- [Usage Guide](docs/GUIDE.md)
- [Internal Architecture](docs/ARCHITECTURE.md)
- [Workspace Architecture](../docs/ARCHITECTURE.md)
- [Build Guide](../docs/BUILDING.md)
