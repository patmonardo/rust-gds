import { DatasetUnit, makeUnitId } from '@organon/gdsl/registry/canon'

/*
C. ACTUALITY

YS IV.22 — citer apratisaṅkramāyās tad-ākārāpattau sva-buddhi-saṁvedanam

“When, due to the non-transference of consciousness, it (buddhi) takes on the form of that (object), there is self-cognition of its own buddhi.”

Reading (Actuality frame)
- Non-transference: consciousness (citi/puruṣa) does not “go over” into objects; witness remains seer-only.
- Assumption-of-form: buddhi/citta takes the object’s form (ākāra-āpatti) under sattva-reflection.
- Self-cognition: in that very form-taking, the buddhi becomes aware of itself as illuminated (not self-luminous per se).
- Actuality: unity-in-act of essence (witness-invariance) and appearance (form-taking), without collapsing seer into seen.
- Locks prior constraints (IV.19–IV.21) while advancing into Absolute/Modality (IV.23–IV.24).
*/

// ---------- Ontology (advanced key terms) ----------
export const YS_IV_22_ONTOLOGY = {
  citi: 'Pure consciousness/light (witness aspect; puruṣa-light)',
  apratisankramaya:
    'Non-transference of consciousness (witness does not migrate into objects or modifications)',
  tadAkaraApattau:
    'Assumption of that form (buddhi/citta takes the object’s ākāra under sattva-reflection)',
  svaBuddhiSamvedanam:
    'Self-cognition of one’s own buddhi (reflexive awareness in the very act of form-taking)',
  buddhiAsMirror:
    'Buddhi as sattva-mirror: reflects light and object-form without becoming the seer',
  actualityUnity:
    'Actuality = unity-in-act of essence (witness invariance) and appearance (form-taking) without identity of roles',
  seerWithoutObjecthood:
    'Witness (puruṣa) is seer-only and never an object among objects (carried from IV.21)',
  notSelfLuminousCitta:
    'Mind is not self-luminous (IV.19); its self-cognition is conditioned by illumination and ākāra-āpatti',
  noDoubleDetermination:
    'No single act simultaneously determines seer and seen (IV.20); non-transference preserves this',
  noIntermentalTransparency:
    'No mind-sees-mind layering (IV.21); avoids atiprasaṅga and smṛti-saṅkara',
  sattvaConstraint:
    'Only śuddha-sattva transparency mediates knownness; tamasic/māyic displays are not the witness’s object',
  // Crosswalks (meta)
  fichteOneSidedOrder:
    'One-sided order: principle → principled thing; witness does not reciprocally become object (hiatum preserved)',
  hegelActuality:
    'Actuality (Wirklichkeit): essence-in-appearance as an accomplished act, not collapse of terms',
  // Guards
  errorPurushaTransforms:
    'Error: saying puruṣa “assumes forms” (violates seer-only non-transference)',
  errorSelfLuminosityUpgrade:
    'Error: upgrading buddhi to self-luminous subject; its self-cognition is conditioned and act-bound',
  errorMindOnMind:
    'Error: implying citta sees citta (inter-mental transparency) contrary to IV.21',
}

// ---------- Chunks ----------
const CHUNKS_YS_IV_22 = [
  {
    id: 'ys-iv-22-text',
    title: 'IV.22 Text & Baseline',
    summary:
      'Non-transference of consciousness; upon assumption of object-form, buddhi cognizes itself.',
  },
  {
    id: 'ys-iv-22-semantics',
    title: 'Semantics: apratisankramā / ākāra-āpatti / sva-buddhi-saṁvedana',
    summary:
      'Witness does not transfer; buddhi takes form; reflexive awareness arises in that act.',
  },
  {
    id: 'ys-iv-22-actuality',
    title: 'Actuality: Essence-in-Act',
    summary:
      'Essence (seer-only) and appearance (form-taking) unified in act without role-collapse.',
  },
  {
    id: 'ys-iv-22-crosswalk',
    title: 'Crosswalk (Fichte/Hegel)',
    summary:
      'One-sided order and hiatus (Fichte); actuality as accomplished unity (Hegel).',
  },
  {
    id: 'ys-iv-22-guards',
    title: 'Guards',
    summary:
      'Deny puruṣa-transformations; deny self-luminous mind; deny mind-on-mind seeing.',
  },
  {
    id: 'ys-iv-22-bridges',
    title: 'Bridges → IV.23–IV.24',
    summary:
      'Prepare modality and absolute relation constraints from the actuality stance.',
  },
]

