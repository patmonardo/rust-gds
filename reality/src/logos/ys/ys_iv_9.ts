import { DatasetUnit, makeUnitId } from '@organon/gdsl/registry/canon'

/*
YS IV.9 (Appearance: Continuity via One-Form Identity)
jāti-deśa-kāla-vyavahitānām api-anantaryaṁ smṛti-saṁskārayor eka-rūpatvāt

Analytic parsing:
- jāti: class/species/birth (identity-type)
- deśa: place/region (spatial locus)
- kāla: time (temporal locus)
- vyavahitānām: of those separated/with intervals intervening
- api: even (despite)
- anantaryam: unbroken succession/continuity (no gap in sequence)
- smṛti-saṁskārayor: of memory and saṁskāra (dual)
- eka-rūpatvāt: because-of one-form-ness (form identity / invariant signature)

Thesis:
Even across separations of class, place, and time, succession is unbroken because memory and saṁskāra share one and the same form. Continuity is secured by form-identity (eka-rūpatva), not by spatiotemporal adjacency. Memory is the readable manifestation of the latent saṁskāra; their invariant signature enforces Appearance-chain contiguity.
*/

// ---------- Ontology ----------
export const YS_IV_9_ONTOLOGY = {
  classSeparation: 'Jāti-difference: separation by type/species/birth',
  spatialSeparation: 'Deśa-difference: separation by place/region',
  temporalSeparation: 'Kāla-difference: separation by time',
  separationVector: 'Composite separation across jāti/deśa/kāla (vyavahita)',
  unbrokenSuccession: 'Anantarya: no gap in the valid appearance sequence',
  memoryStream: 'Smṛti: readable trace/output corresponding to latent form',
  samskaraLatent: 'Saṁskāra: latent impression (proto-form packet)',
  oneFormIdentity: 'Eka-rūpatva: invariant signature/shape shared by smṛti and saṁskāra',
  formSignature: 'Canonical descriptor used to test identity of memory and latent packet',
  continuityLaw: 'Law: if eka-rūpatva(smṛti, saṁskāra) then anantarya holds despite separations',
  adjacencyIndependence: 'Continuity does not require spatial/temporal adjacency',
  identityOverChange: 'Essence-persistence via form identity across locus changes',
  readoutMapping: 'Memory as faithful readout/decoding of the samskaraLatent',
  fragmentationError: 'Error: breaking continuity by ignoring one-form constraint',
  pseudoContinuityRisk: 'Spurious chaining via mere adjacency without form identity',
  colorlessGroundDependency: 'Best-case continuity assumes uncolored ground from IV.7',
}

// ---------- Chunks ----------
const CHUNKS_YS_IV_9 = [
  {
    id: 'ys-iv-9-text',
    title: 'IV.9 Text & Baseline',
    summary: 'Even across class/place/time separations, succession is unbroken due to one-form identity of memory and saṁskāra.',
  },
  {
    id: 'ys-iv-9-identity',
    title: 'One-Form Identity',
    summary: 'Continuity grounded in invariant signature shared by smṛti and saṁskāra.',
  },
  {
    id: 'ys-iv-9-continuity-law',
    title: 'Continuity Law',
    summary: 'Form-identity, not adjacency, determines Appearance-chain contiguity.',
  },
  {
    id: 'ys-iv-9-mechanism',
    title: 'Mechanism',
    summary: 'Memory is a readout of latent saṁskāra; matching signatures enforce linkage across loci.',
  },
  {
    id: 'ys-iv-9-errors',
    title: 'Error Modes',
    summary: 'Errors: adjacency fallacy; reifying separations; ignoring signature test.',
  },
  {
    id: 'ys-iv-9-crosswalk',
    title: 'Crosswalks',
    summary: 'Hegel: identity-in-change; Fichte: consequentia of selection preserves signature across steps.',
  },
]

