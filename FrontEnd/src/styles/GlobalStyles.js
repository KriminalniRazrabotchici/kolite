import { createGlobalStyle } from 'styled-components';

const GlobalStyles = createGlobalStyle`

:root {
    --black: #000;
    --white: #fff;

    --color-grey-50: #f9fafb;
    --color-grey-100: #f3f4f6;
    --color-grey-200: #e5e7eb;
    --color-grey-300: #d1d5db;
    --color-grey-400: #9ca3af;
    --color-grey-500: #6b7280;
    --color-grey-600: #4b5563;
    --color-grey-700: #374151;
    --color-grey-800: #1f2937;
    --color-grey-900: #111827;

    --color-red-100: #fee2e2;
    --color-red-500: #EF0107;
    --color-red-700: #b91c1c;
    --color-red-800: #991b1b;

    --backdrop-color: rgba(255, 255, 255, 0.1);
    
    --shadow-sm: 0 1px 2px rgba(0, 0, 0, 0.04);
    --shadow-md: 0px 0.6rem 2.4rem rgba(0, 0, 0, 0.06);
    --shadow-lg: 0 2.4rem 3.2rem rgba(0, 0, 0, 0.12);

    --border-radius-tiny: 3px;
    --border-radius-sm: 5px;
    --border-radius-md: 7px;
    --border-radius-lg: 9px;
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