// ---------- HLO Clauses ----------
const HLOS_YS_IV_22 = [
  {
    id: 'ys-iv-22-hlo-text',
    chunkId: 'ys-iv-22-text',
    label: 'Baseline',
    clauses: [
      "tag('sutra','IV.22')",
      'assert(apratisankramaya(citi))',
      'assert(tadAkaraApattau(buddhi))',
      'conclude(svaBuddhiSamvedanam in_act_of(tadAkaraApattau))',
    ],
  },
  {
    id: 'ys-iv-22-hlo-semantics',
    chunkId: 'ys-iv-22-semantics',
    label: 'Semantics',
    clauses: [
      'define(apratisankramaya := nonTransferenceOfConsciousness)',
      'define(tadAkaraApattau := assumptionOfObjectForm)',
      'define(svaBuddhiSamvedanam := reflexiveAwareness(buddhi,self))',
      'buddhiAsMirror := gloss(sattva_reflection(light ∧ form))',
      'notSelfLuminousCitta := import(IV_19.notSelfLuminousCitta)',
      'noDoubleDetermination := import(IV_20.noDoubleDetermination)',
      'noIntermentalTransparency := import(IV_21.worldnessMediation)',
    ],
  },
  {
    id: 'ys-iv-22-hlo-actuality',
    chunkId: 'ys-iv-22-actuality',
    label: 'Actuality',
    clauses: [
      'actualityUnity := assert(unify_in_act(essence:seerWithoutObjecthood, appearance:tadAkaraApattau) ∧ preserve(role_distinctions))',
      'sattvaConstraint := constrain(knownness, by = sattva_transparency)',
    ],
  },
  {
    id: 'ys-iv-22-hlo-crosswalk',
    chunkId: 'ys-iv-22-crosswalk',
    label: 'Crosswalk',
    clauses: [
      'fichteOneSidedOrder := note(order(principle → principled_thing) ∧ separation_by(hiatum))',
      'hegelActuality := map(actuality == essence_in_appearance as accomplished act)',
    ],
  },
  {
    id: 'ys-iv-22-hlo-guards',
    chunkId: 'ys-iv-22-guards',
    label: 'Guards',
    clauses: [
      'errorPurushaTransforms := flag(assert(purusha_assumes_forms))',
      'errorSelfLuminosityUpgrade := flag(assert(buddhi_is_self_luminous))',
      'errorMindOnMind := flag(assert(citta_sees_citta))',
    ],
  },
  {
    id: 'ys-iv-22-hlo-bridges',
    chunkId: 'ys-iv-22-bridges',
    label: 'Bridges',
    clauses: [
      'prepare(IV_23_modality := articulate(possibility/necessity under actualityUnity))',
      'prepare(IV_24_absolute_relation := stabilize(relation at witness-locked stance))',
    ],
  },
]

// ---------- Fichte Addendum (two-part) ----------
Object.assign(YS_IV_22_ONTOLOGY, {
  // Review/summary
  heightOfSpeculation: 'Our speculations have reached an unprecedented height (preparatory, not final)',
  preparationForTaskResolution: 'Work so far is preparation toward resolving speculation’s task',
  protectiveTruthsButDisconnected:
    'One may hold significant but disconnected truths; lacking the unified system',
  capacitySystemOfTruths:
    'Aim: capacity to construct the system of truths “out of a single piece” (inner synthesis)',
  // Absolute statement
  absoluteAsLightOrBeing: 'The absolute, whether named “being” or “light,” is already familiar',
  derivingAppearanceNotThing:
    'Task is to derive appearance from the absolute, not to grasp the absolute “thing itself” anew',
  undiscoveredInAbsolute:
    'There remains something in the absolute itself through which it coheres with its appearance',
  coherenceWithAppearance:
    'Coherence-link: inner ground in the absolute that accounts for appearance',
  absoluteInItselfCiter:
    '“citer” flags the Absolute-in-itself (Brahman as truth of citi), now read under Actuality',
  hegelPresentationOfAbsolute:
    'Presentation of the Absolute: actuality where essence shows itself as the truth of being',
  necessityModalityPrelude:
    'Prelude to modality/necessity: Absolute reflection (dharma‑megha trajectory)',
  dharmaMeghaAsAbsoluteReflection:
    'Dharma‑megha as modality of the Absolute (reflection on/within the Absolute)',
})

