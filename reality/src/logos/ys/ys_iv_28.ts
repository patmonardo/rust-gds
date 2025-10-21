import { DatasetUnit, makeUnitId } from '@organon/gdsl/registry/canon';

/*
C. ACTUALITY → Absolute Realization (closure and method)

YS IV.28 — hānam eṣām kleśavad uktam

“Their removal is said to be like (that of) the kleśas.”
*/

// ---------- Ontology ----------
export const YS_IV_28_ONTOLOGY = {
  hanam: 'Removal/abandonment (hāna)',
  esam: '“Of these” — the intruding pratyayas arising in gaps (IV.27)',
  klesavad: 'Like the kleśas — by the already stated method of kleśa-removal',
  uktam: 'As stated/said before — refers back to prior prescriptions',
  intrusionsFromSamskara:
    'Non‑viveka pratyayas arising from saṁskāras (from IV.27)',
  vivekaCurrent: 'Discriminative current established in IV.26',
  methodReapplication:
    'Reapplication of previously stated method, not invention of a new one',
  // Method anchors (Book II cross‑reference, abstracted)
  klesaMethod:
    'Kleśa‑removal method: viveka‑khyāti, pratyaya‑nirodha, saṁskāra‑kṣaya, sattva‑śuddhi (as previously stated)',
  // Crosswalks
  fichteMethodCompass:
    'Fichte: keep penetrating method‑remarks present; method is the compass through the maze',
  fichteCreativeMethod:
    'Method becomes absolutely creative; it must justify itself, then guide application',
  // Guards
  errorNewAsceticMethod:
    'Error: inventing a new ascetic suppression; instruction is re‑apply the stated kleśa method',
  errorBlameWitness:
    'Error: attributing intrusions to witness failure; non‑transference stands',
  errorWorldDenial:
    'Error: denying appearance instead of resolving latencies by method',
};

// ---------- Chunks ----------
const CHUNKS_YS_IV_28 = [
  {
    id: 'ys-iv-28-text',
    title: 'IV.28 Text & Baseline',
    summary: '“Their removal is as stated for the kleśas.”',
  },
  {
    id: 'ys-iv-28-semantics',
    title: 'Semantics: hāna / eṣām / kleśavat / uktam',
    summary:
      'Removal of the IV.27 intrusions by the already stated kleśa‑method; no new procedure.',
  },
  {
    id: 'ys-iv-28-method',
    title: 'Method: Reapplication of Prior Discipline',
    summary:
      'Re‑establish viveka continuity; reduce saṁskāra fuel; rely on previously stated prescriptions.',
  },
  {
    id: 'ys-iv-28-crosswalk',
    title: 'Crosswalk (Fichte — Method/Compass)',
    summary:
      'Keep the method‑remarks present; the compass through the maze ensures categorical advance.',
  },
  {
    id: 'ys-iv-28-guards',
    title: 'Guards',
    summary: 'No novel suppression; no witness blame; no world‑denial.',
  },
  {
    id: 'ys-iv-28-bridges',
    title: 'Bridges → IV.29+',
    summary:
      'From stabilized method to higher abatements and Absolute stance consolidation.',
  },
];

// ---------- HLO Clauses ----------
const HLOS_YS_IV_28 = [
  {
    id: 'ys-iv-28-hlo-text',
    chunkId: 'ys-iv-28-text',
    label: 'Baseline',
    clauses: [
      "tag('sutra','IV.28')",
      'assert(hanam(esam))',
      'define(esam := intrusionsFromSamskara := import(IV_27.pratyayaAntarani))',
      'conclude(klesavad ∧ uktam)',
    ],
  },
  {
    id: 'ys-iv-28-hlo-semantics',
    chunkId: 'ys-iv-28-semantics',
    label: 'Semantics',
    clauses: [
      'define(hanam := removal/abandonment)',
      'define(klesavad := like_the_klesas_in_removal)',
      'define(uktam := as_previously_stated)',
      'assert(vivekaCurrent := import(IV_26.discriminativeCurrent))',
      'assert(klesaMethod := import(II.klesa_removal_method | previously_stated))',
      'methodReapplication := assert(reapply(klesaMethod) to = intrusionsFromSamskara)',
    ],
  },
  {
    id: 'ys-iv-28-hlo-method',
    chunkId: 'ys-iv-28-method',
    label: 'Method',
    clauses: [
      'reestablish := method(restore(vivekaCurrent) ∧ seal_gaps := import(IV_27.sealGaps))',
      'reduce_samskara := method(saṁskāra_kṣaya via klesaMethod)',
      'sattva_shuddhi := method(purify_sattva via dhyāna ∧ nirodha)',
      'conclude(hanam(esam) by = {reestablish, reduce_samskara, sattva_shuddhi})',
    ],
  },
  {
    id: 'ys-iv-28-hlo-crosswalk',
    chunkId: 'ys-iv-28-crosswalk',
    label: 'Crosswalk',
    clauses: [
      'fichteMethodCompass := note(keep_method_remarks_present to_navigate_maze)',
      'fichteCreativeMethod := map(method_self_justified ⇒ categorical_application without_arbitrariness)',
    ],
  },
  {
    id: 'ys-iv-28-hlo-guards',
    chunkId: 'ys-iv-28-guards',
    label: 'Guards',
    clauses: [
      'errorNewAsceticMethod := flag(invent_new_suppression)',
      'errorBlameWitness := flag(assign_fault_to(seer))',
      'errorWorldDenial := flag(deny(appearance) as remedy)',
    ],
  },
  {
    id: 'ys-iv-28-hlo-bridges',
    chunkId: 'ys-iv-28-bridges',
    label: 'Bridges',
    clauses: [
      'prepare(IV_29_plus := consolidate(method → steady_kaivalya_vector))',
    ],
  },
];

// ---------- Export Unit ----------
export const YS_IV_28_UNIT: DatasetUnit = {
  id: makeUnitId('ys-iv-28'),
  title: 'YS IV.28 — hānam eṣām kleśavad uktam',
  scope: 'actuality',
  logosMode: 'appearance',
  synthesis:
    'Remove the residual intrusions exactly as the kleśas were removed; no new method—reapply the stated discipline.',
  faculty: 'buddhi',
  lens: 'yoga',
  chunks: CHUNKS_YS_IV_28 as any,
  hlos: HLOS_YS_IV_28 as any,
};

export const YS_IV_28_SYMBOLS = Object.keys(YS_IV_28_ONTOLOGY);
