import { DatasetUnit, makeUnitId } from '@organon/gdsl/registry/canon'

/*
YS IV.8 (Appearance: First Manifestation from Ground via Congruent Ripening)
tatas tad-vipāka-anuguṇānām eva-abhivyakti vāsanānām

Analytic parsing (metaphysical-scientific):
- tataḥ: from that (immediately preceding ground/condition-set)
- tad-vipāka: that ripening/fruition profile (deterministic maturation vector from prior ground)
- anuguṇānām: of those [vasanas] that are congruent/compatible with that profile
- eva: only, exclusively
- abhivyakti: manifestation, disclosure into appearance
- vāsanānām: of the vāsanas (latent form-trace packets)

Thesis (Conditioned Genesis / Appearance-1):
Only those vāsanas whose determination-signatures are congruent with the present ripening profile
produced by the prior ground actually manifest. This is a precise selection law (no miracle):
Ground → (ripening profile) → filter by congruence → appearance. Vāsanā treated as an ontological
“packet” (proto-form) encoding determinations (saṁskāra/vitarka descriptors) that match or mismatch
the current maturation vector.
*/

// ---------- Ontology ----------
export const YS_IV_8_ONTOLOGY = {
  priorGroundSet: 'The immediately prior ground/condition ensemble (from IV.7 closure)',
  ripeningProfile: 'vipāka: maturation vector computed from priorGroundSet',
  congruencePredicate: 'anuguṇa: compatibility test between ripeningProfile and a vasana’s signature',
  exclusivityOperator: 'eva: only those passing congruencePredicate are eligible',
  manifestationEvent: 'abhivyakti: transition of eligible vāsanas into appearance',
  vasanaPacket: 'Ontological latent packet (proto-form) encoding determinations (saṁskāra, vitarka)',
  vasanaSignature: 'Feature map of a vasanaPacket used for congruence evaluation',
  samskaraDeadBeing: '“Dead being” inscription: structural determination archive within vasanaPacket',
  vitarkaDeterminations: 'Determination descriptors included in the packet’s signature',
  selectionLaw: 'Law: filter(vasanaSet, congruent(ripeningProfile, vasanaSignature)) ⇒ manifest',
  nonExternalCausation: 'No external efficient cause; appearance is profile-congruent selection',
  contaminationRisk: 'If ground is colored, spurious co-manifestations may occur (handled in later sutras)',
  hegelBridge_GroundConditionAppearance: 'Ground → Condition (ripening) → Appearance selection schema',
  fichteBridge_Consequentia: 'Consequentia engine specifies antecedent → necessary selection outcome',
}

// ---------- Chunks ----------
const CHUNKS_YS_IV_8 = [
  {
    id: 'ys-iv-8-text',
    title: 'IV.8 Text & Baseline',
    summary: 'From that, only the manifestation of vāsanas congruent with that ripening occurs.',
  },
  {
    id: 'ys-iv-8-vasana-packet',
    title: 'Vāsanā as Packet',
    summary: 'Vāsanā modeled as latent ontological packet (saṁskāra/vitarka descriptors).',
  },
  {
    id: 'ys-iv-8-selection-law',
    title: 'Selection Law',
    summary: 'Ground → ripeningProfile → congruence filter → manifestation (no external cause).',
  },
  {
    id: 'ys-iv-8-crosswalk',
    title: 'Crosswalks',
    summary: 'Hegel: Ground→Condition→Appearance; Fichte: consequentia governs selection necessity.',
  },
  {
    id: 'ys-iv-8-errors',
    title: 'Error Modes',
    summary: 'Errors: reifying external producer; ignoring exclusivity; leaking colored co-manifestation.',
  },
]

