import { DatasetUnit, makeUnitId } from '@organon/gdsl/registry/canon'

// YS IV.29 — prasaṅkhyāne 'py akusīdasya sarvathā viveka-khyāteḥ dharma-meghaḥ samādhiḥ

export const YS_IV_29_ONTOLOGY = {
  sutraDevanagari:
    "प्रसंख्यानेऽप्यकुसीदस्य सर्वथा विवेकख्यातेर्धर्ममेघः समाधिः",
  sutraIAST:
    "prasaṅkhyāne \'py akusīdasya sarvathā viveka-khyāteḥ dharma-meghaḥ samādhiḥ",
  sutraGloss:
    "Even with regard to prasaṅkhyāna, for one who is non-attached, when complete discriminative knowledge is present, there is Dharma-megha samādhi.",
  nonAttachmentToPrasankhyana:
    "Non-attachment even to prasaṅkhyāna (results and procedures)",
  completeVivekaKhyati:
    "Complete discriminative knowledge (viveka-khyāti) established",
  dharmaMeghaSamadhi:
    "Dharma-megha samādhi: cloud of dharmas — treated here as an algebraic Discriminator",
  algebraicDiscriminatorGuard:
    "“Discriminator” used in the algebraic/analytic sense (not social usage)",
  pratisankhyanirodhaPreferred:
    "Prefer prati-saṁkhyā-nirodha (discriminative cessation) over a contentless default",
  apratisankhyanirodhaWarning:
    "Claiming “sarva-dharma” without enumeration defaults to aprati-saṁkhyā-nirodha (warn)",
  // Fichte — Lecture 23 (bridges)
  absoluteSelfGenesisPrinciple:
    'Principle for absolute self-genesis (light as self-producing)',
  nonGenesisBeingOfKnowing:
    'Non-genesis as “being of knowing” (within knowing itself)',
  facticalJustification:
    'Method justified factically by produced insight within us (we are knowing)',
  objectiveInsightArisen:
    'An objective, compelling insight has arisen for us',
  coherenceWithAbsoluteLight:
    'This insight coheres with the absolute, self-producing light',
  taskInvestigateContent:
    'Next task: investigate the objective insight in respect to its true content',
  towardGeneticJustification:
    'Aim to justify more deeply, perhaps make the point genetic (not merely factual)',
  // Extend ontology — Fichte master statement, Kant difference, Kaivalya, Middle World
  fichteMasterStatement:
    'Pure inner certainty (in-act) remains after abstracting hypothetical terms; grounds being-as-support of the what',
  kantPrincipleAdopted:
    'Adopts Kant: being is not sum of realities and is condition/support of the “what”',
  kantDifferenceFichte:
    'Differs from Kant by aiming at a genetic derivation and performative (in-act) certainty',
  kaivalyaReadingFromFichte:
    'Kaivalya read as independence of the Seer grounded in pure certainty, not as content gain',
  moralAnimalWorldExtremes:
    '“Moral world” and “animal world” are extremes within appearance, not parts of each other',
  middleWorldModality:
    'A Middle World of modality/appearance posits and relates these extremes',
  middleWorldUnknownToKant:
    'This Middle World is unthematized in Kant; clarified via Fichte\'s certainty-in-act',
}

// Modality (Idea/Science) + Certainty (description) — ontology extensions
Object.assign(YS_IV_29_ONTOLOGY, {
  absoluteIdeaCertainty:
    'Absolute Certainty of the Absolute Idea (modality at level of Science/Idea)',
  modalityOfScienceNotLogic:
    'Modality of Science (scientia), not formal Logic',
  modalIdeaNotConcept:
    'Modal locus is the Idea, not the Concept',
  aPosterioriAnalyticaModality:
    'Modality of a posteriori analytica (performative-scientific), not a priori analytica',

  descriptionAsUnchangeability:
    'Pure certainty described as unshakable continuance/resting in the same unchangeable oneness (same “what”/quality)',
  persistingOnenessOfWhat:
    'Unchangeability = persisting oneness of the “what” (quality)',
  formalIndifferenceOfWhat:
    'Description contains absolute indifference to any particular “what”',
  formalPurityCondition:
    'Valid only on condition of formal purity (use only the pure form of the “what”)',
})

