import { DatasetUnit, makeUnitId } from '@organon/gdsl/registry/canon'

/*
YS IV.5  (Essence → Cogito Dialectic / Onset of Vṛtti Section)
pravṛtti-bhede prayojakam cittaṁ ekaṁ anekeṣām

Metaphysical-Scientific Parsing (Identity–Difference Dialectic):
- pravṛtti: primary drive / outgoing determination / identity-asserting initiation (pre-differential thrust)
- bhede (locative singular used adverbially): within the articulated field of difference / differentiation phase
- prayojakam: the functional instigator / determinant (not a pushing external efficient cause but immanent determinant condition)
- cittam: Cogito-field (synthetic unity of apperception; determining consciousness; “I think” substrate)
- ekam: numerically / structurally one (identity pole)
- anekeṣām: of the many (vectors / operations / derivative activity-channels)

Scientific Cogito Thesis:
One determining Cogito (cittam ekam) functions as the immanent determinant (prayojakam) across a multiplicity of operation-vectors (anekesam) inside the dialectic of Identity (pravṛtti) and Difference (bheda). Difference (bheda) is not opposed externality but the articulated dispersion of a single identity-thrust. Thus IV.5 inaugurates the “vṛtti” analytic: mapping how reflective determinations (diverse operational modulations) are all indexed to a single self-identical apperceptive ground.

Not “mystical”; no metaphysical plural mind-substances. Provides law: diversity of reflective operations presupposes a unifying determining consciousness which is prior to derivative formal Logic (logic = later reflective shadow of this identity-difference engine). Prefigures Dhyāna as active dialectic (living mediation of identity/difference), not passive trance.

Crosswalk:
- Fichte: Self-constructing being projecting internal differentiations; plurality of acts = one living principle.
- Kant: “I think must be able to accompany all my representations” formalized here as operative cause-law of variation.
- Hegel (Objective → Subjective Logic bridge): Identity-in-difference prior to explicit conceptual syllogism.
*/

/* Ontology */
export const YS_IV_5_ONTOLOGY = {
  pravrittiIdentityDrive: 'Pravṛtti: initiating identity-thrust; pre-formal determination vector',
  bhedaDifferentiationField: 'Structured dispersion phase where identity articulates as difference',
  identityDifferenceDialectic: 'Dynamic: identity drive continuously producing ordered difference',
  cogitoDeterminant: 'Cittam as immanent determinant (prayojakam) of all operation-vectors',
  singleCogito: 'Ekam cittam: numerically & structurally one determining field',
  multiOperationSet: 'Anekesam: plurality of derivative vṛtti / modulation channels',
  operationVector: 'Individuated modulation path within differentiation field',
  immanentDeterminationLaw: 'Law: plurality presupposes singleCogito as determinant condition',
  reflectiveDispersion: 'Bheda-phase manifestation: distinct operations referencing one source',
  preLogicalStratum: 'Layer prior to formal logic; raw identity-difference engine',
  dhyanaDialecticSeed: 'Seed of Dhyāna as active synthesis of identity & difference',
  pseudoPluralMindError: 'Error: reifying operations as independent consciousnesses',
  ownershipDrift: 'Mistaking channel-index for autonomous ego-center',
  determinationVsReflection: 'Distinction: determining identity-drive vs passive reflective noticing',
  invarianceConstraint: 'Constraint: all valid operations must resolve to singleCogito invariance',
  degenerationCase: 'Case where dispersion loses linkage to singleCogito (invalid vṛtti noise)',
}

/* Chunks */
const CHUNKS_YS_IV_5 = [
  {
    id: 'ys-iv-5-text',
    title: 'IV.5 Text & Baseline',
    summary: 'One Cogito (cittam ekam) is determinant across a plurality of operations within differentiation (pravṛtti–bheda).',
  },
  {
    id: 'ys-iv-5-law',
    title: 'Immanent Determination Law',
    summary: 'Plural vṛtti operations arise via one immanent determinant—not external causation.',
  },
  {
    id: 'ys-iv-5-identity-difference',
    title: 'Identity–Difference Dialectic',
    summary: 'Difference = articulated dispersion of identity-drive; not an external opposition.',
  },
  {
    id: 'ys-iv-5-crosswalk',
    title: 'Crosswalk (Fichte / Kant / Dialectic)',
    summary: 'Maps to Fichtean self-construction; Kantian apperceptive unity; dialectical identity-in-difference engine.',
  },
  {
    id: 'ys-iv-5-errors',
    title: 'Error Diagnostics',
    summary: 'Errors: pseudo-plural minds; ownership drift; losing invariance link.',
  },
  {
    id: 'ys-iv-5-dhyana-seed',
    title: 'Dhyāna (Dialectic) Seed',
    summary: 'Ground for defining Dhyāna as active mediation of identity and difference.',
  },
]