// Extend chunks with Fichte review + absolute statement
CHUNKS_YS_IV_22.push(
  {
    id: 'ys-iv-22-fichte-review',
    title: 'Fichte Review — Preparation and System-Capacity',
    summary:
      'Height of speculation as preparation; from disconnected truths to a system “out of one piece.”',
  },
  {
    id: 'ys-iv-22-absolute-statement',
    title: 'Absolute-in-Itself and Coherence with Appearance',
    summary:
      'Absolute (being/light) is known; derive appearance from it; articulate the coherence-link.',
  },
)

// HLOs for the two new chunks
HLOS_YS_IV_22.push(
  {
    id: 'ys-iv-22-hlo-fichte-review',
    chunkId: 'ys-iv-22-fichte-review',
    label: 'Review',
    clauses: [
      'heightOfSpeculation := note(preparatory_status)',
      'preparationForTaskResolution := assert(prep → task_resolution_pending)',
      'protectiveTruthsButDisconnected := warn(hold(truths) ∧ disconnected)',
      'capacitySystemOfTruths := goal(construct(system_of_truths, as = single_piece))',
    ],
  },
  {
    id: 'ys-iv-22-hlo-absolute-statement',
    chunkId: 'ys-iv-22-absolute-statement',
    label: 'Absolute',
    clauses: [
      'absoluteAsLightOrBeing := recall(familiarity_established)',
      'derivingAppearanceNotThing := assert(task == derive(appearance, from = absolute))',
      'undiscoveredInAbsolute := pose(there_is(something_in_absolute) grounding(coherenceWithAppearance))',
      'coherenceWithAppearance := aim(exhibit(inner_link absolute ↔ appearance))',
      'absoluteInItselfCiter := tag(citer := absolute_in_itself under actuality)',
      'hegelPresentationOfAbsolute := map(actuality == essence_shown_as_truth_of_being)',
      'necessityModalityPrelude := prepare(modality/necessity via dharmaMeghaAsAbsoluteReflection)',
    ],
  },
)

// ---------- Fichte: Disjunction within Oneness (“from” of a “from”) ----------
Object.assign(YS_IV_22_ONTOLOGY, {
  principleOfAppearanceAsDisjunction:
    'Principle of appearance = disjunction within undivided oneness (and within appearance)',
  absoluteDisjunctionSelfIntersecting:
    'Not a simple split: self‑intersecting disjunction of presupposed disjunctions',
  disjunctiveFoundationsTwo:
    'Two different disjunctive foundations intersect (aspectual, within one)',
  fromOfFrom:
    'No simple “from”; a “from” in a “from” (meta‑genesis within genesis)',
  subtleDistinctionDiscipline:
    'Avoid confusion: endlessly similar aspects; requires finest distinguishing',
  methodMiraculousWarning:
    'Method may appear “miraculous” before exposition; account comes after',
  coherenceAimActuality:
    'Use the self‑intersecting disjunction to show coherence of Absolute with appearance (Actuality)',
})

CHUNKS_YS_IV_22.push({
  id: 'ys-iv-22-fichte-disjunction',
  title: 'Fichte — Disjunction within Oneness',
  summary:
    'Appearance arises from a self‑intersecting disjunction in undivided oneness: a “from” of a “from.”',
})

