//! WASM bindings and browser runtime for the Cyanea bioinformatics ecosystem.
//!
//! This crate provides in-memory, JSON-based wrappers around the Cyanea labs
//! crates, designed for environments where file I/O is unavailable (browsers,
//! sandboxed workers). Every public function accepts simple types (`&str`,
//! `f64`, `usize`) and returns a JSON `String`:
//!
//! - Success: `{"ok": <value>}`
//! - Failure: `{"error": "<message>"}`
//!
//! No `wasm-bindgen` dependency is included yet — `#[wasm_bindgen]` annotations
//! are a thin layer added when building for `wasm32`.
//!
//! # Modules
//!
//! - [`seq`] — In-memory FASTA parsing, GC content
//! - [`align`] — DNA and protein sequence alignment
//! - [`stats`] — Descriptive statistics, correlation, hypothesis testing
//! - [`ml`] — K-mer counting, distance metrics
//! - [`chem`] — SMILES parsing, molecular properties, fingerprints, similarity
//! - [`struct_bio`] — PDB parsing, secondary structure, RMSD
//! - [`phylo`] — Newick trees, evolutionary distances, UPGMA/NJ, RF distance
//!
//! # Example
//!
//! ```
//! let json = cyanea_wasm::align_dna("ACGT", "ACGT", "global");
//! let v: serde_json::Value = serde_json::from_str(&json).unwrap();
//! assert!(v["ok"]["score"].as_i64().unwrap() > 0);
//! ```

pub mod align;
pub mod chem;
pub mod core_utils;
pub mod error;
pub mod io;
pub mod ml;
pub mod omics;
pub mod phylo;
pub mod seq;
pub mod stats;
pub mod struct_bio;

/// Crate version (set from Cargo.toml at compile time).
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

// ── Re-exports ───────────────────────────────────────────────────────────

// seq
pub use seq::{
    assembly_stats_json, codon_usage, gc_content, gc_content_json, minhash_compare, minhash_sketch,
    parse_fasta, parse_fasta_bytes, parse_fastq, parse_interleaved_fastq, parse_paired_fastq,
    protein_props, reverse_complement, rna_fold_nussinov, rna_fold_zuker, simulate_reads,
    transcribe, translate, trim_fastq, trim_paired_fastq, validate, JsAssemblyStats, JsCodonUsage,
    JsMinHashComparison, JsMinHashSketch, JsProteinProperties, JsRnaStructure, JsSimulatedRead,
};

// align
pub use align::{
    align_banded, alignment_to_cigar, cigar_stats, cigar_to_alignment, collapse_cigar,
    generate_md_tag, hard_clip_to_soft, merge_cigar, parse_cigar, poa_consensus, progressive_msa,
    reverse_cigar, split_cigar, validate_cigar, JsMsaResult, JsPoaConsensus,
};
pub use align::{align_batch, align_dna, align_dna_custom, align_protein};

// stats
pub use stats::{
    benjamini_hochberg, bonferroni, bootstrap_ci, bray_curtis, cox_ph, describe, fst_hudson,
    kaplan_meier, log_rank_test, mann_whitney_u, pearson, permutation_test, shannon_index,
    simpson_index, spearman, t_test, t_test_two_sample, tajimas_d, wright_fisher, JsCoxPhResult,
    JsDescriptiveStats, JsFstResult, JsKmResult, JsKmStep, JsLogRankResult, JsTajimaD,
    JsTestResult, JsWrightFisherResult,
};

// ml
pub use ml::{
    confusion_matrix, cosine_similarity, cross_validate_rf, euclidean_distance,
    feature_importance_variance, gbdt_classify, gbdt_regression, hamming_distance, hmm_likelihood,
    hmm_viterbi, kmeans, kmer_count, manhattan_distance, pca, pr_curve, random_forest_classify,
    roc_curve, tsne, umap, JsConfusionMatrix, JsCvResult, JsFeatureSelection, JsGbdtClassifyResult,
    JsGbdtRegressionResult, JsHmmViterbiResult, JsKmeansResult, JsKmerCounts, JsPcaResult,
    JsPrCurve, JsPrPoint, JsRandomForestResult, JsRocCurve, JsRocPoint, JsTsneResult, JsUmapResult,
};

