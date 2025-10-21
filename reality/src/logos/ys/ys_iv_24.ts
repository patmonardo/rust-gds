import { DatasetUnit, makeUnitId } from '@organon/gdsl/registry/canon';

/*
C. ACTUALITY

YS IV.24 — tad asaṁkhyeya-vāsanābhi citram api parārtham saṁhatyā-kāritvāt

“Though variegated by countless impressions, that (citta) is for another (parārtham), because it acts as an aggregate (is produced by conjunction).”

Reading (Self‑realization, triadic)
- Asaṁkhyeya-vāsanābhi citram: the mind is multi-patterned by innumerable latent dispositions.
- Parārtham: “for another” — the mind serves the seer; it is instrumentality, not self-purpose.
- Saṁhatyā-kāritvāt: because it functions as a composite/aggregate (conjunction), it cannot be ultimate subject.
- Actuality: preserves non-transference; the seer illumines, the mind bears forms “for” the seer.
- Triadic self-realization: seer, seen, and mind align; mind recognizes its for-another status and turns toward the seer.
*/

// ---------- Ontology ----------
export const YS_IV_24_ONTOLOGY = {
  asamkhyeyaVasanabhi: 'By countless latent impressions (vāsanās)',
  citram: 'Variegated/multicolored; manifoldly patterned',
  parartham: 'For another; instrumentality serving the seer (puruṣa)',
  samhatyaKaritvat:
    'Because it acts by aggregation/conjunction (composite causation)',
  citta: 'Mind-stream/buddhi as reflective composite (saṁhati)',
  instrumentalityForSeer: 'Citta’s purpose is for puruṣa; not for itself',
  notUltimateSubject:
    'As composite, citta is not the ultimate subject (no self-end)',
  triadicSelfRealization:
    'Self-realization as triad: seer illumines, mind bears forms for seer, objects provide forms',
  // Carry-overs (Actuality frame)
  seerWithoutObjecthood:
    'Witness-only seer (puruṣa), never an object among objects',
  apratisankramaya: 'Non-transference of consciousness (IV.22)',
  notSelfLuminousCitta: 'Mind is not self-luminous (IV.19)',
  noDoubleDetermination:
    'No single act fixes both seer/seen simultaneously (IV.20)',
  sattvaConstraint: 'Śuddha-sattva transparency mediates knownness',
  // Crosswalks
  fichteInstrumentality:
    'Derived knowing as instrument; ascent to transcendental knowing clarifies its for-another',
  hegelBeingForAnother:
    'Für-Anderes: being-for-another as moment within actuality (distinct from in-and-for-itself)',
  // Guards
  errorCittaForItself:
    'Error: treating citta as for-itself (self-purpose/ultimate subject)',
  errorPurushaAsAgent:
    'Error: turning witness into an acting composite/aggregate',
};

// ---------- Chunks ----------
const CHUNKS_YS_IV_24 = [
  {
    id: 'ys-iv-24-text',
    title: 'IV.24 Text & Baseline',
    summary:
      'Mind variegated by countless impressions is “for another,” because it acts as an aggregate.',
  },
  {
    id: 'ys-iv-24-semantics',
    title: 'Semantics: vāsanā / citram / parārtham / saṁhatyā-kāritva',
    summary:
      'Define dispositions, variegation, for-another instrumentality, and aggregate causation.',
  },
  {
    id: 'ys-iv-24-actuality',
    title: 'Actuality and Self‑Realization (Triadic)',
    summary:
      'Non-transference preserved; mind’s for-another clarifies the triad seer–mind–seen.',
  },
  {
    id: 'ys-iv-24-crosswalk',
    title: 'Crosswalk (Fichte/Hegel)',
    summary: 'Instrumentality and being-for-another situated within actuality.',
  },
  {
    id: 'ys-iv-24-guards',
    title: 'Guards',
    summary: 'Block “mind-for-itself” and “seer-as-agent” errors.',
  },
  {
    id: 'ys-iv-24-bridges',
    title: 'Bridges',
    summary:
      'From parārtham to explicit Self-Realization flow; prepare liberation grammar.',
  },
];

