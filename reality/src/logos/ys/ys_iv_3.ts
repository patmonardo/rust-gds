import { DatasetUnit, makeUnitId } from '@organon/gdsl/registry/canon'

/*
YS IV.3  (Essence / Citi Phase)
nimittam aprayojakam prakṛtīnām varaṇa-bhedas tu tataḥ kṣetrikavat

Segmentation (analytic):
- nimittam: instrumental condition / occasioning factor (not efficient creator)
- a-prayojakam: not an impelling / driving efficient cause
- prakṛtīnām: of (the) constitutive natures / innate tendencies (plural latent streams)
- varaṇa-bhedaḥ: the breaking / removal of a covering, obstruction, dam
- tu: but / rather (contrastive corrective)
- tataḥ: from that (removal) / thereby
- kṣetrika-vat: like a field-tender / irrigator (who simply opens channels; water flows by its own nature)

Interpretive Seed (Citi = Reflexive Clearing Intelligence):
Citi does not propel prakṛti-forces; it discloses by removing impediments. The plurality of natural tendencies self-differentiate once obstruction is cleared—analogous to irrigation: the farmer does not push water, only opens a sluice. Essence-phase emphasis: causal emptiness of a posited external mover; presence of enabling-negation (removal) as the catalytic (non-producing) condition.

Crosswalks:
- Abhidharma: distinguishing causal types—upādāna (appropriating grasp) vs pratītya (conditional nexus); here a “removal condition” (varaṇa-bheda) releasing inherent dharma flow.
- Fichte (Doctrine of Essence / Reflection): reflection does not create content but cancels a limit to let inner self-articulation appear (negation-of-limit reveals ground).
- Hegel (Essence → Appearance): essence emerges through removing immediacy’s opacity; the “farmer” figure = pure mediation-as-removal (relieving contradiction-barricade).

Citi vs Vitarka/Nirvitarka:
Earlier (Vitarka/Nirvitarka) purified object-luminosity. Now Citi = structural reflective intelligence that operates by subtractive enabling, not by additive production.

Use:
Feeds chain: saturation shift (IV.2) ⇒ enabling differentiation (IV.3) ⇒ refined conditioning specifications (future sutras).
*/

// Ontology
export const YS_IV_3_ONTOLOGY = {
  nimittaOccasion: 'Instrumental / situational condition (not efficient producer)',
  nonImpelling: 'a-prayojakam: lacks driving / pushing causal force',
  prakritiStreams: 'Plural latent tendency-lines (prakṛtīnām)',
  obstructionCover: 'Varaṇa: occluding / damming layer inhibiting differentiation',
  obstructionClearing: 'Varaṇa-bheda: removal / rupture of obstruction',
  negativeCatalysis: 'Causal role via removal (subtractive enabling) not production',
  farmerAnalogy: 'kṣetrikavat: irrigator opening channel without propelling water',
  selfDifferentiation: 'PrakritiStreams articulate themselves once cleared',
  citiFunction: 'Reflective clearing intelligence (enabling luminosity structurally)',
  enablingConditionVsEfficient: 'Logical distinction: removal ≠ production',
  reflectionEmptiness: 'Absence of intrinsic producing power in reflective act',
  latentPotentialFlow: 'Inherent tendency toward articulation awaiting clearance',
  misattributionError: 'Projecting productive agency onto nimittaOccasion',
  causalStratification: 'Layering: inherent tendency / obstruction / clearing occasion',
  renewalVector: 'Output of clearing feeding new immediate configurations',
  crosswalkFichteReflection: 'Mapping: obstructionClearing ↔ removal of limit revealing ground',
  crosswalkAbhidharmaRemoval: 'Mapping: negative condition enabling arising of dharmas',
  crosswalkHegelMediation: 'Mapping: pure mediation as removal letting essence appear',
}

// Chunks
const CHUNKS_YS_IV_3 = [
  {
    id: 'ys-iv-3-text',
    title: 'IV.3 Text & Baseline',
    summary: 'Nimitta is non-impelling; differentiation arises by obstruction removal, like a farmer opening channels.',
  },
  {
    id: 'ys-iv-3-causal-logic',
    title: 'Subtractive Causal Logic',
    summary: 'Removal (varaṇa-bheda) functions as negative catalysis—no efficient production, only enabling.',
  },
  {
    id: 'ys-iv-3-citi-role',
    title: 'Role of Citi',
    summary: 'Citi = reflective clearing intelligence: reveals self-differentiation by dissolving obstruction.',
  },
  {
    id: 'ys-iv-3-crosswalks',
    title: 'Crosswalks',
    summary: 'Aligns with Fichtean reflection (limit-removal), Abhidharma removal-condition, Hegelian mediation.',
  },
  {
    id: 'ys-iv-3-errors',
    title: 'Misattribution Errors',
    summary: 'Error: reifying nimitta as producer; ignoring stratified causal structure.',
  },
  {
    id: 'ys-iv-3-renewal',
    title: 'Renewal Vector',
    summary: 'Clearing event outputs renewalVector feeding subsequent conditioning sequences.',
  },
]

