import styled from 'styled-components';
import { StyledNavLink } from './NavLink';

import { BiSortAlt2, BiHeart } from 'react-icons/bi';
import { Nav } from './Nav';
import { NavItems } from './NavUl';
import { ContainerRight } from './NavRightSide';
import { Button, ButtonLink } from './ButtonNav';
import OpenAdd from '../features/cars/OpenAdd';
import useOpenModal from '../hooks/useOpenModal';

import OpenModal from '../utils/OpenModal';

const Logo = styled.div``;

function NavBar() {
  const { handleLoginButton, handleRegisterButton } = useOpenModal();

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
          <ButtonLink onClick={handleLoginButton}>Login</ButtonLink>
          <OpenModal />
        </li>
        <li>
          <ButtonLink onClick={handleRegisterButton}>Register</ButtonLink>
          <OpenModal />
        </li>
        <li>
          <OpenAdd />
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
