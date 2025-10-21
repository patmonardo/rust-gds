import { DatasetUnit, makeUnitId } from '@organon/gdsl/registry/canon'

/*
YS IV.2  (Dharmapada / Essence Stage Seed)
jāti-antara-pariṇāma prakṛti-āpurāt

Literal segmentation (analytic seed):
- jāti: class / birth-type / categorical configuration
- antara: other / distinct / alternative
- pariṇāma: transformation / phase-change sequence
- prakṛti: underlying nature / primordial constituting matrix
- āpurāt (āpūra / apūrat): filling up / saturation / plenary fulfillment

Essence-phase interpretation:
A shift to a different categorical configuration (jāti-antara-pariṇāma) occurs when the conditioning matrix (prakṛti) reaches saturation of a determinate potential envelope (prakṛti-āpurāt). At Essence granularity: the dharma stream reclassifies because its active pratyaya lattice fully discloses (fills) a modality, triggering a threshold transition to a new stabilizing pattern.

Crosswalk heuristics:
- Abhidharma: phase-shift in skandha / dharma patterning due to full maturation of conditioning set.
- Fichte (Doctrine of Essence): ladder escalation when a principle’s internal differentiation saturates, forcing elevation to a higher unity.
- Hegel (Essence → Actuality): completion of determinate conditions yields transition to a new concrete form.
- Meta SoK: renewalInjection event that seeds a fresh Vitarka layer (new immediacies) from Essence saturation.

Use: Provides the first Essence sutra node (numbered “2” here intentionally) to align with a reframed structural entry (sutra 1 treated separately).
*/

// Mini‑Ontology (YS IV.2 Essence Seed)
export const YS_IV_2_ONTOLOGY = {
  jati: 'Birth-class / categorical configuration template',
  antara: 'Other / distinct variant boundary marker',
  parinama: 'Ordered transformation sequence (phase lineage)',
  jatiAntaraParinama: 'Transition to a distinct categorical configuration',
  prakritiMatrix: 'Underlying conditioning nature-field (prakṛti) at Essence analysis level',
  saturationEvent: 'Threshold where a potential envelope is fully realized (āpurāt)',
  potentialEnvelope: 'Finite set of latent differentiations available to current configuration',
  completionThreshold: 'Limit condition whose attainment triggers class shift',
  conditioningLattice: 'Interlinked pratyaya set structuring a dharma stream',
  dharmaStream: 'Temporal procession of dharmaUnit manifestations',
  reclassificationTrigger: 'Condition that remaps instance stream to a new jati key',
  essencePhaseShift: 'Qualitative jump in categorical identity after saturation',
  residualPotential: 'Unexpended differentiation capacity (its exhaustion precedes shift)',
  elevationAnalogy: 'Fichtean ladder escalation paralleling saturation-driven shift',
  renewalInjection: 'Output signal feeding a new Vitarka immediacy cycle',
  stabilityWindow: 'Interval of post-shift relative invariance before next accumulation',
  misreadPersistence: 'Error: treating saturation plateau as permanent essence',
  crosswalkFichte: 'Mapping: saturationEvent ↔ principle elevation trigger',
  crosswalkAbhidharma: 'Mapping: maturation of pratyaya cluster → dharma reclassification',
}

// Chunks
const CHUNKS_YS_IV_2 = [
  {
    id: 'ys-iv-2-text',
    title: 'IV.2 Text & Baseline',
    summary: 'Jāti-antara transformation arises from saturation (āpurāt) of prakṛti’s potential.',
  },
  {
    id: 'ys-iv-2-mechanism',
    title: 'Saturation Mechanism',
    summary: 'Conditioning lattice sequentially discloses potentialEnvelope; reaching completionThreshold triggers reclassification.',
  },
  {
    id: 'ys-iv-2-crosswalks',
    title: 'Crosswalks (Fichte / Abhidharma / Essence)',
    summary: 'Maps prakṛti saturation to Fichtean elevation and Abhidharma maturation of pratyaya clusters.',
  },
  {
    id: 'ys-iv-2-phase-shift',
    title: 'Phase Shift & Renewal',
    summary: 'Shift emits renewalInjection, seeding new Vitarka immediacies; establishes new stabilityWindow until next accumulation.',
  },
  {
    id: 'ys-iv-2-failure-modes',
    title: 'Failure / Misread Modes',
    summary: 'Misread persistence: mistaking transient plateau for absolute; ignoring residualPotential dynamics.',
  },
]

