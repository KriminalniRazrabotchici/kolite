import styled from 'styled-components';
import {
  respondToLandscapeTablets,
  respondToMobile,
  respondToSmallLaptop,
} from '../styles/mediaQueries';

export const HomePage = styled.div`
  display: grid;
  grid-template-columns: repeat(4, 1fr);

  align-items: start;
  justify-items: center;

  row-gap: 6.4rem;

  height: 75vh;
  margin: 2.4rem;
  margin-top: 1.2rem;
  padding: 3.2rem;

  overflow: scroll;

  &::-webkit-scrollbar {
    display: none;
  }

  ${respondToSmallLaptop(`grid-template-columns: repeat(3, 1fr);`)}

  ${respondToLandscapeTablets(`grid-template-columns: repeat(2, 1fr);`)}

  ${respondToMobile(`
  grid-template-columns: 1fr;
  row-gap: 3.6rem;

  margin: 0;
  `)}
`;