// Ontology — Truth of Māyā + Fichte L23 §6–§7 (grounding in itself, immanence)
Object.assign(YS_IV_29_ONTOLOGY, {
  truthOfMayaInsight:
    'Truth of Māyā: upon being understood, Māyā becomes non-productive for that yogi (appearance ceases to “produce” effects)',
  mayaNonProductiveDiscipline:
    'Discipline: treat Māyā as appearance/measure only; once seen through, it does not bind or generate',
  certaintyGroundedInItself:
    'Certainty is grounded completely and absolutely in itself',
  certaintyPersistsSameWhat:
    'Certainty (by description) = persistence in the same “what” (quality)',
  onenessGroundedInCertainty:
    'Ground of the “what\'s” oneness is entirely inward within certainty itself (no external ground)',
  immanentSelfEnclosedCertainty:
    'Certainty is absolute, immanent, self-enclosed, never going outside itself',
  certaintyAsIWe:
    'In itself it is I/We (immanence as I-form), matching prior proof of being\'s form',
  abstractFromObjectivizedCertainty:
    'Abstract from externalized/objectivized certainty when seeking the absolute; search in immanence (I/We)',
})

// Ontology — Modality bridge (Idea ↔ Concept) and Certainty → Necessity (Fichte L23 continuation)
Object.assign(YS_IV_29_ONTOLOGY, {
  linkIdeaConceptModality:
    'Link Modality of the Idea (Reason, in-act) to Modality of the Concept (Judgment)',
  necessityAsConceptualModality:
    'Necessity is conceptual modality; the Concept\'s modal predicate',
  reasonNoPossibilityActuality:
    'Reason (Idea-level) carries no external possibility/actuality predicates; these belong to the Concept/judgment',
  intellectualIntuitionAspect:
    'Intellectual intuition: immediate identity in act; grounds modal transition without external ground',

  certaintyAsNecessityOfResting:
    'Certainty found as necessity of resting in the procedure\'s qualitative oneness',
  absoluteCertaintyEqualsWe:
    'Absolute certainty is the same as I/We: immanent, self-enclosed, inaccessible',
  weInaccessibleGuard:
    'Guard: the I/We cannot be “accessed” as an external object without contradiction',
  appearanceContradictionNote:
    'Speaking of the absolute I/We is appearance that contradicts its truth; its possibility must be deduced within appearance',
  deductionOfAppearanceTask:
    'Task: deduce the possibility of this appearance from the system of appearance',
})

// Chunk — Modality bridge (Idea ↔ Concept)
const CHUNKS_YS_IV_29_MODALITY_LINK = [
  {
    id: 'ys-iv-29-modality-link',
    title: 'Modality — Idea ↔ Concept',
    summary:
      'Bridge the Modal Idea (Reason) to Conceptual Modality: certainty → necessity; note on intellectual intuition.',
  },
] as const

const HLOS_YS_IV_29_MODALITY_LINK = [
  {
    id: 'ys-iv-29-hlo-modality-link',
    chunkId: 'ys-iv-29-modality-link',
    label: 'Bridge',
    clauses: [
      'define(linkIdeaConceptModality)',
      'define(necessityAsConceptualModality)',
      'assert(modalityOfScienceNotLogic)',
      'note(reasonNoPossibilityActuality)',
      'note(intellectualIntuitionAspect)',
    ],
  },
] as const

// Chunk — Fichte: Certainty → Necessity
const CHUNKS_YS_IV_29_FICHTE_NECESSITY = [
  {
    id: 'ys-iv-29-fichte-necessity',
    title: 'Fichte — Certainty → Necessity',
    summary:
      'Certainty as necessity of resting in qualitative oneness; persistence of the same “what.”',
  },
] as const

const HLOS_YS_IV_29_FICHTE_NECESSITY = [
  {
    id: 'ys-iv-29-hlo-fichte-necessity',
    chunkId: 'ys-iv-29-fichte-necessity',
    label: 'Necessity',
    clauses: [
      'define(certaintyAsNecessityOfResting)',
      'link(certainty → necessity)',
      'assert(persistingOnenessOfWhat)',
    ],
  },
] as const

// Chunk — Fichte: Inaccessibility of the We (immanence)
const CHUNKS_YS_IV_29_FICHTE_WE = [
  {
    id: 'ys-iv-29-fichte-we-immanence',
    title: 'Fichte — Inaccessible We (Immanence)',
    summary:
      'Absolute certainty = I/We; self-enclosed, inaccessible. Speaking of it is appearance; deduce that appearance\'s possibility.',
  },
] as const

