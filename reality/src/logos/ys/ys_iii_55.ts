import { DatasetUnit, makeUnitId } from '@organon/gdsl/registry/canon';

export const YS_III_55_ONTOLOGY = {
  // Sutra and core terms
  iii55Sutra:
    'III.55 — taraka sarva-visaya sarvatha-visayam akramam ceti vivekajam jnanam: Discriminative knowledge (viveka-jam jñānam) is taraka (leading-across/transcendental), having all objects as its scope, in every respect, and non-sequential.',
  iii55Taraka:
    'taraka — “leading across” (transcendental crossing/bridge): Science that unites Nature and Spirit',
  iii55SarvaVisaya:
    'sarva-viṣaya — all objects (complete scope of knowables)',
  iii55SarvathaVisayam:
    'sarvathā-viṣayam — in every mode/respect of objecthood (no remainder)',
  iii55Akramam:
    'akramam — non-sequential (immediate, simultaneity of insight; not stepwise)',
  iii55VivekajamJnanam:
    'viveka-jam jñānam — knowledge born of discrimination, here brought to its consummation as taraka',

  // Culmination and mapping
  iii55TarakaSamadhi:
    'Taraka-samādhi: consummation of viveka-born knowledge as transcendental Science of Nature and Spirit',
  iii55CulminationOfJnanaPada:
    'Culmination of Jñāna-pāda: the end of the “middle = pure conditioned” path in non-sequential discriminative insight',
  iii55BridgeNatureSpirit:
    'Bridge: Nature ⇄ Spirit integration under one Science (taraka)',
  iii55EncyclopediaMapping:
    'Hegel mapping: Encyclopedia (Logic–Nature–Spirit) as the systematic articulation of taraka knowledge',

  // Integrations
  iii55IntegrationWorlds:
    'Integration of the Worlds: Animal (phenomenal), Middle (genetic/causal), Moral (intelligible) under a single discriminative Science',
  iii55TruthAppearanceQualityQuantity:
    'Unifies truth and appearance; reconciles quality and quantity in one immediate insight (non-sequential)',
  iii55OrdinaryKnowingRedeemed:
    'Ordinary knowing’s primordial form is grounded and redeemed within taraka (no external remainder)',

  // Bridges from prior sutras
  iii55FromIII53Viveka:
    'Builds directly on III.53’s viveka-born knowledge (kṣaṇa–krama samyama)',
  iii55FromIII54Sameness:
    'Stabilizes III.54’s sameness-by-non-delimitation into a global, non-sequential scope',

  // LogoLogia / Sarvadharma overlays
  iii55TarakaLogoLogia:
    'Taraka as LogoLogia (Transcendental Science/Logic): systematic Science of Logos uniting Nature and Spirit',
  iii55AllNaturalSpiritualObjects:
    'All natural and spiritual objects fall under sarva-viṣaya (complete scope, no domain remainder)',
  iii55SarvathaAsSarvadharma:
    'sarvathā mapped to Sarvadharma (all dharmas, “in all respects”): fullness with no leftover predicates',
  iii55EssentialRelationsAsDharmaSara:
    'In-all-respects = EssentialRelations: Dharma qua Sara (the First/Absolute Dharma)',
  iii55DharmaMeghaRef:
    'Dharma-megha reference: “rain-cloud of dharma” signaling sarvadharma plenitude and consummation',

  // L27 — Presupposition, recap, midpoint method, new terms
  iii55TranscendentalPresupposesOrdinary:
    'For transcendental knowing (existence of absolute knowing) to arise, ordinary knowing must be presupposed',
  iii55PermanentResultRecapSeeingSurrendersPositsBeing:
    'Permanent result (from last hour): seeing permeating itself as seeing surrenders itself as independent and posits an absolute being',
  iii55FurtherSynthesisCertaintyEnclosure:
    'Further synthesis yielded: certainty described as enclosure into itself',
  iii55CenterWithGaps:
    'This was presented as the center; continuity to endpoints had gaps',
  iii55FillGapsMidpointOutwardOneness:
    'Task: fill gaps from the midpoint outward, according to absolute knowing in its oneness',
  iii55NewTermsNeeded:
    'Requires new terms not derivable from the foregoing',
  iii55ScientistsCreateInsight:
    'We (scientists of knowing) created the insight into seeing’s essence (and thus that seeing is necessarily a form of intuition)',
  iii55OverlookedInsightToPresent:
    'An overlooked, different insight is contained in mastering seeing’s essence and is now to be presented',

  // L27 — Points 1–2: Posited seeing ⇒ existence; genetic derivation of being-there
  iii55SeeingPositedNecessarilyOccurs:
    'If seeing is posited as seeing, then seeing necessarily occurs (actual seeing takes place)',
  iii55OntologicalProofAnalogyCompleted:
    'Completes what scholastic ontological proof sought: from essence-thought to existence — here as constructed seeing ⇒ existence of seeing',
  iii55HypotheticalConstructionNoExistence:
    'Essence-positing (construction) is hypothetical and by itself establishes neither existence nor non-existence',
  iii55ExistencePositivelyEstablished:
    'We reason: seeing necessarily occurs; its existence is positively established and expressed',
  iii55GeneticDerivationBeingThere:
    'Thus we genetically derive being-there (existence) as the inner essence of existence',
  iii55OnlyImmediateExistenceSeeing:
    'The only immediately derivable existence is that of seeing',

  // L27 — Point 2: Necessity and immediacy of seeing as act
  iii55StrenuousAttentionRequired:
    'Full proof/insight requires strenuous attention (do not miss the simplicity)',
  iii55LifeLivesButQuestion:
    'One may say “life lives,” but whether life is situated in seeing as such is the question',
  iii55SelfPermeationAbsoluteNegation:
    'Seeing’s self-permeation = absolute negation of itself as independent and a relating to something external',
  iii55ExistsOnlyInNegationRelation:
    'Seeing exists only in this self-negation and relating, otherwise not',
  iii55NegatingRelatingActImmediate:
    'This negating/relating is an act existing in itself in its immediate completion',
  iii55NecessaryImmediateActual:
    'Hence necessary, immediate, actual; if the whole is to be, this must be and exist',
  iii55SeeingPositedAsLivingPowerfulActive:
    'Seeing cannot be posited except as immediately living, powerful, active',

  // L27 — Pt.3: Absolute insight of reason; doctrine of reason
  iii55AbsoluteInsightOfReason:
    'The completed insight is absolute insight of reason — we immediately become absolute reason and dissolve into it',
  iii55InsightAsSeeingExternal:
    'Reflecting purely on the insight itself: it is seeing/insight, thus something external',
  iii55SeeingOfSeeingAbsoluteSubstantial:
    'It is a seeing of seeing, thus seeing as existent, absolute, substantial, grounded in itself and self-expressive — certainty and self-enclosure by inner necessity',
  iii55LightSelfDeterminingSelfPositing:
    'By its absolute inner essence, it is light that determines and declares itself, cannot not be, and cannot not be what it is — self-positing',
  iii55SelfPositingNotAsItselfExplain:
    'Self-positing “but not as itself”: reason’s absolute insight sees the seeing under discussion, not immediately again the first seeing, because it is absolute insight',
  iii55PropositionInsightBringsExistence:
    'Proposition: the absolute insight of reason brings absolute existence (of seeing) with itself — immediately in performing this action and as its expression',
  iii55ReasonPermeatesItselfAsEffect:
    'Absolute reason permeates itself as reason, exactly thus indicated, as absolute effect',
  iii55PriorFormalSeeingMediatedVsHereImmediate:
    'Previously: formal seeing self-permeated, negated, and posited being by mediation; here: in absolute reason it happens without mediation',
  iii55ReasonsReasonInUs:
    'Reason shows itself in us as reason’s reason — absolute reason in this self-permeation and being permeated by itself',
  iii55DoctrineOfReasonHighestPart:
    'Doctrine of Reason: through itself, from itself, in itself; the first and highest part of the science of knowing — it does not become but is unconditionally in itself and is what it is',

  // L27 — Genetic aspect of reason (no two absolutes; intrinsically genetic)
  iii55GeneticAspectReasonPosits:
    'Genetic aspect: on the condition that seeing is permeated in its inner essence, reason posits absolute existence and permeates itself as positing',
  iii55NoTwoAbsolutes:
    'Guard: do not assume a seeing and its permeatedness apart from reason itself; otherwise there would be two absolutes',
  iii55ReasonExpressionGrounded:
    'This is the expression of reason grounded in itself, occurring to arrive at positing-being and self-permeation in this positing',
  iii55ReasonIntrinsicallyGenetic:
    'Therefore reason is intrinsically genetic: consolidated, necessary, lawful in unchanging oneness',
  iii55GeneticAsLivingActiveExpression:
    'It is genetic insofar as it is truly living and expresses itself actively',
  iii55ReasonNecessarySelfDirectingExistence:
    'Reason exists necessarily and cannot not be: absolutely self-directing and authentic existence',
  iii55InwardlyWhatItAsserts:
    'Reason is inwardly and secretly what it asserts outwardly about external seeing',
  iii55ExistenceAsSpeakingOfSeeing:
    'Its inwardly grounded existence and life consist in speaking about seeing',
  iii55ButFurtherAnchor:
    '…But further: continuation anchor for ongoing derivation',

  // L27 — Pt.4: Ground, hypothesis, circle, direct speech of reason
  iii55ReasonAsOwnGround:
    'Reason as ground of its own proper existence: inward, living, active; posits its life/existence unconditionally from itself',
  iii55WeEscapedAbsolute:
    'Yet we appear to have escaped it; thus the genetic life described is not primordial absolute existence',
  iii55WeAsFreeGroundOnly:
    'We appear as ground of the insight’s possibility (free ground), not as absolute ground of its actuality',
  iii55MediateFacticalAppearance:
    'Therefore only a mediate, factical appearance/insight — not absolute reason',
  iii55ReconstructionOfOriginalConstruction:
    'The insight appears as reconstruction of an original construction in reason; we objectify primordial reason with its construction',
  iii55FreeSurrenderRepeatable:
    'We yield by a free act to reason’s original law and are made manifest/certain; this surrender/manifestness is indefinitely repeatable',
  iii55ReciprocalConditioningCircle:
    'Premise and conclusion reciprocally condition each other; the circle and hypothetical character remain; absolute condition undisclosed',
  iii55HypotheticalFormExposed:
    'Form: “If seeing is, then …; hence …” — the conclusion is presupposed hypothetically in the premise',
  iii55ArbitrarinessWarning:
    'We say “absolute reason posits it,” but it is we who say it (arbitrariness/freedom); do not trust this',
  iii55ReasonMustSpeakDirectly:
    'Requirement: reason itself must start talking directly (non-arbitrary manifestation)',

  // L27 — Conclusion: Absolute appearance/genesis; law; philosophia prima border; pure appearing; plan
  iii55AbsoluteAppearancePresented:
    'Absolute appearance (genesis) has been presented',
  iii55LawOfDerivationPresented:
    'Law for deriving absolute appearance and for deducing inferences from it has been presented',
  iii55DeductionCanProceed:
    'Deduction can proceed from the presented law',
  iii55PhilosophiaPrimaBorder:
    'Standing at the border of a philosophia prima: presenting only appearance’s first basic distinction',
  iii55FirstBasicDistinctionPureAppearing:
    'Appearance’s first basic distinction, which in its oneness constitutes the concept of pure appearing as such',
  iii55PlanSingleLectureMainPoint:
    'Choice of method: either details or a brief, forceful presentation of the main point in a single lecture',
  iii55DeferralForPreparation:
    'Deferral requested to prepare the single concluding lecture (to end on Friday)',
} as const;

