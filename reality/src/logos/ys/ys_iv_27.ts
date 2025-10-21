import { DatasetUnit, makeUnitId } from '@organon/gdsl/registry/canon'

/*
C. ACTUALITY → Absolute Realization (concluding cautions)

YS IV.27 — tad-chidreṣu pratyayāntarāṇi saṁskārebhyaḥ

“In the gaps of that (discriminative current), other cognitions arise, from latent impressions.”
*/

// ---------- Ontology ----------
export const YS_IV_27_ONTOLOGY = {
  tadChidresu:
    'In the gaps/chinks of that (tad = the discriminative current/viveka-trajectory just established)',
  pratyayaAntarani:
    'Other cognitions/ideas intrude (non-viveka pratyayas reassert themselves)',
  samskarebhyah:
    'From saṁskāras (latent impressions) as causal source of intrusions',
  gap:
    'Momentary lapse/discontinuity of the viveka-current in citta (attention slack, sattva drop, habit surge)',
  intrusion:
    'Arising of non-discriminative pratyaya during a gap; not a fall of the witness',
  discriminativeCurrent:
    'The viveka-dominant flow in citta (from IV.26), whose continuity is at issue',
  kaivalyaVector:
    'Forward-weight toward kaivalya; can be perturbed by gaps but not reversed if insight is firm',
  // Carry-overs
  viseshaDarsina: 'Discriminative seer (IV.25) — precondition for the viveka-current',
  parartham: 'Mind is for another (IV.24)',
  sarvartham: 'Mind is omni-instrumental (IV.23)',
  apratisankramaya: 'Non-transference of consciousness (IV.22)',
  seerWithoutObjecthood: 'Witness-only seer (IV.21–IV.22)',
  notSelfLuminousCitta: 'Mind is not self-luminous (IV.19)',
  // Diagnostics/Remedies
  gapDiagnostics:
    'Diagnostics for gaps: fatigue, inattentiveness, strong vasana activation, environmental triggers',
  sealGaps:
    'Seal gaps by stabilizing viveka (continuous insight), strengthening sattva, and reducing vasana fuel',
  remedialPractice:
    'Remedies: sustained viveka-khyāti, dhyāna continuity, pratyaya-nirodha refresh, sattva-purification',
  // Crosswalks
  fichteFromOfFromChinks:
    'Self-intersecting “from-of-from” permits chinks where appearance can intrude if not continuously held',
  hegelActualityWithRemainders:
    'Actuality contains remainders/contingent intrusions until freedom is complete',
  // Guards
  errorSeerFalls:
    'Error: treating intrusions as the seer “falling” or transferring; witness non-transference stands',
  errorMoralizeIntrusions:
    'Error: moralizing intrusions as sin/failure; they are mechanical vasana surfacing',
  errorWorldDenial:
    'Error: reacting with denial of appearance; the task is sealing gaps, not suppressing world-as-appearing',
}

// ---------- Chunks ----------
const CHUNKS_YS_IV_27 = [
  {
    id: 'ys-iv-27-text',
    title: 'IV.27 Text & Baseline',
    summary:
      'In the gaps of the discriminative current, other cognitions arise from saṁskāras.',
  },
  {
    id: 'ys-iv-27-semantics',
    title: 'Semantics: tad / chidra / pratyayāntara / saṁskāra',
    summary:
      '“That” refers to the viveka-current; gaps enable non-viveka pratyayas sourced by latencies.',
  },
  {
    id: 'ys-iv-27-dynamics',
    title: 'Dynamics: Intrusion and Continuity',
    summary:
      'Gaps permit intrusions; vector to kaivalya persists if continuity is re-established.',
  },
  {
    id: 'ys-iv-27-diagnostics-remedies',
    title: 'Diagnostics and Remedies',
    summary:
      'Identify gap causes; stabilize viveka; purify saṁskāras; maintain non-transference.',
  },
  {
    id: 'ys-iv-27-crosswalk',
    title: 'Crosswalk (Fichte/Hegel)',
    summary:
      '“From-of-from” chinks (Fichte); actuality with remainders (Hegel).',
  },
  {
    id: 'ys-iv-27-guards',
    title: 'Guards',
    summary:
      'Do not ascribe intrusions to the witness; avoid moralization and denial.',
  },
  {
    id: 'ys-iv-27-bridges',
    title: 'Bridges → IV.28',
    summary:
      'Removal of these intrusions proceeds as for kleśas — previously stated method.',
  },
]

