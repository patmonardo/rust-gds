import { DatasetUnit, makeUnitId } from '@organon/gdsl/registry/canon'

/*
YS IV.17 — tad-upārāgāpekṣitvāc cittasya vastu jñāta-ajñātam

“The object is known or unknown for the mind depending on its coloring/overlap (upārāga).”

Theme
- World/Thing/Essence thread continues: knownness is conditioned by citta-structure.
- Hold vastu realism at bay: unknown ≠ non-existent; this is epistemic gating.
- Crosswalk: IV.15 (citta-bheda) → IV.16 (not one-mind dependent) → IV.17 (knownness law).
- Fichte: appearance-as-genesis; light’s permeation; performative criterion.
- Vedānta stance: Puruṣa as essential/secondary knowing; pointer beyond to Brahman.
*/

// ---------- Ontology ----------
export const YS_IV_17_ONTOLOGY = {
  vastu: 'Object within appearance (stabilized by identity across change, IV.14)',
  citta: 'Mind-stream/locus of appearing',
  uparaga: 'Coloring/affection/overlap of citta with vastu (contact/attunement condition)',
  apekshitvat: 'Dependence/conditionality “depending on”',
  jnata: 'Known (cognized for this citta)',
  ajnata: 'Unknown (not cognized for this citta)',
  knownnessAsFunction:
    'Known/unknown status of vastu is a function of citta’s upārāga (not an ontic toggle on vastu)',
  lawOfKnownness:
    'Rule: know(vastu | citta) ⇐ upārāga(citta, vastu); else unknown for that citta',
  invariantsVsVariants:
    'Invariants across cittas (IV.15–IV.16) vs variants of knownness by upārāga',
  // Crosswalks
  linkWorldness: 'Built on IV.15 citta-bheda and IV.16 non-solipsistic objectivity',
  purushaSecondaryKnowing:
    'Puruṣa as essential/secondary knowing (mountains and rivers: stabilized appearance)',
  brahmanPointer: 'Pointer beyond essential relation toward absolute knowing',
  fichteSeeingEqualsGenesis:
    'Seeing ≡ genesis: knownness co-arises with the operative genesis (energy/abstraction) of light',
  performativeValidation: 'Say/do coincide: enact the condition that yields knownness',
  // Guards
  errorUnknownAsNonexistent: 'Confusing “unknown” with “non-existent”',
  errorMindCreationism:
    'Claiming vastu depends on one mind (refuted by IV.16); here only knownness depends on citta',
}

// ---------- Chunks ----------
const CHUNKS_YS_IV_17 = [
  {
    id: 'ys-iv-17-text',
    title: 'IV.17 Text & Baseline',
    summary: 'Known/unknown depends on citta’s upārāga with respect to the vastu.',
  },
  {
    id: 'ys-iv-17-parse',
    title: 'Parse & Semantics',
    summary: 'Define upārāga (coloring/overlap); knownness as epistemic gating, not ontic switch.',
  },
  {
    id: 'ys-iv-17-law',
    title: 'Law of Knownness',
    summary: 'know(vastu | citta) ⇐ upārāga(citta, vastu); else unknown-for-that-citta.',
  },
  {
    id: 'ys-iv-17-crosswalk',
    title: 'Crosswalk (Worldness/Fichte/Vedānta)',
    summary:
      'Link to IV.15–IV.16 invariants; Fichte’s appearance-as-genesis; Puruṣa vs Brahman pointer.',
  },
  {
    id: 'ys-iv-17-errors',
    title: 'Error Modes',
    summary: 'Unknown ≠ non-existent; avoid one-mind dependence; avoid mere consensus as validation.',
  },
]

