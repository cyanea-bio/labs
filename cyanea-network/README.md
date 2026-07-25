# cyanea-network

> Network and pathway biology for the Cyanea bioinformatics ecosystem — graph analysis, community detection, PPI networks, gene-regulatory-network inference, and pathway topology.

## What's Inside

- **Graphs** — directed/undirected weighted graph model with node and edge attributes (`Graph`, `GraphType`, `Node`, `Edge`)
- **Centrality & topology** — degree, betweenness, closeness, and related topology metrics (`topology` module)
- **Community detection** — Louvain modularity optimization and label propagation (`louvain`, `label_propagation`, `modularity`, `CommunityResult`)
- **Protein–protein interaction** — PPI network construction and analysis (`ppi` module)
- **Gene-regulatory networks** — GRN inference from expression via correlation, mutual information, and CLR (`infer_grn`, `clr`, `CorrelationMethod`, `ExpressionMatrix`, `GrnResult`)
- **Pathways** — pathway topology scoring and crosstalk (`pathway` module)
- **Formats** — GraphML, SIF, and GEXF import/export (`parse_graphml`, `parse_sif`, `write_graphml`, `write_sif`, `write_gexf`)

## Quick Start

```toml
[dependencies]
cyanea-network = "0.1"
```

```rust
use cyanea_network::{Graph, GraphType, louvain};

let mut graph = Graph::new(GraphType::Undirected);
graph.add_node("A", "gene A").unwrap();
graph.add_node("B", "gene B").unwrap();
graph.add_node("C", "gene C").unwrap();
graph.add_edge("A", "B", 1.0).unwrap();
graph.add_edge("B", "C", 1.0).unwrap();

let result = louvain(&graph, 1.0).unwrap();
println!("modularity: {:.3}", result.modularity);
```

## See Also

- [API Reference](docs/API.md)
- [Usage Guide](docs/GUIDE.md)
- [Internal Architecture](docs/ARCHITECTURE.md)
- [Workspace Architecture](../docs/ARCHITECTURE.md)
- [Build Guide](../docs/BUILDING.md)
