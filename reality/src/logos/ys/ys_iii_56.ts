import { DatasetUnit, makeUnitId } from '@organon/gdsl/registry/canon'

export const YS_III_56_ONTOLOGY = {
  // Sutra and core terms
  iii56Sutra:
    'III.56 — sattva-puruṣayoḥ śuddhi-sāmye kaivalyam: When the purity of sattva and puruṣa are equal, there is kaivalya (isolation/liberation).',
  iii56Sattva:
    'sattva — luminosity/clarity of buddhi (instrument of knowing; mirror of consciousness)',
  iii56Purusha:
    'puruṣa — the Seer; pure consciousness, witness, non-active knower',
  iii56Suddhi:
    'śuddhi — purity (freedom from admixture; transparency)',
  iii56Samya:
    'sāmya — equality/equivalence (isomorphism; no remainder or asymmetry)',
  iii56Kaivalya:
    'kaivalya — aloneness/independence of the Seer; freedom through detachment of guṇa-composites',

  // Reading and criterion
  iii56EqualityCriterion:
    'Criterion: when buddhi\-s luminosity (sattva) is as pure as the Seer (puruṣa), the instrument no longer distorts; equivalence yields release',
  iii56Phenomenology:
    'Phenomenology: flawless reflection; no appropriation; cognition functions without “own-color” → disengagement',

  // Culmination links (from III.53-III.55)
  iii56FromTaraka:
    'From taraka (III.55): non-sequential, all-objects discriminative knowledge matures into equality of purity',
  iii56FromSamenessLaw:
    'From III.54\-s sameness-by-non-delimitation: no residual delimitation between instrument and witness',
  iii56FromKshanaKrama:
    'From III.53\-s kṣaṇa-krama discrimination: transitions fully governed, leaving no temporal remainder',

  // Fichte L27 overlays
  iii56ReasonIndependence:
    'Reason\-s independence: reason is ground of its own existence (fact≡origin) — maps to kaivalya as independence',
  iii56AbsoluteAppearingQualified:
    'Appearing is true only as absolute appearing; equality removes appearance\-s remainder',
  iii56SoKImmediateLife:
    'Science of Knowing as reason\-s immediate life: perfected transparency of the instrument to the Seer',

  // Architecture overlay — Two-fold Unconditioned Condition
  iii56MiddleWayBracketed:
    'Middle Way (Kriyā-Jñāna pāda: II-III) is bracketed by a Two-fold Unconditioned Condition: Prajña-pāda and Dharma-pāda',
  iii56DharmaAsEssenceEmptiesIntoJnana:
    'Dharma-pāda as Essence empties itself into Jñāna — key to Dharma Theory/Abhidharma as LogoGenesis',
  iii56LogoGenesisPrajnaDharma:
    'Prajña:Dharma are the Genesis of Knowledge (LogoGenesis) that grounds the Middle Way',

  // Two-fold Kaivalya
  iii56TwoFoldKaivalya:
    'Kaivalya is two-fold: (a) Kaivalya of Jñāna (Absolute Relation), (b) Kaivalya of Dharma (Absolute Idea, “image” of Absolute Relation)',
  iii56KaivalyaJnanaAbsoluteRelation:
    'Kaivalya of Jñāna = Absolute Relation: the end of the Middle Way (this sutra\-s scope)',
  iii56KaivalyaDharmaAbsoluteIdea:
    'Kaivalya of Dharma = Absolute Idea (image of Absolute Relation): culmination on the Dharma side',

  // Threshold forward (retargeted)
  iii56ThresholdToDharmaPada:
    'Threshold: closes Kriyā-Jñāna (Middle) with Kaivalya of Jñāna; next is Dharma-pāda (Essence → Absolute Idea), not the mainstream IV.1 path',

  // Reading note
  iii56SkipIV1Note:
    'Architectural note: skip YS IV.1 in this reading (used to support a wrong notion of Yoga; part of ordered obscuration)',

  // L28 — Setup: dual occurrence and ambiguity
  iii56L28ReasonDualOccurrence:
    'Reason appears twice (in us and “outside”) as subject and object; both claim absoluteness — ambiguity to be removed',
  iii56L28QuestionProjectionOrder:
    'Which is absolute: our projection from external reason\-s primordial projection, or the outward reason as result of its immediate self-projection in us?',

  // L28 — Pt.1: Reason makes itself unconditionally intuitive
  iii56L28MakesItselfUnconditionallyIntuiting:
    'Reason makes itself unconditionally intuiting (not merely “is intuiting”); this making is necessary and inseparable from its being',
  iii56L28MakingExplainsGapClosure:
    'Pure intellectual activity = making-itself-unconditionally-intuitive: fills the subject-object gap and negates both',
  iii56L28SelfMakingAsEffectiveness:
    'Self-making is effectiveness: inner life and activity — a making oneself into activity',
  iii56L28PrimordialActivityAndImage:
    'At once: (a) primordial activity/movement explaining seized manifestness; (b) a making/copy as its image explaining our reconstruction',
  iii56L28MidpointAbsoluteSelfMaking:
    'Stand in the midpoint: absolutely inwardly effective self-making, real through itself, without any other making or intuition',
  iii56L28AbstractFromObjectifyingMidpoint:
    'Abstract from objectifying this midpoint; otherwise subjective/objective are only added factically',

  // L28 — Pt.2: One essential effect → permanent object and subject
  iii56L28PermanentSubjectObjectSameEffect:
    'As absolute immediate self-making, reason sets itself objectively as existing and as making; permanent object and permanent subject arise from the same original essential effect in the midpoint',

  // L28 — Pt.3: Devolutions
  iii56L28ObjectiveReasonDevolvesToObjectiveLife:
    'The effect that throws down a permanent object is the same that throws down objective living; thus the primordial construction (objective reason) devolves onto objective life',
  iii56L28ImagingDevolvesToSubject:
    'The effect that throws down the persisting subject is the same that throws down imaging as imaging; imaging devolves onto the subject',

  // L28 — Result: Original disjunction
  iii56L28ResultOriginalDisjunction:
    'Reason, as immediate internal self-intuiting making (oneness of effects), breaks within the living of this making into being and making',
  iii56L28MakingOfBeingMadeNotMade:
    'Breakdown: making of being as made and not-made',
  iii56L28MakingOfMakingPrimordialCopied:
    'Breakdown: making of making as primordial/existing and not-primordial/copied',
  iii56L28DisjunctionAbsolutelyOriginal:
    'This disjunction, expressed thus, is absolutely original',

  // L28 — Pt.2b: Abstraction to the midpoint; reason as accomplished self-thinking; fork
  iii56L28AbstractToRecognizeOne:
    'To recognize reason as one, we must abstract from its objectification (we can think it as not valid in itself)',
  iii56L28PrimordialMergesIntoImage:
    'As primordial self-making, reason fully merges into our imitative image — the same relation immediately in us',
  iii56L28StandPurelyInMidpoint:
    'We/Reason stand purely in the midpoint of absolutely effective self-making (neither objective nor subjective reason)',
  iii56L28SoKReasonAliveInnerI:
    'In the science of knowing, reason is immediately alive, opened in itself, become an inward “I” — periphery and center — via abstraction',
  iii56L28AbsoluteSelfThinking:
    'Absolute reason is absolute (accomplished) thinking of oneself; thinking oneself as such is reason',
  iii56L28OptionNoObjectifyEnclosure:
    'If we do not objectify this absoluteness (by abstraction), everything ends here: reason is enclosed in itself',
  iii56L28OptionObjectifyMereFact:
    'If we objectify it, we surrender to a mere fact without principle (not derived from reason nor from anything else)',
  iii56L28PureSimpleAppearance:
    'Such objectification is pure, simple appearance (facticity without grounding)',

  // L28 — Pt.4: Fundamental principle — arising, opposite, and required consciousness
  iii56L28AbstractionArisingOneness:
    'Through abstraction (assertion of appearance), reason as absolute oneness arises and appears as arising',
  iii56L28ArisingNeedsOpposite:
    'All arising appears only with its opposite',
  iii56L28OppositeMultiplicityVariability:
    'Opposite of absolute oneness (in opposition becoming qualitative oneness) is absolute multiplicity and variability',
  iii56L28GeneticRequiresChangeableConsciousness:
    'For oneness to appear genetically, consciousness must appear as absolutely changeable and multiform',
  iii56L28FirstFundamentalPrinciple:
    'First fundamental principle: posit consciousness as absolutely changeable/multiform for the genetic appearing of oneness',
  iii56L28FirstApplicationDeduction:
    'First application of the basic deduction schema: if the Science of Knowing is to arise (as origin), then such-and-such consciousness (absolutely changeable/multiform) must be posited',

  // L28 — Pt.5: Inconceivable I; primordial consciousness; reality = primordial effect
  iii56L28IOfConsciousnessInconceivableEffect:
    'The “I” of consciousness in appearance is an inconceivable effect of reason (above all materially)',
  iii56L28InconceivabilityEntersPrimordial:
    'This inconceivability enters immediately into the primordial consciousness presupposed by the genesis',
  iii56L28PrimordialConsciousnessChangeableInfinite:
    'Primordial consciousness is unconditionally changeable and exhausts itself in infinite multiplicity',
  iii56L28InconceivableAsReal:
    'It appears explicitly as inconceivable — as real (reality-as-appearance)',
  iii56L28RealityEqualsPrimordialEffect:
    'Reality in appearance = the primordial effect of reason',
  iii56L28OneEternallySelfsame:
    'The primordial effect is the one eternally selfsame',

  // L28 — Pt.6: Grand Conclusion — Four Fundamental Principles
  iii56L28IAsReasonEffectConceptual:
    'The I of consciousness in appearance is reason\-s effect in the conceptual form of this effect (per the four presented terms)',
  iii56L28FourTermsSeenAsOne:
    'So far, the four terms were presented collectively by penetrating reason as inner oneness',
  iii56L28GeneticPresupposesExternalInternal:
    'As genetic abstraction, inner oneness presupposes external oneness; as primordial effect in experience, it presupposes inner multiplicity',
  iii56L28LackOfCorrelation:
    'Inner multiplicity/ separateness = lack of correlation in realizing the four terms (apprehended as separate principles)',
  iii56L28FourFundamentalPrinciples:
    'Thus arise four fundamental principles by absolutely necessary differentiation',

  // The four principles
  iii56L28Principle1SensibilityMaterialism:
    'Principle 1 — enduring object (even in the absolutely transient): sensibility; belief in nature; materialism',
  iii56L28Principle2PersonalityLegality:
    'Principle 2 — enduring subject: belief in personality; identity/equality of personality; principle of legality',
  iii56L28Principle3MoralityEnduringI:
    'Principle 3 — real forming of the subject (connected to the enduring subject; conceptual oneness): morality as activity from the enduring I through infinite time',
  iii56L28Principle4ReligionAbsoluteObject:
    'Principle 4 — absolute imaging and living of the absolute object (oneness as in 3): religion; belief in a God true for all lifetimes and inwardly living',

  // Mappings back to the earlier four terms (III.54)
  iii56L28MapTerm1ToP1:
    'Map: earlier “object-side” term → Principle 1 (sensibility/materialism)',
  iii56L28MapTerm2ToP2:
    'Map: enduring subject/inner enclosure → Principle 2 (personality/legality)',
  iii56L28MapTerm3ToP3:
    'Map: constructive integration (living self-enclosure) → Principle 3 (morality/real forming)',
  iii56L28MapTerm4ToP4:
    'Map: drive/coming-out-of-itself → Principle 4 (religion/absolute object living)',

  // L28 — Pt.7: Twenty-five forms; fivefoldness; genesis-conditioned consciousness
  iii56L28TwentyFiveFormsBreakdown:
    'Division into twenty-five forms coincides with the absolute breakdown (absolute multiplicity) of the real — the effect of reason whose oneness is immediately inaccessible',
  iii56L28MultiplicityFromReflectionOnOneness:
    'Multiplicity arises from the genetic nature of reflection on oneness; this reflection immediately breaks into fivefoldness',
  iii56L28ManifoldBreaksIntoFivefoldness:
    'Hence the manifold from which we must abstract breaks down in the form of fivefoldness by the same rule of reason',
  iii56L28ConsciousnessPositedBySoKGenesis:
    'All consciousness is posited and determined solely by the genesis of the Science of Knowing',
  iii56L28IfAbstractFromInsightNothingFurther:
    'If we abstract from rational insight (as condition objectifying in us), things rest there and nothing further is attained',
  iii56L28LawOfConsciousnessFromReflectionDecree:
    'Insight into the law of all consciousness arose only insofar as we reflected and posited the Science of Knowing as genesis (ought-to-arise) by absolute decree',

  // L28 — Closure and applications
  iii56L28TaskFinishedScienceClosed:
    'Task finished; the science has closed',
  iii56L28PrinciplesMaximallyClear:
    'Principles presented with the greatest possible clarity and determinateness',
  iii56L28SchematismForTheAdept:
    'Whoever has truly grasped the principles can carry out the schematism on their own',
  iii56L28ManyWordsObscure:
    'Making many words does not contribute to clarity and can obscure',
  iii56L28FutureApplicationsReligionVirtueRights:
    'Future application: religion (highest) in the inherent spirit of the science; then doctrine of virtue and of rights',
  iii56L28ThanksAndRemembrance:
    'Thanks and request for benevolent remembrance; new courage and prospects for science',
} as const

