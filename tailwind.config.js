/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,jsx,ts,tsx}"
  ],
  theme: {
    extend: {
      colors: {
        'neutron-star-text': '#f8fafc',
        'neutron-star-bg': '#475569',
        'red-star-text': '#fef2f2',
        'red-star-bg': '#b91c1c',
        'orange-star-text': '#431407',
        'orange-star-bg': '#fdba74',
        'blue-star-text': '#083344',
        'blue-star-bg': '#22d3ee',
        'young-star-text': '#022c22',
        'young-star-bg': '#6ee7b7',
        'white-dwarf-text': '#030712',
        'white-dwarf-bg': '#f9fafb',
        'black-hole-text': '#f9fafb',
        'black-hold-bg': '#030712',
        'hypergiant-text': '#2e1065',
        'hypergiant-bg': '#c4b5fd',
        'nebula-text': '#fdf2f8',
        'nebula-bg': '#db2777',
        'unstable-text': '#eef2ff',
        'unstable-bg': '#4f46e5',
        'moon-text': '#f8fafc',
        'moon-bg': '#64748b',
        'gas-giant-text': '#fff7ed',
        'gas-giant-bg': '#ea580c',
        // 'nebula-text': '#422006',
        // 'nebula-bg': '#fde047',
        'asteroid-field-text': '#1a2e05',
        'asteroid-field-bg': '#bef264',
        'planet-text': '#ecfdf5',
        'planet-bg': '#059669',
        'debris-field-text': '#083344',
        'debris-field-bg': '#67e8f9',
        'orbital-station-text': '#fdf4ff',
        'orbital-station-bg': '#c026d3',
        'jump-gate-text': '#030712',
        'jump-gate-bg': '#f9fafb',
        'gravity-well-text': '#f9fafb',
        'gravity-well-bg': '#030712',
      },
      fontFamily: {
        'modeseven': ["Modeseven", "sans-serif"]
      }
    },
  },
  plugins: [],
}