const CHUNKS_YS_III_55 = [
  {
    id: 'ys-iii-55-sutra',
    title: 'III.55 — Taraka: Non-Sequential, All-Objects Viveka-Jñāna',
    summary:
      'Taraka = transcendental leading-across; all objects, in every respect; non-sequential discriminative knowledge.',
  },
  {
    id: 'ys-iii-55-features',
    title: 'Features of Taraka Knowledge',
    summary:
      'All-objects scope; all modes of objecthood; non-sequential immediacy; consummation of viveka.',
  },
  {
    id: 'ys-iii-55-integrations',
    title: 'Integrations: Nature–Spirit; Worlds; Quality–Quantity',
    summary:
      'Bridge Nature and Spirit; integrate Animal–Middle–Moral worlds; reconcile truth/appearance and quality/quantity.',
  },
  {
    id: 'ys-iii-55-bridges',
    title: 'Bridges from III.53–III.54 and to the Encyclopedia',
    summary:
      'Extends III.53 viveka and III.54 sameness into the total, immediate Science; Hegelian Encyclopedia mapping.',
  },
  {
    id: 'ys-iii-55-logologia-dharma',
    title: 'LogoLogia and Sarvadharma',
    summary:
      'Taraka as LogoLogia; sarvathā → Sarvadharma; EssentialRelations as Dharma qua Sara (First Dharma).',
  },
  {
    id: 'ys-iii-55-l27-setup',
    title: 'L27 Setup: Presupposition of Ordinary Knowing',
    summary:
      'Transcendental knowing presupposes ordinary knowing; recap of the permanent result and certainty as enclosure.',
  },
  {
    id: 'ys-iii-55-l27-method',
    title: 'L27 Method: From Midpoint Outward',
    summary:
      'Center with gaps; fill outward in the oneness of absolute knowing; new terms required.',
  },
  {
    id: 'ys-iii-55-l27-overlooked',
    title: 'L27 Overlooked Insight',
    summary:
      'Created insight into seeing’s essence implies another overlooked insight to be presented.',
  },
  {
    id: 'ys-iii-55-l27-pt1-2',
    title: 'L27 Points 1–2: From Posited Seeing to Existence',
    summary:
      'Posit seeing as seeing ⇒ seeing occurs; completes ontological-proof analogy; derive being-there; only immediate existence is seeing.',
  },
  {
    id: 'ys-iii-55-l27-pt2',
    title: 'L27 Pt.2: Necessity — Seeing as Immediate Act',
    summary:
      'Self-permeation as absolute negation/relating; immediate act; necessary, immediate, actual; seeing as living–powerful–active.',
  },
  {
    id: 'ys-iii-55-l27-pt3',
    title: 'L27 Pt.3: Absolute Insight of Reason',
    summary:
      'We become absolute reason; the insight is seeing-of-seeing, self-expressive light, self-positing (but not as itself).',
  },
  {
    id: 'ys-iii-55-l27-doctrine',
    title: 'L27: Doctrine of Reason',
    summary:
      'Immediate (unmediated) self-permeation; reason’s reason; Reason as the first and highest part of the science of knowing.',
  },
  {
    id: 'ys-iii-55-l27-genetic',
    title: 'L27: Genetic Aspect of Reason',
    summary:
      'No two absolutes; reason’s self-grounded expression; intrinsically genetic, necessary, lawful; self-directing existence; inwardly what it asserts; existence as speaking about seeing.',
  },
  // NEW — Pt.4
  {
    id: 'ys-iii-55-l27-pt4',
    title: 'L27 Pt.4: Ground, Hypothesis, and Direct Speech of Reason',
    summary:
      'We as free ground vs absolute ground; reconstruction/hypothesis and reciprocal circle; reason must speak directly.',
  },
  // NEW — L27 Conclusion
  {
    id: 'ys-iii-55-l27-conclusion',
    title: 'L27 Conclusion: Absolute Appearance and Method',
    summary:
      'Absolute appearance/genesis and its law are presented; deduction can proceed; philosophia prima border; pure appearing’s first distinction; single-lecture plan with deferral.',
  },
] as const;

