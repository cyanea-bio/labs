//! Omics data structures for the Cyanea bioinformatics ecosystem.
//!
//! This crate provides core types for working with omics data:
//!
//! - **Genomic coordinates** — [`Strand`], [`GenomicPosition`], [`GenomicInterval`]
//! - **Interval collections** — [`IntervalSet`] with overlap queries
//! - **Expression matrices** — Dense [`ExpressionMatrix`] (features × samples)
//! - **Sparse matrices** — [`SparseMatrix`] in COO format
//! - **Variants** — VCF-style [`Variant`] representation
//! - **Gene annotations** — [`Gene`], [`Transcript`], [`Exon`] hierarchy
//!
//! # Quick start
//!
//! ```
//! use cyanea_omics::ExpressionMatrix;
//! use cyanea_core::Summarizable;
//!
//! let matrix = ExpressionMatrix::new(
//!     vec![vec![1.0, 2.0], vec![3.0, 4.0]],
//!     vec!["gene1".into(), "gene2".into()],
//!     vec!["sample_a".into(), "sample_b".into()],
//! ).unwrap();
//!
//! assert_eq!(matrix.shape(), (2, 2));
//! assert_eq!(matrix.get(0, 1), Some(2.0));
//! assert_eq!(matrix.summary(), "ExpressionMatrix: 2 features \u{00d7} 2 samples");
//! ```

pub mod acmg;
pub mod annotation;
pub mod clinical;
pub mod cnv;
pub mod coverage;
pub mod crispr;
pub mod expr;
pub mod genome_arithmetic;
pub mod genomic;
#[cfg(feature = "h5ad")]
pub mod h5ad;
pub mod haplotype;
pub mod hic;
pub mod interval;
pub mod interval_tree;
pub mod liftover;
pub mod methylation;
pub mod microarray;
pub mod network;
pub mod otu;
pub mod pharmacogenomics;
#[cfg(feature = "single-cell")]
pub mod sc_batch;
#[cfg(feature = "single-cell")]
pub mod sc_cluster;
#[cfg(feature = "single-cell")]
pub mod sc_integrate;
#[cfg(feature = "single-cell")]
pub mod sc_markers;
#[cfg(feature = "single-cell")]
pub mod sc_mtx;
#[cfg(feature = "single-cell")]
pub mod sc_preprocess;
#[cfg(feature = "single-cell")]
pub mod sc_trajectory;
#[cfg(feature = "single-cell")]
pub mod sc_velocity;
pub mod single_cell;
pub mod sparse;
pub mod spatial;
pub mod spatial_cellchat;
pub mod spatial_deconvolution;
pub mod spatial_domains;
pub mod spatial_platforms;
pub mod spatial_segmentation;
pub mod variant;
pub mod variant_annotation;
#[cfg(feature = "zarr")]
pub mod zarr;

