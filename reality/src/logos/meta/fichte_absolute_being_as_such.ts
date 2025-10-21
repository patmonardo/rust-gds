import { DatasetUnit, makeUnitId } from '@organon/gdsl/registry/canon'

/*
Fichte — Absolute as “Being-as-such” (Hegelian Protocol)

Aim
- Read Fichte’s “Essence = Being as such” using Hegel’s protocol:
  Being → Essence (truth-of-being) → Actuality (bridge to Absolute Relation).
- Preserve Fichte’s genetics (light = from; persistence = genesis) while
  organizing the exposition via triads and determinate negation.
- Reuse as crosswalk for YS IV.22–IV.24 (Absolute/Modality).

Scope: meta/crosswalk, not exegesis.
*/

export const FICHTE_ABSOLUTE_BAS_ONTOLOGY = {
  beingAsSuch: 'Being-as-such (immediacy of showing; qualitative oneness of light)',
  essenceTruthOfBeing: 'Essence as the truth-of-being (Being grasped as such)',
  actualityVector: 'Vector toward Actuality (conciliation of essence/appearance)',
  determinateNegation: 'Negation-with-preservation as motor of advance',
  triadBeing: 'Protocol: Being triad (immediacy → mediated determinacy → unity-becoming)',
  triadEssence: 'Protocol: Essence triad (essence ↔ appearance ↔ actuality)',
  fichtePersistenceGenesis: 'Persistence = genesis (Fichte): identity of being and genesis',
  lightEqualsFrom: 'Identity: light = “from” (qualitative oneness of appearing)',
  reasonUnderstandingHinge: 'Reason/understanding mutual presupposition (hinge)',
  witnessCarryOver: 'Carry-over from Appearance: seer-only stance survives as method, not object',
  guardNoSumTotalIntuition: 'Deny “sum-total-of-reality” intuition (contra crude readings)',
  guardNoCollapseSpirit: 'Do not collapse witness-only into Absolute Spirit (mediated actuality)',
  // Crosswalk beacons (for graph tooling)
  'HEGEL.BEING_TRIAD': 'Crosswalk beacon: Being triad',
  'HEGEL.ESSENCE_TRIAD': 'Crosswalk beacon: Essence triad',
  'HEGEL.DETERMINATE_NEGATION': 'Crosswalk beacon: determinate negation',
  'YS.ABSOLUTE.IV_22_24': 'Bridge target: YS Absolute/Modality (IV.22–IV.24)',
}

const CHUNKS_FICHTE_ABSOLUTE_BAS = [
  {
    id: 'bas-protocol',
    title: 'Hegelian Protocol',
    summary: 'Triads + determinate negation to read “Essence = Being-as-such.”',
  },
  {
    id: 'bas-being',
    title: 'Being-as-such',
    summary: 'Immediacy of showing; qualitative oneness; persistence = genesis.',
  },
  {
    id: 'bas-essence',
    title: 'Essence as Truth-of-Being',
    summary: 'From Being to Essence (truth-of-being); appearance retained as moment.',
  },
  {
    id: 'bas-actuality',
    title: 'Toward Actuality',
    summary: 'Bridge to Actuality and Absolute Relation (modality prelude).',
  },
  {
    id: 'bas-guards',
    title: 'Guards',
    summary: 'Avoid Spirit/witness collapse and “sum-total” intuition.',
  },
]

const HLOS_FICHTE_ABSOLUTE_BAS = [
  {
    id: 'bas-hlo-protocol',
    chunkId: 'bas-protocol',
    label: 'Protocol',
    clauses: [
      'assert(triadBeing)',
      'assert(triadEssence)',
      'determinateNegation := method(progress_by(negate_with_preserve))',
      'link(HEGEL.BEING_TRIAD)',
      'link(HEGEL.ESSENCE_TRIAD)',
      'link(HEGEL.DETERMINATE_NEGATION)',
    ],
  },
  {
    id: 'bas-hlo-being',
    chunkId: 'bas-being',
    label: 'Being',
    clauses: [
      'beingAsSuch := assert(qualitative_oneness_of_showing)',
      'fichtePersistenceGenesis := assert(being ≡ genesis at qualitative_oneness)',
      'lightEqualsFrom := assert(light ≡ "from")',
    ],
  },
  {
    id: 'bas-hlo-essence',
    chunkId: 'bas-essence',
    label: 'Essence',
    clauses: [
      'essenceTruthOfBeing := assert(essence == truth_of(beingAsSuch))',
      'reasonUnderstandingHinge := recall(mutual_presupposition)',
      'witnessCarryOver := note(method(seeing_as_genesis) persists without mind-on-mind transparency)',
    ],
  },
  {
    id: 'bas-hlo-actuality',
    chunkId: 'bas-actuality',
    label: 'Actuality',
    clauses: [
      'actualityVector := pose(conciliation(essence, appearance) → actuality)',
      'link(YS.ABSOLUTE.IV_22_24)',
    ],
  },
  {
    id: 'bas-hlo-guards',
    chunkId: 'bas-guards',
    label: 'Guards',
    clauses: [
      'guardNoSumTotalIntuition := flag(assert(sum_total_immediate_intuition))',
      'guardNoCollapseSpirit := flag(collapse(witness_only → absolute_spirit))',
    ],
  },
]

export const FICHTE_ABSOLUTE_BAS_UNIT: DatasetUnit = {
  id: makeUnitId('fichte-absolute-being-as-such'),
  title: 'Fichte — Absolute as “Being-as-such” (Hegelian Protocol)',
  scope: 'meta',
  logosMode: 'appearance',
  synthesis: 'essence-as-truth-of-being; persistence≡genesis; bridge-to-actuality',
  faculty: 'buddhi',
  lens: 'fichte',
  chunks: CHUNKS_FICHTE_ABSOLUTE_BAS as any,
  hlos: HLOS_FICHTE_ABSOLUTE_BAS as any,
}

export const FICHTE_ABSOLUTE_BAS_SYMBOLS = Object.keys(FICHTE_ABSOLUTE_BAS_ONTOLOGY)
