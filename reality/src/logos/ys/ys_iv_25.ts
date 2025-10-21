import { DatasetUnit, makeUnitId } from '@organon/gdsl/registry/canon'

/*
C. ACTUALITY → Absolute Realization

YS IV.25 — viśeṣa-darśina ātma-bhāva-bhāvanā-vinivṛtti

“For the one who sees the distinction (viśeṣa-darśin), there is cessation of the cultivation of self-being (ātma-bhāva-bhāvanā).”
*/

// ---------- Ontology ----------
export const YS_IV_25_ONTOLOGY = {
  visheshaDarsina: 'Discriminative seer: one who sees distinctions (viśeṣa-darśin)',
  atmaBhava: 'Sense of self-being (ātma-bhāva) posited in mind',
  bhavana: 'Cultivation/fabrication/practice of a stance (bhāvanā)',
  vinivrtti: 'Cessation/withdrawal (vinivṛtti)',
  cessationOfSelfCultivation:
    'Cessation of cultivating the self-being stance in/through citta',
  deappropriation:
    'Non-appropriation: buddhi’s operations not taken as “I” (end of selfing)',
  // Carry-overs
  parartham: 'Mind is for another (IV.24); not ultimate subject',
  sarvartham: 'Mind is omni-instrumental (IV.23)',
  apratisankramaya: 'Non-transference of consciousness (IV.22)',
  seerWithoutObjecthood: 'Witness-only seer (IV.21–IV.22)',
  notSelfLuminousCitta: 'Mind not self-luminous (IV.19)',
  noDoubleDetermination: 'No simultaneous seer/seen fixation (IV.20)',
  // Crosswalks
  masteryViyogaSanyoga:
    'Mastery of distinction/conjunction (from IV.24) grounds cessation of selfing',
  fichteCategoricalTurn:
    'Negating inner hypotheticalness → categorical validity (selfing “should” drops as insight becomes law)',
  hegelSublateForItself:
    'Being-for-itself (selfing) is sublated within actuality into witness-aligned stance',
  // Guards
  errorAsceticSuppression:
    'Error: reading cessation as mere suppression/force rather than insight-grounded deappropriation',
  errorConflatePurushaBrahman:
    'Error: collapsing Purusha (tattva) into Brahman (nondual) at this stage',
  errorNihilistSelfDenial:
    'Error: nihilistic denial of experiential stream; only the “selfing” attribution ceases',
}

// ---------- Chunks ----------
const CHUNKS_YS_IV_25 = [
  {
    id: 'ys-iv-25-text',
    title: 'IV.25 Text & Baseline',
    summary:
      'For the discriminative seer, cultivation of the self-being stance ceases.',
  },
  {
    id: 'ys-iv-25-semantics',
    title: 'Semantics: viśeṣa-darśin / ātma-bhāva / bhāvanā / vinivṛtti',
    summary:
      'Define discriminative vision, self-being stance, its cultivation, and cessation.',
  },
  {
    id: 'ys-iv-25-realization',
    title: 'Absolute Realization (Cessation of Selfing)',
    summary:
      'Insight de-appropriates buddhi; parārtham/sarvārtham recognized; witness remains non-transferring.',
  },
  {
    id: 'ys-iv-25-crosswalk',
    title: 'Crosswalk (Fichte/Hegel)',
    summary:
      'Categorical turn (Fichte) and sublation of for-itself (Hegel) as grammar of cessation.',
  },
  {
    id: 'ys-iv-25-guards',
    title: 'Guards',
    summary:
      'Block suppression/nihilism and Purusha=Brahman collapse.',
  },
  {
    id: 'ys-iv-25-bridges',
    title: 'Bridges',
    summary:
      'Prepare the remaining Absolute sequence toward kaivalya.',
  },
]

