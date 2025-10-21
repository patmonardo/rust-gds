import { DatasetUnit, makeUnitId } from '@organon/gdsl/registry/canon'

/*
YS I.51 — tasyāpi nirodhe sarva-nirodhāt nirbījaḥ samādhiḥ
Baseline: “With the cessation of that too, because of the cessation of all, seedless samādhi [obtains].”
“That” = the truth-born saṃskāra of I.50 (taj-ja saṃskāra) arising from ṛtambharā prajñā (I.48–49).
Fifteenth Lecture (Main Conclusion): pure being as living, self-enclosed oneness (absolute I); all relational marks abandoned.
*/

const CHUNKS_I_51 = [
  {
    id: 'ys-i-51-sutra',
    title: 'I.51 — tasyāpi nirodhe sarva-nirodhāt nirbījaḥ samādhiḥ',
    source:
      'Baseline: “When that too ceases, because all have ceased, seedless samādhi [obtains].” “That” = prajñā-born saṃskāra (I.50).',
  },
  {
    id: 'ys-i-51-gloss-terms',
    title: 'Gloss — terms',
    source:
      'tasyāpi = of that too (the truth-born saṃskāra); nirodhe = upon cessation/quiescence; sarva-nirodhāt = due to cessation of all [vṛttis/saṃskāras]; nirbījaḥ samādhiḥ = seedless samādhi.',
  },
  {
    id: 'ys-i-51-arc',
    title: 'Arc — Prajñā → Saṃskāra → Nirodha',
    source:
      'I.48–49: ṛtambharā prajñā (Principle). I.50: taj-ja saṃskāra (Model) inhibits others. I.51: with total inhibition, even that truth-born saṃskāra ceases → nirbīja-samādhi.',
  },
  {
    id: 'ys-i-51-mechanism',
    title: 'Mechanism — inhibitory completion and auto-cessation',
    source:
      'The prajñā-born Model blocks legacy saṃskāras; once no incompatible traces remain, the Model loses function and subsides (tasyāpi nirodha).',
  },
  {
    id: 'ys-i-51-practice',
    title: 'Practice — stabilization to vanishing',
    source:
      'Sustain the truth Model until all incompatible flows end; then relinquish even the truth-born imprint as purpose-complete, revealing seedless clarity.',
  },
  {
    id: 'ys-i-51-bridge-fichte',
    title: 'Bridge — Main Conclusion: pure being as living oneness; absolute I',
    source:
      '(paraphrase) Abandon every relational mark in the in-itself; what remains: simple, pure being as absolute, self-enclosed oneness arising only in itself, as immediate life—occurring as the absolute I/We.',
  },
  {
    id: 'ys-i-51-mainconclusion-abandon-relations',
    title: 'Main Conclusion — abandon relations; self-enclosed being',
    source:
      '(paraphrase) All that pointed to relations is abandoned; only pure being remains as self-enclosed oneness (no outside, no duality).',
  },
  {
    id: 'ys-i-51-mainconclusion-arises-in-itself-life',
    title: 'Main Conclusion — arises only in itself, as life',
    source:
      '(paraphrase) Being arises only in itself, as immediate living; it occurs only where life occurs (no mediation, no projection-gap).',
  },
  {
    id: 'ys-i-51-mainconclusion-absolute-i',
    title: 'Main Conclusion — absolute I (We) as living being',
    source:
      '(paraphrase) This living oneness occurs as absolute I (self-enclosed We). The simplest insight: being exists immediately only as being/life, and only as whole, undivided oneness.',
  },
  {
    id: 'ys-i-51-doctrine-of-truth',
    title: 'Doctrine of truth — single insight consummated',
    source:
      'Surrender objectivizing/projection (per hiatum); stand in living reason/being. Nirbīja-samādhi aligns with abiding as pure being (no seed, no relation).',
  },
]

