// Home/index.ts
import React from 'react'
import { HomeContext, useHomeState } from './model'
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
        <div className="home_indicator"></div>
        <div className="home_resize"></div>
        <div className="home_content">1231231</div>
      </div>
    </HomeContext.Provider>
  )
}

export default Home
