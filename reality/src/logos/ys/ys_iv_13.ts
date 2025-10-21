import { DatasetUnit, makeUnitId } from '@organon/gdsl/registry/canon'

/*
YS IV.13 — te vyakta-sūkṣmā guṇātmānaḥ

“They are manifest–subtle, of the nature of guṇas.”
Read: the dharmas’ state-spectrum (subtle ↔ manifest) is grounded in the guṇic substrate.
Continue B. APPEARANCE with the Fichte L20 aim: derive appearance from the light; present appearance as such.
*/

// ---------- Ontology (single source of truth) ----------
export const YS_IV_13_ONTOLOGY = {
  dharmas: 'Determinacies of appearance under state change',
  vyakta: 'Manifest/disclosed state (presented appearance)',
  suksma: 'Subtle/latent state (near-threshold appearance)',
  guna: 'Constituents: sattva / rajas / tamas',
  gunaAtmanah: 'Dharmas are “of the nature of guṇas” (guṇātmānaḥ)',
  stateSpectrum: 'Continuum: sūkṣma ↔ vyakta as function of guṇa-configuration',
  gunaRatios: 'Relative configuration/ratio of guṇas governing state',
  timeToStateBridge: 'Adhva-bheda (IV.12) feeds lawful transitions across the spectrum',
  identityAcrossTransitions: 'Oneness across lawful change, prefiguring IV.14 pariṇāma-ikatva',

  // Fichte L20 crosswalk (appearance as such, ground of manifold in light)
  deriveAppearanceFromLight:
    'Derive the appearance from the light; manifold arises in the appearing of light',
  groundOfManifoldInLight:
    'Ground for the manifold must appear in light (absolute oneness) and its manifestation',
  presentAppearanceAsSuch:
    'Present appearance in general, genetically (principle-level, not empirical)',
}

// ---------- Chunks ----------
const CHUNKS_YS_IV_13 = [
  {
    id: 'ys-iv-13-text',
    title: 'IV.13 Text & Baseline',
    summary: 'Dharmas are manifest–subtle, of the nature of guṇas (guṇātmānaḥ).',
  },
  {
    id: 'ys-iv-13-spectrum',
    title: 'State Spectrum (sūkṣma ↔ vyakta)',
    summary: 'Model subtle ↔ manifest as a guṇa-grounded continuum governed by guṇa ratios.',
  },
  {
    id: 'ys-iv-13-bridge',
    title: 'Bridge: Time → State → Identity',
    summary: 'Temporal differentiation (IV.12) feeds lawful state transitions and identity across change (IV.14).',
  },
  {
    id: 'ys-iv-13-crosswalk',
    title: 'Crosswalk (Fichte L20)',
    summary: 'Ground of manifold in light; derive appearance from light; present appearance as such.',
  },
  {
    id: 'ys-iv-13-errors',
    title: 'Error Modes',
    summary: 'Avoid reifying states as substances; avoid treating guṇas as things beyond appearance.',
  },
]

// ---------- HLO Clauses ----------
const HLOS_YS_IV_13 = [
  {
    id: 'ys-iv-13-hlo-text',
    chunkId: 'ys-iv-13-text',
    label: 'Baseline',
    clauses: [
      "tag('sutra','IV.13')",
      'gunaAtmanah := assert(ofNature(dharmas, guna))',
    ],
  },
  {
    id: 'ys-iv-13-hlo-spectrum',
    chunkId: 'ys-iv-13-spectrum',
    label: 'Spectrum',
    clauses: [
      'stateSpectrum := continuum(suksma ↔ vyakta, governedBy = gunaRatios)',
      'gunaRatios := config({sattva, rajas, tamas})',
    ],
  },
  {
    id: 'ys-iv-13-hlo-bridge',
    chunkId: 'ys-iv-13-bridge',
    label: 'Bridge',
    clauses: [
      'timeToStateBridge := link(IV_12.adhvaBheda → transitions(stateSpectrum))',
      'identityAcrossTransitions := prepare(IV_14.parinamaEkatva)',
    ],
  },
  {
    id: 'ys-iv-13-hlo-crosswalk',
    chunkId: 'ys-iv-13-crosswalk',
    label: 'Method',
    clauses: [
      'groundOfManifoldInLight := require(ground(stateSpectrum) appear_in light ∧ manifestation)',
      'deriveAppearanceFromLight := method(derive(appearance(stateSpectrum), from = light))',
      'presentAppearanceAsSuch := task(present(appearance, as_such))',
    ],
  },
  {
    id: 'ys-iv-13-hlo-errors',
    chunkId: 'ys-iv-13-errors',
    label: 'Errors',
    clauses: [
      'flag(reify({suksma, vyakta} as substances))',
      'flag(reify(guna beyond appearance))',
    ],
  },
]

// ---------- Export Unit ----------
export const YS_IV_13_UNIT: DatasetUnit = {
  id: makeUnitId('ys-iv-13'),
  title: 'YS IV.13 — te vyakta-sūkṣmā guṇātmānaḥ',
  scope: 'appearance',
  logosMode: 'appearance',
  synthesis: 'guna-substrate-state-spectrum',
  faculty: 'buddhi',
  lens: 'yoga',
  chunks: CHUNKS_YS_IV_13 as any,
  hlos: HLOS_YS_IV_13 as any,
}

export const YS_IV_13_SYMBOLS = Object.keys(YS_IV_13_ONTOLOGY)
