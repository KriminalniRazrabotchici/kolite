import styled from 'styled-components';
import { respondToLandscapeTablets } from '../styles/mediaQueries';

export const StyledMobileNavButton = styled.button`
  padding: 0 1.2rem;

  text-decoration: none;
  background-color: transparent;

  border: none;
  cursor: pointer;

  display: none;

  & svg {
    color: var(--white);
    width: 3rem;
    height: 3rem;
  }

  ${respondToLandscapeTablets(`
display: block;

& svg {
  width: 3rem;
  height: 3rem;
}
`)}
`;
