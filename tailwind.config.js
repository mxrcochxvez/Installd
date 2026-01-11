/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        main: '#050505',
        surface: '#121212',
        surfaceHighlight: '#1E1E1E',
        acid: {
          DEFAULT: '#ccff00',
          hover: '#b3e600',
          dim: 'rgba(204, 255, 0, 0.1)',
        },
        dim: '#888888',
      },
      fontFamily: {
        sans: ['Space Grotesk', 'sans-serif'],
        mono: ['Space Mono', 'monospace'],
      },
      backgroundImage: {
        'grid-pattern': "linear-gradient(to right, #222 1px, transparent 1px), linear-gradient(to bottom, #222 1px, transparent 1px)",
      }
    },
  },
  plugins: [],
}
