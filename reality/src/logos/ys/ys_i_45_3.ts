import { DatasetUnit, makeUnitId } from '@organon/gdsl/registry/canon'

export const CHUNKS_I_45_3 = [
  {
    id: 'ys-i-45-3-alinga-kernel',
    title: 'I.45 — A-liṅga: unmarked ground (culmination of Vicāra)',
    source:
      '(paraphrase) Vicāra on subtle terminates in a-liṅga: the unmarked ground; cessation of marks.',
  },
  {
    id: 'ys-i-45-3-boundary-of-reflection',
    title: 'Boundary of reflection: law remains; marks cease',
    source:
      '(paraphrase) Reflective mediacy reaches its limit; law is retained as meaning-only; beyond lies living light.',
  },
  {
    id: 'ys-i-45-3-through-presupposes-life',
    title: 'Existence of a “through” presupposes original life (not grounded in the through)',
    source:
      '(paraphrase) A “through” shows only formal duality; to complete it needs transition (living oneness). Life as life is not in the through; it comes entirely from itself. Hence any existing “through” presupposes an original life.',
  },
  {
    id: 'ys-i-45-3-explanation-is-through',
    title: 'Explaining the “through” is itself a “through” (horizontal/vertical)',
    source:
      '(paraphrase) Positing the through’s existence brings life in image/concept; explanation is a through. Horizontal: a — a×b (the explaining-through posits the first). Vertical: life = antecedent; through-existence = consequent; both only in connection.',
  },
  {
    id: 'ys-i-45-3-concept-hypothetical-should',
    title: 'Concept’s hypothetical “should”: independence; concept as antecedent/prius',
    source:
      '(paraphrase) Concept constructs a living-through hypothetically (should). The “should” expresses the concept’s independence; contents independent too. Existence of a through is absolute a priori as the concept’s expression; concept is antecedent and absolute prius.',
  },
  {
    id: 'ys-i-45-3-synthesis-chain',
    title: 'Synthesis: One science via extension of “through-one-another” — chain of principles',
    source:
      '(paraphrase) Ignorance repels from knowledge, but the disjunction extends science. Science remains one through an extension of throughs—a chain of principles (Universal Encyclopedia). Objective Logic (vitarka–vicāra) closes here.',
  },
  {
    id: 'ys-i-45-3-transition-triadic-plane',
    title: 'Transition preview: triadic plane — Prajñā, Saṃskāra, Nirodha',
    source:
      '(paraphrase) From a-liṅga synthesis move to the threefold triadic plane of prajñā itself and its dynamics (saṃskāra, nirodha).',
  },
  {
    id: 'ys-i-45-3-realism-maxim-inner-content',
    title: 'Realism maxim: ignore factical self-givenness; validate inner content only',
    source:
      '(paraphrase) Do not reflect on the external, factical self-givenness of thinking/insight; count only inner form/content as valid. Posit absolute truth as content; it alone matters and negates what it does not contain. Made genetic, so it is.',
  },
  {
    id: 'ys-i-45-3-realism-truth-as-image-of-life',
    title: 'Realism’s presupposition: implicit truth = living immutable image of life',
    source:
      '(paraphrase) Implicit absolute truth appears as a living, determined, immutable image that sustains itself; it reveals only in absolute life. Truth is the image of life; only an image of life gives truth. We stand between life and its image—identical in content, different in form (which realism brackets). This nears idealism when compelled to clarify its ground.',
  },
  {
    id: 'ys-i-45-3-law-of-maxims-need',
    title: 'Conflict in maxims ⇒ seek a law of maxims',
    source:
      '(paraphrase) Both idealism and realism rest on assumptions grounded in an inner maxim of the subject (empirical root). Their conflict can be alleviated only by setting out a law of maxims; we must search for it.',
  },
  {
    id: 'ys-i-45-3-preference-realism',
    title: 'Preference for realism (with critique of both)',
    source:
      '(paraphrase) Idealism is one-sided (it makes its opposite impossible). Realism leaves the opposite’s being undisputed but makes it inconceivable—insufficient for a genetic science.',
  },
  {
    id: 'ys-i-45-3-interpolation-appearance',
    title: 'Interpolation: intuition may arise only as appearance (not self-grounded)',
    source:
      '(paraphrase) What is proven by realism is only that self-supporting intuition of absolute life cannot arise. It may and must arise under conditions merely as a phenomenon not grounded in itself.',
  },
  {
    id: 'ys-i-45-3-standpoint-unification',
    title: 'Standpoint: unify idealism and realism via appearance-as-appearance',
    source:
      '(paraphrase) Root self-intuition is the first appearance and ground of all further appearances. This is genuine truth-as-appearance; modifications must be intuited as necessary. Error begins when appearance is taken for being.',
  },
  {
    id: 'ys-i-45-3-derive-error-from-absence',
    title: 'Derivability of error from necessary absence',
    source:
      '(paraphrase) Seeming and error arise necessarily from the absence of truth; given this absence as necessary, their basis and form are derivable.',
  },
  {
    id: 'ys-i-45-3-measure-of-ignorance',
    title: 'Measure of ignorance ⇒ catalogue of errors',
    source:
      '(paraphrase) Rule: tell me exactly what you do not know/understand, and I will list with precision all your errors and illusions—and it will be correct.',
  },
]

