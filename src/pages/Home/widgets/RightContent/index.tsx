// RightContent/index.ts
import React from 'react'
import RoroEditor from '../../../../components/RoroEditor'
import './index.csss'

export interface IRightContentProps {
  value: string
  [key: string]: any
}

const RightContent: React.FC<IRightContentProps> = ({ value }) => {
  return (
    <div className={'right-content'}>
      <RoroEditor value={value} />
    </div>
  )
}

export default RightContent
