/// Bitflag representation of graph capabilities used by the analytics runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GraphCharacteristics {
    bitset: u32,
}

impl GraphCharacteristics {
    pub const DIRECTED: u32 = 0b001;
    pub const UNDIRECTED: u32 = 0b010;
    pub const INVERSE_INDEXED: u32 = 0b100;

    pub const ALL: GraphCharacteristics = GraphCharacteristics { bitset: 0b111 };
    pub const NONE: GraphCharacteristics = GraphCharacteristics { bitset: 0 };

    pub const fn new(bitset: u32) -> Self {
        GraphCharacteristics { bitset }
    }

    pub fn builder() -> GraphCharacteristicsBuilder {
        GraphCharacteristicsBuilder::new()
    }

    pub const fn is_directed(self) -> bool {
        (self.bitset & Self::DIRECTED) != 0
    }

    pub const fn is_undirected(self) -> bool {
        (self.bitset & Self::UNDIRECTED) != 0
    }

    pub const fn is_inverse_indexed(self) -> bool {
        (self.bitset & Self::INVERSE_INDEXED) != 0
    }

    pub const fn intersect(self, other: GraphCharacteristics) -> GraphCharacteristics {
        GraphCharacteristics::new(self.bitset & other.bitset)
    }

    pub const fn bitset(self) -> u32 {
        self.bitset
    }
}

impl Default for GraphCharacteristics {
    fn default() -> Self {
        GraphCharacteristics::NONE
    }
}

/// Builder for constructing [`GraphCharacteristics`] values.
#[derive(Debug, Default, Clone, Copy)]
pub struct GraphCharacteristicsBuilder {
    bitset: u32,
}

impl GraphCharacteristicsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn directed(mut self) -> Self {
        self.bitset |= GraphCharacteristics::DIRECTED;
        self
    }

    pub fn undirected(mut self) -> Self {
        self.bitset |= GraphCharacteristics::UNDIRECTED;
        self
    }

    pub fn inverse_indexed(mut self) -> Self {
        self.bitset |= GraphCharacteristics::INVERSE_INDEXED;
        self
    }

    pub fn with_bitset(mut self, bitset: u32) -> Self {
        self.bitset |= bitset;
        self
    }

    pub fn build(self) -> GraphCharacteristics {
        GraphCharacteristics::new(self.bitset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_sets_flags() {
        let characteristics = GraphCharacteristicsBuilder::new()
            .directed()
            .inverse_indexed()
            .build();
        assert!(characteristics.is_directed());
        assert!(characteristics.is_inverse_indexed());
        assert!(!characteristics.is_undirected());
    }

    #[test]
    fn intersect_retains_shared_flags() {
        let directed = GraphCharacteristicsBuilder::new().directed().build();
        let inverse = GraphCharacteristicsBuilder::new().inverse_indexed().build();
        assert_eq!(directed.intersect(inverse), GraphCharacteristics::NONE);

        let both = GraphCharacteristicsBuilder::new()
            .directed()
            .inverse_indexed()
            .build();
        assert_eq!(directed.intersect(both), directed);
    }
}
