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
        <Modal onClose={() => setIsOpenModal(false)}>
          <Login onCloseModal={() => setIsOpenModal(false)} />
        </Modal>
      )}
    </div>
  );
}

export default OpenLogin;