// ---------- HLO Clauses ----------
const HLOS_YS_IV_9 = [
  {
    id: 'ys-iv-9-hlo-baseline',
    chunkId: 'ys-iv-9-text',
    label: 'Baseline',
    clauses: [
      "tag('sutra','IV.9')",
      'separationVector := tuple(classSeparation, spatialSeparation, temporalSeparation)',
      'assert(unbrokenSuccession) ⇐ oneFormIdentity(memoryStream, samskaraLatent)',
      'adjacencyIndependence := affirm(unbrokenSuccession despite separationVector)',
    ],
  },
  {
    id: 'ys-iv-9-hlo-identity',
    chunkId: 'ys-iv-9-identity',
    label: 'One-Form Identity',
    clauses: [
      'formSignature(smṛti) := derive(memoryStream)',
      'formSignature(saṁskāra) := derive(samskaraLatent)',
      'oneFormIdentity := eq(formSignature(smṛti), formSignature(saṁskāra))',
    ],
  },
  {
    id: 'ys-iv-9-hlo-law',
    chunkId: 'ys-iv-9-continuity-law',
    label: 'Continuity Law',
    clauses: [
      'continuityLaw := rule( oneFormIdentity ⇒ anantarya )',
      'negate(require(adjacency) for anantarya)',
    ],
  },
  {
    id: 'ys-iv-9-hlo-mechanism',
    chunkId: 'ys-iv-9-mechanism',
    label: 'Mechanism',
    clauses: [
      'readoutMapping := decode(samskaraLatent → memoryStream)',
      'identityOverChange := sustain(oneFormIdentity across separationVector)',
      'colorlessGroundDependency := prefer(continuityLaw | uncoloredGround)',
    ],
  },
  {
    id: 'ys-iv-9-hlo-errors',
    chunkId: 'ys-iv-9-errors',
    label: 'Errors',
    clauses: [
      'fragmentationError ⇐ ignore(oneFormIdentity) ∧ relyOn(adjacencyOnly)',
      'pseudoContinuityRisk ⇐ chain(adjacency) ∧ not(oneFormIdentity)',
    ],
  },
  {
    id: 'ys-iv-9-hlo-crosswalk',
    chunkId: 'ys-iv-9-crosswalk',
    label: 'Crosswalks',
    clauses: [
      'map(identityOverChange ↔ hegel_identity_in_change)',
      'map(continuityLaw ↔ fichte_consequentia_signature_preservation)',
    ],
  },
]

// ---------- Export Unit ----------
export const YS_IV_9_UNIT: DatasetUnit = {
  id: makeUnitId('ys-iv-9'),
  title: 'YS IV.9 — jāti-deśa-kāla-vyavahitānām api anantaryaṁ smṛti-saṁskārayor eka-rūpatvāt',
  scope: 'appearance',
  logosMode: 'appearance',
  synthesis: 'form-identity-continuity',
  faculty: 'buddhi',
  lens: 'yoga',
  chunks: CHUNKS_YS_IV_9 as any,
  hlos: HLOS_YS_IV_9 as any,
}

// PATCH: correct the pseudoContinuityRisk clause (remove stray paren and rely on oneFormIdentity).
// In the HLOS_YS_IV_9 'errors' section, replace:
// 'pseudoContinuityRisk ⇐ chain(adjacency) ∧ not(eqSignature))',
// with:
/// 'pseudoContinuityRisk ⇐ chain(adjacency) ∧ not(oneFormIdentity)',


// ============================================================
// EXTENSION – Lecture 17: “Should” as Middle Term for Appearance
// Speculative bridge: eliminate hypothetical status by compressing it
// into the “should” and making it the inner principle of Appearance.
// ============================================================

export const YS_IV_9_ONTOLOGY_EXT = {
  shouldMiddleTerm: 'The “should” as inner principle of Appearance (normative form of genesis)',
  hypotheticalCompression: 'All hypothetical status compressed into the “should” form',
  ascentContentFocus: 'Ascent held content (being) fixed; ignored modal form',
  descentFormDiscipline: 'Descent targets the neglected form (“should”) for justification',
  idealismFormPersistence: 'Idealist form persists after content is overruled; must self-justify',
  selfRefutationRequirement: 'Form must refute its own ungrounded claims by its own law',
  mediatorInItselfNotInItself: '“Should” mediates in-itself and not-in-itself as a new middle term',
  fivefoldSynthesisLink: 'Link back to the earlier fivefold synthesis of appearance relations',
  appearanceFoundationPoint: '“Should” as one of the deepest foundation points of appearance',
  firmPrincipleTarget: 'Objective: surface a self-sustaining principle from the “should” dynamics',
}

const CHUNKS_YS_IV_9_EXT = [
  {
    id: 'ys-iv-9-should-middle',
    title: '“Should” Middle Term',
    summary: 'The “should” functions as inner principle of Appearance, mediating in-itself and not-in-itself.',
  },
  {
    id: 'ys-iv-9-hypothetical-elim',
    title: 'Hypothetical Elimination',
    summary: 'Compress hypothetical status into the “should” and justify it intrinsically.',
  },
  {
    id: 'ys-iv-9-form-vs-content',
    title: 'Descent: Form vs Content',
    summary: 'Ascent fixed content; descent justifies the modal form that persisted.',
  },
  {
    id: 'ys-iv-9-self-refutation',
    title: 'Self-Refutation Discipline',
    summary: 'Form must defeat its own ungrounded claims by its own law.',
  },
  {
    id: 'ys-iv-9-synthesis-bridge',
    title: 'Synthesis Bridge',
    summary: 'Middle term links prior fivefold synthesis to Appearance continuity (eka-rūpatva).',
  },
]

