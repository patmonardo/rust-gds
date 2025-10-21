import { DatasetUnit, makeUnitId } from '@organon/gdsl/registry/canon'

// YS IV.33 — kṣaṇa-pratiyogī pariṇāma-aparānta-nirgrāhyaḥ kramaḥ
export const YS_IV_33_ONTOLOGY = {
  sutraDevanagari: 'क्षणप्रतियोगी परिणामापरान्तनिर्ग्राह्यः क्रमः',
  sutraIAST: 'kṣaṇa-pratiyogī pariṇāma-aparānta-nirgrāhyaḥ kramaḥ',
  sutraGloss:
    'Sequence is that which is graspable at the terminal limit of transformation, having instants as its counterparts.',
  // Morphology beacons
  ksanaPratiyogi:
    'kṣaṇa-pratiyogī: having the moment/instant as counterpart',
  parinamaAparanta:
    'pariṇāma-aparānta: at the end/limit of transformation',
  nirgrahya:
    'nirgrāhyaḥ: graspable/apprehensible',
  krama:
    'kramaḥ: sequence/order',
  // Absolute necessity / relation
  absoluteNecessity:
    'Absolute necessity: law of lawfulness itself (dharma of dharma-megha), no further external ground',
  dharmaOfDharmaMegha:
    'Dharma of dharma-megha: meta-law that governs all appearances once relative necessity is complete',
  abhidharmaOfSarvadharmas:
    'Abhidharma as dharma of sarva-dharmas: meta-law across all dharmas',
  absoluteRelation:
    'Absolute relation: Idea-level reflection in act (not mediated category-relation)',
  // Bridges
  fromRelativeToAbsolute:
    'From IV.32 (completed relative necessity) to IV.33 (absolute necessity via limit of transformation)',
  gapToConceptImage:
    'Fichte: absolute gap yields concept as image of essence; here, limit-grasping fixes necessity',
} as const

const CHUNKS_YS_IV_33 = [
  { id: 'ys-iv-33-text', title: 'IV.33 — Text', summary: 'Sūtra and literal gloss with morphology.' },
  { id: 'ys-iv-33-meaning', title: 'Meaning', summary: 'Absolute necessity via limit-grasp of sequence.' },
] as const

const HLOS_YS_IV_33 = [
  {
    id: 'ys-iv-33-hlo-text',
    chunkId: 'ys-iv-33-text',
    label: 'Text',
    clauses: [
      'define(sutraDevanagari)',
      'define(sutraIAST)',
      'note(sutraGloss)',
      'define(ksanaPratiyogi)',
      'define(parinamaAparanta)',
      'define(nirgrahya)',
      'define(krama)',
    ],
  },
  {
    id: 'ys-iv-33-hlo-meaning',
    chunkId: 'ys-iv-33-meaning',
    label: 'Meaning',
    clauses: [
      'assert(fromRelativeToAbsolute)',
      'define(absoluteNecessity)',
      'define(dharmaOfDharmaMegha)',
      'note(abhidharmaOfSarvadharmas)',
      'define(absoluteRelation)',
      'note(gapToConceptImage)',
    ],
  },
] as const

// Essence — Absolute Necessity (HLO)
const CHUNKS_YS_IV_33_ESSENCE = [
  {
    id: 'ys-iv-33-essence',
    title: 'HLO Essence — Absolute Necessity',
    summary:
      'Sequence is grasped at the transformation limit (moment-counterpart); meta-law (dharma of dharma-megha); absolute relation.',
  },
] as const

const HLOS_YS_IV_33_ESSENCE = [
  {
    id: 'ys-iv-33-hlo-essence',
    chunkId: 'ys-iv-33-essence',
    label: 'Essence',
    clauses: [
      'define(absoluteNecessity)',
      'assert(absoluteRelation)',
      'conclude(dharmaOfDharmaMegha)',
    ],
  },
] as const

