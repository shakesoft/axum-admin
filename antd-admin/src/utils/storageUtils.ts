const USER_TOKEN = 'token'
const USER_TOKEN_EXPIRES = 'expires_at'
const REFRESH_TOKEN = 'refresh_token'
const REFRESH_TOKEN_EXPIRES = 'refresh_expires_at'
const USER_Name = 'name'
const BTN_MENU = 'btnMenu'
const TREE_MENU = 'treeMenu'
/*
包含n 个操作local storage 的工具函数的模块
*/
export const storageUtils = {

    saveToken(token: string) {
        localStorage.setItem(USER_TOKEN, token)
    },
    getToken() {
        return localStorage.getItem(USER_TOKEN)
    },
    saveExpiresAt(at: number) {
        localStorage.setItem(USER_TOKEN_EXPIRES, at.toString())
    },
    getExpiresAt() {
        return localStorage.getItem(USER_TOKEN_EXPIRES)
    },
    saveRefreshToken(token: string, at: number) {
        localStorage.setItem(REFRESH_TOKEN, token)
        localStorage.setItem(REFRESH_TOKEN_EXPIRES, at.toString())
    },
    getRefreshToken() {
        return localStorage.getItem(REFRESH_TOKEN)
    },
    getRefreshExpiresAt() {
        return localStorage.getItem(REFRESH_TOKEN_EXPIRES)
    },

    saveUserName(userName: string) {
        localStorage.setItem(USER_Name, userName)
    },
    getUserName(): string {
        return localStorage.getItem(USER_Name) || '一个游客'
    },

    saveBtnMenu(btnMenu: string[]) {
        localStorage.setItem(BTN_MENU, JSON.stringify(btnMenu))
    },
    getBtnMenu(): string {
        const btnMenu = localStorage.getItem(BTN_MENU);
        return btnMenu != null ? JSON.parse(btnMenu) : [];
    },

    saveTreeMenu(treeMenu: string[]) {
        localStorage.setItem(TREE_MENU, JSON.stringify(treeMenu))
    },
    getTreeMenu(): string[] {
        const treeMenu = localStorage.getItem(TREE_MENU);
        return treeMenu != null ? JSON.parse(treeMenu) : [];
    },

    logout() {
        localStorage.removeItem(USER_TOKEN)
        localStorage.removeItem(USER_TOKEN_EXPIRES)
        localStorage.removeItem(REFRESH_TOKEN)
        localStorage.removeItem(REFRESH_TOKEN_EXPIRES)
        localStorage.removeItem(USER_Name)
        localStorage.removeItem(BTN_MENU)
        localStorage.removeItem(TREE_MENU)
        localStorage.removeItem('openKeys')
        localStorage.removeItem('selectedKeys')
        localStorage.removeItem('breadcrumb')
    }
}
