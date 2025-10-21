import { DatasetUnit, makeUnitId } from '@organon/gdsl/registry/canon';

// YS IV.34 — puruṣārtha-śūnyānāṁ guṇānāṁ pratiprasavaḥ kaivalyaṁ svarūpa-pratiṣṭhā vā citi-śaktir iti
export const YS_IV_34_ONTOLOGY = {
  sutraDevanagari:
    'पुरुषार्थशून्यानां गुणानां प्रतिप्रसवः कैवल्यं स्वरूपप्रतिष्ठा वा चितिशक्तिरिति',
  sutraIAST:
    'puruṣārtha-śūnyānāṁ guṇānāṁ pratiprasavaḥ kaivalyaṁ svarūpa-pratiṣṭhā vā citi-śaktir iti',
  sutraGloss:
    'Kaivalya: the return of the guṇas to their source when devoid of purpose for Puruṣa; or establishment in one’s own nature — the power of consciousness.',
  purusharthaShunyana:
    'puruṣārtha-śūnyānām: “devoid of purpose for Puruṣa” — guṇas no longer serve ends',
  gunanamPratiprasava:
    'guṇānāṁ pratiprasavaḥ: “counter-flow/return” of guṇas to their source',
  kaivalyam:
    'kaivalyaṁ: isolation/independence of the Seer; freedom from the guṇas',
  svarupaPratistha:
    'svarūpa-pratiṣṭhā: establishment in one’s own form (nature)',
  citiShakti:
    'citi-śaktiḥ: power of consciousness; pure seeing established in itself',
  vaReadingGuard:
    'vā (“or”): two perspectives of the same closure (return of guṇas vs establishment in Self)',
  absoluteEssenceClosure:
    'Absolute Essence closure: after absolute relation is fixed (IV.33), guṇas have no further purpose → return; Seer abides independently',
  bridgeFromIv33:
    'Bridge: absolute necessity (law of lawfulness) completes relation; hence puruṣārtha is exhausted → kaivalya',
} as const;

Object.assign(YS_IV_34_ONTOLOGY, {
  // L25 — Genesis of appearance; particular knowing returns
  l25ScienceEmergesAsParticular:
    'Path forward: the science of knowing must re-emerge as a particular knowing',
  l25WeAscendedMustExplainGenesis:
    'We ascended to the present insight; now explain its genesis (not empirically/artificially, but genetically)',
  l25GenesisOfAppearanceNotOrigin:
    'Genesis sought is of absolute knowing’s appearance in us (its actual existence), not of absolute knowing itself (which has no origin)',
  l25ScienceInSpecie:
    'Science of knowing in specie: a particular knowing whose nonexistence is as possible as its existence',

  // L25 — Ordinary knowing as primordial condition
  l25OrdinaryKnowingPrimordialCondition:
    'Ordinary knowing may be the primordial condition for the genetic possibility of absolute knowing’s appearance',
  l25ExplainDeterminationsFromPresupposition:
    'Determinations can be explained from the presupposition that the science of knowing ought to arise',

  // L25 — Rational inference schema
  l25RationalInferenceSchema:
    'Schema: “If absolute knowing is to appear, then X must …; now, knowing is thus determined; therefore this should happen.”',
  l25InferentialAntecedentTodo:
    'Antecedent placeholder (X): articulate the necessary conditions enabling the appearance of absolute knowing',
});

// L25 — Genesis of Appearance
const CHUNKS_YS_IV_34_L25_GENESIS = [
  {
    id: 'ys-iv-34-l25-genesis',
    title: 'L25 — Genesis of Appearance (Particular Knowing Returns)',
    summary:
      'Explain the genesis of absolute knowing’s appearance in us; science of knowing re-emerges as a particular knowing (in specie).',
  },
] as const;

