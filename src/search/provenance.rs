use super::{AdjacencyDirection, GraphEdge, ThoughtLocator};
use std::error::Error;
use std::fmt;

/// One directed step in a graph-expansion path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphExpansionHop {
    /// Traversal direction used for this step.
    pub direction: AdjacencyDirection,
    /// The underlying graph edge that was traversed.
    pub edge: GraphEdge,
}

impl GraphExpansionHop {
    /// Build a new expansion hop.
    pub fn new(direction: AdjacencyDirection, edge: GraphEdge) -> Self {
        Self { direction, edge }
    }

    /// Return the locator this hop departs from.
    pub fn departure(&self) -> &ThoughtLocator {
        self.edge.departure(self.direction)
    }

    /// Return the locator this hop arrives at.
    pub fn arrival(&self) -> &ThoughtLocator {
        self.edge.arrival(self.direction)
    }
}

/// One concrete provenance path from a lexical seed through graph expansion.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphExpansionPath {
    /// Original lexical or seed match that expansion started from.
    pub seed: ThoughtLocator,
    /// Ordered hops taken from the seed.
    pub hops: Vec<GraphExpansionHop>,
}

impl GraphExpansionPath {
    /// Start a new expansion path from one seed thought.
    pub fn new(seed: ThoughtLocator) -> Self {
        Self {
            seed,
            hops: Vec::new(),
        }
    }

    /// Return the current frontier node for further expansion.
    pub fn current(&self) -> &ThoughtLocator {
        self.hops
            .last()
            .map(GraphExpansionHop::arrival)
            .unwrap_or(&self.seed)
    }

    /// Return the number of hops taken from the seed.
    pub fn depth(&self) -> usize {
        self.hops.len()
    }

    /// Return `true` if this path already visited `locator`.
    pub fn contains(&self, locator: &ThoughtLocator) -> bool {
        self.seed == *locator || self.hops.iter().any(|hop| hop.arrival() == locator)
    }

    /// Return the visited locators in order, including the seed.
    pub fn visited(&self) -> Vec<&ThoughtLocator> {
        let mut visited = Vec::with_capacity(self.hops.len() + 1);
        visited.push(&self.seed);
        for hop in &self.hops {
            visited.push(hop.arrival());
        }
        visited
    }

    /// Extend the path by traversing one more edge.
    pub fn extend(
        &self,
        direction: AdjacencyDirection,
        edge: &GraphEdge,
    ) -> Result<Self, GraphExpansionPathError> {
        let departure = edge.departure(direction);
        if departure != self.current() {
            return Err(GraphExpansionPathError::Disconnected {
                expected: self.current().clone(),
                actual: departure.clone(),
            });
        }

        let arrival = edge.arrival(direction);
        if self.contains(arrival) {
            return Err(GraphExpansionPathError::Cycle {
                locator: arrival.clone(),
            });
        }

        let mut hops = self.hops.clone();
        hops.push(GraphExpansionHop::new(direction, edge.clone()));
        Ok(Self {
            seed: self.seed.clone(),
            hops,
        })
    }
}

/// Path-extension errors for graph expansion provenance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GraphExpansionPathError {
    /// The proposed edge does not continue from the path's current frontier.
    Disconnected {
        /// The locator the path expected to depart from.
        expected: ThoughtLocator,
        /// The actual departure locator carried by the edge.
        actual: ThoughtLocator,
    },
    /// The proposed edge would revisit an already-visited locator.
    Cycle {
        /// The locator that would be revisited.
        locator: ThoughtLocator,
    },
}

impl fmt::Display for GraphExpansionPathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Disconnected { expected, actual } => write!(
                f,
                "cannot extend graph-expansion path: expected departure {:?}, got {:?}",
                expected, actual
            ),
            Self::Cycle { locator } => write!(
                f,
                "cannot extend graph-expansion path: revisiting locator {:?}",
                locator
            ),
        }
    }
}

impl Error for GraphExpansionPathError {}
