import { DatasetUnit, makeUnitId } from '@organon/gdsl/registry/canon'

export const YS_III_53_ONTOLOGY = {
  // Sutra and core terms
  iii53Sutra:
    'III.53 — kṣaṇa-tat-kramayoḥ samyama viveka-jam jñānam: By samyama on the moments and their sequence, knowledge born of discrimination arises.',
  iii53Kshana:
    'kṣaṇa — moment (instant of occurrence; atomic temporal mark)',
  iii53Krama:
    'krama — sequence/order of moments (succession; serial nexus)',
  iii53Samyama:
    'samyama — integrated dhāraṇā–dhyāna–samādhi focused as one act',
  iii53VivekaJamJnanam:
    'viveka-jam jñānam — knowledge born of discrimination (clear discernment through pure conditioning)',

  // Path placement
  iii53JnanaPadaClosure:
    'Jnana Pada (Chapter III) — end of the “middle = pure conditioned” path; discriminative knowledge culminates the middle',
  iii53MiddlePureConditioned:
    'Middle as pure conditioned: genetic mediation without occult quality; conditioning is transparent/lawful',

  // Fichte overlays (ordinary knowing, sublation, LogoGenesis)
  iii53OrdinaryKnowingCrucial:
    'Mastery of ordinary knowing is crucial (Fichte): exposition of representation/reconstruction as groundwork',
  iii53SublationAsMethod:
    'Sublation (Aufhebung) as method: the Concept sublates what it is not; knowing what it is not is essential',
  iii53LogoGenesisConstruction:
    'LogoGenesis: construction/generation of the Concept (not the Concept as premise, but its genesis as act)',
  iii53ConceptAsProductNotPremise:
    'The Concept is resultant (product of genesis), not a presupposed axiom',

  // Practice articulation
  iii53SamyamaOnMoments:
    'Practice: apply samyama first on kṣaṇa (moment) then on their krama (succession)',
  iii53DiscriminationOutcome:
    'Outcome: precise discrimination across moments/flows — a non-finitizing clarity that can govern transitions',

  // Bridges (nonlocal links expected)
  bridgeToMiddleWorld:
    'Bridge: aligns with Middle/Genetic world (appearance’s production) and governs transitions toward Moral',

  // L26 — Formal certainty and reciprocal standpoint
  iii53DeJureHowQuestion:
    'The “how” is de jure (not historical): how have we arrived here as absolute knowing?',
  iii53FormalCertaintyCondition:
    'We are absolute knowing only if we are formally certain of it and express this certainty in fulfilling the knowing',
  iii53ConditionsTranscendentalOrdinary:
    'Conditions split: transcendental (in all determinations) and ordinary; proper standpoint = their reciprocal determination',
  iii53CentralPointMethod:
    'Method: proceed immediately to the indicated central point; fill remaining gaps afterward',
  iii53NewTopicsNotice:
    'This investigation is important and raises entirely new topics',

  // L26 — Certainty as expression; inference modes; higher concept
  iii53CertaintyExpressionRequirement:
    'The content should itself be the expression of absolute knowing (certainty)',
  iii53InferenceFromCertaintyToSaying:
    'Inference mode A: We are certain; therefore what we say (here) is certain — expression of inward certainty',
  iii53InferenceFromContentToCertainty:
    'Inference mode B: This is certain (we see into it); therefore we are certain — expressing certainty',
  iii53PresupposeNatureOfCertaintyKnown:
    'Either mode presupposes that the nature of certainty (essence of knowing) is known',

  iii53EssenceFormerOnenessOfQualities:
    'Previously: essence designated as resting-in-itself, a oneness of qualities',
  iii53OnenessLostInForegoing:
    'We cannot assume that oneness here; it has been lost in the foregoing',
  iii53LossViaImageLawExcludingVariability:
    'Loss traced to inner essential quality as image/law of imaging excluding all variability and qualitative similarity (second power)',
  iii53CertaintyAsImmanentSelfEnclosure:
    'Certainty must be characterized by the higher concept: essential, immanent self-enclosure (as absolute being was previously)',

  // L26 §3 — Primordial description (law) and method
  iii53JudgmentPresupposesPrimordialDescription:
    'The judgment “we are certain” presupposes a primordial concept/description of immanent self-enclosure',
  iii53LawForPrimordialDescription:
    'Law for such a primordial description must be presented (understood as primordial)',
  iii53HigherTermCannotDerive:
    'Rising to a higher term than all prior, it cannot be connected to nor explained from earlier terms',
  iii53CreativeIntuitionAppeal:
    'Method: rely on creative intuition, guided toward clarity',
  iii53ClarityInContextFromParts:
    'Clarity will emerge in context if a beginning is made from the parts',

  // L26 — Description = immanent projection; pure ideal seeing; constitution of the “We”
  iii53DescriptionImmanentProjection:
    'Description, as such, is inward, immanent projection of the described',
  iii53NoObjectiveGapProjection:
    'Not an objective projection “through a gap”',
  iii53ProjectionSelfRecognizesNegates:
    'Projection that recognizes itself as projection (superficial), immediately negates itself as such',
  iii53PositDescribedBySelfNegationInwardness:
    'Through this self-negation it posits something described — an inwardness',
  iii53PureIdealSeeingPermeation:
    'Pure ideal seeing (intuition) permeating itself simply as such',
  iii53PermeationNotInItselfButCommended:
    'This permeation is not asserted as “in itself” but commended to the We',
  iii53WeConstitutionThroughPermeation:
    'As the We of the science of knowing: through this permeation you become this We',
  iii53SeeingXMeansNegatingSeeingAsX:
    '“Seeing X” means not regarding the seeing as X (thus negating it)',
  iii53SeeingArisesByAbstractionFromX:
    'In this negation seeing becomes a seeing; something seen arises if it abstracts from X',
  iii53InnerEssencePureSeeingRealized:
    'Inner essence of pure seeing as such is thereby pointed out and realized (if the intuition is followed)',
  iii53ThisIsFirstPoint:
    'This is the first point',
} as const

