export const respondToSmallLaptop = (content) => `
  @media (max-width: 64em) {
    ${content}
  }
`;

export const respondToLandscapeTablets = (content) => `
  @media (max-width: 48em) {
    ${content}
  }
`;

export const respondToMobile = (content) => `
  @media (max-width: 27em) {
    ${content}
  }
`;

export const respondToBiggerTablets = (content) => `
  @media (min-height: 64em) {
    ${content}
  }
`;
