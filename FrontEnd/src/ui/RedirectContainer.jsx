import styled from 'styled-components';
import {
  respondToLandscapeTablets,
  respondToMobile,
} from '../styles/mediaQueries';

export const RedirectContainer = styled.div`
  margin: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.4rem;

  font-size: 1.6rem;

  ${respondToLandscapeTablets(`font-size: 1.4rem;`)}
  ${respondToMobile(`font-size: 1.2rem;`)}
`;
