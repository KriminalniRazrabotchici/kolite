import styled from 'styled-components';
import {
  respondToLandscapeTablets,
  respondToSmallLaptop,
} from '../styles/mediaQueries';

export const NavItems = styled.ul`
  display: flex;
  justify-content: space-between;
  gap: 3.2rem;

  ${respondToSmallLaptop(`
gap: 2.4rem;
`)}

  ${respondToLandscapeTablets(`
  
display: none;
`)}
`;
