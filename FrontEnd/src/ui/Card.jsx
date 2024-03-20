import styled from 'styled-components';

const StyledCard = styled.div`
  /* background-color: blue; */

  width: 80%;
  height: 65vh;

  padding: 2.4rem;

  position: relative;

  /* border: 1px solid var(--color-grey-600); 
  */
  box-shadow: var(--shadow-md);

  &:hover {
    box-shadow: var(--shadow-lg);
  }
`;

const ImgBox = styled.div`
  /* background-color: red; */
  display: flex;
  align-items: center;
  justify-content: center;
  width: 100%;
  height: 40%;
`;

const Img = styled.img`
  width: 100%;
  height: 100%;
`;

const Price = styled.div`
  display: flex;
  align-items: center;
  justify-content: end;

  margin: -0.8rem 0;
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
`;

const Heading = styled.h2`
  margin: 1.2rem 0;

  padding-bottom: 0.8rem;

  font-size: 2.4rem;
  font-weight: 600;

  border-bottom: 1px solid var(--black);
`;

const HeadingMini = styled.h3`
  margin: 2.4rem 0 0.8rem 0;

  font-size: 1.8rem;
  font-weight: 500;
`;

const TextBox = styled.div`
  font-size: 1.4rem;
`;

function Card({ car }) {
  return (
    <StyledCard onClick={() => console.log('Clicked')}>
      <ImgBox>
        <Img src={car.images.at(0)} alt={car.brand} />
      </ImgBox>
      <Price>
        <P>{car.price} lv.</P>
      </Price>
      <Heading>
        {car.brand} {car.model}
      </Heading>
      <TextBox>
        <HeadingMini>
          {car.year}, {car.fuelType}, {car.mileage} km
        </HeadingMini>
        {car.description.slice(0, 150)}
      </TextBox>
    </StyledCard>
  );
}

export default Card;
