import { DatasetUnit, makeUnitId } from '@organon/gdsl/registry/canon'

/*
YS IV.19 — na tat sva-ābhāsam dṛśyatvāt
“It (the citta) is not self-luminous, because it is seen (is an object).”

Reading
- Citta (mind-stream) is an object of seeing; therefore not self-illuminating.
- Purusha (unchanging witness) is the illuminator; citta borrows light (reflective sattva), it does not generate it.
- Continues the Thing/World → Essential Relation sequence: IV.18 (witness invariance) → IV.19 (mind is seen, not seer).
- Fichte L21 crosswalk: unchanging qualitative oneness of light vs the “from” as seen; avoid collapsing seer and seen.
*/

// ---------- Ontology ----------
export const YS_IV_19_ONTOLOGY = {
  citta: 'Mind-stream/locus of appearance (object here)',
  drsyatvat: 'Because of being seen (object-status)',
  svaAbhasam: 'Self-illumination/self-appearance (denied of citta)',
  notSelfLuminousCitta: 'Citta is not self-luminous (na tat sva-ābhāsam)',
  purusha: 'Unchanging witness (seer-only, akartṛtva)',
  borrowedLight: 'Citta appears by borrowed light (reflection of witness), not its own luminosity',
  reflectiveSattva: 'Sāttvika transparency/reflection as citta’s mode of appearing',
  witnessObjectDivide: 'Seer/seen distinction at essential level: Purusha ≠ citta',
  regressWarning: 'If citta were self-luminous, reflexive knowing would induce regress (handled in IV.20–IV.21)',
  // Crosswalks
  linkIV18Witness: 'Builds on IV.18: always-known vṛttis due to unchanging witness',
  fichteQualitativeOneness: 'Unchanging qualitative oneness of light (seer) vs the seen “from”',
  // Guards
  errorMindAsSeer: 'Confusing citta with the seer (Purusha)',
  errorDenyCittaFunction: 'Denying citta’s reflective/functional role within appearance',
}

// ---------- Chunks ----------
const CHUNKS_YS_IV_19 = [
  {
    id: 'ys-iv-19-text',
    title: 'IV.19 Text & Baseline',
    summary: 'Citta is not self-illuminating because it is seen (object of awareness).',
  },
  {
    id: 'ys-iv-19-semantics',
    title: 'Semantics: sva-ābhāsa and dṛśyatvāt',
    summary: 'Define self-illumination vs object-status; citta borrows light, it doesn’t generate it.',
  },
  {
    id: 'ys-iv-19-crosswalk',
    title: 'Crosswalk (IV.18 and Fichte)',
    summary: 'Witness invariance (Purusha) vs seen mind; qualitative oneness of light vs “from.”',
  },
  {
    id: 'ys-iv-19-bridges',
    title: 'Bridges → IV.20–IV.21',
    summary: 'Head off regress; prepare the simultaneity/determination constraints.',
  },
  {
    id: 'ys-iv-19-errors',
    title: 'Error Modes',
    summary: 'Avoid citta≡seer and nihilism about citta’s reflective function.',
  },
]

// ---------- HLO Clauses ----------
const HLOS_YS_IV_19 = [
  {
    id: 'ys-iv-19-hlo-text',
    chunkId: 'ys-iv-19-text',
    label: 'Baseline',
    clauses: [
      "tag('sutra','IV.19')",
      'assert(notSelfLuminousCitta)',
      'reason(notSelfLuminousCitta, because(drsyatvat))',
    ],
  },
  {
    id: 'ys-iv-19-hlo-semantics',
    chunkId: 'ys-iv-19-semantics',
    label: 'Semantics',
    clauses: [
      'define(svaAbhasam := selfIllumination/selfAppearance)',
      'define(drsyatvat := objectStatus(beingSeen))',
      'borrowedLight := assert(citta.appears_by(reflection(purusha)))',
      'reflectiveSattva := gloss(citta_mode := sattva_transparency)',
    ],
  },
  {
    id: 'ys-iv-19-hlo-crosswalk',
    chunkId: 'ys-iv-19-crosswalk',
    label: 'Crosswalk',
    clauses: [
      'linkIV18Witness := link(IV_18.alwaysKnownForPurusha → notSelfLuminousCitta)',
      'witnessObjectDivide := assert(purusha ≠ citta ∧ purusha = seer ∧ citta = seen)',
      'fichteQualitativeOneness := map(seer(light_qualitative_oneness) vs seen("from"))',
    ],
  },
  {
    id: 'ys-iv-19-hlo-bridges',
    chunkId: 'ys-iv-19-bridges',
    label: 'Bridges',
    clauses: [
      'regressWarning := note(if(citta selfIlluminates) ⇒ regress(reflexive_knowing))',
      'prepare(IV_20_constraint := simultaneity/dual-determination_limit)',
      'prepare(IV_21_resolution := seer-status without objecthood)',
    ],
  },
  {
    id: 'ys-iv-19-hlo-errors',
    chunkId: 'ys-iv-19-errors',
    label: 'Errors',
    clauses: [
      'errorMindAsSeer := flag(confuse(citta, purusha))',
      'errorDenyCittaFunction := flag(deny(reflective_function(citta)))',
    ],
  },
]