// HLO Clauses
const HLOS_YS_IV_3 = [
  {
    id: 'ys-iv-3-hlo-baseline',
    chunkId: 'ys-iv-3-text',
    label: 'Baseline',
    clauses: [
      "tag('sutra','IV.3')",
      'statement(nimittaOccasion ∧ nonImpelling)',
      'differentiation(prakritiStreams) ⇐ obstructionClearing',
      'farmerAnalogy := model(obstructionClearing → selfDifferentiation)',
    ],
  },
  {
    id: 'ys-iv-3-hlo-subtractive',
    chunkId: 'ys-iv-3-causal-logic',
    label: 'Subtractive Cause',
    clauses: [
      'negativeCatalysis := causeType(removal ≠ production)',
      'obstructionClearing ⇒ enable(latentPotentialFlow)',
      'not(produce(prakritiStreams, nimittaOccasion))',
    ],
  },
  {
    id: 'ys-iv-3-hlo-citi',
    chunkId: 'ys-iv-3-citi-role',
    label: 'Citi Function',
    clauses: [
      'citiFunction := reflectionEmptiness ∧ enablingConditionVsEfficient',
      'citiFunction ⇒ orchestrate(obstructionClearing)',
    ],
  },
  {
    id: 'ys-iv-3-hlo-crosswalks',
    chunkId: 'ys-iv-3-crosswalks',
    label: 'Crosswalk',
    clauses: [
      'crosswalkFichteReflection(obstructionClearing ↔ limitRemoval)',
      'crosswalkAbhidharmaRemoval(obstructionClearing ↔ removalCondition)',
      'crosswalkHegelMediation(obstructionClearing ↔ pureMediation)',
    ],
  },
  {
    id: 'ys-iv-3-hlo-errors',
    chunkId: 'ys-iv-3-errors',
    label: 'Errors',
    clauses: [
      'misattributionError ⇐ assign(efficientProduction, nimittaOccasion)',
      'guard := detect(misattributionError)',
    ],
  },
  {
    id: 'ys-iv-3-hlo-renewal',
    chunkId: 'ys-iv-3-renewal',
    label: 'Renewal',
    clauses: [
      'renewalVector := emit(after(obstructionClearing))',
      'renewalVector ⇒ seed(newConfigurationSet)',
    ],
  },
]

export const YS_IV_3_UNIT: DatasetUnit = {
  id: makeUnitId('ys-iv-3'),
  title: 'YS IV.3 — nimittam aprayojakam prakṛtīnām varaṇa-bhedas tu tataḥ kṣetrikavat',
  scope: 'essence',
  logosMode: 'essence',
  synthesis: 'subtractive-enabling',
  faculty: 'buddhi',
  lens: 'yoga',
  chunks: CHUNKS_YS_IV_3 as any,
  hlos: HLOS_YS_IV_3 as any,
}

/* ============================================================
   APPEND EXTENSION (v2 – Internal Self-Construction Alignment)
   Fichte Lecture 16 (self-enclosed being constructing itself)
   mapped onto YS IV.3 (Citi as non-impelling clearing).
   If the prior export object was truncated in the file view,
   ensure it is syntactically closed before this section.
============================================================ */

/* Ontology Extension: Internal Construction vs External Cause */
export const YS_IV_3_ONTOLOGY_EXT = {
  internalSelfConstruction: 'Being constructs itself; no external constructing agency',
  realisticConclusion: 'Inference: construction is by being itself (not by external “we”)',
  rejectedIdealistInference: 'Discarded move: being depends on (externalized) construction act as principle',
  agentAsBeingIdentity: 'We act only insofar as we are identical with being (no independent ego cause)',
  appearanceWe: 'Phenomenal “we” seeming external / free from being',
  essentialWe: 'We = identical participation in self-enclosed being',
  nonExternalBasis: 'Ground of construction cannot lie outside being',
  constructionGround: 'Intrinsic basis enabling self-articulation',
  constructionAsSeeming: 'Appearance that being is constructed (for us) while truly self-arising',
  autonomyWithoutExterior: 'Autonomy defined as interior self-basing, not external independence',
  enablingRemovalAnalogy: 'Parallel: clearing (varaṇa-bheda) reveals self-differentiation; construction “through itself” reveals being',
  citiNonProduction: 'Citi does not produce content; witnesses / clears for self-construction expression',
  misprojectionExternalCause: 'Error of positing an external efficient builder (analogous to nimitta misread)',
  absoluteNecessityMarker: 'Claim that construction-basis is absolutely necessary & non-contingent',
  selfEnclosedConstraint: 'Constraint: nothing can “stand apart”; forbids external constructor hypothesis',
  reflectiveShift: 'Turn from factical objectified being to its genetic internal articulation',
}