const HLOS_YS_IV_34_L25_GENESIS = [
  {
    id: 'ys-iv-34-hlo-l25-genesis',
    chunkId: 'ys-iv-34-l25-genesis',
    label: 'Genesis',
    clauses: [
      'define(l25ScienceEmergesAsParticular)',
      'define(l25WeAscendedMustExplainGenesis)',
      'define(l25GenesisOfAppearanceNotOrigin)',
      'define(l25ScienceInSpecie)',
    ],
  },
] as const;

// L25 — Ordinary Knowing as Condition
const CHUNKS_YS_IV_34_L25_ORDINARY_COND = [
  {
    id: 'ys-iv-34-l25-ordinary-cond',
    title: 'L25 — Ordinary Knowing as Primordial Condition',
    summary:
      'Ordinary knowing may condition the genetic possibility of absolute knowing; determinations explained from the presupposition that science ought to arise.',
  },
] as const;

const HLOS_YS_IV_34_L25_ORDINARY_COND = [
  {
    id: 'ys-iv-34-hlo-l25-ordinary-cond',
    chunkId: 'ys-iv-34-l25-ordinary-cond',
    label: 'Ordinary Knowing',
    clauses: [
      'define(l25OrdinaryKnowingPrimordialCondition)',
      'assert(l25ExplainDeterminationsFromPresupposition)',
    ],
  },
] as const;

// L25 — Rational Inference Schema
const CHUNKS_YS_IV_34_L25_INFERENCE = [
  {
    id: 'ys-iv-34-l25-inference',
    title: 'L25 — Rational Inference Schema',
    summary:
      'If absolute knowing is to appear, then X must…; given knowing’s determinations, therefore it should happen.',
  },
] as const;

const HLOS_YS_IV_34_L25_INFERENCE = [
  {
    id: 'ys-iv-34-hlo-l25-inference',
    chunkId: 'ys-iv-34-l25-inference',
    label: 'Inference',
    clauses: [
      'define(l25RationalInferenceSchema)',
      'todo(l25InferentialAntecedentTodo)',
    ],
  },
] as const;

// L25 — Opening (Condition and Law)
const CHUNKS_YS_IV_34_L25_OPENING = [
  {
    id: 'ys-iv-34-l25-opening',
    title: 'L25 — Opening (Condition and Law)',
    summary:
      'If knowing predicates → it must project; form contains a free fact and an absolute law; we are under this law; knowing = objective oneness by invisible law.',
  },
] as const;

const HLOS_YS_IV_34_L25_OPENING = [
  {
    id: 'ys-iv-34-hlo-l25-opening',
    chunkId: 'ys-iv-34-l25-opening',
    label: 'Condition/Law',
    clauses: [
      'define(l25PredicationCondition)',
      'define(l25FactAndLawForm)',
      'define(l25ScientistsUnderLaw)',
      'conclude(l25ObjectiveUnchangeableOneness)',
    ],
  },
] as const;

// L25 — Immanent Projection → Infinite Reconstruction (+ objection)
const CHUNKS_YS_IV_34_L25_IMMANENCE = [
  {
    id: 'ys-iv-34-l25-immanence',
    title: 'L25 — Immanent Projection and Reconstruction',
    summary:
      'Absolute-law projection; light bears principle-providing mark; infinite reconstruction; fundamental disjunction; objection about the law as reconstruction.',
  },
] as const;

const HLOS_YS_IV_34_L25_IMMANENCE = [
  {
    id: 'ys-iv-34-hlo-l25-immanence',
    chunkId: 'ys-iv-34-l25-immanence',
    label: 'Immanence/Reconstruction',
    clauses: [
      'define(l25ProjectionByAbsoluteLaw)',
      'assert(l25AbsolutelyImmanentProjection)',
      'define(l25LightDiffersInProjection)',
      'note(l25ProductOfPrincipleProviding)',
      'assert(l25InfiniteReconstructionSeries)',
      'conclude(l25FundamentalDisjunctionKnowing)',
      'note(l25ObjectionLawReconstructed)',
      'warn(l25StillArbitrarinessNotNecessary)',
    ],
  },
] as const;

