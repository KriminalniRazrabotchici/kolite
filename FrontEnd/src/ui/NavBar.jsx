import styled from 'styled-components';
import { StyledNavLink } from './NavLink';

import { BiSortAlt2, BiHeart } from 'react-icons/bi';
import { MdMenu, MdClose } from 'react-icons/md';
import { Nav } from './Nav';
import { NavItems } from './NavUl';
import { ContainerRight } from './NavRightSide';
import { Button, ButtonLink } from './ButtonNav';
import OpenAdd from '../features/cars/OpenAdd';
import useOpenModal from '../hooks/useOpenModal';

import OpenModal from '../utils/OpenModal';
import Sort from './Sort';
import { StyledMobileNavButton } from './MobileNavButton';
import { useState } from 'react';
import { StyledMobileNav } from './MobileNav';
import { StyledUlMobile } from './MobileUl';
import { StyledBigNav } from './BigNav';

const Logo = styled.div``;

function NavBar() {
  const { handleLoginButton, handleRegisterButton } = useOpenModal();

  const [showMenu, setShowMenu] = useState(false);

  function handleOpenCloseMenu() {
    setShowMenu((menu) => !menu);
  }

  return (
    <>
      <OpenModal />
      <Nav show={showMenu}>
        <StyledBigNav>
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
            </li>
            <li>
              <ButtonLink onClick={handleRegisterButton}>Register</ButtonLink>
            </li>
            <li>
              <OpenAdd />
            </li>
          </NavItems>
          <ContainerRight>
            <Button>
              <BiHeart />
            </Button>
            <Sort />
            <StyledMobileNavButton>
              {!showMenu ? (
                <MdMenu onClick={handleOpenCloseMenu} />
              ) : (
                <MdClose onClick={handleOpenCloseMenu} />
              )}
            </StyledMobileNavButton>
          </ContainerRight>
        </StyledBigNav>

        {showMenu && (
          <StyledMobileNav
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            transition={{ duration: 1.5, delay: 0 }}>
            <StyledUlMobile>
              <li>
                <StyledNavLink to='/home' onClick={() => setShowMenu(false)}>
                  Home
                </StyledNavLink>
              </li>
              <li>
                <StyledNavLink
                  to='/contacts'
                  onClick={() => setShowMenu(false)}>
                  Contacts
                </StyledNavLink>
              </li>
              <li>
                <ButtonLink onClick={handleLoginButton}>Login</ButtonLink>
              </li>
              <li>
                <ButtonLink onClick={handleRegisterButton}>Register</ButtonLink>
              </li>
              <li>
                <OpenAdd />
              </li>
            </StyledUlMobile>
          </StyledMobileNav>
        )}
      </Nav>
    </>
  );
}

export default NavBar;