HLOS_YS_IV_22.push({
  id: 'ys-iv-22-hlo-fichte-disjunction',
  chunkId: 'ys-iv-22-fichte-disjunction',
  label: 'Disjunction/Method',
  clauses: [
    'principleOfAppearanceAsDisjunction := assert(disjunction(within = undivided_oneness ∧ within = appearance))',
    'absoluteDisjunctionSelfIntersecting := assert(self_intersecting(disjunction(presupposed_division)))',
    'disjunctiveFoundationsTwo := pose(two(disjunctive_foundations) ∧ intersect within(one))',
    'fromOfFrom := assert(metaGenesis := "from"("from(x)") within qualitative_oneness)',
    'subtleDistinctionDiscipline := require(fine_grained_distinguishing)',
    'methodMiraculousWarning := note(pre_account_of_method may_appear(miraculous))',
    'coherenceAimActuality := aim(exhibit(coherence(absolute, appearance)) @ actualityUnity)',
  ],
})

// ---------- Fichte — Reflection on the Absolute (Part 1: Review/Intro) ----------
Object.assign(YS_IV_22_ONTOLOGY, {
  principleProvidingOccurrence:
    'Knowing as “principle-providing” occurrence (principle posited within knowing)',
  absoluteSelfGenesisWithPrinciple:
    'Absolute self-genesis posited and given a principle within knowing',
  positiveNegationEnduringBeing:
    'Absolute, positive negation of genesis = completed and enduring being (knowing’s being)',
  pureImmanenceLight:
    'Scope: light’s pure immanence (no appeal to external perceiving/knowing/intuition)',
  presumedExternalBeingIsKnowingBeing:
    'The presumed external being is resolved as knowing’s own completed/enduring being',
  insightMediatesOneTerm:
    'One of the two terms is seen as mediated “through the insight into the connection”',
  weUnconditionallyImmediate:
    'We (this achieved insight) are the unconditionally immediate term',
  holdItIsKnowingBeing:
    'Must hold fast that the inner being/persevering is knowing’s being (avoid relapse)',
  chainOfReasoningMustBePresent:
    'Keep the entire chain of reasoning present to progress further',
})

CHUNKS_YS_IV_22.push({
  id: 'ys-iv-22-fichte-abs-review',
  title: 'Fichte — Reflection on the Absolute (Review/Intro)',
  summary:
    'Principle-providing knowing; positive negation = enduring being; pure immanence; mediation vs immediacy; keep the chain intact.',
})

HLOS_YS_IV_22.push({
  id: 'ys-iv-22-hlo-fichte-abs-review',
  chunkId: 'ys-iv-22-fichte-abs-review',
  label: 'Review/Intro',
  clauses: [
    'principleProvidingOccurrence := assert(knowing provides(principle))',
    'absoluteSelfGenesisWithPrinciple := assert(self_genesis with(principle, within = knowing))',
    'positiveNegationEnduringBeing := assert(posNeg(genesis) == enduring_being(of = knowing))',
    'pureImmanenceLight := scope(light_immanence_only)',
    'presumedExternalBeingIsKnowingBeing := conclude(resolve(presumed_external_being → knowing_own_enduring_being))',
    'insightMediatesOneTerm := note(mediated(term) via insight(into_connection))',
    'weUnconditionallyImmediate := assert(we == unconditional_immediacy by = achieved_insight)',
    'holdItIsKnowingBeing := require(remember(inner_being == knowing_being))',
    'chainOfReasoningMustBePresent := require(maintain(entire_chain_of_reasoning))',
    // Fit to Actuality frame
    'link(actualityUnity, to = {positiveNegationEnduringBeing})',
    'coherenceWithAppearance := aim(exhibit(inner_link absolute ↔ appearance))',
  ],
})

