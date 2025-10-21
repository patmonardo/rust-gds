import { DatasetUnit, makeUnitId } from '@organon/gdsl/registry/canon';

/*
YS IV.11 — Appearance: Absence Rule (Dependent Origination / Asparśa)
hetu-phala-āśraya-ālambana-saṅgṛhītatvād eṣām abhāve tad-abhāva

Literal:
- Because these [phenomena] are gathered/held (saṅgṛhīta) by cause, result, support-basis, and object-support,
  in the absence of those, there is their absence.

Gloss:
- hetu: cause
- phala: result/fruition position within the same complex (not an independent “thing”)
- āśraya: substrate/support-basis
- ālambana: object-support/contact (cognitive/object pole)
- saṅgṛhītatvāt: because they are encompassed/collected under these dependencies
- eṣām: of these (phenomena under discussion)
- abhāve tad-abhāva: when [those supports] are absent, these are absent

Thesis:
What appears (the target phenomena) is factically encompassed by {hetu, phala, āśraya, ālambana}.
Remove any required member and the phenomena do not arise. This is the Absence Rule.
Asparśa-yoga realizes the rule by disabling contact (ālambana).
*/

// ---------- Ontology (single source of truth) ----------
export const YS_IV_11_ONTOLOGY = {
  hetu: 'Cause (initiating condition within the complex)',
  phala: 'Result/fruition role (correlative position within the same complex)',
  asraya: 'Āśraya: substrate/support-basis',
  alambana: 'Ālambana: object-support/contact channel',
  dependencySet: 'D := {hetu, phala, asraya, alambana}',
  sangrhitatva:
    'Facticity: phenomena are “gathered/held” under D (saṅgṛhītatva)',
  phenomenonSet: 'The appearances that only arise when D is intact',
  dependentOriginationLaw: 'Arising is dependent: arise(x) ⇐ allPresent(D)',
  absenceRule:
    'Non-arising by absence: if missingAny(D) ⇒ notArise(phenomenonSet)',
  asparsaMode:
    'Asparśa: disable contact (set ālambana = null) to block arising',
  nonExternalProducer:
    'No extra-external producer beyond D (no miracle override)',
  crosswalkFichte: 'Consequentia: missing antecedent ⇒ no consequent',
  crosswalkHegel: 'Ground–Condition–Appearance schema of Objective Logic',
};

// Extend ontology in-place with Fichte L19 (hypothetical connection, enactment vs saying, intuition crosswalk)
Object.assign(YS_IV_11_ONTOLOGY, {
  hypotheticalConnection:
    'Implication as a connection that rises/falls with the concept (hypothetical modality)',
  presupposedSeeingFixed:
    'Conditioned “seeing” presupposed as fixed in content for hypothetical insight',
  shouldContradiction:
    'In the “should”: enactment (presuppose fixed seeing) vs saying (only possible under condition)',
  propositioFactoContraria:
    'In appearance: proposition contrary to deed (saying vs doing contradiction)',
  derivativeKnowingAppearance:
    'Derivative knowing = appearance (relative knowing)',
  absoluteReasonSayDoCoincide:
    'Absolute reason: what it says and what it does coincide in qualitative sameness',
  implicationRisesFallsWithConcept:
    'Hypothetical truth depends on the concept’s holding; remove concept → connection lapses',
  conceptModeHumanVsDivine:
    'Human conceptual existence (mediated, hypothetical) vs divine/noetic (immediate coincidence)',
  intuitionFormsCrosswalk:
    'Kant’s two sensible forms vs Hegel’s threefold intuition (Being, Essence, Concept)',
});

