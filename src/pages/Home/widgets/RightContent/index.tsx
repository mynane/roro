// RightContent/index.ts
import React from 'react'
import './index.csss'

export interface IRightContentProps {
  [key: string]: any
}

const RightContent: React.FC<IRightContentProps> = (props) => {
  const {} = props
  return <div className={'right-content'}>RightContent</div>
}

export default RightContent