// L25 — (1) Trust vs Primordial Construction
const CHUNKS_YS_IV_34_L25_TRUST = [
  {
    id: 'ys-iv-34-l25-trust',
    title: 'L25 — Trusting Reconstruction vs Primordial Construction',
    summary:
      'Projected law can be reconstruction; inner construction is primordial; necessity = inner expression/causality of law; “posit the law.”',
  },
] as const;

const HLOS_YS_IV_34_L25_TRUST = [
  {
    id: 'ys-iv-34-hlo-l25-trust',
    chunkId: 'ys-iv-34-l25-trust',
    label: 'Trust/Primordial',
    clauses: [
      'define(l25TrustReconstructionLawIssue)',
      'define(l25IfNecessaryThenConformsInnerCausality)',
      'define(l25ImmediateFacticalLawPosit)',
      'note(l25ProjectedLawMayBeReconstruction)',
      'conclude(l25InnerConstructionPrimordial)',
    ],
  },
] as const;

// L25 — (2) Image as Image (nervus probandi)
const CHUNKS_YS_IV_34_L25_IMAGE_NERVUS = [
  {
    id: 'ys-iv-34-l25-image-nervus',
    title: 'L25 — Image as Image (Nervus Probandi)',
    summary:
      'Primordial projection as image; image→content, reconstruction→original; image presupposes higher law; law virtually present posits itself in us.',
  },
] as const;

const HLOS_YS_IV_34_L25_IMAGE_NERVUS = [
  {
    id: 'ys-iv-34-hlo-l25-image-nervus',
    chunkId: 'ys-iv-34-l25-image-nervus',
    label: 'Image/Law Proof',
    clauses: [
      'define(l25PrimordialProjectionImageCharacter)',
      'define(l25ImageRefersToContent)',
      'define(l25TaskPositPriorOriginal)',
      'define(l25HigherLawPresupposedInImage)',
      'assert(l25LawVirtuallyPresentInImage)',
      'assert(l25WeStandInImageAsImage)',
      'conclude(l25LawPositsItselfInUs)',
      'assert(l25ImageAsImageNervus)',
    ],
  },
] as const;

// L25 — Presupposed Law; Form Variation Deferred
const CHUNKS_YS_IV_34_L25_PROOF_FORM = [
  {
    id: 'ys-iv-34-l25-proof-form',
    title: 'L25 — Presupposed Law; Form Variation Deferred',
    summary:
      'Presuppose law as image-ground; arrival at concept explained; variation of form deferred to the “possibility of science of knowing.”',
  },
] as const;

const HLOS_YS_IV_34_L25_PROOF_FORM = [
  {
    id: 'ys-iv-34-hlo-l25-proof-form',
    chunkId: 'ys-iv-34-l25-proof-form',
    label: 'Proof/Form',
    clauses: [
      'define(l25PresupposeLawAsPrimordialGround)',
      'define(l25ArrivalAtConceptExplained)',
      'note(l25FormVariationDeferredPossibility)',
    ],
  },
] as const;

// L25 — Standpoint Between (Image/Law Split)
const CHUNKS_YS_IV_34_L25_STANDPOINT = [
  {
    id: 'ys-iv-34-l25-standpoint',
    title: 'L25 — Standpoint Between (Image/Law Split)',
    summary:
      'Knowing stands between original and reconstruction: image as image where law arises inwardly; divides into objective image and objective law.',
  },
] as const;

const HLOS_YS_IV_34_L25_STANDPOINT = [
  {
    id: 'ys-iv-34-hlo-l25-standpoint',
    chunkId: 'ys-iv-34-l25-standpoint',
    label: 'Standpoint',
    clauses: [
      'define(l25KnowingStandpointBetween)',
      'define(l25ImageOfReconstructionAsImage)',
      'assert(l25PermeationPrimordialOneness)',
      'define(l25ProjectsIntoImageAndLaw)',
      'note(l25PermanentObjectiveImage)',
      'note(l25PermanentObjectiveLaw)',
    ],
  },
] as const;