// Extend ontology with L19 “reconstruction” focus (non-breaking)
Object.assign(YS_IV_11_ONTOLOGY, {
  reconstructionContradiction:
    'Reconstruction contains the appearance-contradiction (saying vs doing) while leaving content unchanged',
  groundlessReconstruction:
    'As mere reconstruction it is groundless relative to absolute necessity unless annulled in descent',
  retainForDescent:
    'Method: retain the contradiction to annul it mediately on the descent',
  absoluteReconstructionLocation:
    'Locate “absolute reconstruction” at the oneness left after complete abstraction',
  pureLightReasonOne:
    'Pure light / reason / inner being are one singularity that remains by itself',
  theOneSelfConstructing:
    'The one is of-itself and self-constructing (positing ≡ constructing)',
  dualityFactical:
    'Duality is annulled intellectually yet remains factically in us',
  noNewContentRule:
    'Reconstruction can alter only form; it cannot create new content without negating relation to the absolute',
});

// L19 — Performative validation: we enacted what we said (saying = doing), so the presupposition stands
Object.assign(YS_IV_11_ONTOLOGY, {
  performativeValidation:
    'Presupposition validated in action: we enacted what we said (saying = doing)',
  barePossibilityShowsTruth:
    'Bare possibility of the presupposition evidences its correctness (here, of the “from” with creation)',
  dropSubordinateProof:
    'Drop subordinate proof; use performative validation as warrant',
  weAsKnowingLight:
    'We (scientists of knowing) are knowing/thinking/light; enactment is immediate light',
  antiFadedSeparation:
    'If knowing were confined to faded thought separated from thinking, presupposition would be unreachable (contradicted by deed)',
});

// L19 — Results (annulment in us; primordial self-conception of light; oneness penetrates the “from”)
Object.assign(YS_IV_11_ONTOLOGY, {
  // Result 1
  annulContradictionInUs:
    'Annulment in us of saying/doing split; criterion of pure reason met',
  criterionPureReason:
    'Criterion: in us, saying = doing (qualitative sameness)',
  ipsoFactoPureReason:
    'Hence ipso facto we are pure reason (for this locus)',
  // Result 2
  primordialSelfConceptionLight:
    'Light has a primordial self-conception of its own nature',
  immediateVisibleCompletion:
    'This self-conception preserves itself in immediate visible completion',
  contentOverFormNow:
    'Hold strictly to content now; form-questions deferred',
  deriveRelationFromAbsolute:
    'Principle: derive relation from the absolute subsequently',
  // Result 3
  lightQualitativeOneness:
    'Light qua light is qualitative oneness (plain seeing, not further seen)',
  onenessPenetratesFrom:
    'This oneness permeates the “from” (genetic relation)',
  dualityOnlyWithinFrom:
    'The a–b duality exists only within the absolute “from”',
  noIndependentDualTerms:
    'No independence/indifferent reversibility of terms outside the “from”',
  sensoryProjectionReversal:
    'Indifferent reversal of terms is a sensory projection, not absolute',
  groundOfPossibilityInWe:
    'Ground of these sensory constructions lies in us as factical concept',
});

