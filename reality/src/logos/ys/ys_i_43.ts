import { DatasetUnit, makeUnitId } from '@organon/gdsl/registry/canon';

/*
YS I.43
smṛti-pariśuddha svarūpa-śūnya-iva-artha-mātra-nirbhāsā nirvitarkā

Literal segmentation (seed):
- smṛti-pariśuddha: memory purified / recollective trace washed (semantic + lexical residues attenuated)
- svarūpa-śūnya-iva: "as if" the own-form (subject-mark / ego-outline) were empty (iva = phenomenological 'as-if', not absolute null)
- artha-mātra-nirbhāsā: object-only luminosity; sheer appearance of the object-content alone
- nirvitarkā: (niḥ + vitarka) — without the mixed discursive entanglement; purified judgment free of the composite interference of śabda–artha–jñāna–vikalpa seen in savitarkā (I.42)

Seed interpretation:
Nirvitarka is the purified cognitive act where buddhi (pure reason) emits jñāna as a seed-luminosity (artha-mātra-nirbhāsa) once memory traces (smṛti) have been clarified (pariśuddha), so the self-form does not project an imposing template (appears as if empty). Jñāna-from-buddhi arises when recollective sediment / conceptual echo no longer refracts present object-only presentation.

Dialectical contrast with I.42:
Savitarka = composite entangled prompting.
Nirvitarka = buddhi’s clarified seed emission: a minimal, non-interfering, object-only judgment whose ground is reason’s self-luminous capacity.

We treat “Pure Reason manifests as Seed”:
- Seed = minimal self-sufficient luminous unit of intelligibility (artha-mātra) free of residual subjective overlay (apparent svarūpa-emptiness)
- Generation = purification(past-trace) → attenuation(linguistic/conceptual noise) → buddhi emits jñāna-seed → stable luminous object-only cognition.

*/

// Mini‑ontology (additions specific to I.43)
export const YS_I_43_MINI_ONTOLOGY = {
  purifiedMemory:
    'smṛti-pariśuddha — memory cleared of residual semantic/lexical echo',
  residualTrace:
    'Subtle mnemonic/conceptual sediment influencing present cognition',
  traceAttenuation: 'Process removing residualTrace influence',
  asIfFormEmpty:
    'svarūpa-śūnya-iva — phenomenological as-if of ego-outline suspension',
  objectOnlyLuminosity:
    'artha-mātra-nirbhāsa — sheer object presentation without mixture',
  buddhiSeed: 'Pure reason as seeding locus of luminous cognition',
  jnanaEmission: 'Production of jñāna from buddhi without discursive mixture',
  interferenceField:
    'Composite entanglement (śabda–artha–jñāna–vikalpa) from I.42',
  deMixedJudgment: 'Judgment after removal of interferenceField',
  apparentEmptinessMarker:
    '“iva” flags phenomenological semblance (not ontic void)',
  seedCognition:
    'Minimal luminous unit: object-only + purified retention backdrop',
  transitionVector: 'Directed trajectory Savitarka → Nirvitarka',
  memoryClarificationPass:
    'Operation extracting active trace vs pure retention substrate',
  egoAttenuation: 'Lowered projection of self-form in presentation',
  nonDualPresentation:
    'Cognition where subject-form does not stand opposite object',
  stabilization:
    'Retention of seedCognition without reintroduction of interference',
};

