import { DatasetUnit, makeUnitId } from '@organon/gdsl/registry/canon'

/*
Fichte – Sixth Lecture (April 26, 1804) — Excerpt Seed
Focus: mediation “between” oneness and multiplicity; organic unity; necessity of fine distinctions (“hairsplitting”);
schema rule of division; true oneness as living, self-active principle simultaneously grounding unity & disjunction.
*/

const CHUNKS_FICHTE_L6 = [
  {
    id: 'fichte-l6-context',
    title: 'Context & Aim',
    summary: 'Science of Knowing stands between pure oneness and empirical multiplicity; task: trace multiplicity back to absolute oneness and deduce multiplicity from it without collapsing either pole.',
  },
  {
    id: 'fichte-l6-method-middle',
    title: 'Methodological Middle',
    summary: 'Stance: neither immersed in sheer multiplicity nor absolutizing an abstract unity; maintains an elevated origin-perspective mediating both.',
  },
  {
    id: 'fichte-l6-new-distinctions',
    title: 'Novel Distinctions',
    summary: 'Required distinctions are subtle and previously unnoticed; ordinary representation fuses them—appearing “minute” (accused as hairsplitting) yet essential for rigor.',
  },
  {
    id: 'fichte-l6-purpose-distinctions',
    title: 'Purpose of Precision',
    summary: 'If any available distinction (latent in the field) remains undrawn, the science fails its purpose; completeness of differentiations is methodological necessity.',
  },
  {
    id: 'fichte-l6-schema-intent',
    title: 'Advance Schema',
    summary: 'Provide a general formal schema/rule for forthcoming divisions so learners can anchor and retain emerging differentiations.',
  },
  {
    id: 'fichte-l6-true-oneness',
    title: 'True Oneness Defined',
    summary: 'True oneness is not a simple object; it is the living, inward, organic principle that is simultaneously the principle of unity and disjunction.',
  },
  {
    id: 'fichte-l6-organic-principle',
    title: 'Organic Simultaneity',
    summary: 'Oneness cannot project unity and multiplicity externally; it internally is both—cannot be unity without at once generating disjunction (and vice versa).',
  },
  {
    id: 'fichte-l6-warning',
    title: 'Warning Against Halfway Unity',
    summary: 'Any philosopher stopping at a merely relative/simple unity mistakes an apparent for the absolute; learners must detect and name this shortfall.',
  },
  {
    id: 'fichte-l6-living-activity',
    title: 'Living Activity',
    summary: 'Oneness = active, powerful, irrepressible essence we live enacting—not a contemplated static object; praxis-ground not spectator-object.',
  },
]

/*
Mini-ontology / clause conventions used here:
- tag(key,value)
- stance(subject := relation)
- require(condition)
- principle(X := Y)
- synthesize(A,B) ⇒ C
- fail(condition) ⇒ consequence
- simultaneous(a,b)
- warn(condition)
- distinguish(set{...})
- organic(unity) marks internally generated dual aspect
*/

