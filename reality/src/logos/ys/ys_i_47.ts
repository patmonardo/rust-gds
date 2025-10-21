import { DatasetUnit, makeUnitId } from '@organon/gdsl/registry/canon'

// Keep arrays module-private to avoid leaking CHUNKS/HLOS.
const CHUNKS_I_47 = [
  {
    id: 'ys-i-47-nirvicara-vaisaradya',
    title: 'I.47 — nirvicāra-vaiśāradye adhyātma-prasāda',
    source:
      'Conventional: “When there is lucidity/maturity (vaiśāradya) in nirvicāra [samāpatti], there is clarity/serenity (prasāda) of the inner domain (adhyātma).”',
  },
  {
    id: 'ys-i-47-technical-gloss',
    title: 'Technical gloss (Yoga usage)',
    source:
      'nirvicāra = reflection-less subtle absorption (tanmātra-object); vaiśāradya = ripeness/crystalline lucidity; adhyātma = inner field (citta/buddhi, sattva-dominant); prasāda = transparency/serene clarity.',
  },
  {
    id: 'ys-i-47-review-preface',
    title: 'Twelfth Lecture preface: confidence in inner spirit and outer method',
    source:
      '(paraphrase) Audience has grasped both inner spirit and outer method; proceed by abstracting from what does not arise on this path and continue with strength.',
  },
  {
    id: 'ys-i-47-review-1-production',
    title: 'Review 1/4 — Production of insight; root factical; presuppose inner life for any “through”',
    source:
      '(paraphrase) Any “through” requires, as condition of possibility, an inner life grounded in itself and independent of the “through.” Produced insight may have genetic aspects in content, but at root is factical.',
  },
  {
    id: 'ys-i-47-review-2-idealism',
    title: 'Review 2/4 — Objectify insight; energetic concept as principle of intuition (idealistic perspective)',
    source:
      '(paraphrase) Make the produced insight an object; propose an energetic, living concept whose inward life is principle of energetic insight into a life beyond—principle of intuition and of life in intuition. This arises only as intuition (form projects external existence). If we begin from reflection’s energy as principle, call it “idealistic perspective.”',
  },
  {
    id: 'ys-i-47-review-3-realism',
    title: 'Review 3/4 — Presuppose life in-itself absolutely; all being originates in it (realistic perspective)',
    source:
      '(paraphrase) Life in itself is entirely and unconditionally in-itself; all being and life originate in it. Subjective condition: do not cling to idealism’s principle; yield to the opposite insight. This is the realistic perspective.',
  },
  {
    id: 'ys-i-47-review-warning-natural-stances',
    title: 'Warning: “idealism/realism” here are natural stances, not school systems',
    source:
      '(paraphrase) Do not treat idealism/realism as artificial systems opposed by science; they arise naturally in common knowing (in derivative expressions) and are to be derived as natural disjunctions/partialities.',
  },
  {
    id: 'ys-i-47-review-4-genesis',
    title: 'Review 4/4 — Elevate above both; from facticity to their genesis; ever higher genesis',
    source:
      '(paraphrase) Specify both perspectives inwardly; elevate above them from facticity to their genesis out of their mutual principles. The lived insight now is their genesis; continue rising to higher genesis until self-dissolution in it.',
  },
  {
    id: 'ys-i-47-review-supersedes-note',
    title: 'Note: review as instructional pipeline; may supersede prior HLO scaffolds',
    source:
      '(paraphrase) Review stages pass “dead being” forward; each stage receives prior payload. This packed pipeline can supersede earlier HLO scaffolds by providing higher-genesis formulations.',
  },
  {
    id: 'ys-i-47-review-idealism-realism-factical-root',
    title: 'Review — Idealism/Realism are factical at root; insufficient as highest principles',
    source:
      '(paraphrase) Idealism absolutizes reflection (self-positing, factical). Realism absolutizes content (self-given, factical). Each annuls the other; each bears the mark of insufficiency as a highest principle for the science of knowing.',
  },
  {
    id: 'ys-i-47-review-highest-contradiction-formula',
    title: 'Review — Highest contradiction: 0 and C; form/content; outer/inner; essence/existence',
    source:
      '(paraphrase) At the apex of contradiction demanding unification stand: 0 and C; form and content; outer and inner existence; essence and existence. Absolute disjunction obtained; its unification promises absolute oneness.',
  },
  {
    id: 'ys-i-47-prelim-cannot-combine-must-investigate-facticity',
    title: 'Prelim — Not solvable by recombining prior; investigate facticity genetically',
    source:
      '(paraphrase) Our next aim cannot be met by combining/rearranging what we know. Prior work was preparation; any further significance must be deduced from the highest principle. Something remains empirical/concrete—investigate its facticity genetically.',
  },
  {
    id: 'ys-i-47-choose-realism-fight-on-own-grounds',
    title: 'Strategy — Develop realism; fight it on its own ground; catch self-contradiction',
    source:
      '(paraphrase) Idealism renders realism impossible; realism denies idealism but still relates negatively to it. Attend to realism, abstracting from idealism; we cannot grant realism absolutely, so correct it by catching it in self-contradiction, turning its empirical principle into a genetic one—toward a higher realism-and-idealism as one.',
  },
  {
    id: 'ys-i-47-in-itself-meaning-negates-thinking',
    title: 'In-itself (adhyātman as “in-itself”) — meaning only as negation of construction/thinking',
    source:
      '(paraphrase) Think the in-itself energetically: it has meaning only as not-constructed and as denying all construction/constructability; “thus it is in itself” = exists independently of any asserting/thinking/intuition. Result: describe in-itself purely as what negates thinking.',
  },
  {
    id: 'ys-i-47-realism-genetic-turn',
    title: 'Genetic turn — Realism acts like the in-itself and collapses into it',
    source:
      '(paraphrase) Previously we factically saw: if life-in-itself is posited, nothing else can exist. Now, genetically: realism itself acts like the in-itself, negating everything outside itself; for this implicit reason, in our earlier appearance, it annulled all else. Thus we comprehend realism genetically, not merely factically.',
  },
  {
    id: 'ys-i-47-in-itself-energetic-thinking',
    title: 'Energetic thinking of the in-itself; proposition: thinking annuls itself before it',
    source:
      '(paraphrase) Exact, energetic thinking of the in-itself is required for the insight; without it, one may “think” the in-itself lifelong and never see. Proposition: “In thought, thinking annuls itself in the face of the in-itself.” Insight presupposes positive thought.',
  },
  {
    id: 'ys-i-47-absolute-intuition-negation',
    title: 'Absolute intuition projects negation (pure nothing); deeper realism refutes absolute idealism',
    source:
      '(paraphrase) The negation of thinking vis‑à‑vis the in‑itself is immediately evident (intuition) — hence absolute intuition here projects negation, pure nothing, against the absolute in‑itself. Thus idealism (absolute intuition of life) is refuted at the root by a deeper founding of realism; it may reappear only as appearance.',
  },
  {
    id: 'ys-i-47-construction-constructs-itself',
    title: 'Genuine construction: it constructs itself; thinking’s negation is directly evident',
    source:
      '(paraphrase) We “constructed” the in‑itself by positing its meaning in pure simplicity; yet in truth we did not construct it — it constructed itself by itself. Thinking’s negation grasped us directly from its simplicity.',
  },
  {
    id: 'ys-i-47-self-construction-light-one',
    title: 'Absolute self‑construction ≡ original light; no pregiven us (higher realism)',
    source:
      '(paraphrase) Intuition — the absolute springing forth of light — is bound with this self‑construction. Absolute self‑construction and original light are entirely one and inseparable; light from self‑construction and self‑construction from light. Nothing remains of a pregiven “us”: higher realistic perspective.',
  },
  {
    id: 'ys-i-47-higher-idealism-return',
    title: 'Requirement remains: think the in‑itself energetically → higher idealism',
    source:
      '(paraphrase) We still rightly require to think the in‑itself energetically: the living self‑construction within light must have yielded to us; this energy is first condition of everything — resulting in an idealism higher yet.',
  },
  {
    id: 'ys-i-47-awareness-light-presupposition',
    title: 'Light presupposed by awareness of thinking/energy; objective light sourced in in‑itself',
    source:
      '(paraphrase) Awareness of our energetic thinking presupposes light. But the light, in its objective form, does not exist in itself apart from the absolute; it has its source in the in‑itself. Hence we cannot appeal to it; examined closely, it bears witness against us.',
  },
  {
    id: 'ys-i-47-idealism-error-facticity-objectifying-light',
    title: 'Idealism’s highest error: fixation on facticity (objectifying light); must begin intellectually',
    source:
      '(paraphrase) Retaining the presupposition of light in all self‑conscious deliverances yields idealism’s constant spirit: fixed on facticity at the objectifying light. One can never begin factically here, only intellectually.',
  },
  {
    id: 'ys-i-47-idealist-objection-and-reply-knowing-sets-in-itself',
    title: 'Objection and reply: your knowing sets the in‑itself; it sets itself in your knowing',
    source:
      '(paraphrase) “You are not originally thinking the in‑itself, nor knowing it via something else; it is merely known by you—your knowing sets it down; more precisely, it sets itself down in and as your knowing.” You have always done this unconsciously in every “x is.” Philosophy binds not the deed but its thoughtlessness; now we must ask how far intuition’s testimony is valid.',
  },
  {
    id: 'ys-i-47-dead-intuition-vs-living-concept',
    title: 'Decisive ground: in‑itself in intuition is dead; in concept it is living',
    source:
      '(paraphrase) Whether as a dead system‑concept or as “is” of a particular, the faded in‑itself is always of intuition and is therefore dead. For us it exists in the concept, therefore living. Hence for us nothing is in intuition; everything is in the concept. This distinguishes the science of knowing from every other standpoint.',
  },
  {
    id: 'ys-i-47-privative-negations-descent',
    title: 'Privative negations and descent: re‑taking one‑sided stances as negations of the absolute',
    source:
      '(paraphrase) The science of knowing grasps other modes as negations of the in‑itself—not absolutely, but privatively. What ascent finds not absolutely valid (e.g., one‑sided realism/idealism), descent takes up again as similar possible negations of absolute insight.',
  },
]

