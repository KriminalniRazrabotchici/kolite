import styled from 'styled-components';
import {
  respondToLandscapeTablets,
  respondToMobile,
  respondToSmallLaptop,
} from '../styles/mediaQueries';

export const Button = styled.button`
  display: flex;
  align-items: center;
  justify-content: center;

  width: 13rem;
  height: 3.5rem;

  font-size: 1.6rem;

  background-color: transparent;
  color: var(--color-red-500);
  /* word-wrap: break-word; */
  border: none;
  /* border-radius: var(--border-radius-round); */
  box-shadow: var(--shadow-md);

  position: relative;

  transition: all 0.5s;

  &::before {
    content: '';
    background-color: var(--color-red-500);
    position: absolute;
    left: 0;
    top: 100%;
    width: 0;
    height: 0.2rem;

    transition: all 0.5s;
  }

  &:hover,
  &:active {
    &::before {
      width: 100%;
    }
    color: var(--black);
    box-shadow: none;
  }

  ${respondToSmallLaptop(`
  width: 11rem;
  height: 3rem;

  font-size: 1.4rem;
  `)}

  ${respondToMobile(`
  width: 10rem;
  height: 2.5rem;

  font-size: 1.2rem;
  `)}
`;