CHUNKS_YS_IV_9.push(...CHUNKS_YS_IV_9_EXT)

const HLOS_YS_IV_9_EXT = [
  {
    id: 'ys-iv-9-hlo-should-middle',
    chunkId: 'ys-iv-9-should-middle',
    label: 'Middle Term',
    clauses: [
      'shouldMiddleTerm := install(normativeKernel within Appearance)',
      'mediatorInItselfNotInItself := role(shouldMiddleTerm)',
      'appearanceFoundationPoint := assert(shouldMiddleTerm)',
    ],
  },
  {
    id: 'ys-iv-9-hlo-hypo-elim',
    chunkId: 'ys-iv-9-hypothetical-elim',
    label: 'Hypothesis Handling',
    clauses: [
      'hypotheticalCompression := compress(hypothesis → shouldMiddleTerm)',
      'firmPrincipleTarget := seek(selfSustainingPrincipleSearch via shouldMiddleTerm)',
    ],
  },
  {
    id: 'ys-iv-9-hlo-form-content',
    chunkId: 'ys-iv-9-form-vs-content',
    label: 'Descent Discipline',
    clauses: [
      'ascentContentFocus := done(contentTruth)',
      'descentFormDiscipline := justify(modalForm(“should”))',
      'idealismFormPersistence := note(persistingForm after(contentOverruled))',
    ],
  },
  {
    id: 'ys-iv-9-hlo-self-refutation',
    chunkId: 'ys-iv-9-self-refutation',
    label: 'Self-Refutation',
    clauses: [
      'selfRefutationRequirement := law(form refutes own ungrounded claims)',
      'validate(form) ⇐ selfRefutationRequirement',
    ],
  },
  {
    id: 'ys-iv-9-hlo-synthesis-bridge',
    chunkId: 'ys-iv-9-synthesis-bridge',
    label: 'Bridge',
    clauses: [
      'fivefoldSynthesisLink := reference(priorSynthesis)',
      'link(shouldMiddleTerm → oneFormIdentity) // continuity via invariant form is the modal content of the “should”',
    ],
  },
]

HLOS_YS_IV_9.push(...HLOS_YS_IV_9_EXT)

export const YS_IV_9_REFERENCED_SYMBOLS = [
  // base continuity layer
  'oneFormIdentity','unbrokenSuccession','separationVector','readoutMapping','continuityLaw',
  // “should” middle-term (prior ext)
  'shouldMiddleTerm','hypotheticalCompression','descentFormDiscipline',
  'idealismFormPersistence','selfRefutationRequirement','mediatorInItselfNotInItself',
  // links
  'fivefoldSynthesisLink'
]

// ============================================================
// EXTENSION – Lecture 17: Categorical core of the “should”
// (self-support, internal self-grounding, ego-bracketing discipline)
// ============================================================

export const YS_IV_9_ONTOLOGY_EXT2 = {
  shouldCategoricalCore: 'Categorical element within the hypothetical “should”',
  internalSelfGrounding: 'Pure inner self-grounding; complete external groundlessness',
  absoluteSelfSupportCondition: 'If it exists, it must hold and sustain itself out of itself',
  dropOrHoldDialectic: 'The absolute assumption may drop; if not, it must self-sustain',
  categoricalInsightShould: 'Insight into the unalterable nature of the “should,” abstracted from existence',
  abstractionFromExistence: 'Bracket outward existence while discerning essence',
  egoBracketDiscipline: 'Exclude the “we/I” of mere consciousness until deduced',
  creatorIrrelevance: 'Even if “I” posits it, the law resides in the “should,” not in the positor',
  shouldLawOfProceeding: 'Norm: the “should” carries the rule of proceeding in itself',
  selfSustainingPrincipleWatch: 'Watchpoint for emergence of a self-sustaining principle (firm ground)',
}

const CHUNKS_YS_IV_9_EXT2 = [
  {
    id: 'ys-iv-9-should-categorical',
    title: 'Categorical in the “Should”',
    summary: 'Find the categorical element inside the hypothetical “should”: self-support if it exists.',
  },
  {
    id: 'ys-iv-9-ego-bracket',
    title: 'Ego-Bracketing Discipline',
    summary: 'Exclude the empirical “we/I”; law lies in the should itself irrespective of creator.',
  },
  {
    id: 'ys-iv-9-should-law',
    title: 'Law of Proceeding',
    summary: 'The “should” contains its own rule of proceeding; existence may be abstracted.',
  },
]