const HLOS_FICHTE_L6 = [
  {
    id: 'fichte-l6-hlo-middle',
    chunkId: 'fichte-l6-method-middle',
    label: 'Mediated Middle Stance',
    clauses: [
      "tag('source','Fichte-1804-L6')",
      "stance(scienceOfKnowing := between(oneness,multiplicity))",
      "aim(trace(multiplicity → oneness) ∧ deduce(multiplicity ← oneness))",
    ],
  },
  {
    id: 'fichte-l6-hlo-distinctions-need',
    chunkId: 'fichte-l6-new-distinctions',
    label: 'Need for Novel Distinctions',
    clauses: [
      'latentDistinctions ⇒ appear(minute)',
      'ordinaryRepresentation ⇒ collapse(subtleDistinctions → apparentOneness)',
      'require(explicitArticulation(distinctions))',
    ],
  },
    {
    id: 'fichte-l6-hlo-completeness',
    chunkId: 'fichte-l6-purpose-distinctions',
    label: 'Completeness Criterion',
    clauses: [
      'if exists(undrawnDistinction) then fail(methodPurpose)',
      'methodPurpose := complete(extraction(allAvailableDistinctions))',
    ],
  },
  {
    id: 'fichte-l6-hlo-schema',
    chunkId: 'fichte-l6-schema-intent',
    label: 'Schema Provision',
    clauses: [
      "provide(schemaRule) ⇒ reduce(cognitiveLoad(futureDivisions))",
      'schemaRule := formalGuide(structure(distinctionEmergence))',
    ],
  },
  {
    id: 'fichte-l6-hlo-true-oneness',
    chunkId: 'fichte-l6-true-oneness',
    label: 'True Oneness Differential',
    clauses: [
      "reject(simpleObject(oneness))",
      "principle(trueOneness := livingOrganicSource(unity,disjunction))",
      'simultaneous(principleOfUnity, principleOfDisjunction)',
    ],
  },
  {
    id: 'fichte-l6-hlo-organic',
    chunkId: 'fichte-l6-organic-principle',
    label: 'Organic Simultaneity',
    clauses: [
      'organic(trueOneness)',
      'not(externalProjection(unity,disjunction))',
      'internalGeneration(unity ∧ disjunction)',
    ],
  },
  {
    id: 'fichte-l6-hlo-warning',
    chunkId: 'fichte-l6-warning',
    label: 'Halfway Unity Warning',
    clauses: [
      'warn(stoppingAt(relativeUnity))',
      'stoppingAt(relativeUnity) ⇒ error(misidentify(absolute))',
    ],
  },
  {
    id: 'fichte-l6-hlo-activity',
    chunkId: 'fichte-l6-living-activity',
    label: 'Lived Activity',
    clauses: [
      'trueOneness ≠ objectOfSpectator',
      'trueOneness = enacted(livingEssence)',
      'practice(live(oneness)) ⇒ comprehend(absolute)',
    ],
  },
]

/* ------------------------------------------------------------------
   Mini‑Ontology (Section 5 extension: Light / Life / Concept / Appearance)
   (Reworded to avoid strong / disruptive verbs; neutral analytic vocabulary.)
------------------------------------------------------------------- */
export const FICHTE_MINI_ONTOLOGY_SEC5 = {
  light: 'Pure absolute insight (pre-objective, self-luminous)',
  life: 'Enacted vitality of light (its living immediacy in act)',
  concept: 'Objectifying distinction; both a limiting and positing operation',
  objectify: 'Render a living immediacy into a fixed conceptual form',
  attenuation: 'Diminution of immediacy through reflective separation',
  positedness: 'Sheer “set-there” status without further mediation',
  selfSubsistentBeing: 'That which stands as grounded by light’s mere positedness',
  negate: 'Suspend or withdraw intrinsic validity of a posit',
  vitalAct: 'Enactive performance sustaining the presence of light',
  essentialContradiction: 'Method-required self-opposition that mediates a higher unity',
  appearance: 'Phenomenal showing (single layer)',
  appearanceLayer: 'Meta-level of appearance (appearance^n)',
  conditioningAppearance: 'An appearance that animates or conditions another',
  primordialAppearance: 'Root living showing (b) prior to derivative layering',
  layeredSchema: 'Regulated descent: light → concept → appearance^n',
  descentRule: 'Rule-governed progression generating nested appearances',
  reIntegration: 'Enactive collapse of analytic layers back into living unity',
  vitalityChain: 'Propagation of lived immediacy: light → concept (as living form) → appearance',
  contradictionMarker: 'Indicator that a constructed opposition is methodologically required',
}

