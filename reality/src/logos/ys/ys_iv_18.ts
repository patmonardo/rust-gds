import { DatasetUnit, makeUnitId } from '@organon/gdsl/registry/canon';

/*
YS IV.18 — sadā jñātā citta-vṛttayas tat-prabhoḥ puruṣasyāpariṇāmitvāt

“The mind’s modifications are always known, because their lord (Purusha) is unchanging.”

Reading
- Purusha = unchanging witness (kūṭastha); “lord” functionally as illuminator/witness.
- Always-known: for Purusha (witness), not necessarily for a given citta (cf. IV.17 upārāga).
- Culmination of Essential Relation: stance-constancy that underwrites world/thing objectivity.
- Pointer beyond: do not collapse Brahman (absolute knowing) into Purusha (essential/secondary knowing).

Fichte L21 crosswalk (second side of proof)
- Unchanging qualitative oneness of light pervades the “from”; witness invariance amid genesis.
- Performative criterion remains: say/do coincide.
*/

// ---------- Ontology ----------
export const YS_IV_18_ONTOLOGY = {
  purusha: 'Unchanging witness (kūṭastha), non-agentive seer',
  aparinamivata:
    'Apariṇāmitvāt — because of unchangeability (cause of always-known status)',
  citta: 'Mind-stream/locus of appearance',
  vritti: 'Modification/fluctuation of citta',
  sadaJnata: 'Always-known (for the witness stance)',
  tatPrabhoh:
    '“Because of its lord” (prabhu = lord/master); functionally: illuminator/witness of vṛttis',
  prabhuVsPrakasha: 'We treat prabhu as witness-illumination (not domination)',
  alwaysKnownForPurusha:
    'All citta-vṛttis are manifest to Purusha without alteration of Purusha',
  knownnessForCitta:
    'Knownness for a given citta still depends on upārāga (IV.17)',
  stanceConstancy:
    'Purusha provides stance-constancy for Essential Relation and worldness',
  essentialRelationApex:
    'Culmination of Essential Relation: witness invariance grounding objectivity',
  seePastPurusha:
    'Pointer: Brahmavidyā lies beyond witness-duality (do not conflate)',
  // Crosswalks
  linkIV15Worldness:
    'Builds on worldness (invariants across cittas with vastu held constant)',
  linkIV16NotSingleMind:
    'Not dependent on one citta; witness is not a private mind',
  linkIV17KnownnessLaw:
    'Reconciles “always-known” (witness) with upārāga-conditioned knownness (citta)',
  fichteWitnessLight:
    'Unchanging qualitative oneness of light pervades all “from” without changing',
  performativeValidation:
    'Say/do coincide: enact the witness stance to see the claim',
  // Guards
  errorPurushaAsMind: 'Confusing Purusha with citta (agentive mind)',
  errorCollapseToBrahman:
    'Collapsing Purusha (essential) into Brahman (absolute knowing)',
};

// ---------- Chunks ----------
const CHUNKS_YS_IV_18 = [
  {
    id: 'ys-iv-18-text',
    title: 'IV.18 Text & Baseline',
    summary:
      'Mind’s modifications are always known, because Purusha (their lord) is unchanging.',
  },
  {
    id: 'ys-iv-18-semantics',
    title: 'Semantics: “Always-known” and Prabhu',
    summary:
      'Always-known for the witness; prabhu read functionally as illuminator/witness.',
  },
  {
    id: 'ys-iv-18-bridge',
    title: 'Bridges: IV.15–IV.17 → IV.18',
    summary: 'Reconcile worldness and knownness-law with witness invariance.',
  },
  {
    id: 'ys-iv-18-crosswalk',
    title: 'Crosswalk (Fichte L21)',
    summary:
      'Unchanging light pervades the “from”; witness invariance amid genesis; performative proof.',
  },
  {
    id: 'ys-iv-18-guards',
    title: 'Guards',
    summary: 'Avoid Purusha≡mind and Purusha≡Brahman conflations.',
  },
];

