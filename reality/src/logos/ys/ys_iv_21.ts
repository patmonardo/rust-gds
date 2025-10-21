import { DatasetUnit, makeUnitId } from '@organon/gdsl/registry/canon';

/*
YS IV.21 — citta-antara-dṛśye buddhi-buddher atiprasaṅgaḥ smṛti-saṅkaraś ca

“If a mind were seen by another mind, there would be an over-application (absurd regress) of intellect upon intellect, and a confusion of memories.”

Reading
- Forbid mind-seeing-mind: prevents infinite regress (atiprasaṅga) and memory-stream mixing (smṛti-saṅkara).
- Locks IV.19–IV.20: citta is seen (not self-luminous) and cannot simultaneously determine seer/seen.
- Forces witness solution: Purusha = non-agentive seer; cittas appear to it, not to each other directly.
- Fichte L21: higher knowing’s hiatus (separation of pure reason from appearance) underwrites non-transparency of minds.
- Hegel crosswalk: avoid collapsing subjective spirits; recognition is mediated, not immediate inter-mental luminosity.
*/

// ---------- Ontology ----------
export const YS_IV_21_ONTOLOGY = {
  citta: 'Mind-stream; locus of vṛttis (seen here)',
  buddhi: 'Intellect/discriminative faculty (here: as object too)',
  cittaAntaraDrsya: 'Mind seen by another mind (assumption denied by reductio)',
  buddhiBuddher:
    'Intellect upon intellect (stacking seer/seen at the same level)',
  atiprasanga:
    'Unwanted overextension/absurd regress (infinite layering of minds seeing minds)',
  smritiSankara: 'Intermixing/confusion of memory streams across minds',
  notSelfLuminousCitta: 'Mind is not self-luminous (IV.19)',
  noDoubleDetermination:
    'No simultaneous determination of both seer/seen in one act (IV.20)',
  purusha: 'Non-agentive, unchanging seer (witness-only)',
  worldnessMediation:
    'Intersubjective invariants via world-objects/signs, not direct mind-visibility',
  // Crosswalks
  fichteHiatum:
    'Projection per hiatus separates pure reason from appearance; forbids mind-on-mind transparency',
  hegelSubjectiveSpirit:
    'Subjective spirits do not immediately intuit each other’s innerness; mediation via world/recognition',
  // Bridges
  seerWithoutObjecthood: 'Witness is seer-only; not an object among objects (locks IV.20 → IV.21)',
  // Guards
  errorIntermentalTransparency:
    'Asserting direct visibility of one mind to another',
  errorMemoryFusion:
    'Allowing cross-mind memory fusion (destroys personal identity/sequence)',
};

// ---------- Chunks ----------
const CHUNKS_YS_IV_21 = [
  {
    id: 'ys-iv-21-text',
    title: 'IV.21 Text & Baseline',
    summary:
      'If a mind were seen by another mind → absurd regress and memory confusion.',
  },
  {
    id: 'ys-iv-21-semantics',
    title: 'Semantics',
    summary:
      'Parse citta-antara-dṛśye; buddhi-buddher; atiprasaṅga; smṛti-saṅkara.',
  },
  {
    id: 'ys-iv-21-argument',
    title: 'Reductio Argument',
    summary:
      'Assume mind-sees-mind; derive regress and memory-mixing; reject assumption.',
  },
  {
    id: 'ys-iv-21-crosswalk',
    title: 'Crosswalk (Fichte/Hegel)',
    summary:
      'Hiatus of higher knowing; mediated recognition; witness solution.',
  },
  {
    id: 'ys-iv-21-bridges',
    title: 'Bridge → Absolute (IV.22–IV.24)',
    summary:
      'From witness-locked appearance to Absolute Relation and modality.',
  },
  {
    id: 'ys-iv-21-errors',
    title: 'Error Modes',
    summary:
      'Inter-mental transparency; memory fusion; collapse of witness into mind.',
  },
];

