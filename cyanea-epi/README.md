# cyanea-epi

> Epigenomics for the Cyanea bioinformatics ecosystem — peak calling, motif discovery, chromatin states, differential binding, and chromatin-accessibility QC.

## What's Inside

- **Peak calling** — MACS2-style narrow and broad peak detection with a Poisson dynamic background (`call_peaks`, `call_broad_peaks`, `Peak`, `PeakSet`, `PeakCallParams`, `PeakStats`)
- **Pileup** — fragment-extended coverage tracks and depth normalization (`pileup` module)
- **Motifs** — de novo motif discovery, PWM scanning, and MEME-format I/O (`motifs` module)
- **Chromatin states** — ChromHMM-style multi-mark state segmentation (`chromatin` module)
- **Differential binding** — DESeq2-style differential peak analysis with MA-plot data (`differential_peaks`, `count_reads_in_peaks`, `ma_plot_data`, `DiffResult`)
- **Nucleosome positioning** — nucleosome occupancy and positioning from paired-end fragments (`nucleosome` module)
- **Accessibility** — ATAC-seq / DNase QC including TSS enrichment and fragment-size distribution (`accessibility` module)

## Quick Start

```toml
[dependencies]
cyanea-epi = "0.1"
```

```rust
use cyanea_epi::{call_peaks, PeakCallParams};

// Aligned tags: (chromosome, start, length).
let tags = vec![
    ("chr1".to_string(), 1_000, 50),
    ("chr1".to_string(), 1_040, 50),
    ("chr1".to_string(), 1_075, 50),
];

let params = PeakCallParams::default();
let peaks = call_peaks(&tags, 200, &params).unwrap();
println!("called {} peaks", peaks.len());
```

## See Also

- [API Reference](docs/API.md)
- [Usage Guide](docs/GUIDE.md)
- [Internal Architecture](docs/ARCHITECTURE.md)
- [Workspace Architecture](../docs/ARCHITECTURE.md)
- [Build Guide](../docs/BUILDING.md)