// Chunks
const CHUNKS_I_43 = [
  {
    id: 'ys-i-43-sutra',
    title: 'I.43 — Text & Baseline',
    summary:
      'Nirvitarkā: purified state where (1) memory is clarified, (2) self-form appears as if empty, (3) only the object shines forth (artha-mātra-nirbhāsa).',
  },
  {
    id: 'ys-i-43-components',
    title: 'Constituent Elements',
    summary:
      'Elements: smṛti-pariśuddha (purified memory), svarūpa-śūnya-iva (as-if self-form empty), artha-mātra-nirbhāsa (object-only luminosity), culminating in nirvitarkā.',
  },
  {
    id: 'ys-i-43-purified-memory',
    title: 'Purified Memory Function',
    summary:
      'Memory purification removes residual traces that would refract present object; enables de-noised receptive field for buddhi emission.',
  },
  {
    id: 'ys-i-43-asif-emptiness',
    title: '“As If” Emptiness',
    summary:
      '“iva” signals phenomenological appearance: self-form not annihilated but non-projective; ego-outline does not interfere.',
  },
  {
    id: 'ys-i-43-object-only',
    title: 'Object-Only Luminosity',
    summary:
      'Artha alone shines: judgment stripped of lexical/conceptual turbulence; buddhi’s seed-cognition = luminous minimal unit.',
  },
  {
    id: 'ys-i-43-transition',
    title: 'Transition from Savitarka',
    summary:
      'Path: savitarka composite → trace attenuation → de-mixing → seed emission (buddhi) → stabilized nirvitarka.',
  },
  {
    id: 'ys-i-43-buddhi-seed',
    title: 'Buddhi as Seed Emitter',
    summary:
      'Pure reason functions as an immanent seeding principle emitting jñāna when field is cleared; no external addition—internal luminosity.',
  },
  {
    id: 'ys-i-43-jnana-genesis',
    title: 'Jñāna Genesis',
    summary:
      'Jñāna-from-buddhi arises by removing mediating overlays so cognition self-presents as object-only without second-order interpretive spin.',
  },
  {
    id: 'ys-i-43-stability',
    title: 'Stabilization & Safeguards',
    summary:
      'Stabilization requires preventing re-import of residualTrace; vigilance over subtle linguistic or mnemonic re-colorings.',
  },
  {
    id: 'ys-i-43-expansion-plan',
    title: 'Planned Expansion',
    summary:
      'Phase II: fine-grain taxonomy of residual traces. Phase III: formal Hegelian correspondences (Essence → Appearance purification).',
  },
];

// Hegelian Logical Operations (HLO) seed clauses
const HLOS_I_43 = [
  {
    id: 'ys-i-43-hlo-baseline',
    chunkId: 'ys-i-43-sutra',
    label: 'Baseline State',
    clauses: [
      "tag('sutra','I.43')",
      "tag('mode','nirvitarka')",
      'state(nirvitarka := conjunction(purifiedMemory, asIfFormEmpty, objectOnlyLuminosity))',
    ],
  },
  {
    id: 'ys-i-43-hlo-purified-memory',
    chunkId: 'ys-i-43-purified-memory',
    label: 'Trace Attenuation',
    clauses: [
      'operation(traceAttenuation(memory) ⇒ purifiedMemory)',
      'residualTrace ⇒ interferenceField',
      'remove(interferenceField(memory)) ⇒ enable(seedCognition)',
    ],
  },
  {
    id: 'ys-i-43-hlo-asif-emptiness',
    chunkId: 'ys-i-43-asif-emptiness',
    label: 'Phenomenological Emptiness',
    clauses: [
      'marker(iva := apparentEmptinessMarker)',
      'asIfFormEmpty := egoAttenuation(selfForm)',
      'egoAttenuation ⇒ reduce(projectionNoise)',
    ],
  },
  {
    id: 'ys-i-43-hlo-object-only',
    chunkId: 'ys-i-43-object-only',
    label: 'Object Luminosity',
    clauses: [
      'objectOnlyLuminosity := artha_matra(lightEmission)',
      'noMix(objectOnlyLuminosity, lexicalResidue, conceptualScatter)',
      'criterion(nirvitarka) ⇐ objectOnlyLuminosity',
    ],
  },
  {
    id: 'ys-i-43-hlo-transition',
    chunkId: 'ys-i-43-transition',
    label: 'Transition Vector',
    clauses: [
      'path(savitarka → nirvitarka) := sequence(traceAttenuation → deMixedJudgment → seedCognition)',
      'failure(path) ⇐ reintrusion(residualTrace)',
    ],
  },
  {
    id: 'ys-i-43-hlo-buddhi-seed',
    chunkId: 'ys-i-43-buddhi-seed',
    label: 'Buddhi Emission',
    clauses: [
      'buddhiSeed ⇒ emit(jnanaEmission)',
      'emit(jnanaEmission) ⇐ field(purifiedMemory ∧ asIfFormEmpty)',
      'jnanaEmission ⇒ objectOnlyLuminosity',
    ],
  },
  {
    id: 'ys-i-43-hlo-jnana-genesis',
    chunkId: 'ys-i-43-jnana-genesis',
    label: 'Genesis Mechanism',
    clauses: [
      'causeOf(jnanaEmission) ≠ externalObjectInsertion',
      'causeOf(jnanaEmission) = internalLuminosity(buddhiSeed)',
      'seedCognition := minimalUnit(jnanaEmission)',
    ],
  },
  {
    id: 'ys-i-43-hlo-stability',
    chunkId: 'ys-i-43-stability',
    label: 'Stability Conditions',
    clauses: [
      'stabilization(nirvitarka) ⇐ guard(against(residualTraceReturn))',
      'reintroduction(lexicalEcho) ⇒ degrade(objectOnlyLuminosity)',
    ],
  },
  {
    id: 'ys-i-43-hlo-expansion',
    chunkId: 'ys-i-43-expansion-plan',
    label: 'Expansion Plan',
    clauses: [
      'phaseII := classify(residualTraceTypes)',
      'phaseIII := correlate(Hegelian(essencePurification), yoga(nirvitarkaTrajectory))',
    ],
  },
];