const HLOS_YS_IV_29_FICHTE_WE = [
  {
    id: 'ys-iv-29-hlo-fichte-we-immanence',
    chunkId: 'ys-iv-29-fichte-we-immanence',
    label: 'We/Immanence',
    clauses: [
      'define(absoluteCertaintyEqualsWe)',
      'assert(weInaccessibleGuard)',
      'define(appearanceContradictionNote)',
      'define(deductionOfAppearanceTask)',
    ],
  },
] as const

// Keep original chunks
const CHUNKS_YS_IV_29 = [
  {
    id: 'ys-iv-29-text',
    title: 'IV.29 — Text',
    summary: 'Sūtra text and literal rendering.',
  },
  {
    id: 'ys-iv-29-meaning',
    title: 'Meaning',
    summary: 'Non-attachment to prasaṅkhyāna; complete viveka-khyāti; Dharma-megha as Discriminator.',
  },
  {
    id: 'ys-iv-29-guards',
    title: 'Guards',
    summary: 'Algebraic “discriminator” sense; prati-saṁkhyā over aprati-saṁkhyā.',
  },
] as const

const HLOS_YS_IV_29 = [
  {
    id: 'ys-iv-29-hlo-text',
    chunkId: 'ys-iv-29-text',
    label: 'Text',
    clauses: [
      'define(sutraDevanagari)',
      'define(sutraIAST)',
      'note(sutraGloss)',
    ],
  },
  {
    id: 'ys-iv-29-hlo-meaning',
    chunkId: 'ys-iv-29-meaning',
    label: 'Meaning',
    clauses: [
      'define(nonAttachmentToPrasankhyana)',
      'assert(completeVivekaKhyati)',
      'define(dharmaMeghaSamadhi := Discriminator_algebraic)',
      'assert(pratisankhyanirodhaPreferred)',
    ],
  },
  {
    id: 'ys-iv-29-hlo-guards',
    chunkId: 'ys-iv-29-guards',
    label: 'Guards',
    clauses: [
      'algebraicDiscriminatorGuard := assert(true)',
      'apratisankhyanirodhaWarning := flag(sarva_dharma_without_enumeration)',
    ],
  },
] as const

// Fichte Lecture 23 — extract as its own chunked HLOs
const CHUNKS_YS_IV_29_FICHTE = [
  {
    id: 'ys-iv-29-fichte-23',
    title: 'Fichte — Lecture 23 (extract)',
    summary: 'Self-genesis → non-genesis (being of knowing); factical method; objective insight; task: content/genesis.',
  },
]

const HLOS_YS_IV_29_FICHTE = [
  {
    id: 'ys-iv-29-hlo-fichte-23',
    chunkId: 'ys-iv-29-fichte-23',
    label: 'Bridges',
    clauses: [
      'define(absoluteSelfGenesisPrinciple)',
      'define(nonGenesisBeingOfKnowing)',
      'assert(facticalJustification)',
      'assert(objectiveInsightArisen)',
      'assert(coherenceWithAbsoluteLight)',
      'taskInvestigateContent := todo(true)',
      'towardGeneticJustification := note(true)',
    ],
  },
] as const

// New chunk/HLO capturing the master statement and Kaivalya read
const CHUNKS_YS_IV_29_FICHTE_MASTER = [
  {
    id: 'ys-iv-29-fichte-master',
    title: 'Fichte — Master Statement → Kaivalya',
    summary:
      'Pure certainty (in-act), Kant principle with Fichtean difference, Middle World, and Kaivalya as independence of the Seer.',
  },
]

const HLOS_YS_IV_29_FICHTE_MASTER = [
  {
    id: 'ys-iv-29-hlo-fichte-master',
    chunkId: 'ys-iv-29-fichte-master',
    label: 'Master → Kaivalya',
    clauses: [
      'define(fichteMasterStatement)',
      'define(kantPrincipleAdopted)',
      'note(kantDifferenceFichte)',
      'assert(completeVivekaKhyati)',
      'define(kaivalyaReadingFromFichte)',
      'define(moralAnimalWorldExtremes)',
      'define(middleWorldModality)',
      'note(middleWorldUnknownToKant)',
      'link(dharmaMeghaSamadhi → kaivalyaReadingFromFichte)',
    ],
  },
] as const

