import { DatasetUnit, makeUnitId } from '@organon/gdsl/registry/canon'

/*
YS IV.4  (Essence / Cogito Science Axis)
nirmāṇa-cittāni asmitā-mātra
*/

// ---------- Baseline Ontology ----------
export const YS_IV_4_ONTOLOGY = {
  nirmanaCitta: 'Constructed cognition-stream instance (internally modulated)',
  nirmanaSet: 'Population of derivative citta-constructions',
  asmitAParameter: 'Minimal pure I-ness parameter (structural identity, not empirical ego)',
  mereIParameterization: 'Exclusion of extrinsic predicates; “nothing but” I-parameter modulation',
  cogitoModulationLaw: 'Law: plurality of constructed cittas = internal differentiations of asmitā alone',
  internalGenerationField: 'Domain in which asmitā self-differentiates into nirmanaCitta instances',
  externalCauseNegation: 'Denial of external efficient producer of cognition-streams',
  identityContinuum: 'Underlying self-sameness across constructed multiplicity',
  modulationVector: 'Specific transformation spec generating a nirmanaCitta from asmitā base',
  reflectiveShadowLogic: 'Later logical layer (tamasic) = attenuated after-image of primary modulation',
  primarySelfLuminosity: 'Sattvic structural brilliance: pure identity capable of internal variation',
  pseudoPlurality: 'Apparent multiplicity of minds; ontologically a single parameter’s indexed folds',
  ownershipIllusion: 'Mistaking constructed stream index for an independent ego-subject',
  fichteParallelSelfConstruction: 'Crosswalk: self-construction = asmitā-driven modulation',
  logicalDerivativeStratum: 'Logic as secondary formalization of already-modulated identity patterns',
}

// ---------- Baseline Chunks ----------
const CHUNKS_YS_IV_4 = [
  {
    id: 'ys-iv-4-text',
    title: 'IV.4 Text & Baseline',
    summary: 'Constructed cognition-streams are nothing but pure I-parameter (asmitā-mātra) modulations.',
  },
  {
    id: 'ys-iv-4-law',
    title: 'Law of Cogito Modulation',
    summary: 'Plurality of nirmāṇa-cittāni arises internally—no external efficient cause.',
  },
  {
    id: 'ys-iv-4-structure',
    title: 'Structure of Construction',
    summary: 'Each citta = modulationVector(asmitā); pseudo-plurality via indexed internal differentiation.',
  },
  {
    id: 'ys-iv-4-crosswalk',
    title: 'Crosswalk (Fichte / Logic)',
    summary: 'Self-construction (Fichte) and pre-logical identity field align with asmitā-mātra basis.',
  },
  {
    id: 'ys-iv-4-errors',
    title: 'Misread Errors',
    summary: 'Errors: ownership illusion; external cause projection; reifying plurality as ontic.',
  },
]

// ---------- Baseline HLOs ----------
const HLOS_YS_IV_4 = [
  {
    id: 'ys-iv-4-hlo-baseline',
    chunkId: 'ys-iv-4-text',
    label: 'Baseline Assertion',
    clauses: [
      "tag('sutra','IV.4')",
      'assert(nirmanaCitta* ← asmitAParameter)',
      'nirmanaSet := derive(asmitAParameter)',
    ],
  },
  {
    id: 'ys-iv-4-hlo-law',
    chunkId: 'ys-iv-4-law',
    label: 'Modulation Law',
    clauses: [
      'cogitoModulationLaw := rule( plurality(nirmanaCitta) = internalDifferentiation(asmitAParameter) )',
      'externalCauseNegation := negate(externalProducer(nirmanaSet))',
    ],
  },
  {
    id: 'ys-iv-4-hlo-structure',
    chunkId: 'ys-iv-4-structure',
    label: 'Construction Structure',
    clauses: [
      'forEach(nirmanaCitta in nirmanaSet) ⇒ exists(modulationVector(asmitAParameter → nirmanaCitta))',
      'identityContinuum := sustain(asmitAParameter)',
      'pseudoPlurality ⇐ index(nirmanaCitta, identityContinuum)',
    ],
  },
  {
    id: 'ys-iv-4-hlo-crosswalk',
    chunkId: 'ys-iv-4-crosswalk',
    label: 'Crosswalk',
    clauses: [
      'fichteParallelSelfConstruction(asmitAParameter ⇒ selfModulate(nirmanaSet))',
      'logicalDerivativeStratum := after(reflectiveShadowLogic(nirmanaSetPatterns))',
    ],
  },
  {
    id: 'ys-iv-4-hlo-errors',
    chunkId: 'ys-iv-4-errors',
    label: 'Error Diagnostics',
    clauses: [
      'ownershipIllusion ⇐ reify(indexLabel(nirmanaCitta))',
      'misattributionError ⇐ posit(externalProducer(nirmanaSet))',
      'guard := monitor(ownershipIllusion ∨ misattributionError)',
    ],
  },
]

// ============================================================
// EXTENSION (v2 – Hypothetical vs Real Self-Construction)
// ============================================================

