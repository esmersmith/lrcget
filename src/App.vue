<template>
  <div class="h-screen w-screen flex flex-col select-none">
    <div class="fixed top-0 left-0 flex justify-end items-start text-sm flex-none z-50">
      <div class="p-0.5 m-1 rounded-full text-sm text-hoa-1400 hover:bg-hoa-600 active:bg-hoa-800 transition" @click="openDevtools">
        <Bug />
      </div>
    </div>
    <div v-if="!loading" class="grow overflow-hidden bg-white dark:bg-brave-background-dark">
      <ChooseDirectory v-if="!init" @progressStep="init = true" />
      <Library v-else @uninitialize-library="uninitializeLibrary" />
    </div>
  </div>

  <ModalsContainer />
</template>

<script setup>
import { Bug, WindowMinimize, WindowMaximize, WindowClose } from 'mdue'
import ChooseDirectory from "./components/ChooseDirectory.vue";
import Library from "./components/Library.vue";
import { ref, onMounted, watch } from 'vue'
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow'
import { invoke } from '@tauri-apps/api/core'
import { ModalsContainer } from 'vue-final-modal'
import { useGlobalState } from './composables/global-state'
import { useDownloader } from '@/composables/downloader.js'
const appWindow = getCurrentWebviewWindow()

const { themeMode, setThemeMode, setLrclibInstance } = useGlobalState()
const { downloadNext } = useDownloader()

const loading = ref(true)
const init = ref(false)
const isProd = ref(import.meta.env.PROD)

const uninitializeLibrary = async () => {
  loading.value = true
  await invoke('uninitialize_library')
  init.value = await invoke('get_init')
  loading.value = false
}

onMounted(async () => {
  init.value = await invoke('get_init')
  loading.value = false
  await loadGlobalState()
  darkModeHandle()
  downloadNext()
})

const loadGlobalState = async () => {
  const config = await invoke('get_config')
  setThemeMode(config.theme_mode)
  setLrclibInstance(config.lrclib_instance)
}

const darkModeHandle = async () => {
  // check and insert the `dark` class to html tag
  if (themeMode.value === 'dark') {
    document.documentElement.classList.add('dark')
  } else {
    document.documentElement.classList.remove('dark')
  }
}

const openDevtools = () => {
  invoke("open_devtools");
}

const minimizeWindow = () => {
  appWindow.minimize()
}

const maximizeWindow = () => {
  appWindow.toggleMaximize()
}

const closeWindow = () => {
  appWindow.close()
}

watch(themeMode, () => {
  darkModeHandle()
})
</script>

<style>
</style>