// L25 — Qualitative oneness removed; image invariant; mutual positing
const CHUNKS_YS_IV_34_L25_QUAL_ONENESS = [
  {
    id: 'ys-iv-34-l25-qual-oneness',
    title: 'L25 — Qualitative Oneness Removed; Image Invariant',
    summary:
      'Image and copy cohere as image positing law; qualitative oneness negates variation; image and its law are intrinsically invariant and mutually posit by essence.',
  },
] as const;

const HLOS_YS_IV_34_L25_QUAL_ONENESS = [
  {
    id: 'ys-iv-34-hlo-l25-qual-oneness',
    chunkId: 'ys-iv-34-l25-qual-oneness',
    label: 'Qualitative Oneness',
    clauses: [
      'define(l25PrimordialImageCopyQualOne)',
      'define(l25CoherenceAsImagePositingLaw)',
      'define(l25QualOnenessNegVariation)',
      'assert(l25ImageIntrinsicInvariance)',
      'assert(l25MutualPositingByEssence)',
      'note(l25RemovalQualOnenessGuaranteeHigher)',
    ],
  },
] as const;

// L25 — Absolute knowing as image-making process; synthetic cycle
const CHUNKS_YS_IV_34_L25_SYNTHETIC_CYCLE = [
  {
    id: 'ys-iv-34-l25-synthetic-cycle',
    title: 'L25 — Absolute Knowing: Synthetic Cycle',
    summary:
      'Absolute knowing = image-making process positing itself as image and positing its own law; closure as a synthetic cycle.',
  },
] as const;

const HLOS_YS_IV_34_L25_SYNTHETIC_CYCLE = [
  {
    id: 'ys-iv-34-hlo-l25-synthetic-cycle',
    chunkId: 'ys-iv-34-l25-synthetic-cycle',
    label: 'Synthetic Cycle',
    clauses: [
      'define(l25AbsoluteKnowingHere)',
      'define(l25ImageMakingPositsItself)',
      'define(l25PositsLawOfImageMaking)',
      'conclude(l25SyntheticCycleClosure)',
      'link(l25ImagePositsLaw → l25PositsLawOfImageMaking)',
    ],
  },
] as const;

// L25 — Qualitative Oneness Negated → Quantifiability
const CHUNKS_YS_IV_34_L25_QUALITY_NEGATION = [
  {
    id: 'ys-iv-34-l25-quality-negation',
    title: 'L25 — Negating Qualitative Oneness → Quantifiability',
    summary:
      'Negate qualitative oneness; establish quantifiability and descent to multiplicity; oneness of image-law (hypothetical → manifest); re-introduce quality only via necessary relation.',
  },
] as const;

const HLOS_YS_IV_34_L25_QUALITY_NEGATION = [
  {
    id: 'ys-iv-34-hlo-l25-quality-negation',
    chunkId: 'ys-iv-34-l25-quality-negation',
    label: 'Quality→Quantity',
    clauses: [
      'define(l25NegationQualOnenessClosesProgress)',
      'assert(l25QuantifiabilityEstablished)',
      'define(l25OnenessImageLawRepelsVariability)',
      'note(l25OnenessHypotheticalThenManifest)',
      'conclude(l25BlendQualityViaNecessaryRelation)',
      'warn(l25OccultQualityCutOff)',
      'link(l25ImageIntrinsicInvariance → l25OnenessImageLawRepelsVariability)',
      'link(l25MutualPositingByEssence → l25OnenessImageLawRepelsVariability)',
    ],
  },
] as const;

// L25 — Absolute Knowing Identity (Science ↔ Knowing)
const CHUNKS_YS_IV_34_L25_ABSOLUTE_IDENTITY = [
  {
    id: 'ys-iv-34-l25-absolute-identity',
    title: 'L25 — Absolute Knowing: Identity with the Science of Knowing',
    summary:
      'Particular “science of knowing” disappears; absolute knowing stands; identity: science of knowing = absolute knowing; we are it insofar as it is.',
  },
] as const;

