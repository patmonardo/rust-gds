import { DatasetUnit, makeUnitId } from '@organon/gdsl/registry/canon'

/*
YS IV.10 (Transition: Vāsanā/Saṁskāra → Dependent Origination/Facticity)
tāsām anāditvaṁ cāśiṣo nityatvāt

Parse:
- tāsām: of those (gen. fem. pl.; most naturally vāsanāḥ; keep smṛti–saṁskāra linked via one-form identity)
- anāditvam: beginninglessness (no first inception in time)
- āśiṣaḥ: of desire/wish/aspiration (aiming tendency)
- nityatvāt: because-of constancy/eternality

Thesis:
The latent impressions (vāsanā/saṁskāra) are beginningless because the desiderative vector (āśiṣ) is constant.
This prepares the Appearance facticity law (IV.11).
*/

// ---------- Grammar ----------
export const YS_IV_10_GRAMMAR = {
  tasamForm: 'tāsām: genitive feminine plural (“of those”)',
  tasamLikelyReferent: 'Refers to vāsanāḥ (fem. pl.); carry saṁskāra via one-form identity with smṛti',
  samskaraGenderNote: 'saṁskāra is masc.; gen. pl. would be teṣām',
}

// ---------- Ontology (merged; single source of truth) ----------
export const YS_IV_10_ONTOLOGY = {
  // IV.10 core
  tasamReferent: 'Map “tāsām” to the IV.9 pair; operationally target vāsanā/saṁskāra',
  anaditvam: 'Beginninglessness: no temporal first-instance',
  asisa: 'Āśiṣ: desiderative aim/wish vector',
  nityatva: 'Constancy/eternality (here: of āśiṣ)',
  samskaraSet: 'Latent impression set (vāsanā/saṁskāra) supporting memory readouts',
  beginninglessSamskaraClaim: 'samskaraSet is beginningless due to āśiṣ nityatva',
  noFirstCauseGuard: 'Forbids absolute first efficient cause inside temporal series',
  transitionToFacticity: 'Bridge to IV.11 hetu–phala–āśraya–ālambana (Appearance facticity)',
  wishShouldBridge: 'Āśiṣ (aim) as modalized trace of the categorical “should”',

  // Lecture 18 — formal genesis and certainty
  invarianceCertainty: 'Certainty as the modality of invariant content (nityatva)',
  constantContent: 'Seeing’s material content remains; only form is further determined',
  formalGenesisSeeing: 'Inner genesis of seeing (formal determination) without altering content',
  alteredEye: 'Change resides in the mode of seeing, not in the seen content',
  fivefoldRelation: 'The “through” (five-fold synthesis) installed by the “should”',
  shouldGenesisPrinciple: '“Should” functions as genesis-principle for formal determination',
  newIdealismAppearance: 'Formal-genesis idealism = principle of appearance (not being-in-itself)',
  hypotheticalOriginalSeeing: 'Original seeing kept hypothetical regarding content',
  certaintyOfInsight: 'Distinguished certainty attaches to the formal-insight relation',
  aimInvarianceBridge: 'Āśiṣ as invariant aim supports anāditva',

  // Lecture 18 — realism turn
  realismTurn: 'Shift to realism to avoid idealist circularity',
  pureReasonEqualsInnerBeing: 'Pure reason is immediately inner being (same singularity)',
  remainsAfterAbstraction: 'What remains absolutely after total abstraction (pure light/reason/being)',
  antiIdealistTransfer: 'Do not transfer the We’s freedom to reason’s emergence',
  onenessInseparable: 'Absolutely self-contained singularity; no internal disjunction',
  selfConstructionIdentity: 'Being cannot be posited “as such” without constructing itself',
  notArbitraryPosit: 'Being/reason is not arbitrarily posited',
  ofItselfNecessity: 'If it remains after abstraction, it is of itself (not of another)',
  samskaraAsBeingFocus: 'Treat saṁskāra’s invariance (via āśiṣ nityatva) as its being-aspect',
  certaintyAsInvariance: 'Prefer certainty of content invariance over procedural genesis',

  // Pair oneness and routes of insight
  tasamGenFemPlural: 'Resolve feminine plural by vāsanā while preserving pair-oneness',
  pairOnenessOperator: 'Identify smṛti–saṁskāra as one-form identity (eka-rūpatva) in operation',
  immediateInsightBeing: 'Immediate insight: being must construct itself (no presupposition)',
  mediateInsightPath: 'Mediate insight via presupposed constructive act (idealist path)',
  idealismRefutedHere: 'Immediate route exists; idealism’s need of presupposition is refuted',
  twoWaysSameInsight: 'Same insight available immediately and mediately',
  unityBeingReasonLight: 'Being ≡ reason ≡ light; self-positing = self-construction',
  realIdealDistinctionAnnulled: 'Earlier real vs ideal self-construction distinction annulled at unity',
  deriveIfReturn: 'If reintroduced, the distinction must be derived, not presupposed',
  residualDualityAsMeans: 'Residual duality remains only as a means to oneness',
  projectionOfShouldBack: 'A “projection of the should” may be posited backward as scaffold',
  circleReadinessPoint: 'We stand at the precise place where completion becomes possible',

  // Supplementary (Jacobi)
  jacobiReconstructionOnly: 'We can only reconstruct what originally exists',
  philosophyRevealsBeing: 'Philosophy should reveal being in and of itself',
  jacobiNoPhilosophy: 'Therefore no philosophy (if only reconstruction is possible)',
  performativeLiftOut: 'Universalizing “we can only reconstruct” lifts beyond reconstruction (self-contradiction)',
  pureReasonBeyondI: 'Philosophy = pure reason beyond the empirical We/I',
  possibilityCondition: 'Possibility hinges on the I’s perishing so reason manifests purely',
  originalPriorToReconstruction: 'Understanding “reconstruction” presupposes an original',
  systemTaskOriginalLaw: 'Grasp the original and the law reconstruction follows from it',
  immediacyRefutesIdealism: 'Immediate insight into being’s self-construction removes need of presupposition',
  twoRoutesOneInsight: 'Two routes (mediate/immediate) to one insight',
  pairOnenessAffirmed: 'Vāsanā–saṁskāra oneness mirrors unity of being/reason/light',
}