// ---------- HLO Clauses ----------
const HLOS_YS_IV_18 = [
  {
    id: 'ys-iv-18-hlo-text',
    chunkId: 'ys-iv-18-text',
    label: 'Baseline',
    clauses: [
      "tag('sutra','IV.18')",
      'assert(sadaJnata(citta.vritti))',
      'assert(because(tatPrabhoh(purusha), purusha.aparinamivata))',
    ],
  },
  {
    id: 'ys-iv-18-hlo-semantics',
    chunkId: 'ys-iv-18-semantics',
    label: 'Semantics',
    clauses: [
      'alwaysKnownForPurusha := scope(witness(purusha) sees(all(citta.vritti)))',
      'prabhuVsPrakasha := gloss(prabhu as witness-illumination, not domination)',
      'errorPurushaAsMind := flag(confuse(purusha, citta))',
    ],
  },
  {
    id: 'ys-iv-18-hlo-bridge',
    chunkId: 'ys-iv-18-bridge',
    label: 'Bridges',
    clauses: [
      'linkIV15Worldness := link(IV_15.lawOfAppearance → stanceConstancy)',
      'linkIV16NotSingleMind := link(IV_16.essentialRelation → nonPrivateWitness)',
      'linkIV17KnownnessLaw := reconcile(alwaysKnownForPurusha, with = IV_17.knownnessAsFunction)',
      'knownnessForCitta := assert(depends(on = uparaga(citta, vastu)))',
    ],
  },
  {
    id: 'ys-iv-18-hlo-crosswalk',
    chunkId: 'ys-iv-18-crosswalk',
    label: 'Fichte',
    clauses: [
      'fichteWitnessLight := assert(unchangingQualitativeOneness(light) ∧ permeates(all("from")))',
      'assert(witnessInvariance amid genesis(appearance))',
      'performativeValidation := require(say(x) == do(x) via adoption(witness-stance))',
    ],
  },
  {
    id: 'ys-iv-18-hlo-guards',
    chunkId: 'ys-iv-18-guards',
    label: 'Guards',
    clauses: [
      'errorCollapseToBrahman := flag(collapse(purusha → brahman))',
      'seePastPurusha := note(pointer(brahmavidya beyond witness-duality))',
    ],
  },
];

// Strengthen non-agentive seer semantics (Sāṃkhya explicit)
Object.assign(YS_IV_18_ONTOLOGY, {
  akartatva: 'Purusha’s non-agency (akartatva): seer-only, not doer',
});

CHUNKS_YS_IV_18.push({
  id: 'ys-iv-18-non-agent',
  title: 'Non-agentive Seer',
  summary:
    'Purusha as witness-only (akartatva); phenomenology of seeing vs doing.',
});

HLOS_YS_IV_18.push({
  id: 'ys-iv-18-hlo-non-agent',
  chunkId: 'ys-iv-18-non-agent',
  label: 'Akartatva',
  clauses: [
    'assert(purusha == witness ∧ ¬agent)',
    'akartatva := gloss(nonAgency(purusha))',
  ],
});

// ---------- Fichte L21: Presupposition-in-We; higher vs lower knowing; boundary ----------
Object.assign(YS_IV_18_ONTOLOGY, {
  presuppositionInWe:
    'Light presupposes itself as a “from” in us insofar as We merge into light',
  weIdenticalWithLight:
    'The “We” that disappears into light = science of knowing (higher knowing)',
  earlierWeFreePosit:
    'The earlier We freely posits premises (lower, ordinary knowing)',
  higherVsLowerKnowing:
    'Distinction: higher scientific/genetic vs lower ordinary/empirical knowing',
  hypotheticalShould:
    'Presuppositions carry a hypothetical “should”: “if light is to be…”',
  realizedQualitativeOneness:
    'Absolute oneness realized (qualitatively) beyond the hypothetical frame',
  categoricalBareOneness:
    'Pure, bare oneness remains categorical (boundary delineation)',
  ascentDescentMethod:
    'Ascent abstracts to clear vision; descent restores subordinate terms in full clarity',
});

CHUNKS_YS_IV_18.push(
  {
    id: 'ys-iv-18-fichte21-we',
    title: 'Fichte L21 — Presupposition in the “We”',
    summary:
      'We merge into light; light presupposes itself as “from” in us (science of knowing).',
  },
  {
    id: 'ys-iv-18-fichte21-levels',
    title: 'Higher vs Lower Knowing',
    summary:
      'Higher (scientific/genetic) vs lower (ordinary/empirical) knowing; Purusha aligns with the essential/secondary.',
  },
  {
    id: 'ys-iv-18-fichte21-boundary',
    title: '“Should” → Categorical Oneness',
    summary:
      'From hypothetical presupposition to realized qualitative oneness; categorical bare oneness as boundary.',
  },
  {
    id: 'ys-iv-18-fichte21-method',
    title: 'Ascent/Descent Method',
    summary:
      'Abstract now; reconstruct detail on the descent (subordinate terms recur).',
  },
);

