// LeftContent/index.ts
import React from 'react'
import './index.scss'

export interface ILeftContentProps {
  [key: string]: any
}

const LeftContent: React.FC<ILeftContentProps> = (props) => {
  const {} = props
  return <div className={'left-content'}>leftContent</div>
}

export default LeftContent