// New chunks/HLOs — Modality clarifications and Certainty description (L23 §2–§3)
const CHUNKS_YS_IV_29_MODALITY = [
  {
    id: 'ys-iv-29-modality-idea',
    title: 'Modality — Idea/Science',
    summary: 'Absolute Idea\'s certainty; Modality of Science (a posteriori analytica), not Concept/Logic.',
  },
] as const

const HLOS_YS_IV_29_MODALITY = [
  {
    id: 'ys-iv-29-hlo-modality-idea',
    chunkId: 'ys-iv-29-modality-idea',
    label: 'Idea/Science',
    clauses: [
      'define(absoluteIdeaCertainty)',
      'assert(modalityOfScienceNotLogic)',
      'assert(modalIdeaNotConcept)',
      'define(aPosterioriAnalyticaModality)',
    ],
  },
] as const

const CHUNKS_YS_IV_29_FICHTE_CERT_DESC = [
  {
    id: 'ys-iv-29-fichte-cert-desc',
    title: 'Fichte — Pure Certainty (Description)',
    summary: 'Describe certainty as unchangeability (persisting oneness of the “what”) with formal indifference/purity.',
  },
] as const

const HLOS_YS_IV_29_FICHTE_CERT_DESC = [
  {
    id: 'ys-iv-29-hlo-fichte-cert-desc',
    chunkId: 'ys-iv-29-fichte-cert-desc',
    label: 'Description',
    clauses: [
      'define(descriptionAsUnchangeability)',
      'assert(persistingOnenessOfWhat)',
      'define(formalIndifferenceOfWhat)',
      'assert(formalPurityCondition)',
    ],
  },
] as const

// Chunks — Māyā truth; Fichte grounding/immanence
const CHUNKS_YS_IV_29_MAYA_TRUTH = [
  {
    id: 'ys-iv-29-maya-truth',
    title: 'Truth of Māyā — Non-productivity',
    summary:
      'When Māyā is understood, it becomes non-productive for the yogi; keep Māyā as appearance/measure only.',
  },
] as const

const HLOS_YS_IV_29_MAYA_TRUTH = [
  {
    id: 'ys-iv-29-hlo-maya-truth',
    chunkId: 'ys-iv-29-maya-truth',
    label: 'Māyā',
    clauses: [
      'define(truthOfMayaInsight)',
      'assert(mayaNonProductiveDiscipline)',
      'link(dharmaMeghaSamadhi → truthOfMayaInsight)',
    ],
  },
] as const

const CHUNKS_YS_IV_29_FICHTE_GROUNDS = [
  {
    id: 'ys-iv-29-fichte-grounding',
    title: 'Fichte — L23 §6–§7: Grounding and Immanence',
    summary:
      'Certainty grounded in itself; oneness of the “what” grounded inwardly; certainty as immanent I/We; abstract from objectivized certainty.',
  },
] as const

const HLOS_YS_IV_29_FICHTE_GROUNDS = [
  {
    id: 'ys-iv-29-hlo-fichte-grounding',
    chunkId: 'ys-iv-29-fichte-grounding',
    label: 'Grounding/Immanence',
    clauses: [
      'define(certaintyGroundedInItself)',
      'define(certaintyPersistsSameWhat)',
      'assert(onenessGroundedInCertainty)',
      'assert(immanentSelfEnclosedCertainty)',
      'define(certaintyAsIWe)',
      'assert(abstractFromObjectivizedCertainty)',
    ],
  },
] as const

// Ontology — Fichte: Immanent Intuition (Light as Principle)
Object.assign(YS_IV_29_ONTOLOGY, {
  soonToShowGround:
    'Ground of this immanent expression will show itself shortly (not yet clear)',
  immanentExpressionOfCertainty:
    'Certainty expresses itself immanently (in us) as perception of a particular, completely unchanging process',
  manifestationAsAbsoluteFactObscure:
    'This manifestation appears only as absolute fact; an obscurity still remains until the ground shows itself',
  processAsLiving:
    'Process is living as living',
  qualitativeOnenessAsLifeImmanence:
    'The process\'s unchanging qualitative oneness is life\'s immanence and self-groundedness expressed immediately in life',
  lightAsPrinciple:
    'Immediately living and immanent being-a-principle is light',
  intuitionWithInnerNecessity:
    'Light as principle is intuition with an inner necessity',
  beingAPrincipleDefinition:
    'Being-a-principle = projecting and intuiting',
  projectedTermImmanence:
    'The projected term is immediately living and immanent in intuiting, from intuiting, and out of intuiting',
  projectingAsLightsLife:
    'Projecting is light\'s life as “principle-providing”',
  innerNecessityMustExpress:
    'Inner necessity must completely express itself because it is principle-providing as such',
})

