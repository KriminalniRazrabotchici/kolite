import styled from 'styled-components';

export const Nav = styled.nav`
  background-color: var(--black);
  color: var(--white);
  max-width: 100%;

  height: ${(props) => (props.show ? '100vh' : '7rem')};
  padding: 0;

  @media (min-width: 48.1em) {
    height: 7rem;
    padding: 0 2.4rem;
  }
`;