CHUNKS_YS_IV_9.push(...CHUNKS_YS_IV_9_EXT2)

const HLOS_YS_IV_9_EXT2 = [
  {
    id: 'ys-iv-9-hlo-should-categorical',
    chunkId: 'ys-iv-9-should-categorical',
    label: 'Categorical Core',
    clauses: [
      'shouldCategoricalCore := discover(within(“should”))',
      'internalSelfGrounding := assert(externalGroundlessness ∧ innerSelfGround)',
      'absoluteSelfSupportCondition := rule(if(exists(“should”)) ⇒ must(selfSustain))',
      'categoricalInsightShould := insight(unalterableNature(“should”))',
      'abstractionFromExistence := permit(ignore(existence) while(analyzingEssence))',
    ],
  },
  {
    id: 'ys-iv-9-hlo-ego-bracket',
    chunkId: 'ys-iv-9-ego-bracket',
    label: 'Ego Bracket',
    clauses: [
      'egoBracketDiscipline := method(bracket(empiricalWeI))',
      'creatorIrrelevance := note(lawInShould ≠ lawInCreator)',
    ],
  },
  {
    id: 'ys-iv-9-hlo-should-law',
    chunkId: 'ys-iv-9-should-law',
    label: 'Rule of Proceeding',
    clauses: [
      'shouldLawOfProceeding := norm(carriedBy(“should”))',
      'dropOrHoldDialectic := option(drop(assumption) ∨ must(selfSustain))',
      'selfSustainingPrincipleWatch := monitor(emergence(selfSustainingPrincipleSearch))',
    ],
  },
]

HLOS_YS_IV_9.push(...HLOS_YS_IV_9_EXT2)

// OPTIONAL: append new symbols
// (If you prefer a single list, replace the earlier export with the merged array below)
export const YS_IV_9_REFERENCED_SYMBOLS_EXT2 = [
  'shouldCategoricalCore',
  'internalSelfGrounding',
  'absoluteSelfSupportCondition',
  'dropOrHoldDialectic',
  'categoricalInsightShould',
  'abstractionFromExistence',
  'egoBracketDiscipline',
  'creatorIrrelevance',
  'shouldLawOfProceeding',
  'selfSustainingPrincipleWatch',
]

/* ============================================================
   EXTENSION – Imago/Projection Bridge (Biblical + Vedic crosswalk)
   Image (imago) externalization: Purusha–Prakriti projector, body as screen.
   Ties to IV.9: smriti as image-readout of samskara seed; continuity via one-form.
============================================================ */

export const YS_IV_9_ONTOLOGY_EXT4 = {
  imagoArchetype: 'Archetypal image (imago) whose signature persists from seed to display',
  projectorActUniversal: 'Universal projector act (śakti) externalizing latent forms',
  screenBody: 'Embodied field as screen of manifestation',
  polarityDyad: 'Male–female polarity as formal dyad (not moralized color)',
  imageReadoutOfSeed: 'Smriti as image/readout of samskara latent seed',
  imagoContinuity: 'Continuity grounded in sameness of imago-signature (eka-rupatva)',
  purushaPrakritiBridge: 'Purusha (form-essence) expressed via Prakriti (display medium)',
  nonmagicalProjectionGuard: 'Guard: projection is lawful selection, not miracle',
}

const CHUNKS_YS_IV_9_EXT4 = [
  {
    id: 'ys-iv-9-imago-projection',
    title: 'Imago → Projection',
    summary: 'Latent seed carries imago-signature; projector act displays it on the embodied screen.',
  },
  {
    id: 'ys-iv-9-polarity-dyad',
    title: 'Polarity Dyad (Male–Female)',
    summary: 'Formal dyad of display, subordinate to invariant form-signature.',
  },
  {
    id: 'ys-iv-9-body-screen',
    title: 'Body as Screen',
    summary: 'Embodiment functions as the screen for lawful manifestation (no external producer).',
  },
]

CHUNKS_YS_IV_9.push(...CHUNKS_YS_IV_9_EXT4)

