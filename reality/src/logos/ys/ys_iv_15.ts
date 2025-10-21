import { DatasetUnit, makeUnitId } from '@organon/gdsl/registry/canon'

/*
YS IV.15 — vastu-sāmye citta-bhedāt tayoḥ vibhaktaḥ panthāḥ
“Even with the object the same, due to differences of minds, the paths are divided.”

Seed: Bridge from Thingness (IV.14) to Worldness and the Law of Appearance.
Hold vastu constant; parameterize appearance by citta; extract invariants/variants.
*/

// ---------- Ontology ----------
export const YS_IV_15_ONTOLOGY = {
  vastu: 'Held-constant object (as stabilized appearance via IV.14 identity)',
  samya: 'Sameness/equality (control variable on vastu)',
  citta: 'Mind-stream/locus of appearing',
  cittaBheda: 'Differences among cittas (structure, guṇa-config, saṁskāra)',
  vibhaktaPanthah: 'Divergent paths/trajectories of appearance',
  worldness: 'Intersubjective stabilization with lawful variance across cittas',
  lawOfAppearance:
    'Rule: appear(vastu | citta) yields divergent paths with identifiable invariants and variants',
  invariantsUnderCitta: 'Features stable across citta differences (world-regularities)',
  variantsUnderCitta: 'Features that vary with citta (perspectival/conditioning effects)',
  holdVastuConstant: 'Method: analyze divergence while holding vastu constant',
  linkFromIdentity: 'Built on IV.14 pariṇāma-ikatva (identity across change)',
  // Stances (optional)
  purusha: 'Unchanging seer; intuition does not alter what is intuited',
  pumanJiva: 'Adaptive agent in appearance; contributes to citta-bheda',
  // Errors
  errorNaiveRealism: 'Treating all appearance as identical across cittas (denies variance)',
  errorSolipsism: 'Denying invariants/world-regularities (collapses worldness)',
}

// ---------- Chunks ----------
const CHUNKS_YS_IV_15 = [
  {
    id: 'ys-iv-15-text',
    title: 'IV.15 Text & Baseline',
    summary:
      'With vastu held the same, citta differences yield divergent paths of appearance.',
  },
  {
    id: 'ys-iv-15-law',
    title: 'Law of Appearance',
    summary:
      'Parameterize by citta; extract invariants/variants while holding vastu constant.',
  },
  {
    id: 'ys-iv-15-worldness',
    title: 'Worldness (Intersubjectivity)',
    summary:
      'Stabilization across cittas under lawful variance; build on IV.14 identity.',
  },
  {
    id: 'ys-iv-15-stances',
    title: 'Stances (Puruṣa / Puman–Jīva)',
    summary:
      'Unchanging seer vs adaptive agent; contributions to citta-bheda.',
  },
  {
    id: 'ys-iv-15-errors',
    title: 'Error Modes',
    summary:
      'Avoid naive realism (no variance) and solipsism (no invariants).',
  },
]

// ---------- HLO Clauses ----------
const HLOS_YS_IV_15 = [
  {
    id: 'ys-iv-15-hlo-text',
    chunkId: 'ys-iv-15-text',
    label: 'Baseline',
    clauses: [
      "tag('sutra','IV.15')",
      'samya(vastu) := holdVastuConstant',
      'cittaBheda := differentiate(citta)',
      'vibhaktaPanthah := assert(diverge(paths(appearance), by = cittaBheda) when samya(vastu))',
    ],
  },
  {
    id: 'ys-iv-15-hlo-law',
    chunkId: 'ys-iv-15-law',
    label: 'Law',
    clauses: [
      'lawOfAppearance := rule(appear(vastu | citta) → {invariantsUnderCitta, variantsUnderCitta})',
      'invariantsUnderCitta := infer(stableFeatures over all citta)',
      'variantsUnderCitta := infer(variableFeatures keyed_by citta)',
    ],
  },
  {
    id: 'ys-iv-15-hlo-worldness',
    chunkId: 'ys-iv-15-worldness',
    label: 'Worldness',
    clauses: [
      'worldness := stabilizeAcross(citta, invariantsUnderCitta)',
      'linkFromIdentity := link(IV_14.parinamaEkatva → worldness)',
    ],
  },
  {
    id: 'ys-iv-15-hlo-stances',
    chunkId: 'ys-iv-15-stances',
    label: 'Stances',
    clauses: [
      'purusha := seer(unchanging, nonAgentive)',
      'pumanJiva := agent(adaptive, withinAppearance)',
      'contrib(cittaBheda) := from(pumanJiva ∧ conditioning({guna, samskara, vasana}))',
    ],
  },
  {
    id: 'ys-iv-15-hlo-errors',
    chunkId: 'ys-iv-15-errors',
    label: 'Errors',
    clauses: [
      'errorNaiveRealism := flag(assert(allAppearancesIdentical across citta))',
      'errorSolipsism := flag(deny(invariantsUnderCitta))',
    ],
  },
]

