import { DatasetUnit, makeUnitId } from '@organon/gdsl/registry/canon'

/*
Meta Scaffold: Science of Knowing — Vitarka Logogenesis Stage

Intent:
- Abstract layer tying Yoga Sutra cognitive purifications (savitarka → nirvitarka → nirvicāra → prajñā)
  to Fichte’s genetic exposition (light → representation → primordial concept → self-grounding concept).
- Provide tri-partite science schema: (1) Principle (Being / Light Ground), (2) Mediation (Division / Concept Genesis),
  (3) Systematic Integration (Organic Self-Grounding & Projection of empirical domain).
- Serves as cross-reference nexus for units: YS I.42, YS I.43, Fichte L6–L7b.

Terminology Alignment (heuristic):
- Vitarka (mixed discursive cognition) ~ Composite Given Judgment / mediated light via representative.
- Nirvitarka (purified object-only) ~ Primordial concept’s first clarified seed appearance (light as term).
- Nirvicāra (forthcoming) ~ Removal of subtle conceptual structuring (image/imaged inessential disjunction).
- Prajñā (forthcoming) ~ Stable self-luminous organic through-one-another (absolute self-grounding insight).

This file seeds only the Vitarka ↔ Nirvitarka crossing and tri-partite science shell; later expansions will extend.
*/

export const META_SOK_VITARKA_ONTOLOGY = {
  scienceTriad: 'Principle / Mediation / Integration structural frame',
  principlePhase: 'Initial positing (light / being) as source-term',
  mediationPhase: 'Genetic differentiation & purification (vitarka → nirvitarka)',
  integrationPhase: 'Organic self-grounding & projection (toward prajñā)',
  vitarkaState: 'Mixed entangled cognition (savitarka composite)',
  nirvitarkaState: 'Purified object-only seed cognition',
  forthcomingNirvicara: 'Planned subtle reflective removal stage',
  forthcomingPrajna: 'Planned stabilized transcendental insight',
  fichteLight: 'Living immediate luminosity (pre-representational)',
  fichteRepresentative: 'Proxy that objectifies light (killing immediacy)',
  primordialConceptCore: 'Deeper source generating light as term',
  organicSelfGrounding: 'Concept = its own inner organization & being',
  logogenesis: 'Process of concept becoming living category',
  triPartiteScience: 'Claim: every science structurally tripartite',
  antiSystematicDrift: 'Baggage / fragmentation resisting genetic unity',
  crosswalk: 'Mapping heuristic across traditions (Yoga / Fichte)',
}

const CHUNKS_META_VITARKA = [
  {
    id: 'meta-vitarka-scope',
    title: 'Scope & Aim',
    summary: 'Anchor Vitarka (mixed cognition) as logogenesis entry point linking YS I.42/I.43 with Fichtean genetic method.',
  },
  {
    id: 'meta-vitarka-triad',
    title: 'Science Triad',
    summary: 'Principle (Light/Being) → Mediation (Division & Purification) → Integration (Organic Self-Grounding).',
  },
  {
    id: 'meta-vitarka-crosswalk',
    title: 'Crosswalk: YS ↔ Fichte',
    summary: 'Vitarka = composite (image+imaged entanglement); Nirvitarka = seed emission; Fichte: representative → primordial concept.',
  },
  {
    id: 'meta-vitarka-logogenesis',
    title: 'Logogenesis Function',
    summary: 'Vitarka stage supplies raw entangled material for genetic clarification—birth of living scientific concept.',
  },
  {
    id: 'meta-vitarka-future-stages',
    title: 'Future Stages (Planned)',
    summary: 'Nirvicāra (subtle discrimination cleared) → Prajñā (stable self-grounding insight).',
  },
]

const HLOS_META_VITARKA = [
  {
    id: 'meta-vitarka-hlo-triad',
    chunkId: 'meta-vitarka-triad',
    label: 'Triad Structure',
    clauses: [
      'scienceTriad := chain(principlePhase → mediationPhase → integrationPhase)',
      'closurePending := awaiting(forthcomingNirvicara, forthcomingPrajna)',
    ],
  },
  {
    id: 'meta-vitarka-hlo-crosswalk',
    chunkId: 'meta-vitarka-crosswalk',
    label: 'Tradition Crosswalk',
    clauses: [
      'crosswalk(vitarkaState ↔ fichteRepresentative)',
      'crosswalk(nirvitarkaState ↔ primordialConceptCore.seedAppearance)',
    ],
  },
  {
    id: 'meta-vitarka-hlo-logogenesis',
    chunkId: 'meta-vitarka-logogenesis',
    label: 'Logogenesis Mechanism',
    clauses: [
      'logogenesis(vitarkaState) ⇒ purification(nirvitarkaState)',
      'purification(nirvitarkaState) ⇒ enable(organicSelfGrounding)',
    ],
  },
  {
    id: 'meta-vitarka-hlo-future',
    chunkId: 'meta-vitarka-future-stages',
    label: 'Future Stage Placeholders',
    clauses: [
      'placeholder(forthcomingNirvicara)',
      'placeholder(forthcomingPrajna)',
      'integrationPhase ⇐ stabilized(forthcomingPrajna)',
    ],
  },
]

