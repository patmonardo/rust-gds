import { DatasetUnit, makeUnitId } from '@organon/gdsl/registry/canon'

/*
YS IV.16 — na caika-citta-tantraṁ vastu tad-apramāṇakaṁ tadā kim syāt
“The object is not dependent on a single mind; otherwise unvalidated, what would it be then?”

Seed:
- Objectivity (vastu) is not the tantra (dependence) of one citta.
- Worldness/Essential Relation: intersubjective invariants across cittas validate vastu (cf. IV.15).
- Vedānta framing: Puruṣa = essential (secondary) knowing; Brahman = absolute knowing (beyond this sutra’s scope).
- Fichte crosswalk: appearance-as-such; performative criterion.
*/

// ---------- Ontology ----------
export const YS_IV_16_ONTOLOGY = {
  vastu: 'Object within appearance (stabilized by identity across change, IV.14)',
  ekaCittaTantram: 'Dependent on a single citta (denied by the sutra)',
  apramanakam: 'Unvalidated / without corroboration',
  essentialRelation: 'Objectivity via invariants across multiple cittas (worldness)',
  objectivityInvariants: 'Features of appearance that remain stable across citta-bheda',
  worldnessLink: 'Built on IV.15: with vastu held constant, paths diverge by citta; extract invariants',
  // Vedānta stance
  purushaSecondaryKnowing: 'Puruṣa as essential/secondary knowing (mountains and rivers: stabilized appearance)',
  brahmanAbsoluteKnowing: 'Brahman as absolute knowing (non-relational self-showing, beyond essential relation)',
  seePastPurusha: 'Methodical move beyond seer-stance toward Brahmavidyā (signposted, not executed here)',
  // Fichte crosswalk
  fichteAppearanceAsSuch: 'Present appearance in general (principle), not by empirical appeal',
  performativeValidation: 'Say/do coincide where principle holds (deed as proof of objectivity method)',
  // Errors
  errorSolipsism: 'Collapse to one-mind dependence (denied by the sutra)',
  errorConsensusFallacy: 'Mistaking mere agreement for lawlike invariants (lack of genetic grounding)',
}

// Science criterion: requires invariants beyond one citta
Object.assign(YS_IV_16_ONTOLOGY, {
  scienceCriterion: 'Science requires intersubjective invariants; denies eka-citta-tantram',
})

// ---------- Chunks ----------
const CHUNKS_YS_IV_16 = [
  {
    id: 'ys-iv-16-text',
    title: 'IV.16 Text & Baseline',
    summary: 'Not single-mind-dependent; otherwise unvalidated—what would it be then?',
  },
  {
    id: 'ys-iv-16-essential-relation',
    title: 'Essential Relation (Worldness/Objectivity)',
    summary: 'Objectivity via invariants across cittas with vastu held constant (cf. IV.15).',
  },
  {
    id: 'ys-iv-16-crosswalk',
    title: 'Crosswalk (Vedānta/Fichte)',
    summary: 'Puruṣa as secondary knowing; pointer beyond to Brahman; appearance-as-such; performative criterion.',
  },
  {
    id: 'ys-iv-16-errors',
    title: 'Error Modes',
    summary: 'Avoid solipsism and consensus fallacy; require principled invariants.',
  },
  {
    id: 'ys-iv-16-science',
    title: 'Science Criterion',
    summary: 'Science possible only if invariants across cittas validate vastu.',
  },
]

// ---------- HLO Clauses ----------
const HLOS_YS_IV_16 = [
  {
    id: 'ys-iv-16-hlo-text',
    chunkId: 'ys-iv-16-text',
    label: 'Baseline',
    clauses: [
      "tag('sutra','IV.16')",
      'deny(ekaCittaTantram(vastu))',
      'pose(apramanakam if only(one(citta)) validates(vastu)))',
      'query("tadā kim syāt") // what would it be then?',
    ],
  },
  {
    id: 'ys-iv-16-hlo-essential-relation',
    chunkId: 'ys-iv-16-essential-relation',
    label: 'Criterion',
    clauses: [
      'essentialRelation := criterion(objectivityInvariants across(cittas) when hold(IV_15.vastu))',
      'objectivityInvariants := infer(stableFeatures over all(cittas))',
      'worldnessLink := link(IV_15.lawOfAppearance → essentialRelation)',
    ],
  },
  {
    id: 'ys-iv-16-hlo-crosswalk',
    chunkId: 'ys-iv-16-crosswalk',
    label: 'Crosswalk',
    clauses: [
      'purushaSecondaryKnowing := scope(essentialRelation)',
      'brahmanAbsoluteKnowing := note(beyond(essentialRelation))',
      'seePastPurusha := method(signpost(essential → absolute))',
      'fichteAppearanceAsSuch := assert(principleLevel(appearance))',
      'performativeValidation := require(say(x) == do(x))',
    ],
  },
  {
    id: 'ys-iv-16-hlo-errors',
    chunkId: 'ys-iv-16-errors',
    label: 'Errors',
    clauses: [
      'errorSolipsism := flag(assert(ekaCittaTantram(vastu)))',
      'errorConsensusFallacy := flag(assert(objectivity by mereAgreement))',
    ],
  },
  {
    id: 'ys-iv-16-hlo-science',
    chunkId: 'ys-iv-16-science',
    label: 'Science',
    clauses: [
      'scienceCriterion := assert(possible(science) ⇐ invariantsAcross(cittas) ∧ deny(ekaCittaTantram))',
      'link(worldnessLink, scienceCriterion)',
    ],
  },
]

