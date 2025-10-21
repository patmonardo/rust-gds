import { DatasetUnit, makeUnitId } from '@organon/gdsl/registry/canon';

/*
Meta — Fichte’s Compulsory Method (1804 SK)
Reusable method kit for IV.10–IV.14 crosswalks.
*/

// ---------- Ontology (single source of truth) ----------
export const FICHTE_METHOD_ONTOLOGY = {
  // Method basics
  compulsoryMethod:
    'Immanent/genetic derivation from the absolute; no external premises',
  sayDoCriterion:
    'Validity where saying and doing coincide (qualitative sameness)',
  reconstructionScope:
    'Reconstruction alters form only; cannot add new content',
  reconstructionContradiction:
    'Reconstruction carries saying/doing split until annulled (descent)',

  // “From/Through” justification
  justifyFrom:
    'Justify the “from/through” as reflective necessity (Law of Principles)',
  bareIsObjection: 'Objection: “it is, and that is all” (denies the “from”)',
  reflectiveSplitDeath:
    'Reply: reflection splits being and consciousness (death of reason) unless resolved',
  fromLawOfPrinciples:
    'Either from-itself or from-another (reflective law, not reified thing)',
  fromAsDependentOperator:
    'Crosswalk: “from/through” ↔ dependent-origination operator (hetu/pratītya)',

  // Absolute “from” and light
  absoluteFromInvisible:
    'Absolute, immediate “from” must appear-in-seeing while itself remaining invisible',
  primordialCreationCoOrigination:
    'Absolute thinking/light co‑originates with real creation (carries the “through”)',
  lightEqualsInnerBeing:
    'Light ≡ inner being; no light without creation, no creation without light',
  onenessPenetratesFrom:
    'Light as qualitative oneness permeates the “from”; duality exists only within it',
  appearanceOfAppearance:
    'a–b duality = appearance of appearance (second-order appearing)',

  // Performative validation
  performativeValidation:
    'Presupposition validated in deed: we enacted what we said (saying = doing)',
  retainForDescent:
    'Method: retain the contradiction now; annul it mediately on the descent',

  // Identity as result (not postulate)
  identityAsDerived:
    'Identity (light ≡ being) is a derived result of method, not an assumption',
  critiqueSchellingProof:
    'Schelling’s Absolute Identity cannot be “proved” by SK as external warrant; requires immanent reconstruction',

  // Section hinge
  reflectionToAppearanceHinge:
    'IV.11 functions as hinge from Reflection (conditions) to Appearance (facticity)',

  // Reuse anchors
  linkIV10:
    'IV.10: nityatva (constancy) → anāditva (beginninglessness) as invariance',
  linkIV11: 'IV.11: absence rule under {hetu, phala, āśraya, ālambana}',
  linkIV12:
    'IV.12: time as appearance (adhva-bheda) and existence-as-form (svarūpa)',
  linkIV13: 'IV.13: guṇa substrate of vyakta/sūkṣma states',
  linkIV14: 'IV.14: vastu and pariṇāma-ekatva (identity across change)',
};

// ---------- Chunks ----------
const CHUNKS_FICHTE_METHOD = [
  {
    id: 'fichte-method-basics',
    title: 'Compulsory Method',
    summary: 'Immanent derivation; say/do criterion; scope of reconstruction.',
  },
  {
    id: 'fichte-method-justify-from',
    title: 'Justifying the “From”',
    summary:
      'Law of Principles; objection “bare is”; dependent-origination crosswalk.',
  },
  {
    id: 'fichte-method-absolute-from',
    title: 'Absolute “From” and Light',
    summary:
      'Invisible showing; co‑origination; oneness permeates the “from”; appearance of appearance.',
  },
  {
    id: 'fichte-method-performative',
    title: 'Performative Validation',
    summary: 'Deed as proof; retain-then-annul on descent.',
  },
  {
    id: 'fichte-method-identity',
    title: 'Identity as Derived; Critique of Proof',
    summary:
      'Identity is a result, not a postulate; why SK cannot externally prove Schelling.',
  },
  {
    id: 'fichte-method-hinge',
    title: 'Reflection → Appearance Hinge',
    summary: 'IV.11 hinge and links across IV.10–IV.14.',
  },
];

