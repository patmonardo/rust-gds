import { DatasetUnit, makeUnitId } from '@organon/gdsl/registry/canon';

/*
YS IV.12 — atīta-anāgataṁ svarūpato ’sty adhva-bhedād dharmāṇām

Seed for debate: svarūpataḥ — “in its own form” vs “in my own form”
You note: avoid a naive realistic reading; press the deictic/reflexive force (“my own form”).
*/

// ---------- Ontology (single source of truth) ----------
export const YS_IV_12_ONTOLOGY = {
  // Lexical
  atita: 'Past modality',
  anagata: 'Future modality',
  vartamana: 'Present modality (contrast, implicit)',
  dharmanam:
    'Genitive plural “of the dharmas” (determinacies/units of appearance)',
  adhva: 'Temporal path (past/present/future)',
  adhvaBheda: 'Difference of temporal paths across dharmas',
  svarupa: 'Own-form; form-existence (not necessarily present-instantiation)',
  svarupatah:
    'In/by respect to own-form (instrumental/ablative sense: “as to [their] own-form”)',

  // Two readings to seed the debate
  svarupaCommonReading:
    '“in its own form” — impersonal/objective possession (standard translation)',
  svarupaDeicticReading:
    '“in my own form” — deictic/reflexive possession anchored in the seer/citta',
  svarupaReadingNote:
    'Debate focuses on whose “own” is indexed: the object’s (its) vs the locus of appearing (my/citta).',

  // Appearance thesis
  timeAsAppearance: 'Time is a mode of appearance (not an external container)',
  existenceAsForm:
    'Existence here = existence-as-form (svarūpa), not presence-now',
  globalAppearanceLaw:
    'Past/future exist as forms because dharmas differ by temporal path (adhva-bheda)',

  // Pitfalls to avoid
  errorPresentism: 'Denying form-existence of past/future (naive presentism)',
  errorNaiveRealismSvarupa:
    'Reading svarūpa as realistic present thinghood rather than form-existence (or deictic own-form)',
};

// ---------- Chunks ----------
const CHUNKS_YS_IV_12 = [
  {
    id: 'ys-iv-12-text',
    title: 'IV.12 Text',
    summary: 'atīta-anāgataṁ svarūpato ’sty adhva-bhedād dharmāṇām',
  },
  {
    id: 'ys-iv-12-parse',
    title: 'Literal Parse',
    summary:
      'Past/future exist as to own-form because of temporal-path difference of the dharmas.',
  },
  {
    id: 'ys-iv-12-svarupa-debate',
    title: 'Svarūpa Debate',
    summary:
      'Seed both readings: “its own form” (common) vs “my own form” (deictic/reflexive).',
  },
  {
    id: 'ys-iv-12-appearance',
    title: 'Time as Appearance',
    summary:
      'Treat time as a mode of appearance; existence means existence-as-form.',
  },
  {
    id: 'ys-iv-12-errors',
    title: 'Error Modes',
    summary: 'Avoid naive presentism and naive realist readings of svarūpa.',
  },
];

// ---------- HLO Clauses ----------
const HLOS_YS_IV_12 = [
  {
    id: 'ys-iv-12-hlo-text',
    chunkId: 'ys-iv-12-text',
    label: 'Sutra',
    clauses: ["tag('sutra','IV.12')"],
  },
  {
    id: 'ys-iv-12-hlo-parse',
    chunkId: 'ys-iv-12-parse',
    label: 'Parse',
    clauses: [
      'modalities := {atita, anagata, vartamana}',
      'adhva := modalities',
      'adhvaBheda := differentiate(dharmanam, by = adhva)',
      'svarupatah := mode(ownForm)',
      'globalAppearanceLaw := rule(existsAsForm({atita, anagata}) ⇐ adhvaBheda)',
    ],
  },
  {
    id: 'ys-iv-12-hlo-svarupa-debate',
    chunkId: 'ys-iv-12-svarupa-debate',
    label: 'Debate Seed',
    clauses: [
      'svarupaCommonReading := gloss("in its own form" — impersonal/objective)',
      'svarupaDeicticReading := gloss("in my own form" — deictic/reflexive to locus-of-appearing)',
      'svarupaReadingNote := note(indexing_problem(owner(ownForm)))',
    ],
  },
  {
    id: 'ys-iv-12-hlo-appearance',
    chunkId: 'ys-iv-12-appearance',
    label: 'Appearance',
    clauses: [
      'timeAsAppearance := assert(mode(appearance, time))',
      'existenceAsForm := clarify(exist(x, svarupa) ≠ presentInstantiation(x))',
    ],
  },
  {
    id: 'ys-iv-12-hlo-errors',
    chunkId: 'ys-iv-12-errors',
    label: 'Errors',
    clauses: [
      'errorPresentism := flag(deny(existsAsForm({atita, anagata})))',
      'errorNaiveRealismSvarupa := flag(read(svarupa as presentThinghood))',
    ],
  },
];

