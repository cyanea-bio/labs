# cyanea-meta

> Metagenomics for the Cyanea bioinformatics ecosystem — taxonomic classification, community diversity, compositional statistics, binning, and assembly QC.

## What's Inside

- **Taxonomy** — k-mer LCA classification, lineage lookup, and an NCBI-style rank hierarchy (`TaxonomyDB`, `TaxonNode`, `TaxonRank`)
- **Profiling** — relative-abundance taxonomic profiles from classified reads (`TaxonomicProfile`)
- **Diversity** — alpha diversity (Shannon, Simpson, ACE, observed species, Fisher's alpha) and beta-diversity distance matrices (`AlphaDiversity`, `BetaDiversityMatrix`)
- **Composition** — compositional transforms (CLR/ILR) and differential-abundance testing in the ALDEx2/ANCOM style (`CompositionTransform`)
- **Functional annotation** — pathway and gene-family functional profiles (`FunctionalProfile`)
- **Binning** — tetranucleotide-frequency + coverage contig binning (`Bin`, `Contig`)
- **Assembly QC** — N50/L50, auN, and related assembly statistics (`AssemblyStats`)

## Quick Start

```toml
[dependencies]
cyanea-meta = "0.1"
```

```rust
use cyanea_meta::alpha_diversity;

// Per-taxon read counts for one sample.
let counts = [128_u64, 64, 32, 16, 8, 4];
let diversity = alpha_diversity(&counts).unwrap();

// diversity.shannon, diversity.simpson, diversity.ace, diversity.observed_species, ...
println!("Shannon index: {:.3}", diversity.shannon);
```

## See Also

- [API Reference](docs/API.md)
- [Usage Guide](docs/GUIDE.md)
- [Internal Architecture](docs/ARCHITECTURE.md)
- [Workspace Architecture](../docs/ARCHITECTURE.md)
- [Build Guide](../docs/BUILDING.md)
