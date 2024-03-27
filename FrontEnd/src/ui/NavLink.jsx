import styled from 'styled-components';
import { NavLink } from 'react-router-dom';

export const StyledNavLink = styled(NavLink)`
  text-decoration: none;
  display: flex;
  align-items: center;
  justify-content: center;
  /* width: 10rem; */
  height: 4rem;

  text-transform: uppercase;
  font-size: 1.8rem;

  position: relative;

  &:link,
  &:visited {
    color: var(--white);

    transition: all 0.5s;
  }

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
  &:active,
  &.active:link,
  &.active:visited {
    /* text-decoration: line-through; */
    &::before {
      width: 100%;
    }

    transform: translateY(-0.5rem);
  }
`;