// ---------- Cit · Citi · Citer (Madhya) → Sanyoga: Citta ----------
Object.assign(YS_IV_22_ONTOLOGY, {
  cit: 'Cit — pure being/awareness (non-relational light)',
  citi: 'Citi — knowing-as-act (relational showing/presentation)',
  citer:
    'Citer — Madhya mediator: operative unity of Cit∧Citi named in this sutra; bearer of non-transference',
  sanyogaCitta:
    'Citta = saṁyoga (conjunction) of Cit and Citi; reflective field under sattva',
  deriveOrdinaryKnowing:
    'Knowing’s derived being yields ordinary (non-transcendental) knowing (Fichte)',
  principleOfDerivedKnowing:
    'Principle of the recently-derived knowing (to be reflected upon for method)',
  insightIntoConnection:
    'Insight into the connection elevates to transcendental knowing (science of knowing)',
  higherRegionWork:
    'Work in the higher region to find the principle of appearance and disjunction',
  applicationToOrdinaryKnowing:
    'Apply discovered principle back to existing, ordinary actual knowing',
  // Guards (triad hygiene)
  errorCollapseCitCiti: 'Error: collapsing Cit (being) and Citi (act) into an undifferentiated blob',
  errorCiterAsSubstance:
    'Error: treating Citer as a third substance; it is the Madhya operation Cit∧Citi under non-transference',
})

CHUNKS_YS_IV_22.push(
  {
    id: 'ys-iv-22-cit-citi-citer',
    title: 'Cit · Citi · Citer — Saṁyoga → Citta',
    summary:
      'Discriminate Cit and Citi; Citer as Madhya operation; their saṁyoga grounds Citta under non‑transference.',
  },
  {
    id: 'ys-iv-22-fichte-abs-derivation',
    title: 'Fichte — Deriving Ordinary Knowing',
    summary:
      'Knowing’s derived being → ordinary knowing; reflect its principle to ascend to transcendental knowing.',
  },
)

HLOS_YS_IV_22.push(
  {
    id: 'ys-iv-22-hlo-cit-citi-citer',
    chunkId: 'ys-iv-22-cit-citi-citer',
    label: 'Triad/Madhya',
    clauses: [
      'define(cit := pure_awareness)',
      'define(citi := knowing_as_act)',
      'citer := assert(Madhya(op(unite(cit, citi)) ∧ apratisankramaya))',
      'sanyogaCitta := assert(conjoin(cit, citi) ⇒ citta under(sattvaConstraint))',
      'link(tadAkaraApattau, to = {buddhiAsMirror, sanyogaCitta})',
      'errorCollapseCitCiti := flag(conflate(cit, citi))',
      'errorCiterAsSubstance := flag(reify(citer_as_third_substance))',
      // Bridges
      'prepare(IV_23_sarvartham := from(sanyogaCitta ⇒ omni_instrumentality))',
    ],
  },
  {
    id: 'ys-iv-22-hlo-fichte-abs-derivation',
    chunkId: 'ys-iv-22-fichte-abs-derivation',
    label: 'Derivation/Method',
    clauses: [
      'deriveOrdinaryKnowing := conclude(knowing_derived_being ⇒ ordinary_knowing)',
      'principleOfDerivedKnowing := pose(principle(recently_derived_knowing))',
      'insightIntoConnection := method(reflect(principleOfDerivedKnowing) ⇒ transcendental_knowing)',
      'higherRegionWork := require(search(principleOfAppearanceAsDisjunction))',
      'applicationToOrdinaryKnowing := advise(apply(principle, to = ordinary_actual_knowing))',
      'link(actualityUnity, to = {deriveOrdinaryKnowing})',
    ],
  },
)

// ---------- Export Unit ----------
export const YS_IV_22_UNIT: DatasetUnit = {
  id: makeUnitId('ys-iv-22'),
  title:
    'YS IV.22 — citer apratisaṅkramāyās tad-ākārāpattau sva-buddhi-saṁvedanam',
  scope: 'actuality',
  logosMode: 'appearance',
  synthesis:
    'actuality: non-transference of witness; form-taking yields buddhi self-cognition',
  faculty: 'buddhi',
  lens: 'yoga',
  chunks: CHUNKS_YS_IV_22 as any,
  hlos: HLOS_YS_IV_22 as any,
}

export const YS_IV_22_SYMBOLS = Object.keys(YS_IV_22_ONTOLOGY);

