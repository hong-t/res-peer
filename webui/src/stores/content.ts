import { defineStore } from 'pinia'

export interface Content {
  cid: string
  author: string
  content: string
  title: string
  likes: number
  dislikes: number
  accounts: Record<string, boolean>
  createdAt: number
}

export const useContentStore = defineStore('content', {
  state: () => ({
    contentsKeys: [] as Array<string>,
    contents: new Map<string, Content>(),
    mutateKeys: [] as Array<string>
  }),
  getters: {},
  actions: {}
})