// ---------- HLO Clauses ----------
const HLOS_YS_IV_24 = [
  {
    id: 'ys-iv-24-hlo-text',
    chunkId: 'ys-iv-24-text',
    label: 'Baseline',
    clauses: [
      "tag('sutra','IV.24')",
      'assert(asamkhyeyaVasanabhi ∧ citram(citta))',
      'conclude(parartham(citta))',
      'justify(parartham, by = samhatyaKaritvat)',
    ],
  },
  {
    id: 'ys-iv-24-hlo-semantics',
    chunkId: 'ys-iv-24-semantics',
    label: 'Semantics',
    clauses: [
      'define(asamkhyeyaVasanabhi := countless_latent_impressions)',
      'define(citram := variegated_by_vasanas)',
      'define(parartham := for_another(instrumentality_for_seer))',
      'define(samhatyaKaritvat := because(composite_conjunction_operation))',
      'notSelfLuminousCitta := import(IV_19.notSelfLuminousCitta)',
      'noDoubleDetermination := import(IV_20.noDoubleDetermination)',
      'apratisankramaya := import(IV_22.apratisankramaya)',
    ],
  },
  {
    id: 'ys-iv-24-hlo-actuality',
    chunkId: 'ys-iv-24-actuality',
    label: 'Actuality',
    clauses: [
      'instrumentalityForSeer := assert(parartham(citta) ∧ seerWithoutObjecthood)',
      'notUltimateSubject := assert(samhatyaKaritvat ⇒ ¬ultimate_subject(citta))',
      'triadicSelfRealization := assert(align({seer, citta, object_form}) ∧ preserve(non_transference))',
      'sattvaConstraint := constrain(knownness, by = sattva_transparency)',
    ],
  },
  {
    id: 'ys-iv-24-hlo-crosswalk',
    chunkId: 'ys-iv-24-crosswalk',
    label: 'Crosswalk',
    clauses: [
      'fichteInstrumentality := note(ordinary_knowing as instrument; clarified by transcendental method)',
      'hegelBeingForAnother := map(parartham ↔ being_for_another within actuality)',
    ],
  },
  {
    id: 'ys-iv-24-hlo-guards',
    chunkId: 'ys-iv-24-guards',
    label: 'Guards',
    clauses: [
      'errorCittaForItself := flag(assert(citta_for_itself))',
      'errorPurushaAsAgent := flag(assert(purusha acts_as_aggregate))',
    ],
  },
  {
    id: 'ys-iv-24-hlo-bridges',
    chunkId: 'ys-iv-24-bridges',
    label: 'Bridges',
    clauses: [
      'link(IV_23.sarvartham → parartham)',
      'prepare(self_realization_flow := articulate(citta_turns_toward_seer by seeing(parartham)))',
      'prepare(IV_25_plus := liberation_grammar from triadicSelfRealization)',
    ],
  },
];

// ---------- Fichte Addendum (Self-Realization triad: Viyoga/Saṁyoga; “Should” as connector) ----------
Object.assign(YS_IV_24_ONTOLOGY, {
  viyoga:
    'Disjunction/distinction (seer vs mind vs seen) — explicit viśeṣa-darśana ground',
  sanyoga:
    'Conjunction/aggregation (saṁhati) — the composite operation of citta',
  buddhiEvaLaw:
    'Buddhi‑eva recognizes its law: Cit–Citi (witness light and knowing act) as its principle; hence parārtham',
  hypotheticalShouldConnector:
    'Fichte’s “should” (Sollen) as pure connector linking conditioning/conditioned — the yoga‑operator',
  ordinaryKnowingShould:
    'In ordinary knowing, “should” operates tacitly with assumed premises (yokes appearances)',
  transcendentalKnowingShould:
    'In transcendental knowing, premises are grounded genetically as emanations of the “should”',
  middlePremiseDual:
    'Middle ground (premise) with duality: two outer grounds distinguished (ordinary vs transcendental) yet united',
  compassTriad:
    'Compass for the path: Seer (Cit/Citi), Citta (saṁyoga), and Object (ākāra) aligned without role collapse',
  masterViyogaSanyoga:
    'Mastery = seeing distinction (viyoga) and conjunction (saṁyoga) together as law; mind is for‑another',
});

CHUNKS_YS_IV_24.push(
  {
    id: 'ys-iv-24-fichte-should',
    title: 'Fichte — “Should” as Connector (Yoga-Operator)',
    summary:
      'The hypothetical “should” creates connection; tacit in ordinary knowing, genetic in transcendental knowing; provides the compass.',
  },
  {
    id: 'ys-iv-24-viyoga-samyoga',
    title: 'Viyoga · Saṁyoga — Buddhi‑eva Law',
    summary:
      'Distinctly see Cit/Citi/Citta: mind as aggregate (saṁyoga) for another, governed by Cit–Citi; mastery of viyoga/saṁyoga.',
  },
);

HLOS_YS_IV_24.push(
  {
    id: 'ys-iv-24-hlo-fichte-should',
    chunkId: 'ys-iv-24-fichte-should',
    label: 'Connector/Compass',
    clauses: [
      'hypotheticalShouldConnector := assert(connector(conditions ↔ conditioned))',
      'ordinaryKnowingShould := note(operates_tacitly in = ordinary_knowing)',
      'transcendentalKnowingShould := assert(grounds(premises) genetically)',
      'middlePremiseDual := pose(premise as middle that unites_and_separates {ordinary, transcendental})',
      'compassTriad := assert(path_compass({seer(Cit,Citi), citta(saṁyoga), object(ākāra)}))',
    ],
  },
  {
    id: 'ys-iv-24-hlo-viyoga-samyoga',
    chunkId: 'ys-iv-24-viyoga-samyoga',
    label: 'Mastery',
    clauses: [
      'viyoga := assert(distinguish(seerWithoutObjecthood, citta, object))',
      'sanyoga := assert(conjunction(saṁhati_operation(citta)))',
      'buddhiEvaLaw := assert(buddhi recognizes(law = Cit ⊕ Citi) ∧ conclude(parartham(citta)))',
      'masterViyogaSanyoga := assert(see(viyoga ∧ sanyoga) ⇒ self_realization_flow)',
      'link(hegelBeingForAnother, to = parartham)',
      'link(fichteInstrumentality, to = {ordinaryKnowingShould, transcendentalKnowingShould})',
    ],
  },
);