// ===================== APPENDED EXTENSION (v2) =====================

/*
Extension: Cross-differential with I.42 (savitarka) and refinement of nirvitarka seed model.
Adds:
- Extended ontology (transition + failure / safeguard semantics)
- Differential & pipeline chunks
- Additional HLO clauses (delta, guard, failure modes, verification)
*/

export const YS_I_43_MINI_ONTOLOGY_EXT = {
  savitarkaComposite:
    'Entangled mixture state from I.42 (śabda–artha–jñāna–vikalpa)',
  purificationPipeline:
    'Ordered operations producing nirvitarka from savitarka',
  lexicalResidue: 'Persisting word-token echo after initial purification',
  conceptualScatter: 'Diffuse conceptual halo around object-only luminosity',
  projectionNoise: 'Residual self-form / ego-outline coloration',
  relapseVector: 'Trajectory reintroducing traces → partial re-entanglement',
  guardCondition: 'Operational check preventing relapseVector activation',
  seedVerification: 'Heuristic confirming stability of seedCognition',
  luminosityDrift: 'Gradual loss of object-only purity via subtle trace return',
  attenuationBudget: 'Threshold of allowable residual signal before failure',
  saturationPoint:
    'Completion moment where further attenuation adds no clarity',
  deltaJudgment: 'Resultant clarified judgment contrasted with prior composite',
  semanticInertia: 'Tendency of prior traces to reassert patterning',
  antiReintroductionLoop: 'Feedback cycle monitoring reintroduction vectors',
  reasonEmissionWindow:
    'Temporal window in which buddhi emission is maximally noise-free',
  stabilityMetric: 'Quantified (model-level) indicator of sustained purity',
  failureModePartial: 'Only some mixture factors removed (pseudo-nirvitarka)',
  failureModeMasking:
    'Suppression (not dissolution) of traces that later rebound',
  integralityCriterion:
    'All three components (memory purification, ego attenuation, object-only luminosity) concurrently realized',
};

/* Added Chunks */
const CHUNKS_I_43_EXTENSION = [
  {
    id: 'ys-i-43-diff-i42',
    title: 'Differential: I.42 → I.43',
    summary:
      'Shift from composite mixture (savitarka) to purified seed (nirvitarka): remove interferenceField; retain artha stripped of lexical/conceptual scatter and ego projection.',
  },
  {
    id: 'ys-i-43-pipeline',
    title: 'Purification Pipeline',
    summary:
      'Ordered operations: (1) trace isolation (2) attenuation (3) ego-outline softening (4) de-mix check (5) buddhi emission (6) stabilization guard.',
  },
  {
    id: 'ys-i-43-failure-modes',
    title: 'Failure Modes',
    summary:
      'Partial removal (pseudo-nirvitarka), masking (suppressed traces rebound), drift (gradual contamination), relapse (full re-entanglement).',
  },
  {
    id: 'ys-i-43-guards',
    title: 'Guards & Metrics',
    summary:
      'Guard conditions and stability metrics ensure integralityCriterion; antiReintroductionLoop monitors relapse vectors.',
  },
  {
    id: 'ys-i-43-delta-judgment',
    title: 'Delta Judgment',
    summary:
      'deltaJudgment = (nirvitarka seed) – (residual semantic inertia) — formal contrast clarifying net cognitive purification.',
  },
  {
    id: 'ys-i-43-verification',
    title: 'Seed Verification',
    summary:
      'Verification heuristics: no lexical residue; zero conceptual scatter; stable attenuation budget; luminosity drift below threshold.',
  },
];