// ---------- HLO Clauses ----------
const HLOS_YS_IV_25 = [
  {
    id: 'ys-iv-25-hlo-text',
    chunkId: 'ys-iv-25-text',
    label: 'Baseline',
    clauses: [
      "tag('sutra','IV.25')",
      'assert(visheshaDarsina)',
      'conclude(cessationOfSelfCultivation := vinivrtti( bhavana(atmaBhava) ))',
    ],
  },
  {
    id: 'ys-iv-25-hlo-semantics',
    chunkId: 'ys-iv-25-semantics',
    label: 'Semantics',
    clauses: [
      'define(visheshaDarsina := discriminative_vision(distinctions: seer/mind/seen))',
      'define(atmaBhava := posited_self_being_in_citta)',
      'define(bhavana := cultivation/fabrication/practice)',
      'define(vinivrtti := cessation/withdrawal)',
      'notSelfLuminousCitta := import(IV_19.notSelfLuminousCitta)',
      'noDoubleDetermination := import(IV_20.noDoubleDetermination)',
      'apratisankramaya := import(IV_22.apratisankramaya)',
      'parartham := import(IV_24.parartham)',
      'sarvartham := import(IV_23.sarvartham)',
    ],
  },
  {
    id: 'ys-iv-25-hlo-realization',
    chunkId: 'ys-iv-25-realization',
    label: 'Realization',
    clauses: [
      'deappropriation := assert(see(parartham ∧ sarvartham) ⇒ cease(appropriate(buddhi_ops, as = "I"))) ',
      'assert(seerWithoutObjecthood ∧ apratisankramaya)',
      'masteryViyogaSanyoga := import(IV_24.masterViyogaSanyoga)',
      'conclude(cessationOfSelfCultivation)',
    ],
  },
  {
    id: 'ys-iv-25-hlo-crosswalk',
    chunkId: 'ys-iv-25-crosswalk',
    label: 'Crosswalk',
    clauses: [
      'fichteCategoricalTurn := map(negate(inner_hypotheticalness) ⇒ categorical(cessationOfSelfCultivation))',
      'hegelSublateForItself := map(sublate(for_itself_selfing) within actuality → witness_alignment)',
    ],
  },
  {
    id: 'ys-iv-25-hlo-guards',
    chunkId: 'ys-iv-25-guards',
    label: 'Guards',
    clauses: [
      'errorAsceticSuppression := flag(read(cessation as mere_suppression))',
      'errorConflatePurushaBrahman := flag(collapse(purusha → brahman))',
      'errorNihilistSelfDenial := flag(deny(stream_of_experience))',
    ],
  },
  {
    id: 'ys-iv-25-hlo-bridges',
    chunkId: 'ys-iv-25-bridges',
    label: 'Bridges',
    clauses: [
      'prepare(IV_26_plus := unfold(discriminative_knowledge → kaivalya_path))',
    ],
  },
]

// ---------- Fichte Addendum: Absolute Genesis (Kant link: possibility/facticity) ----------
Object.assign(YS_IV_25_ONTOLOGY, {
  absoluteGenesisSelfEnclosed:
    'Absolute self-enclosed genesis; not a genesis-of-a-genesis (negates itself inwardly within knowing)',
  notGenesisOfGenesis:
    'Guard: deny regress of a genesis of a genesis (no meta-genesis stack)',
  manifestsAsGenesisForUs:
    'To us (constructing in laws) it manifests as genesis; for immediate knowing it does not',
  immediateIntuitionAsBeing:
    'In immediate knowing: persisting intuition as non-genesis = being',
  nonGenesisEqualsBeing:
    'Non-genesis explicitly equals being (result external as persistence)',
  proofViaPossibilityAndFacticity:
    'Proof conducted purely via possibility and facticity (Kantian cue for “Absolute”)',
  coincidenceFacticityGenesis:
    'In this case, facticity and genesis entirely coincide',
  immediateFacticityIsAbsoluteGenesis:
    'Knowing’s immediate facticity just is absolute genesis (no further ground)',
  noFurtherGround:
    'No possible further ground beyond absolute genesis (ground terminates here)',
  arrivalAtGroundNecessity:
    'It must be so if one is ever to arrive at “the ground” (termination condition)',
  kantAbsoluteDefinitionCue:
    'Kant cue: possibility/facticity used to fix the sense of “Absolute” (without appeal to external grounds)',
  errorSeekExternalGround:
    'Error: seeking an external/deeper ground for absolute genesis (category mistake)',
})