export const META_SCIENCE_OF_KNOWING_VITARKA_UNIT: DatasetUnit = {
  id: makeUnitId('meta-vitarka'),
  title: 'Meta — Science of Knowing: Vitarka Logogenesis Stage',
  scope: 'meta',
  logosMode: 'vitarka',
  synthesis: 'genetic-seed',
  faculty: 'buddhi',
  lens: 'cross-tradition',
  chunks: CHUNKS_META_VITARKA as any,
  hlos: HLOS_META_VITARKA as any,
}

// ===================== APPENDED EXTENSION (v2 – Logogenesis Mechanical Mapping) =====================

/*
Extension Focus:
Map vitarka / vicāra / prajñā to a generic tri-partite Logogenesis of any Science:
- Vitarka = Stage of Immediate Beings (Kinematics: sheer positional / given determinations)
- Vicāra = Stage of Essence (Dynamics: laws governing transformations of beings)
- Prajñā = Stage of Concept (Mechanical Syllogism / Inference unifying beings & laws into organic self-grounding)
Terminological Parallel:
Kinematics (Immediate) → Dynamics (Law / Essence) → Concept (Inference / Integration)
*/

export const META_SOK_VITARKA_ONTOLOGY_EXTENSION = {
  logogenesisStages: 'Ordered ascent: vitarka → vicara → prajna',
  vitarkaImmediateBeings: 'Immediate givens / positional determinations (kinematics)',
  vicaraEssenceLaws: 'Essence-stage: relational / dynamical law articulation',
  prajnaConceptIntegration: 'Concept-stage: organic inference integrating beings & laws',
  kinematicsDomain: 'Domain of “mechanical beings” (positions / states without inner law explicit)',
  dynamicsDomain: 'Domain of lawful transitions (mechanical laws of beings)',
  conceptDomain: 'Domain of syllogistic / inferential closure (system self-mediation)',
  mechanicalBeing: 'Element at kinematic level (state descriptor)',
  mechanicalLaw: 'Rule expressing lawful transformation among mechanical beings',
  mechanicalSyllogism: 'Inferential pattern: Being ⇒ Law ⇒ Integrated Concept (closure)',
  stageTransition: 'Transformation operator between consecutive stages',
  emergenceCriterion: 'Condition that licenses ascent to next stage',
  kinematicsToDynamics: 'Transition: differentiate stable relations → extract law',
  dynamicsToConcept: 'Transition: internalize law as self-mediating inference',
  inferenceClosure: 'Achievement of prajñā: system infers and grounds itself',
  degenerationRisk: 'Risk of remaining at a lower stage (anti-systematic drift)',
  diagnosticMarker: 'Symbolic clause verifying stage authenticity',
}

const CHUNKS_META_VITARKA_EXTENSION = [
  {
    id: 'meta-vitarka-stage-map',
    title: 'Stage Map (Vitarka → Vicāra → Prajñā)',
    summary: 'Tri-partite logogenesis: Immediate Beings (vitarka) → Essence Laws (vicāra) → Concept Integration (prajñā).',
  },
  {
    id: 'meta-vitarka-kinematics',
    title: 'Kinematics / Immediate Beings',
    summary: 'Vitarka level: mechanical beings as positional givens; law-structure latent, not explicit.',
  },
  {
    id: 'meta-vicara-dynamics',
    title: 'Dynamics / Essence Laws',
    summary: 'Vicāra level: extraction of mechanical laws (essence-relations) from patterned immediacies.',
  },
  {
    id: 'meta-prajna-concept',
    title: 'Concept / Inferential Closure',
    summary: 'Prajñā level: laws internalized; system performs mechanical syllogism unifying being–law–concept.',
  },
  {
    id: 'meta-mechanical-syllogism',
    title: 'Mechanical Syllogism Pattern',
    summary: 'Syllogistic schema: Given Being → Law mediation → Concept result (self-grounding inference loop).',
  },
  {
    id: 'meta-transition-chains',
    title: 'Stage Transition Operators',
    summary: 'Transitions: kinematicsToDynamics (law emergence), dynamicsToConcept (inferential internalization).',
  },
]

