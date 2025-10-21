import { DatasetUnit, makeUnitId } from '@organon/gdsl/registry/canon';

export const YS_III_54_ONTOLOGY = {
  // Sutra and core terms (ascii transliteration)
  iii54Sutra:
    'III.54 — jati-laksana-desi anyata-anavacchedat tulyayos tatah pratipatti: When there is no delimitation by class, mark, or place, recognition of sameness between the two arises.',
  iii54Jati: 'jati — class/kind (generic determination)',
  iii54Lakshana: 'lakshana — mark/characteristic (specific determination)',
  iii54Desha: 'desha — place/position (spatial-temporal locus)',
  iii54AnyataAnavacchedat:
    'anyata-anavacchedat — absence of delimitation by otherness (no cutting off by difference)',
  iii54Tulyayos:
    'tulyayos — of the two as equal (sameness across a pair: moments, streams, or objects)',
  iii54TatahPratipatti:
    'tatah pratipatti — therefore recognition/comprehension (of sameness)',

  // Criterion and application
  iii54SamenessCriterion:
    'Criterion: if no difference by class, characteristic, or place delimits, then recognize sameness',
  iii54ApplicationToMoments:
    'Application: extends III.53 — moments and sequences are recognized as the same where jati/lakshana/desha do not delimit',
  iii54ContinuityRecognition:
    'Continuity recognition: identity across flow when delimiting predicates are null',

  // Fichte L26 overlay — grounding recognition in self-enclosure of certainty
  iii54RecognitionFromSelfEnclosure:
    'Recognition of sameness is grounded in immanent self-enclosure of certainty (pure ideal seeing)',
  iii54NonDelimitationAsSelfNegation:
    'Non-delimitation corresponds to projection that self-recognizes and negates itself, positing the inwardly same',
  iii54ExpressionGroundsPratipatti:
    'Formal certainty as expression grounds pratipatti (recognition) without appeal to external marks',
  iii54PrimordialLawOperative:
    'Primordial description-law (for certainty) operates as the rule of recognition in absence of external delimiters',
  iii54WeConstitutionInRecognition:
    'The We is constituted in the act of recognition that preserves sameness under negation of superficial projection',

  // L26 §1 — Living self-enclosure, act/drive, truth/appearance, quality/quantity
  iii54LivingEnclosingPositsAct:
    'If being’s “living self-enclosing” is literal, it posits an act — an absolute act qua act (effect of the drive to come out of itself)',
  iii54AnnulsDriveWithoutResistance:
    'This act/drive is to be annulled without resistance in truth',
  iii54LivingnessAsResultOfSeeing:
    'Whole “livingness” is only a result of seeing’s inner expression/projection (as such and as intuition)',
  iii54IntuitionOverthrownAsValid:
    'Intuition has been overthrown as valid in itself, though factically it persists',
  iii54TruthAppearanceUnified:
    'Unification of truth in itself and its appearance restores a true ground for the oneness of quality',
  iii54DriveEmergesAppearanceStruckDown:
    'The drive to come out of oneself always emerges in appearance and is struck down; it can form the appearance of freedom/genesis, yet does not enter truth',
  iii54DriveGroundsQuantities:
    'This drive provides the true real ground for quantities',
  iii54UnifyQuantityWithQualityForPrimordialOrdinaryKnowing:
    'By unifying these quantities with the principle of quality, we aim to deduce the phenomenon of ordinary knowing’s primordial form',

  // L26 §2 — Being in absolute seeing; certainty; objectivity/genesis; manifesting principle
  iii54BeingResidesInAbsoluteSeeing:
    'Being resides simply in absolute seeing itself and is to be sought only there (in truth as in appearance)',
  iii54NotOutsideKnowingButKnowingIsInconceivable:
    'Inconceivability is not because being lies outside knowing, but because absolute knowing itself is inconceivable; absolute conceiving ≠ absolute knowing',
  iii54DirectInsightSeeingAsExternalityCertain:
    'Directly: seeing is externality, and this was certain to us in seeing of seeing (insight)',
  iii54CertaintyAsInnerEnclosureAgainstDrive:
    'Certainty described as inner self-enclosing against the principle of coming out of oneself',
  iii54PrimordialDescriptionCompletion:
    'This is the primordial description — authentic completion of the one certainty/knowing (not a mere re-description)',
  iii54BeingArisesFromNegationWithinKnowing:
    'Being arising from negation lies within knowing’s original description and its further syntheses',
  iii54KnowingSustainOneStandpoint:
    'Necessity: knowing must hold and sustain itself within its one unchangeable standpoint (independence)',
  iii54AppearanceOfFreeCreation:
    'The reflection appears as freely created and objectified by us; nevertheless must be explained as appearance',
  iii54ObjectivityEqualsGenesis:
    'Objectivity and genesis are entirely one; genesis is the inner externalization (principle of this oneness)',
  iii54NeedSpecificManifestingPrinciple:
    'Therefore a specific manifesting principle is required, to which the recapitulated is related as to a containing oneness',
  iii54MutualDerivationOnenessAndPrinciple:
    'This manifesting principle can only be derived from the enclosing oneness, and the latter only from it (reciprocal derivation)',
} as const;

