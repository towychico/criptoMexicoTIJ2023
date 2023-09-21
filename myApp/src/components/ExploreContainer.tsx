import './ExploreContainer.css';
import Post from './Post.jsx';


import React from 'react';
import { IonContent, IonPage } from '@ionic/react';

function MyPage() {

    const postsArray = Array.from({ length: 20 }, (_, index) => <Post key={index} />);

    return (
        <IonPage>
            <IonContent>
                {postsArray}
            </IonContent>
        </IonPage>
    );
}

export default MyPage;