const HLOS_YS_IV_34_L25_ABSOLUTE_IDENTITY = [
  {
    id: 'ys-iv-34-hlo-l25-absolute-identity',
    chunkId: 'ys-iv-34-l25-absolute-identity',
    label: 'Absolute Identity',
    clauses: [
      'define(l25ConceptScienceAsParticularGone)',
      'define(l25OnePureKnowingAbsoluteDisjunction)',
      'assert(l25WeAreThisOnePureKnowing)',
      'conclude(l25ScienceIsAbsoluteKnowingIdentity)',
      'note(l25WeAreAbsoluteKnowingIffScienceIs)',
      'link(absoluteEssenceClosure → l25ScienceIsAbsoluteKnowingIdentity)',
    ],
  },
] as const;

const CHUNKS_YS_IV_34 = [
  {
    id: 'ys-iv-34-text',
    title: 'IV.34 — Text',
    summary: 'Sūtra with gloss and key terms.',
  },
  {
    id: 'ys-iv-34-meaning',
    title: 'Meaning',
    summary: 'Return of guṇas; Seer’s independence; two perspectives (vā).',
  },
  {
    id: 'ys-iv-34-closure',
    title: 'Closure',
    summary:
      'Absolute Essence → Kaivalya; bridge from IV.33 absolute relation.',
  },
] as const;

const HLOS_YS_IV_34 = [
  {
    id: 'ys-iv-34-hlo-text',
    chunkId: 'ys-iv-34-text',
    label: 'Text',
    clauses: [
      'define(sutraDevanagari)',
      'define(sutraIAST)',
      'note(sutraGloss)',
      'define(purusharthaShunyana)',
      'define(gunanamPratiprasava)',
      'define(kaivalyam)',
      'define(svarupaPratistha)',
      'define(citiShakti)',
      'note(vaReadingGuard)',
    ],
  },
  {
    id: 'ys-iv-34-hlo-meaning',
    chunkId: 'ys-iv-34-meaning',
    label: 'Meaning',
    clauses: [
      'assert(absoluteEssenceClosure)',
      'conclude(kaivalyam)',
      'note(svarupaPratistha)',
      'note(citiShakti)',
    ],
  },
  {
    id: 'ys-iv-34-hlo-closure',
    chunkId: 'ys-iv-34-closure',
    label: 'Closure',
    clauses: [
      'link(absoluteNecessity → absoluteEssenceClosure)',
      'link(absoluteRelation → absoluteEssenceClosure)',
      'define(bridgeFromIv33)',
    ],
  },
] as const;

// L25 — Unconditional “should”, descent via life, specie-standpoint, teleology
Object.assign(YS_IV_34_ONTOLOGY, {
  l25UnconditionalShouldPrinciple:
    'All determinations must be explicable on the presupposition: “It should unconditionally ___ …”',
  l25KnowingUnconditionallyOneNoQualQuant:
    'Knowing in itself is unconditionally one, without any material quality or quantity',
  l25FormsOfQuantityTimeSpace:
    'Forms of quantity: time, space, etc. — the infinity of quantitative determination',
  l25QuestionDescentQualityQuantity:
    'How does knowing descend to qualitative multiplicity and to the infinity of quantity and its forms?',

  l25DescentBecauseGeneticOnly:
    'Descent occurs because absolute knowing’s being can be produced only genetically',
  l25GeneticConditionsFromLife:
    'This genetic production is only under the types of conditions found originally in living',
  l25LifeCohesionWithScienceAndProducts:
    'Therefore life coheres indivisibly with the science of knowing and with what it produces',
  l25ExistentialClaimValueElevation:
    'Apart from elevation to absolute knowing, life would be nothing, without worth or meaning, and would not truly exist',

  l25AbsoluteGenesisAffirmationUnitesEnds:
    'Absolute affirmation of the genesis of the existence of absolute knowing unites ordinary and absolute knowing and clarifies them reciprocally',
  l25StandpointScienceInSpecieGenuine:
    'This point is the genuine standpoint of the science of knowing in specie (as particular knowing)',

  l25OutcomeTeleologyAbsolutePurpose:
    'Simple existence (up to the existence of absolute knowing) has its ground not in itself but in an absolute purpose',
  l25PurposeThatAbsoluteKnowingShouldBe:
    'The absolute purpose is that absolute knowing should be',
  l25AllPositedByPurpose:
    'Everything is posited and determined through this purpose and reaches its destination only in attaining it',
  l25ValueOnlyInAbsoluteKnowing:
    'Value exists only in knowing — indeed in absolute knowing; all else is without value',
  l25ScienceInSpecieInstrumentalOnly:
    'The science of knowing in specie is instrumental only (not intrinsic value)',
  l25LadderMetaphor: 'Whoever has arrived no longer worries about the ladder',
});