// Chunk — Fichte: Immanent Intuition (Light as Principle)
const CHUNKS_YS_IV_29_FICHTE_IMMANENT = [
  {
    id: 'ys-iv-29-fichte-immanent-intuition',
    title: 'Fichte — Immanent Intuition (Light as Principle)',
    summary:
      'Immanent certainty as unchanging process; life\'s qualitative oneness; light = being-a-principle; intuition with inner necessity.',
  },
] as const

const HLOS_YS_IV_29_FICHTE_IMMANENT = [
  {
    id: 'ys-iv-29-hlo-fichte-immanent-intuition',
    chunkId: 'ys-iv-29-fichte-immanent-intuition',
    label: 'Immanent Intuition',
    clauses: [
      'note(soonToShowGround)',
      'define(immanentExpressionOfCertainty)',
      'define(manifestationAsAbsoluteFactObscure)',
      'define(processAsLiving)',
      'assert(qualitativeOnenessAsLifeImmanence)',
      'define(lightAsPrinciple)',
      'define(intuitionWithInnerNecessity)',
      'define(beingAPrincipleDefinition)',
      'define(projectedTermImmanence)',
      'define(projectingAsLightsLife)',
      'assert(innerNecessityMustExpress)',
      'link(intuitionWithInnerNecessity → certaintyAsNecessityOfResting)',
    ],
  },
] as const

// Ontology — Truth of Māyā as Absolute Form (immanent self-projection) + three light modifications
Object.assign(YS_IV_29_ONTOLOGY, {
  absoluteFormProjection:
    'Absolute, immanent self-projection (veiling/revealing) — projection of nothing but itself as it inwardly is',
  veilingRevealingOperation:
    'Māyā as dividing principle operates as simultaneous veiling/revealing (appearance logic)',
  innerQualitativeSelfProjecting:
    'Inner qualitative self-projecting (not objectively understood): being-a-principle as thinking/intuiting',
  principleProvidingLife:
    'Light\'s life as “principle-providing” (projecting and intuiting) with inner necessity',
  necessaryObjectiveProjection:
    'From inner self-projection, a necessary objective projection results (not yet the objective I)',
  livingOneSelfGroundedFormal:
    'What is projected first appears as living, one, self-grounded (formally)',
  processPureQualitativeOneness:
    'This “first” is process in pure qualitative oneness (intuition of inner certainty and oneness of process)',
  necessityOfExpression:
    'Oneness expresses itself with necessity as result of absolute, living principle-providing',
  describingCertaintyFoundGround:
    '“Describing certainty” found its ground in life\'s principle-providing necessity',
  lifeNecessityInCertainty:
    'Life is unconditionally necessary in being/certainty; description is its necessary expression',
  selfConstructionInProjection:
    'Life constructs itself in projection; construction is also projective',
  certaintyMorePrimordialInUs:
    'In living description, certainty is more primordial in us than objectively in itself',
  objectiveCertaintyResultOfConstruction:
    'Objective certainty (taken “in itself”) is only a result of the constructive projection',

  // Three primary modifications of the primordial light
  lightMod1InnerPrinciple:
    'Modification 1 — Inner qualitative self-projection (being-a-principle: thinking/intuiting)',
  lightMod2ObjectiveProjection:
    'Modification 2 — Necessary objective projection (as living, one, self-grounded; not yet objective I)',
  lightMod3ConstructiveDescription:
    'Modification 3 — Constructive description (self-construction in projection → objective certainty)',
})

// Chunk — Truth of Māyā: Absolute Form (immanent self-projection)
const CHUNKS_YS_IV_29_MAYA_ABS_FORM = [
  {
    id: 'ys-iv-29-maya-absolute-form',
    title: 'Truth of Māyā — Absolute Form (Immanent Self-projection)',
    summary:
      'Māyā as veiling/revealing absolute form: inner qualitative self-projection → necessary objective appearing; construction is projective.',
  },
] as const

