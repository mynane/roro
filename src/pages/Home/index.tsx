/* eslint-disable react-hooks/exhaustive-deps */
// Home/index.ts
import React, { useEffect, useMemo, useState } from 'react'
import {
  ChakraProvider,
  Input,
  InputGroup,
  InputLeftAddon,
  InputRightAddon,
  Menu,
  MenuButton,
  MenuItemOption,
  MenuList,
  MenuOptionGroup,
} from '@chakra-ui/react'
import { AddIcon, ChevronDownIcon } from '@chakra-ui/icons'
import {
  isPermissionGranted,
  requestPermission,
  sendNotification,
} from '@tauri-apps/api/notification'
import { HomeContext, useHomeState } from './model'
import {
  createPost,
  deletePost,
  fold,
  getCatalogues,
  getItemsByCuid,
  isfold,
  putCurrent,
  unfold,
  updatePost,
} from '../../services/cmds'
import './index.scss'
import MDEditor from '../../components/MDEditor'

export interface IHomeProps {
  [key: string]: any
}

const Home: React.FC<IHomeProps> = (props) => {
  const [states, funs] = useHomeState()
  const [lists, setLists]: any = useState([])
  const [catas, setCatas]: any = useState({ current: null, items: [] })

  const getPostsList = async () => {
    if (!catas.current) {
      return
    }
    try {
      const list: any = await getItemsByCuid(catas.current)
      setLists(list)
    } catch (error) {
      console.log(error)
    }
  }

  const getCataloguesLists = async () => {
    try {
      const list: any = await getCatalogues()
      setCatas(list)
    } catch (error) {}
  }

  useEffect(() => {
    getCataloguesLists()
    // ;(async () => {
    //   await register('CommandOrControl+Shift+C', async () => {
    //     const result: boolean = await isfold()
    //     if (result) {
    //       await unfold()
    //     } else {
    //       await fold()
    //     }
    //   })
    //   await register('CommandOrControl+Shift+M', async () => {
    //     const result: boolean = await isfold()
    //     if (result) {
    //       await unfold()
    //     }
    //     console.log(catas)
    //     await createPost({ cuid: catas.current })
    //     await getPostsList()
    //   })
    // })()

    // return () => {
    //   unregister('CommandOrControl+Shift+C')
    //   unregister('CommandOrControl+Shift+M')
    // }
  }, [])

  useEffect(() => {
    if (catas.current) {
      getPostsList()
    }
  }, [catas.current])

  const current = useMemo(() => {
    return catas?.items?.filter((item: any) => item.uid === catas.current)?.[0]
  }, [catas])

  return (
    <ChakraProvider>
      <HomeContext.Provider value={[states, funs]}>
        <div className="home">
          <div className="search">
            <InputGroup size="lg">
              <InputLeftAddon>
                <Menu>
                  <MenuButton
                    px={4}
                    py={2}
                    transition="all 0.2s"
                    borderRadius="md"
                    borderWidth="1px"
                    _hover={{ bg: 'gray.400' }}
                    _expanded={{ bg: 'blue.400' }}
                    _focus={{ boxShadow: 'outline' }}
                  >
                    {current?.name} <ChevronDownIcon />
                  </MenuButton>
                  <MenuList>
                    <MenuOptionGroup
                      value={catas.current}
                      onChange={async (uid) => {
                        await putCurrent(uid as string)
                        await getCataloguesLists()
                      }}
                      title="目录"
                      type="radio"
                    >
                      {catas.items.map((cata: any) => {
                        return <MenuItemOption value={cata.uid}>{cata.name}</MenuItemOption>
                      })}
                    </MenuOptionGroup>
                  </MenuList>
                </Menu>
              </InputLeftAddon>
              <Input placeholder="搜索" />
              <InputRightAddon>
                <AddIcon
                  onClick={async () => {
                    let permissionGranted = await isPermissionGranted()

                    if (!permissionGranted) {
                      const permission = await requestPermission()
                      permissionGranted = permission === 'granted'
                    }

                    if (permissionGranted) {
                      sendNotification({ title: 'TAURI', body: 'Tauri is awesome!' })
                    }
                    console.log(
                      '🚀 ~ file: index.tsx ~ line 143 ~ onClick={ ~ permission',
                      permissionGranted
                    )
                    await createPost({ cuid: catas.current })
                    await getPostsList()
                  }}
                />
              </InputRightAddon>
            </InputGroup>
          </div>
          <div
            className="home_indicator"
            onClick={async () => {
              // const result1 = await syncCatalogues()
              // const result2 = await createCatalogue({ name: '123123' })
              // const result3 = await updateCatalogue('cizHJaCc92qM3', { name: '宝贝我爱你1' })
              // const result4 = await deleteCatalogue('cizHJaCc92qM3')
              // ciciXDfav8L8c

              // const result2 = await createPost({ content: '123123', cuid: 'ciciXDfav8L8c' })

              // await updatePost('pimKgcKsgCz9n', { content: 'shijinhua' })
              // const result = await getPosts()

              // console.log(result)
              const result: boolean = await isfold()
              if (result) {
                await unfold()
              } else {
                await fold()
              }
            }}
          ></div>
          <div className="home_content">
            {lists.map((i: any) => {
              return (
                <MDEditor
                  key={i.uid}
                  defaultProps={i.content}
                  onBlur={async (e: any) => {
                    await updatePost(i.uid, { content: e.target.value.trim() })
                    await getPostsList()
                  }}
                  onRemove={async () => {
                    await deletePost(i.uid)
                    await getPostsList()
                  }}
                />
              )
            })}
          </div>
        </div>
      </HomeContext.Provider>
    </ChakraProvider>
  )
}

export default Home