const CHUNKS_YS_III_53 = [
  {
    id: 'ys-iii-53-sutra',
    title: 'III.53 — Samyama on Moments and Sequence',
    summary:
      'kṣaṇa/krama samyama → viveka-born knowledge; end of the middle/pure-conditioned path.',
  },
  {
    id: 'ys-iii-53-middle',
    title: 'Middle = Pure Conditioned (Closure of Jnana Pada)',
    summary:
      'Discriminative knowledge culminates the Middle; conditioning is lawful and transparent.',
  },
  {
    id: 'ys-iii-53-fichte-overlay',
    title: 'Fichte Overlay: Ordinary Knowing, Sublation, LogoGenesis',
    summary:
      'Master ordinary knowing; Concept as product of genesis; sublate what the Concept is not.',
  },
  {
    id: 'ys-iii-53-practice',
    title: 'Practice: Samyama Protocol',
    summary:
      'Focus on kṣaṇa then krama; attain non-finitizing discrimination to govern transitions.',
  },
  {
    id: 'ys-iii-53-certainty-standpoint',
    title: 'Formal Certainty and Reciprocal Standpoint',
    summary:
      'De jure “how”; formal certainty requirement; conditions: transcendental and ordinary; method: begin at the central point.',
  },
  {
    id: 'ys-iii-53-certainty-expression',
    title: 'Certainty as Expression — Two Inference Modes',
    summary:
      'Either from being-certain to certain-saying, or from certain-content to being-certain; both presuppose knowing certainty’s essence.',
  },
  {
    id: 'ys-iii-53-certainty-self-enclosure',
    title: 'Higher Concept of Certainty: Immanent Self-Enclosure',
    summary:
      'Former “oneness of qualities” is lost; certainty is characterized as essential, immanent self-enclosure.',
  },
  {
    id: 'ys-iii-53-primordial-description',
    title: 'Primordial Description of Self-Enclosure (Law)',
    summary:
      'Judgment “we are certain” presupposes a primordial description; present the law for such a description.',
  },
  {
    id: 'ys-iii-53-method-creative-intuition',
    title: 'Method: Higher Term and Creative Intuition',
    summary:
      'Cannot derive from earlier terms; rely on guided creative intuition; clarity from parts in context.',
  },
  {
    id: 'ys-iii-53-description-immanent',
    title: 'Description as Immanent Projection',
    summary:
      'Description = inward projection that self-negates to posit an inwardly described.',
  },
  {
    id: 'ys-iii-53-pure-seeing',
    title: 'Pure Ideal Seeing and the “We”',
    summary:
      'Self-permeating intuition; “seeing X” as negation; constitution of the We.',
  },
] as const

