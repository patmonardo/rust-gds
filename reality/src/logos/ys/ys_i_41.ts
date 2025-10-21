import {
  DatasetUnit,
  makeUnitId,
  Chunk,
  Hlo,
} from '@organon/gdsl/registry/canon';

/*
I.41 — Canonical chunks
*/
export const YS_I_41_CHUNKS: Chunk[] = [
  {
    id: 'ys-i41-1-procedure-schema',
    title: 'Procedure schema: immediacy → mediacy (factical → genetic)',
    source:
      'We act under an immediate rule of reason (factual apex), then reveal the guiding law (mediate insight), ascending from factical terms to genetic terms; this ascent can iterate until the absolute source.',
    mode: 'summary',
  },
  {
    id: 'ys-i41-2-x-y-z-dependency',
    title: 'Dependency: x → y → z; loss of z nullifies x and y',
    source:
      'x is only the developmental link to y, and y to z. If z is not comprehended or is forgotten, neither x nor y effectively exist; the whole becomes empty oration. Reason is unitary; recollection restores the sequence.',
    mode: 'summary',
  },
  {
    id: 'ys-i41-3-reconstruction-method',
    title: 'Reconstruction by reason’s unity',
    source:
      'To recover: identify the factical term; locate it relative to earlier ones; test whether the genetic term was presented. Even if the second step was missed, one must rediscover it, since reason that collects itself is self-same.',
    mode: 'summary',
  },
  {
    id: 'ys-i41-4-factical-term-identity',
    title:
      'Factical term: not in A or point alone, but unconditionally in both',
    source:
      'The given term contains A, the point, and a background union; denying either A or the point as the locus of unity posits their union. Insight arose by shifting from content to procedure (origin), making immediacy mediately visible.',
    mode: 'summary',
  },
  {
    id: 'ys-i41-5-manifestness-and-negation',
    title: 'Pure light annuls absolute division; posits intrinsic unity',
    source:
      'Division/disjunction (as absolute) is invalidated by a self-generating manifestness (pure light). The same manifestness posits an intrinsically valid oneness. The principle of division equals the principle of construction (concept). Absolute division is unconditionally negated in the light; “being” without this relation is only absolute self-sufficiency, and even “is” derives from manifestness. The pure light is the sole remaining ground/midpoint.',
    mode: 'summary',
  },
  {
    id: 'ys-i41-6-clarity-and-attention',
    title: 'Intrinsic clarity requires undivided attention',
    source:
      'The foregoing is intrinsically clear and orderly; failure to see it stems from lack of undivided attention.',
    mode: 'summary',
  },
];

/*
I.41 — Hegelian Logical Operations (HLO)
*/
export const YS_I_41_OPS: Hlo[] = [
  {
    id: 'ys-i41-op-1-procedure',
    chunkId: 'ys-i41-1-procedure-schema',
    label: 'Immediacy → Mediacy → Iterated ascent (to absolute source)',
    clauses: [
      'perform(action) guidedBy reason.immediateRule → facticity',
      'reveal(guidingLaw) → mediacy',
      'ascend(factical → genetic)',
      'while not absoluteSource: geneticBecomesFactical; ascend()',
    ],
  },
  {
    id: 'ys-i41-op-2-dependency',
    chunkId: 'ys-i41-2-x-y-z-dependency',
    label: 'Dependency chain and loss condition',
    clauses: [
      'link(x → y) && link(y → z)',
      'if lost(z) then invalid(x) && invalid(y)',
      'reason.isUnitary = true',
      'recollect(sequence) ⇒ restore(z) ⇒ restore(y,x)',
    ],
  },
  {
    id: 'ys-i41-op-3-reconstruct',
    chunkId: 'ys-i41-3-reconstruction-method',
    label: 'Reconstruct by querying factical/genetic placement',
    clauses: [
      'ask(factualTerm?)',
      'place(factualTerm, after(previousTerms))',
      'verify(presented(geneticTerm after factualTerm))',
      'if missing(geneticTerm) then rediscover(geneticTerm) because reason.selfSame',
    ],
  },
  {
    id: 'ys-i41-op-4-factual-identity',
    chunkId: 'ys-i41-4-factical-term-identity',
    label:
      'Unity is not in A nor point, but in their union (procedurally disclosed)',
    clauses: [
      '¬unityIn(A) && ¬unityIn(point)',
      'unity = union(A, point)',
      'focus(procedure/origin) ⇒ immediacy → mediatedVisibility',
    ],
  },
  {
    id: 'ys-i41-op-5-light-negation',
    chunkId: 'ys-i41-5-manifestness-and-negation',
    label: 'Pure light annuls absolute division; posits oneness',
    clauses: [
      'division.asAbsolute ⇒ invalid',
      'insight := selfGenerated(manifestness = pureLight)',
      'pureLight ⇒ posit(unity.intrinsic, noInnerDisjunction)',
      'principle(division) == principle(construction == concept)',
      'negateAbsolute(division) in pureLight',
      'withoutRelation(being) ⇒ absoluteSelfSufficiency',
      'predicate("is") derivesFrom manifestness',
      'pureLight == soleGroundAndMidpoint',
    ],
  },
  {
    id: 'ys-i41-op-6-clarity',
    chunkId: 'ys-i41-6-clarity-and-attention',
    label: 'Clarity requires undivided attention',
    clauses: ['if lack(undividedAttention) then miss(intrinsicClarity)'],
  },
];