/* Integrate new chunks */
(CHUNKS_I_43 as any).push(...CHUNKS_I_43_EXTENSION);

/* Added HLO clauses */
const HLOS_I_43_EXTENSION = [
  {
    id: 'ys-i-43-hlo-diff',
    chunkId: 'ys-i-43-diff-i42',
    label: 'Differential Map',
    clauses: [
      "tag('diff','i.42→i.43')",
      'deltaJudgment := contrast(nirvitarka, savitarkaComposite)',
      'remove(interferenceField) ⇒ expose(objectOnlyLuminosity)',
      'integralityCriterion ⇐ conjunction(purifiedMemory, asIfFormEmpty, objectOnlyLuminosity)',
    ],
  },
  {
    id: 'ys-i-43-hlo-pipeline',
    chunkId: 'ys-i-43-pipeline',
    label: 'Pipeline',
    clauses: [
      'purificationPipeline := sequence(isolateTraces → traceAttenuation → egoAttenuation → deMixCheck → jnanaEmission → stabilization)',
      'reasonEmissionWindow ⇐ deMixCheck.pass',
    ],
  },
  {
    id: 'ys-i-43-hlo-failure',
    chunkId: 'ys-i-43-failure-modes',
    label: 'Failure Modes',
    clauses: [
      'failureModePartial ⇐ omit(component ∈ {purifiedMemory, asIfFormEmpty, objectOnlyLuminosity})',
      'failureModeMasking ⇐ suppress(residualTrace) ∧ not(dissolve(residualTrace))',
      'luminosityDrift ⇒ degrade(seedCognition)',
      'relapseVector := path(nirvitarka → savitarkaComposite)',
    ],
  },
  {
    id: 'ys-i-43-hlo-guards',
    chunkId: 'ys-i-43-guards',
    label: 'Guards',
    clauses: [
      'guardCondition := monitor(residualTraceReturn, projectionNoise, conceptualScatter)',
      'antiReintroductionLoop ⇒ intercept(relapseVector)',
      'stabilityMetric := evaluate(seedCognition, timeSeries(luminosityPurity))',
    ],
  },
  {
    id: 'ys-i-43-hlo-delta',
    chunkId: 'ys-i-43-delta-judgment',
    label: 'Delta Judgment',
    clauses: [
      'deltaJudgment := subtract(nirvitarka, semanticInertia)',
      'semanticInertia ⇒ risk(luminosityDrift)',
    ],
  },
  {
    id: 'ys-i-43-hlo-verification',
    chunkId: 'ys-i-43-verification',
    label: 'Verification Heuristics',
    clauses: [
      'seedVerification ⇐ (zero(lexicalResidue) ∧ zero(conceptualScatter) ∧ below(luminosityDrift, threshold) ∧ hold(integralityCriterion))',
      'success(seedVerification) ⇒ stabilize(nirvitarka)',
    ],
  },
];

(HLOS_I_43 as any).push(...HLOS_I_43_EXTENSION);

/* Optional exported helper (aggregated operators referenced here for parser tooling) */
export const YS_I_43_REFERENCED_SYMBOLS = [
  'deltaJudgment',
  'purificationPipeline',
  'failureModePartial',
  'failureModeMasking',
  'luminosityDrift',
  'relapseVector',
  'guardCondition',
  'antiReintroductionLoop',
  'stabilityMetric',
  'semanticInertia',
  'seedVerification',
  'integralityCriterion',
  'reasonEmissionWindow',
];

// ===================== APPENDED EXTENSION (v3 – Concept Deepening) =====================

/*
Concept Deepening Passage Integration:
- Former phase: concept = dividing principle, contentless; expires in light; merely qualifies appearance.
- New phase: concept possesses self‑subsistent, immutable internal content (same substantial being formerly projected as “light”).
- Division principle now only conditions the concept’s life/appearance; not essential to its inner being.
- Appearance becomes exponent (expressive surface) of inner organized through‑one‑another.
- Absolute oneness: inner organization = being; self‑grounding unity.
*/

