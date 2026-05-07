/**
 * Tailwind CSS configuration for ZingerBoost
 * Dark mode default, medical/diagnostic aesthetic
 */
/** @type {import('tailwindcss').Config} */
export default {
  darkMode: 'class',
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        brand: {
          50: '#f0f9ff',
          100: '#e0f2fe',
          500: '#0ea5e9',
          600: '#0284c7',
          900: '#0c4a6e',
        },
        risk: {
          safe: '#10b981',
          moderate: '#f59e0b',
          advanced: '#ef4444',
        },
        surface: {
          DEFAULT: '#0a0a0a',
          elevated: '#171717',
          border: '#262626',
        },
      },
      fontFamily: {
        sans: ['Inter', 'system-ui', 'sans-serif'],
      },
    },
  },
  plugins: [],
};
