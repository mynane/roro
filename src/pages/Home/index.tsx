// Home/index.ts
import React from 'react'
import { HomeContext, useHomeState } from './model'
// import LeftContent from './widgets/LeftContent'
import RightContent from './widgets/RightContent'
import './index.scss'

export interface IHomeProps {
  [key: string]: any
}

const Home: React.FC<IHomeProps> = (props) => {
  const [states, funs] = useHomeState()
  console.log(13123)
  return (
    <HomeContext.Provider value={[states, funs]}>
      <div className="home">
        {/* <LeftContent /> */}
        <RightContent
          value={
            '{"root":{"children":[{"children":[{"detail":0,"format":0,"mode":"normal","style":"","text":"1231221321312313","type":"text","version":1}],"direction":null,"format":"","indent":0,"type":"paragraph","version":1},{"children":[],"direction":null,"format":"","indent":0,"type":"paragraph","version":1},{"children":[],"direction":null,"format":"","indent":0,"type":"paragraph","version":1},{"children":[{"detail":0,"format":0,"mode":"normal","style":"","text":"1233","type":"text","version":1}],"direction":null,"format":"justify","indent":0,"type":"paragraph","version":1},{"children":[{"detail":0,"format":0,"mode":"normal","style":"","text":"12312312","type":"text","version":1}],"direction":null,"format":"left","indent":0,"type":"paragraph","version":1}],"direction":null,"format":"","indent":0,"type":"root","version":1}}'
          }
        />
      </div>
    </HomeContext.Provider>
  )
}

export default Home
