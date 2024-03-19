import { useState } from 'react';
import { ButtonLink } from '../../ui/ButtonNav';
import Modal from '../../ui/Modal';
import Login from './LoginForm';

function OpenLogin() {
  const [isOpenModal, setIsOpenModal] = useState(false);

  return (
    <div>
      <ButtonLink onClick={() => setIsOpenModal((show) => !show)}>
        Login
      </ButtonLink>
      {isOpenModal && (
        <Modal>
          <Login />
        </Modal>
      )}
    </div>
  );
}

export default OpenLogin;
