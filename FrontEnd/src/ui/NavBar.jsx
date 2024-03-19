import styled from 'styled-components';
import { StyledNavLink } from './NavLink';

import { BiSortAlt2, BiHeart } from 'react-icons/bi';
import { Nav } from './Nav';
import { NavItems } from './NavUl';
import { ContainerRight } from './NavRightSide';
import { Button, ButtonLink } from './ButtonNav';
import OpenLogin from '../features/login/OpenLogin';

const Logo = styled.div``;

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
          <OpenLogin />
        </li>
        <li>
          <ButtonLink>Register</ButtonLink>
        </li>
        <li>
          <ButtonLink>Add yours</ButtonLink>
        </li>
      </NavItems>
      <ContainerRight>
        <Button>
          <BiHeart />
        </Button>
        <Button>
          <BiSortAlt2 />
        </Button>
      </ContainerRight>
    </Nav>
  );
}

export default NavBar;