HLOS_YS_IV_18.push(
  {
    id: 'ys-iv-18-hlo-fichte21-we',
    chunkId: 'ys-iv-18-fichte21-we',
    label: 'We/Light',
    clauses: [
      'presuppositionInWe := assert(light presupposes_itself_as("from") in(us where weIdenticalWithLight))',
      'weIdenticalWithLight := define(We == light at science_of_knowing)',
      'earlierWeFreePosit := contrast(We_free_positor of premises)',
    ],
  },
  {
    id: 'ys-iv-18-hlo-fichte21-levels',
    chunkId: 'ys-iv-18-fichte21-levels',
    label: 'Levels',
    clauses: [
      'higherVsLowerKnowing := distinguish(higher = genetic(scientia), lower = empirical)',
      'link(purusha, lower as essential/secondaryKnowing)',
      'seePastPurusha := note(pointer(brahmavidya beyond essential))',
    ],
  },
  {
    id: 'ys-iv-18-hlo-fichte21-boundary',
    chunkId: 'ys-iv-18-fichte21-boundary',
    label: 'Boundary',
    clauses: [
      'hypotheticalShould := note(presuppose with "should")',
      'realizedQualitativeOneness := assert(realize(oneness(qualitative)))',
      'categoricalBareOneness := mark(boundary(pure_bare_oneness as categorical))',
    ],
  },
  {
    id: 'ys-iv-18-hlo-fichte21-method',
    chunkId: 'ys-iv-18-fichte21-method',
    label: 'Method',
    clauses: [
      'ascentDescentMethod := remind(abstract_now ∧ reconstruct_on_descent)',
      'performativeValidation := require(say(x) == do(x))',
    ],
  },
);

// ---------- Fichte L21 (Part 3): Immediate “from”; drop ascent constructs ----------
Object.assign(YS_IV_18_ONTOLOGY, {
  genesisEqualsFromQualitative:
    'Genesis = the “from” in qualitative oneness (demonstrated previously)',
  weImmediateFrom:
    'We (knowing/light) are this “from” immediately, in what we pursue and live',
  dropAscentMeans:
    'Let go of the posited/presupposed “from” and its derivations as mere means of ascent; recover on descent',
  liveFromDissolution:
    'Pursuing and living follow directly from dissolution into genesis',
});

CHUNKS_YS_IV_18.push({
  id: 'ys-iv-18-fichte21-p3',
  title: 'Fichte L21 — Part 3: Immediate “From”',
  summary:
    'We are the “from” immediately; ascent means are dropped until the descent; living follows dissolution into genesis.',
});

HLOS_YS_IV_18.push({
  id: 'ys-iv-18-hlo-fichte21-p3',
  chunkId: 'ys-iv-18-fichte21-p3',
  label: 'Part 3',
  clauses: [
    'genesisEqualsFromQualitative := assert(genesis ≡ "from"(qualitativeOneness))',
    'weImmediateFrom := assert(we == knowing == light == immediate("from"))',
    'dropAscentMeans := advise(let_go({posited("from"), derivedAnalyses}) until(descent))',
    'liveFromDissolution := assert((pursue ∧ live) follow_from dissolve_into(genesis))',
  ],
});

// ---------- Fichte L21 (Part 4): Absolute genesis; two knowings as aspects (not two things) ----------
Object.assign(YS_IV_18_ONTOLOGY, {
  shouldMakesKnowingGeneticToItself:
    'By the hypothetical “should,” the We/knowing is absolutely genetic relative to itself',
  absoluteGenesisSelfEnclosed:
    'Absolute genesis/light is completely self-enclosed; never goes outside itself',
  twoKnowingsAppearImpermanent:
    'Two origin points/knowings appear impermanently: our hypothetical We vs the one when its principle is fulfilled',
  aspectualDisjunctionWithinOne:
    'Disjunction is within one (different aspects), not between two fundamentally distinct terms',
  fulfilledPrincipleKnowing:
    'Second knowing: obtains when its principle is fulfilled (not by our hypothetical positing)',
});