const HLOS_YS_III_53 = [
  {
    id: 'ys-iii-53-hlo-sutra',
    chunkId: 'ys-iii-53-sutra',
    label: 'Sutra',
    clauses: [
      'define(iii53Sutra)',
      'define(iii53Kshana)',
      'define(iii53Krama)',
      'define(iii53Samyama)',
      'conclude(iii53VivekaJamJnanam)',
    ],
  },
  {
    id: 'ys-iii-53-hlo-middle',
    chunkId: 'ys-iii-53-middle',
    label: 'Middle',
    clauses: [
      'define(iii53JnanaPadaClosure)',
      'define(iii53MiddlePureConditioned)',
      'link(awtGenesisToMiddle → iii53MiddlePureConditioned)',
      'link(mwCausalMiddleWorld → bridgeToMiddleWorld)',
    ],
  },
  {
    id: 'ys-iii-53-hlo-fichte',
    chunkId: 'ys-iii-53-fichte-overlay',
    label: 'Fichte',
    clauses: [
      'define(iii53OrdinaryKnowingCrucial)',
      'define(iii53SublationAsMethod)',
      'define(iii53LogoGenesisConstruction)',
      'conclude(iii53ConceptAsProductNotPremise)',
      'link(l25ImagePositsLaw → iii53LogoGenesisConstruction)',
      'link(l25LivingSelfEnclosedImaging → iii53LogoGenesisConstruction)',
    ],
  },
  {
    id: 'ys-iii-53-hlo-practice',
    chunkId: 'ys-iii-53-practice',
    label: 'Practice',
    clauses: [
      'define(iii53SamyamaOnMoments)',
      'conclude(iii53DiscriminationOutcome)',
      'link(awtMiddleWorld → iii53DiscriminationOutcome)',
      'link(mwAstralTransitionToMoral → iii53DiscriminationOutcome)',
    ],
  },
  {
    id: 'ys-iii-53-hlo-certainty-standpoint',
    chunkId: 'ys-iii-53-certainty-standpoint',
    label: 'Certainty/Standpoint',
    clauses: [
      'define(iii53DeJureHowQuestion)',
      'define(iii53FormalCertaintyCondition)',
      'define(iii53ConditionsTranscendentalOrdinary)',
      'note(iii53CentralPointMethod)',
      'note(iii53NewTopicsNotice)',
    ],
  },
  {
    id: 'ys-iii-53-hlo-certainty-expression',
    chunkId: 'ys-iii-53-certainty-expression',
    label: 'Expression/Inference',
    clauses: [
      'define(iii53CertaintyExpressionRequirement)',
      'define(iii53InferenceFromCertaintyToSaying)',
      'define(iii53InferenceFromContentToCertainty)',
      'conclude(iii53PresupposeNatureOfCertaintyKnown)',
    ],
  },
  {
    id: 'ys-iii-53-hlo-certainty-self-enclosure',
    chunkId: 'ys-iii-53-certainty-self-enclosure',
    label: 'Self-Enclosure',
    clauses: [
      'note(iii53EssenceFormerOnenessOfQualities)',
      'warn(iii53OnenessLostInForegoing)',
      'define(iii53LossViaImageLawExcludingVariability)',
      'conclude(iii53CertaintyAsImmanentSelfEnclosure)',
      'link(l25OnenessImageLawRepelsVariability → iii53LossViaImageLawExcludingVariability)',
    ],
  },
  {
    id: 'ys-iii-53-hlo-primordial-description',
    chunkId: 'ys-iii-53-primordial-description',
    label: 'Primordial/Law',
    clauses: [
      'define(iii53JudgmentPresupposesPrimordialDescription)',
      'define(iii53LawForPrimordialDescription)',
      'link(iii53CertaintyAsImmanentSelfEnclosure → iii53JudgmentPresupposesPrimordialDescription)',
    ],
  },
  {
    id: 'ys-iii-53-hlo-method-creative-intuition',
    chunkId: 'ys-iii-53-method-creative-intuition',
    label: 'Method',
    clauses: [
      'define(iii53HigherTermCannotDerive)',
      'define(iii53CreativeIntuitionAppeal)',
      'note(iii53ClarityInContextFromParts)',
    ],
  },
  {
    id: 'ys-iii-53-hlo-description-immanent',
    chunkId: 'ys-iii-53-description-immanent',
    label: 'Immanent Description',
    clauses: [
      'define(iii53DescriptionImmanentProjection)',
      'define(iii53NoObjectiveGapProjection)',
      'define(iii53ProjectionSelfRecognizesNegates)',
      'conclude(iii53PositDescribedBySelfNegationInwardness)',
      'link(iii53CertaintyAsImmanentSelfEnclosure → iii53DescriptionImmanentProjection)',
    ],
  },
  {
    id: 'ys-iii-53-hlo-pure-seeing',
    chunkId: 'ys-iii-53-pure-seeing',
    label: 'Pure Seeing / We',
    clauses: [
      'define(iii53PureIdealSeeingPermeation)',
      'note(iii53PermeationNotInItselfButCommended)',
      'define(iii53WeConstitutionThroughPermeation)',
      'define(iii53SeeingXMeansNegatingSeeingAsX)',
      'conclude(iii53SeeingArisesByAbstractionFromX)',
      'conclude(iii53InnerEssencePureSeeingRealized)',
      'note(iii53ThisIsFirstPoint)',
    ],
  },
] as const

export const YS_III_53_UNIT: DatasetUnit = {
  id: makeUnitId('ys-iii-53'),
  title: 'YS III.53 — Samyama on Moments and Sequence (Viveka-born Knowledge)',
  scope: 'actuality',
  logosMode: 'appearance',
  synthesis:
    'By samyama on moments and their succession, discriminative knowledge arises — the closure of the middle/pure-conditioned path; aligns with Fichte’s genetic method (LogoGenesis, sublation).',
  faculty: 'buddhi',
  lens: 'yoga',
  chunks: CHUNKS_YS_III_53 as any,
  hlos: HLOS_YS_III_53 as any,
}

export const YS_III_53_SYMBOLS = Object.keys(YS_III_53_ONTOLOGY)