const HLOS_YS_III_55 = [
  {
    id: 'ys-iii-55-hlo-sutra',
    chunkId: 'ys-iii-55-sutra',
    label: 'Sutra',
    clauses: [
      'define(iii55Sutra)',
      'define(iii55Taraka)',
      'define(iii55SarvaVisaya)',
      'define(iii55SarvathaVisayam)',
      'define(iii55Akramam)',
      'conclude(iii55VivekajamJnanam)',
    ],
  },
  {
    id: 'ys-iii-55-hlo-features',
    chunkId: 'ys-iii-55-features',
    label: 'Features',
    clauses: [
      'define(iii55TarakaSamadhi)',
      'define(iii55CulminationOfJnanaPada)',
      'assert(iii55OrdinaryKnowingRedeemed)',
      'conclude(iii55TruthAppearanceQualityQuantity)',
    ],
  },
  {
    id: 'ys-iii-55-hlo-integrations',
    chunkId: 'ys-iii-55-integrations',
    label: 'Integrations',
    clauses: [
      'define(iii55BridgeNatureSpirit)',
      'define(iii55IntegrationWorlds)',
      'link(awtAnimalWorld → iii55IntegrationWorlds)',
      'link(awtMiddleWorld → iii55IntegrationWorlds)',
      'link(awtMoralWorld → iii55IntegrationWorlds)',
      'link(hegelIdeaUnity → iii55EncyclopediaMapping)',
      'define(iii55EncyclopediaMapping)',
    ],
  },
  {
    id: 'ys-iii-55-hlo-bridges',
    chunkId: 'ys-iii-55-bridges',
    label: 'Bridges',
    clauses: [
      'define(iii55FromIII53Viveka)',
      'define(iii55FromIII54Sameness)',
      'link(iii53VivekaJamJnanam → iii55VivekajamJnanam)',
      'link(iii54BridgeToIII55 → iii55CulminationOfJnanaPada)',
      'link(iii54ContinuityRecognition → iii55Akramam)',
      'link(iii54TruthAppearanceUnified → iii55TruthAppearanceQualityQuantity)',
    ],
  },
  {
    id: 'ys-iii-55-hlo-logologia-dharma',
    chunkId: 'ys-iii-55-logologia-dharma',
    label: 'LogoLogia/Sarvadharma',
    clauses: [
      'define(iii55TarakaLogoLogia)',
      'define(iii55AllNaturalSpiritualObjects)',
      'define(iii55SarvathaAsSarvadharma)',
      'define(iii55EssentialRelationsAsDharmaSara)',
      'note(iii55DharmaMeghaRef)',
      'link(iii55SarvaVisaya → iii55AllNaturalSpiritualObjects)',
      'link(iii55SarvathaVisayam → iii55SarvathaAsSarvadharma)',
      'link(iii55Taraka → iii55TarakaLogoLogia)',
      'link(iii55EncyclopediaMapping → iii55TarakaLogoLogia)',
    ],
  },
  {
    id: 'ys-iii-55-hlo-l27-setup',
    chunkId: 'ys-iii-55-l27-setup',
    label: 'L27 Presupposition/Recap',
    clauses: [
      'define(iii55TranscendentalPresupposesOrdinary)',
      'define(iii55PermanentResultRecapSeeingSurrendersPositsBeing)',
      'define(iii55FurtherSynthesisCertaintyEnclosure)',
      'link(iii53OrdinaryKnowingCrucial → iii55TranscendentalPresupposesOrdinary)',
      'link(iii54PrimordialDescriptionCompletion → iii55FurtherSynthesisCertaintyEnclosure)',
    ],
  },
  {
    id: 'ys-iii-55-hlo-l27-method',
    chunkId: 'ys-iii-55-l27-method',
    label: 'L27 Method',
    clauses: [
      'define(iii55CenterWithGaps)',
      'define(iii55FillGapsMidpointOutwardOneness)',
      'note(iii55NewTermsNeeded)',
    ],
  },
  {
    id: 'ys-iii-55-hlo-l27-overlooked',
    chunkId: 'ys-iii-55-l27-overlooked',
    label: 'L27 Overlooked Insight',
    clauses: [
      'define(iii55ScientistsCreateInsight)',
      'note(iii55OverlookedInsightToPresent)',
      'link(iii53LogoGenesisConstruction → iii55ScientistsCreateInsight)',
    ],
  },
  {
    id: 'ys-iii-55-hlo-l27-pt1-2',
    chunkId: 'ys-iii-55-l27-pt1-2',
    label: 'L27 Pt.1–2',
    clauses: [
      'define(iii55SeeingPositedNecessarilyOccurs)',
      'note(iii55HypotheticalConstructionNoExistence)',
      'define(iii55ExistencePositivelyEstablished)',
      'conclude(iii55GeneticDerivationBeingThere)',
      'note(iii55OnlyImmediateExistenceSeeing)',
      'note(iii55OntologicalProofAnalogyCompleted)',
      'link(iii54BeingResidesInAbsoluteSeeing → iii55OnlyImmediateExistenceSeeing)',
      'link(iii54DirectInsightSeeingAsExternalityCertain → iii55SeeingPositedNecessarilyOccurs)',
    ],
  },
  {
    id: 'ys-iii-55-hlo-l27-pt2',
    chunkId: 'ys-iii-55-l27-pt2',
    label: 'L27 Pt.2',
    clauses: [
      'note(iii55StrenuousAttentionRequired)',
      'note(iii55LifeLivesButQuestion)',
      'define(iii55SelfPermeationAbsoluteNegation)',
      'define(iii55ExistsOnlyInNegationRelation)',
      'define(iii55NegatingRelatingActImmediate)',
      'conclude(iii55NecessaryImmediateActual)',
      'conclude(iii55SeeingPositedAsLivingPowerfulActive)',
      'link(iii55SeeingPositedNecessarilyOccurs → iii55NecessaryImmediateActual)',
      'link(iii54LivingEnclosingPositsAct → iii55SeeingPositedAsLivingPowerfulActive)',
    ],
  },
  {
    id: 'ys-iii-55-hlo-l27-pt3',
    chunkId: 'ys-iii-55-l27-pt3',
    label: 'L27 Pt.3 — Absolute Insight',
    clauses: [
      'define(iii55AbsoluteInsightOfReason)',
      'define(iii55InsightAsSeeingExternal)',
      'define(iii55SeeingOfSeeingAbsoluteSubstantial)',
      'define(iii55LightSelfDeterminingSelfPositing)',
      'note(iii55SelfPositingNotAsItselfExplain)',
      'define(iii55PropositionInsightBringsExistence)',
      'define(iii55ReasonPermeatesItselfAsEffect)',
      'link(iii54R3PureLight → iii55LightSelfDeterminingSelfPositing)',
      'link(iii53CertaintyAsImmanentSelfEnclosure → iii55SeeingOfSeeingAbsoluteSubstantial)',
    ],
  },
  {
    id: 'ys-iii-55-hlo-l27-doctrine',
    chunkId: 'ys-iii-55-l27-doctrine',
    label: 'L27 — Doctrine of Reason',
    clauses: [
      'define(iii55PriorFormalSeeingMediatedVsHereImmediate)',
      'define(iii55ReasonsReasonInUs)',
      'conclude(iii55DoctrineOfReasonHighestPart)',
      'link(iii54R3GenesisOfLight → iii55PriorFormalSeeingMediatedVsHereImmediate)',
    ],
  },
  {
    id: 'ys-iii-55-hlo-l27-genetic',
    chunkId: 'ys-iii-55-l27-genetic',
    label: 'L27 — Genetic Reason',
    clauses: [
      'define(iii55GeneticAspectReasonPosits)',
      'warn(iii55NoTwoAbsolutes)',
      'define(iii55ReasonExpressionGrounded)',
      'define(iii55ReasonIntrinsicallyGenetic)',
      'note(iii55GeneticAsLivingActiveExpression)',
      'define(iii55ReasonNecessarySelfDirectingExistence)',
      'conclude(iii55InwardlyWhatItAsserts)',
      'conclude(iii55ExistenceAsSpeakingOfSeeing)',
      'note(iii55ButFurtherAnchor)',
      'link(iii55PropositionInsightBringsExistence → iii55GeneticAspectReasonPosits)',
      'link(iii55ReasonPermeatesItselfAsEffect → iii55ReasonIntrinsicallyGenetic)',
      'link(iii54ObjectivityEqualsGenesis → iii55ReasonExpressionGrounded)',
    ],
  },
  // NEW — Pt.4
  {
    id: 'ys-iii-55-hlo-l27-pt4',
    chunkId: 'ys-iii-55-l27-pt4',
    label: 'L27 Pt.4 — Ground/Hypothesis',
    clauses: [
      'define(iii55ReasonAsOwnGround)',
      'note(iii55WeEscapedAbsolute)',
      'define(iii55WeAsFreeGroundOnly)',
      'conclude(iii55MediateFacticalAppearance)',
      'define(iii55ReconstructionOfOriginalConstruction)',
      'note(iii55FreeSurrenderRepeatable)',
      'warn(iii55ReciprocalConditioningCircle)',
      'define(iii55HypotheticalFormExposed)',
      'warn(iii55ArbitrarinessWarning)',
      'assert(iii55ReasonMustSpeakDirectly)',
      'link(iii55ButFurtherAnchor → iii55ReasonAsOwnGround)',
      'link(iii55CenterWithGaps → iii55ReasonMustSpeakDirectly)',
    ],
  },
  // NEW — L27 Conclusion
  {
    id: 'ys-iii-55-hlo-l27-conclusion',
    chunkId: 'ys-iii-55-l27-conclusion',
    label: 'L27 — Conclusion',
    clauses: [
      'define(iii55AbsoluteAppearancePresented)',
      'define(iii55LawOfDerivationPresented)',
      'conclude(iii55DeductionCanProceed)',
      'define(iii55PhilosophiaPrimaBorder)',
      'define(iii55FirstBasicDistinctionPureAppearing)',
      'note(iii55PlanSingleLectureMainPoint)',
      'note(iii55DeferralForPreparation)',
      'link(iii55TarakaSamadhi → iii55DeductionCanProceed)',
      'link(iii55CulminationOfJnanaPada → iii55PhilosophiaPrimaBorder)',
    ],
  },
] as const;

export const YS_III_55_UNIT: DatasetUnit = {
  id: makeUnitId('ys-iii-55'),
  title: 'YS III.55 — Taraka (Transcendental, Non-Sequential Viveka-Jñāna)',
  scope: 'actuality',
  logosMode: 'appearance',
  synthesis:
    'Taraka knowledge “leads across”: all-objects, in every respect, non-sequential. It consummates viveka as a transcendental Science uniting Nature and Spirit, integrating the three worlds and reconciling truth/appearance and quality/quantity.',
  faculty: 'buddhi',
  lens: 'yoga',
  chunks: CHUNKS_YS_III_55 as any,
  hlos: HLOS_YS_III_55 as any,
};

export const YS_III_55_SYMBOLS = Object.keys(YS_III_55_ONTOLOGY);
