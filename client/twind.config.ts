import { Options } from '$fresh/plugins/twind.ts';
import * as colors from 'twind/colors';

export default {
  selfURL: import.meta.url,
  theme: {
    colors,
    extend: {
      width: {
        76: '19rem',
        'fit-content': 'fit-content',
      },
      height: {
        76: '19rem',
      },
    },
  },
} as Options;
