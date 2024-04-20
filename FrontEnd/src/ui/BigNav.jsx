import styled from 'styled-components';
import { respondToLandscapeTablets } from '../styles/mediaQueries';

export const StyledBigNav = styled.div`
  padding-top: 1.2rem;

  display: flex;
  align-items: center;
  justify-content: space-between;

  padding: 0 2.4rem;

  font-size: 2rem;
  height: 7rem;

  ${respondToLandscapeTablets(`
  

`)}
`;