/* HLO Clauses */
const HLOS_YS_IV_5 = [
  {
    id: 'ys-iv-5-hlo-baseline',
    chunkId: 'ys-iv-5-text',
    label: 'Baseline',
    clauses: [
      "tag('sutra','IV.5')",
      'assert(singleCogito)',
      'prayojakam(singleCogito, multiOperationSet)',
      'multiOperationSet := set(operationVector*)',
    ],
  },
  {
    id: 'ys-iv-5-hlo-law',
    chunkId: 'ys-iv-5-law',
    label: 'Determination Law',
    clauses: [
      'immanentDeterminationLaw := rule( plurality(operationVector) ⇒ require(singleCogito) )',
      'negate(externalEfficientCause(operationVector*))',
    ],
  },
  {
    id: 'ys-iv-5-hlo-identity-diff',
    chunkId: 'ys-iv-5-identity-difference',
    label: 'Identity–Difference Engine',
    clauses: [
      'pravrittiIdentityDrive ⇒ generate(bhedaDifferentiationField)',
      'bhedaDifferentiationField ⇒ express(identity(pravrittiIdentityDrive))',
      'identityDifferenceDialectic := loop(pravrittiIdentityDrive ↔ bhedaDifferentiationField)',
    ],
  },
  {
    id: 'ys-iv-5-hlo-crosswalk',
    chunkId: 'ys-iv-5-crosswalk',
    label: 'Crosswalk',
    clauses: [
      'map(singleCogito ↔ apperceptiveUnity)',
      'map(identityDifferenceDialectic ↔ fichteSelfConstructionDispersion)',
      'preLogicalStratum := prior(formalLogicLayer)',
    ],
  },
  {
    id: 'ys-iv-5-hlo-errors',
    chunkId: 'ys-iv-5-errors',
    label: 'Errors',
    clauses: [
      'pseudoPluralMindError ⇐ reify(operationVector as independentMind)',
      'ownershipDrift ⇐ misassign(authorship, operationVector)',
      'degenerationCase ⇐ loseLink(operationVector, singleCogito)',
    ],
  },
  {
    id: 'ys-iv-5-hlo-dhyana',
    chunkId: 'ys-iv-5-dhyana-seed',
    label: 'Dhyāna Seed',
    clauses: [
      'dhyanaDialecticSeed := activeSynthesis(pravrittiIdentityDrive, bhedaDifferentiationField)',
      'dhyanaDialecticSeed ⇒ stabilize(identityDifferenceDialectic)',
    ],
  },
]

/* ---------- FIX EXPORT (complete object) ---------- */
export const YS_IV_5_UNIT: DatasetUnit = {
  id: makeUnitId('ys-iv-5'),
  title: 'YS IV.5 — pravṛtti-bhede prayojakam cittaṁ ekaṁ anekeṣām',
  scope: 'essence',
  logosMode: 'essence',
  synthesis: 'identity-difference-dialectic',
  faculty: 'buddhi',
  lens: 'yoga',
  chunks: CHUNKS_YS_IV_5 as any,
  hlos: HLOS_YS_IV_5 as any,
}

/* ============================================================
   APPENDED EXTENSION (v2 – Absolute Insight / Genesis-of-Genesis)
   Source: Fichte (Absolute Insight passage) mapped to IV.5 Cogito Dialectic
============================================================ */

