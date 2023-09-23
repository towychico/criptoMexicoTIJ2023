
import { Account } from './account';
import "./Header.css"

import {LocalBalanceToken} from "../LocalBalance/LocalBalanceToken";

type Props = {
  isAccountVisible: boolean;
};

function Header({ isAccountVisible }: Props) {
  return (
    <header className='headerContainer'>

      {isAccountVisible && <Account />}
      <div className="logoTextSerenium" />
 {/*        eslint-disable-next-line react/button-has-type */}
       <LocalBalanceToken/>
        <button className="dropdownButtonMenu" type="button"> </button>
    </header>
  );
}

export { Header };
