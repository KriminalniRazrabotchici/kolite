import styled from 'styled-components';
import {
  respondToLandscapeTablets,
  respondToMobile,
  respondToSmallLaptop,
} from '../styles/mediaQueries';

export const SearchDiv = styled.div`
  /* background-color: purple; */
  width: 90%;
  height: 10%;
  margin: 2.4rem auto;

  display: flex;
  align-items: center;
  justify-content: center;

  flex-flow: row wrap;

  gap: 0.8rem;

  ${respondToSmallLaptop(`
    height: 18%;
  `)}

  ${respondToLandscapeTablets(`
    margin: 3rem auto;
    height: 23%;
  `)}

  ${respondToMobile(`
    height: 28%;
  `)}
`;
