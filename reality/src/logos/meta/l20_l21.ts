import { DatasetUnit, makeUnitId } from '@organon/gdsl/registry/canon'

/*
Fichte 1804 — Lectures 20–21 (Appearance / Witness / Principle)

Aim
- Present appearance as such; light = “from” in qualitative oneness.
- Hypothetical “should” = postulation of genesis; absolute genesis self-enclosed.
- Immediate seeing (idealism) as working principle; inner self‑genesis of I.
- Higher knowing: positive non‑self‑genesis; projection per hiatum; reason/understanding hinge.
- Bridge cleanly into YS IV.18–IV.21.

Scope: crosswalk/meta for reuse in YS and beyond.
*/

// ---------- Ontology ----------
export const FICHTE_L20_L21_ONTOLOGY = {
  light: 'Qualitative oneness of appearing',
  fromOp: '“From/through” operator (genetic)',
  lightEqualsFrom: 'Identity: light = “from” (qualitative oneness)',
  abstractionToPureLight: 'Abstract from all content → pure light (absolute knowing-as-such)',
  seeingEqualsGenesis: 'Seeing ≡ genesis (appearance-as-genesis)',
  shouldHypothesis: 'The hypothetical “should” as postulation of genesis',
  postulationAsGenesis: 'Postulation is a (ideal) genesis',
  absoluteGenesisSelfEnclosed: 'Absolute genesis/light is self-enclosed, never goes outside itself',
  realIdealDisjunction: 'Disjunction within one genesis: real vs ideal (aspectual, not two things)',
  immediateSeeingPrinciple: 'Take immediate seeing/life as provisional absolute (idealism)',
  dropAscentMeans: 'Let ascent constructs go; recover on descent',
  innerSelfGenesis: 'Living self‑genesis of I (light and We merge)',
  higherKnowingNonSelfGenesis: 'Higher knowing = positive non‑self‑genesis, yet immanent I',
  positiveNegationAsBeing: 'Positive negation of genesis = enduring being (objective being of knowing)',
  projectionPerHiatum: 'Necessary gap separating pure reason’s oneness from appearance',
  reasonUnderstandingHinge:
    'No insight into reason without presupposing understanding as absolute; no insight into understanding except via reason’s negation',
  persistenceAsGenesis: 'Pure being/persistence = genesis at qualitative oneness',
}

// ---------- Chunks ----------
const CHUNKS_FICHTE_L20_L21 = [
  { id: 'fichte-l20-from', title: 'L20 — Light = “From”', summary: 'Identity; abstraction; seeing ≡ genesis.' },
  { id: 'fichte-l20-genesis', title: 'L20 — Indivisible Genesis', summary: 'Self‑enclosure; real/ideal within one.' },
  { id: 'fichte-l21-should', title: 'L21 — “Should” as Genesis', summary: 'Postulation; organic law; method.' },
  { id: 'fichte-l21-principle', title: 'L21 — Immediate Seeing Principle', summary: 'Idealism as working ground.' },
  { id: 'fichte-l21-self', title: 'L21 — Inner Self‑Genesis / Higher Knowing', summary: 'I emerges; positive non‑self‑genesis; hiatus.' },
  { id: 'fichte-l21-hinge', title: 'L21 — Reason ↔ Understanding', summary: 'Mutual presupposition; persistence = genesis.' },
  { id: 'fichte-bridges-ys', title: 'Bridges to YS IV.18–IV.21', summary: 'Witness; no self‑luminous citta; no double determination.' },
]

// ---------- HLOs ----------
const HLOS_FICHTE_L20_L21 = [
  {
    id: 'fichte-l20-hlo-from',
    chunkId: 'fichte-l20-from',
    label: 'From-Operator',
    clauses: [
      'lightEqualsFrom := assert(light ≡ "from")',
      'abstractionToPureLight := assert(abstract(allContent) ⇒ pureLight)',
      'seeingEqualsGenesis := assert(seeing ≡ genesis)',
    ],
  },
  {
    id: 'fichte-l20-hlo-genesis',
    chunkId: 'fichte-l20-genesis',
    label: 'Genesis',
    clauses: [
      'absoluteGenesisSelfEnclosed := assert(selfEnclosed(genesis))',
      'realIdealDisjunction := pose(disjunctionWithin(one(genesis), aspects = {real, ideal}))',
      'dropAscentMeans := advise(drop(ascentConstructs) ▷ recover_on(descent))',
    ],
  },
  {
    id: 'fichte-l21-hlo-should',
    chunkId: 'fichte-l21-should',
    label: '“Should”',
    clauses: [
      'shouldHypothesis := assert(hypothesis("should") == postulate(genesis))',
      'postulationAsGenesis := assert(postulation == idealGenesis)',
    ],
  },
  {
    id: 'fichte-l21-hlo-principle',
    chunkId: 'fichte-l21-principle',
    label: 'Principle',
    clauses: [
      'immediateSeeingPrinciple := assert(principle = immediateSeeing)',
      'performative := require(say(x) == do(x))',
    ],
  },
  {
    id: 'fichte-l21-hlo-self',
    chunkId: 'fichte-l21-self',
    label: 'Self/Hiatus',
    clauses: [
      'innerSelfGenesis := assert(merge(light, We_I) at immediate(selfGenesis))',
      'higherKnowingNonSelfGenesis := assert(nonSelfGenesis(higherKnowing) ∧ immanent(I))',
      'positiveNegationAsBeing := assert(posNeg(genesis) == enduring(being_of_knowing))',
      'projectionPerHiatum := assert(necessary_gap separates(pureReason, appearance))',
    ],
  },
  {
    id: 'fichte-l21-hlo-hinge',
    chunkId: 'fichte-l21-hinge',
    label: 'Hinge',
    clauses: [
      'reasonUnderstandingHinge := assert(no_insight(reason) without presuppose(understanding as absolute) ∧ no_insight(understanding) except_via(negation_by reason))',
      'persistenceAsGenesis := assert(being == genesis at qualitative_oneness)',
    ],
  },
  {
    id: 'fichte-hlo-bridges-ys',
    chunkId: 'fichte-bridges-ys',
    label: 'YS Bridges',
    clauses: [
      'link(IV_18.purusha ← immediateSeeingPrinciple)',
      'link(IV_19.notSelfLuminousCitta ← absoluteGenesisSelfEnclosed)',
      'link(IV_20.noDoubleDetermination ← method(organic_law))',
      'link(IV_21.seerWithoutObjecthood ← projectionPerHiatum)',
    ],
  },
]

// ---------- Export Unit ----------
export const FICHTE_L20_L21_UNIT: DatasetUnit = {
  id: makeUnitId('fichte-l20-l21'),
  title: 'Fichte 1804 — Lectures 20–21 (Appearance/Witness)',
  scope: 'meta',
  logosMode: 'appearance',
  synthesis: 'appearance-as-genesis; immediate-seeing principle; hinge(reason,understanding)',
  faculty: 'buddhi',
  lens: 'fichte',
  chunks: CHUNKS_FICHTE_L20_L21 as any,
  hlos: HLOS_FICHTE_L20_L21 as any,
}

export const FICHTE_L20_L21_SYMBOLS = Object.keys(FICHTE_L20_L21_ONTOLOGY)