// ---------- HLO Clauses ----------
const HLOS_FICHTE_METHOD = [
  {
    id: 'fichte-method-hlo-basics',
    chunkId: 'fichte-method-basics',
    label: 'Method',
    clauses: [
      'compulsoryMethod := assert(immanent(genetic))',
      'sayDoCriterion := require(say(x) == do(x))',
      'reconstructionScope := limit(reconstruction, to = formChangeOnly)',
      'reconstructionContradiction := note(carries(sayDoSplit))',
    ],
  },
  {
    id: 'fichte-method-hlo-justify-from',
    chunkId: 'fichte-method-justify-from',
    label: 'From/Through',
    clauses: [
      'justifyFrom := task(justify("from"))',
      'fromLawOfPrinciples := law( (¬fromAnother ⇒ fromItself) ∧ (¬fromItself ⇒ fromAnother) )',
      'bareIsObjection := pose("it is, and that is all")',
      'reflectiveSplitDeath := reply(split(being, consciousness))',
      'fromAsDependentOperator := map("from/through" ↔ hetu/pratityasamutpada)',
    ],
  },
  {
    id: 'fichte-method-hlo-absolute-from',
    chunkId: 'fichte-method-absolute-from',
    label: 'Light/From',
    clauses: [
      'absoluteFromInvisible := assert(mustAppearIn(seeing, absoluteFrom) ∧ invisible(absoluteFrom))',
      'primordialCreationCoOrigination := assert(light cooriginates_with creation and carries("through"))',
      'lightEqualsInnerBeing := assert(light ≡ innerBeing)',
      'onenessPenetratesFrom := assert(penetrates(oneness(light), "from"))',
      'appearanceOfAppearance := assert(a_b == appearance(appearance))',
    ],
  },
  {
    id: 'fichte-method-hlo-performative',
    chunkId: 'fichte-method-performative',
    label: 'Deed',
    clauses: [
      'performativeValidation := assert(say(x) == do(x))',
      'retainForDescent := method(hold(contradiction) → annul(onDescent))',
    ],
  },
  {
    id: 'fichte-method-hlo-identity',
    chunkId: 'fichte-method-identity',
    label: 'Identity',
    clauses: [
      'identityAsDerived := conclude(identity(light, being) as result(method))',
      'critiqueSchellingProof := assert(not(proveBy(SK, external(AbsoluteIdentity))))',
    ],
  },
  {
    id: 'fichte-method-hlo-hinge',
    chunkId: 'fichte-method-hinge',
    label: 'Hinge/Links',
    clauses: [
      'reflectionToAppearanceHinge := mark(IV_11 as hinge)',
      'linkIV10 := reference(ys_iv_10)',
      'linkIV11 := reference(ys_iv_11)',
      'linkIV12 := reference(ys_iv_12)',
      'linkIV13 := reference(ys_iv_13)',
      'linkIV14 := reference(ys_iv_14)',
    ],
  },
];

// ---------- Export Unit ----------
export const FICHTE_METHOD_UNIT: DatasetUnit = {
  id: makeUnitId('fichte-method'),
  title: 'Meta — Fichte’s Compulsory Method (1804 SK)',
  scope: 'appearance',
  logosMode: 'appearance',
  synthesis: 'method-immanent-genesis',
  faculty: 'buddhi',
  lens: 'yoga',
  chunks: CHUNKS_FICHTE_METHOD as any,
  hlos: HLOS_FICHTE_METHOD as any,
};

export const FICHTE_METHOD_SYMBOLS = Object.keys(FICHTE_METHOD_ONTOLOGY);