// Chunks (two short parts a/b)
CHUNKS_YS_IV_25.push(
  {
    id: 'ys-iv-25-fichte-abs-a',
    title: 'Fichte — Absolute Genesis (a)',
    summary:
      'Self-enclosed genesis; not genesis-of-genesis; for us genesis, for immediate knowing: persisting intuition = being.',
  },
  {
    id: 'ys-iv-25-fichte-abs-b',
    title: 'Fichte — Proof via Possibility & Facticity (b)',
    summary:
      'Proof by possibility/facticity; coincidence of facticity and genesis; no further ground.',
  },
)

// HLOs
HLOS_YS_IV_25.push(
  {
    id: 'ys-iv-25-hlo-fichte-abs-a',
    chunkId: 'ys-iv-25-fichte-abs-a',
    label: 'Absolute Genesis (a)',
    clauses: [
      'absoluteGenesisSelfEnclosed := assert(self_enclosed(absolute_genesis))',
      'notGenesisOfGenesis := deny(genesis(genesis))',
      'manifestsAsGenesisForUs := note(for_us(construct_in_laws) ⇒ appears_as(genesis))',
      'immediateIntuitionAsBeing := assert(in_immediate_knowing(persisting_intuition) == non_genesis == being)',
      'nonGenesisEqualsBeing := assert(non_genesis == being)',
      // Yoga fit
      'link(cessationOfSelfCultivation, to = {visheshaDarsina})',
    ],
  },
  {
    id: 'ys-iv-25-hlo-fichte-abs-b',
    chunkId: 'ys-iv-25-fichte-abs-b',
    label: 'Possibility/Facticity (b)',
    clauses: [
      'proofViaPossibilityAndFacticity := assert(proof(absolute_genesis) via {possibility, facticity})',
      'coincidenceFacticityGenesis := assert(coincide(facticity, genesis))',
      'immediateFacticityIsAbsoluteGenesis := assert(immediate_facticity(knowing) == absolute_genesis)',
      'noFurtherGround := assert(no(further_ground beyond absolute_genesis))',
      'arrivalAtGroundNecessity := justify(must_be_so ⇒ arrive_at(ground))',
      'kantAbsoluteDefinitionCue := note(kant_cue(absolute via modality: possibility ∧ facticity))',
      // Yoga fit
      'link(deappropriation, to = {categoricalValidityInference})',
    ],
  },
  // Extra guards under existing Guards section (reuse chunk)
  {
    id: 'ys-iv-25-hlo-guards-abs',
    chunkId: 'ys-iv-25-guards',
    label: 'Guards (Absolute)',
    clauses: [
      'notGenesisOfGenesis',
      'errorSeekExternalGround := flag(seek(external_ground for absolute_genesis))',
    ],
  },
)

// ---------- Export Unit ----------
export const YS_IV_25_UNIT: DatasetUnit = {
  id: makeUnitId('ys-iv-25'),
  title:
    'YS IV.25 — viśeṣa-darśina ātma-bhāva-bhāvanā-vinivṛtti',
  scope: 'actuality',
  logosMode: 'appearance',
  synthesis:
    'discriminative vision ends the cultivation of self-being; de-appropriation under witness non-transference',
  faculty: 'buddhi',
  lens: 'yoga',
  chunks: CHUNKS_YS_IV_25 as any,
  hlos: HLOS_YS_IV_25 as any,
}

export const YS_IV_25_SYMBOLS = Object.keys(YS_IV_25_ONTOLOGY);
