import { DatasetUnit, makeUnitId } from '@organon/gdsl/registry/canon'

// Module-private to avoid leaking CHUNKS/HLOS.
const CHUNKS_I_48 = [
  {
    id: 'ys-i-48-rtambhara-tatra-prajna',
    title: 'I.48 — ṛtambharā tatra prajñā',
    source:
      'Conventional: “There (tatra), prajñā is truth-bearing (ṛtambharā).” I.e., in that nirvicāra-lucidity, the insight bears truth (ṛta).',
  },
  {
    id: 'ys-i-48-technical-gloss',
    title: 'Technical gloss (Yoga usage)',
    source:
      'tatra = in the state of nirvicāra-vaiśāradya (cf. I.47); prajñā = special insight distinct from ordinary jñāna/pramāṇa; ṛtambharā = filled-with/bearing ṛta (truth/order), i.e., conforming to reality without admixture.',
  },
  {
    id: 'ys-i-48-bridge-from-i-47',
    title: 'Bridge from I.47 (adhyātma-prasāda → ṛtambharā prajñā)',
    source:
      'From inner-field transparency in nirvicāra (I.47) to prajñā that bears truth (I.48). “Tatra” indexes the same purified locus.',
  },
  {
    id: 'ys-i-48-principle-reading-note',
    title: 'Note (Principle reading, to be developed)',
    source:
      'Prajñā read as Principle (impersonal insight) rather than only “wisdom.” Truth-bearing = validity in itself (in-itself), aligning with the In‑Itself/Principle debate; deeper analysis to follow.',
  },

  // Thirteenth Lecture — prefatory frame for the debate
  {
    id: 'ys-i-48-lecture-preface-free-ascent',
    title: 'Thirteenth Lecture preface: free ascent using distinctions before grounding',
    source:
      '(paraphrase) Proceed “freely”: distinctions must be used before fully founded; rule of ascent underlies the use. If grasped precisely, no confusion.',
  },
  {
    id: 'ys-i-48-perspectives-as-guides',
    title: 'Perspectives as present guides: realism = genesis of life; idealism = genesis of concept',
    source:
      '(paraphrase) Replace earlier L (life) and C (concept) with perspectives: realism (genesis of life) and idealism (genesis of concept), guiding until their oneness-principle is found.',
  },
  {
    id: 'ys-i-48-link-to-life-in-itself',
    title: 'Link back: adhere to life-in-itself required to animate a “through”',
    source:
      '(paraphrase) As before, adhere to life-in-itself as required for the “through”; we do not move far from this and can reproduce/trace what is needed from it.',
  },
  {
    id: 'ys-i-48-hold-fast-avoid-anticipation',
    title: 'Method: hold fast to what is presented; avoid anticipation; preserve genetic links',
    source:
      '(paraphrase) Hold tight to what is presented and separate it from rationally related items; avoid leaping ahead that collapses the genetic linkage into factical flow.',
  },
  {
    id: 'ys-i-48-speculative-reason-warning',
    title: 'Warning: speculative reason’s automatic leaps; make it harder; prefer serene reason',
    source:
      '(paraphrase) Speculative reason can run as vigorously as empirical association; be watchful of leaps as you would of empiricism. Make things “harder” to avoid fantasy; hold to pure, serene reason.',
  },
  {
    id: 'ys-i-48-in-itself-source-of-light-higher-realism',
    title: 'In-itself constructs itself; united with absolute light; light not primordial (higher realism)',
    source:
      '(paraphrase) The in-itself reveals itself as self-constructing by denying thinking; absolute light is immediately united with this; thus the in-itself is source of light, and light is not primordial—mark of higher realism.',
  },
  {
    id: 'ys-i-48-rise-of-higher-idealism-absolute-reflection',
    title: 'Rise of a higher idealism: absolute reflection on the unconditioned in-itself',
    source:
      '(paraphrase) Since we saw the in-itself as negating vision, we must have reflected energetically. Though it constructs itself and the light, this is qualified by our vigorous reflection as highest term—an idealism higher than the previous (which reflected on a conditioned “through”).',
  },
  {
    id: 'ys-i-48-rebuttal-immediate-awareness-claim',
    title: 'Rebuttal: idealism bases itself on immediate awareness “I just see it” (objective intuition)',
    source:
      '(paraphrase) Address to idealism: “You think the in-itself; on what basis? Only: I immediately see it (objective, intuiting).”',
  },
  {
    id: 'ys-i-48-living-genesis-vs-opaque-thinking',
    title: 'Contrast: realism’s living genesis vs idealism’s opaque thinking',
    source:
      '(paraphrase) In realism we saw into a living self-construction that swept insight along—hinting at a unity of outer/inner, factical/genetic. In idealism we do not witness thinking producing; intuition aligns only to an essentially opaque thinking. Ambiguity remains: does thinking yield intuition, vice versa, or both appear from a deeper oneness?',
  },
  {
    id: 'ys-i-48-facticity-no-genetic-middle',
    title: 'Facticity: thinking and consciousness inseparable; no genetic middle term',
    source:
      '(paraphrase) Genuine energetic thinking cannot be without awareness, nor awareness without real thinking. Consciousness’s testimony asserts absolute validity. But the genetic middle between thinking and awareness is missing—idealism remains stuck in facticity.',
  },
  {
    id: 'ys-i-48-light-presupposed-originates-from-in-itself',
    title: 'Light presupposed by consciousness originates from in-itself; inversion blocked',
    source:
      '(paraphrase) From realism: we do not know your “thinking” term, but we recognize its verifier’s principle—consciousness presupposes light; light originates from the in-itself’s absolute self-construction. Hence the in-itself cannot originate from light, contra idealism.',
  },
  {
    id: 'ys-i-48-new-idealism-absolute-intuition-of-reflection',
    title: 'Determination and refutation: absolute intuition of reflection; valid only as appearance',
    source:
      '(paraphrase) The new idealism posits as absolute not reflection itself but its immediate intuition—different in kind from the first. It is refuted as true standpoint but allowed as appearance (not yet derived). Hold to this and set aside deeper points.',
  },
  {
    id: 'ys-i-48-factual-vs-genetic-distinction',
    title: 'Distinction: factical “regarding” vs genetic insight (production witnessed)',
    source:
      '(paraphrase) We cannot witness thinking as producing (only as already being); but we do witness the in-itself existing and self-constructing, simultaneously and reciprocally. This becomes a higher disjunction for a higher oneness.',
  },
  {
    id: 'ys-i-48-reinhold-bardili-critique',
    title: 'Historical note: Reinhold/Bardili — thinking as principle of being (idealism trapped)',
    source:
      '(paraphrase) Places thinking-qua-thinking as principle (closest to the idealism just described); fails to show in-itself negates seeing; avoids appealing to consciousness to dodge idealism; tries to build realism on nothing.',
  },
  {
    id: 'ys-i-48-reinhold-consequence-obscurity',
    title: 'Consequence: no genetic deduction of thinking ⇒ Spinoza-like inference ⇒ obscurity',
    source:
      '(paraphrase) Since thinking cannot be seen producing itself, no genetic deduction follows; only “what exists must lie in it” remains. Pressed to deduce, system turns dark/obscure—clear from the Science of Knowing standpoint.',
  },
  {
    id: 'ys-i-48-idealism-absolute-consciousness',
    title: 'Refuted idealism: immediate consciousness as absolute (absolute intuition/self-consciousness)',
    source:
      '(paraphrase) Makes immediate consciousness the absolute source and protector of truth; absolute consciousness reveals as unity of all consciousness, reflection’s self-consciousness; any “I am conscious” claims intrinsic validity.',
  },
  {
    id: 'ys-i-48-self-consciousness-root',
    title: 'Self-consciousness as root: deduce all modes/disjunctions from it',
    source:
      '(paraphrase) Consciousness realized as self-consciousness/reflection at the root; all disjunctions/modes of consciousness must be deduced from it—yielding a comprehensive study.',
  },
  {
    id: 'ys-i-48-absolute-i-vs-absolute',
    title: 'One consciousness: absolute I (self-identical) — but not The Absolute',
    source:
      '(paraphrase) Thinking of the in-itself is one/self-same; consciousness of it is one; the arising self is the absolute I (purely self-identical), yet not The Absolute.',
  },
  {
    id: 'ys-i-48-need-disjunctive-principle',
    title: 'Need: a disjunctive principle for multifaceted view within “one in-itself” and thinking/I',
    source:
      '(paraphrase) a) While thinking “the one in-itself,” it can appear multi-faceted yet remain one in the background. b) Likewise thinking/reflection and the I can appear manifold yet be one. If we remain trapped in the absolute I, this principle may only be disclosed factically, not genetically.',
  },
  {
    id: 'ys-i-48-historical-misread-absolute-i',
    title: 'Historical note: SoK misread as Absolute-I idealism; higher understanding unwritten',
    source:
      '(paraphrase) Science of Knowing has been taken as the idealism that makes the absolute I the absolute; no writer (friend/foe) has reached a higher conception. A higher understanding would arise only among present listeners; what’s writable stands under the older rule.',
  },
  {
    id: 'ys-i-48-idealism-as-appearance-ground',
    title: 'Refutation and preservation: idealism invalid in-itself, preserved as appearance/ground of appearance',
    source:
      '(paraphrase) Intrinsic validity refuted, yet idealism may remain as appearance and as the foundation of appearance. It is factical: a “fact of consciousness,” which would make the absolute a fact—primary error.',
  },
  {
    id: 'ys-i-48-genesis-vs-fact',
    title: 'SoK principle: not fact but enactment (genesis); beyond idealism from the start',
    source:
      '(paraphrase) SoK declares prior systems erred by positing the absolute in something factical. It grounds itself in an enactment—genesis (production)—and thereby already surpasses that idealism.',
  },
  {
    id: 'ys-i-48-pure-i-only-as-produced',
    title: 'Pure I only as produced, never as found; “found pure I” = psychological illusion',
    source:
      '(paraphrase) The I “as found” is a personal being, not the pure I. SoK recognizes the pure I only as produced; whoever “finds” it as pure is in illusion.',
  },
  {
    id: 'ys-i-48-production-over-product-task',
    title: 'Production stands higher than product; SoK never crowns I; task: produce I and consciousness',
    source:
      '(paraphrase) As science, SoK never places the I at the pinnacle; production is higher than what’s produced. The production of the I—and with it, of the whole of consciousness—is now the task.',
  },
  {
    id: 'ys-i-48-deny-immediate-consciousness-validity',
    title: 'Only pure reason valid; deny immediate consciousness’s testimony (and prove it)',
    source:
      '(paraphrase) The rejected idealism = absolute immediate consciousness. SoK denies the validity of immediate consciousness’s testimony as such—and proves this—leaving only pure reason (grasped by intellect) as valid.',
  },
  {
    id: 'ys-i-48-separate-consciousness-from-truth',
    title: 'Separate truth from consciousness; consciousness = appearance of truth',
    source:
      '(paraphrase) You cannot grasp intellectually without being conscious, but truth’s ground is not in consciousness; it lies entirely in truth itself. Consciousness is only an outer appearance of truth. If something seems true because you are conscious of it, that is illusion and error.',
  },
  {
    id: 'ys-i-48-expunge-facticity-phenomenology',
    title: 'Expunge facticity; phenomenology as second part on basis of the first',
    source:
      '(paraphrase) SoK keeps its promise to expunge facticity. Primordial fact of all factical = consciousness; it verifies nothing where truth is at issue (abstract from it). As second part, a phenomenology deduces appearance/illusion as factical on the ground of the first part.',
  },
  {
    id: 'ys-i-48-entrance-criterion-intellect',
    title: 'Entrance criterion: begin in intellect; external objections fail',
    source:
      '(paraphrase) Nothing external can be brought against SoK. Any beginning against it is either intellectual (immediate = principle of SoK; or mediate = deduction from the fundamental phenomenon) or it appeals to immediate consciousness—then it is rejected at once.',
  },
  {
    id: 'ys-i-48-skepticism-too-late',
    title: 'Skepticism arrives too late; SoK proves invalidity beyond provisional doubt',
    source:
      '(paraphrase) In pure reason, doubt no longer arises; reason holds itself. “Skepticism” that doubts consciousness’s validity only provisionally comes too late: SoK asserts and proves invalidity. Only the possessor of this science could exhibit a total skepticism that dwarfs playful doubts.',
  },
]