// L24 — Comprehension under conditions; predicate schema; logical clarification
Object.assign(YS_IV_33_ONTOLOGY, {
  l24PrimordialActClaim:
    'Claim: light’s absolutely primordial act = making itself an intuition (by science of knowing, not ordinary knowing)',
  l24SelfProducingInsight:
    'This must be demonstrated in an immediately self-producing insight/manifestness',
  l24AbsNecComprehendedUnderConditions:
    'Absolute necessity is comprehended — but under conditions (not unconditionally)',
  l24PredicateConditionalSchema:
    'If predication (i.e., intuition) occurs, then projection through the gap must occur (formal objectification)',
  l24AntecedentQuestion:
    'Question: should the antecedent (predication/intuition) be? If not, neither is the consequent',
  l24ConditionedIntellectual:
    'Conditioned intellectually: the first (insight) seen only under a condition',
  l24ConditionedReal:
    'Conditioned really: if the consequent happens, it is real conditionedness',
  l24NotYetUnconditionedOneness:
    'Thus we have what we wished, but not yet the unconditionedness and oneness we strove for',

  // Logical clarification (a)
  l24LogicalPredicateMinor:
    'Logical clarification: Predicate = minor premise',
  l24LogicalMajorSubjectObjectification:
    'Absolute objectification of the logical subject = major premise',
  l24ReciprocalPositing:
    'Minor and major posit themselves unconditionally reciprocally',
  l24DeeperUnderlyingThanMajor:
    'A deeper ground underlies the inference more than the major premise (the true absolute major)',
  l24FailureWithoutAbsoluteMajor:
    'Ignoring this, systems fail to arrive at an absolute major premise',
  l24RootlessSkepticism:
    'Without this, to avoid arbitrariness, one sinks into rootless skepticism',

  // Higher relatedness (b)
  l24HigherRelatedTerms:
    'The related terms now are higher than the previous ones (we are not backsliding)',
  l24PrevSelfConstructionInEstablishedIntuition:
    'Previously: self-construction in already established intuition',
  l24BeingOfKnowingAsEstablishedIntuition:
    'Previously: “being of knowing” taken as the highest established intuition',
  l24EstablishedIntuitionDiscoveredInSelfProjection:
    'Now: established intuition is discovered in pure, real self-projection (not displaced by a higher one)',
})

const CHUNKS_YS_IV_33_L24_CONDITIONS = [
  {
    id: 'ys-iv-33-l24-conditions',
    title: 'L24 — Comprehended Under Conditions',
    summary:
      'Absolute necessity grasped conditionally; predication schema: if intuition, then projection-through-gap; intellectual vs real conditionedness.',
  },
] as const

const HLOS_YS_IV_33_L24_CONDITIONS = [
  {
    id: 'ys-iv-33-hlo-l24-conditions',
    chunkId: 'ys-iv-33-l24-conditions',
    label: 'Conditions',
    clauses: [
      'define(l24PrimordialActClaim)',
      'define(l24SelfProducingInsight)',
      'assert(l24AbsNecComprehendedUnderConditions)',
      'define(l24PredicateConditionalSchema)',
      'define(l24AntecedentQuestion)',
      'note(l24ConditionedIntellectual)',
      'note(l24ConditionedReal)',
      'note(l24NotYetUnconditionedOneness)',
      'link(formalPredicateThesis → l24PredicateConditionalSchema)',
      'link(predicatingLightMeansProjection → l24PredicateConditionalSchema)',
    ],
  },
] as const

const CHUNKS_YS_IV_33_L24_LOGIC = [
  {
    id: 'ys-iv-33-l24-logic',
    title: 'L24 — Logical Clarification',
    summary:
      'Predicate=minor; objectified subject=major; reciprocal positing; deeper absolute major or skepticism.',
  },
] as const

const HLOS_YS_IV_33_L24_LOGIC = [
  {
    id: 'ys-iv-33-hlo-l24-logic',
    chunkId: 'ys-iv-33-l24-logic',
    label: 'Logic',
    clauses: [
      'define(l24LogicalPredicateMinor)',
      'define(l24LogicalMajorSubjectObjectification)',
      'assert(l24ReciprocalPositing)',
      'define(l24DeeperUnderlyingThanMajor)',
      'assert(l24FailureWithoutAbsoluteMajor)',
      'warn(l24RootlessSkepticism)',
      'note(l24HigherRelatedTerms)',
      'note(l24PrevSelfConstructionInEstablishedIntuition)',
      'note(l24BeingOfKnowingAsEstablishedIntuition)',
      'conclude(l24EstablishedIntuitionDiscoveredInSelfProjection)',
    ],
  },
] as const

// L24 — Oneness-through-itself; Law ↔ Fact reciprocity
Object.assign(YS_IV_33_ONTOLOGY, {
  l24NecessaryUnchangeableRelation:
    'Scientist of knowing intuits an absolutely necessary, unchangeable relation and projects knowing as this relation',
  l24OnenessThroughItself:
    'Oneness through itself: self-determined relation with no assistance from any external factor',
  l24PermeateConstructEssence:
    'Relation is simultaneously permeated and constructed in its inner essence and content',
  l24ArbitraryVsUnconditional:
    'Content pairs: the arbitrary (freedom/facticity) and the unconditionally necessary (lawfulness)',
  l24FacticityGraspsDetermines:
    'Facticity, once called into life, grasps and determines without further ado',
  l24MutualPrincipiation:
    'Each is the proper principle of its being only in the same undivided stroke as it has its principle in the other (mutual co-implication)',
  l24LawDefinition:
    'Law: a principle that, to provide a principle factically, presupposes an absolutely self-producing principle',
  l24FactDefinition:
    'Fact: a pure, primordial fact that is only possible according to a law',
  l24SelfProducingPrinciple:
    'Absolutely self-producing principle underlies the reciprocity (no higher external ground)',
})