export const YS_I_43_MINI_ONTOLOGY_DEEPEN = {
  formerConceptPhase:
    'Earlier state: concept as bare dividing principle, contentless, derivative',
  deepenedConceptPhase:
    'Current state: concept with self‑subsistent immutable internal content',
  divisionPrinciple:
    'Principle generating disjunctive terms; now merely conditional for appearance',
  conditionalizer:
    'Role of divisionPrinciple as conditioning life/appearance, not essence',
  selfSubsistentContent:
    'Intrinsic substantial being of concept prior to projected intuition',
  projectedLight:
    'Previously “original” light now seen as derivative projection of concept content',
  reciprocalQualification:
    'Earlier mutual external qualification (concept ↔ light/appearance)',
  unilateralGrounding: 'Now concept alone grounds its appearance',
  appearanceExponent: 'Appearance as exponent / announcement of inner being',
  organicThroughOneAnother:
    'Inner organization where image & imaged co‑generate',
  imageImagedOrganicPair:
    'Image and what it images posited absolutely through one another',
  absoluteSelfGrounding: 'Oneness grounded and explained purely through itself',
  phaseTransitionMarker:
    'Shift flag from formerConceptPhase → deepenedConceptPhase',
  invarianceOfContent: 'Content unchanged across bidirectional constructions',
  conditioningOnlyStatus:
    'Status of divisionPrinciple as non‑essential conditioner',
  internalOrganizationIdentity:
    'Identity of inner organization with being (no external add-ons)',
};

const CHUNKS_I_43_DEEPEN = [
  {
    id: 'ys-i-43-concept-transition',
    title: 'Concept Phase Transition',
    summary:
      'Transition from contentless dividing principle to concept with self‑subsistent inner content; division now only conditions appearance.',
  },
  {
    id: 'ys-i-43-self-subsistent-content',
    title: 'Self‑Subsistent Content',
    summary:
      'Concept’s content = same substantial being once projected as “light”; now prior to intuition and its principle.',
  },
  {
    id: 'ys-i-43-grounding-shift',
    title: 'Grounding Shift',
    summary:
      'From reciprocal qualification (concept ↔ light) to unilateral grounding: concept grounds its appearance.',
  },
  {
    id: 'ys-i-43-organic-exponent',
    title: 'Appearance as Exponent',
    summary:
      'Appearance organically announces inner being; image & imaged posited absolutely through one another.',
  },
  {
    id: 'ys-i-43-absolute-self-grounding',
    title: 'Absolute Self‑Grounding',
    summary:
      'Inner organization = being = oneness; absolute unity now self‑explains without external supplementation.',
  },
];

(CHUNKS_I_43 as any).push(...CHUNKS_I_43_DEEPEN);

const HLOS_I_43_DEEPEN = [
  {
    id: 'ys-i-43-hlo-phase-transition',
    chunkId: 'ys-i-43-concept-transition',
    label: 'Phase Transition',
    clauses: [
      'phaseTransitionMarker(formerConceptPhase → deepenedConceptPhase)',
      'formerConceptPhase := state(concept = divisionPrinciple ∧ lacks(selfSubsistentContent))',
      'deepenedConceptPhase := state(concept possesses selfSubsistentContent)',
      'divisionPrinciple ⇒ condition(appearance) ∧ not(define(essence))',
    ],
  },
  {
    id: 'ys-i-43-hlo-self-subsistent',
    chunkId: 'ys-i-43-self-subsistent-content',
    label: 'Self‑Subsistent Content',
    clauses: [
      'selfSubsistentContent = substantialBeing',
      'projectedLight ⇐ projection(substantialBeing)',
      'moreOriginal(substantialBeing, projectedLight)',
    ],
  },
  {
    id: 'ys-i-43-hlo-grounding-shift',
    chunkId: 'ys-i-43-grounding-shift',
    label: 'Grounding Shift',
    clauses: [
      'reciprocalQualification(formerConceptPhase)',
      'unilateralGrounding(deepenedConceptPhase)',
      'concept ⇒ grounds(appearance)',
      'appearance ⇒ exponent(concept)',
    ],
  },
  {
    id: 'ys-i-43-hlo-organic-exponent',
    chunkId: 'ys-i-43-organic-exponent',
    label: 'Organic Exponent',
    clauses: [
      'imageImagedOrganicPair(image, imaged)',
      'organicThroughOneAnother(image, imaged)',
      'appearanceExponent(appearance := announce(innerBeing(concept)))',
    ],
  },
  {
    id: 'ys-i-43-hlo-absolute-self-grounding',
    chunkId: 'ys-i-43-absolute-self-grounding',
    label: 'Absolute Self‑Grounding',
    clauses: [
      'internalOrganizationIdentity(innerOrganization(concept), being(concept))',
      'absoluteSelfGrounding := selfExplain(onenness(concept))',
      'absoluteSelfGrounding ⇒ negate(need(externalSupplement))',
    ],
  },
];