const HLOS_I_48 = [
  {
    id: 'ys-i-48-hlo-baseline',
    chunkId: 'ys-i-48-rtambhara-tatra-prajna',
    label: 'Baseline reading',
    clauses: [
      "tag('sutra','I.48')","tag('mode','vicara')","tag('phase','nirvicara')","tag('lens','yoga')",
      'state(tatra := nirvicaraVaisaradya) ⇒ prajna := truthBearing(rtambhara)',
    ],
  },
  {
    id: 'ys-i-48-hlo-gloss',
    chunkId: 'ys-i-48-technical-gloss',
    label: 'Yoga-technical meaning (no analysis yet)',
    clauses: [
      "tag('sutra','I.48')","tag('note','technical')",
      'define(tatra := purifiedNirvicaraContext)',
      'define(prajna := specialInsight ≠ ordinaryJnana/Pramana)',
      'define(rtambhara := bearingRta/truth, withoutAdmixture)',
    ],
  },
  {
    id: 'ys-i-48-hlo-bridge',
    chunkId: 'ys-i-48-bridge-from-i-47',
    label: 'Bridge: from inner transparency to truth-bearing insight',
    clauses: [
      "tag('sutra','I.48')","tag('link','I.47→I.48')",
      'from(adhyatmaPrasada) ⇒ to(prajna: rtambhara)',
    ],
  },
  {
    id: 'ys-i-48-hlo-principle-reading-note',
    chunkId: 'ys-i-48-principle-reading-note',
    label: 'Principle reading (placeholder for Fichtean deep dive)',
    clauses: [
      "tag('sutra','I.48')","tag('lens','fichte')","tag('topic','principle/in-itself')",
      'intend(read(prajna := Principle))',
      'truthBearing ⇒ validity(inItself) // to be analyzed',
    ],
  },

  // Thirteenth Lecture HLOs
  {
    id: 'ys-i-48-hlo-lecture-preface-free-ascent',
    chunkId: 'ys-i-48-lecture-preface-free-ascent',
    label: 'Free ascent: use distinctions before grounding; rule of ascent implicit',
    clauses: [
      "tag('sutra','I.48')","tag('lens','fichte')","tag('stance','method')",
      'proceed(freely) ∧ employ(distinctions) ≺ fullGrounding',
      'assume(ruleOfAscent) underlies(use)',
    ],
  },
  {
    id: 'ys-i-48-hlo-perspectives-as-guides',
    chunkId: 'ys-i-48-perspectives-as-guides',
    label: 'Guides: realism(genesis of life) and idealism(genesis of concept) until unity-principle',
    clauses: [
      "tag('sutra','I.48')","tag('lens','fichte')","tag('stance','dialectic')",
      'setGuides({realism := genesis(life), idealism := genesis(concept)})',
      'until(find(principleOfOneness))',
    ],
  },
  {
    id: 'ys-i-48-hlo-link-to-life-in-itself',
    chunkId: 'ys-i-48-link-to-life-in-itself',
    label: 'Anchor: life-in-itself animates the “through”',
    clauses: [
      "tag('sutra','I.48')","tag('lens','fichte')",
      'adhere(lifeInItself) ⇒ conditionOf(through)',
      'reproduceOrTrace(presentations, from: thisAnchor)',
    ],
  },
  {
    id: 'ys-i-48-hlo-hold-fast-avoid-anticipation',
    chunkId: 'ys-i-48-hold-fast-avoid-anticipation',
    label: 'Discipline: hold fast; separate; avoid anticipation; keep genetic linkage',
    clauses: [
      "tag('sutra','I.48')","tag('lens','fichte')","tag('stance','method')",
      'holdFast(presented) ∧ separate(from: merelyRationallyBound)',
      'avoid(anticipation) ⇒ preserve(linkage(genetic:first→higher))',
    ],
  },
  {
    id: 'ys-i-48-hlo-speculative-reason-warning',
    chunkId: 'ys-i-48-speculative-reason-warning',
    label: 'Warning: speculative leaps; make it harder; prefer serene reason',
    clauses: [
      "tag('sutra','I.48')","tag('lens','fichte')","tag('stance','caution')",
      'speculativeReason := active∧light',
      'watchAgainst(leaps(speculation)) ≈ watchAgainst(stubbornEmpiricism)',
      'makeHarder(subject) ⇒ avoid(fantasy) ∧ favor(pureSereneReason)',
    ],
  },
  {
    id: 'ys-i-48-hlo-in-itself-source-of-light-higher-realism',
    chunkId: 'ys-i-48-in-itself-source-of-light-higher-realism',
    label: 'In-itself → source of light; light not primordial (higher realism)',
    clauses: [
      "tag('sutra','I.48')","tag('lens','fichte')","tag('stance','realism:high')",
      'reveal(inItself := selfConstruction ⊢ deny(thinking))',
      'unite(absoluteLight, with: thisConcept)',
      'infer(source(light) := inItself) ∧ mark(light := non-primordial)',
    ],
  },
  {
    id: 'ys-i-48-hlo-rise-of-higher-idealism-absolute-reflection',
    chunkId: 'ys-i-48-rise-of-higher-idealism-absolute-reflection',
    label: 'Higher idealism from absolute reflection on the unconditioned',
    clauses: [
      "tag('sutra','I.48')","tag('lens','fichte')","tag('stance','idealism:high')",
      'premise: saw(inItself := negatingVision) ⇒ presuppose(energeticReflection)',
      'qualify(selfConstruction ⊕ light, by: ourReflection := highestTerm)',
      'define(idealismHigh := reflect(on: unconditionedInItself))',
    ],
  },
  {
    id: 'ys-i-48-hlo-rebuttal-immediate-awareness-claim',
    chunkId: 'ys-i-48-rebuttal-immediate-awareness-claim',
    label: 'Rebuttal: basis is “I immediately see it” (objective intuition)',
    clauses: [
      "tag('sutra','I.48')","tag('lens','fichte')","tag('stance','critique')",
      'idealism.claim := basis(awarenessImmediate: “I see it”)',
      'classify(this := objectiveIntuition)',
    ],
  },
  {
    id: 'ys-i-48-hlo-living-genesis-vs-opaque-thinking',
    chunkId: 'ys-i-48-living-genesis-vs-opaque-thinking',
    label: 'Contrast: living self-construction vs opaque thinking (ambiguous relation)',
    clauses: [
      "tag('sutra','I.48')","tag('lens','fichte')",
      'realism: see(livingGenesis) ⇒ sweep(insightAlong) ⇒ hint(unify{outer/inner, factical/genetic})',
      'idealism: notWitness(producingThinking) ∧ intuition→opaqueThinking',
      'ambiguous(cause: thinking↔intuition ∨ deeperOneness)',
    ],
  },
  {
    id: 'ys-i-48-hlo-facticity-no-genetic-middle',
    chunkId: 'ys-i-48-facticity-no-genetic-middle',
    label: 'Fact: thinking ≡ awareness; testimony absolute; no genetic middle',
    clauses: [
      "tag('sutra','I.48')","tag('lens','fichte')","tag('stance','diagnosis')",
      'genuineThinking ⇔ awarenessOfThinking',
      'intuition(testimony) ⇒ asserts(absoluteValidity)',
      'missing(geneticMiddle(thinking, awareness)) ⇒ stuck(facticity)',
    ],
  },
  {
    id: 'ys-i-48-hlo-light-presupposed-originates-from-in-itself',
    chunkId: 'ys-i-48-light-presupposed-originates-from-in-itself',
    label: 'Consciousness presupposes light; light from in-itself ⇒ no inversion',
    clauses: [
      "tag('sutra','I.48')","tag('lens','fichte')","tag('stance','realism:argument')",
      'recognize(principle := light) in(idealism’s verifier)',
      'light := originatesFrom(inItself.selfConstruction)',
      'therefore: ¬(inItself originatesFrom light)',
    ],
  },
  {
    id: 'ys-i-48-hlo-new-idealism-absolute-intuition-of-reflection',
    chunkId: 'ys-i-48-new-idealism-absolute-intuition-of-reflection',
    label: 'Determine/refute: absolute intuition of reflection; appearance only',
    clauses: [
      "tag('sutra','I.48')","tag('lens','fichte')","tag('stance','adjudication')",
      'idealismNew := positAbsolute(intuition(of: reflection))',
      'refute(as: trueStandpoint) ∧ allow(as: appearance)',
      'hold(thisConclusion) ∧ drop(deeperPoints)',
    ],
  },
  {
    id: 'ys-i-48-hlo-factual-vs-genetic-distinction',
    chunkId: 'ys-i-48-factual-vs-genetic-distinction',
    label: 'Law: thinking ≠ witnessed as producing; in-itself is witnessed producing (genetic)',
    clauses: [
      "tag('sutra','I.48')","tag('lens','fichte')","tag('topic','method')",
      'see(thinking) := only-as(already-is) // factical regarding',
      'see(inItself) := {exists ∧ selfConstructs} // genetic witnessing',
      'mark(disjunction := for-higher-oneness)',
    ],
  },
  {
    id: 'ys-i-48-hlo-reinhold-bardili-critique',
    chunkId: 'ys-i-48-reinhold-bardili-critique',
    label: 'Critique: thinking-as-principle (idealism) avoids consciousness; realism on nothing',
    clauses: [
      "tag('sutra','I.48')","tag('lens','fichte')","tag('stance','history')",
      'principle := thinkingQuaThinking ⇒ situate(idealismJustDescribed)',
      'miss(negationOfSeeing by: inItself)',
      'avoid(appealTo: consciousness) ⇒ to-dodge(idealism) ⇒ builds(realism,on:nothing)',
    ],
  },
  {
    id: 'ys-i-48-hlo-reinhold-consequence-obscurity',
    chunkId: 'ys-i-48-reinhold-consequence-obscurity',
    label: 'No genetic deduction of thinking ⇒ Spinoza-mode ⇒ obscurity',
    clauses: [
      "tag('sutra','I.48')","tag('lens','fichte')","tag('stance','diagnosis')",
      'cannot(see(thinking, producingItself)) ⇒ cannot(deduceGenetically)',
      'fallback := infer(spinozaStyle: “so-and-so exists ⇒ lies in principle”)',
      'result := obscurity(system)',
    ],
  },
  {
    id: 'ys-i-48-hlo-idealism-absolute-consciousness',
    chunkId: 'ys-i-48-idealism-absolute-consciousness',
    label: 'Idealism (refuted): absolute consciousness = absolute intuition claims intrinsic validity',
    clauses: [
      "tag('sutra','I.48')","tag('lens','fichte')","tag('stance','idealism:critique')",
      'claim: consciousness := absoluteSource ∧ protectorOfTruth',
      'absoluteConsciousness := unity(allConsciousness) := reflectionSelfConsciousness',
      'any(“I am conscious of x”) ⇒ claims(intrinsicValidity)',
    ],
  },
  {
    id: 'ys-i-48-hlo-self-consciousness-root',
    chunkId: 'ys-i-48-self-consciousness-root',
    label: 'Root: deduce all disjunctions/modes from self-consciousness',
    clauses: [
      "tag('sutra','I.48')","tag('lens','fichte')","tag('stance','method')",
      'root := selfConsciousness(reflection)',
      'deduce(all{disjunctions, modesOfConsciousness}, from: root)',
    ],
  },
  {
    id: 'ys-i-48-hlo-absolute-i-vs-absolute',
    chunkId: 'ys-i-48-absolute-i-vs-absolute',
    label: 'Absolute I (self-same) ≠ The Absolute',
    clauses: [
      "tag('sutra','I.48')","tag('lens','fichte')",
      'think(inItself) ⇒ one(selfSame) ⇒ consciousness := one',
      'arises(self := absoluteI) ∧ note(self ≠ TheAbsolute)',
    ],
  },
  {
    id: 'ys-i-48-hlo-need-disjunctive-principle',
    chunkId: 'ys-i-48-need-disjunctive-principle',
    label: 'Need a disjunctive principle for manifold appearance within underlying oneness',
    clauses: [
      "tag('sutra','I.48')","tag('lens','fichte')","tag('goal','higher-oneness')",
      'require(principle(disjunction), for: {inItself, thinking/reflection, I})',
      'appearance(manifold) ∧ background(one)',
      'warn: trapped(absoluteI) ⇒ disclose(principle, factically ¬ genetically)',
    ],
  },
]

export const YS_I_48_UNIT: DatasetUnit = {
  id: makeUnitId('i.48'),
  title: 'YS I.48 — ṛtambharā tatra prajñā (truth-bearing insight)',
  scope: 'essence',
  logosMode: 'prajna',
  synthesis: 'pre-factum',
  faculty: 'buddhi',
  lens: 'yoga',
  chunks: CHUNKS_I_48 as any,
  hlos: HLOS_I_48 as any,
}
