module.exports = {
  content: ["./src/**/*.rs", "./index.html"],
  theme: {
    extend: {
      colors: {
        magenta: "#A733C1",
        blue: "#3548CF",
        cyan: "#57C4B8",
        green: "#93DD3A",
        yellow: "#F5BD47",
        orange: "#F17719",
        red: "#D44729",
        black: "#0E202C",
      },
      spacing: {
        rail: "6px",
        stop: "48px",
        ring: "38px",
        "rail-lg": "8px",
        "stop-lg": "64px",
        "ring-lg": "48px",
      },
      borderWidth: {
        rail: "6px",
        "rail-lg": "8px",
      },
      fontFamily: {
        body: ["Merriweather Sans", "sans-serif"],
        display: ["Basteleur", "sans-serif"],
        mono: ["Fira Code", "Fira Mono", "monospace"],
      },
    },
  },
  plugins: [],
};
