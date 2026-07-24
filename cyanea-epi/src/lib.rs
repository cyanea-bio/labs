//! Epigenomics analysis for the Cyanea bioinformatics ecosystem.
//!
//! `cyanea-epi` provides comprehensive tools for analyzing epigenomic data from
//! ChIP-seq, ATAC-seq, and MNase-seq experiments:
//!
//! - **Peak calling** — MACS2-style narrow and broad peak calling with local background estimation
//! - **Signal analysis** — Pileup construction, normalization, smoothing, and replicate correlation
//! - **Motif discovery** — K-mer enrichment-based motif discovery and PWM scanning
//! - **Chromatin states** — ChromHMM-like state learning via EM and genomic segmentation
//! - **Differential analysis** — DESeq2-style differential binding with negative binomial test
//! - **Nucleosome positioning** — MNase-seq nucleosome calling with periodicity detection
//! - **ATAC-seq QC** — TSS enrichment, FRiP, NFR ratio, and fragment size metrics
//!
//! # Example
//!
//! ```
//! use cyanea_epi::pileup::{build_pileup, TagPileup};
//!
//! // Build a pileup from aligned reads
//! let reads = vec![
//!     ("chr1".to_string(), 100, 50),
//!     ("chr1".to_string(), 150, 50),
//!     ("chr1".to_string(), 200, 50),
//! ];
//!
//! let pileup = build_pileup(&reads, 200);
//! assert!(pileup.coverage.contains_key("chr1"));
//! ```

pub mod accessibility;
pub mod chromatin;
pub mod differential;
pub mod error;
pub mod motifs;
pub mod nucleosome;
pub mod peaks;
pub mod pileup;

// Re-export error types
pub use error::{EpiError, Result};

// Re-export peak types
pub use peaks::{call_broad_peaks, call_peaks, Peak, PeakCallParams, PeakSet, PeakStats};

// Re-export pileup types and functions
pub use pileup::{
    build_pileup, fingerprint, normalize_pileup, pileup_correlation, smooth_pileup, TagPileup,
};

// Re-export motif types and functions
pub use motifs::{
    compare_motifs, discover_motifs, motif_enrichment, parse_meme, scan_sequence, write_meme,
    DiscoveryParams, Motif, MotifMatch,
};

// Re-export chromatin types and functions
pub use chromatin::{
    learn_chromatin_states, segment_genome, state_enrichment, ChromHMMModel, ChromHMMParams,
    ChromatinSegmentation, ChromatinState,
};

// Re-export differential types and functions
pub use differential::{count_reads_in_peaks, differential_peaks, ma_plot_data, DiffResult};

// Re-export nucleosome types and functions
pub use nucleosome::{
    call_nucleosomes, nfr_score, periodicity, NucleosomeParams, NucleosomePosition,
};

// Re-export accessibility types and functions
pub use accessibility::{
    atacqc, fragment_size_distribution, frip, insert_size_metrics, tss_enrichment, AtacQcResult,
    InsertSizeMetrics,
};
