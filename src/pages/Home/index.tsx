/* eslint-disable react-hooks/exhaustive-deps */
// Home/index.ts
import React, { useEffect, useMemo, useState } from 'react'
import {
  ChakraProvider,
  IconButton,
  Input,
  InputGroup,
  InputLeftAddon,
  InputRightAddon,
  Menu,
  MenuButton,
  MenuItemOption,
  MenuList,
  MenuOptionGroup,
  Textarea,
} from '@chakra-ui/react'
import { AddIcon, ChevronDownIcon, DeleteIcon } from '@chakra-ui/icons'
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
                <div className="home_editor">
                  <Textarea
                    autoFocus
                    placeholder="请输入"
                    border="none"
                    borderBottom="1px"
                    key={i.uid}
                    defaultValue={i.content}
                    onBlur={async (e) => {
                      await updatePost(i.uid, { content: e.target.value.trim() })
                      await getPostsList()
                    }}
                  ></Textarea>
                  <div className="home_editor_toolbar">
                    <div></div>
                    <div>
                      <IconButton
                        size="sm"
                        aria-label="删除"
                        icon={<DeleteIcon />}
                        onClick={async () => {
                          await deletePost(i.uid)
                          await getPostsList()
                        }}
                      />
                    </div>
                  </div>
                </div>
                // <MDEditor
                //   key={i.uid}
                //   value={i.content}
                //   onChange={async (value: any) => {
                //     await updatePost(i.uid, { content: value })
                //     await getPostsList()
                //     console.log(value)
                //   }}
                // />
              )
            })}
          </div>
        </div>
      </HomeContext.Provider>
    </ChakraProvider>
  )
}

export default Home