// ---------- Chunks ----------
const CHUNKS_YS_IV_11 = [
  {
    id: 'ys-iv-11-text',
    title: 'IV.11 Text & Baseline',
    summary:
      'Absence rule: lacking cause/result/support/object, the encompassed phenomena do not arise.',
  },
  {
    id: 'ys-iv-11-dependencies',
    title: 'Dependency Complex',
    summary:
      'Define D = {hetu, phala, āśraya, alambana}; phenomena are factically encompassed (saṅgṛhitatva).',
  },
  {
    id: 'ys-iv-11-absence-rule',
    title: 'Absence Rule',
    summary:
      'If any required dependency is absent, non-arising follows. No miracle overrides.',
  },
  {
    id: 'ys-iv-11-asparsa',
    title: 'Asparśa Mode (No Contact)',
    summary: 'Disable ālambana (contact) to lawfully prevent arising.',
  },
  {
    id: 'ys-iv-11-crosswalk',
    title: 'Crosswalks',
    summary:
      'Dependent Origination; Fichtean consequentia; Hegelian Ground→Condition→Appearance.',
  },
  {
    id: 'ys-iv-11-errors',
    title: 'Error Modes',
    summary:
      'Errors: treating “absence” as bare negation; adding an external producer.',
  },
  {
    id: 'ys-iv-11-hypothetical',
    title: 'Hypothetical Connection',
    summary:
      'Implication as connection that rises/falls with the concept; presupposed fixed seeing.',
  },
  {
    id: 'ys-iv-11-contradiction',
    title: 'Enactment vs Saying',
    summary:
      'propositio facto contraria: the “should” contradicts itself in appearance.',
  },
  {
    id: 'ys-iv-11-absolute-reason',
    title: 'Absolute Reason',
    summary:
      'In absolute reason, saying and doing coincide; removes the appearance-contradiction.',
  },
  {
    id: 'ys-iv-11-implication-ontology',
    title: 'Ontology of Implication',
    summary:
      'Hypotheticalness belongs to reflection; dependence on the concept, not extra “thinghood.”',
  },
  {
    id: 'ys-iv-11-intuition-crosswalk',
    title: 'Intuition Forms',
    summary:
      'Kant’s two sensible forms vs Hegel’s threefold intuition (Being/Essence/Concept).',
  },
  {
    id: 'ys-iv-11-reconstruction-contradiction',
    title: 'Reconstruction: Contradiction',
    summary:
      'Reconstruction bears the saying/doing split and cannot add new content.',
  },
  {
    id: 'ys-iv-11-absolute-reconstruction',
    title: 'Absolute Reconstruction: Location',
    summary:
      'Identify the locus after complete abstraction: the one that remains by itself.',
  },
  {
    id: 'ys-iv-11-one-light-reason',
    title: 'One = Light = Reason',
    summary:
      'Pure light/reason/inner being as one; of-itself and self-constructing.',
  },
  {
    id: 'ys-iv-11-method-descent',
    title: 'Method: Descent',
    summary:
      'Retain the contradiction now; annul it mediately on the descent.',
  },
  {
    id: 'ys-iv-11-performative',
    title: 'Performative Validation',
    summary:
      'We enacted what we said; presupposition stands by deed (saying = doing).',
  },
  {
    id: 'ys-iv-11-bare-possibility',
    title: 'Bare Possibility ⇒ Warrant',
    summary:
      'The bare possibility of this presupposition shows its correctness for the case at hand.',
  },
  {
    id: 'ys-iv-11-we-light',
    title: 'We as Immediate Light',
    summary:
      'We are knowing/light; not confined to faded thought; hence reach the presupposition.',
  },
  {
    id: 'ys-iv-11-results-1',
    title: 'Result 1: Annulment in Us',
    summary:
      'Say/do contradiction annulled in us; criterion satisfied; ipso facto pure reason.',
  },
  {
    id: 'ys-iv-11-results-2',
    title: 'Result 2: Primordial Self-Conception of Light',
    summary:
      'Light preserves itself in immediate visible completion; hold to content; derive relation later.',
  },
  {
    id: 'ys-iv-11-results-3',
    title: 'Result 3: Oneness Penetrates the “From”',
    summary:
      'Light as qualitative oneness penetrates the “from”; duality exists only within the “from.”',
  },
];

