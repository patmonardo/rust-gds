import { DatasetUnit, makeUnitId } from '@organon/gdsl/registry/canon';

export const CHUNKS_I_46 = [
  {
    id: 'ys-i-46-sabija-samadhi',
    title: 'YS I.46 — ta eva sabīja samādhi (these are seed-bearing samādhi)',
    source:
      '(note) Closure of vitarka-vicāra (Objective Logic); bridge from Visesa-Aviseṣa to Liṅga-mātra samādhi track (dhāraṇā-dhyāna-samādhi).',
  },
];

export const HLOS_I_46 = [
  {
    id: 'ys-i-46-hlo-bridge',
    chunkId: 'ys-i-46-sabija-samadhi',
    label: 'Bridge: close Objective Logic; prepare transition to samādhi',
    clauses: [
      "tag('sutra','I.46')",
      "tag('moment','closure')",
      "tag('mode','vicara')",
      "tag('phase','savicara')",
      "tag('lens','fichte')",
      "tag('scope','essence')",
      'close(vitarka⊕vicara := sabijaSamadhi)',
      'prepare(transition := {prajna, samskara, nirodha})',
    ],
  },
];

export const YS_I_46_UNIT: DatasetUnit = {
  id: makeUnitId('i.46'),
  title: 'YS I.46 — ta eva sabīja samādhi',
  scope: 'essence',
  logosMode: 'prajna',
  synthesis: 'pre-factum',
  faculty: 'buddhi',
  lens: 'fichte',
  chunks: CHUNKS_I_46,
  hlos: HLOS_I_46,
};