/* New Chunks */
const CHUNKS_YS_IV_3_EXT = [
  {
    id: 'ys-iv-3-internal-construction',
    title: 'Internal Self-Construction',
    summary: 'Being constructs itself; no external constructive principle—identity of agent and being.',
  },
  {
    id: 'ys-iv-3-rejected-idealist',
    title: 'Rejected Idealist Inference',
    summary: 'Avoids inference “being depends on its (external) construction”; restricts meaning to objectified appearance only.',
  },
  {
    id: 'ys-iv-3-agent-identity',
    title: 'Agent = Being Identity',
    summary: '“We” construct only insofar as we are being itself, not as an independent ego-pole.',
  },
  {
    id: 'ys-iv-3-clearing-parallel',
    title: 'Clearing Parallel',
    summary: 'Construction-by-being parallels obstruction-clearing: both negate external production, affirm intrinsic articulation.',
  },
  {
    id: 'ys-iv-3-error-projection',
    title: 'Error: External Projection',
    summary: 'Misprojection of an outside cause violates self-enclosed constraint and repeats nimitta efficiency error.',
  },
]

;(CHUNKS_YS_IV_3 as any).push(...CHUNKS_YS_IV_3_EXT)

/* HLO Clauses Extension */
const HLOS_YS_IV_3_EXT = [
  {
    id: 'ys-iv-3-hlo-internal-construction',
    chunkId: 'ys-iv-3-internal-construction',
    label: 'Internal Self-Construction',
    clauses: [
      'internalSelfConstruction := assertion(being ⇒ construct(being))',
      'nonExternalBasis ⇒ require(selfEnclosedConstraint)',
      'constructionGround = being',
    ],
  },
  {
    id: 'ys-iv-3-hlo-rejected-idealist',
    chunkId: 'ys-iv-3-rejected-idealist',
    label: 'Rejected Idealist Inference',
    clauses: [
      'rejectedIdealistInference := infer(beingDependsOn(externalConstructionAct))',
      'negate(rejectedIdealistInference)',
      'limitValidity(constructionAsSeeming, objectifiedAppearanceScope)',
    ],
  },
  {
    id: 'ys-iv-3-hlo-agent-identity',
    chunkId: 'ys-iv-3-agent-identity',
    label: 'Agent Identity',
    clauses: [
      'agentAsBeingIdentity := identity(essentialWe, being)',
      'appearanceWe ≠ essentialWe',
      'misattributionError ⇐ treat(appearanceWe = externalConstructor)',
    ],
  },
  {
    id: 'ys-iv-3-hlo-clearing-parallel',
    chunkId: 'ys-iv-3-clearing-parallel',
    label: 'Clearing Parallel',
    clauses: [
      'enablingRemovalAnalogy := map(obstructionClearing ↔ internalSelfConstruction)',
      'citiFunction ⇒ witness(internalSelfConstruction)',
      'citiNonProduction := negate(produce(content, citiFunction))',
    ],
  },
  {
    id: 'ys-iv-3-hlo-error-projection',
    chunkId: 'ys-iv-3-error-projection',
    label: 'External Projection Error',
    clauses: [
      'misprojectionExternalCause ⇐ posit(externalConstructor)',
      'guard := extend(guard, detect(misprojectionExternalCause))',
      'absoluteNecessityMarker ⇒ forbid(externalConstructor)',
    ],
  },
]

;(HLOS_YS_IV_3 as any).push(...HLOS_YS_IV_3_EXT)

/* Optional: referenced symbols aggregation (if earlier pattern used) */
export const YS_IV_3_REFERENCED_SYMBOLS = [
  'internalSelfConstruction','agentAsBeingIdentity','appearanceWe','essentialWe',
  'citiNonProduction','enablingRemovalAnalogy','nonExternalBasis','constructionGround',
  'misprojectionExternalCause','absoluteNecessityMarker'
]

/* ============================================================
   END APPEND EXTENSION
*/