const CHUNKS_YS_IV_33_L24_OPENING = [
  {
    id: 'ys-iv-33-l24-opening',
    title: 'L24 Opening — Authorization and Fall of Arbitrariness',
    summary:
      'Revisit principle-providing light; ask for authorization; prior proof falls away; higher premises; locus where acting (I/light) and arbitrariness fall away.',
  },
] as const;

const HLOS_YS_IV_33_L24_OPENING = [
  {
    id: 'ys-iv-33-hlo-l24-opening',
    chunkId: 'ys-iv-33-l24-opening',
    label: 'Authorization',
    clauses: [
      'note(l24PrimordialLightPrincipleProvidingRecap)',
      'note(l24ThreeDeterminationsRecap)',
      'define(l24AuthorizationQuestion)',
      'assert(l24PriorProofModeFallsAway)',
      'define(l24HigherPremisesRequired)',
      'define(l24LocusActingFallsAway)',
      'conclude(l24ArbitrarinessFallsAway)',
      'link(fromRelativeToAbsolute → l24HigherPremisesRequired)',
      'link(absoluteNecessity → l24ArbitrarinessFallsAway)',
    ],
  },
] as const;

const CHUNKS_YS_IV_33_L24_LAW_FACT = [
  {
    id: 'ys-iv-33-l24-law-fact',
    title: 'L24 — Oneness Through Itself; Law and Fact',
    summary:
      'Self-determined relation; arbitrary vs unconditional; mutual principiation; Law presupposes self-producing principle; Fact only per Law.',
  },
] as const

const HLOS_YS_IV_33_L24_LAW_FACT = [
  {
    id: 'ys-iv-33-hlo-l24-law-fact',
    chunkId: 'ys-iv-33-l24-law-fact',
    label: 'Law↔Fact',
    clauses: [
      'define(l24NecessaryUnchangeableRelation)',
      'define(l24OnenessThroughItself)',
      'note(l24PermeateConstructEssence)',
      'define(l24ArbitraryVsUnconditional)',
      'assert(l24FacticityGraspsDetermines)',
      'assert(l24MutualPrincipiation)',
      'define(l24LawDefinition)',
      'define(l24FactDefinition)',
      'define(l24SelfProducingPrinciple)',
      'link(absoluteNecessity → l24OnenessThroughItself)',
      'link(absoluteRelation → l24MutualPrincipiation)',
    ],
  },
] as const

// L24 §4 — Law real causality; projection (formal/material); description as image/ideal element
Object.assign(YS_IV_33_ONTOLOGY, {
  l24AbsoluteLawRealCausality:
    'Absolute law has real causality on the inwardness of the act; law and act permeate without gap',
  l24ProjectionFormalMaterial:
    'Projection is at once formal (objectifying) and material (expressing essence of knowing)',
  l24MaterialMustExpressForm:
    'Material expression must simultaneously express projection\'s form (one stroke by effective law)',
  l24NotJustInItselfWithoutProjection:
    'Knowing in projection cannot simply be what it is in itself by law without projection',
  l24InnerEssenceIsPrincipleProviding:
    'Inner essence of projection is living principle-providing that cannot be destroyed',
  l24AbsoluteDescriptionBetweenTerms:
    'Absolute description (as description) steps between the two terms',
  l24RelationAsDescription:
    'Relation = describing one on the basis of the other',
  l24PerpetualRenewalSameContent:
    'Living principle-providing is perpetually renewed while content determined by absolute law remains the same',
  l24EnergeticReflectionInfiniteRepetition:
    'Appearance of energetic reflection: infinite repetition of qualitatively one and the same content',
  l24DescriptionAsReconstruction:
    'Description appears as re-construction of an original pre-construction',
  l24ConceptAsImageStatement:
    'As image: mere statement/enunciation of what should be so by itself',
  l24IdealElementSeeingInReflection:
    'The whole ideal element: our seeing in the standpoint of reflection of principle-providing',
})

const CHUNKS_YS_IV_33_L24_LAW_ACT_PROJECTION = [
  {
    id: 'ys-iv-33-l24-law-act-projection',
    title: 'L24 — Law, Act, and Projection (Formal/Material)',
    summary:
      'Law with real causality; projection formal+material; description enters between terms; relation as description; image/ideal element.',
  },
] as const

