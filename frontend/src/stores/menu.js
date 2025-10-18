import { defineStore } from 'pinia'
import cssData from '@/styles/cssData.module.scss'
import Cookies from 'js-cookie'


export const useMenuStore = defineStore('menu', {
    state: () => ({
        cssData: cssData,

        opened: Cookies.get('sidebarStatus') === '1',
        withoutAnimation: false,

        tagsViewList: Cookies.get('tagsViewList') ? JSON.parse(Cookies.get('tagsViewList')) : [],
    }),
    actions: {
        toggleOpened() {
            this.opened = !this.opened
            this.withoutAnimation = false
            if (this.opened) {
                Cookies.set('sidebarStatus', 1)
            } else {
                Cookies.set('sidebarStatus', 0)
            }
        },
        closeSidebar(withoutAnimation) {
            this.opened = false
            this.withoutAnimation = withoutAnimation
        },

        addTagView(tag) {
            if (this.tagsViewList.some(v => v.path === tag.path)) return
            this.tagsViewList.push(tag)
            Cookies.set('tagsViewList', JSON.stringify(this.tagsViewList))
        },
        removeTagView(index, type) {
            if (type === 'index') {
                this.tagsViewList.splice(index, 1)
            } else if (type === 'right') {
                this.tagsViewList.splice(index + 1, this.tagsViewList.length - index + 1)
            } else if (type === 'other') {
                this.tagsViewList.splice(index + 1, this.tagsViewList.length - index + 1)
                this.tagsViewList.splice(0, index)
            } else {
                this.tagsViewList = []
            }
            Cookies.set('tagsViewList', JSON.stringify(this.tagsViewList))
        },
        initTagView() {
            const tagsViewList = Cookies.get('tagsViewList')
            if (tagsViewList) {
                this.tagsViewList = JSON.parse(tagsViewList)
            }
        }
    }
})