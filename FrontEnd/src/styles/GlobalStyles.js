import { createGlobalStyle } from 'styled-components';

const GlobalStyles = createGlobalStyle`

:root {
--black: #000;
--white: #fff;
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
  font-family: "Roboto", sans-serif;

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
