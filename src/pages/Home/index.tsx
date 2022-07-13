// Home/index.ts
import React from 'react'
import {
  ChakraProvider,
  Input,
  InputGroup,
  InputLeftAddon,
  InputRightAddon,
  Menu,
  MenuButton,
  MenuDivider,
  MenuItem,
  MenuList,
} from '@chakra-ui/react'
import { AddIcon, ChevronDownIcon } from '@chakra-ui/icons'
import MDEditor from '../../components/MDEditor'
import { HomeContext, useHomeState } from './model'
import './index.scss'
import { fold, isfold, unfold } from '../../services/cmds'

export interface IHomeProps {
  [key: string]: any
}

const Home: React.FC<IHomeProps> = (props) => {
  const [states, funs] = useHomeState()
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
                    <MenuItem>New File</MenuItem>
                    <MenuItem>New Window</MenuItem>
                    <MenuDivider />
                    <MenuItem>Open...</MenuItem>
                    <MenuItem>Save File</MenuItem>
                  </MenuList>
                </Menu>
              </InputLeftAddon>
              <Input placeholder="搜索" />
              <InputRightAddon>
                <AddIcon />
              </InputRightAddon>
            </InputGroup>
          </div>
          <div
            className="home_indicator"
            onClick={async () => {
              const result: boolean = await isfold()
              if (result) {
                await unfold()
              } else {
                await fold()
              }
            }}
          ></div>
          <div className="home_content">
            {new Array(100).fill(1).map((i, index) => {
              return <MDEditor key={index} />
            })}
          </div>
        </div>
      </HomeContext.Provider>
    </ChakraProvider>
  )
}

export default Home
