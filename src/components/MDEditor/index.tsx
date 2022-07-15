import {
  Box,
  ChakraProvider,
  Flex,
  HStack,
  IconButton,
  Menu,
  MenuButton,
  MenuItem,
  MenuList,
  Modal,
  ModalBody,
  ModalCloseButton,
  ModalContent,
  ModalOverlay,
  Spacer,
  Textarea,
  useDisclosure,
} from '@chakra-ui/react'
import ResizeTextarea from 'react-textarea-autosize'
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
import { FiSettings } from 'react-icons/fi'
import { IoMdColorFilter } from 'react-icons/io'

import { HamburgerIcon } from '@chakra-ui/icons'
import { SketchPicker } from 'react-color'
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
  const { backgroundColor = '#fff' } = props
  const [focused, setFocused] = React.useState(false)
  const { isOpen, onOpen, onClose } = useDisclosure()
  const { ref, commandController } = useTextAreaMarkdownEditor({
    commandMap: commandMap(),
  })

  return (
    <div
      className={classNames('mdEditor', {
        'mdEditor-focused': focused,
      })}
      style={{ backgroundColor }}
    >
      <ChakraProvider>
        <Box p={3} pb={0}>
          <Textarea
            ref={ref}
            minH="unset"
            overflow="auto"
            w="100%"
            resize="none"
            minRows={3}
            maxRows={20}
            as={ResizeTextarea}
            placeholder="请输入"
            {...props}
            fontFamily={'monospace'}
            variant="unstyled"
            onBlur={(e) => {
              setFocused(false)
              props?.onBlur && props?.onBlur(e)
            }}
            onFocus={(e) => {
              setFocused(true)
              props?.onFocus && props?.onFocus(e)
            }}
          />
          <div className="mdEditor-footer">
            <Flex minWidth="max-content" alignItems="center" gap="2">
              <HStack spacing="10px">
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
                <Menu>
                  <MenuButton>
                    <IconButton
                      variant="outline"
                      aria-label="Options"
                      icon={<FiSettings />}
                      size="sm"
                    />
                  </MenuButton>
                  <MenuList border={'1px solid #eee'}>
                    <MenuItem
                      icon={<IoMdColorFilter />}
                      onClick={async () => {
                        onOpen()
                      }}
                    >
                      背景色
                    </MenuItem>
                  </MenuList>
                </Menu>
              </HStack>
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

      <Modal isOpen={isOpen} onClose={onClose}>
        <ModalOverlay />
        <ModalContent width={'initial'}>
          <ModalCloseButton />
          <ModalBody width={'initial'} padding={0}>
            <SketchPicker
              disableAlpha
              color={backgroundColor}
              onChange={(color) => {
                props?.onBgColorChange(color.hex)
              }}
            />
          </ModalBody>
        </ModalContent>
      </Modal>
    </div>
  )
}

export default MDEditor