// HLO clauses
const HLOS_YS_IV_2 = [
  {
    id: 'ys-iv-2-hlo-baseline',
    chunkId: 'ys-iv-2-text',
    label: 'Baseline Definition',
    clauses: [
      "tag('sutra','IV.2')",
      'jatiAntaraParinama := cause(saturationEvent(prakritiMatrix))',
      'saturationEvent(prakritiMatrix) ⇐ fill(potentialEnvelope)',
    ],
  },
  {
    id: 'ys-iv-2-hlo-mechanism',
    chunkId: 'ys-iv-2-mechanism',
    label: 'Mechanism',
    clauses: [
      'conditioningLattice ⇒ disclose(potentialEnvelope)',
      'completionThreshold := allRealized(potentialEnvelope)',
      'reclassificationTrigger ⇐ completionThreshold',
      'reclassificationTrigger ⇒ jatiAntaraParinama',
    ],
  },
  {
    id: 'ys-iv-2-hlo-crosswalks',
    chunkId: 'ys-iv-2-crosswalks',
    label: 'Crosswalk Mapping',
    clauses: [
      'crosswalkFichte(saturationEvent ↔ elevationAnalogy)',
      'crosswalkAbhidharma(saturationEvent ↔ maturation(pratyayaSet))',
      'essencePhaseShift ⇐ jatiAntaraParinama',
    ],
  },
  {
    id: 'ys-iv-2-hlo-phase-shift',
    chunkId: 'ys-iv-2-phase-shift',
    label: 'Renewal Emission',
    clauses: [
      'renewalInjection ⇐ essencePhaseShift',
      'renewalInjection ⇒ seed(vitarkaImmediateBeings_nextCycle)',
      'stabilityWindow := maintain(newJati, until(accumulate(residualPotential = 0)))',
    ],
  },
  {
    id: 'ys-iv-2-hlo-failure',
    chunkId: 'ys-iv-2-failure-modes',
    label: 'Failure Modes',
    clauses: [
      'misreadPersistence ⇐ treat(stabilityWindow = absoluteIdentity)',
      'errorFlag ⇐ misreadPersistence',
      'monitor(residualPotential) ⇒ predict(next(saturationEvent))',
    ],
  },
]

export const YS_IV_2_UNIT: DatasetUnit = {
  id: makeUnitId('ys-iv-2'),
  title: 'YS IV.2 — jāti-antara-pariṇāma prakṛti-āpurāt',
  scope: 'essence',
  logosMode: 'essence',
  synthesis: 'saturation-shift',
  faculty: 'buddhi',
  lens: 'yoga',
  chunks: CHUNKS_YS_IV_2 as any,
  hlos: HLOS_YS_IV_2 as any,
}

/* ============================================================
   APPEND EXTENSION (Frame of Extremes / Absolute Product Thesis)
   (If a stray line like `}; // closes YS_IV_2_UNIT` was inserted earlier,
    remove it; the YS_IV_2_UNIT object already ended properly.)
============================================================ */

export const YS_IV_2_ONTOLOGY_EXT = {
  pureUnconditioned: 'Limit-pole: pristine Prajñā (unmixed luminosity)',
  impureUnconditioned: 'Limit-pole: inert residual substrate notion (to be dissolved)',
  conditionedMiddle: 'Dynamic conditioned field between unconditioned extremes',
  prajnaDharmaFrame: 'Framing pair Pure/Impure delimiting conditioned analysis',
  digestionProcess: 'Self-return where I=I (Absolute) is product of prior genesis, not primitive axiom',
  absoluteProductThesis: 'Fichte 1804: identity (I=I) emerges as result, not starting principle',
  framingCycle: 'Cycle: Principle of Being → Essence Saturation → Absolute Product',
}

const CHUNKS_YS_IV_2_EXT = [
  {
    id: 'ys-iv-2-frame-extremes',
    title: 'Frame of Extremes',
    summary: 'Pure & impure unconditioned poles frame the conditioned middle (prajna–dharma lens).',
  },
  {
    id: 'ys-iv-2-absolute-product',
    title: 'Absolute as Product',
    summary: 'I=I (Brahmāsmi) as digestive outcome of Essence saturation, not initial presupposition.',
  },
]

;(CHUNKS_YS_IV_2 as any).push(...CHUNKS_YS_IV_2_EXT)

const HLOS_YS_IV_2_EXT = [
  {
    id: 'ys-iv-2-hlo-frame-extremes',
    chunkId: 'ys-iv-2-frame-extremes',
    label: 'Framing Poles',
    clauses: [
      'prajnaDharmaFrame := pair(pureUnconditioned, impureUnconditioned)',
      'conditionedMiddle := field(between(pureUnconditioned, impureUnconditioned))',
    ],
  },
  {
    id: 'ys-iv-2-hlo-absolute-product',
    chunkId: 'ys-iv-2-absolute-product',
    label: 'Absolute Product Thesis',
    clauses: [
      'absoluteProductThesis := emerge(I_equals_I, after(saturationEvent_chain))',
      'digestionProcess := transform(principleOfBeing → absoluteProduct)',
    ],
  },
]

;(HLOS_YS_IV_2 as any).push(...HLOS_YS_IV_2_EXT)

/* ============================================================
   END
*/
