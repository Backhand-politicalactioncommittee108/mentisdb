//! Search-specific derived state and ranking helpers.
//!
//! These modules build rebuildable indexes over committed thoughts without
//! changing the append-only chain itself.

/// Deterministic breadth-first expansion helpers built on top of the adjacency
/// layer.
pub mod expansion;
/// Graph adjacency and edge-provenance structures derived from committed
/// thoughts.
pub mod graph;
/// BM25-style lexical indexing and ranking over committed thoughts.
pub mod lexical;
/// Provenance path structures for graph expansion starting from lexical seeds.
pub mod provenance;

pub use expansion::{
    GraphExpansionHit, GraphExpansionMode, GraphExpansionQuery, GraphExpansionResult,
    GraphExpansionStats,
};
pub use graph::{
    AdjacencyDirection, GraphEdge, GraphEdgeProvenance, ThoughtAdjacencyIndex, ThoughtLocator,
};
pub use provenance::{GraphExpansionHop, GraphExpansionPath, GraphExpansionPathError};
