import { DatasetUnit, makeUnitId } from '@organon/gdsl/registry/canon'

/*
YS IV.14 — pariṇāmaikatvād vastu tattvam

Seed for lecture analysis (Fichte L20–L21 “from/through” ↔ pariṇāma as “change-of-name”):
- Fichte’s “from” (genetic connection) maps to pariṇāma (processual transformation).
- “Change of name” (pariṇāma) = the same underlying one shows up under different determinate names/states.
- Identity across change (ekatva of pariṇāma) grounds thingness/truth (vastu-tattvam) within appearance.
- Bridge: IV.12 (time as appearance) → IV.13 (guṇa-grounded state spectrum) → IV.14 (identity across change).

We’ll attach detailed notes from 20th–21st Lectures here (Appearance week: 2 dense talks).
*/

// ---------- Ontology (single source of truth) ----------
export const YS_IV_14_ONTOLOGY = {
  parinama: 'Transformation/process; “change-of-name” (nāma-parivartana) within appearance',
  ekatva: 'Oneness/identity (sameness that persists)',
  parinamaEkatva: 'Oneness of transformation: identity-across-change',
  vastu: 'Thingness (stabilized appearance under lawful transformation)',
  tattvam: 'Truth/reality-of-vastu within appearance',

  // Semantics/crosswalk
  nameChangeSemantics: 'Same underlying one appears under differing names/states via process',
  fromOperatorParinama: 'Crosswalk: Fichte’s “from/through” ↔ pariṇāma (genetic connection as process)',
  appearanceOfEffect: 'Qualitative relation is the appearance of absolute effect (Fichte L19–L20)',

  // Lawful basis and bridges
  lawfulTransitions: 'Transitions grounded in substrate/law (e.g., guṇa-configs from IV.13)',
  timeToStateBridge: 'IV.12 adhva-bheda (time modalities) feeds IV.13 state spectrum',
  stateToIdentityBridge: 'IV.13 spectrum feeds IV.14 identity-across-change',
  worldnessBridge: 'Prepares IV.15 worldness (invariants/variants with vastu held constant)',

  // Method anchors
  presentAppearanceAsSuch: 'Present appearance genetically (principle-level, not empirical)',
  identityAsDerived: 'Identity is derived immanently (no external postulate)',
  sayDoCriterion: 'Validity: saying coincides with doing (performative consistency)',

  // Errors
  errorStaticSubstance: 'Error: reifying a static thing-in-itself behind appearance',
  errorIdentityWithoutProcess: 'Error: asserting identity without lawful transitional basis',
}

// ---------- Chunks ----------
const CHUNKS_YS_IV_14 = [
  {
    id: 'ys-iv-14-text',
    title: 'IV.14 Text & Baseline',
    summary: 'pariṇāma-ikatvād vastu-tattvam — thingness is true by oneness of transformation.',
  },
  {
    id: 'ys-iv-14-parse',
    title: 'Parse & Semantics',
    summary: 'Pariṇāma as “change-of-name” with process; ekatva as the persisting same.',
  },
  {
    id: 'ys-iv-14-criterion',
    title: 'Criterion: Identity Across Change',
    summary: 'Define how lawful transitions ground stabilized appearance (vastu).',
  },
  {
    id: 'ys-iv-14-crosswalk',
    title: 'Crosswalk (Fichte/Hegel)',
    summary: '“From/through” ↔ pariṇāma; appearance-of-effect; identity as derived.',
  },
  {
    id: 'ys-iv-14-bridges',
    title: 'Bridges (IV.12 → IV.13 → IV.14 → IV.15)',
    summary: 'Time-form → state-spectrum → identity → worldness.',
  },
  {
    id: 'ys-iv-14-errors',
    title: 'Error Modes',
    summary: 'Avoid static-substance realism; avoid identity without process.',
  },
  {
    id: 'ys-iv-14-lectures',
    title: 'Notes Placeholder — Fichte L20–L21',
    summary: 'Attach detailed excerpts/analyses from the two dense Appearance lectures.',
  },
]

