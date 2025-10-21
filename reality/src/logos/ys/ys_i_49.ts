import { DatasetUnit, makeUnitId } from '@organon/gdsl/registry/canon'

/*
YS I.49 — śrutānumāna-prajñābhyām anya-viṣayā viśeṣārthatvāt
Conventional: “It (that prajñā) has a different object than [knowledge from] testimony and inference,
because it has a specific (particular) object.”
Yoga frame: distinct from śruti/śruta (what is heard/scriptural testimony) and anumāna (inference).
“Viśeṣa-arthatvāt” = its purport is the particular-as-such (non-general, non-conceptual universal).
*/

const CHUNKS_I_49 = [
  {
    id: 'ys-i-49-sutra',
    title: 'I.49 — śrutānumāna-prajñābhyām anya-viṣayā viśeṣārthatvāt',
    source:
      'Baseline: “There, prajñā is of a different object than testimony and inference, because its purport is the particular (viśeṣa).”',
  },
  {
    id: 'ys-i-49-gloss-terms',
    title: 'Gloss — terms and scope',
    source:
      'śruta/śruti = heard, scriptural/testimonial cognition; anumāna = inferential cognition; anya-viṣayā = different domain/object; viśeṣa-arthatvāt = because it intends the determinate particular-as-such.',
  },
  {
    id: 'ys-i-49-contrast-epistemes',
    title: 'Contrast — epistemes and deliverances',
    source:
      'Testimony and inference deliver mediated, general, and conceptual contents; ṛtambharā prajñā (from I.48) delivers immediate, non-admixt, determinate suchness (particular-in-truth).',
  },
  {
    id: 'ys-i-49-bridge-fichte',
    title: 'Bridge — Pure Reason vs consciousness: mediation vs in-itself validity',
    source:
      'In the Fichtean frame: śruta/anumāna belong to appearance (phenomenology of consciousness); ṛtambharā prajñā belongs to pure reason (principle), its validity is in-itself, not grounded in consciousness.',
  },
  {
    id: 'ys-i-49-principle-reading',
    title: 'Principle reading — Prajñā as Principle (Supreme Being doctrine seed)',
    source:
      'Prajñā = Principle (impersonal), bearing truth (ṛta) as self-certainty of the Absolute. Distinct from tattva-level puruṣa; aligned with Brahman-as-Truth. Seed for concluding realism/idealism adjudication.',
  },

  // Lecture 14 — part 1: maxim and method after rejecting intrinsic validity of consciousness
  {
    id: 'ys-i-49-lecture14-part1-chief-result-abstract',
    title: 'L14/1 — Chief result: deny intrinsic validity of consciousness; abstract when judging truth',
    source:
      '(paraphrase) Consciousness is rejected in intrinsic validity, yet inescapable factically. Therefore, in judging truth we must abstract from it—conditionally, if we want truth.',
  },
  {
    id: 'ys-i-49-lecture14-part1-mediacy-genetic-i',
    title: 'L14/1 — We are mediately entwined (as consciousness) → material for genetic deduction of the I',
    source:
      '(paraphrase) Speaking of consciousness makes us occur empirically as consciousness; this mediacy can serve the genetic deduction of the I.',
  },
  {
    id: 'ys-i-49-lecture14-part1-maxim-of-judging',
    title: 'L14/1 — Maxim of judging (by freedom): conditional principle in-and-for-us',
    source:
      '(paraphrase) Cultivate a maxim by freedom: “If truth is to be valid, abstract from consciousness.” As principle for appearance, may ground a new idealism (of appearance).',
  },
  {
    id: 'ys-i-49-lecture14-part1-higher-maxim-reunites',
    title: 'L14/1 — Higher maxim reunites idealism/realism; differs from prior realism’s unconditionality',
    source:
      '(paraphrase) Confirms: conflicting maxims reunite via a higher maxim. Present maxim is conditional (if truth is to be valid), unlike prior realism’s unconditional validity of truth.',
  },
  {
    id: 'ys-i-49-lecture14-part1-freedom-negative',
    title: 'L14/1 — Freedom as negative: averting illusion, not creating truth',
    source:
      '(paraphrase) Freedom shows itself originally as negative operation—averting illusion rather than affirmatively creating truth.',
  },
  {
    id: 'ys-i-49-lecture14-part1-method-abstract-effects',
    title: 'L14/1 — Method: abstract from all effects of consciousness to deliver truth and the Absolute',
    source:
      '(paraphrase) If consciousness has no validity re: truth, then abstract from all its effects in investigations aimed at delivering truth and the Absolute.',
  },

  // Lecture 14 — part 2: from what to abstract; discontinuous projection; “is” as original appearance
  {
    id: 'ys-i-49-lecture14-part2-what-to-abstract',
    title: 'L14/2 — What to abstract from and its effect: factical projection with a genetic gap',
    source:
      '(paraphrase) Abstract from the salient nerve for which consciousness was rejected: it projects factically (at highest potency) an energy → thinking but cannot give the genetic connection—projecting through an absolute gap.',
  },
  {
    id: 'ys-i-49-lecture14-part2-discontinuous-projection',
    title: 'L14/2 — Discontinuous projection = form of outer existence',
    source:
      '(paraphrase) The same discontinuous projection shows as the form of outer existence present in every categorical “is.” As projection without further account, it is the “death at the root”; the rupture of intellectual activity is death’s lair.',
  },
  {
    id: 'ys-i-49-lecture14-part2-maxim-apply',
    title: 'L14/2 — Maxim: do not admit validity of the “is”-projection; it is mere consciousness’s effect',
    source:
      '(paraphrase) Though inescapable factically, we should not admit validity of this projection; know it signifies nothing, is only the effect of mere consciousness (whose roots stay hidden), and do not be led astray by it.',
  },
  {
    id: 'ys-i-49-lecture14-part2-original-appearance',
    title: 'L14/2 — The “is” as original appearance; kin to the I as original appearance',
    source:
      '(paraphrase) This very “is” is the original appearance—closely related to, perhaps identical with, the I previously presented as original appearance.',
  },

  // Lecture 14 — part 3: test the maxim; re-construct the in-itself; hidden idealism exposed
  {
    id: 'ys-i-49-lecture14-part3-maxim-highest-realism-test',
    title: 'L14/3 — Maxim imposes the highest realism; test it against its own law',
    source:
      '(paraphrase) Against highest idealism, the maxim imposes the highest realism. Before proceeding, test it by its own criterion to see if it is pure realism: it proceeds from the in-itself as absolute—what is this as such?',
  },
  {
    id: 'ys-i-49-lecture14-part3-new-reconstruction-method',
    title: 'L14/3 — New reconstruction: in-itself as presupposed, immediate, independent of “life”',
    source:
      '(paraphrase) Reconstruct the originally completed in-itself as presupposed, unconditionally immediate, determinate, comprehensible, and independent of the living reconstruction (primal fantasy). Deny the real validity of that vivacity, even if factically inescapable.',
  },
  {
    id: 'ys-i-49-lecture14-part3-in-itself-relative',
    title: 'L14/3 — Deeper grasp: the in-itself is relative (oneness of duality), hence not absolute',
    source:
      '(paraphrase) However taken, the in-itself is qualified via negation of an opposed; as such it is relative, a oneness of duality—synthetic/analytic but not true self-sufficient oneness. Even our realism has not reached the absolute.',
  },
  {
    id: 'ys-i-49-lecture14-part3-oneness-as-projection-gap',
    title: 'L14/3 — Projection through a gap: oneness projects in-itself/not-in-itself and vice versa',
    source:
      '(paraphrase) In the background, in-itself and not-in-itself reciprocally posit/explain and negate each other; the oneness is itself a projection of both. This happens immediately, through a gap, without an account: how they follow from pure oneness cannot be explained.',
  },
  {
    id: 'ys-i-49-lecture14-part3-possibility-of-consciousness-root',
    title: 'L14/3 — Determinateness rests only on immediacy/possibility of consciousness',
    source:
      '(paraphrase) The determinateness “oneness of in-itself/not-in-itself” has no warrant beyond immediate awareness: “Think an in-itself”—this possibility of consciousness has shaped the path so far; we relied on consciousness not in actuality but in possibility.',
  },
  {
    id: 'ys-i-49-lecture14-part3-highest-realism-revealed-idealism',
    title: 'L14/3 — Conclusion: our highest realism is hidden idealism; factical, discontinuous; give it up',
    source:
      '(paraphrase) Thus the highest realism is revealed as idealism hidden at its root—fundamentally factical, a discontinuous projection—failing its own criteria and, by its own rules, to be relinquished.',
  },

  // Lecture 14 — part 4: relinquish hidden idealism; absolute being; genesis of appearance
  {
    id: 'ys-i-49-lecture14-part4-why-give-up',
    title: 'L14/4 — Why give it up? Error-source: in-itself as negation/relation',
    source:
      '(paraphrase) The error: “in‑itself” appeared as a negation and relational term. Therefore it (and any system built on it) must be let go if it is to survive.',
  },
  {
    id: 'ys-i-49-lecture14-part4-absolute-being-remains',
    title: 'L14/4 — What remains: absolute being/existence/resting (self-resting adds nothing)',
    source:
      '(paraphrase) Being/existence/resting, taken as absolute, remains. “Resting on itself” is only an illustrative supplement that adds nothing to the inner essence. The regress of “not‑needing” adds nothing; relation to not‑in‑itself is null versus essence.',
  },
  {
    id: 'ys-i-49-lecture14-part4-projection-adds-nothing',
    title: 'L14/4 — Objectifying pure being: projection means nothing; outer form perishes',
    source:
      '(paraphrase) I can notice I project/objectify pure being, but this alters nothing about being. This projection is the discarded supplement (in‑itself) whose nothingness is realized. Thus the outer existential form perishes here; only inner essence remains to be worked through.',
  },
  {
    id: 'ys-i-49-lecture14-part4-genesis-of-appearance',
    title: 'L14/4 — Task: see essence as genesis of appearance; do not be deceived by form',
    source:
      '(paraphrase) We truly work essence through by seeing it as the genesis of its outer appearance. Only by not being deceived by outer form do we arrive at the origin; if deceived, we are dissolved in it. Point added to avoid ending with mere negation; development follows.',
  },
]