// ---------- Fichte L21 (Part 5): Choice of principle — Immediate seeing (idealism) ----------
Object.assign(YS_IV_19_ONTOLOGY, {
  immediateSeeingAsAbsolute:
    'Take immediate seeing/life as provisional absolute; never the objectified “is”',
  remainInLife:
    'To proceed, we must stay within living knowing, not fixate on the object as principle',
  neverObjectivityAsPrinciple:
    'Do not use objectivity as first principle; if idealism is insufficient, seek a higher unifying principle',
  idealismVsRealismBridge:
    'Science stands between idealism and realism; necessity pushes toward idealism, aiming at higher unity',
})

CHUNKS_YS_IV_19.push({
  id: 'ys-iv-19-fichte21-p5',
  title: 'Fichte L21 — Part 5: Immediate Seeing as Principle',
  summary:
    'Choose living, immediate seeing (idealism) over objectified “is”; never use objectivity as principle; aim at higher unity.',
})

HLOS_YS_IV_19.push({
  id: 'ys-iv-19-hlo-fichte21-p5',
  chunkId: 'ys-iv-19-fichte21-p5',
  label: 'Principle Choice',
  clauses: [
    'immediateSeeingAsAbsolute := assert(principle == immediate_seeing ∧ ¬objectified_is)',
    'remainInLife := require(continue(research) ⇒ stay_within(living_knowing))',
    'neverObjectivityAsPrinciple := rule(¬use(objectivity, as = first_principle))',
    'idealismVsRealismBridge := note(stand_between(idealism, realism) ⇒ seek(higher_unity))',
    'link(linkIV18Witness, immediateSeeingAsAbsolute)',
  ],
})

// ---------- Fichte L21 (Part 6): Inner self-genesis; inseparable seeing/genesis; higher knowing ----------
Object.assign(YS_IV_19_ONTOLOGY, {
  innerSelfGenesis:
    'Presuppose inner self-genesis as living oneness (qualitative light), to be enacted, not merely understood',
  inseparableSeeingGenesis:
    'Seeing and arising of the absolute “from” are inseparable; genesis-of-self (I) in immediate light',
  weLightMerge:
    'Light and We/I merge purely in immediate self-genesis',
  uniteObjectiveAspectInKnowing:
    'Objective aspect may stand only insofar as united within knowing',
  higherKnowingUnites:
    'Assume a principle for absolute inner living self-genesis; a higher knowing unites subjective and objective',
  subordinateThroughHigher:
    'Two subordinate terms are mediated through the higher principle',
})

CHUNKS_YS_IV_19.push({
  id: 'ys-iv-19-fichte21-p6',
  title: 'Fichte L21 — Part 6: Self‑Genesis and Higher Knowing',
  summary:
    'Enact inner self-genesis as living oneness; seeing≡genesis; I emerges; higher knowing unites subjective/objective.',
})

HLOS_YS_IV_19.push({
  id: 'ys-iv-19-hlo-fichte21-p6',
  chunkId: 'ys-iv-19-fichte21-p6',
  label: 'Genesis/Unity',
  clauses: [
    'innerSelfGenesis := assert(presuppose(self_genesis as living_oneness(qualitative_light)) ∧ enact)',
    'inseparableSeeingGenesis := assert(seeing ≡ arising(absolute_from))',
    'weLightMerge := assert(merge(light, We_I, in = immediate_self_genesis))',
    'uniteObjectiveAspectInKnowing := assert(allow(objective_aspect) only_if(united_in(knowing)))',
    'higherKnowingUnites := pose(higher_knowing unites({subjective_genetic, objective_aspect}))',
    'subordinateThroughHigher := conclude(mediated(subordinates, by = higherKnowingUnites))',
    // Bridges to next sutras
    'prepare(IV_20_constraint := avoid(regress) ∧ enforce(non_self_luminous_mind))',
    'prepare(IV_21_resolution := articulate(seer_without_objecthood ∧ higher_unifying_principle))',
  ],
})

// ---------- Export Unit ----------
export const YS_IV_19_UNIT: DatasetUnit = {
  id: makeUnitId('ys-iv-19'),
  title: 'YS IV.19 — na tat sva-ābhāsam dṛśyatvāt',
  scope: 'appearance',
  logosMode: 'appearance',
  synthesis: 'mind-not-self-luminous (seer/seen divide)',
  faculty: 'buddhi',
  lens: 'yoga',
  chunks: CHUNKS_YS_IV_19 as any,
  hlos: HLOS_YS_IV_19 as any,
}

// Complete the truncated export
export const YS_IV_19_SYMBOLS = {}
