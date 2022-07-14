/* eslint-disable react-hooks/exhaustive-deps */
// Home/index.ts
import React, { useEffect, useState } from 'react'
import {
  ChakraProvider,
  Input,
  InputGroup,
  InputLeftAddon,
  InputRightAddon,
  Menu,
  MenuButton,
  MenuItem,
  MenuList,
  Textarea,
} from '@chakra-ui/react'
import { AddIcon, ChevronDownIcon } from '@chakra-ui/icons'
import MDEditor from '../../components/MDEditor'
import { HomeContext, useHomeState } from './model'
import './index.scss'
import {
  createPost,
  fold,
  getCatalogues,
  getItemsByCuid,
  isfold,
  putCurrent,
  unfold,
  updatePost,
} from '../../services/cmds'

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
      if (list.current) {
        getPostsList()
      }
    } catch (error) {}
  }

  useEffect(() => {
    getCataloguesLists()
  }, [])

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
                    File <ChevronDownIcon />
                  </MenuButton>
                  <MenuList>
                    {catas.items.map((cata: any) => {
                      return (
                        <MenuItem
                          key={cata.uid}
                          onClick={async () => {
                            await putCurrent(cata.uid)
                            await getCataloguesLists()
                          }}
                        >
                          {cata.name}
                        </MenuItem>
                      )
                    })}
                  </MenuList>
                </Menu>
              </InputLeftAddon>
              <Input placeholder="搜索" />
              <InputRightAddon>
                <AddIcon
                  onClick={async () => {
                    await createPost({ cuid: catas.current })
                    console.log(catas.cuid)
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
                    defaultValue={i.content}
                    onBlur={async (e) => {
                      await updatePost(i.uid, { content: e.target.value.trim() })
                      await getPostsList()
                    }}
                  ></Textarea>
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