// ---------- HLO Clauses ----------
const HLOS_YS_IV_8 = [
  {
    id: 'ys-iv-8-hlo-baseline',
    chunkId: 'ys-iv-8-text',
    label: 'Baseline',
    clauses: [
      "tag('sutra','IV.8')",
      'ripeningProfile := compute(vipaka, priorGroundSet)',
      'eligibleVasanas := { v in vasanaSet | congruencePredicate(ripeningProfile, vasanaSignature(v)) }',
      'exclusivityOperator := only(eligibleVasanas)',
      'manifestationEvent := appear(eligibleVasanas)',
    ],
  },
  {
    id: 'ys-iv-8-hlo-vasana',
    chunkId: 'ys-iv-8-vasana-packet',
    label: 'Vāsanā Packet Model',
    clauses: [
      'vasanaPacket := pack({ samskaraDeadBeing, vitarkaDeterminations, metadata })',
      'vasanaSignature := deriveSignature(vasanaPacket)',
      'nonExternalCausation := assert(selectionLaw)',
    ],
  },
  {
    id: 'ys-iv-8-hlo-law',
    chunkId: 'ys-iv-8-selection-law',
    label: 'Selection Law',
    clauses: [
      'selectionLaw := rule( manifestationEvent ≡ filter(vasanaSet, congruent(ripeningProfile, vasanaSignature)) )',
      'negate(externalProducer(manifestationEvent))',
    ],
  },
  {
    id: 'ys-iv-8-hlo-crosswalk',
    chunkId: 'ys-iv-8-crosswalk',
    label: 'Crosswalk',
    clauses: [
      'hegelBridge_GroundConditionAppearance(priorGroundSet → ripeningProfile → manifestationEvent)',
      'fichteBridge_Consequentia(if(priorGroundSet) ⇒ must(selectionLaw))',
    ],
  },
  {
    id: 'ys-iv-8-hlo-errors',
    chunkId: 'ys-iv-8-errors',
    label: 'Errors',
    clauses: [
      'error_externalization ⇐ posit(externalProducer)',
      'error_ignoringEva ⇐ allow(nonCongruent(manifest))',
      'contaminationRisk ⇐ coloredGround(priorGroundSet)',
    ],
  },
]

// ---------- Export Unit ----------
export const YS_IV_8_UNIT: DatasetUnit = {
  id: makeUnitId('ys-iv-8'),
  title: 'YS IV.8 — tataḥ tad-vipāka-anuguṇānām',
  scope: 'appearance',
  logosMode: 'appearance',
  synthesis: 'congruent-manifestation',
  faculty: 'buddhi',
  lens: 'yoga',
  chunks: CHUNKS_YS_IV_8 as any,
  hlos: HLOS_YS_IV_8 as any,
}

/* ============================================================
   APPEND EXTENSION (Lecture 17 — Opening: Multiplicity → One,
   Unavoidable Circle, New Principle, Uniting Parts I & II)
   Source: Fichte 1804 L17 intro mapped to YS IV.8 (Appearance-1)
============================================================ */

/*
Context:
- We shift from pure unity (earlier principle) to handling multiplicity (vāsanā packets) and bringing it back to unity.
- Terms (vāsanā descriptors) must be ordered for insight, yet the principle that orders them is itself discovered from them
  → unavoidable pedagogical circle resolved only by completion (fixed-point).
- A “new, unknown” principle is to be presented and simultaneously united with the earlier part of the science.

Appearance mapping:
- Multiplicity = vasanaSet (latent packets).
- Order-for-insight = preliminary taxonomy before full genetic grounding.
- Principle-from-terms = derive selectionLaw from behavior of manifestations under ripeningProfile.
- Circle resolution = fixed point: priorGroundSet → ripeningProfile → selectionLaw that, when applied, reproduces the observed
  manifestation pattern (eligibleVasanas) without residue or contradiction.
- Unite Parts = connect Essence/Ground (IV.2–IV.7) to Appearance (IV.8ff) via congruencePredicate and ripeningProfile.
*/

export const YS_IV_8_ONTOLOGY_EXT = {
  multiplicityLayout: 'Ordering of many terms (vāsanā descriptors) for provisional insight',
  onenessVsMultiplicity: 'Shift from given unity to reconstructing unity across a genuine multiplicity',
  principleDiscoveryFromTerms: 'Principle to be discovered out of the ordered terms themselves',
  unavoidableCircle: 'Need principle to order terms; need terms to discover principle',
  circleCompletionResolution: 'Circle annulled only by full completion (fixed-point attainment)',
  newUnknownPrinciple: 'A new principle governing appearance to be presented (selection in congruence)',
  unitePartsI_II: 'Unite prior Essence/Ground with new Appearance analysis',
  pedagogicalDifficulty: 'Admitted difficulty/confusion in external exposition order',
  orderingForInsight: 'Didactic arrangement chosen for maximal intelligibility before genetics',
  externalLectureOrder: 'Expository sequence differing from intrinsic genesis until completion',
  appearancePrinciple: 'Operational statement: only congruent vāsanās manifest from given ripening profile',
  circleFixedPoint: 'State where derived selection law reproduces observed manifestation without residue',
}

