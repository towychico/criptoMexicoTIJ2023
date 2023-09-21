import {IonContent, IonHeader, IonItem, IonPage, IonTitle, IonToolbar} from '@ionic/react';
import ExploreContainer from '../../components/ExploreContainer/ExploreContainer';
import './Home.css';
import NewPostBtn from "../../components/NewPostBtn/NewPostBtn";

const Home: React.FC = () => {
  return (
    <IonPage>
      <IonHeader>
        <IonToolbar>
          <IonTitle>Serenium</IonTitle>
        </IonToolbar>
      </IonHeader>
      <IonContent fullscreen>
        <IonHeader collapse="condense">
          <IonToolbar>
            <IonTitle size="large">Serenium</IonTitle>
          </IonToolbar>
        </IonHeader>
        <ExploreContainer />
          <NewPostBtn/>
      </IonContent>
    </IonPage>
  );
};

export default Home;
