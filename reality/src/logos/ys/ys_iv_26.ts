import { DatasetUnit, makeUnitId } from '@organon/gdsl/registry/canon'

/*
C. ACTUALITY → Absolute Realization (conclusions of viśeṣa-darśana)

YS IV.26 — tadā viveka-nimnaṁ kaivalya-prāg-bhāraṁ cittaṁ

“Then, the mind, inclined toward discrimination, bears the forward-weight toward isolation (kaivalya).”
*/

// ---------- Ontology ----------
export const YS_IV_26_ONTOLOGY = {
  tada: 'Then — reflective connective: consequence of prior viśeṣa-darśana (IV.25)',
  vivekaNimnam: 'Inclined/sloping toward discriminative insight (viveka)',
  kaivalyaPragBharam: 'Bearing the preponderance/forward-weight toward kaivalya (isolation/liberation)',
  citta: 'Mind-stream/buddhi as reflective locus',
  viseshaDarsina: 'Discriminative seer from IV.25 (precondition)',
  cessationOfSelfCultivation: 'Cessation of cultivating self-being (IV.25)',
  discriminativeCurrent: 'Dominant current in citta is discrimination (viveka) rather than appropriation',
  kaivalyaVector: 'Vector toward kaivalya becomes stable/dominant',
  preLiberativeMind: 'Pre-liberative mind: instrument aligned “for another,” trending to isolation',
  // Carry-overs
  parartham: 'Mind is for another (IV.24)',
  sarvartham: 'Mind is omni-instrumental (IV.23)',
  apratisankramaya: 'Non-transference of consciousness (IV.22)',
  seerWithoutObjecthood: 'Witness-only seer (IV.21–IV.22)',
  // Crosswalks
  fichteCompassVector: 'The “compass” now fixes the path: categorical insight sets a stable vector',
  hegelActualityToFreedom: 'From actuality toward freedom: weight shifts to the Absolute stance (kaivalya)',
  // Guards
  errorVivekaAsMereInference: 'Error: reducing viveka to discursive syllogism; it is discriminative seeing',
  errorWorldDenial: 'Error: taking kaivalya-weight as denial of appearance rather than non-appropriation',
  errorPurushaAgentive: 'Error: making the witness an agent pushing the mind',
}

// ---------- Chunks ----------
const CHUNKS_YS_IV_26 = [
  {
    id: 'ys-iv-26-text',
    title: 'IV.26 Text & Baseline',
    summary: '“Then” (as conclusion): mind inclined to discrimination; bears the forward-weight toward kaivalya.',
  },
  {
    id: 'ys-iv-26-semantics',
    title: 'Semantics: tadā / viveka-nimna / kaivalya-prāg-bhāra',
    summary: '“Then” indicates reflective consequence; inclination as dominant current; weight toward isolation.',
  },
  {
    id: 'ys-iv-26-gradient',
    title: 'Realization Gradient',
    summary: 'From cessation of selfing to a stabilized discriminative vector and kaivalya-weight.',
  },
  {
    id: 'ys-iv-26-crosswalk',
    title: 'Crosswalk (Fichte/Hegel)',
    summary: 'Compass fixed (Fichte); actuality tilts to freedom/Absolute (Hegel).',
  },
  {
    id: 'ys-iv-26-guards',
    title: 'Guards',
    summary: 'Avoid inference-reduction, world-denial, and agentive-witness errors.',
  },
  {
    id: 'ys-iv-26-bridges',
    title: 'Bridges → IV.27',
    summary: 'Prepare for residual impressions intruding in “gaps” (saṁskāra chinks).',
  },
]

// ---------- HLO Clauses ----------
const HLOS_YS_IV_26 = [
  {
    id: 'ys-iv-26-hlo-text',
    chunkId: 'ys-iv-26-text',
    label: 'Baseline',
    clauses: [
      "tag('sutra','IV.26')",
      'assert(tada ← viseshaDarsina ∧ cessationOfSelfCultivation)',
      'conclude(vivekaNimnam(citta))',
      'conclude(kaivalyaPragBharam(citta))',
    ],
  },
  {
    id: 'ys-iv-26-hlo-semantics',
    chunkId: 'ys-iv-26-semantics',
    label: 'Semantics',
    clauses: [
      'define(tada := reflective_consequence_of_prior_sutra)',
      'define(vivekaNimnam := inclination/dominant_current_toward_discriminative_seeing)',
      'define(kaivalyaPragBharam := forward_weight/vector_toward_isolation)',
      'discriminativeCurrent := assert(vivekaNimnam(citta))',
      'kaivalyaVector := assert(kaivalyaPragBharam(citta))',
    ],
  },
  {
    id: 'ys-iv-26-hlo-gradient',
    chunkId: 'ys-iv-26-gradient',
    label: 'Gradient',
    clauses: [
      'assert(parartham ∧ sarvartham ∧ apratisankramaya ∧ seerWithoutObjecthood)',
      'preLiberativeMind := assert(citta aligned_for_another ∧ trending(kaivalyaVector))',
      'conclude(discriminativeCurrent ∧ kaivalyaVector)',
    ],
  },
  {
    id: 'ys-iv-26-hlo-crosswalk',
    chunkId: 'ys-iv-26-crosswalk',
    label: 'Crosswalk',
    clauses: [
      'fichteCompassVector := map(categorical_turn ⇒ stable_vector(viveka → kaivalya))',
      'hegelActualityToFreedom := map(actuality_weight → freedom/absolute_orientation)',
    ],
  },
  {
    id: 'ys-iv-26-hlo-guards',
    chunkId: 'ys-iv-26-guards',
    label: 'Guards',
    clauses: [
      'errorVivekaAsMereInference := flag(reduce(viveka to syllogism))',
      'errorWorldDenial := flag(read(kaivalyaVector as denial_of_appearance))',
      'errorPurushaAgentive := flag(assert(purusha pushes_citta))',
    ],
  },
  {
    id: 'ys-iv-26-hlo-bridges',
    chunkId: 'ys-iv-26-bridges',
    label: 'Bridges',
    clauses: [
      'prepare(IV_27 := caution(tac_chidresu pratyaya_antarani samskarabhyah))',
      'prepare(remedial_practice := stabilize(discriminativeCurrent) ∧ seal_gaps_against_samskara)',
    ],
  },
]

// ---------- Export Unit ----------
export const YS_IV_26_UNIT: DatasetUnit = {
  id: makeUnitId('ys-iv-26'),
  title: 'YS IV.26 — tadā viveka-nimnaṁ kaivalya-prāg-bhāraṁ cittaṁ',
  scope: 'actuality',
  logosMode: 'appearance',
  synthesis:
    '“Then”: discriminative current dominates; mind bears forward-weight toward kaivalya (pre-liberative vector).',
  faculty: 'buddhi',
  lens: 'yoga',
  chunks: CHUNKS_YS_IV_26 as any,
  hlos: HLOS_YS_IV_26 as any,
}

export const YS_IV_26_SYMBOLS = Object.keys(YS_IV_26_ONTOLOGY);