// ---------- HLO Clauses ----------
const HLOS_YS_IV_14 = [
  {
    id: 'ys-iv-14-hlo-text',
    chunkId: 'ys-iv-14-text',
    label: 'Baseline',
    clauses: [
      "tag('sutra','IV.14')",
      'parinamaEkatva := assert(ekatva(parinama))',
      'tattvam(vastu) ⇐ parinamaEkatva',
    ],
  },
  {
    id: 'ys-iv-14-hlo-parse',
    chunkId: 'ys-iv-14-parse',
    label: 'Semantics',
    clauses: [
      'define(parinama := process(changeOfName))',
      'nameChangeSemantics := assert(sameUnderlyingOne shows_as differentStatesNames via parinama)',
    ],
  },
  {
    id: 'ys-iv-14-hlo-criterion',
    chunkId: 'ys-iv-14-criterion',
    label: 'Identity',
    clauses: [
      'lawfulTransitions := require(groundedTransitions by IV_13.guna)',
      'identityAcrossChange := criterion(sameThroughTransitions(lawfulTransitions))',
      'define(vastu := stabilized(appearance) by identityAcrossChange)',
      'assert(tattvam(vastu))',
    ],
  },
  {
    id: 'ys-iv-14-hlo-crosswalk',
    chunkId: 'ys-iv-14-crosswalk',
    label: 'Method',
    clauses: [
      'fromOperatorParinama := map(Fichte.from_through ↔ parinama)',
      'appearanceOfEffect := link(Fichte.appearanceOfEffect, to = parinama_view)',
      'presentAppearanceAsSuch := assert(principleLevel(appearance))',
      'identityAsDerived := assert(derived(identity) ∧ sayDoCriterion)',
    ],
  },
  {
    id: 'ys-iv-14-hlo-bridges',
    chunkId: 'ys-iv-14-bridges',
    label: 'Bridges',
    clauses: [
      'timeToStateBridge := link(IV_12.adhvaBheda → IV_13.stateSpectrum)',
      'stateToIdentityBridge := link(IV_13.stateSpectrum → identityAcrossChange)',
      'worldnessBridge := prepare(IV_15.lawOfAppearance)',
    ],
  },
  {
    id: 'ys-iv-14-hlo-errors',
    chunkId: 'ys-iv-14-errors',
    label: 'Errors',
    clauses: [
      'errorStaticSubstance := flag(reify(thingInItself behind appearance))',
      'errorIdentityWithoutProcess := flag(assert(identityAcrossChange without(lawfulTransitions)))',
    ],
  },
  {
    id: 'ys-iv-14-hlo-lectures',
    chunkId: 'ys-iv-14-lectures',
    label: 'Notes',
    clauses: [
      'note("Attach L20–L21 excerpts: absolute from (invisible), appearance of effect, manifold-in-light")',
      'todo("Integrate quotes and performative validation links")',
    ],
  },
]

// ---------- Fichte L20–L21: “from” figures and creation levels ----------
Object.assign(YS_IV_14_ONTOLOGY, {
  lightPositsFrom: 'A “from” is posited immediately through the light (L: a — b)',
  lightIdenticalWithFrom:
    'If light ≡ “from”, light spreads in unchanged qualitative oneness across every “from”',
  nestedFromsReconstruction:
    'Secondary splitting of the original “from” is reconstructible purely a priori (no empirical presupposition)',
  principleOfPrinciple:
    'Higher task: present the principle of the principle (not merely its a priori reconstruction)',
  firstCreationAbsoluteFrom:
    'Pure, absolute, immediate “from” as self-positing of original light: first and absolute creation; ground of “is”',
  secondCreationDisjunction:
    'Disjunction within the “from” (in divided light) = second re‑creation in intuition; living reduced to dead being',
  worldPuzzleResolution:
    'Science of Knowing resolves the puzzle of world and consciousness by this genesis',
})

