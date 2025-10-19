//! WriteRelationshipResult - Faithful 1:1 translation from Java GDS
//!
//! Translated from: org.neo4j.gds.algorithms.similarity.WriteRelationshipResult
//!
//! Result from writing relationships to database.

/// Write relationship result - translated from Java WriteRelationshipResult
/// 
/// Result from writing relationships to database.
/// 
/// Java class:
/// ```java
/// public final class WriteRelationshipResult {
///     private final long relationshipsWritten;
///     private final long writeMilliseconds;
/// 
///     WriteRelationshipResult(long relationshipsWritten, long writeMilliseconds) {
///         this.relationshipsWritten = relationshipsWritten;
///         this.writeMilliseconds = writeMilliseconds;
///     }
/// 
///     public long relationshipsWritten() {
///         return relationshipsWritten;
///     }
/// 
///     long writeMilliseconds() {
///         return writeMilliseconds;
///     }
/// }
/// ```
#[derive(Debug, Clone)]
pub struct WriteRelationshipResult {
    /// Relationships written - translated from Java: private final long relationshipsWritten;
    pub relationships_written: u64,
    
    /// Write milliseconds - translated from Java: private final long writeMilliseconds;
    pub write_milliseconds: u64,
}

impl WriteRelationshipResult {
    /// Constructor - translated from Java constructor
    /// 
    /// Java constructor:
    /// ```java
    /// WriteRelationshipResult(long relationshipsWritten, long writeMilliseconds) {
    ///     this.relationshipsWritten = relationshipsWritten;
    ///     this.writeMilliseconds = writeMilliseconds;
    /// }
    /// ```
    pub fn new(relationships_written: u64, write_milliseconds: u64) -> Self {
        Self {
            relationships_written,
            write_milliseconds,
        }
    }
    
    /// Get relationships written - translated from Java relationshipsWritten method
    /// 
    /// Java method:
    /// ```java
    /// public long relationshipsWritten() {
    ///     return relationshipsWritten;
    /// }
    /// ```
    pub fn relationships_written(&self) -> u64 {
        self.relationships_written
    }
    
    /// Get write milliseconds - translated from Java writeMilliseconds method
    /// 
    /// Java method:
    /// ```java
    /// long writeMilliseconds() {
    ///     return writeMilliseconds;
    /// }
    /// ```
    pub fn write_milliseconds(&self) -> u64 {
        self.write_milliseconds
    }
}
