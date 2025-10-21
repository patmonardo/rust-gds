import { DatasetUnit, makeUnitId } from '@organon/gdsl/registry/canon'

// YS IV.30 — tataḥ kleśa‑karma‑nivṛttiḥ
export const YS_IV_30_ONTOLOGY = {
  sutraDevanagari: 'ततः क्लेशकर्मनिवृत्तिः',
  sutraIAST: 'tataḥ kleśa‑karma‑nivṛttiḥ',
  sutraGloss: 'From that, the cessation of afflictions and karmic momentum.',
  tataMarker: 'tataḥ = "from this (follows)" — corollary marker of IV.29',
  corollaryOfIv29: 'Corollary of IV.29 (Dharma‑megha as relative necessity)',
  effectKlesaKarmaNivrtti: 'Effect: cessation of kleśas and of karma‑flow (nivṛtti)',
  triadRole: 'rajas pacified (appearance triad beacon)',
} as const

const CHUNKS_YS_IV_30 = [
  { id: 'ys-iv-30-text', title: 'IV.30 — Text', summary: 'Sūtra and literal gloss.' },
  { id: 'ys-iv-30-meaning', title: 'Meaning', summary: 'From Dharma‑megha follows cessation of kleśa/karma.' },
] as const

const HLOS_YS_IV_30 = [
  {
    id: 'ys-iv-30-hlo-text',
    chunkId: 'ys-iv-30-text',
    label: 'Text',
    clauses: ['define(sutraDevanagari)', 'define(sutraIAST)', 'note(sutraGloss)'],
  },
  {
    id: 'ys-iv-30-hlo-meaning',
    chunkId: 'ys-iv-30-meaning',
    label: 'Meaning',
    clauses: [
      'define(tataMarker)',
      'assert(corollaryOfIv29)',
      'conclude(effectKlesaKarmaNivrtti)',
      'note(triadRole)',
    ],
  },
] as const

export const YS_IV_30_UNIT: DatasetUnit = {
  id: makeUnitId('ys-iv-30'),
  title: 'YS IV.30 — tataḥ kleśa‑karma‑nivṛttiḥ',
  scope: 'actuality',
  logosMode: 'appearance',
  synthesis: 'First corollary (tataḥ) of Dharma‑megha: cessation of kleśas and karmic momentum.',
  faculty: 'buddhi',
  lens: 'yoga',
  chunks: CHUNKS_YS_IV_30 as any,
  hlos: HLOS_YS_IV_30 as any,
}

export const YS_IV_30_SYMBOLS = Object.keys(YS_IV_30_ONTOLOGY)
