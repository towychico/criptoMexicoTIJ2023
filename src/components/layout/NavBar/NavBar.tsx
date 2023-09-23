import react, {useState} from "react"

import './navBar.css'
import {GearApi,GearKeyring} from "@gear-js/api";

import {AlertInput, IonAlert, IonButton} from '@ionic/react';
import CustomAlert from "../CustomAlert/CustomAlert";

const submitContract = async function (content:string) {
    const gearApi = await GearApi.create({
        providerAddress: 'wss://vit.vara-network.io',
    });
    const keyring = await GearKeyring.fromSuri('//Alice');

    // Replace the 'fs.readFileSync' call with browser-friendly file loading
    // Example using fetch to load a file:
    try {
        const response = await fetch('../../../contracts_wasm/thread.wasm');
        if (!response.ok) {
            throw new Error("Failed to fetch the file.");
        }
        const arrayBuffer = await response.arrayBuffer();
        const code = new Uint8Array(arrayBuffer);

        const somePayload = {
            "id": "1",
            "owner": "100",
            "postType": "Challenge",
            "content": content,
            "replies": "sdsfdsfdsf",
            "state": "Expired"
        }

        const program = {
            code,
            gasLimit: 1000000,
            value: 1000000000000000,
            initPayload: somePayload,
        };

        try {
            const { programId, codeId, salt, extrinsic } = gearApi.program.upload(
                program
            );

            try {
                await extrinsic.signAndSend(keyring, (event: { toHuman: () => any; }) => {
                    console.log(event.toHuman());
                });
            } catch (error) {
                // @ts-ignore
                console.error(`${error.name}: ${error.message}`);
            }
        } catch (error) {
            // @ts-ignore
            console.error(`${error.name}: ${error.message}`);
        }
    } catch (error) {
        // Handle file loading error
        // @ts-ignore
        console.error(`${error.name}: ${error.message}`);
    }
}
function createPost(){

}


function NavBar() {
    const [showAlert, setShowAlert] = useState(false);

    const handleShowAlert = () => {
        setShowAlert(true);
    };

    const handleConfirm = (input1: string, input2: string) => {
        submitContract(input2)

        console.log('Input 1:', input1);
        console.log('Input 2:', input2);
        // Handle the input values as needed
    };
    // eslint-disable-next-line react/function-component-definition


        // @ts-ignore
        // @ts-ignore
        // @ts-ignore
        // @ts-ignore
        return (<div className="navBar">

            {/* eslint-disable-next-line jsx-a11y/control-has-associated-label */}
            < button type='button' id="navBtn1" className="navBarButton"/>
            {/* eslint-disable-next-line jsx-a11y/control-has-associated-label */}
            < button type='button' id="navBtn2" className="navBarButton"/>
            {/* eslint-disable-next-line jsx-a11y/control-has-associated-label */}
            < button type='button' onClick={handleShowAlert} id="navBtn3" className="navBarButton"/>
            <CustomAlert isOpen={showAlert} onClose={() => setShowAlert(false)} onConfirm={handleConfirm} />



            {/* eslint-disable-next-line jsx-a11y/control-has-associated-label */}
            < button type='button' id="navBtn4" className="navBarButton"/>
            {/* eslint-disable-next-line jsx-a11y/control-has-associated-label */}
            < button type='button' id="navBtn5" className="navBarButton"/>
        </div>)

}

// eslint-disable-next-line import/no-default-export
export default NavBar;
