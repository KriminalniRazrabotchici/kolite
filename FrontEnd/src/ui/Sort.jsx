import { BiSortAlt2 } from 'react-icons/bi';
import { Button, SortButton } from './ButtonNav';
import { useState } from 'react';
import styled from 'styled-components';
import { useOutsideClick } from '../hooks/useOutsideClick';

const SortDiv = styled.div`
  display: flex;
  justify-content: center;

  height: 20rem;
  width: 20rem;

  position: fixed;
  top: 7%;
  right: 0;

  background-color: var(--black);

  z-index: 9999;
`;

const Ul = styled.ul`
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: space-around;
`;

function Sort() {
  const [showSort, setShowSort] = useState(false);

  function close() {
    setShowSort(false);
  }

  function handleClick(e) {
    e.stopPropagation(false);
    setShowSort((sort) => !sort);
  }

  const ref = useOutsideClick(close, false);
  return (
    <>
      <Button onClick={handleClick}>
        <BiSortAlt2 />
      </Button>

      {showSort && (
        <SortDiv ref={ref}>
          <Ul>
            <li>
              <SortButton>Highest price</SortButton>
            </li>
            <li>
              <SortButton>Lowest price</SortButton>
            </li>
            <li>
              <SortButton>Newest(year)</SortButton>
            </li>
            <li>
              <SortButton>Oldest(year)</SortButton>
            </li>
          </Ul>
        </SortDiv>
      )}
    </>
  );
}

export default Sort;