// ---------- HLO Clauses ----------
const HLOS_YS_IV_11 = [
  {
    id: 'ys-iv-11-hlo-baseline',
    chunkId: 'ys-iv-11-text',
    label: 'Baseline',
    clauses: [
      "tag('sutra','IV.11')",
      'dependencySet := {hetu, phala, asraya, alambana}',
      'sangrhitatva := forall(x in phenomenonSet) require(dependencySet)',
    ],
  },
  {
    id: 'ys-iv-11-hlo-dependencies',
    chunkId: 'ys-iv-11-dependencies',
    label: 'Dependency Complex',
    clauses: [
      'dependentOriginationLaw := schema( arise(x) ⇐ allPresent(dependencySet) )',
    ],
  },
  {
    id: 'ys-iv-11-hlo-absence',
    chunkId: 'ys-iv-11-absence-rule',
    label: 'Absence Rule',
    clauses: [
      'absenceRule := rule( if(missingAny(dependencySet)) ⇒ notArise(phenomenonSet) )',
      'nonExternalProducer := assert(noOverride(absenceRule))',
    ],
  },
  {
    id: 'ys-iv-11-hlo-asparsa',
    chunkId: 'ys-iv-11-asparsa',
    label: 'Asparśa (No Contact)',
    clauses: [
      'asparsaMode := set(alambana, null)',
      'notArise(phenomenonSet) ⇐ asparsaMode',
    ],
  },
  {
    id: 'ys-iv-11-hlo-crosswalk',
    chunkId: 'ys-iv-11-crosswalk',
    label: 'Crosswalk',
    clauses: [
      'map(absenceRule ↔ pratityasamutpada_nonarising)',
      'map(absenceRule ↔ crosswalkFichte)',
      'map(dependencySet ↔ crosswalkHegel)',
    ],
  },
  {
    id: 'ys-iv-11-hlo-errors',
    chunkId: 'ys-iv-11-errors',
    label: 'Errors',
    clauses: [
      'error_externalProducer ⇐ posit(externalProducer overrides absenceRule)',
      'error_negationOnly ⇐ define(absence) without(reference(dependencySet))',
    ],
  },
  {
    id: 'ys-iv-11-hlo-hypothetical',
    chunkId: 'ys-iv-11-hypothetical',
    label: 'Hypothesis',
    clauses: [
      'hypotheticalConnection := define(implication as connection(concept))',
      'presupposedSeeingFixed := require(fixedContent(seeing) for hypotheticalConnection)',
    ],
  },
  {
    id: 'ys-iv-11-hlo-contradiction',
    chunkId: 'ys-iv-11-contradiction',
    label: 'Contradiction',
    clauses: [
      'shouldContradiction := contrast(enact(presupposeFixedSeeing), say(possibleUnderCondition))',
      'propositioFactoContraria := mark(appearance, contradiction(saying, doing))',
      'derivativeKnowingAppearance := alias(derivativeKnowing, appearance)',
    ],
  },
  {
    id: 'ys-iv-11-hlo-absolute-reason',
    chunkId: 'ys-iv-11-absolute-reason',
    label: 'Coincidence',
    clauses: [
      'absoluteReasonSayDoCoincide := assert(say(x) == do(x))',
      'resolve(contradiction in appearance) ⇐ absoluteReasonSayDoCoincide',
    ],
  },
  {
    id: 'ys-iv-11-hlo-implication-ontology',
    chunkId: 'ys-iv-11-implication-ontology',
    label: 'Reflection',
    clauses: [
      'implicationRisesFallsWithConcept := rule(holds(implication) ⇔ holds(concept))',
      'forbid(reify(implication as thing))',
    ],
  },
  {
    id: 'ys-iv-11-hlo-intuition-crosswalk',
    chunkId: 'ys-iv-11-intuition-crosswalk',
    label: 'Crosswalk',
    clauses: [
      'intuitionFormsCrosswalk := map({kant: twoSensibleForms, hegel: threefoldIntuition(Being, Essence, Concept)})',
      'conceptModeHumanVsDivine := contrast(human(hypothetical, mediated), divine(immediate, coincident))',
    ],
  },
  {
    id: 'ys-iv-11-hlo-reconstruction-contradiction',
    chunkId: 'ys-iv-11-reconstruction-contradiction',
    label: 'Contradiction',
    clauses: [
      'reconstructionContradiction := contain(shouldContradiction in reconstruction)',
      'noNewContentRule := assert(formChangeOnly(reconstruction))',
      'groundlessReconstruction := diagnose(reconstruction, lacks(absoluteGround))',
    ],
  },
  {
    id: 'ys-iv-11-hlo-absolute-reconstruction',
    chunkId: 'ys-iv-11-absolute-reconstruction',
    label: 'Location',
    clauses: [
      'absoluteReconstructionLocation := at(oneness after completeAbstraction)',
      'dualNature := note(dualityFactical ∧ annulledIntellectually)',
    ],
  },
  {
    id: 'ys-iv-11-hlo-one-light-reason',
    chunkId: 'ys-iv-11-one-light-reason',
    label: 'Unity',
    clauses: [
      'pureLightReasonOne := assert(light ≡ reason ≡ innerBeing)',
      'theOneSelfConstructing := assert(posit(one as such) ≡ construct(one))',
    ],
  },
  {
    id: 'ys-iv-11-hlo-method-descent',
    chunkId: 'ys-iv-11-method-descent',
    label: 'Method',
    clauses: [
      'retainForDescent := method(hold(contradiction) → annul(mediately, onDescent))',
    ],
  },
  {
    id: 'ys-iv-11-hlo-performative',
    chunkId: 'ys-iv-11-performative',
    label: 'Deed as Proof',
    clauses: [
      'performativeValidation := assert(say(x) == do(x))',
      'use(dropSubordinateProof) ⇐ performativeValidation',
      'link(performativeValidation, absoluteReasonSayDoCoincide)',
    ],
  },
  {
    id: 'ys-iv-11-hlo-bare-possibility',
    chunkId: 'ys-iv-11-bare-possibility',
    label: 'Possibility/Warrant',
    clauses: [
      'barePossibilityShowsTruth := argue(possible(presupposition) ⇒ warranted(presupposition))',
      'scope(barePossibilityShowsTruth, this_case_only)',
    ],
  },
  {
    id: 'ys-iv-11-hlo-we-light',
    chunkId: 'ys-iv-11-we-light',
    label: 'We/Light',
    clauses: [
      'weAsKnowingLight := assert(we == knowing == light)',
      'antiFadedSeparation := refute(limit(knowing, fadedThought ∥ separatedFrom(thinking)))',
      'reach(presupposition) ⇐ weAsKnowingLight',
    ],
  },
  {
    id: 'ys-iv-11-hlo-results-1',
    chunkId: 'ys-iv-11-results-1',
    label: 'Annulment',
    clauses: [
      'annulContradictionInUs := assert(say(x) == do(x))',
      'criterionPureReason := mark(annulContradictionInUs)',
      'ipsoFactoPureReason := conclude(criterionPureReason)',
      'link(annulContradictionInUs, absoluteReasonSayDoCoincide)',
    ],
  },
  {
    id: 'ys-iv-11-hlo-results-2',
    chunkId: 'ys-iv-11-results-2',
    label: 'Primordial Light',
    clauses: [
      'primordialSelfConceptionLight := assert(light knows(light))',
      'immediateVisibleCompletion := assert(selfPreserving(light))',
      'contentOverFormNow := scope(hold(content))',
      'deriveRelationFromAbsolute := defer(derive(relation, from = absolute))',
    ],
  },
  {
    id: 'ys-iv-11-hlo-results-3',
    chunkId: 'ys-iv-11-results-3',
    label: 'Oneness ↔ From',
    clauses: [
      'lightQualitativeOneness := assert(oneness(light))',
      'onenessPenetratesFrom := assert(penetrates(oneness(light), "from"))',
      'dualityOnlyWithinFrom := assert(exists(duality(a,b), onlyWithin("from")))',
      'noIndependentDualTerms := forbid(independent({a,b}, outside("from")))',
      'sensoryProjectionReversal := diagnose(reversalIndifference as sensoryProjection)',
      'groundOfPossibilityInWe := locate(ground(sensoryProjection), in = we_factical_concept)',
      'map("from" ↔ dependentOriginationLaw)',
    ],
  },
];

// ---------- Export Unit ----------
export const YS_IV_11_UNIT: DatasetUnit = {
  id: makeUnitId('ys-iv-11'),
  title:
    'YS IV.11 — hetu-phala-āśraya-ālambana-saṅgṛhītatvād eṣām abhāve tad-abhāva',
  scope: 'appearance',
  logosMode: 'appearance',
  synthesis: 'absence-rule-dependent-origination',
  faculty: 'buddhi',
  lens: 'yoga',
  chunks: CHUNKS_YS_IV_11 as any,
  hlos: HLOS_YS_IV_11 as any,
};

export const YS_IV_11_SYMBOLS = Object.keys(YS_IV_11_ONTOLOGY);