const HLOS_YS_IV_9_EXT4 = [
  {
    id: 'ys-iv-9-hlo-imago-projection',
    chunkId: 'ys-iv-9-imago-projection',
    label: 'Imago/Seed',
    clauses: [
      'imageReadoutOfSeed := map(samskaraLatent → memoryStream)',
      'imagoContinuity := hold( oneFormIdentity(memoryStream, samskaraLatent) )',
      'nonmagicalProjectionGuard := assert(selectionLaw)',
    ],
  },
  {
    id: 'ys-iv-9-hlo-polarity-dyad',
    chunkId: 'ys-iv-9-polarity-dyad',
    label: 'Dyad',
    clauses: [
      'polarityDyad := formalDyad(male, female)',
      'polarityDyad ≺ oneFormIdentity // dyad is subordinated to invariant form',
    ],
  },
  {
    id: 'ys-iv-9-hlo-body-screen',
    chunkId: 'ys-iv-9-body-screen',
    label: 'Projector/Screen',
    clauses: [
      'projectorActUniversal := enact(prakritiDisplay)',
      'screenBody := host(manifestationEvent)',
      'purushaPrakritiBridge := bridge(formEssence → projectorActUniversal → screenBody)',
    ],
  },
]

HLOS_YS_IV_9.push(...HLOS_YS_IV_9_EXT4)

export const YS_IV_9_REFERENCED_SYMBOLS_EXT4 = [
  'imagoArchetype','projectorActUniversal','screenBody','polarityDyad',
  'imageReadoutOfSeed','imagoContinuity','purushaPrakritiBridge','nonmagicalProjectionGuard'
]

/* ============================================================
   EXTENSION – Projection/Māyā Law (Absolute Form as Projective Power)
   Hegel: Logic of Essence–Ground–Appearance ≡ theory of Māyā’s projector.
   Here: continuity holds because the projector preserves Form (eka-rūpatva).
============================================================ */

export const YS_IV_9_ONTOLOGY_EXT5 = {
  mayaProjection: 'Projective power that externalizes Absolute Form as appearance (śakti)',
  absoluteForm: 'Form-as-invariance; the signature preserved in projection (eka-rūpatva)',
  projectorGround: 'Uncolored Ground (IV.7) functioning as the lawful projector lens',
  displayField: 'The field/screen upon which projection appears (deha/loka)',
  projectiveSelection: 'SelectionLaw seen as projector’s filtering of congruent forms',
  reflexiveJnana: 'Knowing that sees projection as projection (non-deluded appearance)',
  hegelLogicOfMaya: 'Essence→Ground→Appearance as complete theory of projection',
  nondualGuard: 'No second substrate: projection is Self of Self (no external producer)',
}

const CHUNKS_YS_IV_9_EXT5 = [
  {
    id: 'ys-iv-9-maya-law',
    title: 'Māyā as Projector',
    summary: 'Ground acts as projector; Absolute Form is preserved as invariant signature.',
  },
  {
    id: 'ys-iv-9-projective-selection',
    title: 'Projective Selection',
    summary: 'Manifestation = selection of congruent forms under the projector (no miracle).',
  },
  {
    id: 'ys-iv-9-reflexive-jnana',
    title: 'Reflexive Jñāna',
    summary: 'Science sees the projection as projection; cancels delusion of external cause.',
  },
]

CHUNKS_YS_IV_9.push(...CHUNKS_YS_IV_9_EXT5)

const HLOS_YS_IV_9_EXT5 = [
  {
    id: 'ys-iv-9-hlo-maya-law',
    chunkId: 'ys-iv-9-maya-law',
    label: 'Projector',
    clauses: [
      'mayaProjection := enact(projectorGround)',
      'absoluteForm := invariant(formSignature(smṛti) == formSignature(saṁskāra))',
      'hegelLogicOfMaya := map(Essence→Ground→Appearance, mayaProjection)',
    ],
  },
  {
    id: 'ys-iv-9-hlo-projective-selection',
    chunkId: 'ys-iv-9-projective-selection',
    label: 'Selection',
    clauses: [
      'projectiveSelection := identify(selectionLaw as action(mayaProjection))',
      'manifestationEvent := appear( filter(vasanaSet, congruent(ripeningProfile, vasanaSignature)) )',
      'nondualGuard := assert(not(externalProducer(manifestationEvent)))',
    ],
  },
  {
    id: 'ys-iv-9-hlo-reflexive-jnana',
    chunkId: 'ys-iv-9-reflexive-jnana',
    label: 'Reflexivity',
    clauses: [
      'reflexiveJnana := know(manifestationEvent as projection(mayaProjection))',
      'assert(unbrokenSuccession) ⇐ absoluteForm',
    ],
  },
]

HLOS_YS_IV_9.push(...HLOS_YS_IV_9_EXT5)

export const YS_IV_9_REFERENCED_SYMBOLS_EXT5 = [
  'mayaProjection','absoluteForm','projectorGround','displayField',
  'projectiveSelection','reflexiveJnana','hegelLogicOfMaya','nondualGuard'
]