pub use annotation::{Exon, Gene, GeneType, Transcript};
pub use cnv::{
    baf_segmentation, circular_binary_segmentation, detect_sv_breakpoints, merge_cnv_segments,
    BafSegment, CbsConfig, CnvSegment, SvBreakpoint, SvType,
};
pub use coverage::RleCoverage;
pub use expr::ExpressionMatrix;
pub use genome_arithmetic::{
    closest, complement, genome_info, intersect, intersect_report_a, jaccard, jaccard_stats,
    make_sliding_windows, make_windows, merge, subtract, union, windows_around, ClosestResult,
    GenomeInfo, JaccardStats, StrandMode,
};
pub use genomic::{GenomicInterval, GenomicPosition, Strand};
pub use haplotype::{
    haplotype_blocks, haplotype_diversity, phase_em, Haplotype, HaplotypeBlock, PhasedGenotypes,
};
pub use interval::IntervalSet;
pub use interval_tree::{Interval, IntervalTree};
pub use liftover::{liftover, liftover_batch, parse_chain, ChainFile, LiftoverResult};
pub use methylation::{
    bisulfite_convert, call_methylation, find_cpg_islands, find_dmrs, CpgIsland, CpgSite, DmRegion,
    DmrConfig,
};
pub use network::{CentralityScores, Community, Graph};
pub use otu::OtuTable;
pub use single_cell::ColumnData;
pub use sparse::SparseMatrix;
pub use spatial::{
    cooccurrence, delaunay_neighbors, gearys_c, knn_spatial_neighbors, ligand_receptor_score,
    morans_i, CooccurrenceResult, GearysC, LrInteraction, SpatialAutocorrelation, SpatialGraph,
    SpatialPoint,
};
pub use spatial_cellchat::{
    aggregate_pathways, analyze_communication, demo_lr_database, CommParams, CommunicationResult,
    LrPair, PathwayCommunication,
};
pub use spatial_deconvolution::{
    nnls_deconvolve, score_enrichment, CellTypeSignature, DeconvolutionResult, EnrichmentScore,
    SpotDeconvolution,
};
pub use spatial_domains::{
    detect_domains, find_spatially_variable_genes, hmrf_smooth, DomainParams, DomainResult,
    SpatialDomain, SpatiallyVariableGene,
};
pub use spatial_platforms::{
    merfish_to_spatial_points, slideseq_to_spatial_points, visium_to_spatial_points, MerfishData,
    SlideseqData, VisiumData, VisiumScaleFactors,
};
pub use spatial_segmentation::{
    expansion_segmentation, voronoi_segmentation, watershed_grid, ExpansionParams,
    SegmentationResult, SegmentedCell,
};
pub use variant::{Variant, VariantFilter, VariantType, Zygosity};
pub use variant_annotation::{
    annotate_variant, annotate_variants, score_splice_disruption, AnnotationConfig, Consequence,
    SpliceScore, VariantEffect,
};
// Re-export ACMG/ClinVar types
pub use acmg::{
    auto_evidence, match_clinvar, parse_clinvar_tsv, AcmgClass, AcmgClassification, AcmgCriterion,
    AcmgEvidence, ClinVarAnnotation, EvidenceStrength,
};

// Re-export clinical genomics types
pub use clinical::{
    bethesda_markers, call_msi, compute_tmb, hla_compatibility, parse_hla_typing, HlaAllele,
    HlaTypingResult, MsiLocus, MsiResult, MsiStatus, TmbCategory, TmbResult,
};

// Re-export Hi-C types
pub use hic::{
    call_compartments, call_loops, call_tads, contacts_to_matrix, insulation_scores,
    parse_cool_text, parse_pairs, write_pairs, ChromatinLoop, Compartment, CompartmentResult,
    ContactMatrix, CoolHeader, LoopParams, SparseContact, Tad, TadParams,
};

// Re-export microarray analysis types
pub use microarray::{
    beta_to_m_value, compute_beta, diff_methylation, limma_diff_expr, m_value_to_beta,
    median_polish, quantile_normalize, rma_normalize, swan_normalize, DiffExprResult,
    DiffMethResult, InfiniumType, MethylationProbe,
};

// Re-export pharmacogenomics types
pub use pharmacogenomics::{
    activity_to_phenotype, call_star_alleles, demo_cyp2d6_database, lookup_drug_interactions,
    AlleleFunction, DrugGeneInteraction, MetabolizerPhenotype, PgxDatabase, StarAllele,
    StarAlleleCall,
};

// Re-export CRISPR analysis types
pub use crispr::{
    analyze_screen, cfd_score, count_mismatches, find_off_targets, predict_editing,
    score_guide_rs2, BaseEditor, EditingOutcome, GuideRna, OffTarget, ScreenGeneResult,
};

#[cfg(feature = "h5ad")]
pub use h5ad::{read_h5ad, write_h5ad};
#[cfg(feature = "zarr")]
pub use zarr::{read_zarr, write_zarr};
