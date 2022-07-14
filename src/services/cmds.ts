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

export async function getCatalogues() {
  return invoke<void>('get_catalogues')
}

export async function syncCatalogues() {
  return invoke<void>('sync_catalogues')
}

export async function createCatalogue(item: any) {
  return invoke<void>('create_catalogue', { item })
}

export async function updateCatalogue(uid: string, item: any) {
  return invoke<void>('update_catalogue', { uid, item })
}

export async function deleteCatalogue(uid: string) {
  return invoke<void>('delete_catalogue', { uid })
}

export async function putCurrent(uid: string) {
  return invoke<void>('put_current', { uid })
}

export async function getCurrent() {
  return invoke<void>('get_current')
}

export async function getPosts() {
  return invoke<void>('get_posts')
}

export async function syncPosts() {
  return invoke<void>('sync_posts')
}

export async function createPost(item: any) {
  return invoke<void>('create_post', { item })
}

export async function updatePost(uid: string, item: any) {
  return invoke<void>('update_post', { uid, item })
}

export async function deletePost(uid: string) {
  return invoke<void>('delete_post', { uid })
}

export async function deleteItemsByCuid(cuid: string) {
  return invoke<void>('delete_items_by_cuid', { cuid })
}

export async function getItemsByCuid(cuid: string) {
  return invoke<void>('get_items_by_cuid', { cuid })
}