export const YS_IV_4_ONTOLOGY_EXT = {
  hypotheticalAssertion: '“If being is constructed” – provisional framing to license genetic inquiry',
  consciousnessProofAttempt: 'Appeal to sheer consciousness report as proof (insufficient alone)',
  validityQuestion: 'Scope to which consciousness claims (re: construction) are admissible',
  idealisticImaginalConstruction: 'Merely ideal / imaginal self-construction posited via reflective inference',
  realImmediateSelfConstruction: 'Intrinsic esse = self-construction: being is only in constructing itself',
  contrastiveRealPredicate: '“Real” gains sense only relationally vs imaginal construction',
  nonAnticipationDiscipline: 'Method rule: do not smuggle future genetic results into hypothesis',
  analyticSyntheticPrinciple: 'Underlying principle whose validity must be genetically deduced',
  geneticDeductionProtocol: 'Procedure deriving validity of analyticSyntheticPrinciple from ground',
  hypotheticalValidityBracket: 'Status: distinction (ideal vs real) held only under hypothesis until deduction completes',
  consciousnessScopeLimit: 'Boundary of what unvalidated consciousness reports can ground',
  misattributionIdealAsReal: 'Error: taking imaginal construction as the intrinsic real act',
  methodologicalGuard: 'Operational guard preventing anticipation & misattribution',
}

const CHUNKS_YS_IV_4_EXT = [
  {
    id: 'ys-iv-4-hypothesis-frame',
    title: 'Hypothetical Frame',
    summary: '“If being is constructed” introduced as controlled hypothesis for genetic validation.',
  },
  {
    id: 'ys-iv-4-consciousness-scope',
    title: 'Consciousness Proof Limits',
    summary: 'Sheer consciousness report insufficient; scope of admissibility under inquiry.',
  },
  {
    id: 'ys-iv-4-dual-construction',
    title: 'Dual Constructions',
    summary: 'Distinguishes idealistic imaginal vs real immediate self-construction.',
  },
  {
    id: 'ys-iv-4-validity-task',
    title: 'Validity Task',
    summary: 'Determine validity of analytic/synthetic principle grounding construction claims.',
  },
  {
    id: 'ys-iv-4-genetic-protocol',
    title: 'Genetic Deduction Protocol',
    summary: 'Principle validity only via genetic deduction (no anticipatory import).',
  },
  {
    id: 'ys-iv-4-method-guards',
    title: 'Method Guards',
    summary: 'Guards: nonAnticipationDiscipline; prevent misattributionIdealAsReal.',
  },
]

CHUNKS_YS_IV_4.push(...CHUNKS_YS_IV_4_EXT)

const HLOS_YS_IV_4_EXT = [
  {
    id: 'ys-iv-4-hlo-hypothesis',
    chunkId: 'ys-iv-4-hypothesis-frame',
    label: 'Hypothesis Framing',
    clauses: [
      'hypotheticalAssertion := frame(if(constructed(being)))',
      'hypotheticalValidityBracket := status(provisional)',
      'nonAnticipationDiscipline ⇒ forbid(import(futureResults))',
    ],
  },
  {
    id: 'ys-iv-4-hlo-consciousness-scope',
    chunkId: 'ys-iv-4-consciousness-scope',
    label: 'Consciousness Scope',
    clauses: [
      'consciousnessProofAttempt ⇒ insufficient(alone)',
      'validityQuestion := scope(consciousnessClaims)',
    ],
  },
  {
    id: 'ys-iv-4-hlo-dual-construction',
    chunkId: 'ys-iv-4-dual-construction',
    label: 'Dual Construction Distinction',
    clauses: [
      'idealisticImaginalConstruction ≠ realImmediateSelfConstruction',
      'realImmediateSelfConstruction := esse(selfConstruction)',
      'contrastiveRealPredicate := derive(realImmediateSelfConstruction / idealisticImaginalConstruction)',
    ],
  },
  {
    id: 'ys-iv-4-hlo-validity',
    chunkId: 'ys-iv-4-validity-task',
    label: 'Validity Inquiry',
    clauses: [
      'analyticSyntheticPrinciple ⇒ requires(geneticDeductionProtocol)',
      'not(accept(analyticSyntheticPrinciple)) before(geneticDeductionProtocol.complete)',
    ],
  },
  {
    id: 'ys-iv-4-hlo-genetic',
    chunkId: 'ys-iv-4-genetic-protocol',
    label: 'Genetic Protocol',
    clauses: [
      'geneticDeductionProtocol := procedure(derive(validity(analyticSyntheticPrinciple)))',
      'success(geneticDeductionProtocol) ⇒ lift(hypotheticalValidityBracket)',
    ],
  },
  {
    id: 'ys-iv-4-hlo-guards',
    chunkId: 'ys-iv-4-method-guards',
    label: 'Method Guards',
    clauses: [
      'misattributionIdealAsReal ⇐ conflate(idealisticImaginalConstruction, realImmediateSelfConstruction)',
      'methodologicalGuard := monitor(nonAnticipationDiscipline ∧ negate(misattributionIdealAsReal))',
    ],
  },
]

HLOS_YS_IV_4.push(...HLOS_YS_IV_4_EXT)

// ---------- Optional symbol list ----------
export const YS_IV_4_REFERENCED_SYMBOLS = [
  'nirmanaCitta','asmitAParameter','cogitoModulationLaw',
  'hypotheticalAssertion','idealisticImaginalConstruction','realImmediateSelfConstruction',
  'analyticSyntheticPrinciple','geneticDeductionProtocol','nonAnticipationDiscipline',
  'misattributionIdealAsReal','methodologicalGuard'
]

// ---------- Export Unit ----------
export const YS_IV_4_UNIT: DatasetUnit = {
  id: makeUnitId('ys-iv-4'),
  title: 'YS IV.4 — nirmāṇa-cittāni asmitā-mātra',
  scope: 'essence',
  logosMode: 'essence',
  synthesis: 'cogito-modulation-law',
  faculty: 'buddhi',
  lens: 'yoga',
  chunks: CHUNKS_YS_IV_4 as any,
  hlos: HLOS_YS_IV_4 as any,
}