CHUNKS_YS_IV_18.push({
  id: 'ys-iv-18-fichte21-p4',
  title: 'Fichte L21 — Part 4: Absolute Genesis and Aspects',
  summary:
    '“Should” makes knowing genetic to itself; absolute self-enclosure; two knowings as aspects within the one.',
});

HLOS_YS_IV_18.push({
  id: 'ys-iv-18-hlo-fichte21-p4',
  chunkId: 'ys-iv-18-fichte21-p4',
  label: 'Part 4',
  clauses: [
    'shouldMakesKnowingGeneticToItself := assert("should" ⇒ absolute(genesis(of = knowing, of_itself)))',
    'absoluteGenesisSelfEnclosed := assert(self_enclosed(genesis) ∧ ¬go_outside_itself(genesis))',
    'twoKnowingsAppearImpermanent := pose(two(originPoints(knowing)) appear_impermanently)',
    'define(hypotheticalWeKnowing := say("If knowing/we should be … then …"))',
    'fulfilledPrincipleKnowing := define(otherKnowing when(principle_fulfilled))',
    'aspectualDisjunctionWithinOne := assert(disjunctionWithin(one, by = aspects) ∧ ¬twoThings)',
  ],
});

// ---------- Qualitative Oneness (LogoLogia / Pure Reason) ----------
Object.assign(YS_IV_18_ONTOLOGY, {
  qualitativeOneness:
    'Unchanging qualitative oneness of light; immediate identity, not mediated inference',
  qualitativeSyllogismContrast:
    'Contrast: not a “qualitative syllogism”; this is unity of showing, not syllogistic mediation',
  logoLogia: 'LogoLogia: cognition from Pure Reason (genetic, principle-level presentation)',
  pureReasonCognition:
    'Cognition from Pure Reason (Fichte/Hegel at principle level); here focused on qualitative oneness',
  ordinaryImpureReason:
    'Ordinary/abstract (impure) reason: reconstructive, secondary knowing (appearance-level)',
  hegelPureLogicCrosswalk:
    'Hegel’s Pure Science of Logic as cognition from pure reason; our emphasis: oneness over syllogistic form',
  errorConflateSyllogismOneness:
    'Error: conflating qualitative oneness with qualitative syllogism (category mistake)',
})

CHUNKS_YS_IV_18.push({
  id: 'ys-iv-18-qual-oneness',
  title: 'Qualitative Oneness (LogoLogia)',
  summary:
    'Pure-Reason cognition: qualitative oneness of light pervading the “from”; not a qualitative syllogism.',
})

HLOS_YS_IV_18.push({
  id: 'ys-iv-18-hlo-qual-oneness',
  chunkId: 'ys-iv-18-qual-oneness',
  label: 'LogoLogia',
  clauses: [
    'qualitativeOneness := assert(oneness(qualitative(light)) ∧ permeate(light, all("from")))',
    'contrast(qualitativeOneness, qualitativeSyllogismContrast)',
    'logoLogia := assert(pureReasonCognition)',
    'link(qualitativeOneness, to = {fichteWitnessLight, essentialRelationApex, performativeValidation})',
    'hegelPureLogicCrosswalk := note(mapping(pureReason ↔ qualitativeOneness focus))',
    'errorConflateSyllogismOneness := flag(conflate(qualitativeOneness, syllogisticForm))',
  ],
})

// ---------- Export Unit ----------
export const YS_IV_18_UNIT: DatasetUnit = {
  id: makeUnitId('ys-iv-18'),
  title:
    'YS IV.18 — sadā jñātā citta-vṛttayas tat-prabhoḥ puruṣasyāpariṇāmitvāt',
  scope: 'appearance',
  logosMode: 'appearance',
  synthesis: 'witness-invariance (Essential Relation apex)',
  faculty: 'buddhi',
  lens: 'yoga',
  chunks: CHUNKS_YS_IV_18 as any,
  hlos: HLOS_YS_IV_18 as any,
};

// Complete the truncated export
export const YS_IV_18_SYMBOLS = Object.keys(YS_IV_18_ONTOLOGY);
