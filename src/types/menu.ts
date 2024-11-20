export enum MenuCommand {
  ADD_DATABASE = 'ADD_DATABASE'
  // 可以继续添加其他命令
}

export interface MenuCommandHandler {
  execute(): void
}

// 命令模式的具体实现类
export class AddDatabaseCommand implements MenuCommandHandler {
  constructor(private showDialog: () => void) {}
  
  execute(): void {
    this.showDialog()
  }
}

// 菜单项接口
export interface MenuItem {
  id: string
  label: string
  handler: () => void
} 