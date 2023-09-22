import { useApi, useAccount } from '@gear-js/react-hooks';
import { Routing } from 'pages';
import { Header, ApiLoader } from 'components';
import {IonButton,
    setupIonicReact} from '@ionic/react';
import { withProviders } from 'hocs';
import 'App.css';

setupIonicReact();
function Component() {
  const { isApiReady } = useApi();
  const { isAccountReady } = useAccount();

  const isAppReady = isApiReady && isAccountReady;

  return (
    <>
      <Header isAccountVisible={isAccountReady} />
        <IonButton>Code sex and crypto</IonButton>
      <main>{isAppReady ? <Routing /> : <ApiLoader />}</main>
      
    </>
  );
}

export const App = withProviders(Component);
