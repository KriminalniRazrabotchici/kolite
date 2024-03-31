export async function getCabins() {
  const res = await fetch('http://localhost:8000/brands');

  if (!res.ok) {
    console.error(res.error);
    throw new Error('Cars could not be loaded!');
  }

  const data = await res.json();

  return data;
}