const HLOS_I_47 = [
  {
    id: 'ys-i-47-hlo-baseline',
    chunkId: 'ys-i-47-nirvicara-vaisaradya',
    label: 'Baseline reading',
    clauses: [
      "tag('sutra','I.47')","tag('mode','vicara')","tag('phase','nirvicara')","tag('lens','yoga')",
      'if(lucidity(nirvicara)) ⇒ arise(clarity(innerField:=adhyatma))',
    ],
  },
  {
    id: 'ys-i-47-hlo-gloss',
    chunkId: 'ys-i-47-technical-gloss',
    label: 'Yoga-technical meaning (no analysis yet)',
    clauses: [
      "tag('sutra','I.47')","tag('note','technical')",
      'define(nirvicara := subtle, reflection-less samapatti)',
      'define(vaisaradya := maturity/lucidity)',
      'define(adhyatma := inner field, not metaphysical Self)',
      'define(prasada := sattva-transparency/serenity)',
    ],
  },
  {
    id: 'ys-i-47-hlo-review-preface',
    chunkId: 'ys-i-47-review-preface',
    label: 'Confidence in method: proceed on the path',
    clauses: [
      "tag('sutra','I.47')","tag('lens','fichte')","tag('mode','vicara')","tag('phase','nirvicara')",
      'assume(audience: grasp{innerSpirit, outerMethod})',
      'proceed(abstractFrom: non-arising, with: strength∧depth)',
    ],
  },
  {
    id: 'ys-i-47-hlo-review-1-production',
    chunkId: 'ys-i-47-review-1-production',
    label: 'Law: any “through” presupposes inner self-grounded life; root facticity of produced insight',
    clauses: [
      "tag('sutra','I.47')","tag('lens','fichte')","tag('stance','synthesis')",
      'exists(through) ⇒ presuppose(innerLife := self-grounded, independent(of: through))',
      'producedInsight := mayHave(geneticContent) ∧ root := factical',
    ],
  },
  {
    id: 'ys-i-47-hlo-review-2-idealism',
    chunkId: 'ys-i-47-review-2-idealism',
    label: 'Idealistic perspective: energetic concept ⇒ principle of intuition',
    clauses: [
      "tag('sutra','I.47')","tag('lens','fichte')","tag('stance','idealism')",
      'objectify(producedInsight) ⇒ posit(concept := energetic∧living)',
      'principle(concept.innerLife ⇒ insightInto(life-beyond))',
      'form(intuition) ⇒ projects(self-sufficient as external-existence)',
    ],
  },
  {
    id: 'ys-i-47-hlo-review-3-realism',
    chunkId: 'ys-i-47-review-3-realism',
    label: 'Realistic perspective: absolute life as origin of being; subjective yielding',
    clauses: [
      "tag('sutra','I.47')","tag('lens','fichte')","tag('stance','realism')",
      'declare(absoluteLife := in-itself, unconditional)',
      'origin(allBeing) := absoluteLife',
      'condition(subject) := yield(¬cling(idealismPrinciple))',
    ],
  },
  {
    id: 'ys-i-47-hlo-review-warning',
    chunkId: 'ys-i-47-review-warning-natural-stances',
    label: 'Natural stances, not systems; derive as disjunctions of common knowing',
    clauses: [
      "tag('sutra','I.47')","tag('lens','fichte')","tag('stance','synthesis')",
      'idealism∨realism := naturalStances(in: commonKnowing)',
      'task(science) := derive(disjunctions ∧ partialities, from: themselves)',
    ],
  },
  {
    id: 'ys-i-47-hlo-review-4-genesis',
    chunkId: 'ys-i-47-review-4-genesis',
    label: 'Meta-genesis: elevate from facticity to genesis of both; ascend to higher genesis',
    clauses: [
      "tag('sutra','I.47')","tag('lens','fichte')","tag('stance','synthesis')",
      'elevate(above: {idealism, realism})',
      'transition(factical ⇒ genetic{ofBoth, from: mutualPrinciples})',
      'live(insight := theirGenesis) ⇒ iterate(ascend ⇒ higherGenesis ⇒ self-dissolve)',
    ],
  },
  {
    id: 'ys-i-47-hlo-review-supersedes-note',
    chunkId: 'ys-i-47-review-supersedes-note',
    label: 'Instructional pipeline: later stages supersede earlier HLO scaffolds',
    clauses: [
      "tag('sutra','I.47')","tag('lens','fichte')","tag('note','pipeline')",
      'stage(n) := receives(packedPayload, from: stages < n)',
      'maySupersede(HLOs: earlierObjectiveLogic) // review provides higher-genesis forms',
    ],
  },
  {
    id: 'ys-i-47-hlo-idealism-realism-factical-root',
    chunkId: 'ys-i-47-review-idealism-realism-factical-root',
    label: 'Both perspectives: root facticity ⇒ insufficiency as highest principles',
    clauses: [
      "tag('sutra','I.47')","tag('lens','fichte')","tag('mode','vicara')","tag('phase','nirvicara')",
      "tag('stance','dialectic')","tag('note','factical-root')",
      'idealism := absolutize(reflection) ∧ self-positing(factical)',
      'realism := absolutize(content) ∧ self-given(factical)',
      'each(annuls(other)) ∧ each(marked(insufficientFor: highestPrinciple))',
    ],
  },
  {
    id: 'ys-i-47-hlo-highest-contradiction-formula',
    chunkId: 'ys-i-47-review-highest-contradiction-formula',
    label: 'Apex formula: {0,C} ≡ {form,content} ≡ {outer,inner} ≡ {essence,existence}',
    clauses: [
      "tag('sutra','I.47')","tag('lens','fichte')","tag('stance','dialectic')",
      "tag('topic','absolute-disjunction')","tag('goal','absolute-oneness')",
      'apexContradiction := pairs{{0,C},{form,content},{outer,inner},{essence,existence}}',
      'task := unify(absoluteDisjunction) ⇒ obtain(absoluteOneness)',
    ],
  },
  {
    id: 'ys-i-47-hlo-prelim-cannot-combine-must-investigate-facticity',
    chunkId: 'ys-i-47-prelim-cannot-combine-must-investigate-facticity',
    label: 'Prelim law: no recombination; investigate remaining facticity genetically',
    clauses: [
      "tag('sutra','I.47')","tag('lens','fichte')","tag('stance','dialectic')",
      '¬solveBy({combine, rearrange})(prior)',
      'require(deduce(from: highestPrinciple))',
      'task := investigate(facticity) ⊢ method(genetic)',
    ],
  },
  {
    id: 'ys-i-47-hlo-choose-realism-fight-on-own-grounds',
    chunkId: 'ys-i-47-choose-realism-fight-on-own-grounds',
    label: 'Method: develop realism; refute via self-contradiction → genetic principle',
    clauses: [
      "tag('sutra','I.47')","tag('lens','fichte')","tag('stance','realism')",
      'context: idealism ⇒ ¬possible(realism) ; realism ⇒ deny(idealism) ∧ relateNegatively',
      'choose(develop := realism)',
      'fightOn(ground := realism) ⇒ catch(self-contradiction) ⇒ lift(empiricalPrinciple → genetic)',
      'aim := higherUnity(realism ⊕ idealism)',
    ],
  },
  {
    id: 'ys-i-47-hlo-in-itself-meaning-negates-thinking',
    chunkId: 'ys-i-47-in-itself-meaning-negates-thinking',
    label: 'Define: in-itself = not-constructed; negates construction/constructability/thinking',
    clauses: [
      "tag('sutra','I.47')","tag('lens','fichte')","tag('topic','adhyatman/in-itself')",
      'thinkEnergetically(inItself)',
      'inItself := ¬constructed ∧ deny({construction, constructability})',
      'thus(inItself) ⇒ independentOf({asserting, thinking, intuiting})',
      'therefore: describe(inItself) := negation(of: thinking)',
    ],
  },
  {
    id: 'ys-i-47-hlo-realism-genetic-turn',
    chunkId: 'ys-i-47-realism-genetic-turn',
    label: 'Genetic comprehension: realism behaves as in-itself, hence annuls the other',
    clauses: [
      "tag('sutra','I.47')","tag('lens','fichte')","tag('stance','realism')",
      'prior := factical(“posited life-in-itself ⇒ nothing else exists”)',
      'now := genetic(see: realism ≈ inItself) ⇒ negate(outsideSelf)',
      'hence(explain: earlierAppearance := annuls(everythingOutside))',
    ],
  },
  {
    id: 'ys-i-47-hlo-in-itself-energetic-thinking',
    chunkId: 'ys-i-47-in-itself-energetic-thinking',
    label: 'Law: energetic thinking needed; “thinking annuls itself before the in‑itself”',
    clauses: [
      "tag('sutra','I.47')","tag('lens','fichte')","tag('topic','adhyatman/in-itself')",
      'require(thinkEnergetically(inItself)) ⇒ obtain(insight)',
      'insight ⊢ presuppose(positiveThought)',
      'proposition: inThought(thinking) ⇒ annulsSelf(against: inItself)',
    ],
  },
  {
    id: 'ys-i-47-hlo-absolute-intuition-negation',
    chunkId: 'ys-i-47-absolute-intuition-negation',
    label: 'Absolute intuition = immediate negation; deeper realism undercuts absolute idealism',
    clauses: [
      "tag('sutra','I.47')","tag('lens','fichte')","tag('stance','realism')",
      'negation(thinking ↔ inItself) := intuited(immediate)',
      'absoluteIntuition ⇒ project(negation := pureNothing) ⟂ (absoluteInItself)',
      'therefore: refute(idealism_as:absoluteIntuitionOfLife) atRoot',
      'allow(idealism := appearanceOnly)',
    ],
  },
  {
    id: 'ys-i-47-hlo-construction-constructs-itself',
    chunkId: 'ys-i-47-construction-constructs-itself',
    label: 'Genuine construction: self‑construction; we do not construct it',
    clauses: [
      "tag('sutra','I.47')","tag('lens','fichte')","tag('stance','dialectic')",
      'posit(inItself, with: pureSimplicity∧meaning)',
      'see: weDidNotConstruct(it) ∧ itConstructed(itself, by: itself)',
      'negation(thinking) := directlyEvident(graspsUs)',
    ],
  },
  {
    id: 'ys-i-47-hlo-self-construction-light-one',
    chunkId: 'ys-i-47-self-construction-light-one',
    label: 'Identity: absolute self‑construction ≡ original light (higher realism)',
    clauses: [
      "tag('sutra','I.47')","tag('lens','fichte')","tag('stance','realism:high')",
      'bind(intuition := springingForth(light), with: selfConstruction)',
      'equate(selfConstruction ≡ originalLight) // inseparable',
      'eliminate(pregivenUs)',
    ],
  },
  {
    id: 'ys-i-47-hlo-higher-idealism-return',
    chunkId: 'ys-i-47-higher-idealism-return',
    label: 'Return demand: think in‑itself energetically → higher idealism',
    clauses: [
      "tag('sutra','I.47')","tag('lens','fichte')","tag('stance','idealism:high')",
      'require(thinkEnergetically(inItselfWithin: lightSelfConstruction))',
      'energy := firstCondition(of: everything)',
      'conclude(perspective := idealismHigher)',
    ],
  },
]

// Fichte mapping (In-Itself, Prajñā-as-Principle, Brahman/Puruṣa) to be added after the intro.
export const YS_I_47_UNIT: DatasetUnit = {
  id: makeUnitId('i.47'),
  title: 'YS I.47 — Nirvicāra-vaiśāradya: adhyātma-prasāda',
  scope: 'essence',
  logosMode: 'prajna',
  synthesis: 'pre-factum',
  faculty: 'buddhi',
  lens: 'yoga',
  chunks: CHUNKS_I_47 as any,
  hlos: HLOS_I_47 as any,
}
