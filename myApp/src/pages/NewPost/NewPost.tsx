import React, { useState } from "react";
import {
    IonButton,
    IonContent,
    IonHeader,
    IonItem,
    IonPage,
    IonRadio,
    IonRadioGroup,
    IonTextarea,
    IonTitle,
    IonToolbar,
} from "@ionic/react";

const NewPost: React.FC = () => {
    const [type, setType] = useState(""); // Initialize with an empty string
    const [content, setContent] = useState("");

    const handleRadioChange = (event: CustomEvent) => {
        setType(event.detail.value);
    };

    const handleTextareaChange = (event: CustomEvent) => {
        setContent(event.detail.value || ""); // Ensure content is not undefined or null
    };

    return (
        <IonPage>
            <IonHeader>
                <IonToolbar>
                    <IonTitle>New Post</IonTitle>
                </IonToolbar>
            </IonHeader>
            <IonContent fullscreen>
                <IonRadioGroup value={type} onIonChange={handleRadioChange}>
                    <IonItem>
                        <IonRadio value="grapes">Question</IonRadio>
                    </IonItem>
                    <IonItem>
                        <IonRadio value="strawberries">Challenge</IonRadio>
                    </IonItem>
                </IonRadioGroup>
                <IonItem>
                    <IonTextarea
                        label="Speak to the world"
                        placeholder="Type something here"
                        labelPlacement="stacked"
                        value={content}
                        onIonChange={handleTextareaChange}
                    ></IonTextarea>
                </IonItem>
                <IonButton>Publish</IonButton>
            </IonContent>
        </IonPage>
    );
};

export default NewPost;