// ---------- HLO Clauses ----------
const HLOS_YS_IV_21 = [
  {
    id: 'ys-iv-21-hlo-text',
    chunkId: 'ys-iv-21-text',
    label: 'Baseline',
    clauses: [
      "tag('sutra','IV.21')",
      'assume(cittaAntaraDrsya)',
      'derive(atiprasanga ∧ smritiSankara)',
      'reject(cittaAntaraDrsya)',
    ],
  },
  {
    id: 'ys-iv-21-hlo-semantics',
    chunkId: 'ys-iv-21-semantics',
    label: 'Semantics',
    clauses: [
      'define(cittaAntaraDrsya := mind_seen_by(another_mind))',
      'define(buddhiBuddher := intellect_on_intellect layering)',
      'define(atiprasanga := absurd_overextension/regress)',
      'define(smritiSankara := confusion(memory_streams across minds))', // fixed typo
    ],
  },
  {
    id: 'ys-iv-21-hlo-argument',
    chunkId: 'ys-iv-21-argument',
    label: 'Reductio',
    clauses: [
      'premise1 := import(IV_19.notSelfLuminousCitta)',
      'premise2 := import(IV_20.noDoubleDetermination)',
      'if(assume(cittaAntaraDrsya)) then {',
      '  atiprasanga := spawn(infinite_stack(buddhiBuddher))',
      '  smritiSankara := violate(separation(memory_streams))',
      '  contradiction := {premise1, premise2} // clashes with prior constraints',
      '}',
      'therefore(reject(cittaAntaraDrsya) ∧ assert(purusha = witness_only))',
      'worldnessMediation := assert(knowledge_of_other_minds via signs/appearances, not direct vision)',
    ],
  },
  {
    id: 'ys-iv-21-hlo-crosswalk',
    chunkId: 'ys-iv-21-crosswalk',
    label: 'Crosswalk',
    clauses: [
      'fichteHiatum := link(separate(pure_reason, appearance) ⇒ forbid(mind_on_mind_transparency))',
      'hegelSubjectiveSpirit := note(recognition is mediated; no immediate intuition of the other’s innerness)',
      'map(purusha ↔ witness that avoids regress; citta ↔ seen/appearing)',
    ],
  },
  {
    id: 'ys-iv-21-hlo-bridges',
    chunkId: 'ys-iv-21-bridges',
    label: 'Bridge',
    clauses: [
      'prepare(IV_22_absolute_entry := witness-locked appearance → absolute_relation)',
      'prepare(IV_23_IV_24_modality := handle(possibility/necessity under absolute stance))',
    ],
  },
  {
    id: 'ys-iv-21-hlo-errors',
    chunkId: 'ys-iv-21-errors',
    label: 'Errors',
    clauses: [
      'errorIntermentalTransparency := flag(assert(cittaAntaraDrsya))',
      'errorMemoryFusion := flag(assert(crossMindMemoryMerge))',
      'flag(collapse(purusha → citta))',
    ],
  },
];

// ---------- Fichte L21 — Single Segment (Reason ↔ Understanding; Persistence-as-Genesis) ----------
Object.assign(YS_IV_21_ONTOLOGY, {
  persistenceAsGenesis:
    'Pure being/persistence = genesis (qualitative oneness of light)',
  absoluteInwardAwareness:
    'Inward self-awareness without external perceiving/knowing/intuition',
  oneSidedOrder:
    'Original principle → principled thing in a one-sided (non-reciprocal) order',
  pureReasonNegatesGenesis:
    'Pure reason, a priori, independent of all genesis; negates genesis as absolute',
  understandingAsReconstruction:
    'Higher knowing reconstructs the non-appearing original genesis (clarifies its terms)',
  mutualPresupposition:
    'No insight into reason without presupposing understanding as absolute; no insight into understanding except via reason’s absolute negation',
  highestStandpointOneness:
    'We are the understanding of reason and the reason of understanding—both in oneness (aspectual, not two things)',
  projectionHiatumHere:
    'A necessary gap (hiatum) separates pure reason’s oneness from appearance (non-irrational separation)',
});

CHUNKS_YS_IV_21.push({
  id: 'ys-iv-21-fichte-segment',
  title: 'Fichte L21 — Reason/Understanding Hinge',
  summary:
    'Persistence-as-genesis; inward, one-sided order; reason negates genesis-as-absolute; understanding reconstructs genesis; both posited in one standpoint.',
});