// ---------- Fichte L20–L21 crosswalk: “from” ↔ panthāḥ (paths) ----------
Object.assign(YS_IV_15_ONTOLOGY, {
  fichteLightFrom: 'Light is a “from”; in its second power (appearance) light itself is genesis',
  seeingEqualsGenesis: 'Genesis and seeing converge unconditionally (appearance-as-genesis)',
  disjunctionVsOneness:
    'Disjunction in the “from” (vibhakta) vs essential qualitative oneness of the “from” (light)',
  panthahAsFromTrajectories:
    'Panthāḥ (paths) as sequences/trajectories of “from”-relations under the same vastu',
  performativeJustification:
    'Presupposition justified by deed: we knew ourselves as the knowing that posits the “from”',
  lassitudeToEnergyExample:
    'Example: transition from lassitude to energy illustrates appearing genesis (path-level change)',
})

CHUNKS_YS_IV_15.push(
  {
    id: 'ys-iv-15-fichte-crosswalk',
    title: 'Fichte: “From” and Paths',
    summary:
      'Vibhakta panthāḥ as disjunctions of “from”-trajectories; invariants = qualitative oneness of light.',
  },
  {
    id: 'ys-iv-15-fichte-example',
    title: 'Example: Lassitude → Energy',
    summary:
      'Concrete illustration of appearance-as-genesis; path change without losing underlying oneness.',
  },
)

HLOS_YS_IV_15.push(
  {
    id: 'ys-iv-15-hlo-fichte-crosswalk',
    chunkId: 'ys-iv-15-fichte-crosswalk',
    label: 'Crosswalk',
    clauses: [
      'panthahAsFromTrajectories := define(paths(appearance) == trajectories(of = "from"))',
      'disjunctionVsOneness := assert(vibhakta(panthah) ∧ invariant(qualitativeOneness(light)))',
      'link(vibhaktaPanthah, disjunctionVsOneness)',
      'seeingEqualsGenesis := assert(genesis(appearance) ≡ seeing(light))',
      'lawOfAppearance := enrich(lawOfAppearance, with = seeingEqualsGenesis)',
    ],
  },
  {
    id: 'ys-iv-15-hlo-fichte-example',
    chunkId: 'ys-iv-15-fichte-example',
    label: 'Illustration',
    clauses: [
      'lassitudeToEnergyExample := model(transition(state: lassitude → energy), as = pathChange)',
      'invariantsUnderCitta := mark(preserve(qualitativeOneness(light)))',
      'variantsUnderCitta := mark(diverge(trajectories by cittaBheda))',
      'performativeJustification := note(we enact knowing-as-"from" while asserting it)',
    ],
  },
)

// ---------- Export Unit ----------
export const YS_IV_15_UNIT: DatasetUnit = {
  id: makeUnitId('ys-iv-15'),
  title: 'YS IV.15 — vastu-sāmye citta-bhedāt tayoḥ vibhaktaḥ panthāḥ',
  scope: 'appearance',
  logosMode: 'appearance',
  synthesis: 'law-of-appearance-worldness',
  faculty: 'buddhi',
  lens: 'yoga',
  chunks: CHUNKS_YS_IV_15 as any,
  hlos: HLOS_YS_IV_15 as any,
}

export const YS_IV_15_SYMBOLS = Object.keys(YS_IV_15_ONTOLOGY);

