import { useEffect } from 'react'
import { useLexicalComposerContext } from '@lexical/react/LexicalComposerContext'

export default function OnValueChangePlugin({ value }: { value?: string }) {
  const [editor] = useLexicalComposerContext()

  useEffect(() => {
    if (value && editor) {
      editor.parseEditorState(value)
    }
  }, [editor, value])

  return null
}