// ---------- Chunks (single accumulator) ----------
const CHUNKS_YS_IV_10 = [
  { id: 'ys-iv-10-text', title: 'IV.10 Text & Baseline', summary: 'Beginninglessness of vāsanā/saṁskāra due to āśiṣ nityatva.' },
  { id: 'ys-iv-10-referent', title: 'Referent of “tāsām”', summary: 'Gen. fem. pl.; prefer vāsanā; preserve one-form identity with smṛti–saṁskāra.' },
  { id: 'ys-iv-10-reason', title: 'Reason: Āśiṣ Nityatva', summary: 'Constant desiderative aim furnishes standing condition → no first inception.' },
  { id: 'ys-iv-10-guards', title: 'Guards & Bridges', summary: 'No-first-cause guard; bridge to facticity (IV.11) and to the “should.”' },
  { id: 'ys-iv-10-fichte18', title: 'Fichte L18 Seed', summary: 'Drive (Trieb): aim → inscription (supports transition to facticity).' },

  // L18: formal genesis and certainty
  { id: 'ys-iv-10-invariance-certainty', title: 'Invariance and Certainty', summary: 'Certainty tracks invariant content; nityatva grounds anāditva.' },
  { id: 'ys-iv-10-formal-genesis', title: 'Formal Genesis of Seeing', summary: 'Further determine mode of seeing; content/signature remains constant.' },
  { id: 'ys-iv-10-idealism-bridge', title: 'Idealism as Appearance', summary: '“Should” installs the fivefold relation; idealism scoped to appearance.' },
  { id: 'ys-iv-10-transition-iv11', title: 'Transition to IV.11', summary: 'From invariant aim → factical dependency-set and absence rule.' },

  // Realism turn
  { id: 'ys-iv-10-realism-turn', title: 'Realism Turn', summary: 'Leave the idealist circle; pure reason = inner being as left-over.' },
  { id: 'ys-iv-10-leftover', title: 'Left-over After Abstraction', summary: 'Pure light/reason/being remains by itself after complete abstraction.' },
  { id: 'ys-iv-10-self-construction', title: 'Self-Construction ≡ Being', summary: 'Being “as such” coincides with constructing itself; not arbitrarily posited.' },
  { id: 'ys-iv-10-samskara-being', title: 'Saṁskāra as Being', summary: 'Nityatva of āśiṣ → anāditva; saṁskāra’s being-aspect foregrounded.' },
  { id: 'ys-iv-10-certainty', title: 'Certainty as Invariance', summary: 'Prefer certainty of content invariance to certainty of formal genesis.' },

  // Pair-oneness and routes
  { id: 'ys-iv-10-grammar', title: 'Grammar: tāsām', summary: 'Resolve feminine plural by vāsanā; keep pair-oneness.' },
  { id: 'ys-iv-10-immediate-vs-mediate', title: 'Immediate vs Mediate', summary: 'Immediate insight refutes presupposition-need.' },
  { id: 'ys-iv-10-two-ways', title: 'Two Ways to One Insight', summary: 'Philosophical vs common knowing as degrees of mediation.' },
  { id: 'ys-iv-10-unity', title: 'Unity of Being/Reason/Light', summary: 'Self-positing and self-construction coincide; annul prior split.' },
  { id: 'ys-iv-10-residual-duality', title: 'Residual Duality (as Means)', summary: 'Duality persists only as means; “projection of the should” backward.' },

  // Supplementary (Jacobi)
  { id: 'ys-iv-10-jacobi-setup', title: 'Jacobi: Setup', summary: 'Only reconstruction; philosophy must reveal being; thus (he says) no philosophy.' },
  { id: 'ys-iv-10-jacobi-critique', title: 'Jacobi: Critique', summary: 'Universal “we can only reconstruct” self-undercuts via lift beyond reconstruction.' },
  { id: 'ys-iv-10-philo-possibility', title: 'Possibility of Philosophy', summary: 'Philosophy = pure reason beyond I; hinges on the I’s perishing.' },
  { id: 'ys-iv-10-immediate-mediate-supp', title: 'Immediate vs Mediate (Supp.)', summary: 'Two routes to one insight; immediate route refutes idealist need.' },
  { id: 'ys-iv-10-pair-oneness-link', title: 'Pair-Oneness Link', summary: 'Vāsanā–saṁskāra oneness mirrors unity of being/reason/light.' },
]