const HLOS_YS_IV_33_L24_LAW_ACT_PROJECTION = [
  {
    id: 'ys-iv-33-hlo-l24-law-act-projection',
    chunkId: 'ys-iv-33-l24-law-act-projection',
    label: 'Law/Act/Projection',
    clauses: [
      'define(l24AbsoluteLawRealCausality)',
      'define(l24ProjectionFormalMaterial)',
      'assert(l24MaterialMustExpressForm)',
      'note(l24NotJustInItselfWithoutProjection)',
      'define(l24InnerEssenceIsPrincipleProviding)',
      'define(l24AbsoluteDescriptionBetweenTerms)',
      'define(l24RelationAsDescription)',
      'assert(l24PerpetualRenewalSameContent)',
      'note(l24EnergeticReflectionInfiniteRepetition)',
      'define(l24DescriptionAsReconstruction)',
      'define(l24ConceptAsImageStatement)',
      'conclude(l24IdealElementSeeingInReflection)',
      'link(absoluteNecessity → l24AbsoluteLawRealCausality)',
      'link(formalPredicateThesis → l24MaterialMustExpressForm)',
      'link(predicatingLightMeansProjection → l24ProjectionFormalMaterial)',
      'link(absoluteRelation → l24RelationAsDescription)',
    ],
  },
] as const

// L24 — Objectivity has no truth; knowing's duality (primordial ↔ reconstruction)
Object.assign(YS_IV_33_ONTOLOGY, {
  l24ObjectivityNoTruthRelation:
    'The entire form of objectivity (form of existence) has in itself no relation to truth',
  l24KnowingDualityPrimordialReconstruction:
    'Knowing splits absolutely into a duality: (1) the primordial and (2) the reconstruction of the primordial',
  l24NoDiversityOfContent:
    'Both terms have no diversity of content; they are absolutely one in content',
  l24DifferenceOnlyInForm:
    'They differ only in the given form, indicating a reciprocal relation',
  l24ObjectRepresentationTest:
    'Every consciousness shows this: object ↔ representation (test of the proposition)',
})

const CHUNKS_YS_IV_33_L24_OBJECTIVITY_DUALITY = [
  {
    id: 'ys-iv-33-l24-objectivity-duality',
    title: 'L24 — Objectivity and Knowing\'s Duality',
    summary:
      'Objectivity lacks truth; knowing splits into primordial and reconstruction; one in content, different in form; reciprocal relation.',
  },
] as const

const HLOS_YS_IV_33_L24_OBJECTIVITY_DUALITY = [
  {
    id: 'ys-iv-33-hlo-l24-objectivity-duality',
    chunkId: 'ys-iv-33-l24-objectivity-duality',
    label: 'Objectivity/Duality',
    clauses: [
      'define(l24ObjectivityNoTruthRelation)',
      'define(l24KnowingDualityPrimordialReconstruction)',
      'assert(l24NoDiversityOfContent)',
      'assert(l24DifferenceOnlyInForm)',
      'note(l24ObjectRepresentationTest)',
      'link(absoluteRelation → l24DifferenceOnlyInForm)',
      'link(l24DescriptionAsReconstruction → l24KnowingDualityPrimordialReconstruction)',
    ],
  },
] as const

// L24 — Ostensible primordial, inner disjunction, self-constructing law
Object.assign(YS_IV_33_ONTOLOGY, {
  l24OstensiblePrimordialIsReconstruction:
    'The ostensible primordial construction that justifies reconstruction is itself a reconstruction that does not present itself as such',
  l24AppearanceDisappearsOnInspection:
    'On closer inspection the entire appearance disappears; that standpoint is not the highest',
  l24InnerDisjunctionTwoFormsOfLife:
    'A deeper disjunction remains: not outer subject/object, but an inner living difference — two forms of life',
  l24NotSubjectObjectDisjunction:
    'The outward subject/object disjunction fell away with the annulment of persistent projection/objectivity',
  l24AbsoluteLawConstructsItself:
    'We cannot construct the absolute law; it constructs itself on us and in us',
  l24LawPositsUsAndItselfInUs:
    'It is the law itself which posits us, and itself in us',
})

// Chunk — L24 Appearance drops; ostensible primordial = reconstruction
const CHUNKS_YS_IV_33_L24_APPEARANCE = [
  {
    id: 'ys-iv-33-l24-appearance',
    title: 'L24 — Appearance Drops',
    summary:
      'Ostensible primordial is hidden reconstruction; appearance disappears; not the highest standpoint.',
  },
] as const

