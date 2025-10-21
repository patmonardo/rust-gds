import { DatasetUnit, makeUnitId } from '@organon/gdsl/registry/canon'

// YS IV.32 — tataḥ kṛtārthānāṃ pariṇāma‑krama‑samāptiḥ guṇānām
export const YS_IV_32_ONTOLOGY = {
  sutraDevanagari: 'ततः कृतार्थानां परिणामक्रमसमाप्तिर्गुणानाम्',
  sutraIAST: 'tataḥ kṛtārthānāṃ pariṇāma‑krama‑samāptiḥ guṇānām',
  sutraGloss:
    'From that, the sequence of transformations of the guṇas comes to completion for those who have fulfilled their purpose.',
  tadaTatahMarker: 'tataḥ = "from this (follows)" — third corollary of IV.29',
  krtarthanamPurusha:
    'kṛtārthānām: of those whose purpose for Puruṣa is fulfilled (guṇas no longer serve ends)',
  gunaSequenceResolution:
    'Resolution of the guṇa sequence/order (pariṇāma‑krama‑samāptiḥ)',
  completedRelativeNecessity:
    'Completed form of relative necessity: ordered sequence closes (samāpti)',
  preludeAbsoluteNecessity:
    'Prelude to absolute necessity (IV.33) — transition of modality',
  essenceToConceptTransition:
    'Transition from essence (process/appearance) to concept (necessity/form)',
  seedHegelEssenceConcept:
    'Seed of Hegel’s Essence→Concept treatment (more comprehensible via Dharma‑megha schema)',
  systemOfSarvadharma:
    'System of Dharma‑megha as system of sarva‑dharmas (meta‑law across appearances)',
  triadRole:
    'tamas sequence resolution (appearance triad beacon)',
} as const

const CHUNKS_YS_IV_32 = [
  { id: 'ys-iv-32-text', title: 'IV.32 — Text', summary: 'Sūtra and literal gloss.' },
  {
    id: 'ys-iv-32-meaning',
    title: 'Meaning',
    summary:
      'Third corollary: completion of guṇa transformation sequence; completed relative necessity; prelude to absolute necessity.',
  },
] as const

const HLOS_YS_IV_32 = [
  {
    id: 'ys-iv-32-hlo-text',
    chunkId: 'ys-iv-32-text',
    label: 'Text',
    clauses: ['define(sutraDevanagari)', 'define(sutraIAST)', 'note(sutraGloss)'],
  },
  {
    id: 'ys-iv-32-hlo-meaning',
    chunkId: 'ys-iv-32-meaning',
    label: 'Meaning',
    clauses: [
      'define(tadaTatahMarker)',
      'define(krtarthanamPurusha)',
      'assert(gunaSequenceResolution)',
      'conclude(completedRelativeNecessity)',
      'note(preludeAbsoluteNecessity)',
      'define(essenceToConceptTransition)',
      'note(seedHegelEssenceConcept)',
      'note(systemOfSarvadharma)',
      'note(triadRole)',
    ],
  },
] as const

// Optional Essence bridge (Fichte/Hegel beacons)
const CHUNKS_YS_IV_32_ESSENCE = [
  {
    id: 'ys-iv-32-essence-bridge',
    title: 'Essence → Concept (Beacon)',
    summary:
      'Handoff of modality: from process/appearance to necessity/form; prepares IV.33 absolute necessity.',
  },
] as const

const HLOS_YS_IV_32_ESSENCE = [
  {
    id: 'ys-iv-32-hlo-essence-bridge',
    chunkId: 'ys-iv-32-essence-bridge',
    label: 'Bridge',
    clauses: [
      'define(essenceToConceptTransition)',
      'conclude(preludeAbsoluteNecessity)',
    ],
  },
] as const

export const YS_IV_32_UNIT: DatasetUnit = {
  id: makeUnitId('ys-iv-32'),
  title: 'YS IV.32 — tataḥ kṛtārthānāṃ pariṇāma‑krama‑samāptiḥ guṇānām',
  scope: 'actuality',
  logosMode: 'appearance',
  synthesis:
    'Third corollary: completion of the guṇa transformation sequence (completed relative necessity); prepares absolute necessity.',
  faculty: 'buddhi',
  lens: 'yoga',
  chunks: [...CHUNKS_YS_IV_32, ...CHUNKS_YS_IV_32_ESSENCE] as any,
  hlos: [...HLOS_YS_IV_32, ...HLOS_YS_IV_32_ESSENCE] as any,
}

export const YS_IV_32_SYMBOLS = Object.keys(YS_IV_32_ONTOLOGY)