// ---------- Ontology (Fichte L20 crosswalk) ----------
Object.assign(YS_IV_12_ONTOLOGY, {
  lightBeingOne: 'Being and light are one self-enclosed living oneness',
  ordinaryConsciousnessManifold:
    'In light’s existence (ordinary consciousness) a manifold appears',
  groundOfManifoldInLight:
    'Ground for the manifold must appear in the light itself (absolute oneness) and in its manifestation',
  deriveAppearanceFromLight:
    'Derive the appearance of the light from the light; the manifold arises therein',
  presentAppearanceAsSuch:
    'Task: present appearance in general and as such (principle-level, not empirical)',
  geneticSupersedesEmpirical:
    'Once the principle is shown a priori, empirical appeal falls away; genetic replaces factical',
});

// ---------- Chunks (Fichte L20) ----------
CHUNKS_YS_IV_12.push(
  {
    id: 'ys-iv-12-fichte20-outline',
    title: 'Fichte L20 — Outline',
    summary:
      'Being ≡ light; ground of manifold must appear in light; derive appearance of light from light.',
  },
  {
    id: 'ys-iv-12-fichte20-genesis',
    title: 'Fichte L20 — Genesis vs Empirical',
    summary:
      'Present appearance as such; genetic explanation replaces empirical appeal.',
  },
);

// ---------- HLO Clauses (Fichte L20) ----------
HLOS_YS_IV_12.push(
  {
    id: 'ys-iv-12-hlo-fichte20-outline',
    chunkId: 'ys-iv-12-fichte20-outline',
    label: 'Light/Being and Ground',
    clauses: [
      'lightBeingOne := assert(being ≡ light)',
      'ordinaryConsciousnessManifold := observe(manifold in light_existence)',
      'groundOfManifoldInLight := require(ground(manifold) appear_in light ∧ its_manifestation)',
      'deriveAppearanceFromLight := method(derive(appearance(light), from = light))',
    ],
  },
  {
    id: 'ys-iv-12-hlo-fichte20-genesis',
    chunkId: 'ys-iv-12-fichte20-genesis',
    label: 'Appearance-as-such',
    clauses: [
      'presentAppearanceAsSuch := task(present(appearance, as_such))',
      'geneticSupersedesEmpirical := assert(genetic(principle) ⇒ drop(empiricalAppeal))',
    ],
  },
);

// ---------- Ontology (Fichte L20 continuation) ----------
Object.assign(YS_IV_12_ONTOLOGY, {
  presuppositionAccompaniesCreation:
    'True seeing/light must accompany actual Creation; light is immanent creation (absolute “from”)',
  presuppositionMereButWarranted:
    'Presupposition proves itself by bare possibility and facticity (performative warrant)',
  weAsKnowingConductProof:
    'We are knowing/light; we conducted the proof and posited knowing as “from”',
  knowingOnlyWithinView:
    'Knowing can be both “in itself” and “from” only within a view into itself',
  notByFreedomButSelfMaking:
    'Not made by freedom (We) but made itself directly; auto-posit of the presupposition',
  shouldOperator:
    'Earlier premises carried a “should”: hypothetical conditioning characteristic of reflection',
  manifestnessGrips:
    'From energetic thinking, manifestness grips and carries us to its conditioned premise',
  reconstructionSecondaryKnowing:
    'Reconstruction transfers unconditional content into a conditioned relation (secondary, apparent knowing)',
  hypotheticalOnlyInSystems:
    'Other systems remain with hypothetical premises and arbitrary reliance, risking skepticism',
});

