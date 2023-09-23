import react from "react"
import { setupIonicReact } from '@ionic/react';
import './home.css'
import NavBar from "../../components/layout/NavBar/NavBar";
import PostContainer from "../../components/layout/PostContainer/PostContainer";
import '@ionic/react/css/core.css';

setupIonicReact();
function Home() {
 

  // @ts-ignore
  // @ts-ignore
  return (
    <div className='homeScreen'>

    <PostContainer/>
    <NavBar />
    </div>
  )
}

export { Home };