// Chunks to surface Fichte’s constructions
CHUNKS_YS_IV_14.push(
  {
    id: 'ys-iv-14-fichte-from-figures',
    title: 'Fichte: “From” Figures',
    summary:
      'L: a—b posited by light; identity of light and “from”; spread across nested “froms”.',
  },
  {
    id: 'ys-iv-14-fichte-creation-levels',
    title: 'Fichte: Creation Levels',
    summary:
      'First creation: absolute “from”; second re‑creation: disjunction in intuition; world‑puzzle resolution.',
  },
)

HLOS_YS_IV_14.push(
  {
    id: 'ys-iv-14-hlo-fichte-from-figures',
    chunkId: 'ys-iv-14-fichte-from-figures',
    label: 'Figures',
    clauses: [
      'lightPositsFrom := assert(derive("from", from = light))',
      'lightIdenticalWithFrom := assert(light ≡ "from" ∧ spreads(qualitativeOneness, across = all("from")))',
      'nestedFromsReconstruction := assert(reconstruct(a_priori, nested("from"), noEmpiricalPresupposition))',
      'principleOfPrinciple := task(present(principle(principle)))',
      'fromOperatorParinama := map("from" ↔ parinama)',
    ],
  },
  {
    id: 'ys-iv-14-hlo-fichte-creation-levels',
    chunkId: 'ys-iv-14-fichte-creation-levels',
    label: 'Creation',
    clauses: [
      'firstCreationAbsoluteFrom := assert(selfPosit(light) == absolute("from") ∧ ground("is"))',
      'secondCreationDisjunction := assert(disjoin(within = "from", in = dividedLight) ⇒ mereIntuition(deadBeing))',
      'worldPuzzleResolution := conclude(resolve(puzzle(world ∧ consciousness), by = genesis("from"))) ',
    ],
  },
)

// ---------- Samkhya crosswalk: aviśeṣa/viśeṣa, liṅgamātra ----------
Object.assign(YS_IV_14_ONTOLOGY, {
  avisesa: 'Undifferentiated universals (Sāṃkhya) — crosswalk to abstract reason',
  visesa: 'Particulars (Sāṃkhya) — crosswalk to determinate appearance',
  lingamātra:
    'Liṅgamātra (mark-only, subtle principle) — crosswalk to pure reason/inscrutable oneness of light',
  avisesaVisesaParinama:
    'Pariṇāma mediates aviśeṣa ↔ viśeṣa as lawful differentiation without loss of identity',
})

CHUNKS_YS_IV_14.push(
  {
    id: 'ys-iv-14-samkhya-crosswalk',
    title: 'Sāṃkhya Crosswalk',
    summary:
      'aviśeṣa/viśeṣa and liṅgamātra mapped to abstract/pure reason; pariṇāma as mediation.',
  },
)

HLOS_YS_IV_14.push(
  {
    id: 'ys-iv-14-hlo-samkhya-crosswalk',
    chunkId: 'ys-iv-14-samkhya-crosswalk',
    label: 'Crosswalk',
    clauses: [
      'map(avisesa ↔ abstractReason)',
      'map(visesa ↔ determinateAppearance)',
      'map(lingamātra ↔ pureReason/inscrutableOneness)',
      'avisesaVisesaParinama := assert(mediate(aviśeṣa, viśeṣa, by = parinamaEkatva))',
    ],
  },
)

// ---------- Export Unit ----------
export const YS_IV_14_UNIT: DatasetUnit = {
  id: makeUnitId('ys-iv-14'),
  title: 'YS IV.14 — pariṇāmaikatvād vastu tattvam',
  scope: 'appearance',
  logosMode: 'appearance',
  synthesis: 'identity-across-change → thingness',
  faculty: 'buddhi',
  lens: 'yoga',
  chunks: CHUNKS_YS_IV_14 as any,
  hlos: HLOS_YS_IV_14 as any,
}

export const YS_IV_14_SYMBOLS = Object.keys(YS_IV_14_ONTOLOGY)