const CHUNKS_YS_IV_8_EXT = [
  {
    id: 'ys-iv-8-multiplicity-one',
    title: 'Multiplicity Laid Out to One',
    summary: 'Lay out vāsanā multiplicity and trace it back to unity via congruent selection.',
  },
  {
    id: 'ys-iv-8-unavoidable-circle',
    title: 'Unavoidable Circle',
    summary: 'Order depends on principle; principle discovered from ordered terms—resolved by completion.',
  },
  {
    id: 'ys-iv-8-new-principle',
    title: 'New Principle of Appearance',
    summary: 'Present selection-by-congruence as the new (unknown) principle for manifestation.',
  },
  {
    id: 'ys-iv-8-unite-parts',
    title: 'Unite Parts I and II',
    summary: 'Connect Essence/Ground (IV.2–IV.7) to Appearance (IV.8ff) via ripening and congruence.',
  },
  {
    id: 'ys-iv-8-pedagogy',
    title: 'Pedagogical Note',
    summary: 'Admit difficulty; provisional ordering for insight prior to genetic deduction.',
  },
]

CHUNKS_YS_IV_8.push(...CHUNKS_YS_IV_8_EXT)

const HLOS_YS_IV_8_EXT = [
  {
    id: 'ys-iv-8-hlo-multiplicity-one',
    chunkId: 'ys-iv-8-multiplicity-one',
    label: 'Multiplicity → Unity',
    clauses: [
      'vasanaSet := collect(vasanaPacket*)',
      'unityRecovery := compose(ripeningProfile, congruencePredicate, exclusivityOperator)',
      'assert(unity(manifestationEvent)) via(unityRecovery)',
    ],
  },
  {
    id: 'ys-iv-8-hlo-unavoidable-circle',
    chunkId: 'ys-iv-8-unavoidable-circle',
    label: 'Circle & Resolution',
    clauses: [
      'unavoidableCircle := need(principleDiscoveryFromTerms ∧ orderingForInsight)',
      'circleFixedPoint := solve( priorGroundSet → ripeningProfile → selectionLaw → manifestationEvent ~ observed )',
      'circleCompletionResolution := achieve(circleFixedPoint)',
    ],
  },
  {
    id: 'ys-iv-8-hlo-new-principle',
    chunkId: 'ys-iv-8-new-principle',
    label: 'Appearance Principle',
    clauses: [
      'appearancePrinciple := law( only(congruent(ripeningProfile, vasanaSignature)) ⇒ manifest )',
      'newUnknownPrinciple := declare(appearancePrinciple)',
    ],
  },
  {
    id: 'ys-iv-8-hlo-unite-parts',
    chunkId: 'ys-iv-8-unite-parts',
    label: 'Unite Parts',
    clauses: [
      'unitePartsI_II := link({Essence: priorGroundSet}, {Appearance: manifestationEvent}) via(ripeningProfile, congruencePredicate)',
      'nonExternalCausation := inherit(from Essence colorlessGround)',
    ],
  },
  {
    id: 'ys-iv-8-hlo-pedagogy',
    chunkId: 'ys-iv-8-pedagogy',
    label: 'Pedagogical Discipline',
    clauses: [
      'ack(pedagogicalDifficulty)',
      'externalLectureOrder := permit(provisional(orderingForInsight))',
      'commit := later(geneticDeduction(selectionLaw))',
    ],
  },
]

HLOS_YS_IV_8.push(...HLOS_YS_IV_8_EXT)

// Optional: reference list
export const YS_IV_8_REFERENCED_SYMBOLS = [
  'unavoidableCircle','circleFixedPoint','appearancePrinciple','unitePartsI_II',
  'principleDiscoveryFromTerms','orderingForInsight'
]

/* ============================================================
   END EXTENSION
============================================================ */

/* ============================================================
   APPEND EXTENSION (Lecture 17 — Absolute as Revelation,
   Necessity-in-Reason vs Contingency-in-Connection,
   Reliable Principle, Hypothetical Status until Emergence)
============================================================ */

/*
Mapping of the remark:
- Absolute (pure immanent being) ≡ substance/God; Appearance at its highest = revelation/manifestation of the Absolute’s
  internal genetic construction; therefore appearance is necessary in reason (not optional adornment).
- The same content later “appears” as contingent in another connection (to be worked out): necessity-in-reason vs
  contingency-in-appearance-connection must be reconciled by a reliable principle.
- Warning: do not chain contingencies; require a grounding principle that is self-sustaining.
- Current state: our “construction” term remains hypothetical until a self-sustaining principle emerges
  (watchpoint for when the fixed point and necessity criteria are satisfied).
*/