;(CHUNKS_META_VITARKA as any).push(...CHUNKS_META_VITARKA_EXTENSION)

const HLOS_META_VITARKA_EXTENSION = [
  {
    id: 'meta-vitarka-hlo-stage-map',
    chunkId: 'meta-vitarka-stage-map',
    label: 'Stage Ordering',
    clauses: [
      'logogenesisStages := chain(vitarkaImmediateBeings → vicaraEssenceLaws → prajnaConceptIntegration)',
      'diagnosticMarker(stageCount = 3)',
    ],
  },
  {
    id: 'meta-vitarka-hlo-kinematics',
    chunkId: 'meta-vitarka-kinematics',
    label: 'Kinematics Layer',
    clauses: [
      'kinematicsDomain ⇒ host(mechanicalBeing*)',
      'absence(explicit(mechanicalLaw)) ⇒ remain(vitarkaImmediateBeings)',
    ],
  },
  {
    id: 'meta-vitarka-hlo-dynamics',
    chunkId: 'meta-vicara-dynamics',
    label: 'Dynamics Emergence',
    clauses: [
      'kinematicsToDynamics := stageTransition( detect(pattern(mechanicalBeing)) ⇒ extract(mechanicalLaw) )',
      'emergenceCriterion(vicaraEssenceLaws) ⇐ existence(mechanicalLawSet ≥ 1)',
    ],
  },
  {
    id: 'meta-vitarka-hlo-concept',
    chunkId: 'meta-prajna-concept',
    label: 'Concept Integration',
    clauses: [
      'dynamicsToConcept := stageTransition( internalize(mechanicalLawSet) ⇒ inferenceClosure )',
      'inferenceClosure ⇒ prajnaConceptIntegration',
    ],
  },
  {
    id: 'meta-vitarka-hlo-syllogism',
    chunkId: 'meta-mechanical-syllogism',
    label: 'Mechanical Syllogism',
    clauses: [
      'mechanicalSyllogism := form( mechanicalBeing → mechanicalLaw → integratedConcept )',
      'integratedConcept ⇒ reproduce(mechanicalBeing) ∧ validate(mechanicalLaw)',
    ],
  },
  {
    id: 'meta-vitarka-hlo-transitions',
    chunkId: 'meta-transition-chains',
    label: 'Transition Chain',
    clauses: [
      'stageTransition(vitarkaImmediateBeings → vicaraEssenceLaws) = kinematicsToDynamics',
      'stageTransition(vicaraEssenceLaws → prajnaConceptIntegration) = dynamicsToConcept',
      'degenerationRisk ⇐ stall(stageTransition)',
    ],
  },
]

;(HLOS_META_VITARKA as any).push(...HLOS_META_VITARKA_EXTENSION)

// ===================== END EXTENSION v2

// ===================== APPENDED EXTENSION (v3 – Mechanical Science Stage Deepening) =====================

/*
Focus:
- A merely mechanical “science” at Vitarka = pre-reflective intuiting of immediate beings (not yet Science proper).
- Vicāra introduces reflective extraction of laws (essential relations of appearance of mechanical beings).
- Pre-Concept transitional layer: laws present, but not yet organically internalized (pre-concept latency).
- Prajñā / Absolute Concept: internalizes laws; system self-mediates; yields renewed Concept ready as Principle for a next cycle.
- Cycle: Immediate Beings → Reflective Laws → Pre-Concept Integration → Absolute Concept (Concept Renewal) → (feeds next domain’s Vitarka).
*/

export const META_SOK_VITARKA_ONTOLOGY_EXTENSION_V3 = {
  preReflectiveIntuiting: 'Vitarka-level raw apprehension of mechanical beings (no explicit law form)',
  reflectiveEssenceRelations: 'Vicāra-level articulated essential / law relations among beings',
  preConceptPhase: 'Transitional phase: laws available, not yet internally unified as concept',
  absoluteConceptIntegration: 'Prajñā-level self-mediation: laws + beings = one inferential organism',
  conceptRenewal: 'Emergence of a fresh concept functioning as new principle for further scientific expansion',
  scienceReadiness: 'Criterion set marking transition from pre-concept to concept-ready stage',
  mechanicalLawField: 'Structured set of extracted mechanical laws (vicāra output)',
  lawExtractionProcess: 'Operation deriving laws from patterned immediate beings',
  conceptSynthesisLoop: 'Internal loop fusing beings and laws into organic inference',
  stageValidation: 'Meta-check confirming genuine ascent (non-stagnation)',
  stagnationRisk: 'Risk of halting at preReflective or reflective stages (pseudo-science)',
  renewalCycle: 'Iterative spiral: Concept of cycle n seeds Vitarka of cycle n+1',
  readinessCriterion: 'Composite requirement for ascending to absolute concept',
  improperFixation: 'Attachment to raw immediacies blocking ascent',
  dispersionRisk: 'Over-proliferation of unintegrated laws (pre-concept drift)',
}

