export async function getBrands() {
  const res = await fetch('http://localhost:8000/brands');

  if (!res.ok) {
    console.error(res.error);
    throw new Error('Brands could not be loaded!');
  }

  const data = await res.json();

  return data;
}

export async function getModels() {
  const res = await fetch('http://localhost:8000/models');

  if (!res.ok) {
    console.error(res.error);
    throw new Error('Models could not be loaded!');
  }

  const data = await res.json();

  return data;
}

export async function getCoupes() {
  const res = await fetch('http://localhost:8000/coupes');

  if (!res.ok) {
    console.error(res.error);
    throw new Error('Coupes could not be loaded!');
  }

  const data = await res.json();

  return data;
}

export async function getFuel() {
  const res = await fetch('http://localhost:8000/fuel');

  if (!res.ok) {
    console.error(res.error);
    throw new Error('Fuel types could not be loaded!');
  }

  const data = await res.json();

  return data;
}

export async function getWheel() {
  const res = await fetch('http://localhost:8000/wheel');

  if (!res.ok) {
    console.error(res.error);
    throw new Error('Wheel type could not be loaded!');
  }

  const data = await res.json();

  return data;
}

export async function getExtras() {
  const res = await fetch('http://localhost:8000/extras');

  if (!res.ok) {
    console.error(res.error);
    throw new Error('Extras could not be loaded!');
  }

  const data = await res.json();

  return data;
}

export async function getDoors() {
  const res = await fetch('http://localhost:8000/doors');

  if (!res.ok) {
    console.error(res.error);
    throw new Error('Doors could not be loaded!');
  }

  const data = await res.json();

  return data;
}

export async function getColors() {
  const res = await fetch('http://localhost:8000/colors');

  if (!res.ok) {
    console.error(res.error);
    throw new Error('Colors could not be loaded!');
  }

  const data = await res.json();

  return data;
}

export async function getCities() {
  const res = await fetch('http://localhost:8000/cities');

  if (!res.ok) {
    console.error(res.error);
    throw new Error('Cities could not be loaded!');
  }

  const data = await res.json();

  return data;
}

export async function getTransmission() {
  const res = await fetch('http://localhost:8000/transmission');

  if (!res.ok) {
    console.error(res.error);
    throw new Error('Transmission type could not be loaded!');
  }

  const data = await res.json();

  return data;
}
