import { invoke } from '@tauri-apps/api/tauri'

export async function fold() {
  return invoke<void>('fold')
}

export async function unfold() {
  return invoke<void>('unfold')
}

export async function isfold() {
  return invoke<boolean>('isfold')
}
