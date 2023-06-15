/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,jsx,ts,tsx}"
  ],
  theme: {
    extend: {
      colors: {
        'hover': '#24c8db',
        's-neutron-t': '#f8fafc',
        's-neutron-b': '#475569',
        's-red-t': '#fef2f2',
        's-red-b': '#b91c1c',
        's-orange-t': '#431407',
        's-orange-b': '#fdba74',
        's-blue-t': '#083344',
        's-blue-b': '#22d3ee',
        's-young-t': '#022c22',
        's-young-b': '#6ee7b7',
        's-white-t': '#030712',
        's-white-b': '#f9fafb',
        's-black-t': '#f9fafb',
        's-white-b': '#030712',
        's-hyper-t': '#2e1065',
        's-hypber-b': '#c4b5fd',
        's-nebula-t': '#fdf2f8',
        's-nebula-b': '#db2777',
        's-unstable-t': '#eef2ff',
        's-unstable-b': '#4f46e5',
        'w-moon-t': '#f8fafc',
        'w-moon-b': '#64748b',
        'w-gas-t': '#fff7ed',
        'w-gas-b': '#ea580c',
        'w-nebula-t': '#422006',
        'w-nebula-b': '#fde047',
        'w-asteroid-t': '#1a2e05',
        'w-asteroid-b': '#bef264',
        'w-planet-t': '#ecfdf5',
        'w-planet-b': '#059669',
        'w-debris-t': '#083344',
        'w-debris-b': '#67e8f9',
        'w-orbital-t': '#fdf4ff',
        'w-orbital-b': '#c026d3',
        'w-jump-t': '#030712',
        'w-jump-b': '#f9fafb',
        'w-gravity-t': '#f9fafb',
        'w-gravity-b': '#030712',
      },
      fontFamily: {
        'modeseven': ["Modeseven", "sans-serif"]
      }
    },
  },
  plugins: [],
}
