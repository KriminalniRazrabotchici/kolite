export async function login({ email, password }) {
  const response = await fetch('localhost:8000/user/login', {
    method: 'POST',
    body: JSON.stringify({ email, password }),
    headers: { 'Content-Type': 'application/json' },
  });

  if (!response.ok) {
    throw new Error(response.message);
  }

  console.log(response);
  return response;
}

export async function register({ name, email, number, password }) {
  const response = await fetch('localhost:8000/user/register', {
    method: 'POST',
    body: JSON.stringify({ name, email, password }),
    headers: { 'Content-Type': 'application/json' },
  });

  if (!response.ok) {
    throw new Error(response.message);
  }

  return response;
}
