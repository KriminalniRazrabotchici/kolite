import styled from 'styled-components';
import { StyledNavLink } from './NavLink';

const Nav = styled.nav`
  background-color: var(--black);
  color: var(--white);

  padding: 0 2.4rem;

  font-size: 2rem;

  display: flex;
  align-items: center;
  justify-content: space-between;

  height: 6rem;
`;

const Logo = styled.div``;

const NavItems = styled.ul`
  display: flex;
  justify-content: space-between;
  gap: 1.6rem;
`;

const Container = styled.div`
  display: flex;
  gap: 1.6rem;
`;

const Button = styled.button`
  color: var(--white);
  font-size: 2rem;
  background: transparent;
  border: none;
`;

function NavBar() {
  return (
    <Nav>
      <Logo>
        <span>Kolite</span>
      </Logo>
      <NavItems>
        <li>
          <StyledNavLink to='/home'>Home</StyledNavLink>
        </li>
        <li>
          <StyledNavLink to='/contacts'>Contacts</StyledNavLink>
        </li>
        <li>
          <StyledNavLink to='/login'>Login</StyledNavLink>
        </li>
        <li>
          <StyledNavLink to='/register'>Register</StyledNavLink>
        </li>
        <li>
          <StyledNavLink to='/publish'>Add yours</StyledNavLink>
        </li>
      </NavItems>
      <Container>
        <Button>♥</Button>
        <Button>sort</Button>
      </Container>
    </Nav>
  );
}

export default NavBar;
