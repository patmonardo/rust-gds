import { DatasetUnit, makeUnitId } from '@organon/gdsl/registry/canon'

/*
Meta Scaffold: Science of Knowing — Essence (Dharmapada / YS IV Seed)
Intent:
- Bridge Fichte’s Doctrine of Essence to Yoga Sutra Chapter IV (seen here as Dharmapada: Abhidharma-grade analysis).
- Provide abstraction for: dharma (event-unit), pratyaya (conditioning nexus), samāpatti/prajñā (stabilized integrative cognition).
- Establish transition: Prajñā (Concept closure) → Essence (Conditioned Actuality Web) → renewed Principle cycle.

Crosswalk Heuristics:
Fichte (Light / Representation / Primordial Concept / Self-Grounding) →
  Essence Phase: Ground (conditioning set) → Appearance (structured projection) → Actuality Loop (self-sustaining nexus)
Abhidharma:
  dharmaUnit (moment-event), pratyayaSet (conditions), series (stream), superior insight = prajñā
Hegel (orientation layer):
  Ground → Appearance → Actuality

Use: later YS IV sutras will attach to these nodes rather than raw narrative.
*/

export const META_SOK_ESSENCE_ONTOLOGY = {
  essencePhase: 'Stage after Concept closure: analysis of conditioning web',
  dharmaUnit: 'Minimal conditioned event / occurrence',
  pratyayaSet: 'Structured set of conditions enabling a dharmaUnit',
  conditioningLink: 'Relation expressing dependency (A conditions B)',
  groundMatrix: 'Total network of mutual conditionings (Fichtean deeper principle articulation)',
  appearanceStratum: 'Phenomenal layer projected from groundMatrix',
  actualityLoop: 'Closed reproduction cycle (conditions regenerate conditioned)',
  prajnaStabilization: 'Stable seeing of conditioned co-arising without reification',
  deReification: 'Dissolution of “thing” hypostasis into conditioning patterns',
  essenceDifferential: 'Shift mapping from Concept-unity to conditioning multiplicity',
  renewalInjection: 'How Essence analysis seeds next Vitarka immediacies',
  ladderEscalation: 'Insertion of higher-order conditioning principles (Fichtean ladder)',
  abhidharmaParallel: 'Alignment tag with classical Abhidharma analytic taxonomy',
  satCitBridge: 'Vedāntic link: sat = subsistence (ground), cit = luminous cognition of ground',
  liberationHeuristic: 'Marker: recognition of pure conditioning empties self-form projection',
}

const CHUNKS_META_ESSENCE = [
  {
    id: 'meta-essence-scope',
    title: 'Essence Scope',
    summary: 'Defines Essence phase as conditioned actuality analysis (Dharmapada / Abhidharma alignment).',
  },
  {
    id: 'meta-essence-entities',
    title: 'Core Entities',
    summary: 'dharmaUnit + pratyayaSet + conditioningLink form the basic analytic triad.',
  },
  {
    id: 'meta-essence-ground',
    title: 'Ground Matrix',
    summary: 'Interlinked conditioning forming groundMatrix; projects appearanceStratum.',
  },
  {
    id: 'meta-essence-actuality',
    title: 'Actuality Loop',
    summary: 'Self-reproducing nexus: condition ↔ conditioned cycle = actualityLoop.',
  },
  {
    id: 'meta-essence-prajna',
    title: 'Prajñā Stabilization',
    summary: 'Stable insight: prajnaStabilization = deReification(appearance via groundMatrix vision).',
  },
  {
    id: 'meta-essence-renewal',
    title: 'Renewal Injection',
    summary: 'Essence differentials seed new Vitarka immediacies (renewalInjection).',
  },
]

const HLOS_META_ESSENCE = [
  {
    id: 'meta-essence-hlo-scope',
    chunkId: 'meta-essence-scope',
    label: 'Phase Definition',
    clauses: [
      'essencePhase := post(conceptClosure)',
      'aim(essencePhase) := analyze(conditioningNetwork)',
    ],
  },
  {
    id: 'meta-essence-hlo-entities',
    chunkId: 'meta-essence-entities',
    label: 'Entity Triad',
    clauses: [
      'triad(dharmaUnit, pratyayaSet, conditioningLink)',
      'pratyayaSet ⇒ enable(dharmaUnit)',
    ],
  },
  {
    id: 'meta-essence-hlo-ground',
    chunkId: 'meta-essence-ground',
    label: 'Ground Projection',
    clauses: [
      'groundMatrix := closure(conditioningLink*)',
      'appearanceStratum := projection(groundMatrix)',
    ],
  },
  {
    id: 'meta-essence-hlo-actuality',
    chunkId: 'meta-essence-actuality',
    label: 'Actuality Cycle',
    clauses: [
      'actualityLoop := cycle(pratyayaSet ↔ dharmaUnit)',
      'stability(actualityLoop) ⇐ regenerationRate(pratyayaSet)',
    ],
  },
  {
    id: 'meta-essence-hlo-prajna',
    chunkId: 'meta-essence-prajna',
    label: 'Prajñā Insight',
    clauses: [
      'prajnaStabilization := deReification(appearanceStratum)',
      'liberationHeuristic ⇐ sustained(prajnaStabilization)',
    ],
  },
  {
    id: 'meta-essence-hlo-renewal',
    chunkId: 'meta-essence-renewal',
    label: 'Renewal Path',
    clauses: [
      'renewalInjection := extract(essenceDifferential)',
      'renewalInjection ⇒ seed(vitarkaImmediateBeings_nextCycle)',
    ],
  },
]

export const META_SCIENCE_OF_KNOWING_ESSENCE_UNIT: DatasetUnit = {
  id: makeUnitId('meta-essence'),
  title: 'Meta — Science of Knowing: Essence (Dharmapada Seed)',
  scope: 'meta',
  logosMode: 'essence',
  synthesis: 'conditioning-web',
  faculty: 'buddhi',
  lens: 'cross-tradition',
  chunks: CHUNKS_META_ESSENCE as any,
  hlos: HLOS_META_ESSENCE as any,
}
