
import './Post.css';

import React, { useState } from 'react';
import { IonCard, IonCardContent, IonCardHeader, IonCardSubtitle, IonCardTitle, IonIcon } from '@ionic/react';
import { thumbsUpOutline, thumbsUpSharp } from 'ionicons/icons';


function Post() {
    const [liked, setLiked] = useState(false);

    const toggleLike = () => {
        setLiked(!liked);
    };

    return (
        <IonCard className="post-card">
            <IonCardHeader>
                <IonCardTitle>Publicación</IonCardTitle>
            </IonCardHeader>
            <img alt="Silhouette of mountains" src="https://ionicframework.com/docs/img/demos/thumbnail.svg" />
            <IonCardContent>
                <p>Este es el texto de ejemplo de una publicación.</p>
                <button onClick={toggleLike} className="like-button">
                    <IonIcon icon={liked ? thumbsUpSharp : thumbsUpOutline} />
                    {liked ? 'Unlike' : 'Like'}
                </button>
            </IonCardContent>
        </IonCard>
    );
}

export default Post;