/* ------------------ Chunks: Section 5 (Light–Life–Concept – Layered Appearance) ------------------ */
CHUNKS_FICHTE_L6.push(
  {
    id: 'fichte-l6-sec5-surrender-vs-consider',
    title: 'Surrender vs Consideration',
    summary: 'Pure surrender: immersion in living light. Consideration: reflective objectification that attenuates immediacy by separating “life” from “light.”',
  },
  {
    id: 'fichte-l6-sec5-light-grounds-being',
    title: 'Light Grounds Self-Subsistent Being',
    summary: 'Light, by mere positedness, sets a self-subsistent being and simultaneously the concept (as both limited/negated and affirmed).',
  },
  {
    id: 'fichte-l6-sec5-life-added-split',
    title: 'Adding Life Introduces Split',
    summary: 'Introducing “life” as an added predicate analytically separates what in enactment is one stroke—producing reflective attenuation.',
  },
  {
    id: 'fichte-l6-sec5-essential-contradiction',
    title: 'Essential Contradiction',
    summary: 'We distinguish life from light and immediately deny their separability; the structured self-opposition mediates higher insight.',
  },
  {
    id: 'fichte-l6-sec5-layered-appearance',
    title: 'Layered Appearance Cascade',
    summary: 'Concept posits A and point only as appearance; derivative appearances condition a primordial appearance (b): an appearance-of-appearance chain.',
  },
  {
    id: 'fichte-l6-sec5-reintegration',
    title: 'Re‑Integration by Enactment',
    summary: 'Enactive performance collapses the analytic descent; layers reunify in lived immediacy (re-integration).',
  },
)

/* --------------- HLO Clauses: Section 5 --------------- */
HLOS_FICHTE_L6.push(
  {
    id: 'fichte-l6-sec5-hlo-surrender-consider',
    chunkId: 'fichte-l6-sec5-surrender-vs-consider',
    label: 'Mode Shift',
    clauses: [
      "tag('source','Fichte-1804-L6')",
      'mode(surrender := immersion(light, life))',
      'mode(consideration := objectify(light) ⇒ attenuation(immediacy))',
    ],
  },
  {
    id: 'fichte-l6-sec5-hlo-light-grounds',
    chunkId: 'fichte-l6-sec5-light-grounds-being',
    label: 'Light Grounds Being & Concept',
    clauses: [
      'light ⇒ posited(selfSubsistentBeing)',
      'light ⇒ coPosit(concept{negatedAspect, affirmedAspect})',
    ],
  },
  {
    id: 'fichte-l6-sec5-hlo-life-split',
    chunkId: 'fichte-l6-sec5-life-added-split',
    label: 'Analytic Split',
    clauses: [
      'addPredicate(life, light) ⇒ introduce(distinction(life, light))',
      'introduce(distinction) ⇒ attenuation(livingImmediacy)',
    ],
  },
  {
    id: 'fichte-l6-sec5-hlo-essential-contradiction',
    chunkId: 'fichte-l6-sec5-essential-contradiction',
    label: 'Essential Contradiction',
    clauses: [
      'assert(distinct(life, light)) ∧ deny(distinct(life, light)) ⇒ essentialContradiction',
      'essentialContradiction ⇒ mediation(higherUnity)',
    ],
  },
  {
    id: 'fichte-l6-sec5-hlo-layered-appearance',
    chunkId: 'fichte-l6-sec5-layered-appearance',
    label: 'Appearance Layers',
    clauses: [
      'concept ⇒ posit(A, point)',
      'A, point ⇒ appearance(layer1)',
      'appearance(layer1) ⇒ condition(primordialAppearance(b))',
      'chain(appearance^n) ⇒ layeredSchema',
    ],
  },
  {
    id: 'fichte-l6-sec5-hlo-reintegration',
    chunkId: 'fichte-l6-sec5-reintegration',
    label: 'Re‑Integration',
    clauses: [
      'analyticDescent(light → concept → appearance^n) ⇒ potentialFragmentation',
      'reIntegration := enact(unity(light, life, concept, appearance))',
      'reIntegration ⇒ dissolve(layering)',
    ],
  },
)

export const YS_I_42_UNIT: DatasetUnit = {
  id: makeUnitId('fichte-1804-l6'),
  title: 'Fichte 1804 — Sixth Lecture (Oneness & Distinction Method)',
  scope: 'essence',
  logosMode: 'dialectic',
  synthesis: 'schema-seed',
  faculty: 'buddhi',
  lens: 'fichte',
  chunks: CHUNKS_FICHTE_L6 as any,
  hlos: HLOS_FICHTE_L6 as any,
}