// ---------- Chunks (Fichte L20 continuation) ----------
CHUNKS_YS_IV_12.push(
  {
    id: 'ys-iv-12-presupposition-validated',
    title: 'Presupposition: Validated by Deed',
    summary:
      'Bare possibility and facticity warrant the presupposition; we as knowing/light enacted it.',
  },
  {
    id: 'ys-iv-12-we-and-should',
    title: 'We, Freedom, and the “Should”',
    summary:
      'Not by freedom but self-making; earlier ascent used hypothetical “should”-premises; manifestness grips.',
  },
  {
    id: 'ys-iv-12-reconstruction-vs-absolute',
    title: 'Reconstruction vs Absolute Knowing',
    summary:
      'Reconstruction is secondary knowing: conditions the unconditional; appearance-level only.',
  },
  {
    id: 'ys-iv-12-skepticism-risk',
    title: 'Hypothesis Reliance and Skepticism',
    summary:
      'Systems relying on hypothetical premises risk skepticism without genetic grounding.',
  },
);

// ---------- HLO Clauses (Fichte L20 continuation) ----------
HLOS_YS_IV_12.push(
  {
    id: 'ys-iv-12-hlo-presupposition-validated',
    chunkId: 'ys-iv-12-presupposition-validated',
    label: 'Deed/Warrant',
    clauses: [
      'presuppositionAccompaniesCreation := assert(light accompanies(Creation) as absoluteFrom)',
      'presuppositionMereButWarranted := argue(possible ∧ factical ⇒ warranted)',
      'weAsKnowingConductProof := assert(we == knowing == light ∧ enact(proof))',
      'knowingOnlyWithinView := assert(both(in_itself, from) only_within(view(into_itself)))',
    ],
  },
  {
    id: 'ys-iv-12-hlo-we-and-should',
    chunkId: 'ys-iv-12-we-and-should',
    label: 'Method/Ascent',
    clauses: [
      'notByFreedomButSelfMaking := assert(autoPosit(presupposition))',
      'shouldOperator := note(hypothetical(conditioning) in ascent)',
      'manifestnessGrips := assert(carried_by(manifestness) to(conditionedPremise))',
    ],
  },
  {
    id: 'ys-iv-12-hlo-reconstruction-vs-absolute',
    chunkId: 'ys-iv-12-reconstruction-vs-absolute',
    label: 'Levels of Knowing',
    clauses: [
      'reconstructionSecondaryKnowing := define(reconstruction == transfer(unconditional → conditionedRelation))',
      'link(reconstructionSecondaryKnowing, timeAsAppearance)',
    ],
  },
  {
    id: 'ys-iv-12-hlo-skepticism-risk',
    chunkId: 'ys-iv-12-skepticism-risk',
    label: 'Risk',
    clauses: [
      'hypotheticalOnlyInSystems := diagnose(rely(hypotheticalPremises) ⇒ risk(skepticism))',
      'require(geneticSupersedesEmpirical)',
    ],
  },
);

// ---------- Export Unit ----------
export const YS_IV_12_UNIT: DatasetUnit = {
  id: makeUnitId('ys-iv-12'),
  title: 'YS IV.12 — atīta-anāgataṁ svarūpato ’sty adhva-bhedād dharmāṇām',
  scope: 'appearance',
  logosMode: 'appearance',
  synthesis: 'time-as-appearance-own-form',
  faculty: 'buddhi',
  lens: 'yoga',
  chunks: CHUNKS_YS_IV_12 as any,
  hlos: HLOS_YS_IV_12 as any,
};

// FIX: export symbols via keys to avoid drift
export const YS_IV_12_SYMBOLS = Object.keys(YS_IV_12_ONTOLOGY).sort();