// ---------- Fichte Addendum (Parts 2–4): “Should”, Categorical Turn, Creative Method, Proof ----------
Object.assign(YS_IV_24_ONTOLOGY, {
  shouldReturnsInCompletedInsight:
    'In completed insight a hypothetical “should” reappears as connector within insight itself',
  termsNowAPrioriInInsight:
    'Previously factical terms now reside a priori within the genetic insight (empirical construction can be dropped)',
  innerHypotheticalnessWe:
    'Inner root of hypotheticalness = the “free We” (science of knowing) producing/containing itself',
  negateInnerHypotheticalness:
    'Negate inward hypotheticalness to manifest categorical character (truth/necessity/priority)',
  categoricalValidityInference:
    'Inference (self‑genesis ∧ being) holds only mediately in insight into their oneness; categorically validated',
  methodBecomesCreative:
    'Method remark: here method becomes absolutely creative; it must justify itself',
  scienceProvesItself:
    'Science of knowing should justify/prove itself before it truly begins (liberates from arbitrariness)',
  liberatedFromArbitrariness:
    'Liberation from freedom/arbitrariness/accident by self‑justifying method',
  insightActualInKnowing:
    'We produced this insight; thus it is possible in knowing and actual in our present knowing',
  geneticOnenessInsight:
    'Genetic insight unites the two terms (self‑genesis and being) per rules/maxims',
  // Yoga crosswalk anchors
  categoricalParartham:
    'Mind’s for‑another (parārtham) read categorically once hypothetical yoke is internalized/genetically grounded',
  premiseMiddleTriad:
    'Premise as middle ground uniting/separating ordinary vs transcendental knowing (triadic compass)',
});

CHUNKS_YS_IV_24.push(
  {
    id: 'ys-iv-24-fichte-should-ii',
    title: 'Fichte — “Should” in Completed Insight',
    summary:
      'Terms now a priori in insight; “should” reappears as inner connector; genetic oneness replaces factical assumption.',
  },
  {
    id: 'ys-iv-24-fichte-categorical-method-proof',
    title: 'Fichte — Categorical Turn, Creative Method, Proof',
    summary:
      'Negate inner hypotheticalness → categorical validity; method proves itself; insight actual in knowing.',
  },
);

HLOS_YS_IV_24.push(
  {
    id: 'ys-iv-24-hlo-fichte-should-ii',
    chunkId: 'ys-iv-24-fichte-should-ii',
    label: 'Completed Insight',
    clauses: [
      'shouldReturnsInCompletedInsight := assert(hypothetical_should within(insight))',
      'termsNowA PrioriInInsight := conclude(terms reside a_priori in genetic_insight)',
      'geneticOnenessInsight := assert(unite(self_genesis, enduring_being) by rules|maxims)',
      'categoricalParartham := link(parartham ← internalize(connector_should) as genetic)',
      'premiseMiddleTriad := note(premise as middle unites_and_separates {ordinary, transcendental})',
    ],
  },
  {
    id: 'ys-iv-24-hlo-fichte-categorical-method-proof',
    chunkId: 'ys-iv-24-fichte-categorical-method-proof',
    label: 'Categorical/Method/Proof',
    clauses: [
      'innerHypotheticalnessWe := assert(we == inner_root(hypotheticalness))',
      'negateInnerHypotheticalness := method(exhibit(categorical_character))',
      'categoricalValidityInference := assert(only_mediately(self_genesis ∧ being) in insight(one))',
      'methodBecomesCreative := note(method == absolutely_creative)',
      'scienceProvesItself := require(self_justification(method ∧ maxims))',
      'liberatedFromArbitrariness := conclude(no(arbitrariness|accident))',
      'insightActualInKnowing := assert(produced(insight) ∧ actual(now, in = knowing))',
      // Yoga fit
      'link(notUltimateSubject, to = {categoricalParartham})',
      'link(masterViyogaSanyoga, to = {premiseMiddleTriad})',
    ],
  },
);

// ---------- Export Unit ----------
export const YS_IV_24_UNIT: DatasetUnit = {
  id: makeUnitId('ys-iv-24'),
  title:
    'YS IV.24 — tad asaṁkhyeya-vāsanābhi citram api parārtham saṁhatyā-kāritvāt',
  scope: 'actuality',
  logosMode: 'appearance',
  synthesis:
    'variegated-by-vāsanās mind is for another due to aggregate operation; triadic self-realization',
  faculty: 'buddhi',
  lens: 'yoga',
  chunks: CHUNKS_YS_IV_24 as any,
  hlos: HLOS_YS_IV_24 as any,
};

export const YS_IV_24_SYMBOLS = Object.keys(YS_IV_24_ONTOLOGY);
