import React from 'react';
import { IonFab, IonFabButton, IonIcon } from '@ionic/react';
import { add } from 'ionicons/icons';

function NewPostBtn() {
    return (
        <IonFab>
            <IonFabButton routerLink={"/new-post"}>
                <IonIcon icon={add}></IonIcon>
            </IonFabButton>
        </IonFab>
    );
}
export default NewPostBtn;