const HLOS_YS_IV_29_MAYA_ABS_FORM = [
  {
    id: 'ys-iv-29-hlo-maya-absolute-form',
    chunkId: 'ys-iv-29-maya-absolute-form',
    label: 'Absolute Form',
    clauses: [
      'define(absoluteFormProjection)',
      'define(veilingRevealingOperation)',
      'define(innerQualitativeSelfProjecting)',
      'assert(principleProvidingLife)',
      'conclude(necessaryObjectiveProjection)',
      'define(livingOneSelfGroundedFormal)',
      'assert(processPureQualitativeOneness)',
      'assert(necessityOfExpression)',
      'define(describingCertaintyFoundGround)',
      'assert(lifeNecessityInCertainty)',
      'define(selfConstructionInProjection)',
      'assert(certaintyMorePrimordialInUs)',
      'conclude(objectiveCertaintyResultOfConstruction)',
      'link(truthOfMayaInsight → absoluteFormProjection)',
      'link(lightAsPrinciple → principleProvidingLife)',
      'link(intuitionWithInnerNecessity → necessityOfExpression)',
    ],
  },
] as const

// Chunk — Three primary modifications of the primordial light
const CHUNKS_YS_IV_29_LIGHT_MODS = [
  {
    id: 'ys-iv-29-light-mods',
    title: 'Primordial Light — Three Modifications',
    summary:
      '1) Inner principle (self-projection). 2) Necessary objective projection. 3) Constructive description → objective certainty.',
  },
] as const

const HLOS_YS_IV_29_LIGHT_MODS = [
  {
    id: 'ys-iv-29-hlo-light-mods',
    chunkId: 'ys-iv-29-light-mods',
    label: 'Light Modifications',
    clauses: [
      'define(lightMod1InnerPrinciple)',
      'define(lightMod2ObjectiveProjection)',
      'define(lightMod3ConstructiveDescription)',
      'note("Use these as beacons for bridging Modality (Idea ↔ Concept).")',
    ],
  },
] as const

// Ontology — Kant over-division: Modality (Idea vs Concept) and Relation
Object.assign(YS_IV_29_ONTOLOGY, {
  kantOverDividedSystem:
    'Kant is over-divided: Certainty treated in Jäsche Logic Intro; CPR handles Concept-modality and Relation under Understanding',
  kantJascheIntroCertainty:
    'Jäsche Logic (Introduction): locus for Kant\'s remarks on certainty',
  kantCPRModalityConcept:
    'Critique of Pure Reason: modality taught as modality of the Concept (Understanding)',
  kantCPRRelationEssential:
    'CPR Relation is essential/mediated relation (categories), not Absolute Relation in act',
  absoluteRelationAtIdea:
    'Absolute Relation belongs to the Idea-level (Reason), not to Concept-level predicates',
  guardNoConflationKantLevels:
    'Guard: do not conflate Idea-modality/Absolute Relation with Concept-modality/Essential Relation',
})

// Chunk — Idea vs Concept (Modal Reasoning)
const CHUNKS_YS_IV_29_MODAL_REASONING = [
  {
    id: 'ys-iv-29-modal-reasoning',
    title: 'Idea vs Concept — Modal Reasoning',
    summary:
      'Modality of the Idea (Reason) vs Modality of the Concept (Understanding); align with light-mods.',
  },
] as const

const HLOS_YS_IV_29_MODAL_REASONING = [
  {
    id: 'ys-iv-29-hlo-modal-reasoning',
    chunkId: 'ys-iv-29-modal-reasoning',
    label: 'Reason/Understanding',
    clauses: [
      'define(modalityOfScienceNotLogic)',
      'define(modalIdeaNotConcept)',
      'define(aPosterioriAnalyticaModality)',
      'define(kantCPRModalityConcept)',
      'assert(guardNoConflationKantLevels)',
      'link(lightMod1InnerPrinciple → modalityOfScienceNotLogic)',
    ],
  },
] as const

// Chunk — Absolute vs Essential Relation
const CHUNKS_YS_IV_29_RELATIONS = [
  {
    id: 'ys-iv-29-relations',
    title: 'Absolute vs Essential Relation',
    summary:
      'Absolute Relation (Idea-level reflection in act) vs Essential Relation (category of relation in CPR).',
  },
] as const

const HLOS_YS_IV_29_RELATIONS = [
  {
    id: 'ys-iv-29-hlo-relations',
    chunkId: 'ys-iv-29-relations',
    label: 'Relations',
    clauses: [
      'define(absoluteRelationAtIdea)',
      'define(kantCPRRelationEssential)',
      'assert(guardNoConflationKantLevels)',
      'link(absoluteFormProjection → absoluteRelationAtIdea)',
      'link(lightMod3ConstructiveDescription → absoluteRelationAtIdea)',
    ],
  },
] as const