const HLOS_YS_IV_33_L24_APPEARANCE = [
  {
    id: 'ys-iv-33-hlo-l24-appearance',
    chunkId: 'ys-iv-33-l24-appearance',
    label: 'Appearance',
    clauses: [
      'define(l24OstensiblePrimordialIsReconstruction)',
      'conclude(l24AppearanceDisappearsOnInspection)',
      'note(l24NotSubjectObjectDisjunction)',
    ],
  },
] as const

// Chunk — L24 Inner disjunction: two forms of life
const CHUNKS_YS_IV_33_L24_TWO_LIVES = [
  {
    id: 'ys-iv-33-l24-two-lives',
    title: 'L24 — Inner Disjunction (Two Forms of Life)',
    summary:
      'Not subject/object, but inner living difference; two forms of life to be genetically grounded.',
  },
] as const

const HLOS_YS_IV_33_L24_TWO_LIVES = [
  {
    id: 'ys-iv-33-hlo-l24-two-lives',
    chunkId: 'ys-iv-33-l24-two-lives',
    label: 'Two Lives',
    clauses: [
      'define(l24InnerDisjunctionTwoFormsOfLife)',
      'assert(l24NotSubjectObjectDisjunction)',
      'link(absoluteRelation → l24InnerDisjunctionTwoFormsOfLife)',
    ],
  },
] as const

// Chunk — L24 Law constructs itself (self-positing in us)
const CHUNKS_YS_IV_33_L24_LAW_SELF = [
  {
    id: 'ys-iv-33-l24-law-self',
    title: 'L24 — Law Constructs Itself',
    summary:
      'We cannot construct the absolute law; it constructs itself on us and in us; it posits us and itself in us.',
  },
] as const

const HLOS_YS_IV_33_L24_LAW_SELF = [
  {
    id: 'ys-iv-33-hlo-l24-law-self',
    chunkId: 'ys-iv-33-l24-law-self',
    label: 'Self-Constructing Law',
    clauses: [
      'define(l24AbsoluteLawConstructsItself)',
      'conclude(l24LawPositsUsAndItselfInUs)',
      'link(absoluteNecessity → l24AbsoluteLawConstructsItself)',
    ],
  },
] as const

export const YS_IV_33_UNIT: DatasetUnit = {
  id: makeUnitId('ys-iv-33'),
  title: 'YS IV.33 — kṣaṇa-pratiyogī pariṇāma-aparānta-nirgrāhyaḥ kramaḥ',
  scope: 'actuality',
  logosMode: 'appearance',
  synthesis:
    'Absolute necessity: sequence as graspable at the limit of transformation, with moments as counterparts; meta-law and absolute relation.',
  faculty: 'buddhi',
  lens: 'yoga',
  chunks: [
    ...CHUNKS_YS_IV_33,
    ...CHUNKS_YS_IV_33_ESSENCE,
    ...CHUNKS_YS_IV_33_L24_OPENING,
    ...CHUNKS_YS_IV_33_L24_CONDITIONS,
    ...CHUNKS_YS_IV_33_L24_LOGIC,
    ...CHUNKS_YS_IV_33_L24_LAW_FACT,
    ...CHUNKS_YS_IV_33_L24_LAW_ACT_PROJECTION,
    ...CHUNKS_YS_IV_33_L24_OBJECTIVITY_DUALITY,
    ...CHUNKS_YS_IV_33_L24_APPEARANCE,   // NEW
    ...CHUNKS_YS_IV_33_L24_TWO_LIVES,    // NEW
    ...CHUNKS_YS_IV_33_L24_LAW_SELF,     // NEW
  ] as any,
  hlos: [
    ...HLOS_YS_IV_33,
    ...HLOS_YS_IV_33_ESSENCE,
    ...HLOS_YS_IV_33_L24_OPENING,
    ...HLOS_YS_IV_33_L24_CONDITIONS,
    ...HLOS_YS_IV_33_L24_LOGIC,
    ...HLOS_YS_IV_33_L24_LAW_FACT,
    ...HLOS_YS_IV_33_L24_LAW_ACT_PROJECTION,
    ...HLOS_YS_IV_33_L24_OBJECTIVITY_DUALITY,
    ...HLOS_YS_IV_33_L24_APPEARANCE,     // NEW
    ...HLOS_YS_IV_33_L24_TWO_LIVES,      // NEW
    ...HLOS_YS_IV_33_L24_LAW_SELF,       // NEW
  ] as any,
}

export const YS_IV_33_SYMBOLS = Object.keys(YS_IV_33_ONTOLOGY)