// core
pub use core_utils::sha256;
#[cfg(feature = "compress")]
pub use core_utils::{zstd_compress, zstd_decompress};

// chem
pub use chem::{
    canonical, maccs_fingerprint, parse_sdf, smiles_fingerprint, smiles_properties,
    smiles_substructure, tanimoto, tanimoto_maccs, JsFingerprint, JsMaccsFingerprint,
    JsMolecularProperties, JsSdfMolecule, JsSubstructureResult,
};

// struct_bio
pub use struct_bio::{
    contact_map, kabsch_align, parse_mmcif, pdb_info, pdb_secondary_structure,
    ramachandran_analysis, rmsd, JsChainInfo, JsContactMap, JsKabschResult, JsMmcifInfo,
    JsRamachandranEntry, JsSSAssignment, JsSecondaryStructure, JsStructureInfo,
};

// phylo
pub use phylo::{
    build_nj, build_upgma, evolutionary_distance, newick_info, parse_nexus, rf_distance,
    simulate_coalescent, simulate_coalescent_growth, simulate_evolution, write_nexus,
    JsCoalescentTree, JsNamedTree, JsNexusFile, JsRFDistance, JsSimulatedAlignment, JsTreeInfo,
};

// io
pub use io::{
    depth_stats_from_sam, ncbi_fetch_url, parse_bed_text, parse_bedgraph, parse_blast_xml,
    parse_gfa, parse_gff3_text, parse_vcf_text, pileup_from_sam, pileup_to_mpileup_text,
    JsBedGraphRecord, JsBedRecord, JsBlastXmlHit, JsBlastXmlResult, JsDepthStats, JsGfaGraph,
    JsGfaSegment, JsGff3Gene, JsPileup, JsPileupColumn, JsVcfVariant,
};

// omics
pub use omics::{
    annotate_variant, bisulfite_convert, cbs_segment, closest_intervals, complement_intervals,
    find_cpg_islands, gearys_c, intersect_intervals, jaccard_intervals, liftover_interval,
    make_windows, merge_intervals, morans_i, subtract_intervals, JsClosestResult, JsCnvSegment,
    JsCpgIsland, JsGearysC, JsGenomicInterval, JsJaccard, JsLiftoverResult,
    JsSpatialAutocorrelation, JsVariantEffect,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_is_set() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn reexports_accessible() {
        // Verify that key functions are accessible from the crate root.
        let _ = parse_fasta(">s\nACGT\n");
        let _ = align_dna("A", "A", "global");
        let _ = describe("[1,2,3]");
        let _ = kmer_count("ACGT", 2);
        let _ = reverse_complement("ACGT");
        let _ = transcribe("ACGT");
        let _ = translate("ATGAAA");
        let _ = validate("ACGT", "dna");
        let _ = sha256("hello");
        let _ = spearman("[1,2,3]", "[1,2,3]");
        let _ = bonferroni("[0.01,0.04]");
        let _ = smiles_properties("CCO");
        let _ = canonical("CCO");
        let _ = smiles_fingerprint("CCO", 2, 2048);
        let _ = tanimoto("CCO", "CCO");
        let _ = smiles_substructure("CCO", "CC");
        let _ = pdb_info("HEADER\nEND\n");
        let _ = rmsd("[[0,0,0]]", "[[0,0,0]]");
        let _ = newick_info("(A,B);");
        let _ = evolutionary_distance("ACGT", "ACGT", "p");
        let _ = build_upgma(r#"["A","B"]"#, "[[0,1],[1,0]]");
        let _ = build_nj(r#"["A","B"]"#, "[[0,1],[1,0]]");
        let _ = rf_distance("(A,B);", "(A,B);");
    }
}