// ---------- HLO Clauses (single array) ----------
const HLOS_YS_IV_10 = [
  // Baseline
  { id: 'ys-iv-10-hlo-baseline', chunkId: 'ys-iv-10-text', label: 'Baseline', clauses: [
    "tag('sutra','IV.10')",
    'assert(beginninglessSamskaraClaim)',
  ]},

  // Core referent/reason/guards
  { id: 'ys-iv-10-hlo-referent', chunkId: 'ys-iv-10-referent', label: 'Referent', clauses: [
    'tasamReferent := resolve(tāsām → samskaraSet from IV.9)',
    'pairOnenessOperator := assert(ekaRupatva(smṛti, saṁskāra))',
  ]},
  { id: 'ys-iv-10-hlo-reason', chunkId: 'ys-iv-10-reason', label: 'Reasoning', clauses: [
    'nityatva := constancy(asisa)',
    'anaditvam(samskaraSet) ⇐ nityatva',
    'noFirstCauseGuard := forbid(firstCauseInTemporalSeries)',
  ]},
  { id: 'ys-iv-10-hlo-guards', chunkId: 'ys-iv-10-guards', label: 'Guards/Bridges', clauses: [
    'transitionToFacticity := prepare(IV_11_absenceRule ∧ dependencySet)',
    'wishShouldBridge := map(asisa ↔ modalizedShould)',
  ]},
  { id: 'ys-iv-10-hlo-fichte18', chunkId: 'ys-iv-10-fichte18', label: 'L18 Seed', clauses: [
    'fichte18_driveKernel := operator(aim → inscription(saṁskāra))',
    'support(transitionToFacticity) ⇐ fichte18_driveKernel',
  ]},

  // L18: certainty and formal genesis
  { id: 'ys-iv-10-hlo-invariance-certainty', chunkId: 'ys-iv-10-invariance-certainty', label: 'Certainty', clauses: [
    'invarianceCertainty := map(nityatva ↔ certaintyOfInsight)',
    'constantContent := hold(signature(samskaraSet) == signature(samskaraSet))',
    'aimInvarianceBridge := assert(nityatva(asisa) ⇒ anaditvam(samskaraSet))',
  ]},
  { id: 'ys-iv-10-hlo-formal-genesis', chunkId: 'ys-iv-10-formal-genesis', label: 'Formal Genesis', clauses: [
    'formalGenesisSeeing := determine(Form(seeing))',
    'alteredEye := locus(change, seeingMode)',
    'objectivityUnchanged := assert(not(change(contentSignature)))',
  ]},
  { id: 'ys-iv-10-hlo-idealism-bridge', chunkId: 'ys-iv-10-idealism-bridge', label: 'Appearance Idealism', clauses: [
    'shouldGenesisPrinciple := base(formalGenesisSeeing, “should”)',
    'fivefoldRelation := install(relationThrough)',
    'newIdealismAppearance := scope(idealism, appearanceOnly)',
    'hypotheticalOriginalSeeing := status(originalSeeing, hypothetical)',
    'certaintyOfInsight := status(formalInsight, certain)',
  ]},
  { id: 'ys-iv-10-hlo-transition-iv11', chunkId: 'ys-iv-10-transition-iv11', label: 'Bridge', clauses: [
    'prepare(IV_11_absenceRule) ⇐ (formalGenesisSeeing ∧ shouldGenesisPrinciple)',
    'link(beginninglessSamskaraClaim → dependencySetPrereq)',
  ]},

  // Realism turn
  { id: 'ys-iv-10-hlo-realism-turn', chunkId: 'ys-iv-10-realism-turn', label: 'Realism Move', clauses: [
    'realismTurn := shift(idealism → realism)',
    'antiIdealistTransfer := guard(not(transfer(freedom(We), emergence(reason))))',
  ]},
  { id: 'ys-iv-10-hlo-leftover', chunkId: 'ys-iv-10-leftover', label: 'Left-over', clauses: [
    'remainsAfterAbstraction := result(completeAbstraction) == pureReason',
    'pureReasonEqualsInnerBeing := alias(pureReason, innerBeing)',
    'onenessInseparable := assert(selfContainedSingularity(pureReason))',
  ]},
  { id: 'ys-iv-10-hlo-self-construction', chunkId: 'ys-iv-10-self-construction', label: 'Identity Law', clauses: [
    'notArbitraryPosit := negate(arbitraryPosit(being))',
    'ofItselfNecessity := rule(remainsAfterAbstraction ⇒ ofItself(being))',
    'selfConstructionIdentity := assert(posit(being as such) ≡ construct(being))',
  ]},
  { id: 'ys-iv-10-hlo-samskara-being', chunkId: 'ys-iv-10-samskara-being', label: 'Saṁskāra ↔ Being', clauses: [
    'samskaraAsBeingFocus := map(nityatva(asisa) → anaditvam(samskaraSet) as beingAspect)',
    'link(beginninglessSamskaraClaim, certaintyAsInvariance)',
  ]},
  { id: 'ys-iv-10-hlo-certainty', chunkId: 'ys-iv-10-certainty', label: 'Certainty', clauses: [
    'certaintyAsInvariance := prefer(certainty(contentInvariance) over(certainty(formalGenesis)))',
  ]},

  // Pair-oneness and routes
  { id: 'ys-iv-10-hlo-grammar', chunkId: 'ys-iv-10-grammar', label: 'Grammar', clauses: [
    'tasamGenFemPlural := parse(tāsām)',
    'vasanaPreferredReferent := choose(vāsanāḥ)',
    'pairOnenessOperator := assert(ekaRupatva(smṛti, saṁskāra))',
  ]},
  { id: 'ys-iv-10-hlo-immediate-vs-mediate', chunkId: 'ys-iv-10-immediate-vs-mediate', label: 'Paths of Insight', clauses: [
    'immediateInsightBeing := see(beingMustConstructItself, without(presupposition))',
    'mediateInsightPath := via(presupposition(constructiveAct))',
    'idealismRefutedHere := conclude(immediateInsightBeing ∧ not(need(presupposition)))',
  ]},
  { id: 'ys-iv-10-hlo-two-ways', chunkId: 'ys-iv-10-two-ways', label: 'Standpoints', clauses: [
    'twoWaysSameInsight := {immediateInsightBeing, mediateInsightPath}',
    'map(standpoints, {philosophical: immediate, common: mediate(degrees)})',
  ]},
  { id: 'ys-iv-10-hlo-unity', chunkId: 'ys-iv-10-unity', label: 'Unity', clauses: [
    'unityBeingReasonLight := assert(being ≡ reason ≡ light)',
    'realIdealDistinctionAnnulled := annul(distinction(realSelfConstruction, idealSelfConstruction))',
    'deriveIfReturn := require(derivation(before(reintroduce(distinction))))',
  ]},
  { id: 'ys-iv-10-hlo-residual-duality', chunkId: 'ys-iv-10-residual-duality', label: 'Means to Oneness', clauses: [
    'residualDualityAsMeans := hold(duality only_as meansToOneness)',
    'projectionOfShouldBack := positBackward(“should”, as conditionalScaffold)',
    'circleReadinessPoint := mark(placeOfCompletion)',
  ]},

  // Supplementary (Jacobi)
  { id: 'ys-iv-10-hlo-jacobi-setup', chunkId: 'ys-iv-10-jacobi-setup', label: 'Setup', clauses: [
    'jacobiReconstructionOnly := postulate("we can only reconstruct")',
    'philosophyRevealsBeing := goal(philosophy, reveal(being_in_itself))',
    'jacobiNoPhilosophy := conclude(jacobiReconstructionOnly ∧ philosophyRevealsBeing)',
  ]},
  { id: 'ys-iv-10-hlo-jacobi-critique', chunkId: 'ys-iv-10-jacobi-critique', label: 'Critique', clauses: [
    'performativeLiftOut := detect(universalClaim(jacobiReconstructionOnly) ⇒ beyond(reconstruction))',
    'originalPriorToReconstruction := require(original(≺ reconstruction))',
    'systemTaskOriginalLaw := task(grasp(original) ∧ law(reconstruction ⟵ original))',
  ]},
  { id: 'ys-iv-10-hlo-philo-possibility', chunkId: 'ys-iv-10-philo-possibility', label: 'Beyond the I', clauses: [
    'pureReasonBeyondI := define(philosophy == pureReason | beyond(empiricalWe))',
    'possibilityCondition := reduce(question(possible(philosophy)) to possible(perish(I)))',
  ]},
  { id: 'ys-iv-10-hlo-immediate-mediate-supp', chunkId: 'ys-iv-10-immediate-mediate-supp', label: 'Two Routes', clauses: [
    'immediacyRefutesIdealism := assert(immediateInsightBeing ∧ not(need(presupposition)))',
    'twoRoutesOneInsight := set({immediateInsightBeing, mediateInsightPath})',
  ]},
  { id: 'ys-iv-10-hlo-pair-oneness-link', chunkId: 'ys-iv-10-pair-oneness-link', label: 'Oneness Link', clauses: [
    'pairOnenessAffirmed := assert(ekaRupatva(vāsanā, saṁskāra))',
    'map(pairOnenessAffirmed ↔ unityBeingReasonLight)',
  ]},
]

