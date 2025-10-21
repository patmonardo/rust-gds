import { DatasetUnit, makeUnitId } from '@organon/gdsl/registry/canon'

// YS IV.31 — tadā sarvāvaraṇa‑malāpetasya jñānasyānantyāt jñeyam alpam
export const YS_IV_31_ONTOLOGY = {
  sutraDevanagari: 'तदा सर्वावरणमलापेतस्य ज्ञानस्य अनन्त्यात् ज्ञेयमल्पम्',
  sutraIAST: 'tadā sarvāvaraṇa‑malāpetasya jñānasyānantyāt jñeyam alpam',
  sutraGloss:
    'Then, for knowledge cleared of all coverings and taints, because of its infinity, what remains to be known is little.',
  tadaMarker: 'tadā = "then; from this follows" — corollary marker of IV.29',
  corollaryOfIv29:
    'Corollary of IV.29 (Dharma‑megha as relative necessity): unobstructed knowledge; residual knowables are minimal',
  effectUnobstructedKnowledge:
    'Effect: knowledge without coverings/taints (sarvāvaraṇa‑mala‑apetasya)',
  effectResidualKnowablesLittle:
    'Effect: because knowledge is infinite, what remains to be known is little (jñeyam alpam)',
  triadRole: 'sattva clarity (appearance triad beacon)',

  // Essence links (your notes)
  arthamatraSamadhi:
    'arthamātra samādhi: meaning‑only absorption (object‑only), independence from substance',
  samyagJnana:
    'samyag‑jñāna: complete right knowledge; unobstructed meaning',
  definitionPerfectMeaning:
    'Kant (logic): definition as concept with perfect meaning; here read as completion of meaning (Idea→Concept handoff)',
  meaningOverSubstance:
    'Fichte/Hegel: transition from substance to the meaning of substance (no hypostasis)',

  // Fichte — Triadic Projection: from itself, out of itself, through itself
  fichteTriadicProjectionFormula:
    '"from itself, out of itself, and through itself" — triadic projection formula',
  triadicProjectionRoles:
    'Triad roles: from-itself (origin), out-of-itself (emanation), through-itself (self-mediation via gap)',
  projectionThroughGap:
    'Projection through an absolute gap; concept/intuition appears separated from essence',
  conceptAsImageNotEssence:
    'Concept (or intuition) as image, not the essence itself; prior negation of essence assumed',
  principleProvidingAppearsIndependent:
    'In immanent view, principle-providing appears as self-producing independence',
  higherUnintuitablePrincipleProviding:
    'True ground is a higher, absolutely unintuitable principle-providing (not the immanent appearance)',
  flexibilityNotGroundedInTruth:
    'Flexibility in the procedure\'s appearance is not grounded in truth; difficult to grasp',
  qualitativeOnenessRemains:
    'Qualitative oneness of intuition does not end with this appearance-process',
  gapAsMayaOperator:
    'The gap functions as a Maya operator: veiling/revealing by separating image from essence',
} as const

const CHUNKS_YS_IV_31 = [
  { id: 'ys-iv-31-text', title: 'IV.31 — Text', summary: 'Sūtra and literal gloss.' },
  { id: 'ys-iv-31-meaning', title: 'Meaning', summary: 'Unobstructed knowledge; few residual knowables.' },
] as const

const HLOS_YS_IV_31 = [
  {
    id: 'ys-iv-31-hlo-text',
    chunkId: 'ys-iv-31-text',
    label: 'Text',
    clauses: ['define(sutraDevanagari)', 'define(sutraIAST)', 'note(sutraGloss)'],
  },
  {
    id: 'ys-iv-31-hlo-meaning',
    chunkId: 'ys-iv-31-meaning',
    label: 'Meaning',
    clauses: [
      'define(tadaMarker)',
      'assert(corollaryOfIv29)',
      'conclude(effectUnobstructedKnowledge)',
      'conclude(effectResidualKnowablesLittle)',
      'note(triadRole)',
    ],
  },
] as const

// HLO essence for dense mapping (arthamātra, samyag‑jñāna, definition/meaning)
const CHUNKS_YS_IV_31_ESSENCE = [
  {
    id: 'ys-iv-31-essence',
    title: 'HLO Essence — Unobstructed Meaning',
    summary:
      'arthamātra samādhi; samyag‑jñāna; definition as perfect meaning; meaning over substance.',
  },
] as const

const HLOS_YS_IV_31_ESSENCE = [
  {
    id: 'ys-iv-31-hlo-essence',
    chunkId: 'ys-iv-31-essence',
    label: 'Essence',
    clauses: [
      'define(arthamatraSamadhi)',
      'define(samyagJnana)',
      'note(definitionPerfectMeaning)',
      'assert(meaningOverSubstance)',
    ],
  },
] as const

const CHUNKS_YS_IV_31_TRIADIC_PROJECTION = [
  {
    id: 'ys-iv-31-triadic-projection',
    title: 'Fichte — Triadic Projection (from/out/through itself)',
    summary:
      'Immanent self-projection with an absolute gap; concept as image; higher ground beyond appearance.',
  },
] as const

const HLOS_YS_IV_31_TRIADIC_PROJECTION = [
  {
    id: 'ys-iv-31-hlo-triadic-projection',
    chunkId: 'ys-iv-31-triadic-projection',
    label: 'Triadic Projection',
    clauses: [
      'define(fichteTriadicProjectionFormula)',
      'define(triadicProjectionRoles)',
      'define(projectionThroughGap)',
      'define(conceptAsImageNotEssence)',
      'assert(principleProvidingAppearsIndependent)',
      'note(higherUnintuitablePrincipleProviding)',
      'note(flexibilityNotGroundedInTruth)',
      'assert(qualitativeOnenessRemains)',
      'note(gapAsMayaOperator)',
    ],
  },
] as const

export const YS_IV_31_UNIT: DatasetUnit = {
  id: makeUnitId('ys-iv-31'),
  title: 'YS IV.31 — tadā … jñeyam alpam',
  scope: 'actuality',
  logosMode: 'appearance',
  synthesis:
    'Second corollary (tadā): unobstructed knowledge (infinite); what remains to be known is little.',
  faculty: 'buddhi',
  lens: 'yoga',
  chunks: [...CHUNKS_YS_IV_31, ...CHUNKS_YS_IV_31_ESSENCE, ...CHUNKS_YS_IV_31_TRIADIC_PROJECTION] as any,
  hlos: [...HLOS_YS_IV_31, ...HLOS_YS_IV_31_ESSENCE, ...HLOS_YS_IV_31_TRIADIC_PROJECTION] as any,
}

export const YS_IV_31_SYMBOLS = Object.keys(YS_IV_31_ONTOLOGY)
