import {
  Box,
  ChakraProvider,
  Flex,
  IconButton,
  Menu,
  MenuButton,
  MenuItem,
  MenuList,
  Spacer,
  Textarea,
} from '@chakra-ui/react'
import * as React from 'react'
import classNames from 'classnames'
import {
  useTextAreaMarkdownEditor,
  bold,
  italic,
  code,
  codeBlock,
  quote,
  header,
  strikethrough,
  unorderedListCommand,
  orderedListCommand,
  checkedListCommand,
  link,
  image,
} from 'react-mde'
import {
  FaBold,
  FaCheckSquare,
  FaCode,
  FaHeading,
  FaImage,
  FaItalic,
  FaLink,
  FaListOl,
  FaListUl,
  FaQuoteLeft,
  FaStrikethrough,
} from 'react-icons/fa'
import { BiCodeCurly } from 'react-icons/bi'
import { MdDeleteOutline } from 'react-icons/md'

import { HamburgerIcon } from '@chakra-ui/icons'
import './index.scss'

const commandMapConfigs = [
  {
    type: 'bold',
    handle: bold,
    icons: <FaBold />,
    command: '⌘B',
    title: '加粗',
  },
  {
    type: 'italic',
    handle: italic,
    icons: <FaItalic />,
    command: '⌘B',
    title: '斜体',
  },
  {
    type: 'code',
    handle: code,
    icons: <FaCode />,
    command: '⌘B',
    title: '代码段',
  },
  {
    type: 'codeBlock',
    handle: codeBlock,
    icons: <BiCodeCurly />,
    command: '⌘B',
    title: '代码块',
  },
  {
    type: 'quote',
    handle: quote,
    icons: <FaQuoteLeft />,
    command: '⌘B',
    title: '引用',
  },
  {
    type: 'header',
    handle: header,
    icons: <FaHeading />,
    command: '⌘B',
    title: '标题',
  },
  {
    type: 'link',
    handle: link,
    icons: <FaLink />,
    command: '⌘B',
    title: '链接',
  },
  {
    type: 'image',
    handle: image,
    icons: <FaImage />,
    command: '⌘B',
    title: '图片',
  },
  {
    type: 'strikethrough',
    handle: strikethrough,
    icons: <FaStrikethrough />,
    command: '⌘B',
    title: '删除线',
  },
  {
    type: 'unorderedListCommand',
    handle: unorderedListCommand,
    icons: <FaListUl />,
    command: '⌘B',
    title: '无序列表',
  },
  {
    type: 'orderedListCommand',
    handle: orderedListCommand,
    icons: <FaListOl />,
    command: '⌘B',
    title: '有序列表',
  },
  {
    type: 'checkedListCommand',
    handle: checkedListCommand,
    icons: <FaCheckSquare />,
    command: '⌘B',
    title: '代办列表',
  },
]

const commandMap = () => {
  let temp: any = {}
  for (const i of commandMapConfigs) {
    temp[i.type] = i.handle
  }

  return temp
}

const MDEditor: React.FC<any> = (props) => {
  const [focused, setFocused] = React.useState(false)
  const { ref, commandController } = useTextAreaMarkdownEditor({
    commandMap: commandMap(),
  })

  return (
    <div
      className={classNames('mdEditor', {
        'mdEditor-focused': focused,
      })}
    >
      <ChakraProvider>
        <Box p={3} pb={0}>
          <Textarea
            ref={ref}
            placeholder="请输入"
            {...props}
            fontFamily={'monospace'}
            variant="unstyled"
            onBlur={(e) => {
              setFocused(false)
              props?.onBlur(e)
            }}
            onFocus={(e) => {
              setFocused(true)
              props?.onFocus(e)
            }}
          />
          <div className="mdEditor-footer">
            <Flex minWidth="max-content" alignItems="center" gap="2">
              <Box p="2">
                <Menu>
                  <MenuButton>
                    <IconButton
                      variant="outline"
                      aria-label="Options"
                      icon={<HamburgerIcon />}
                      size="sm"
                    />
                  </MenuButton>
                  <MenuList border={'1px solid #eee'}>
                    {commandMapConfigs.map((cof) => {
                      return (
                        <MenuItem
                          key={cof.type}
                          icon={cof.icons}
                          // command={cof.command}
                          onClick={async () => {
                            await commandController.executeCommand(cof.type as any)
                          }}
                        >
                          {cof.title}
                        </MenuItem>
                      )
                    })}
                  </MenuList>
                </Menu>
              </Box>
              <Spacer />
              <IconButton
                size="sm"
                aria-label="删除"
                variant="outline"
                icon={<MdDeleteOutline />}
                onClick={props?.onRemove}
              />
            </Flex>
          </div>
        </Box>
      </ChakraProvider>
    </div>
  )
}

export default MDEditor
