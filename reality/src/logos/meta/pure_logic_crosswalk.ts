import { DatasetUnit, makeUnitId } from '@organon/gdsl/registry/canon'

/*
Hegel — Pure Logic (Crosswalk: Seer of Essential Relation)

Aim
- Seer of Essential Relation as God-like Seer: World-as-such (not empirical totality).
- Kaivalya stance: witness without objecthood; isolation of the seer.
- No “sum-total” intellectual intuition attributed to God (contra Kant).
- Recognition remains mediated (Spirit), distinct from witness-only stance.
- Bridge to YS IV.18–IV.21 (Appearance apex).

Scope: crosswalk/meta, reusable.
*/

// ---------- Ontology ----------
export const HEGEL_ESSENTIAL_SEER_ONTOLOGY = {
  seerOfEssentialRelation: 'God-like Seer at the level of Essential Relation (Welt-als-solche)',
  worldAsSuch: 'World-in-itself (as such) under pure-reason presentation; not empirical aggregate',
  kaivalyaStance: 'Witness isolation (seer without objecthood among objects)',
  noSumTotalIntuition: 'Deny “sum-total-of-reality” immediate intuition (contra Kantian extrapolation)',
  mediatedRecognition: 'Spirit: recognition mediated through world/signs, not mind-on-mind transparency',
  witnessVsSpirit: 'Witness-only ≠ Absolute Spirit (which completes mediation/actuality)',
  pureSattvaAnalogy: 'Śuddha-sattva transparency ~ pure-reason showing (unity of appearing)',
  qualitativeOnenessPriority: 'Priority of qualitative oneness of showing over syllogistic mediation',
  guardCollapseWitnessSpirit: 'Do not collapse Purusha (witness) into Spirit (mediated actuality)',
  guardTotalityMyth: 'Reject “God intuits totality directly”; holds only the world-as-such stance',
}

// ---------- Chunks ----------
const CHUNKS_HEGEL_ESSENTIAL_SEER = [
  { id: 'hegel-essential-seer', title: 'Seer of Essential Relation', summary: 'God-like Seer; world-as-such; kaivalya stance.' },
  { id: 'hegel-vs-kant', title: 'Contrast with Kant', summary: 'No sum-total intellectual intuition; witness of world-as-such.' },
  { id: 'hegel-spirit', title: 'Spirit Mediation', summary: 'Recognition is mediated; distinct from witness-only.' },
  { id: 'hegel-bridges-ys', title: 'Bridges to YS IV.18–IV.21', summary: 'Map witness stance to Purusha and appearance constraints.' },
]

// ---------- HLOs ----------
const HLOS_HEGEL_ESSENTIAL_SEER = [
  {
    id: 'hegel-hlo-essential-seer',
    chunkId: 'hegel-essential-seer',
    label: 'Seer/World-as-such',
    clauses: [
      'seerOfEssentialRelation := assert(god_like_seer at essential_relation)',
      'worldAsSuch := assert(world_in_itself as world_as_such under pure_reason_presentation)',
      'kaivalyaStance := assert(seer_without_objecthood)',
      'qualitativeOnenessPriority := assert(priority(oneness_of_showing))',
      'pureSattvaAnalogy := map(suddha_sattva ↔ pure_reason_showing)',
    ],
  },
  {
    id: 'hegel-hlo-vs-kant',
    chunkId: 'hegel-vs-kant',
    label: 'Kant Contrast',
    clauses: [
      'noSumTotalIntuition := deny(god_has(intellectual_intuition(sum_total_reality)))',
      'guardTotalityMyth := flag(assertion(sum_total_intuition))',
    ],
  },
  {
    id: 'hegel-hlo-spirit',
    chunkId: 'hegel-spirit',
    label: 'Spirit',
    clauses: [
      'mediatedRecognition := assert(recognition via world/signs)',
      'witnessVsSpirit := contrast(purusha:witness_only, spirit:mediated_actuality)',
      'guardCollapseWitnessSpirit := flag(collapse(purusha → spirit))',
    ],
  },
  {
    id: 'hegel-hlo-bridges-ys',
    chunkId: 'hegel-bridges-ys',
    label: 'YS Bridges',
    clauses: [
      'link(IV_18.seerWithoutObjecthood ← kaivalyaStance)',
      'link(IV_19.notSelfLuminousCitta ← qualitativeOnenessPriority)',
      'link(IV_20.noDoubleDetermination ← mediatedRecognition ⇒ avoid(illegitimateSimultaneity))',
      'link(IV_21.worldnessMediation ← mediatedRecognition)',
    ],
  },
]

// ---------- Export Unit ----------
export const HEGEL_ESSENTIAL_SEER_UNIT: DatasetUnit = {
  id: makeUnitId('hegel-essential-seer-crosswalk'),
  title: 'Hegel — Seer of Essential Relation (Crosswalk)',
  scope: 'meta',
  logosMode: 'appearance',
  synthesis: 'world-as-such witness; kaivalya; mediation guard',
  faculty: 'buddhi',
  lens: 'hegel',
  chunks: CHUNKS_HEGEL_ESSENTIAL_SEER as any,
  hlos: HLOS_HEGEL_ESSENTIAL_SEER as any,
}

export const HEGEL_ESSENTIAL_SEER_SYMBOLS = Object.keys(HEGEL_ESSENTIAL_SEER_ONTOLOGY)
