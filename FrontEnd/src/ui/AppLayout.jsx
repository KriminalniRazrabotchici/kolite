import { Outlet } from 'react-router';
import NavBar from './NavBar';

function AppLayout() {
  return (
    <div>
      <NavBar />

      <main>
        <Outlet />
      </main>
    </div>
  );
}

export default AppLayout;