// L25 — Unconditional “Should” and Descent Question
const CHUNKS_YS_IV_34_L25_UNCONDITIONAL = [
  {
    id: 'ys-iv-34-l25-unconditional',
    title: 'L25 — Unconditional “Should” and the Descent Question',
    summary:
      'All determinations from “It should unconditionally …”; knowing one in itself; how descent to quality and quantity (time/space) is possible.',
  },
] as const;

const HLOS_YS_IV_34_L25_UNCONDITIONAL = [
  {
    id: 'ys-iv-34-hlo-l25-unconditional',
    chunkId: 'ys-iv-34-l25-unconditional',
    label: 'Unconditional/Descent',
    clauses: [
      'define(l25UnconditionalShouldPrinciple)',
      'define(l25KnowingUnconditionallyOneNoQualQuant)',
      'note(l25FormsOfQuantityTimeSpace)',
      'define(l25QuestionDescentQualityQuantity)',
    ],
  },
] as const;

// L25 — Descent via Life (Genetic Production)
const CHUNKS_YS_IV_34_L25_DESCENT = [
  {
    id: 'ys-iv-34-l25-descent',
    title: 'L25 — Descent via Life (Genetic Production)',
    summary:
      'Being of absolute knowing is produced only genetically under life’s original conditions; thus life coheres with science and its products.',
  },
] as const;

const HLOS_YS_IV_34_L25_DESCENT = [
  {
    id: 'ys-iv-34-hlo-l25-descent',
    chunkId: 'ys-iv-34-l25-descent',
    label: 'Descent/Life',
    clauses: [
      'assert(l25DescentBecauseGeneticOnly)',
      'define(l25GeneticConditionsFromLife)',
      'conclude(l25LifeCohesionWithScienceAndProducts)',
      'warn(l25ExistentialClaimValueElevation)',
    ],
  },
] as const;

// L25 — Standpoint of Science in Specie
const CHUNKS_YS_IV_34_L25_SPECIE = [
  {
    id: 'ys-iv-34-l25-specie',
    title: 'L25 — Standpoint: Science of Knowing in Specie',
    summary:
      'Affirm genesis of existence of absolute knowing; unite ordinary and absolute; genuine standpoint of science in specie.',
  },
] as const;

const HLOS_YS_IV_34_L25_SPECIE = [
  {
    id: 'ys-iv-34-hlo-l25-specie',
    chunkId: 'ys-iv-34-l25-specie',
    label: 'Specie',
    clauses: [
      'define(l25AbsoluteGenesisAffirmationUnitesEnds)',
      'conclude(l25StandpointScienceInSpecieGenuine)',
    ],
  },
] as const;