// Chunk — Kant Over-division (note)
const CHUNKS_YS_IV_29_KANT_MODALITY_REL = [
  {
    id: 'ys-iv-29-kant-modality-relation',
    title: 'Kant — Over-division (Modality and Relation)',
    summary:
      'Jäsche Intro (certainty) vs CPR (Concept-modality; essential relation). Absolute Relation sits at Idea-level.',
  },
] as const

const HLOS_YS_IV_29_KANT_MODALITY_REL = [
  {
    id: 'ys-iv-29-hlo-kant-modality-relation',
    chunkId: 'ys-iv-29-kant-modality-relation',
    label: 'Kant',
    clauses: [
      'define(kantOverDividedSystem)',
      'note(kantJascheIntroCertainty)',
      'define(kantCPRModalityConcept)',
      'define(kantCPRRelationEssential)',
      'define(absoluteRelationAtIdea)',
      'assert(guardNoConflationKantLevels)',
      'link(ys-iv-29-hlo-modality-idea → ys-iv-29-hlo-kant-modality-relation)',
    ],
  },
] as const

// Export unit (merge core + Fichte chunks/HLOs)
export const YS_IV_29_UNIT: DatasetUnit = {
  id: makeUnitId('ys-iv-29'),
  title: 'YS IV.29 — Dharma-megha',
  scope: 'actuality',
  logosMode: 'appearance',
  synthesis:
    'Non-attachment to prasaṅkhyāna, complete viveka-khyāti, and Dharma-megha samādhi as algebraic Discriminator.',
  faculty: 'buddhi',
  lens: 'yoga',
  chunks: [
    ...CHUNKS_YS_IV_29,
    ...CHUNKS_YS_IV_29_FICHTE,
    ...CHUNKS_YS_IV_29_FICHTE_MASTER,
    ...CHUNKS_YS_IV_29_MODALITY,
    ...CHUNKS_YS_IV_29_FICHTE_CERT_DESC,
    ...CHUNKS_YS_IV_29_MAYA_TRUTH,
    ...CHUNKS_YS_IV_29_FICHTE_GROUNDS,
    ...CHUNKS_YS_IV_29_MODALITY_LINK,
    ...CHUNKS_YS_IV_29_FICHTE_NECESSITY,
    ...CHUNKS_YS_IV_29_FICHTE_WE,
    ...CHUNKS_YS_IV_29_FICHTE_IMMANENT,
    ...CHUNKS_YS_IV_29_MAYA_ABS_FORM,
    ...CHUNKS_YS_IV_29_LIGHT_MODS,
    ...CHUNKS_YS_IV_29_MODAL_REASONING,
    ...CHUNKS_YS_IV_29_RELATIONS,
    ...CHUNKS_YS_IV_29_KANT_MODALITY_REL,
  ] as any,
  hlos: [
    ...HLOS_YS_IV_29,
    ...HLOS_YS_IV_29_FICHTE,
    ...HLOS_YS_IV_29_FICHTE_MASTER,
    ...HLOS_YS_IV_29_MODALITY,
    ...HLOS_YS_IV_29_FICHTE_CERT_DESC,
    ...HLOS_YS_IV_29_MAYA_TRUTH,
    ...HLOS_YS_IV_29_FICHTE_GROUNDS,
    ...HLOS_YS_IV_29_MODALITY_LINK,
    ...HLOS_YS_IV_29_FICHTE_NECESSITY,
    ...HLOS_YS_IV_29_FICHTE_WE,
    ...HLOS_YS_IV_29_FICHTE_IMMANENT,
    ...HLOS_YS_IV_29_MAYA_ABS_FORM,
    ...HLOS_YS_IV_29_LIGHT_MODS,
    ...HLOS_YS_IV_29_MODAL_REASONING,
    ...HLOS_YS_IV_29_RELATIONS,
    ...HLOS_YS_IV_29_KANT_MODALITY_REL,
  ] as any,
}

// Optional: simple validator hook for tests/tooling
export const SUTRA_DEVANAGARI_KEY = 'sutraDevanagari'
export function hasSutraDevanagari(o: Record<string, unknown>): boolean {
  return typeof o?.[SUTRA_DEVANAGARI_KEY] === 'string' && (o[SUTRA_DEVANAGARI_KEY] as string).length > 0
}