export const YS_IV_8_ONTOLOGY_EXT3 = {
  absoluteBeingAsSubstance: 'Pure immanent being ≡ absolute/substance/God',
  appearanceAsRevelation: 'Appearance (at its apex) = revelation of absolute’s internal genetic construction',
  necessityInReason: 'In reason and in itself the revelation is absolutely necessary',
  contingencyInConnection: 'The same content may appear contingent within another connection (to be derived)',
  reliablePrincipleRequirement: 'Genuine derivation requires a reliable principle (no mere pretense)',
  mutualContingencyFallacy: 'Error: deriving contingencies from contingencies (reciprocal dependence)',
  freedomObjectionVector: 'Resistance that tries to rescue contingency/freedom against necessity-in-reason',
  selfSustainingPrincipleSearch: 'Ongoing search for a principle that stands by itself (no external prop)',
  hypotheticalStatusPersistence: 'Construction remains hypothetical until the grounding condition is met',
  principleEmergenceCriterion: 'Emerges when necessity-in-reason + fixed-point selection law cohere without residue',
  revelationInThisModule: 'In IV.8 terms: manifestationEvent is revelation when driven by uncolored Ground via congruence',
}

const CHUNKS_YS_IV_8_EXT3 = [
  {
    id: 'ys-iv-8-absolute-revelation',
    title: 'Absolute as Revelation',
    summary: 'Appearance as revelation of absolute internal construction; necessary in reason.',
  },
  {
    id: 'ys-iv-8-necessity-vs-contingency',
    title: 'Necessity vs Contingency',
    summary: 'Necessary in reason; appears contingent in another connection—bridge via reliable principle.',
  },
  {
    id: 'ys-iv-8-principle-discipline',
    title: 'Reliable Principle Discipline',
    summary: 'Forbid mutual contingency chains; demand self-sustaining ground of derivation.',
  },
  {
    id: 'ys-iv-8-hypothesis-watch',
    title: 'Hypothesis Watchpoint',
    summary: 'Construction stays hypothetical until the principle emerges (fixed point + necessity).',
  },
]

CHUNKS_YS_IV_8.push(...CHUNKS_YS_IV_8_EXT3)

const HLOS_YS_IV_8_EXT3 = [
  {
    id: 'ys-iv-8-hlo-absolute-revelation',
    chunkId: 'ys-iv-8-absolute-revelation',
    label: 'Revelation Claim',
    clauses: [
      'absoluteBeingAsSubstance := alias(pureBeingSelfEnclosed)',
      'appearanceAsRevelation := identify(manifestationEvent, revelationInThisModule)',
      'necessityInReason := assert(necessary(appearanceAsRevelation) @ reasonPlane)',
    ],
  },
  {
    id: 'ys-iv-8-hlo-necessity-vs-contingency',
    chunkId: 'ys-iv-8-necessity-vs-contingency',
    label: 'Two Aspects',
    clauses: [
      'contingencyInConnection := appears(contingent, under(otherConnection))',
      'bridge := require(reliablePrincipleRequirement)',
    ],
  },
  {
    id: 'ys-iv-8-hlo-principle-discipline',
    chunkId: 'ys-iv-8-principle-discipline',
    label: 'Derivation Discipline',
    clauses: [
      'mutualContingencyFallacy := detect(chain(contingent → contingent → ...))',
      'forbid(mutualContingencyFallacy)',
      'reliablePrincipleRequirement := demand(selfSustainingPrincipleSearch)',
    ],
  },
  {
    id: 'ys-iv-8-hlo-hypothesis-watch',
    chunkId: 'ys-iv-8-hypothesis-watch',
    label: 'Emergence Watch',
    clauses: [
      'hypotheticalStatusPersistence := status(constructionProjection, hypothetical)',
      'principleEmergenceCriterion := (circleFixedPoint ∧ necessityInReason ∧ nonExternalCausation)',
      'lift(hypotheticalStatusPersistence) ⇐ principleEmergenceCriterion',
    ],
  },
]

HLOS_YS_IV_8.push(...HLOS_YS_IV_8_EXT3)

// Optional: symbols
export const YS_IV_8_REFERENCED_SYMBOLS_EXT3 = [
  'appearanceAsRevelation','necessityInReason','contingencyInConnection',
  'reliablePrincipleRequirement','mutualContingencyFallacy',
  'hypotheticalStatusPersistence','principleEmergenceCriterion'
]

/* ============================================================
   END EXTENSION
============================================================ */

/* ============================================================
   APPEND EXTENSION (Lecture 17 — Conditional Necessitation,
   Hypothetical Transfer, Firm Standpoint, Corrected Attribution)
============================================================ */

