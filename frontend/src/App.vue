<script setup>
import { RouterView } from 'vue-router'
import logo from '@/assets/img/handflow-logo.svg'
import logoDark from '@/assets/img/handflow-logo-dark.svg'
import { PhMoon, PhSun, PhGithubLogo } from '@phosphor-icons/vue'
import { useTheme } from '@/composables/useTheme'
import AppNotification from '@/components/AppNotification.vue'
import '@/assets/styles/layout/app.css'

const { isDark, toggleTheme } = useTheme()
</script>

<template>
  <div class="app-wrapper">
    <AppNotification />
    <button 
      class="theme-toggle" 
      @click="toggleTheme" 
      :title="isDark ? 'Switch to light mode' : 'Switch to dark mode'"
    >
      <component :is="isDark ? PhSun : PhMoon" :size="24" weight="fill" />
    </button>
    <header class="app-header">
      <div class="header-content">
        <router-link to="/" class="logo-link">
          <img :src="isDark ? logoDark : logo" alt="HandFlow Logo" class="logo" fetchpriority="high">
          <div class="header-title">
            <h1>HandFlow</h1>
            <h2>Handpan Tablature Generator</h2>
          </div>
        </router-link>
      </div>
    </header>

    <main class="app-main">
      <div class="main-content">
        <RouterView />
      </div>
    </main>

    <footer class="app-footer">
      <div class="footer-content">
        <div class="footer-links">
          <a href="https://github.com/MusicFlow-app/HandFlow" class="github-link" target="_blank" rel="noopener">
            <PhGithubLogo :size="32" weight="duotone" />
            <span>GitHub</span>
          </a>
        </div>
        <p class="copyright">© {{ new Date().getFullYear() }} HandFlow</p>
      </div>
    </footer>
  </div>
</template>


