# ADR 0005 — Property Selection and Fallback Semantics

- Status: Proposed
- Date: 2025-10-05
- Owner: Pat

## Context

The PureGraphStore implementation supports properties at three levels: graph, node, and relationship. Each property is represented by a `Property` (schema + `PropertyValues`) and is held inside a `PropertyStore` specific to the element type (graph/node/relationship). During traversal and graph view construction, the system often needs to choose a single property to use as "the property" for a relationship type (e.g., a weight used during traversal). This choice is currently implemented in `build_selected_relationship_properties` with a heuristic: if a store has exactly one property, that property is auto-selected; otherwise a selector map (string key per relationship type) may be consulted; if nothing is selected, traversal falls back to a numeric default value.

Several ambiguities were discovered during review:

- What should happen when multiple properties exist for a relationship type and no explicit selector is provided?
- Should auto-selection ever occur when there are multiple properties (e.g., choose first by key order)?
- How do property types (ValueType) map to traversal and cursor expectations? Many code paths assume numeric `double` values.
- What errors or warnings should surface if a selector points to a missing key or to a property of an incompatible type?
- Are properties mutable or should selection snapshots be considered immutable views?

This ADR proposes a clear, conservative policy for selection and fallback semantics that prioritizes deterministic behavior, explicitness, and safety.

## Decision

1. Selection precedence (explicit > implicit):

   - If an explicit selector is provided for a relationship type (via `relationship_property_selectors`), use the keyed property if present.
   - If the selector is provided but the key is missing, return an error to the caller when they attempt to create a graph view that requires the selection (fail-fast). The selector contract encourages callers to set selectors only when they know the property exists.
   - If no selector is provided and the store contains exactly one property, auto-select that single property (backwards-compatible behavior).
   - If no selector is provided and there are multiple properties, do NOT auto-select implicitly. Instead, the graph view should be created with no selected property for that relationship type; traversal that requests a property value will receive a fallback default value instead of silently picking a potentially wrong property.

2. Type safety and traversal consumption:

   - The traversal APIs (RelationshipCursor, RelationshipPropertyValues accessors) must express the expected value type. For common numeric traversals (weights), callers should request a numeric traversal mode (e.g., `PropertyTraversalMode::with_value(fallback: PropertyValue)`), and when a selected property is present, the system will attempt to coerce or convert to the requested numeric type where safe.
   - If the selected property has a ValueType incompatible with the requested traversal type (e.g., a string property selected for numeric traversal), the system will: when possible, attempt a well-defined conversion (e.g., parse numeric strings) and otherwise log a trace-level warning and fall back to the traversal fallback value. Conversions should be conservative; prefer fallback over surprising conversions.

3. Explicit error/document warning policy:

   - Fail-fast error when an explicit selector is set to a non-existent key during creation of a graph view that requires property selection (i.e., code that builds `selected_relationship_properties` with `selector` present should return Result::Err). This prevents silent misconfiguration.
   - If a selector points at a property with an incompatible type, return an Err at graph view creation time where possible. If the graph view is still created without selection (because traversal didn't require it), log a warning.

4. Snapshot semantics:

   - `graph()` returns a lightweight view that shares underlying `PropertyValues` via `Arc` where possible (shallow copy). Selection decisions are made at graph creation and stored in the view as `selected_relationship_properties` snapshots (these snapshots reference the original `PropertyValues` by `Arc`). This means: updating the store's property values after graph creation changes the underlying data (views observe changes) unless the store is replaced via the builder pattern. To preserve immutability semantics where desired, callers should make copies of stores or use the builder to create new stores and then re-create graph views.

5. Migration & Compatibility:
   - Preserve current auto-select behavior only for the single-property case to avoid breaking existing code.
   - New behavior is more conservative for multiple properties: prefer explicitness.

## Consequences

- Positive:

  - Eliminates silent mis-selection of properties in multi-property stores.
  - Encourages clear configuration via selectors when multiple candidates exist.
  - Makes error cases explicit and easier to debug.
  - Improves type-safety posture and reduces surprising conversions.

- Negative:
  - Slightly more verbose setup in callers when they have multi-property stores (they must provide explicit selectors if they want a property selected for traversal).
  - Some clients that relied on implicit first-key selection will need minor updates.

## Implementation notes (practical steps)

1. Update `build_selected_relationship_properties` to:

   - If selector exists, validate key presence; if missing, return Err from the caller (graph view builder) or omit selection but log/warn (choice depends on call site). We'll implement fail-fast when the caller explicitly provided a selector.
   - If no selector and store.len() == 1, select the single property (current behavior).
   - If no selector and store.len() > 1, leave unselected (no property snapshot created for that type).

2. Add helper APIs to produce explicit selector maps (Builder convenience methods) and add tests for selector behavior.

3. Add unit tests for:

   - Single-property stores (auto-select)
   - Multi-property stores without selector (no selection)
   - Selector pointing at missing key (error)
   - Selector pointing at property with incompatible type (error or warning depending on context)

4. Document the behavior in `doc/api_contract_pure_graphstore.md` and in `doc/pure_graphstore_checklist.md` with a short note.

## Examples

- Single-property store (auto-select):

  If a `DefaultRelationshipPropertyStore` has only `"weight"`, then `build_selected_relationship_properties` will select `weight` automatically for traversal.

- Multi-property store (no selector):

  If a `DefaultRelationshipPropertyStore` has `"weight"` and `"capacity"` and the caller does not pass a selector, traversal requests for property values will use the supplied fallback value (e.g., `0.0`) instead of silently choosing `weight` or `capacity`.

- Explicit selector (fail-fast):

  If the caller sets `relationship_property_selectors[KNOWS] = "weight"` and `weight` does not exist, graph creation that requires the selection will return an error indicating `PropertyNotFound("weight")`.

## Related ADRs

- ADR 0001: Property Graph Store Design
- ADR 0002: Barrel and Prelude Strategy
- ADR 0003: Node Property Value Contract
- ADR 0004: Property Cursors

---

Status: Proposed — please review and suggest edits or approve to proceed with code changes (tests + enforcement).
