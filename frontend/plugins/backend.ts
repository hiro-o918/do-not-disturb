import { Plugin, } from '@nuxt/types'
import { NuxtAxiosInstance } from '@nuxtjs/axios'

declare module 'vue/types/vue' {
  interface Vue {
    $backendClient: NuxtAxiosInstance
  }
}

declare module '@nuxt/types' {
  interface NuxtAppOptions {
    $backendClient: NuxtAxiosInstance
  }
}

declare module 'vuex/types/index' {
  interface Store<S> {
    $backendClient: NuxtAxiosInstance
  }
}

const plugin: Plugin = ({ env, $axios }: any, inject: any) => {
  inject('backendClient', $axios.create({
    baseURL: env.backendBaseURL
  }))
}

export default plugin