(HLOS_I_43 as any).push(...HLOS_I_43_DEEPEN);

/* Update referenced symbols helper */
if (Array.isArray((globalThis as any).YS_I_43_REFERENCED_SYMBOLS)) {
  (YS_I_43_REFERENCED_SYMBOLS as any).push(
    'formerConceptPhase',
    'deepenedConceptPhase',
    'divisionPrinciple',
    'selfSubsistentContent',
    'projectedLight',
    'unilateralGrounding',
    'appearanceExponent',
    'organicThroughOneAnother',
    'absoluteSelfGrounding',
    'phaseTransitionMarker',
    'internalOrganizationIdentity',
  );
}

// ===================== APPENDED EXTENSION (v4 – Final Passage: Through-One-Another Schema) =====================

/*
Final Passage Integration (Closing Seed for YS I.43)
Focus:
- Pure enduring “through-one-another” (organic co‑generative structure) in living appearance.
- Act–consequence unity (ideal or real modality) – concept proceeds as living, unfinished, must be enacted.
- Concept projects one eternally self‑same light as intuition; what stands “beneath” = further modifications of its appearance (not of the light itself).
- Reciprocity formula: only through life → concept; only through concept → life and appearance of light (never the pure form directly, only modified variants).
- Horizon: creation / modulation space of the science of knowing in possible lawful modifications.
*/

/* Ontology (Final Additions) */
export const YS_I_43_MINI_ONTOLOGY_FINAL = {
  pureThroughOneAnother:
    'Enduring organic co-generation (no external junction)',
  livingAppearance:
    'Enacted appearing that is itself alive (not static display)',
  actConsequenceDyad:
    'Single act whose consequence is immanent (ideal or real modality)',
  idealModality: 'Act-consequence relation held conceptually / inwardly',
  realModality: 'Act-consequence relation effectuated phenomenally',
  unfinishedConcept: 'Concept lives forward; completion only by enactment',
  projectionOfLight: 'Concept projects self-same light as intuitive field',
  externalizationParts:
    'Lower strata = parts of projected appearance, not light’s essence',
  modificationOfAppearance: 'Differentiation affecting appearance-layer only',
  lifeConceptReciprocity: 'Bidirectional dependence: life ⇄ concept emergence',
  firstModification:
    'Earliest variant of light’s appearance (never pure absolute form)',
  modificationCascade: 'Sequence of lawful appearance-variants',
  scienceCreationField:
    'Domain of possible constructive modifications (science of knowing)',
  possibilityEnvelope: 'Structured space of permissible variants',
  modulationRule: 'Constraint generating admissible modifications',
};

/* Final Chunks */
const CHUNKS_I_43_FINAL = [
  {
    id: 'ys-i-43-through-one-another',
    title: 'Pure Through-One-Another',
    summary:
      'Schema of enduring organic co-generation (“through-one-another”) grounding living appearance.',
  },
  {
    id: 'ys-i-43-act-dyad',
    title: 'Act–Consequence Dyad',
    summary:
      'Act and consequence (ideal or real) remain one act; concept proceeds alive, never passively complete.',
  },
  {
    id: 'ys-i-43-projection',
    title: 'Projection of Self-Same Light',
    summary:
      'Concept projects one self-same light; subordinate strata = modifications of appearance, not of light’s intrinsic essence.',
  },
  {
    id: 'ys-i-43-reciprocity',
    title: 'Life–Concept Reciprocity',
    summary:
      'Only through life to the concept; only through the concept to life and the light’s appearance—always as modified variant.',
  },
  {
    id: 'ys-i-43-modification-chain',
    title: 'Modification Cascade',
    summary:
      'Appearance layers = structured externalizations; first modification never absolute purity; cascade governed by modulation rules.',
  },
  {
    id: 'ys-i-43-creation-field',
    title: 'Creation Field',
    summary:
      '“Creation of the science of knowing” = exploration of lawful possible modifications within a possibility envelope.',
  },
];

(CHUNKS_I_43 as any).push(...CHUNKS_I_43_FINAL);

