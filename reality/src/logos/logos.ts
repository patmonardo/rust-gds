export const LOGOS_NAMESPACE = 'reality:logos'
export const LOGOS_SCOPE: 'being-only' = 'being-only'
export const INVARIANTS = {
  throughOneAnother: 'through-one-another',
  firstPrinciple: 'light'
}

export const METHOD_SPEC = {
  chunks: [
    {
      id: 'fichte-1804-cycle-a',
      title: 'Fichte 1804 — Science of Knowing (Cycle A.1)',
      summary: 'Samapatti method articulated in Fichte’s Science of Knowing.',
      source: 'doc/spec/index.ts'
    }
  ],
  hlos: [
    {
      id: 'fichte-op-1-original-positing',
      chunkId: 'fichte-1804-cycle-a',
      label: 'Original positing (Ich-Setzung) — vitarka/savitarka',
      digest: 'Immediate determination of Being by consciousness (original).',
      clauses: [
        "tag('lens','fichte')","tag('method','samapatti')",
        "tag('mode','vitarka')","tag('phase','savitarka')",
        "tag('faculty','manas')","tag('role','original')",
        "tag('plane','dyadic')","tag('cycle','fi:sk:1804:A.1')","tag('order','1')",
        "tag('principle','prajna')",
        "annotate('meaning',{ lens:'fichte', gloss:'Ich-Setzung as immediate (original) positing; Being is posited by consciousness.' })"
      ]
    },
    {
      id: 'fichte-op-2-judgment',
      chunkId: 'fichte-1804-cycle-a',
      label: 'Judgment (counter-positing) — vitarka/nirvitarka',
      digest: 'Reason’s counter-judgment clarifies the original.',
      clauses: [
        "tag('lens','fichte')",
        "tag('method','samapatti')",
        "tag('mode','vitarka')",
        "tag('phase','nirvitarka')",
        "tag('faculty','buddhi')",
        "tag('role','judgment')",
        "tag('plane','dyadic')",
        "tag('cycle','fi:sk:1804:A.1')",
        "tag('order','2')"
      ]
    },
    {
      id: 'fichte-op-3-reflection',
      chunkId: 'fichte-1804-cycle-a',
      label: 'Reflection (essence articulation) — vicara/savicara',
      digest: 'Reflection articulates essence and conditions.',
      clauses: [
        "tag('lens','fichte')",
        "tag('method','samapatti')",
        "tag('mode','vicara')",
        "tag('phase','savicara')",
        "tag('faculty','manas')",
        "tag('role','reflection')",
        "tag('plane','dyadic')",
        "tag('cycle','fi:sk:1804:A.1')",
        "tag('order','3.1')",
        "tag('model','samskara')"
      ]
    },
    {
      id: 'fichte-op-4-reconstruction',
      chunkId: 'fichte-1804-cycle-a',
      label: 'Reconstruction (unity) — vicara/nirvicara',
      digest: 'Unity re-established; Absolute Knowing (cycle closure).',
      clauses: [
        "tag('lens','fichte')",
        "tag('method','samapatti')",
        "tag('mode','vicara')",
        "tag('phase','nirvicara')",
        "tag('faculty','buddhi')",
        "tag('role','reconstruction')",
        "tag('plane','dyadic')",
        "tag('cycle','fi:sk:1804:A.1')",
        "tag('order','4')",
        "tag('resolution','nirodha')"
      ]
    },
    {
      id: 'fichte-alignment-method',
      chunkId: 'fichte-1804-cycle-a',
      label: 'Method umbrella (species)',
      witnessEdges: [
        { type: 'METHOD_HAS', from: 'samapatti', to: 'vitarka' },
        { type: 'METHOD_HAS', from: 'samapatti', to: 'vicara' },
        { type: 'DYAD_HAS',   from: 'vitarka',   to: 'savitarka' },
        { type: 'DYAD_HAS',   from: 'vitarka',   to: 'nirvitarka' },
        { type: 'DYAD_HAS',   from: 'vicara',    to: 'savicara' },
        { type: 'DYAD_HAS',   from: 'vicara',    to: 'nirvicara' },
        { type: 'TRIAD_HAS',  from: 'prajna',    to: 'samskara' },
        { type: 'TRIAD_HAS',  from: 'samskara',  to: 'nirodha' }
      ]
    }
  ]
}

// Restore legacy exports to unblock imports
export const CHUNKS = METHOD_SPEC.chunks
export const HLOS = METHOD_SPEC.hlos