const CHUNKS_META_VITARKA_V3 = [
  {
    id: 'meta-mechanical-limits',
    title: 'Mechanical Limits (Vitarka ≠ Science)',
    summary: 'Vitarka mechanical immediacy is pre-reflective; without law extraction it is not yet Science.',
  },
  {
    id: 'meta-reflective-laws',
    title: 'Reflective Law Extraction (Vicāra)',
    summary: 'Vicāra articulates reflective essence-relations: mechanical law field emerges from patterned immediacies.',
  },
  {
    id: 'meta-preconcept-transition',
    title: 'Pre-Concept Transitional Layer',
    summary: 'Laws present but un-internalized; dispersion risk if synthesis does not proceed.',
  },
  {
    id: 'meta-absolute-concept',
    title: 'Absolute Concept Integration (Prajñā)',
    summary: 'Self-mediation: beings + laws internal to a single inferential organism—science readiness achieved.',
  },
  {
    id: 'meta-concept-renewal',
    title: 'Concept Renewal Cycle',
    summary: 'Renewed concept becomes next-cycle principle; establishes renewal spiral of scientific logogenesis.',
  },
  {
    id: 'meta-stage-guards',
    title: 'Stage Guards & Risks',
    summary: 'Guards prevent stagnation (improper fixation) or law dispersion (pre-concept drift).',
  },
]

;(CHUNKS_META_VITARKA as any).push(...CHUNKS_META_VITARKA_V3)

const HLOS_META_VITARKA_V3 = [
  {
    id: 'meta-hlo-mechanical-limits',
    chunkId: 'meta-mechanical-limits',
    label: 'Mechanical Pre-Science',
    clauses: [
      'preReflectiveIntuiting := vitarkaImmediateBeings',
      'improperFixation ⇐ cling(preReflectiveIntuiting)',
      'not(scienceReady) ⇐ absence(mechanicalLaw)',
    ],
  },
  {
    id: 'meta-hlo-reflective-laws',
    chunkId: 'meta-reflective-laws',
    label: 'Law Extraction',
    clauses: [
      'lawExtractionProcess(preReflectiveIntuiting) ⇒ mechanicalLawField',
      'reflectiveEssenceRelations := mechanicalLawField',
      'emergenceCriterion(vicaraEssenceLaws) ⇐ size(mechanicalLawField) > 0',
    ],
  },
  {
    id: 'meta-hlo-preconcept',
    chunkId: 'meta-preconcept-transition',
    label: 'Pre-Concept Drift',
    clauses: [
      'preConceptPhase ⇐ (mechanicalLawField ∧ not(absoluteConceptIntegration))',
      'dispersionRisk ⇐ proliferation(mechanicalLawField) ∧ lack(conceptSynthesisLoop)',
    ],
  },
  {
    id: 'meta-hlo-absolute-concept',
    chunkId: 'meta-absolute-concept',
    label: 'Absolute Integration',
    clauses: [
      'conceptSynthesisLoop(mechanicalBeing*, mechanicalLawField) ⇒ absoluteConceptIntegration',
      'readinessCriterion := conjunction(mechanicalLawField, conceptSynthesisLoop)',
      'scienceReadiness ⇐ readinessCriterion',
    ],
  },
  {
    id: 'meta-hlo-concept-renewal',
    chunkId: 'meta-concept-renewal',
    label: 'Renewal Spiral',
    clauses: [
      'conceptRenewal ⇐ absoluteConceptIntegration',
      'renewalCycle := iterate(conceptRenewal → seed(vitarkaImmediateBeings_next))',
    ],
  },
  {
    id: 'meta-hlo-stage-guards',
    chunkId: 'meta-stage-guards',
    label: 'Guards & Risks',
    clauses: [
      'stagnationRisk ⇐ (improperFixation ∨ dispersionRisk)',
      'stageValidation := monitor(negate(stagnationRisk))',
      'failure(stageValidation) ⇒ alert(antiSystematicDrift)',
    ],
  },
]

;(HLOS_META_VITARKA as any).push(...HLOS_META_VITARKA_V3)

// ===================== END EXTENSION v3 =====================
