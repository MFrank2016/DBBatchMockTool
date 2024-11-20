import { defineStore } from 'pinia'
import { MenuCommand, MenuCommandHandler } from '../types/menu'

export const useMenuStore = defineStore('menu', {
  state: () => ({
    commands: new Map<MenuCommand, MenuCommandHandler>()
  }),
  
  actions: {
    registerCommand(command: MenuCommand, handler: MenuCommandHandler) {
      this.commands.set(command, handler)
    },
    
    executeCommand(command: MenuCommand) {
      const handler = this.commands.get(command)
      if (handler) {
        handler.execute()
      }
    }
  }
}) 