export const HLOS_I_45_3 = [
  {
    id: 'ys-i-45-3-hlo-kernel',
    chunkId: 'ys-i-45-3-alinga-kernel',
    label: 'Kernel: culmination in a-liṅga (unmarked)',
    clauses: [
      "tag('method','samapatti')","tag('mode','vicara')","tag('phase','nirvicara')",
      "tag('object','subtle/alinga')","tag('faculty','buddhi')",
      "tag('lens','fichte')","tag('scope','essence')","tag('order','4.21')",

      'terminate(derivation(subtle)) ⇒ ground := aLinga(unmarked)',
      'cease(mark) ∧ retain(law, as: meaningOnly)',
      'invariant(throughOneAnother)',
    ],
  },
  {
    id: 'ys-i-45-3-hlo-beyond-to-light',
    chunkId: 'ys-i-45-3-boundary-of-reflection',
    label: 'Beyond the limit: insight points to pure living light',
    clauses: [
      "tag('method','samapatti')","tag('mode','vicara')","tag('phase','nirvicara')",
      "tag('faculty','buddhi')","tag('lens','fichte')","tag('scope','essence')","tag('order','4.22')",

      'conceptFinds(limit) ⇒ reflectionStops',
      'pointer := to(livingLight) ¬to(nullAppearances)',
    ],
  },
  {
    id: 'ys-i-45-3-hlo-through-presupposes-life',
    chunkId: 'ys-i-45-3-through-presupposes-life',
    label: 'Law: any existing “through” presupposes original life',
    clauses: [
      "tag('method','samapatti')","tag('mode','vicara')","tag('phase','nirvicara')",
      "tag('faculty','buddhi')","tag('lens','fichte')","tag('scope','essence')","tag('order','4.36')",

      'through := dual(formal) ∧ needs(transition := livingOneness)',
      'life := fromItself ¬from(through)',
      'exists(through) ⇒ presuppose(originalLife)',
    ],
  },
  {
    id: 'ys-i-45-3-hlo-explanation-is-through',
    chunkId: 'ys-i-45-3-explanation-is-through',
    label: 'Meta: explanation of through is a through (horizontal/vertical)',
    clauses: [
      "tag('method','samapatti')","tag('mode','vicara')","tag('phase','savicara')",
      "tag('faculty','manas')","tag('lens','fichte')","tag('scope','essence')","tag('order','4.37')",

      'horizontal: explain(a) ⇒ posits(a×b)',
      'vertical: antecedent := life ; consequent := existence(through)',
      'onlyInConnection(antecedent, consequent)',
    ],
  },
  {
    id: 'ys-i-45-3-hlo-concept-should-prius',
    chunkId: 'ys-i-45-3-concept-hypothetical-should',
    label: 'Concept’s “should” = independence; concept is antecedent/prius of the through',
    clauses: [
      "tag('method','samapatti')","tag('mode','vicara')","tag('phase','nirvicara')",
      "tag('faculty','buddhi')","tag('lens','fichte')","tag('scope','essence')","tag('order','4.38')",

      'concept(hypothesis:“should”) ⇒ declares(independence)',
      'existence(through) := expressionOf(concept) (a priori, absolute)',
      'concept := antecedent ∧ absolutePrius',
    ],
  },
  {
    id: 'ys-i-45-3-hlo-synthesis-chain',
    chunkId: 'ys-i-45-3-synthesis-chain',
    label: 'Synthesis: one science via extended throughs (chain of principles)',
    clauses: [
      "tag('method','samapatti')","tag('mode','vicara')","tag('phase','nirvicara')",
      "tag('faculty','buddhi')","tag('lens','fichte')","tag('scope','essence')","tag('order','4.39')",

      'ignorance(repelsFrom: knowledge) ⇒ yields(extensionOf(science))',
      'maintain(unity(science)) via extend(throughOneAnother, as: chain(principles))',
      'close(ObjectiveLogic := vitarka⊕vicara)',
    ],
  },
  {
    id: 'ys-i-45-3-hlo-transition-triadic',
    chunkId: 'ys-i-45-3-transition-triadic-plane',
    label: 'Transition: to triadic plane (prajñā, saṃskāra, nirodha)',
    clauses: [
      "tag('method','samapatti')","tag('mode','vicara')","tag('phase','savicara')",
      "tag('faculty','manas')","tag('lens','fichte')","tag('scope','essence')","tag('order','4.40')",

      'from(alinga-synthesis) ⇒ enter(triadicPlane{prajna, samskara, nirodha})',
      'preview(nextBook: subjective/practical logic)',
    ],
  },
  {
    id: 'ys-i-45-3-hlo-realism-maxim-inner-content',
    chunkId: 'ys-i-45-3-realism-maxim-inner-content',
    label: 'Maxim (realism): validate inner content, not factical existence of thought',
    clauses: [
      "tag('method','samapatti')","tag('mode','vicara')","tag('stance','realism')",
      "tag('phase','savicara')","tag('faculty','manas')","tag('lens','fichte')",
      "tag('scope','essence')","tag('order','4.41')",

      'ignore(facticalSelfGivenness(thinking/insight))',
      'validate(only(innerContent))',
      'posit(absoluteTruth := content(thinking))',
      'content(negates: what-it-does-not-contain)',
    ],
  },
  {
    id: 'ys-i-45-3-hlo-realism-truth-as-image-of-life',
    chunkId: 'ys-i-45-3-realism-truth-as-image-of-life',
    label: 'Truth = image of life; reveals only in life; content-identity, form-difference',
    clauses: [
      "tag('method','samapatti')","tag('mode','vicara')","tag('stance','realism')",
      "tag('phase','nirvicara')","tag('faculty','buddhi')","tag('lens','fichte')",
      "tag('scope','essence')","tag('order','4.42')",

      'truth := image(of: life){living, determined, immutable, self-sustaining}',
      'revealsOnlyIn(absoluteLife)',
      'identifyContent(life ≡ imageOfLife)',
      'differentiate(form(life) ≠ form(image)) ∧ realism(brackets(form))',
      'tendsToward(idealismLikePerspective, when: clarified)',
    ],
  },
  {
    id: 'ys-i-45-3-hlo-law-of-maxims-need',
    chunkId: 'ys-i-45-3-law-of-maxims-need',
    label: 'Need a law of maxims to resolve the conflict (idealism ↔ realism)',
    clauses: [
      "tag('method','samapatti')","tag('mode','vicara')",
      "tag('faculty','buddhi')","tag('lens','fichte')","tag('scope','essence')",
      "tag('order','4.43')",

      'both{idealism, realism} := restOn(assumptions ← innerMaxim(subject))',
      'conflict(maxims) ⇒ seek(lawOfMaxims)',
    ],
  },
  {
    id: 'ys-i-45-3-hlo-preference-realism',
    chunkId: 'ys-i-45-3-preference-realism',
    label: 'Preference: realism (idealism too one-sided; realism not yet genetic)',
    clauses: [
      "tag('method','samapatti')","tag('mode','vicara')",
      "tag('stance','realism')","tag('lens','fichte')","tag('scope','essence')",
      "tag('order','4.44')",

      'idealism ⇒ negates(beingOfOpposite) // one-sided',
      'realism ⇒ leavesOpposite(undisputed) ∧ yet renders(inconceivable)',
      'thus(realism) := insufficientFor(geneticScience)',
    ],
  },
  {
    id: 'ys-i-45-3-hlo-interpolation-appearance',
    chunkId: 'ys-i-45-3-interpolation-appearance',
    label: 'Interpolation: intuition arises only as conditioned appearance',
    clauses: [
      "tag('method','samapatti')","tag('mode','vicara')",
      "tag('stance','synthesis')","tag('lens','fichte')","tag('scope','essence')",
      "tag('order','4.45')",

      'deny(self-supportingIntuition) // proven',
      'allow(conditionalIntuition, as: phenomenon-not-self-grounded)',
    ],
  },
  {
    id: 'ys-i-45-3-hlo-standpoint-unification',
    chunkId: 'ys-i-45-3-standpoint-unification',
    label: 'Standpoint: truth-as-appearance unifies idealism and realism',
    clauses: [
      "tag('method','samapatti')","tag('mode','vicara')",
      "tag('stance','synthesis')","tag('lens','fichte')","tag('scope','essence')",
      "tag('order','4.46')",

      'rootSelfIntuition := firstAppearance ∧ ground(of: allAppearances)',
      'validate(appearance, as: genuineTruth-as-appearance)',
      'require(modifications := necessary ⊢ intuited)',
      'error := taking(appearance, as: being)',
    ],
  },
  {
    id: 'ys-i-45-3-hlo-derive-error-from-absence',
    chunkId: 'ys-i-45-3-derive-error-from-absence',
    label: 'Derive seeming/error from necessary absence of truth',
    clauses: [
      "tag('method','samapatti')","tag('mode','vicara')",
      "tag('stance','synthesis')","tag('lens','fichte')","tag('scope','essence')",
      "tag('order','4.47')",

      'assume(necessaryAbsenceOfTruth) ⇒ derive({basis, form}(seeming⊕error))',
    ],
  },
  {
    id: 'ys-i-45-3-hlo-measure-of-ignorance',
    chunkId: 'ys-i-45-3-measure-of-ignorance',
    label: 'Law: specify ignorance ⇒ enumerate errors',
    clauses: [
      "tag('method','samapatti')","tag('mode','vicara')",
      "tag('stance','synthesis')","tag('lens','fichte')","tag('scope','essence')",
      "tag('order','4.48')",

      'rule: tell(whatYouDoNotKnow) ⇒ listPrecisely(errors⊕illusions)',
    ],
  },
]

export const YS_I_45_3_UNIT: DatasetUnit = {
  id: makeUnitId('i.45.3'),
  title: 'YS I.45 — Vicāra (Lecture 3): Perspectives (idealism, realism, law of maxims)',
  scope: 'essence',
  logosMode: 'prajna',
  synthesis: 'pre-factum',
  faculty: 'buddhi',
  lens: 'fichte',
  chunks: CHUNKS_I_45_3,
  hlos: HLOS_I_45_3,
}