/* Ontology Extension 4 */
export const YS_IV_8_ONTOLOGY_EXT4 = {
  correctedAttribution: 'Misattribution of construction to the vain I is rescinded; grounded in being',
  projectionAsAbsoluteFact: 'Factical projection stands as absolute fact in appearance domain',
  freeVentureConditioning: 'The obtained insight is conditioned by our freely adopted procedure',
  barePossibilityLegitimacy: 'Procedure’s legitimacy shown by its bare possibility',
  higherTermInsight: 'Higher term = insight (conditional antecedent in consequentia)',
  lowerTermProjection: 'Lower term = projection-through-gap (form of outer existence)',
  conditionalNecessitation: 'If the higher term (insight) is to arise, projection becomes necessary',
  hypotheticalTransfer: 'Hypothetical status moves from lower term to higher term',
  firmStandpointPursuit: 'Aim to locate a self-sustaining principle (firm standpoint)',
  locationOfHypothesis: 'Proper seat of hypotheticality identified at the higher term',
  necessityExplanation: 'Projection’s necessity explained under the assumed insight',
}

/* Chunks Extension 4 */
const CHUNKS_YS_IV_8_EXT4 = [
  {
    id: 'ys-iv-8-corrected-attribution',
    title: 'Corrected Attribution',
    summary: 'Construction not by the vain I; it is grounded in being; projection acknowledged as fact.',
  },
  {
    id: 'ys-iv-8-conditional-necessity',
    title: 'Conditional Necessitation',
    summary: 'If the insight is to arise, projection-through-gap becomes necessary (not merely possible).',
  },
  {
    id: 'ys-iv-8-hypothesis-transfer',
    title: 'Hypothetical Transfer',
    summary: 'Hypothetical status relocates from projection (lower) to insight (higher).',
  },
  {
    id: 'ys-iv-8-firm-standpoint',
    title: 'Toward a Firm Standpoint',
    summary: 'Legitimize procedure by bare possibility; seek self-sustaining principle.',
  },
]

CHUNKS_YS_IV_8.push(...CHUNKS_YS_IV_8_EXT4)

/* HLO Clauses Extension 4 */
const HLOS_YS_IV_8_EXT4 = [
  {
    id: 'ys-iv-8-hlo-corrected-attribution',
    chunkId: 'ys-iv-8-corrected-attribution',
    label: 'Corrected Attribution',
    clauses: [
      'correctedAttribution := negate(baseIn(lowerTermProjection, emptyConsciousnessI)) ∧ baseIn(lowerTermProjection, being)',
      'projectionAsAbsoluteFact := assert(factical(lowerTermProjection))',
    ],
  },
  {
    id: 'ys-iv-8-hlo-conditional-necessity',
    chunkId: 'ys-iv-8-conditional-necessity',
    label: 'Conditional Necessity',
    clauses: [
      'conditionalNecessitation := rule( if(higherTermInsight) ⇒ must(lowerTermProjection) )',
      'necessityExplanation := explain(must(lowerTermProjection) | given(higherTermInsight))',
    ],
  },
  {
    id: 'ys-iv-8-hlo-hypothesis-transfer',
    chunkId: 'ys-iv-8-hypothesis-transfer',
    label: 'Hypothesis Transfer',
    clauses: [
      'hypotheticalTransfer := move(hypothesis(lowerTermProjection) → hypothesis(higherTermInsight))',
      'locationOfHypothesis := set(hypothesisSeat, higherTermInsight)',
    ],
  },
  {
    id: 'ys-iv-8-hlo-firm-standpoint',
    chunkId: 'ys-iv-8-firm-standpoint',
    label: 'Firm Standpoint',
    clauses: [
      'freeVentureConditioning := note(condition(higherTermInsight, adoptedProcedure))',
      'barePossibilityLegitimacy := justify(adoptedProcedure, by(possibility))',
      'firmStandpointPursuit := seek(selfSustainingPrincipleSearch)',
    ],
  },
]

HLOS_YS_IV_8.push(...HLOS_YS_IV_8_EXT4)

/* Symbols append */
export const YS_IV_8_REFERENCED_SYMBOLS_EXT4 = [
  'conditionalNecessitation','hypotheticalTransfer','higherTermInsight','lowerTermProjection',
  'firmStandpointPursuit','barePossibilityLegitimacy','projectionAsAbsoluteFact','correctedAttribution'
]

/* ============================================================
   END EXTENSION 4
============================================================ */