/*
I.41 — Developmental analysis (Light, Insight, Emanence/Immanence, Doing/Being)
*/
export const YS_I_41_CHUNKS_B: Chunk[] = [
  {
    id: 'ys-i41-7-insight-into-light-production',
    title: 'Insight into light: agent/representative; how produced?',
    source:
      'Light is sole midpoint; we cannot ask how light is produced (as absolute principle), but we can analyze how insight-into-light is produced: we set the condition and observe the genesis; the mediator is “insight into the insight into light.”',
    mode: 'summary',
  },
  {
    id: 'ys-i41-8-emanence-immanence',
    title: 'Emanence vs. Immanence of producing insight',
    source:
      'Production of insight is viewed as emanative (from light) and as immanent (within insight). The same distinction extends to insight and to light: pure light “enters” insight; objective light as object is not the true light.',
    mode: 'summary',
  },
  {
    id: 'ys-i41-9-light-vs-substance-insight',
    title:
      'Highest object is light; substance is its form; insight disengages from negation',
    source:
      'The highest object is no longer substance but light. Substance is the self-sufficient form of light. Insight (subjectivity), inner expression and life of light, disengages from negation of concept/division, approaching the true midpoint.',
    mode: 'summary',
  },
  {
    id: 'ys-i41-10-doing-being-reversal',
    title:
      'Immediate doing dissolves into immanence; fundamental reversal (doing ↔ being)',
    source:
      'Being is grasped only in immediate doing. Immediate doing dissolves into immanence. Fundamental reversal: doing deposes being; being-beyond-actual deposes doing. Positing without actuality deduces/idealizes doing.',
    mode: 'summary',
  },
  {
    id: 'ys-i41-11-self-negation-and-return',
    title:
      'Self-negation restores the principle; division (being/doing) is null primordially',
    source:
      'Being negates itself in the other through its own doing; prior principle reappears. The “dead in-itself” concept shows a division of being/doing intended to construct non-separation; primordially, this division is nothing.',
    mode: 'summary',
  },
];

export const YS_I_41_OPS_B: Hlo[] = [
  {
    id: 'ys-i41-op-7-produce-insight-into-light',
    chunkId: 'ys-i41-7-insight-into-light-production',
    label: 'Analyze genesis of insight-into-light (agent/representative)',
    clauses: [
      'assert(midpoint(light))',
      'forbid(queryProduction(light))  // absolute principle',
      'allow(queryProduction(insight(light)))',
      'set(condition(selfDisposition))',
      'mediate := insight(insight(light))',
      'observe(genesis(mediate))',
      'aim(greaterClarity)',
    ],
  },
  {
    id: 'ys-i41-op-8-emanence-immanence',
    chunkId: 'ys-i41-8-emanence-immanence',
    label: 'Two modes of production: emanative and immanent',
    clauses: [
      'mode(emanence): from(light) → into(insight)',
      'mode(immanence): within(insight) → disclose(lightAspect)',
      'objective(light-as-object) ≠ trueLight',
      'pureLight ⟶ enters(insight) as aspect',
    ],
  },
  {
    id: 'ys-i41-op-9-light-substance-insight',
    chunkId: 'ys-i41-9-light-vs-substance-insight',
    label:
      'Light is highest object; substance is form; insight disengages from negation',
    clauses: [
      'highestObject := light',
      'substance := formOf(light, selfSufficient)',
      'insight := innerExpression(light)',
      'insight ⟶ disengage(negation(concept ∥ division))',
      'approach(trueMidpoint)',
    ],
  },
  {
    id: 'ys-i41-op-10-doing-being-reversal',
    chunkId: 'ys-i41-10-doing-being-reversal',
    label:
      'Fundamental reversal: doing ↔ being; doing dissolves into immanence',
    clauses: [
      'grasp(being) ↔ onlyIn(immediateDoing)',
      'immediateDoing ⟶ dissolve(intoImmanence)',
      'depose(doing, being) && depose(beingBeyondActual, doing)',
      'positWithoutActuality(being) ⟶ deduce(doing) ⟶ idealize(doing)',
    ],
  },
  {
    id: 'ys-i41-op-11-self-negation-return',
    chunkId: 'ys-i41-11-self-negation-and-return',
    label:
      'Self-negation restores the principle; division is null primordially',
    clauses: [
      'negateSelf(being, inOther) via doing',
      'return(principle(previous))',
      'concept(deadInItself) ⇒ divide(being, doing) to construct(nonSeparation)',
      'primordially: division(being, doing) = null',
    ],
  },
];

// Convenience aggregates (non-breaking): use *_ALL if you want both parts together.
export const YS_I_41_CHUNKS_ALL: Chunk[] = [
  ...YS_I_41_CHUNKS,
  ...YS_I_41_CHUNKS_B,
];
export const YS_I_41_OPS_ALL: Hlo[] = [...YS_I_41_OPS, ...YS_I_41_OPS_B];

// Unit export (for barrel)
export const YS_I_41_UNIT: DatasetUnit = {
  id: makeUnitId('ys:i.41'),
  title: 'YS I.41 — Canon and Development (Light, Insight, Method)',
  scope: 'idea',
  logosMode: 'prajna',
  synthesis: 'pre-factum',
  faculty: 'buddhi',
  lens: 'fichte',
  chunks: YS_I_41_CHUNKS_ALL,
  hlos: YS_I_41_OPS_ALL,
};
