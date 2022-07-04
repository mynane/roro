// Home/index.ts
import React from 'react'
import './index.scss'
import { HomeContext, useHomeState } from './model'
import LeftContent from './widgets/LeftContent'
import RightContent from './widgets/RightContent'

export interface IHomeProps {
  [key: string]: any
}

const Home: React.FC<IHomeProps> = (props) => {
  const [states, funs] = useHomeState()
  console.log(13123)
  return (
    <HomeContext.Provider value={[states, funs]}>
      <div className="home">
        <LeftContent />
        <RightContent />
      </div>
    </HomeContext.Provider>
  )
}

export default Home