HLOS_YS_IV_21.push({
  id: 'ys-iv-21-hlo-fichte-segment',
  chunkId: 'ys-iv-21-fichte-segment',
  label: 'Reason/Understanding',
  clauses: [
    'persistenceAsGenesis := assert(being == genesis at qualitative_oneness)',
    'absoluteInwardAwareness := assert(inward_awareness without {external_perceiving, knowing, intuiting})',
    'oneSidedOrder := assert(order(principle → principled_thing) ∧ ¬reciprocal)',
    'pureReasonNegatesGenesis := assert(pure_reason ⊥ absolute_genesis)',
    'understandingAsReconstruction := assert(higher_knowing == reconstruct(non_appearing_original_genesis))',
    'mutualPresupposition := assert(no_insight(reason) without presuppose(understanding as absolute) ∧ no_insight(understanding) except_via(negation_by reason))',
    'highestStandpointOneness := assert(posited(both(reason, understanding)) ∧ one(standpoint) ∧ negate_one_to_show_other)',
    'projectionHiatumHere := link(fichteHiatum)',
    // Fit to sutra’s reductio
    'reject(cittaAntaraDrsya) // inter-mental transparency contradicts one-sided order and hiatus',
  ],
});
//

// ---------- Sattva-only Witness (Seer without Objecthood) ----------
Object.assign(YS_IV_21_ONTOLOGY, {
  pureSattvaWitness:
    'Purusha witnesses only śuddha‑sattva buddhi (pure sattva transparency); does not enter tamasic reflective displays',
  manasTransactionalWorld:
    'Transactional world (vyavahāra) of manas is what appears to Purusha as citta‑vṛtti under sattva',
  tamasicMayicDisplay:
    'Tāmasic/Mayic reflective displays are not directly intuited by Purusha; burden of jīva/puman',
  kaivalyaOrientation:
    'Seer‑only isolation (kaivalya): witness without objecthood among objects; frees from tamasic entanglement',
  purushaSeesCittaVritti:
    'Purusha always sees citta‑vṛttis; knownness mediated by sattva transparency, not tamas',
  kantContrast:
    'Against Kant’s “God = intellectual intuition of sum total of reality”: Yoga holds seer‑only of sattva vṛttis',
  jagatMithyaCrosswalk:
    'Jagat mithyā: world as appearance; not directly intuited by Purusha (seer of sattva only)',
})

CHUNKS_YS_IV_21.push(
  {
    id: 'ys-iv-21-sattva-witness',
    title: 'Seer without Objecthood (Śuddha‑Sattva Witness)',
    summary:
      'Purusha witnesses pure sattva vṛttis; does not enter tamasic displays; kaivalya orientation.',
  },
  {
    id: 'ys-iv-21-contrast',
    title: 'Contrast: Kant vs Yoga',
    summary:
      'No “sum‑total” intellectual intuition; seer‑only of sattva vṛttis; jagat mithyā as appearance.',
  },
)

HLOS_YS_IV_21.push(
  {
    id: 'ys-iv-21-hlo-sattva-witness',
    chunkId: 'ys-iv-21-sattva-witness',
    label: 'Sattva Witness',
    clauses: [
      'assert(seerWithoutObjecthood)',
      'purushaSeesCittaVritti := assert(always(purusha sees citta.vritti))',
      'pureSattvaWitness := constrain(citta.vritti, by = sattva_transparency)',
      'deny(purusha enters tamasicMayicDisplay)',
      'manasTransactionalWorld := scope(vyavahara(manas) as what_appears_to(purusha))',
      'kaivalyaOrientation := link(seerWithoutObjecthood → isolation(kaivalya))',
    ],
  },
  {
    id: 'ys-iv-21-hlo-contrast',
    chunkId: 'ys-iv-21-contrast',
    label: 'Kant/Vedānta Contrast',
    clauses: [
      'kantContrast := contrast(kant:intellectualIntuition(sum_total_reality), yoga:seer_only(sattva_vrittis))',
      'jagatMithyaCrosswalk := assert(world_as(appearance) ∧ not_directly_intuited_by(purusha))',
      'link(worldnessMediation, to = {manasTransactionalWorld})',
    ],
  },
)

// ---------- Export Unit ----------
export const YS_IV_21_UNIT: DatasetUnit = {
  id: makeUnitId('ys-iv-21'),
  title:
    'YS IV.21 — citta-antara-dṛśye buddhi-buddher atiprasaṅgaḥ smṛti-saṅkaraś ca',
  scope: 'appearance',
  logosMode: 'appearance',
  synthesis: 'witness-lock: no mind-on-mind seeing; avoid regress/memory-mix',
  faculty: 'buddhi',
  lens: 'yoga',
  chunks: CHUNKS_YS_IV_21 as any,
  hlos: HLOS_YS_IV_21 as any,
};

export const YS_IV_21_SYMBOLS = Object.keys(YS_IV_21_ONTOLOGY);
