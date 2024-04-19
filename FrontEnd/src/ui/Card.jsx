import styled from 'styled-components';

import { IoMdHeart, IoMdHeartEmpty } from 'react-icons/io';
import { useState } from 'react';
import {
  respondToLandscapeTablets,
  respondToMobile,
  respondToSmallLaptop,
} from '../styles/mediaQueries';

const StyledCard = styled.div`
  width: 80%;
  height: 65vh;

  padding: 2.4rem;

  position: relative;

  box-shadow: var(--shadow-md);

  &:hover {
    box-shadow: var(--shadow-lg);
  }

  ${respondToMobile(`
    width: 100%;
    height: 20vh;
    
    display: flex;
    gap: 1.2rem;

  `)}
`;

const Favourite = styled.button`
  border: none;
  background-color: transparent;

  & svg {
    fill: var(--color-red-500);
    width: 3rem;
    height: 3rem;
  }

  ${respondToSmallLaptop(`
  & svg {
    width: 2.5rem;
    height: 2.5rem;
  }
  `)}

  ${respondToMobile(`
    display: none;
  `)}
`;

const ImgBox = styled.div`
  display: flex;
  align-items: center;
  justify-content: center;
  width: 100%;
  height: 40%;

  ${respondToMobile(`
  width: 60%;
    height: 100%;
  `)}
`;

const Img = styled.img`
  width: 100%;
  height: 100%;
`;

const Body = styled.div`
  ${respondToMobile(`
  display: flex;
  flex-direction: column;
  `)}
`;

const Price = styled.div`
  display: flex;
  align-items: center;
  justify-content: end;

  margin: -0.8rem 0;

  ${respondToMobile(`

    display: none;
  `)}
`;

const PriceMobile = styled.div`
  display: flex;
  align-items: center;
  justify-content: end;

  margin: -0.8rem 0;
  display: none;

  ${respondToMobile(`
    margin: 0;
    display: flex;
  `)}
`;

const P = styled.p`
  background-color: var(--white);
  display: flex;
  align-items: center;
  justify-content: center;

  font-size: 1.4rem;
  font-weight: 600;

  width: 10rem;
  height: 3rem;

  border: 1px solid var(--color-red-500);
  border-radius: var(--border-radius-round);

  transform: translateY(-0.8rem);

  ${respondToSmallLaptop(`
    
  font-size: 1.2rem;

  width: 9rem;
  height: 2.5rem;
  `)}

  ${respondToMobile(`
    width: 12rem;

    font-size: 1.4rem;
    font-weight: 500;

    border: 1px solid var(--color-grey-300);
    border-radius: 0;

    transform: translateY(0);
  `)}
`;

const Heading = styled.h2`
  /* background-color: red; */
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin: 1.2rem 0;

  padding-bottom: 0.8rem;

  font-size: 2.4rem;
  font-weight: 600;

  border-bottom: 1px solid var(--black);

  ${respondToSmallLaptop(`
    
  font-size: 2rem;
  `)}

  ${respondToLandscapeTablets(`
    
    font-size: 1.8rem;
    `)}

  ${respondToMobile(`
    font-size: 1.6rem;

    margin: 0;
    margin-bottom: 1.2rem ;
    padding-bottom: 0rem;

    border-bottom: 0px solid var(--black);
  `)}
`;

const HeadingMini = styled.h3`
  margin: 2.4rem 0 0.8rem 0;

  font-size: 1.8rem;
  font-weight: 500;

  ${respondToSmallLaptop(`
    
  font-size: 1.6rem;
  `)}

  ${respondToLandscapeTablets(`
    font-size: 1.4rem;
  `)}

${respondToMobile(`
    margin: 0.8rem 0;
    font-size: 1.2rem;
    `)}
`;

const Description = styled.p`
  ${respondToMobile(`display: none;`)}
`;

const TextBox = styled.div`
  font-size: 1.4rem;

  ${respondToLandscapeTablets(`
    
  font-size: 1.2rem;
  `)}

  ${respondToMobile(`
    
  `)}
`;

function Card({ car }) {
  const [isFavourite, setIsFavourite] = useState(false);

  return (
    <StyledCard onClick={() => console.log('Clicked')}>
      <ImgBox>
        <Img src={car.images.at(0)} alt={car.brand} />
      </ImgBox>
      <Body>
        <Price>
          <P>{car.price} lv.</P>
        </Price>
        <Heading>
          {car.brand} {car.model}
          <Favourite onClick={() => setIsFavourite((fav) => !fav)}>
            {isFavourite ? <IoMdHeart /> : <IoMdHeartEmpty />}
          </Favourite>
        </Heading>
        <PriceMobile>
          <P>{car.price} lv.</P>
        </PriceMobile>
        <TextBox>
          <HeadingMini>
            {car.year}, {car.fuelType}, {car.mileage} km
          </HeadingMini>
          <Description>{car.description.slice(0, 150)}</Description>
        </TextBox>
      </Body>
    </StyledCard>
  );
}

export default Card;