// ---------- HLO Clauses ----------
const HLOS_YS_IV_17 = [
  {
    id: 'ys-iv-17-hlo-text',
    chunkId: 'ys-iv-17-text',
    label: 'Baseline',
    clauses: [
      "tag('sutra','IV.17')",
      'assert(depends(knownness(vastu, for = citta), on = uparaga(citta, vastu)))',
    ],
  },
  {
    id: 'ys-iv-17-hlo-parse',
    chunkId: 'ys-iv-17-parse',
    label: 'Semantics',
    clauses: [
      'define(uparaga := overlap/coloring/attunement(citta ↔ vastu))',
      'knownnessAsFunction := clarify(epistemic(known/unknown) ≠ ontic(exist/not))',
    ],
  },
  {
    id: 'ys-iv-17-hlo-law',
    chunkId: 'ys-iv-17-law',
    label: 'Law',
    clauses: [
      'lawOfKnownness := rule(know(vastu | citta) ⇐ sufficient(uparaga(citta, vastu)))',
      'else(unknown(vastu | citta))',
      'invariantsVsVariants := relate({IV_15.invariantsUnderCitta, IV_15.variantsUnderCitta}, to = knownness)',
    ],
  },
  {
    id: 'ys-iv-17-hlo-crosswalk',
    chunkId: 'ys-iv-17-crosswalk',
    label: 'Crosswalk',
    clauses: [
      'linkWorldness := link(IV_15.lawOfAppearance ∧ IV_16.essentialRelation → lawOfKnownness)',
      'fichteSeeingEqualsGenesis := assert(seeing ≡ genesis; knownness coArises with operative genesis)',
      'performativeValidation := require(say(x) == do(x) at condition(uparaga))',
      'purushaSecondaryKnowing := scope(knownness within appearance)',
      'brahmanPointer := note(beyond(essentialRelation))',
    ],
  },
  {
    id: 'ys-iv-17-hlo-errors',
    chunkId: 'ys-iv-17-errors',
    label: 'Errors',
    clauses: [
      'errorUnknownAsNonexistent := flag(confuse(unknown, nonExistent))',
      'errorMindCreationism := flag(assert(ekaCittaTantram(vastu)))',
      'flag(assert(validation by mereConsensus))',
    ],
  },
]

// ---------- Fichte L20 wrap-up: necessity, proof nerve, prerequisites ----------
Object.assign(YS_IV_17_ONTOLOGY, {
  lightEqualsFrom: 'light = “from” (identity reiterated)',
  permeationNecessary: 'If light is to be, its permeation of the “from” is necessary (a priori)',
  performativeProofNerve:
    'We ourselves are knowing; since we can and do know only thus, knowing is constituted thus',
  errorOutsideKnowing: 'Error: searching for knowing outside of knowing',
  competenceDiscipline:
    'Clarity requires capacity and discipline (sharp thinking, strong attention); otherwise barred from judgment',
  apexNotRequired:
    'Even before the apex, the method and its clarity stand; they are not transferrable without prerequisites',
  defenseAgainstIgnorance:
    'Use the performative criterion and prerequisite reminder as a defense against misreadings',
})

CHUNKS_YS_IV_17.push(
  {
    id: 'ys-iv-17-fichte-wrap',
    title: 'Fichte L20 — Conclusion',
    summary:
      'Light permeates the “from” by necessity; identity light = “from”; performative proof nerve; defense against the ignorant.',
  },
  {
    id: 'ys-iv-17-prereq',
    title: 'Prerequisites and Discipline',
    summary:
      'Capacity and discipline condition access to clarity; avoid seeking knowing outside of knowing.',
  },
)

HLOS_YS_IV_17.push(
  {
    id: 'ys-iv-17-hlo-fichte-wrap',
    chunkId: 'ys-iv-17-fichte-wrap',
    label: 'Necessity & Proof',
    clauses: [
      'lightEqualsFrom := assert(light ≡ "from")',
      'permeationNecessary := assert(necessary(permeate(light, "from")))',
      'performativeProofNerve := assert(we_are(knowing) ∧ we_know_only_thus ⇒ thus_is(knowing))',
      'errorOutsideKnowing := flag(search(knowing, outside(knowing)))',
      'defenseAgainstIgnorance := advise(use(performativeProofNerve ∧ method))',
    ],
  },
  {
    id: 'ys-iv-17-hlo-prereq',
    chunkId: 'ys-iv-17-prereq',
    label: 'Prereq',
    clauses: [
      'competenceDiscipline := require({capacity(sharpThinking), discipline(strongAttention)})',
      'apexNotRequired := note(method_clarity_before(apex))',
    ],
  },
)

// ---------- Export Unit ----------
export const YS_IV_17_UNIT: DatasetUnit = {
  id: makeUnitId('ys-iv-17'),
  title: 'YS IV.17 — tad-upārāgāpekṣitvāc cittasya vastu jñāta-ajñātam',
  scope: 'appearance',
  logosMode: 'appearance',
  synthesis: 'law-of-knownness',
  faculty: 'buddhi',
  lens: 'yoga',
  chunks: CHUNKS_YS_IV_17 as any,
  hlos: HLOS_YS_IV_17 as any,
}

export const YS_IV_17_SYMBOLS = Object.keys(YS_IV_17_ONTOLOGY)
