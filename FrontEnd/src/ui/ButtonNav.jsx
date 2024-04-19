import styled from 'styled-components';
import {
  respondToLandscapeTablets,
  respondToMobile,
} from '../styles/mediaQueries';

export const Button = styled.button`
  color: var(--white);
  font-size: 2rem;
  background: transparent;
  border: none;

  & svg {
    width: 2.4rem;
    height: 2.4rem;
  }

  &:hover,
  &:active {
    & svg {
      color: var(--white);
    }
  }
`;

export const SortButton = styled(Button)`
  transition: all 0.5s;

  &:hover,
  &:active {
    color: var(--color-red-500);
    transform: translateX(2rem);
  }
`;

export const ButtonLink = styled.button`
  border: none;
  background: transparent;
  text-decoration: none;
  display: flex;
  align-items: center;
  justify-content: center;
  /* width: 10rem; */
  height: 4rem;

  text-transform: uppercase;
  font-size: 1.8rem;

  position: relative;

  color: var(--white);

  transition: all 0.5s;

  &::before {
    content: '';
    background-color: var(--white);
    position: absolute;
    /* left: 0; */
    width: 0;
    height: 0.3rem;

    transition: all 0.5s;
  }

  &:hover,
  &:active {
    /* text-decoration: line-through; */
    &::before {
      width: 100%;
    }

    transform: translateY(-0.5rem);
  }

  ${respondToLandscapeTablets(`font-size: 1.6rem;`)}
`;

export const ButtonRedirect = styled(ButtonLink)`
  height: 2rem;

  text-transform: lowercase;
  font-size: 1.6rem;

  color: var(--color-red-500);

  transition: all 0.5s;

  &::before {
    content: '';
    background-color: var(--color-red-500);
    position: absolute;
    left: 0;
    top: 100%;
    width: 0;
    height: 0.15rem;

    transition: all 0.5s;
  }

  &:hover,
  &:active {
    color: var(--black);
  }

  ${respondToLandscapeTablets(`font-size: 1.4rem;`)}
  ${respondToMobile(`font-size: 1.2rem;`)}
`;