/* Final HLO Clauses */
const HLOS_I_43_FINAL = [
  {
    id: 'ys-i-43-hlo-through-one-another',
    chunkId: 'ys-i-43-through-one-another',
    label: 'Through-One-Another Schema',
    clauses: [
      'pureThroughOneAnother ⇒ ground(livingAppearance)',
      'livingAppearance ⇒ express(pureThroughOneAnother)',
    ],
  },
  {
    id: 'ys-i-43-hlo-act-dyad',
    chunkId: 'ys-i-43-act-dyad',
    label: 'Act–Consequence Unity',
    clauses: [
      'actConsequenceDyad(act, consequence)',
      'modality(actConsequenceDyad) ∈ {idealModality, realModality}',
      'unfinishedConcept ⇐ actConsequenceDyad',
    ],
  },
  {
    id: 'ys-i-43-hlo-projection',
    chunkId: 'ys-i-43-projection',
    label: 'Light Projection Relation',
    clauses: [
      'concept ⇒ projectionOfLight(selfSameLight)',
      'externalizationParts ⇒ modificationOfAppearance(not(modify(lightEssence)))',
    ],
  },
  {
    id: 'ys-i-43-hlo-reciprocity',
    chunkId: 'ys-i-43-reciprocity',
    label: 'Life–Concept Reciprocity',
    clauses: [
      'lifeConceptReciprocity := (life → concept) ∧ (concept → lifeAppearance)',
      'firstModification := earliest(appearanceVariant)',
      'never(access(pureFormDirect))',
    ],
  },
  {
    id: 'ys-i-43-hlo-modification-chain',
    chunkId: 'ys-i-43-modification-chain',
    label: 'Cascade Logic',
    clauses: [
      'modificationCascade := sequence(firstModification → appearanceVariant^n)',
      'each(appearanceVariant) ⇒ obey(modulationRule)',
      'violation(modulationRule) ⇒ reject(variant)',
    ],
  },
  {
    id: 'ys-i-43-hlo-creation-field',
    chunkId: 'ys-i-43-creation-field',
    label: 'Creation / Possibility Envelope',
    clauses: [
      'scienceCreationField := possibilityEnvelope(modificationCascade)',
      'explore(scienceCreationField) ⇒ enumerate(lawfulVariants)',
      'creationAct ⇒ instantiate(appearanceVariant) ∧ verify(modulationRule)',
    ],
  },
];

(HLOS_I_43 as any).push(...HLOS_I_43_FINAL);

/* Update referenced symbols helper */
if (Array.isArray((globalThis as any).YS_I_43_REFERENCED_SYMBOLS)) {
  (YS_I_43_REFERENCED_SYMBOLS as any).push(
    'pureThroughOneAnother',
    'livingAppearance',
    'actConsequenceDyad',
    'idealModality',
    'realModality',
    'projectionOfLight',
    'externalizationParts',
    'modificationOfAppearance',
    'lifeConceptReciprocity',
    'firstModification',
    'modificationCascade',
    'scienceCreationField',
    'possibilityEnvelope',
    'modulationRule',
    'unfinishedConcept',
  );
}

// ---------- Consolidated ontology and symbols ----------
export const YS_I_43_ONTOLOGY = {
  ...YS_I_43_MINI_ONTOLOGY,
  ...YS_I_43_MINI_ONTOLOGY_EXT,
  ...YS_I_43_MINI_ONTOLOGY_DEEPEN,
  ...YS_I_43_MINI_ONTOLOGY_FINAL,
}

export const YS_I_43_SYMBOLS = Array.from(
  new Set([
    ...Object.keys(YS_I_43_ONTOLOGY),
    ...((globalThis as any).YS_I_43_REFERENCED_SYMBOLS ?? []),
  ]),
)

// ---------- Export Unit ----------
export const YS_I_43_UNIT: DatasetUnit = {
  id: makeUnitId('ys-i-43'),
  title: 'YS I.43 — smṛti-pariśuddha … artha-mātra-nirbhāsā nirvitarkā',
  scope: 'essence',
  logosMode: 'dialectic',
  synthesis: 'nirvitarka — object-only luminosity (artha-mātra-nirbhāsa)',
  faculty: 'buddhi',
  lens: 'yoga',
  chunks: CHUNKS_I_43 as any,
  hlos: HLOS_I_43 as any,
}