// ---------- HLO Clauses ----------
const HLOS_YS_IV_27 = [
  {
    id: 'ys-iv-27-hlo-text',
    chunkId: 'ys-iv-27-text',
    label: 'Baseline',
    clauses: [
      "tag('sutra','IV.27')",
      'assert(tadChidresu(gap in = discriminativeCurrent))',
      'conclude(pratyayaAntarani arise_from(samskarebhyah))',
    ],
  },
  {
    id: 'ys-iv-27-hlo-semantics',
    chunkId: 'ys-iv-27-semantics',
    label: 'Semantics',
    clauses: [
      'define(tad := the_viveka_current from(IV_26))',
      'define(chidra := momentary_gap/chink in continuity)',
      'define(pratyayaAntarani := non_viveka_cognitions)',
      'define(samskarebhyah := from_latent_impressions)',
      'notSelfLuminousCitta := import(IV_19.notSelfLuminousCitta)',
      'apratisankramaya := import(IV_22.apratisankramaya)',
      'seerWithoutObjecthood := import(IV_21.seerWithoutObjecthood)',
    ],
  },
  {
    id: 'ys-iv-27-hlo-dynamics',
    chunkId: 'ys-iv-27-dynamics',
    label: 'Dynamics',
    clauses: [
      'assert(discriminativeCurrent := import(IV_26.discriminativeCurrent))',
      'assert(kaivalyaVector := import(IV_26.kaivalyaVector))',
      'intrusion := assert(pratyayaAntarani when(gap))',
      'conclude(kaivalyaVector persists_if(sealGaps ∧ restore(discriminativeCurrent)))',
    ],
  },
  {
    id: 'ys-iv-27-hlo-diagnostics-remedies',
    chunkId: 'ys-iv-27-diagnostics-remedies',
    label: 'Diagnostics/Remedies',
    clauses: [
      'gapDiagnostics := note({fatigue, inattentiveness, strong_vasana, triggers})',
      'sealGaps := method(stabilize(discriminativeCurrent) ∧ reduce(samskara_fuel) ∧ strengthen(sattva))',
      'remedialPractice := advise({viveka_khyati_continuity, dhyana_continuity, pratyaya_nirodha_refresh})',
      'assert(parartham ∧ sarvartham)',
      'assert(apratisankramaya ∧ seerWithoutObjecthood)',
    ],
  },
  {
    id: 'ys-iv-27-hlo-crosswalk',
    chunkId: 'ys-iv-27-crosswalk',
    label: 'Crosswalk',
    clauses: [
      'fichteFromOfFromChinks := map(self_intersecting_from permits(chidra unless(method holds)))',
      'hegelActualityWithRemainders := map(actuality_contains(remainders) until freedom_complete)',
    ],
  },
  {
    id: 'ys-iv-27-hlo-guards',
    chunkId: 'ys-iv-27-guards',
    label: 'Guards',
    clauses: [
      'errorSeerFalls := flag(assert(seer falls_or_transfers))',
      'errorMoralizeIntrusions := flag(moralize(intrusion))',
      'errorWorldDenial := flag(deny(appearance) as remedy)',
    ],
  },
  {
    id: 'ys-iv-27-hlo-bridges',
    chunkId: 'ys-iv-27-bridges',
    label: 'Bridges',
    clauses: [
      'prepare(IV_28 := removal_of_intrusions as_klesa_method(previously_stated))',
    ],
  },
]

// ---------- Export Unit ----------
export const YS_IV_27_UNIT: DatasetUnit = {
  id: makeUnitId('ys-iv-27'),
  title:
    'YS IV.27 — tad-chidreṣu pratyayāntarāṇi saṁskārebhyaḥ',
  scope: 'actuality',
  logosMode: 'appearance',
  synthesis:
    'Residual latencies intrude through gaps in the discriminative current; seal gaps, maintain non-transference.',
  faculty: 'buddhi',
  lens: 'yoga',
  chunks: CHUNKS_YS_IV_27 as any,
  hlos: HLOS_YS_IV_27 as any,
}

export const YS_IV_27_SYMBOLS = Object.keys(YS_IV_27_ONTOLOGY);
