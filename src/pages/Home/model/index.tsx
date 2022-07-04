// Home/model/index.ts
import { createContext, useEffect, useReducer } from 'react'
import { listen, UnlistenFn } from '@tauri-apps/api/event'
import { readDir } from '@tauri-apps/api/fs'

type IHomeAction = {
  [key: string]: any
}

type THomeOutFn = {}

const HomeInitialSate = {}

type IHomeState = typeof HomeInitialSate

const unmount: UnlistenFn[] = []

export const useHomeState = (): [IHomeState, THomeOutFn] => {
  const [state] = useReducer(
    (ostate: IHomeState, action: IHomeAction) => {
      return { ...ostate, ...action }
    },
    {
      ...HomeInitialSate,
    }
  )

  const initEvent = async () => {
    const unlisten = await listen('on-target-change', async (event) => {
      const { message }: any = event.payload
      const entries = await readDir(message)
      console.log('🚀 ~ file: index.tsx ~ line 32 ~ unlisten ~ entries', entries)
      // emit('client-message', { message })
    })

    return unlisten
  }

  useEffect(() => {
    console.log(123123)
    initEvent()

    return () => {
      console.log(unmount, '====')
      unmount.forEach((item) => item())
    }
  }, [])

  return [state, {}]
}

export const HomeContext = createContext<[IHomeState, Partial<THomeOutFn>]>([HomeInitialSate, {}])