/* Ontology Extension */
export const YS_IV_5_ONTOLOGY_EXT = {
  absoluteInsight: 'Insight positing ideal construction organically within essence (no hiatus)',
  idealConstructionInEssence: 'Ideal (constructed) = internal to essence without real disjunction',
  noHiatusPrinciple: 'Denial of any real gap inside essence during self-construction',
  geneticAbsolutePosit: 'Posit of absolute origin as unconditionally necessary (given it be ground)',
  noHowOpacity: 'Limit: insight yields a “that” (quia) not a “how” (quomodo) of self-construction',
  securedAbsoluteness: 'Opacity of how functions to secure absoluteness (no higher explanatory court)',
  gapForWeOnly: 'Apparent gap subsists only relative to the We (observer standpoint)',
  genesisOfGenesis: 'Consciousness proper as second-order: regeneration/reflection of absolute genesis',
  displacementOfDifficulty: 'Problem relocated downward after knot cut at root',
  methodologicalNonAnticipation: 'Rule: proceed without forcing a constructive how beyond absolute',
  imaginalVsRealMarker: 'Filter distinguishing reflective imaginal inference from real intrinsic genesis',
  dispersionGoverned: 'Multiplicity of operations still governed by single absolute insight',
}

/* New Chunks */
const CHUNKS_YS_IV_5_EXT = [
  {
    id: 'ys-iv-5-absolute-insight',
    title: 'Absolute Insight',
    summary: 'Ideal construction organically posited within essence; no internal hiatus.',
  },
  {
    id: 'ys-iv-5-no-how',
    title: 'No-How Opacity',
    summary: 'Absoluteness secured: only the “that” of self-construction given; the “how” remains opaque.',
  },
  {
    id: 'ys-iv-5-gap-we',
    title: 'Gap Relative to We',
    summary: 'Gap exists only for the We as standpoint; not in essence itself.',
  },
  {
    id: 'ys-iv-5-genesis-genesis',
    title: 'Genesis of Genesis',
    summary: 'Consciousness = reflexive regeneration of absolute genesis (second-order).',
  },
  {
    id: 'ys-iv-5-difficulty-shift',
    title: 'Difficulty Displacement',
    summary: 'Primary knot cut; residual difficulty descends to derivative layer.',
  },
]

CHUNKS_YS_IV_5.push(...CHUNKS_YS_IV_5_EXT)

/* HLO Extensions */
const HLOS_YS_IV_5_EXT = [
  {
    id: 'ys-iv-5-hlo-absolute-insight',
    chunkId: 'ys-iv-5-absolute-insight',
    label: 'Absolute Insight',
    clauses: [
      'absoluteInsight := posit(idealConstructionInEssence ∧ noHiatusPrinciple)',
      'immanent(idealConstructionInEssence)',
      'assert(noHiatusPrinciple)',
    ],
  },
  {
    id: 'ys-iv-5-hlo-no-how',
    chunkId: 'ys-iv-5-no-how',
    label: 'No-How Opacity',
    clauses: [
      'noHowOpacity := limit(see(that(selfConstruction)), not(see(how(selfConstruction))))',
      'securedAbsoluteness ⇐ noHowOpacity',
    ],
  },
  {
    id: 'ys-iv-5-hlo-gap-we',
    chunkId: 'ys-iv-5-gap-we',
    label: 'We-Relative Gap',
    clauses: [
      'gapForWeOnly := appearGap(We)',
      'negate(gapForWeOnly) @ essence',
    ],
  },
  {
    id: 'ys-iv-5-hlo-genesis-genesis',
    chunkId: 'ys-iv-5-genesis-genesis',
    label: 'Genesis-of-Genesis',
    clauses: [
      'genesisOfGenesis := reflect(selfConstruction)',
      'consciousnessProper := genesisOfGenesis',
      'identityDifferenceDialectic ⇒ host(genesisOfGenesis)',
    ],
  },
  {
    id: 'ys-iv-5-hlo-difficulty-shift',
    chunkId: 'ys-iv-5-difficulty-shift',
    label: 'Difficulty Displacement',
    clauses: [
      'displacementOfDifficulty := relocate(problemRoot → derivativeLayer)',
      'securedAbsoluteness ⇒ enable(displacementOfDifficulty)',
    ],
  },
]

HLOS_YS_IV_5.push(...HLOS_YS_IV_5_EXT)

/* Optional symbol aggregation */
export const YS_IV_5_REFERENCED_SYMBOLS = [
  'absoluteInsight','idealConstructionInEssence','noHiatusPrinciple','noHowOpacity',
  'gapForWeOnly','genesisOfGenesis','displacementOfDifficulty'
]

/* ============================================================
   END EXTENSION v2
============================================================ */