const CHUNKS_YS_III_54 = [
  {
    id: 'ys-iii-54-sutra',
    title: 'III.54 — Sameness by Non-Delimitation',
    summary:
      'No delimitation by class/mark/place → recognition (pratipatti) of sameness between two.',
  },
  {
    id: 'ys-iii-54-criterion',
    title: 'Criterion and Application',
    summary:
      'Operational criterion; apply to moments/sequence from III.53 to read continuity/identity.',
  },
  {
    id: 'ys-iii-54-fichte-overlay',
    title: 'Fichte Overlay: Self-Enclosure and Recognition',
    summary:
      'Ground recognition in pure seeing/self-enclosure; non-delimitation as self-negation of projection.',
  },
  {
    id: 'ys-iii-54-bridge',
    title: 'Bridge to III.55',
    summary:
      'Stabilized sameness readies the higher integration of knowledge in the next sutra.',
  },
  {
    id: 'ys-iii-54-remarks-1-2',
    title: 'Remarks 1-2: Seeing, Self-Negation, Higher Being as Expression',
    summary:
      'Seeing as external emanence annuls itself; persistence as expression; higher being expresses itself absolutely.',
  },
  {
    id: 'ys-iii-54-remark-3',
    title: 'Remark 3: Pure Light and Its Genesis',
    summary:
      'Seeing’s self-negation → pure light; pure light “just is”; emerges only in insight via absolute self-negation.',
  },
  {
    id: 'ys-iii-54-remarks-4-5',
    title: 'Remarks 4-5: Grounds of Negation and Fate of Intuition',
    summary:
      'Negation because seeing is expression of another; what is negated = absolute intuition; intuition persists but is to be negated relative to essence.',
  },

  // NEW — Four interrelated terms (and the fifth condition)
  {
    id: 'ys-iii-54-four-terms',
    title: 'Four Terms: Seeing/Being, Twofold Views, Drive',
    summary:
      'Term 1–2: seeing as externality/self-negation ↔ being as inward enclosure/life; Term 3: twofold seeing/being and their integration; Term 4: drive.',
  },
  {
    id: 'ys-iii-54-fifth-term',
    title: 'Fifth Term: Synthesis Condition (Freedom-Dependence)',
    summary:
      'Condition for arising/persistence of the four terms: grasp externality, negate as immanence, preserve freedom-dependent factors in the certainty-description.',
  },
  {
    id: 'ys-iii-54-prep-syntheses',
    title: 'Prep: Living Enclosure → Act/Drive; Truth/Appearance; Quality/Quantity',
    summary:
      'Living self-enclosure posits absolute act/drive; annulled in truth; unifies truth/appearance to restore quality; drive grounds quantity toward ordinary knowing’s primordial form.',
  },
  {
    id: 'ys-iii-54-being-in-seeing',
    title: 'Being in Absolute Seeing; Primordial Description of Certainty',
    summary:
      'Being sought only in absolute seeing; certainty as inner enclosure against drive; primordial description completes knowing.',
  },
  {
    id: 'ys-iii-54-manifesting-principle',
    title: 'Objectivity ≡ Genesis; Need a Manifesting Principle',
    summary:
      'Objectivity and genesis are one; genesis as inner externalization; posit a specific manifesting principle in reciprocal derivation with enclosing oneness.',
  },
] as const;

