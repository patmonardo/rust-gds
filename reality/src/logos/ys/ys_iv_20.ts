import { DatasetUnit, makeUnitId } from '@organon/gdsl/registry/canon';

/*
YS IV.20 — eka-samaye ca-ubhaya-anavadhāraṇam
“At one time, there is no determination of both.”

Reading
- No simultaneous determination of both (seer/seen; subject/object) by citta.
- Supports IV.19: citta is seen (not self-luminous); prevents reflexive regress.
- Prepares IV.21: Purusha as seer without objecthood (non-agentive witness).
- Fichte L21 crosswalk: organic law forbids collapsing aspects into a single simultaneous act.
*/

// ---------- Ontology ----------
export const YS_IV_20_ONTOLOGY = {
  ekaSamaya: 'One time/instant; within a single cognitive act',
  ubhaya: 'Both (seer and seen; subject and object; knower and known)',
  anavadharanam:
    'No fixation/determination (cannot jointly determine both at once)',
  noDoubleDetermination:
    'A single act cannot simultaneously fix both knower and known',
  cittaNotSeerAndSeen:
    'Mind cannot be simultaneously seer and seen (avoids regress)',
  supportsIV19:
    'Reinforces: citta is not self-luminous because it is seen (IV.19)',
  preparesIV21: 'Prepares: seer-status without objecthood (IV.21)',
  regressGuard: 'Blocks infinite regress from reflexive self-illumination',
  determinationOperator:
    'avadhāraṇa = determination/fixation of content in an act',
  // Crosswalk
  fichteOrganicLaw:
    'Organic law of presentation forbids illegitimate simultaneity of aspects in one act',
  // ---------- Fichte L21: Higher knowing, positive non-self-genesis, hiatum ----------
  higherKnowingPrinciple:
    'In higher knowing, a principle is presupposed for absolute self-genesis',
  higherKnowingNonSelfGenesis:
    'Inwardly/materially higher knowing is non-self-genesis, yet positively existent (immanent I)',
  positiveNegationAsBeing:
    'Positive negation of genesis = enduring being (knowing’s absolute, objective being posited)',
  projectionPerHiatum:
    'A necessary “gap” (hiatum) in continuity of genesis; projection separating pure reason from appearance',
  immanentKnowingNotSelfGenesis:
    'Immanent knowing never appears as self-genesis; only as negation of all genesis',
};

// ---------- Chunks ----------
const CHUNKS_YS_IV_20 = [
  {
    id: 'ys-iv-20-text',
    title: 'IV.20 Text & Baseline',
    summary: 'In one instant, there is no determination of both.',
  },
  {
    id: 'ys-iv-20-semantics',
    title: 'Semantics',
    summary:
      'Define eka-samaya, ubhaya, anavadhāraṇam; determination as act-level fixation.',
  },
  {
    id: 'ys-iv-20-constraint',
    title: 'Constraint',
    summary: 'No simultaneous seer/seen determination by citta; regress guard.',
  },
  {
    id: 'ys-iv-20-bridges',
    title: 'Bridges',
    summary: 'Supports IV.19; prepares IV.21 witness articulation.',
  },
  {
    id: 'ys-iv-20-fichte-higher',
    title: 'Fichte L21 — Higher Knowing',
    summary:
      'Principle for absolute self-genesis; higher knowing as positive non-self-genesis; absolute being of knowing.',
  },
  {
    id: 'ys-iv-20-fichte-hiatum',
    title: 'Fichte L21 — Hiatus and Separation',
    summary:
      'Projection per hiatum separates pure reason’s oneness from appearance; prepares Purusha’s seer-only status.',
  },
];

// ---------- HLO Clauses ----------
const HLOS_YS_IV_20 = [
  {
    id: 'ys-iv-20-hlo-text',
    chunkId: 'ys-iv-20-text',
    label: 'Baseline',
    clauses: [
      "tag('sutra','IV.20')",
      'assert(anavadharanam(ubhaya) at = ekaSamaya)',
    ],
  },
  {
    id: 'ys-iv-20-hlo-semantics',
    chunkId: 'ys-iv-20-semantics',
    label: 'Semantics',
    clauses: [
      'define(ekaSamaya := singleCognitiveInstant)',
      'define(ubhaya := {seer, seen})',
      'define(anavadharanam := not(determineBothSimultaneously))',
      'determinationOperator := gloss(avadharana == fixation_of_content)',
    ],
  },
  {
    id: 'ys-iv-20-hlo-constraint',
    chunkId: 'ys-iv-20-constraint',
    label: 'Constraint',
    clauses: [
      'noDoubleDetermination := rule(¬determine(seer ∧ seen, sameAct))',
      'cittaNotSeerAndSeen := assert(¬(citta == seer ∧ citta == seen) @ ekaSamaya)',
      'regressGuard := prevent(reflexive_regress via noDoubleDetermination)',
      'fichteOrganicLaw := note(forbid(illegitimateSimultaneity(aspects)))',
    ],
  },
  {
    id: 'ys-iv-20-hlo-bridges',
    chunkId: 'ys-iv-20-bridges',
    label: 'Bridges',
    clauses: [
      'supportsIV19 := link(IV_19.notSelfLuminousCitta ← noDoubleDetermination)',
      'preparesIV21 := prepare(IV_21.seerWithoutObjecthood)',
    ],
  },
  {
    id: 'ys-iv-20-hlo-fichte-higher',
    chunkId: 'ys-iv-20-fichte-higher',
    label: 'Higher Knowing',
    clauses: [
      'higherKnowingPrinciple := assert(presuppose(principle(self_genesis), in = higher_knowing))',
      'higherKnowingNonSelfGenesis := assert(non_self_genesis(higher_knowing) ∧ immanent(I))',
      'positiveNegationAsBeing := assert(posNeg(genesis) == enduring(being_of_knowing))',
      'link(preparesIV21, to = positiveNegationAsBeing)',
    ],
  },
  {
    id: 'ys-iv-20-hlo-fichte-hiatum',
    chunkId: 'ys-iv-20-fichte-hiatum',
    label: 'Hiatum',
    clauses: [
      'immanentKnowingNotSelfGenesis := assert(immanent_knowing ≠ appear_as(self_genesis))',
      'projectionPerHiatum := assert(necessary_gap in(genesis_continuity) ⇒ project(separate(pure_reason, appearance)))',
      'preparesIV21 := reinforce(IV_21.seerWithoutObjecthood)',
    ],
  },
];

// ---------- Export Unit ----------
export const YS_IV_20_UNIT: DatasetUnit = {
  id: makeUnitId('ys-iv-20'),
  title: 'YS IV.20 — eka-samaye ca-ubhaya-anavadhāraṇam',
  scope: 'appearance',
  logosMode: 'appearance',
  synthesis: 'no-simultaneous-determination (seer/seen)',
  faculty: 'buddhi',
  lens: 'yoga',
  chunks: CHUNKS_YS_IV_20 as any,
  hlos: HLOS_YS_IV_20 as any,
};

export const YS_IV_20_SYMBOLS = Object.keys(YS_IV_20_ONTOLOGY);