const CHUNKS_YS_III_56 = [
  {
    id: 'ys-iii-56-sutra',
    title: 'III.56 — Equality of Purity (Sattva-Puruṣa) → Kaivalya',
    summary:
      'When sattva and puruṣa have equal purity (śuddhi-sāmya), kaivalya ensues.',
  },
  {
    id: 'ys-iii-56-criterion',
    title: 'Criterion and Phenomenology',
    summary:
      'Instrument equals Seer in purity: flawless reflection; no appropriation; disengagement.',
  },
  {
    id: 'ys-iii-56-fichte',
    title: 'Fichte Overlay: Independence and Absolute Appearing',
    summary:
      'Reason as own ground (fact≡origin); appearing true only as absolute appearing; immediate life of reason.',
  },
  // New — Architecture + Two-fold Kaivalya
  {
    id: 'ys-iii-56-architecture',
    title: 'Architecture: Two-fold Unconditioned (Prajña-Dharma) and LogoGenesis',
    summary:
      'Middle Way bracketed by Prajña and Dharma; Dharma as Essence empties into Jñāna (LogoGenesis).',
  },
  {
    id: 'ys-iii-56-twofold-kaivalya',
    title: 'Two-fold Kaivalya: Jñāna (Absolute Relation) and Dharma (Absolute Idea)',
    summary:
      'III.56 delivers Kaivalya of Jñāna; Dharma-side Kaivalya is Absolute Idea (image of the Relation).',
  },
  {
    id: 'ys-iii-56-bridge',
    title: 'Bridge: From Taraka → Dharma-pāda (Essence)',
    summary:
      'Non-sequential total insight stabilizes as equality; close the Middle; proceed to Dharma-pāda (skip IV.1).',
  },
  {
    id: 'ys-iii-56-l28-setup',
    title: 'L28 Setup: Dual Reason and Ambiguity',
    summary:
      'Reason as subject/object (in us and outside); remove the ambiguity of the absolute.',
  },
  {
    id: 'ys-iii-56-l28-pt1',
    title: 'L28 Pt.1: Reason Makes Itself Unconditionally Intuitive',
    summary:
      'Necessary self-making as intuiting; closes the subject-object gap; midpoint stance; activity and its image.',
  },
  {
    id: 'ys-iii-56-l28-pt2-3',
    title: 'L28 Pt.2-3: One Effect; Devolutions to Object and Subject',
    summary:
      'Permanent object and subject from one essential effect; objective reason → objective life; imaging → subject.',
  },
  {
    id: 'ys-iii-56-l28-result',
    title: 'L28 Result: Original Disjunction',
    summary:
      'Immediate self-intuiting making breaks into being/making; making-of-being (made/not-made); making-of-making (primordial/copied).',
  },
  {
    id: 'ys-iii-56-l28-pt4-abstraction',
    title: 'L28 Pt.2 (cont.): Abstraction → Midpoint; Enclosure vs Appearance',
    summary:
      'Abstract from objectification to see the One; reason as accomplished self-thinking; fork: enclosure (no objectification) vs mere appearance (objectification).',
  },
  {
    id: 'ys-iii-56-l28-pt4-fundamental',
    title: 'L28 Pt.4: Fundamental Principle — Arising and Opposite',
    summary:
      'Oneness appears as arising only with its opposite (multiplicity); genetic requirement: posit an absolutely changeable, multiform consciousness.',
  },
  {
    id: 'ys-iii-56-l28-pt5',
    title: 'L28 Pt.5: Inconceivable I and Reality-as-Primordial Effect',
    summary:
      'The appearing I is an inconceivable effect of reason; enters primordial, changeable consciousness; reality in appearance equals the one selfsame primordial effect.',
  },
  // NEW — L28 Pt.6
  {
    id: 'ys-iii-56-l28-pt6',
    title: 'L28 Pt.6: Grand Conclusion — Four Fundamental Principles',
    summary:
      'Lack of correlation among the four terms yields four principles: sensibility/materialism; personality/legality; morality; religion.',
  },
  // NEW — L28 Pt.7
  {
    id: 'ys-iii-56-l28-pt7',
    title: 'L28 Pt.7: 25-Form Breakdown and Fivefold Rule',
    summary:
      'Twenty-five forms = absolute breakdown; reflection on oneness breaks into fivefoldness; consciousness posited by SoK genesis.',
  },
  // NEW — L28 Closure
  {
    id: 'ys-iii-56-l28-closure',
    title: 'L28 Closure: Science Closed and Applications',
    summary:
      'Task finished; principles clear; do the schematism; few words; future applications to religion, virtue, rights; thanks.',
  },
] as const

