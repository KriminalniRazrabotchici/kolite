import { HomePage } from '../ui/HomePage';
import Card from '../ui/Card';
import SearchBy from '../ui/SearchBy';
import { useEffect } from 'react';

const cars = [
  {
    id: 1350,
    brand: 'BMW',
    model: '330',
    type: 'Sedan',
    month: 'October',
    year: 2018,
    price: '30,000',
    fuelType: 'Diesel',
    mileage: '63 000',
    transmission: 'manual',
    doors: '4/5',
    horsepower: 263,
    condition: 'used',
    color: 'white metalic',
    euroStandart: 'EURO 5',
    cube: '3000',
    images: ['bmw1.jpg'],
    description:
      'Lorem, ipsum dolor sit amet consectetur adipisicing elit. Sit quam cupiditate velit fugiat ipsam aut non ipsa sapiente, molestias numquam a quaerat impedit nemo reprehenderit quis dolore maiores quo mollitia!',
  },
  {
    id: 1265,
    brand: 'Mercedes',
    model: 'E 350',
    type: 'Wagon',
    month: 'October',
    year: 2013,
    price: '15,000',
    fuelType: 'Diesel',
    mileage: '183 000',
    transmission: 'manual',
    doors: '4/5',
    horsepower: 170,
    condition: 'used',
    color: 'dark grey metalic',
    euroStandart: 'EURO 5',
    cube: '2200',
    images: ['merc1.jpg'],
    description:
      'Нов внос северна Италия! ! ! EURO 5! ! ! FACE LIFT! ! ! AMG-пакет! ! ! AVANTGARDE! ! ! 2,2CDI-170k.с-много икономичен и динамичен двигател с верига! ! ! Седемстепенна автоматична скоростна кутия! ! ! Кожен ел салон! ! ! Мултифункционален кожен волан! ! ! Навигация! ! ! Автопилот! ! ! Сензор за дъжд! ! ! Двузонов климатроник! ! ! Автопилот! ! ! Автоматични ксенонови фарове+LED дневни светлини и стопове! ! ! Ел LED огледала с подгрев и прибиране! ! ! USB/AUX-изводи! ! ! Блутут! ! ! Преден и заден парктроник! ! ! Хладилна жабка! ! ! Седемнадесет цолови джанти с нови зимни гуми! ! ! Налични DPF и катализатори! ! ! Два броя ключове! ! ! Един собственик! ! ! УНИКАЛНО ЗАПАЗЕН АВТОМОБИЛ БЕЗ КАКВИТО И ДА БИЛО ЗАБЕЛЕЖКИ! ! ! БУКВАЛНО КАТО НОВ! ! ! Преглед в доверен сервиз на клиента! ! ! Съдействие при регистрация и транзитни номера! ! ! Лизинг чрез лизингова компания TBI BANK и AMIGO LIZING! ! ! БЛАГОДАРИМ ВИ ЗА ИНТЕРЕСА И УСПЕХ В ИЗБОРА НА АВТОМОБИЛ!',
  },
  {
    id: 2694,
    brand: 'Opel',
    model: 'Astra',
    type: 'Sedan',
    month: 'April',
    year: 2004,
    price: '6,000',
    fuelType: 'Benzin',
    mileage: '263 000',
    transmission: 'manual',
    doors: '4/5',
    horsepower: 101,
    condition: 'used',
    color: 'black metalic',
    euroStandart: 'EURO 4',
    cube: '1600',
    images: ['opel1.jpg'],
    description:
      'Lorem, ipsum dolor sit amet consectetur adipisicing elit. Sit quam cupiditate velit fugiat ipsam aut non ipsa sapiente, molestias numquam a quaerat impedit nemo reprehenderit quis dolore maiores quo mollitia!',
  },
  {
    id: 4965,
    brand: 'Audi',
    model: 'A3',
    type: 'Hatchback',
    month: 'December',
    year: 2007,
    price: '8,200',
    fuelType: 'Diesel',
    mileage: '247 000',
    transmission: 'manual',
    doors: '4/5',
    horsepower: 140,
    condition: 'used',
    color: 'grey metalic',
    euroStandart: 'EURO 5',
    cube: '2000',
    images: ['audi1.jpg'],
    description:
      'Lorem, ipsum dolor sit amet consectetur adipisicing elit. Sit quam cupiditate velit fugiat ipsam aut non ipsa sapiente, molestias numquam a quaerat impedit nemo reprehenderit quis dolore maiores quo mollitia!',
  },
  {
    id: 3974,
    brand: 'Kia',
    model: 'Ceed',
    type: 'Hatchback',
    month: 'April',
    year: 2008,
    price: '7,000',
    fuelType: 'Benzin',
    mileage: '294 000',
    transmission: 'manual',
    doors: '4/5',
    horsepower: 78,
    condition: 'used',
    color: 'blue metalic',
    euroStandart: 'EURO 4',
    cube: '1600',
    images: ['kia1.jpg'],
    description:
      'Lorem, ipsum dolor sit amet consectetur adipisicing elit. Sit quam cupiditate velit fugiat ipsam aut non ipsa sapiente, molestias numquam a quaerat impedit nemo reprehenderit quis dolore maiores quo mollitia!',
  },
];

function Home() {
  useEffect(function () {
    async function fetchData() {
      const res = await fetch('http://localhost:8000/brands');

      const data = await res.json();

      console.log(data);
    }
    fetchData();
  }, []);

  return (
    <>
      <SearchBy />
      <HomePage>
        {cars.map((car) => (
          <Card car={car} key={car.id} />
        ))}
      </HomePage>
    </>
  );
}

export default Home;
