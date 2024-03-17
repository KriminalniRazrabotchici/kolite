import { createGlobalStyle } from 'styled-components';

const GlobalStyles = createGlobalStyle`

:root {
/* define colors */
}

*,
*::before,
*::after {
  box-sizing: border-box;
  padding: 0;
  margin: 0;
}

html {
  font-size: 62.5%;
}

body {
  font-family: /* select font-family*/ sans-serif;

  line-height: 1.5;

  overflow: hidden;
}

button {
  cursor: pointer;
}

button:disabled {
  cursor: not-allowed;
}

ul {
  list-style: none;
}

p,
h1,
h2,
h3,
h4,
h5,
h6 {
  overflow-wrap: break-word;
}
`;

export default GlobalStyles;

// - FONT SIZE SYSTEM (px)
// 10 / 12 / 14 / 16 / 18 / 20 / 24 / 30 / 36 / 44 / 52 / 62 / 74 / 86 / 98

// - SPACING SYSTEM (px)
// 2 / 4 / 8 / 12 / 16 / 24 / 32 / 48 / 64 / 80 / 96 / 128
