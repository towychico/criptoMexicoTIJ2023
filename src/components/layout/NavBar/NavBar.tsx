import react from "react"

import './navBar.css'

function NavBar() {
  return <div className="navBar" >
      {/* eslint-disable-next-line jsx-a11y/control-has-associated-label */}
      < button type='button'  id="navBtn1" className="navBarButton"/>
      {/* eslint-disable-next-line jsx-a11y/control-has-associated-label */}
      < button type='button'  id="navBtn2" className="navBarButton"/>
      {/* eslint-disable-next-line jsx-a11y/control-has-associated-label */}
      < button type='button'  id="navBtn3" className="navBarButton"/>
      {/* eslint-disable-next-line jsx-a11y/control-has-associated-label */}
      < button type='button'  id="navBtn4" className="navBarButton"/>
      {/* eslint-disable-next-line jsx-a11y/control-has-associated-label */}
      < button type='button'  id="navBtn5" className="navBarButton"/>
  </div>
}
// eslint-disable-next-line import/no-default-export
export default NavBar;