// L25 — Teleological Outcome and Value
const CHUNKS_YS_IV_34_L25_TELEOLOGY = [
  {
    id: 'ys-iv-34-l25-teleology',
    title: 'L25 — Teleological Outcome and Value',
    summary:
      'Ground of existence is absolute purpose: that absolute knowing should be; value only in absolute knowing; science-in-specie is instrumental (ladder).',
  },
] as const;

const HLOS_YS_IV_34_L25_TELEOLOGY = [
  {
    id: 'ys-iv-34-hlo-l25-teleology',
    chunkId: 'ys-iv-34-l25-teleology',
    label: 'Teleology/Value',
    clauses: [
      'define(l25OutcomeTeleologyAbsolutePurpose)',
      'define(l25PurposeThatAbsoluteKnowingShouldBe)',
      'assert(l25AllPositedByPurpose)',
      'conclude(l25ValueOnlyInAbsoluteKnowing)',
      'note(l25ScienceInSpecieInstrumentalOnly)',
      'note(l25LadderMetaphor)',
    ],
  },
] as const;

export const YS_IV_34_UNIT: DatasetUnit = {
  id: makeUnitId('ys-iv-34'),
  title: 'YS IV.34 — Kaivalya (Return of the Guṇas; Seer’s Independence)',
  scope: 'actuality',
  logosMode: 'appearance',
  synthesis:
    'Final closure: guṇas return to source (no purpose for Puruṣa); or establishment in one’s own nature — citi-śakti.',
  faculty: 'buddhi',
  lens: 'yoga',
  chunks: [
    ...CHUNKS_YS_IV_34,
    ...CHUNKS_YS_IV_34_L25_OPENING,
    ...CHUNKS_YS_IV_34_L25_IMMANENCE,
    ...CHUNKS_YS_IV_34_L25_TRUST,
    ...CHUNKS_YS_IV_34_L25_IMAGE_NERVUS,
    ...CHUNKS_YS_IV_34_L25_PROOF_FORM,
    ...CHUNKS_YS_IV_34_L25_STANDPOINT,
    ...CHUNKS_YS_IV_34_L25_QUALITY_NEGATION,
    ...CHUNKS_YS_IV_34_L25_ABSOLUTE_IDENTITY,
    ...CHUNKS_YS_IV_34_L25_GENESIS,
    ...CHUNKS_YS_IV_34_L25_ORDINARY_COND,
    ...CHUNKS_YS_IV_34_L25_INFERENCE,
    ...CHUNKS_YS_IV_34_L25_UNCONDITIONAL,
    ...CHUNKS_YS_IV_34_L25_DESCENT,
    ...CHUNKS_YS_IV_34_L25_SPECIE,
    ...CHUNKS_YS_IV_34_L25_TELEOLOGY,
  ] as any,
  hlos: [
    ...HLOS_YS_IV_34,
    ...HLOS_YS_IV_34_L25_OPENING,
    ...HLOS_YS_IV_34_L25_IMMANENCE,
    ...HLOS_YS_IV_34_L25_TRUST,
    ...HLOS_YS_IV_34_L25_IMAGE_NERVUS,
    ...HLOS_YS_IV_34_L25_PROOF_FORM,
    ...HLOS_YS_IV_34_L25_STANDPOINT,
    ...HLOS_YS_IV_34_L25_QUALITY_NEGATION,
    ...HLOS_YS_IV_34_L25_ABSOLUTE_IDENTITY,
    ...HLOS_YS_IV_34_L25_GENESIS,
    ...HLOS_YS_IV_34_L25_ORDINARY_COND,
    ...HLOS_YS_IV_34_L25_INFERENCE,
    ...HLOS_YS_IV_34_L25_UNCONDITIONAL,
    ...HLOS_YS_IV_34_L25_DESCENT,
    ...HLOS_YS_IV_34_L25_SPECIE,
    ...HLOS_YS_IV_34_L25_TELEOLOGY,
  ] as any,
};

export const YS_IV_34_SYMBOLS = Object.keys(YS_IV_34_ONTOLOGY);