const HLOS_I_49 = [
  {
    id: 'ys-i-49-hlo-baseline',
    chunkId: 'ys-i-49-sutra',
    label: 'Baseline separation',
    clauses: [
      "tag('sutra','I.49')","tag('lens','yoga')","tag('link','I.48→I.49')",
      'domain(prajna_rtambhara) ≠ domain({sruta, anumana})',
      'reason := visesaArthatvat // purport: determinate particular',
    ],
  },
  {
    id: 'ys-i-49-hlo-gloss-terms',
    chunkId: 'ys-i-49-gloss-terms',
    label: 'Define terms',
    clauses: [
      "tag('sutra','I.49')","tag('note','technical')",
      'define(sruta := testimonial/scriptural cognition)',
      'define(anumana := inferential cognition)',
      'define(anyaVisaya := different object/domain)',
      'define(visesaArtha := determinate particular-as-such)',
    ],
  },
  {
    id: 'ys-i-49-hlo-contrast-epistemes',
    chunkId: 'ys-i-49-contrast-epistemes',
    label: 'Mediated general vs immediate particular-in-truth',
    clauses: [
      "tag('sutra','I.49')","tag('lens','yoga')",
      'deliver(sruta∧anumana := mediated ∧ general/universal ∧ conceptual)',
      'deliver(prajna_rtambhara := immediate ∧ non-admixt ∧ determinateSuchness)',
    ],
  },
  {
    id: 'ys-i-49-hlo-bridge-fichte',
    chunkId: 'ys-i-49-bridge-fichte',
    label: 'Appearance vs Principle',
    clauses: [
      "tag('sutra','I.49')","tag('lens','fichte')","tag('topic','pure-reason/in-itself')",
      'locate({sruta, anumana} → appearance@consciousness)',
      'locate(prajna_rtambhara → pureReason@principle)',
      'validity(prajna) := inItself ; ¬groundedIn(consciousness)',
    ],
  },
  {
    id: 'ys-i-49-hlo-principle-reading',
    chunkId: 'ys-i-49-principle-reading',
    label: 'Doctrine seed: Prajñā as Principle (Supreme Being)',
    clauses: [
      "tag('sutra','I.49')","tag('lens','fichte')","tag('stance','doctrine')",
      'prajna := Principle(bearing: rta)',
      'distinguish(BrahmanTruth ≠ tattva:purusha)',
      'prepare(adjudication: realism ⊕ idealism ⇒ absoluteOneness)',
    ],
  },

  // Lecture 14 — part 1 HLOs
  {
    id: 'ys-i-49-hlo-lecture14-part1-chief-result-abstract',
    chunkId: 'ys-i-49-lecture14-part1-chief-result-abstract',
    label: 'Chief result: abstract from consciousness when judging truth (conditional maxim)',
    clauses: [
      "tag('sutra','I.49')","tag('lens','fichte')","tag('topic','maxim/method')",
      'rejectValidity(consciousness: intrinsic) ∧ admit(inescapable: factical)',
      'judge(truth) ⇒ must(abstractFrom(consciousness)) when(want(truth))',
    ],
  },
  {
    id: 'ys-i-49-hlo-lecture14-part1-mediacy-genetic-i',
    chunkId: 'ys-i-49-lecture14-part1-mediacy-genetic-i',
    label: 'We-as-consciousness: mediacy usable for genetic deduction of the I',
    clauses: [
      "tag('sutra','I.49')","tag('lens','fichte')","tag('topic','genesis/I')",
      'weOccur(empirically := consciousness) via(speakingOf: consciousness)',
      'use(for: geneticDeduction(I))',
    ],
  },
  {
    id: 'ys-i-49-hlo-lecture14-part1-maxim-of-judging',
    chunkId: 'ys-i-49-lecture14-part1-maxim-of-judging',
    label: 'Maxim (by freedom): “If truth is to be valid, abstract from consciousness”',
    clauses: [
      "tag('sutra','I.49')","tag('lens','fichte')","tag('stance','idealism@appearance')",
      'maxim := acquireBy(freedom)',
      'principle(in∧forUs) := if(wantTruth) then abstractFrom(consciousness)',
      'scope := principleOf(appearance) ⇒ allow(newIdealism@appearance)',
    ],
  },
  {
    id: 'ys-i-49-hlo-lecture14-part1-higher-maxim-reunites',
    chunkId: 'ys-i-49-lecture14-part1-higher-maxim-reunites',
    label: 'Higher maxim reunites idealism/realism; contrasts prior realism',
    clauses: [
      "tag('sutra','I.49')","tag('lens','fichte')","tag('stance','dialectic')",
      'reunite({idealism, realism}) via(higherMaxim)',
      'contrast(previousRealism := truthValid(unconditional), present := validIf(wantTruth))',
    ],
  },
  {
    id: 'ys-i-49-hlo-lecture14-part1-freedom-negative',
    chunkId: 'ys-i-49-lecture14-part1-freedom-negative',
    label: 'Freedom operates negatively: averting illusion, not producing truth',
    clauses: [
      "tag('sutra','I.49')","tag('lens','fichte')","tag('topic','freedom')",
      'freedom.operation := negative',
      'task := avert(illusion) ∧ ¬create(truth)',
    ],
  },
  {
    id: 'ys-i-49-hlo-lecture14-part1-method-abstract-effects',
    chunkId: 'ys-i-49-lecture14-part1-method-abstract-effects',
    label: 'Method: abstract from all effects of consciousness to deliver truth/Absolute',
    clauses: [
      "tag('sutra','I.49')","tag('lens','fichte')","tag('stance','method')",
      'if(noValidity(consciousness, wrt: truth)) then abstractFrom(allEffects(consciousness))',
      'aim := deliver({truth, Absolute})',
    ],
  },

  // Lecture 14 — part 2 HLOs
  {
    id: 'ys-i-49-hlo-lecture14-part2-what-to-abstract',
    chunkId: 'ys-i-49-lecture14-part2-what-to-abstract',
    label: 'Abstract from: factical projection with genetic gap (energy→thinking)',
    clauses: [
      "tag('sutra','I.49')","tag('lens','fichte')","tag('topic','abstraction')",
      'nerve := project(factical: energy→thinking) ∧ ¬give(geneticLink)',
      'character := absoluteGap(disjunction)',
      'maxim: abstractFrom(nerve)',
    ],
  },
  {
    id: 'ys-i-49-hlo-lecture14-part2-discontinuous-projection',
    chunkId: 'ys-i-49-lecture14-part2-discontinuous-projection',
    label: 'Discontinuous projection = outer-existence form; “categorical is” = death at root',
    clauses: [
      "tag('sutra','I.49')","tag('lens','fichte')","tag('topic','appearance/form')",
      'outerExistenceForm := discontinuousProjection',
      'forall x: categoricalIs(x) ⇒ instance(outerExistenceForm)',
      'rupture(intellectualActivity) ⇒ name("deathAtRoot")',
    ],
  },
  {
    id: 'ys-i-49-hlo-lecture14-part2-maxim-apply',
    chunkId: 'ys-i-49-lecture14-part2-maxim-apply',
    label: 'Apply maxim: deny validity of “is”-projection; mark as effect of mere consciousness',
    clauses: [
      "tag('sutra','I.49')","tag('lens','fichte')","tag('stance','method')",
      'inescapable(factical) ∧ nevertheless(denyValidity(outerExistenceForm))',
      'know(signifiesNothing) ∧ classify(effectOf := mereConsciousness)',
      'rule: doNotBeLedAstray(by: projection)',
    ],
  },
  {
    id: 'ys-i-49-hlo-lecture14-part2-original-appearance',
    chunkId: 'ys-i-49-lecture14-part2-original-appearance',
    label: 'Original appearance: “is” ~ I (earlier original appearance)',
    clauses: [
      "tag('sutra','I.49')","tag('lens','fichte')","tag('link','I(originalAppearance)')",
      'identify(originalAppearance := categoricalIs)',
      'relate(originalAppearance:is, originalAppearance:I) ~≈ identical',
    ],
  },

  // Lecture 14 — part 3 HLOs
  {
    id: 'ys-i-49-hlo-lecture14-part3-maxim-highest-realism-test',
    chunkId: 'ys-i-49-lecture14-part3-maxim-highest-realism-test',
    label: 'Test the maxim: proceeds from in-itself as absolute—interrogate its status',
    clauses: [
      "tag('sutra','I.49')","tag('lens','fichte')","tag('topic','criterion')",
      'maxim ⇒ impose(realism:highest)',
      'testBy(itsOwnLaw): premise := proceed(from: inItself:=absolute) ⇒ ask(whatIs(inItself, asSuch))',
    ],
  },
  {
    id: 'ys-i-49-hlo-lecture14-part3-new-reconstruction-method',
    chunkId: 'ys-i-49-lecture14-part3-new-reconstruction-method',
    label: 'Method: reconstruct presupposed in-itself; deny validity of “life” (primal fantasy)',
    clauses: [
      "tag('sutra','I.49')","tag('lens','fichte')","tag('stance','method')",
      'presuppose(inItself := unconditionedImmediate ∧ determinate ∧ comprehensible)',
      'independence(from: livingReconstruction)',
      'denyValidity(primalFantasy/vivacity) // though factically present',
    ],
  },
  {
    id: 'ys-i-49-hlo-lecture14-part3-in-itself-relative',
    chunkId: 'ys-i-49-lecture14-part3-in-itself-relative',
    label: 'Analysis: in-itself = relative oneness-of-duality ⇒ not absolute',
    clauses: [
      "tag('sutra','I.49')","tag('lens','fichte')","tag('stance','diagnosis')",
      'qualify(inItself, by: negate(opposed)) ⇒ relative',
      'structure(inItself) := oneness(duality) // synthetic∧analytic, not self-sufficient oneness',
      'conclude: realism[so-far] ≠ absolute',
    ],
  },
  {
    id: 'ys-i-49-hlo-lecture14-part3-oneness-as-projection-gap',
    chunkId: 'ys-i-49-lecture14-part3-oneness-as-projection-gap',
    label: 'Oneness/projection via gap: cannot deduce in-/not-in-itself from pure oneness',
    clauses: [
      "tag('sutra','I.49')","tag('lens','fichte')","tag('topic','projection')",
      'background: project↔{inItself, notInItself} ; negate↔each(other)',
      'oneness := projection(of: both)',
      'immediateThroughGap ⇒ no(account) for derivation(from: pureOneness)',
    ],
  },
  {
    id: 'ys-i-49-hlo-lecture14-part3-possibility-of-consciousness-root',
    chunkId: 'ys-i-49-lecture14-part3-possibility-of-consciousness-root',
    label: 'Root: determinateness rests on immediate awareness (possibility of consciousness)',
    clauses: [
      "tag('sutra','I.49')","tag('lens','fichte')","tag('topic','awareness/possibility')",
      'determinateness(onenessOf{inItself,notInItself}) ⇒ warrantedOnlyBy(immediateAwareness)',
      'pathSoFar := rely(consciousness: possibility, not actuality)',
    ],
  },
  {
    id: 'ys-i-49-hlo-lecture14-part3-highest-realism-revealed-idealism',
    chunkId: 'ys-i-49-lecture14-part3-highest-realism-revealed-idealism',
    label: 'Verdict: “highest realism” = hidden idealism; factical/discontinuous ⇒ relinquish',
    clauses: [
      "tag('sutra','I.49')","tag('lens','fichte')","tag('stance','adjudication')",
      'reveal(realism_highest → idealism_hidden@root)',
      'classify := {factical, discontinuousProjection}',
      'fails(ownCriteria) ⇒ giveUp(standpoint)',
    ],
  },

  // Lecture 14 — part 4 HLOs
  {
    id: 'ys-i-49-hlo-lecture14-part4-why-give-up',
    chunkId: 'ys-i-49-lecture14-part4-why-give-up',
    label: 'Relinquish: “in-itself” = relational/negative ⇒ drop the standpoint',
    clauses: [
      "tag('sutra','I.49')","tag('lens','fichte')","tag('stance','adjudication')",
      'diagnose(error := inItself as {negation, relation})',
      'therefore: relinquish(standpoint)',
    ],
  },
  {
    id: 'ys-i-49-hlo-lecture14-part4-absolute-being-remains',
    chunkId: 'ys-i-49-lecture14-part4-absolute-being-remains',
    label: 'Remain with absolute being; “self-resting” supplement adds nothing',
    clauses: [
      "tag('sutra','I.49')","tag('lens','fichte')","tag('topic','being/essence')",
      'remain(being := absolute{existence, resting})',
      'supplement(selfResting) := illustrative ∧ addsNothing',
      'regress(notNeeding^n) ⇒ addsNothing',
      'nullify(relation(with: notInItself), vs: essence)',
    ],
  },
  {
    id: 'ys-i-49-hlo-lecture14-part4-projection-adds-nothing',
    chunkId: 'ys-i-49-lecture14-part4-projection-adds-nothing',
    label: 'Projection/objectification of being: no change; outer form perishes',
    clauses: [
      "tag('sutra','I.49')","tag('lens','fichte')","tag('topic','appearance/form')",
      'aware(project(pureBeing)) ⇒ effectOnBeing := none',
      'identify(projection := discardedSupplement(inItself)) ⇒ nothingness(realized)',
      'perish(outerExistentialForm) @ highestElement(inItself)',
      'remain(only := innerEssence)',
    ],
  },
  {
    id: 'ys-i-49-hlo-lecture14-part4-genesis-of-appearance',
    chunkId: 'ys-i-49-lecture14-part4-genesis-of-appearance',
    label: 'Method: see essence as genesis of appearance; avoid deception',
    clauses: [
      "tag('sutra','I.49')","tag('lens','fichte')","tag('stance','method')",
      'workThrough(essence) := see(as: genesis(of: outerAppearance))',
      'rule := doNotBeDeceived(by: outerForm)',
      'if(deceived) ⇒ dissolveIn(form) ∧ neverArrive(origin)',
    ],
  },
]

// Export unit only
export const YS_I_49_UNIT: DatasetUnit = {
  id: makeUnitId('i.49'),
  title: 'YS I.49 — śrutānumāna-prajñābhyām anya-viṣayā (viśeṣārthatvāt)',
  scope: 'essence',
  logosMode: 'prajna',
  synthesis: 'pre-factum',
  faculty: 'buddhi',
  lens: 'yoga',
  chunks: CHUNKS_I_49 as any,
  hlos: HLOS_I_49 as any,
}