const HLOS_YS_III_54 = [
  {
    id: 'ys-iii-54-hlo-sutra',
    chunkId: 'ys-iii-54-sutra',
    label: 'Sutra',
    clauses: [
      'define(iii54Sutra)',
      'define(iii54Jati)',
      'define(iii54Lakshana)',
      'define(iii54Desha)',
      'define(iii54AnyataAnavacchedat)',
      'define(iii54Tulyayos)',
      'conclude(iii54TatahPratipatti)',
    ],
  },
  {
    id: 'ys-iii-54-hlo-criterion',
    chunkId: 'ys-iii-54-criterion',
    label: 'Criterion',
    clauses: [
      'define(iii54SamenessCriterion)',
      'define(iii54ApplicationToMoments)',
      'conclude(iii54ContinuityRecognition)',
      'link(iii53Kshana → iii54ApplicationToMoments)',
      'link(iii53Krama → iii54ApplicationToMoments)',
      'link(iii53VivekaJamJnanam → iii54ContinuityRecognition)',
    ],
  },
  {
    id: 'ys-iii-54-hlo-fichte',
    chunkId: 'ys-iii-54-fichte-overlay',
    label: 'Fichte',
    clauses: [
      'define(iii54RecognitionFromSelfEnclosure)',
      'define(iii54NonDelimitationAsSelfNegation)',
      'define(iii54ExpressionGroundsPratipatti)',
      'note(iii54PrimordialLawOperative)',
      'note(iii54WeConstitutionInRecognition)',
      'link(iii53CertaintyAsImmanentSelfEnclosure → iii54RecognitionFromSelfEnclosure)',
      'link(iii53DescriptionImmanentProjection → iii54NonDelimitationAsSelfNegation)',
      'link(iii53CertaintyExpressionRequirement → iii54ExpressionGroundsPratipatti)',
    ],
  },
  {
    id: 'ys-iii-54-hlo-bridge',
    chunkId: 'ys-iii-54-bridge',
    label: 'Bridge',
    clauses: ['note(iii54BridgeToIII55)'],
  },
  {
    id: 'ys-iii-54-hlo-remarks-1-2',
    chunkId: 'ys-iii-54-remarks-1-2',
    label: 'R1-R2',
    clauses: [
      'define(iii54R1SeeingAsExternalEmanence)',
      'define(iii54R2ExpressionAsHigherBeing)',
    ],
  },
  {
    id: 'ys-iii-54-hlo-remark-3',
    chunkId: 'ys-iii-54-remark-3',
    label: 'R3',
    clauses: [
      'define(iii54R3PureLight)',
      'note(iii54R3PureLightJustIs)',
      'conclude(iii54R3GenesisOfLight)',
    ],
  },
  {
    id: 'ys-iii-54-hlo-remarks-4-5',
    chunkId: 'ys-iii-54-remarks-4-5',
    label: 'R4-R5',
    clauses: [
      'define(iii54R4NegationGrounds)',
      'define(iii54R4WhatIsNegated)',
      'define(iii54R5IntuitionInInsightToBeNegated)',
      'note(iii54R5OriginInThirdRemark)',
    ],
  },

  // NEW — Four interrelated terms (and the fifth condition)
  {
    id: 'ys-iii-54-hlo-four-terms',
    chunkId: 'ys-iii-54-four-terms',
    label: 'Four Terms',
    clauses: [
      'define(iii54Term1SeeingExternalitySelfNegation)',
      'define(iii54Term1BeingInwardSelfEnclosure)',
      'define(iii54Term2SeeingPersistsAsInwardExternality)',
      'define(iii54Term2BeingCarriesLifeExternality)',
      'define(iii54Term3TwofoldSeeingIntuitionDeadBeing)',
      'define(iii54Term3TwofoldSeeingInwardExternalityLifeBeing)',
      'conclude(iii54Term3BeingLivingSelfEnclosureMergesIntoKnowing)',
      'define(iii54EssenceConstructionAsExternality)',
      'define(iii54ProjectionIntoBeingByNegation)',
      'define(iii54Term4DrivePrinciple)',
      'link(iii54DrivePrinciple → iii54Term4DrivePrinciple)',
    ],
  },
  {
    id: 'ys-iii-54-hlo-fifth-term',
    chunkId: 'ys-iii-54-fifth-term',
    label: 'Fifth Term',
    clauses: [
      'define(iii54Term5SynthesisConditionFreedom)',
      'link(iii54DescriptionAsLivelinessConstruction → iii54Term5SynthesisConditionFreedom)',
      'link(iii53CertaintyExpressionRequirement → iii54Term5SynthesisConditionFreedom)',
    ],
  },
  {
    id: 'ys-iii-54-hlo-prep-syntheses',
    chunkId: 'ys-iii-54-prep-syntheses',
    label: 'Prep §1',
    clauses: [
      'define(iii54LivingEnclosingPositsAct)',
      'define(iii54AnnulsDriveWithoutResistance)',
      'define(iii54LivingnessAsResultOfSeeing)',
      'note(iii54IntuitionOverthrownAsValid)',
      'conclude(iii54TruthAppearanceUnified)',
      'define(iii54DriveEmergesAppearanceStruckDown)',
      'define(iii54DriveGroundsQuantities)',
      'conclude(iii54UnifyQuantityWithQualityForPrimordialOrdinaryKnowing)',
      'link(iii54Term4DrivePrinciple → iii54DriveGroundsQuantities)',
    ],
  },
  {
    id: 'ys-iii-54-hlo-being-in-seeing',
    chunkId: 'ys-iii-54-being-in-seeing',
    label: '§2 Being/Certainty',
    clauses: [
      'define(iii54BeingResidesInAbsoluteSeeing)',
      'define(iii54NotOutsideKnowingButKnowingIsInconceivable)',
      'define(iii54DirectInsightSeeingAsExternalityCertain)',
      'define(iii54CertaintyAsInnerEnclosureAgainstDrive)',
      'conclude(iii54PrimordialDescriptionCompletion)',
      'note(iii54BeingArisesFromNegationWithinKnowing)',
      'assert(iii54KnowingSustainOneStandpoint)',
    ],
  },
  {
    id: 'ys-iii-54-hlo-manifesting-principle',
    chunkId: 'ys-iii-54-manifesting-principle',
    label: 'Manifesting Principle',
    clauses: [
      'define(iii54AppearanceOfFreeCreation)',
      'define(iii54ObjectivityEqualsGenesis)',
      'define(iii54NeedSpecificManifestingPrinciple)',
      'conclude(iii54MutualDerivationOnenessAndPrinciple)',
    ],
  },
] as const;

export const YS_III_54_UNIT: DatasetUnit = {
  id: makeUnitId('ys-iii-54'),
  title: 'YS III.54 — Sameness by Non-Delimitation (Jāti-Lakṣaṇa-Deśa)',
  scope: 'actuality',
  logosMode: 'appearance',
  synthesis:
    'When no difference by class/mark/place delimits, recognition of sameness arises; grounded in self-enclosed certainty and pure seeing, extending III.53 toward III.55.',
  faculty: 'buddhi',
  lens: 'yoga',
  chunks: CHUNKS_YS_III_54 as any,
  hlos: HLOS_YS_III_54 as any,
};

export const YS_III_54_SYMBOLS = Object.keys(YS_III_54_ONTOLOGY);
