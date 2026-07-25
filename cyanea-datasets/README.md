# cyanea-datasets

> Bundled sample datasets and protocol templates for the Cyanea bioinformatics ecosystem — ready-to-use demo data for examples, tutorials, and tests.

## What's Inside

Small, permissively-usable reference data embedded directly in the crate — no downloads, no network access:

- **Genomics** — demo FASTA/VCF/gene records (E. coli 16S rRNA, SARS-CoV-2 spike RBD, chr22 variants, TP53/BRCA1) (`genomics`)
- **Alignment** — sample sequence sets for pairwise/multiple alignment demos (`alignment`)
- **Epigenomics** — demo peak/coverage data (`epigenomics`)
- **Single-cell** — a small PBMC expression matrix with helper accessors (`single_cell`, `demo_pbmc_50`)
- **Chemistry** — demo molecules and SMILES (`chemistry`)
- **Phylogenetics** — demo trees and taxa (`phylogenetics`)
- **Metagenomics** — demo community count tables (`metagenomics`)
- **Structural** — demo structure records (`structural`)
- **Protocols** — 16 structured wet-lab and dry-lab protocol templates, renderable to Markdown (`protocols`, `all_protocols`, `wet_lab_protocols`, `dry_lab_protocols`)

## Quick Start

```toml
[dependencies]
cyanea-datasets = "0.1"
```

```rust
use cyanea_datasets::{genomics, protocols};

// Embedded demo sequence: (name, FASTA bytes).
let (name, fasta) = genomics::ecoli_16s_rrna();
println!("{name}: {} bytes", fasta.len());

// Structured protocol templates.
for protocol in protocols::all_protocols() {
    println!("{}", protocol.to_markdown());
}
```

## See Also

- [API Reference](docs/API.md)
- [Usage Guide](docs/GUIDE.md)
- [Internal Architecture](docs/ARCHITECTURE.md)
- [Workspace Architecture](../docs/ARCHITECTURE.md)
- [Build Guide](../docs/BUILDING.md)
