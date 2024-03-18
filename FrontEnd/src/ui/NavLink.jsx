import styled from 'styled-components';
import { NavLink } from 'react-router-dom';

export const StyledNavLink = styled(NavLink)`
  text-decoration: none;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 13rem;
  height: 4rem;

  text-transform: uppercase;
  font-size: 1.8rem;

  &:link,
  &:visited {
    color: var(--white);

    transition: all 0.5s;
  }

  &:hover,
  &:active,
  &.active:link,
  &.active:visited {
    /* text-decoration: line-through; */

    transform: translateY(-0.5rem);
  }
`;