const HLOS_I_51 = [
  {
    id: 'ys-i-51-hlo-baseline',
    chunkId: 'ys-i-51-sutra',
    label: 'Baseline',
    clauses: [
      "tag('sutra','I.51')","tag('lens','yoga')","tag('link','I.49→I.50→I.51')",
      'that := samskara_truthBorn(I.50)',
      'if cease(that) ∧ because cease(all{vritti,samskara})) ⇒ samadhi := nirbija',
    ],
  },
  {
    id: 'ys-i-51-hlo-gloss-terms',
    chunkId: 'ys-i-51-gloss-terms',
    label: 'Define terms',
    clauses: [
      "tag('sutra','I.51')","tag('note','technical')",
      'define(tasyaApi := ofThatToo)',
      'define(nirodha := cessation/quiescence)',
      'define(sarvaNirodhat := dueToCessationOfAll)',
      'define(nirbijaSamadhi := seedlessSamadhi)',
    ],
  },
  {
    id: 'ys-i-51-hlo-arc',
    chunkId: 'ys-i-51-arc',
    label: 'Arc: Principle → Model → Cessation',
    clauses: [
      "tag('sutra','I.51')","tag('lens','prajna/samskara/nirodha')",
      'prajna(Principle) ⇒ generate(samskara:Model)',
      'samskara_truthBorn ⇒ inhibit(others)',
      'totalInhibition ⇒ autoCessation(samskara_truthBorn) ⇒ nirbijaSamadhi',
    ],
  },
  {
    id: 'ys-i-51-hlo-mechanism',
    chunkId: 'ys-i-51-mechanism',
    label: 'Inhibitory completion → auto-cessation',
    clauses: [
      "tag('sutra','I.51')","tag('lens','yoga')","tag('topic','dynamics')",
      'while(exists(legacySamskara)) ⇒ maintain(inhibition)',
      'when(¬exists(legacySamskara)) ⇒ loseFunction(samskara_truthBorn) ⇒ cease',
    ],
  },
  {
    id: 'ys-i-51-hlo-practice',
    chunkId: 'ys-i-51-practice',
    label: 'Practice operator',
    clauses: [
      "tag('sutra','I.51')","tag('lens','sadhana')",
      'stabilize(model_truth) ⇒ inhibit(legacy) ⇒ release(model_truth) when(purposeComplete)',
    ],
  },
  {
    id: 'ys-i-51-hlo-bridge-fichte',
    chunkId: 'ys-i-51-bridge-fichte',
    label: 'Fichte bridge: pure being; absolute I; no relations',
    clauses: [
      "tag('sutra','I.51')","tag('lens','fichte')","tag('topic','pure-reason')",
      'abandon(all: relationalMarks)',
      'remain(being := livingOneness ∧ selfEnclosed)',
      'occur(as := absoluteI/We) ∧ forbid(duality ∨ projectionGap)',
    ],
  },
  {
    id: 'ys-i-51-hlo-mainconclusion-abandon-relations',
    chunkId: 'ys-i-51-mainconclusion-abandon-relations',
    label: 'Abandon relations; self-enclosed oneness',
    clauses: [
      "tag('sutra','I.51')","tag('lens','fichte')",
      'drop({inItself/notInItself relations})',
      'assert(oneness := noOutside ∧ noDuality)',
    ],
  },
  {
    id: 'ys-i-51-hlo-mainconclusion-arises-in-itself-life',
    chunkId: 'ys-i-51-mainconclusion-arises-in-itself-life',
    label: 'Arises only in itself, as life',
    clauses: [
      "tag('sutra','I.51')","tag('lens','fichte')",
      'being := arisesOnly(inItself) ∧ mode := immediateLife',
      'ban(mediation/projectionGap)',
    ],
  },
  {
    id: 'ys-i-51-hlo-mainconclusion-absolute-i',
    chunkId: 'ys-i-51-mainconclusion-absolute-i',
    label: 'Absolute I (We) as living being',
    clauses: [
      "tag('sutra','I.51')","tag('lens','fichte')",
      'identify(being ≡ absoluteI/We)',
      'insight := “being exists immediately only as life; only as whole undivided oneness”',
    ],
  },
  {
    id: 'ys-i-51-hlo-doctrine-of-truth',
    chunkId: 'ys-i-51-doctrine-of-truth',
    label: 'Doctrine of truth consummated',
    clauses: [
      "tag('sutra','I.51')","tag('lens','fichte')","tag('stance','completion')",
      'surrender(objectivizing/projection) ⇒ abide(pureBeing)',
      'yoga(nirbijaSamadhi) ≅ SoK(singleInsight: livingOneness)',
    ],
  },
]

// Export unit only
export const YS_I_51_UNIT: DatasetUnit = {
  id: makeUnitId('i.51'),
  title: 'YS I.51 — tasyāpi nirodhe sarva-nirodhāt nirbījaḥ samādhiḥ',
  scope: 'essence',
  logosMode: 'nirodha',
  synthesis: 'pre-factum',
  faculty: 'buddhi',
  lens: 'yoga',
  chunks: CHUNKS_I_51 as any,
  hlos: HLOS_I_51 as any,
}