// ---------- Export Unit ----------
export const YS_IV_10_UNIT: DatasetUnit = {
  id: makeUnitId('ys-iv-10'),
  title: 'YS IV.10 — tāsām anāditvaṁ cāśiṣo nityatvāt',
  scope: 'appearance',
  logosMode: 'appearance',
  synthesis: 'beginninglessness-via-desiderative-constancy',
  faculty: 'buddhi',
  lens: 'yoga',
  chunks: CHUNKS_YS_IV_10 as any,
  hlos: HLOS_YS_IV_10 as any,
}

// Optional: single consolidated symbol list (avoids fragmented *_EXT exports)
export const YS_IV_10_SYMBOLS = [
  'invarianceCertainty',
  'constantContent',
  'formalGenesisSeeing',
  'alteredEye',
  'fivefoldRelation',
  'shouldGenesisPrinciple',
  'newIdealismAppearance',
  'hypotheticalOriginalSeeing',
  'certaintyOfInsight',
  'aimInvarianceBridge',
  'realismTurn',
  'pureReasonEqualsInnerBeing',
  'remainsAfterAbstraction',
  'antiIdealistTransfer',
  'onenessInseparable',
  'selfConstructionIdentity',
  'notArbitraryPosit',
  'ofItselfNecessity',
  'samskaraAsBeingFocus',
  'certaintyAsInvariance',
  'tasamGenFemPlural',
  'vasanaPreferredReferent',
  'pairOnenessOperator',
  'immediateInsightBeing',
  'mediateInsightPath',
  'idealismRefutedHere',
  'twoWaysSameInsight',
  'unityBeingReasonLight',
  'realIdealDistinctionAnnulled',
  'deriveIfReturn',
  'residualDualityAsMeans',
  'projectionOfShouldBack',
  'circleReadinessPoint',
  'jacobiReconstructionOnly',
  'philosophyRevealsBeing',
  'jacobiNoPhilosophy',
  'performativeLiftOut',
  'pureReasonBeyondI',
  'possibilityCondition',
  'originalPriorToReconstruction',
  'systemTaskOriginalLaw',
  'immediacyRefutesIdealism',
  'twoRoutesOneInsight',
  'pairOnenessAffirmed',
]