const HLOS_YS_III_56 = [
  {
    id: 'ys-iii-56-hlo-sutra',
    chunkId: 'ys-iii-56-sutra',
    label: 'Sutra',
    clauses: [
      'define(iii56Sutra)',
      'define(iii56Sattva)',
      'define(iii56Purusha)',
      'define(iii56Suddhi)',
      'define(iii56Samya)',
      'conclude(iii56Kaivalya)',
    ],
  },
  {
    id: 'ys-iii-56-hlo-criterion',
    chunkId: 'ys-iii-56-criterion',
    label: 'Criterion',
    clauses: [
      'define(iii56EqualityCriterion)',
      'note(iii56Phenomenology)',
      'link(iii55Akramam → iii56EqualityCriterion)',
    ],
  },
  {
    id: 'ys-iii-56-hlo-fichte',
    chunkId: 'ys-iii-56-fichte',
    label: 'Fichte',
    clauses: [
      'define(iii56ReasonIndependence)',
      'define(iii56AbsoluteAppearingQualified)',
      'conclude(iii56SoKImmediateLife)',
      'link(iii55FactIsOriginOriginIsFact → iii56ReasonIndependence)',
      'link(iii55AppearingTrueOnlyAsAbsoluteAppearing → iii56AbsoluteAppearingQualified)',
      'link(iii55SoKAsImmediateLife → iii56SoKImmediateLife)',
    ],
  },
  // New — Architecture + Two-fold Kaivalya
  {
    id: 'ys-iii-56-hlo-architecture',
    chunkId: 'ys-iii-56-architecture',
    label: 'Architecture',
    clauses: [
      'define(iii56MiddleWayBracketed)',
      'define(iii56DharmaAsEssenceEmptiesIntoJnana)',
      'define(iii56LogoGenesisPrajnaDharma)',
      'link(iii55TarakaSamadhi → iii56LogoGenesisPrajnaDharma)',
    ],
  },
  {
    id: 'ys-iii-56-hlo-twofold-kaivalya',
    chunkId: 'ys-iii-56-twofold-kaivalya',
    label: 'Two-fold Kaivalya',
    clauses: [
      'define(iii56TwoFoldKaivalya)',
      'define(iii56KaivalyaJnanaAbsoluteRelation)',
      'define(iii56KaivalyaDharmaAbsoluteIdea)',
      'link(iii56Kaivalya → iii56KaivalyaJnanaAbsoluteRelation)',
    ],
  },
  {
    id: 'ys-iii-56-hlo-bridge',
    chunkId: 'ys-iii-56-bridge',
    label: 'Bridge',
    clauses: [
      'define(iii56FromTaraka)',
      'define(iii56FromSamenessLaw)',
      'define(iii56FromKshanaKrama)',
      'conclude(iii56ThresholdToDharmaPada)',
      'note(iii56SkipIV1Note)',
      'link(iii55TarakaSamadhi → iii56FromTaraka)',
      'link(iii54ContinuityRecognition → iii56FromSamenessLaw)',
      'link(iii53VivekaJamJnanam → iii56FromKshanaKrama)',
    ],
  },
  {
    id: 'ys-iii-56-hlo-l28-setup',
    chunkId: 'ys-iii-56-l28-setup',
    label: 'L28 Setup',
    clauses: [
      'define(iii56L28ReasonDualOccurrence)',
      'define(iii56L28QuestionProjectionOrder)',
      'link(iii56ReasonIndependence → iii56L28ReasonDualOccurrence)',
    ],
  },
  {
    id: 'ys-iii-56-hlo-l28-pt1',
    chunkId: 'ys-iii-56-l28-pt1',
    label: 'L28 Pt.1',
    clauses: [
      'define(iii56L28MakesItselfUnconditionallyIntuiting)',
      'define(iii56L28MakingExplainsGapClosure)',
      'define(iii56L28SelfMakingAsEffectiveness)',
      'define(iii56L28PrimordialActivityAndImage)',
      'define(iii56L28MidpointAbsoluteSelfMaking)',
      'warn(iii56L28AbstractFromObjectifyingMidpoint)',
    ],
  },
  {
    id: 'ys-iii-56-hlo-l28-pt2-3',
    chunkId: 'ys-iii-56-l28-pt2-3',
    label: 'L28 Pt.2-3',
    clauses: [
      'define(iii56L28PermanentSubjectObjectSameEffect)',
      'define(iii56L28ObjectiveReasonDevolvesToObjectiveLife)',
      'define(iii56L28ImagingDevolvesToSubject)',
    ],
  },
  {
    id: 'ys-iii-56-hlo-l28-result',
    chunkId: 'ys-iii-56-l28-result',
    label: 'L28 Result',
    clauses: [
      'define(iii56L28ResultOriginalDisjunction)',
      'define(iii56L28MakingOfBeingMadeNotMade)',
      'define(iii56L28MakingOfMakingPrimordialCopied)',
      'assert(iii56L28DisjunctionAbsolutelyOriginal)',
      'link(iii56TwoFoldKaivalya → iii56L28ResultOriginalDisjunction)',
    ],
  },
  {
    id: 'ys-iii-56-hlo-l28-pt4-abstraction',
    chunkId: 'ys-iii-56-l28-pt4-abstraction',
    label: 'L28 Pt.2b — Abstraction/Fork',
    clauses: [
      'define(iii56L28AbstractToRecognizeOne)',
      'define(iii56L28PrimordialMergesIntoImage)',
      'define(iii56L28StandPurelyInMidpoint)',
      'define(iii56L28SoKReasonAliveInnerI)',
      'define(iii56L28AbsoluteSelfThinking)',
      'branch(iii56L28OptionNoObjectifyEnclosure)',
      'branch(iii56L28OptionObjectifyMereFact)',
      'conclude(iii56L28PureSimpleAppearance)',
      'link(iii56L28MidpointAbsoluteSelfMaking → iii56L28StandPurelyInMidpoint)',
      'link(iii55AppearingTrueOnlyAsAbsoluteAppearing → iii56L28PureSimpleAppearance)',
    ],
  },
  {
    id: 'ys-iii-56-hlo-l28-pt4-fundamental',
    chunkId: 'ys-iii-56-l28-pt4-fundamental',
    label: 'L28 Pt.4 — Fundamental Principle',
    clauses: [
      'define(iii56L28AbstractionArisingOneness)',
      'define(iii56L28ArisingNeedsOpposite)',
      'define(iii56L28OppositeMultiplicityVariability)',
      'define(iii56L28GeneticRequiresChangeableConsciousness)',
      'assert(iii56L28FirstFundamentalPrinciple)',
      'conclude(iii56L28FirstApplicationDeduction)',
      'link(iii55DeductionSchema → iii56L28FirstApplicationDeduction)',
    ],
  },
  {
    id: 'ys-iii-56-hlo-l28-pt5',
    chunkId: 'ys-iii-56-l28-pt5',
    label: 'L28 Pt.5 — Inconceivable I / Reality',
    clauses: [
      'define(iii56L28IOfConsciousnessInconceivableEffect)',
      'define(iii56L28InconceivabilityEntersPrimordial)',
      'define(iii56L28PrimordialConsciousnessChangeableInfinite)',
      'define(iii56L28InconceivableAsReal)',
      'conclude(iii56L28RealityEqualsPrimordialEffect)',
      'note(iii56L28OneEternallySelfsame)',
      'link(iii56L28FirstFundamentalPrinciple → iii56L28PrimordialConsciousnessChangeableInfinite)',
      'link(iii56L28OppositeMultiplicityVariability → iii56L28PrimordialConsciousnessChangeableInfinite)',
      'link(iii55AppearingTrueOnlyAsAbsoluteAppearing → iii56L28InconceivableAsReal)',
    ],
  },
  // NEW — L28 Pt.6
  {
    id: 'ys-iii-56-hlo-l28-pt6',
    chunkId: 'ys-iii-56-l28-pt6',
    label: 'L28 Pt.6 — Four Principles',
    clauses: [
      'define(iii56L28IAsReasonEffectConceptual)',
      'note(iii56L28FourTermsSeenAsOne)',
      'define(iii56L28GeneticPresupposesExternalInternal)',
      'define(iii56L28LackOfCorrelation)',
      'assert(iii56L28FourFundamentalPrinciples)',
      'define(iii56L28Principle1SensibilityMaterialism)',
      'define(iii56L28Principle2PersonalityLegality)',
      'define(iii56L28Principle3MoralityEnduringI)',
      'define(iii56L28Principle4ReligionAbsoluteObject)',
      'map(iii54Term1SeeingExternalitySelfNegation → iii56L28MapTerm1ToP1)',
      'map(iii54Term2SeeingPersistsAsInwardExternality → iii56L28MapTerm2ToP2)',
      'map(iii54Term3BeingLivingSelfEnclosureMergesIntoKnowing → iii56L28MapTerm3ToP3)',
      'map(iii54Term4DrivePrinciple → iii56L28MapTerm4ToP4)',
    ],
  },
  // NEW — L28 Pt.7
  {
    id: 'ys-iii-56-hlo-l28-pt7',
    chunkId: 'ys-iii-56-l28-pt7',
    label: 'L28 Pt.7 — Breakdown/Fivefold',
    clauses: [
      'define(iii56L28TwentyFiveFormsBreakdown)',
      'define(iii56L28MultiplicityFromReflectionOnOneness)',
      'define(iii56L28ManifoldBreaksIntoFivefoldness)',
      'define(iii56L28ConsciousnessPositedBySoKGenesis)',
      'warn(iii56L28IfAbstractFromInsightNothingFurther)',
      'conclude(iii56L28LawOfConsciousnessFromReflectionDecree)',
      'link(iii56L28ResultOriginalDisjunction → iii56L28TwentyFiveFormsBreakdown)',
      'link(iii55DeductionSchema → iii56L28LawOfConsciousnessFromReflectionDecree)',
    ],
  },
  // NEW — L28 Closure
  {
    id: 'ys-iii-56-hlo-l28-closure',
    chunkId: 'ys-iii-56-l28-closure',
    label: 'L28 — Closure',
    clauses: [
      'conclude(iii56L28TaskFinishedScienceClosed)',
      'note(iii56L28PrinciplesMaximallyClear)',
      'note(iii56L28SchematismForTheAdept)',
      'warn(iii56L28ManyWordsObscure)',
      'plan(iii56L28FutureApplicationsReligionVirtueRights)',
      'gratitude(iii56L28ThanksAndRemembrance)',
      'link(iii56KaivalyaJnanaAbsoluteRelation → iii56L28TaskFinishedScienceClosed)',
    ],
  },
] as const

export const YS_III_56_UNIT: DatasetUnit = {
  id: makeUnitId('ys-iii-56'),
  title: 'YS III.56 — Equality of Purity (Sattva-Puruṣa) → Kaivalya',
  scope: 'actuality',
  logosMode: 'appearance',
  synthesis:
    'When buddhi\-s luminosity equals the Seer\-s purity, the instrument is flawlessly transparent and kaivalya ensues — the threshold beyond taraka.',
  faculty: 'buddhi',
  lens: 'yoga',
  chunks: CHUNKS_YS_III_56 as any,
  hlos: HLOS_YS_III_56 as any,
}

export const YS_III_56_SYMBOLS = Object.keys(YS_III_56_ONTOLOGY)
