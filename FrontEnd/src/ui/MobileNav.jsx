import styled from 'styled-components';
import { motion } from 'framer-motion';
import { respondToLandscapeTablets } from '../styles/mediaQueries';

export const StyledMobileNav = styled(motion.div)`
  display: none;
  opacity: 0;
  pointer-events: none;
  height: 100vh;
  background-color: transparent;

  ${respondToLandscapeTablets(`
  opacity: 1;
  pointer-events: auto;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: start;
`)};
`;