// ---------- Fichte L20–L21: Abstraction → Pure Light → Indivisible Genesis ----------
Object.assign(YS_IV_16_ONTOLOGY, {
  abstractionToPureLight: 'Abstracting from all content yields pure light (absolute, qualitative knowing/seeing)',
  consciousnessOfKnowingFrom: 'Consciousness here is of knowing, and of knowing as a “from”',
  genesisAsFreedom: 'Genesis/freedom can be posited in the act of abstraction from all content',
  indivisibleAppearances: 'Pure light cannot arise without abstraction; content cannot appear without pure light; they are indivisible and permeate one another',
  lightPermeatesGenesis: 'Pure light appears as permeating genesis, producing itself (appearance-as-genesis)',
  lightPositsFromAndSelf: 'Light posits the “from,” and posits itself as a “from”',
  selfProducingKnowing: 'Absolute, self-producing knowing that does not occur without genesis',
  energyReflectionNewConsciousness: 'New consciousness co-arises only with the energy of reflection; they open together',
  splitEnergyVsReasonAbstracted: 'Genesis partly in self (energy) and partly in reason itself; this split has no standing and should be abstracted out',
  overProvedAnticipation: 'More is proved than should be proved; anticipates further research',
})

CHUNKS_YS_IV_16.push(
  {
    id: 'ys-iv-16-fichte-abstraction',
    title: 'Abstraction → Pure Light',
    summary: 'Abstract from all content: pure light; consciousness of knowing as “from”; posit genesis/freedom in abstraction.',
  },
  {
    id: 'ys-iv-16-fichte-genesis',
    title: 'Indivisible Genesis',
    summary: 'Pure light and genesis co-appear and permeate; light posits the “from” and itself as “from”; self-producing knowing.',
  },
  {
    id: 'ys-iv-16-fichte-examples',
    title: 'Example and Split',
    summary: 'Energy of reflection with new consciousness co-arising; abstract the self/reason split.',
  },
)

HLOS_YS_IV_16.push(
  {
    id: 'ys-iv-16-hlo-fichte-abstraction',
    chunkId: 'ys-iv-16-fichte-abstraction',
    label: 'Abstraction',
    clauses: [
      'abstractionToPureLight := assert(abstraction(from = allContent) ⇒ pureLight)',
      'consciousnessOfKnowingFrom := scope(consciousness(knowing_as("from")))',
      'genesisAsFreedom := locate(genesis, in = actOfAbstraction)',
      'performativeValidation := link(performance, warrant(presupposition("from")))',
    ],
  },
  {
    id: 'ys-iv-16-hlo-fichte-genesis',
    chunkId: 'ys-iv-16-fichte-genesis',
    label: 'Genesis',
    clauses: [
      'indivisibleAppearances := assert(indivisible(pureLight, abstraction) ∧ permeate(pureLight, genesis))',
      'lightPermeatesGenesis := assert(appears(pureLight, as = permeating(genesis)) ∧ produces(itself))',
      'lightPositsFromAndSelf := assert(posit(light, "from") ∧ posit(light, as = "from"))',
      'selfProducingKnowing := assert(abs(know) where notOccurWithout(genesis))',
      'overProvedAnticipation := note(anticipate(futureResearch))',
    ],
  },
  {
    id: 'ys-iv-16-hlo-fichte-examples',
    chunkId: 'ys-iv-16-fichte-examples',
    label: 'Illustration',
    clauses: [
      'energyReflectionNewConsciousness := model(coArise(energyOfReflection, newConsciousness))',
      'splitEnergyVsReasonAbstracted := assert(split(genesis, {selfEnergy, reasonEssence}) ▷ abstractAway)',
      'essentialRelation := reinforce(via = objectivityInvariants)',
    ],
  },
)

// ---------- Export Unit ----------
export const YS_IV_16_UNIT: DatasetUnit = {
  id: makeUnitId('ys-iv-16'),
  title: 'YS IV.16 — na caika-citta-tantraṁ vastu tad-apramāṇakaṁ tadā kim syāt',
  scope: 'appearance',
  logosMode: 'appearance',
  synthesis: 'essential-relation-objectivity',
  faculty: 'buddhi',
  lens: 'yoga',
  chunks: CHUNKS_YS_IV_16 as any,
  hlos: HLOS_YS_IV_16 as any,
}

export const YS_IV_16_SYMBOLS = Object.keys(YS_IV_16_ONTOLOGY);
